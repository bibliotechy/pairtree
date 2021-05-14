use structopt::{StructOpt};
use walkdir::{WalkDir,DirEntry};
use std::fs::read;
use std::io;
use std::str::FromStr;
use md5::{Md5, Digest};
use sha1::{Sha1};
use hex;
use regex;

#[derive(Clone, Copy)]
enum HashType{
    MD5,
    SHA1,
    Blake3,
}

impl FromStr for HashType {
    type Err = String;
    fn from_str(hash_type: &str) -> Result<Self, Self::Err> {
        match hash_type {
            "md5" => Ok(HashType::MD5),
            "sha1" => Ok(HashType::SHA1),
            "blake3" => Ok(HashType::Blake3),
            _ => Err(String::from("Please prove a valid Hash type"))
        }
    }
}




#[derive(StructOpt)]
#[structopt(rename_all = "kebab-case")]
struct Cli {
    
    #[structopt(short = "s", long, parse(from_os_str))]
    source_dir: std::path::PathBuf,

    #[structopt(short = "d", long, parse(from_os_str))]
    dest_dir: std::path::PathBuf,

    #[structopt(short = "h", long, default_value = "blake3")]
    hash_type: HashType,

    #[structopt(short = "u", long)]
    use_hash_for_filename: bool,

    #[structopt(short = "k", long)]
    keep_file_extension: bool,

    #[structopt(short = "o", long)]
    origin_path_in_dest_name: bool

}


struct Pairtree {
    entry: DirEntry,
    dest_dir: String,
    hash_type: HashType, 
    use_hash_for_filename: bool,
    keep_file_extension: bool,
    origin_path_in_dest_name: bool
}

impl Pairtree {
    fn hex_string(&self) -> String{
        let f = read(self.entry.path()).expect("no file found").to_owned();
        let hex = match self.hash_type {
            HashType::MD5    => self.md5_hash(&f),
            HashType::Blake3 => blake3::hash(f.as_ref()).to_hex().as_str().to_owned(),
            HashType::SHA1   => self.sha1_hash(&f)
        };
        return hex
    }

    fn dest_path_base(&self) -> std::path::PathBuf{
        let out_path: std::path::PathBuf = [
            &self.dest_dir, 
            &self.hex_string()[0..2].to_string(), 
            &self.hex_string()[2..4].to_string()
        ].iter().collect();
        return out_path
    }

    fn dest_path(&self) -> std::path::PathBuf {
        let mut out_path = self.dest_path_base();
        if !self.use_hash_for_filename {
            if self.origin_path_in_dest_name {
                let path = self.entry.path().to_str().unwrap();
                let pattern = regex::Regex::new(r"[/\\]").unwrap();
                let new_path = pattern.replace_all(&path, "_");
                out_path.push(new_path.to_string());
            }
            else {
                out_path.push(&self.entry.file_name().to_str().unwrap());
            }

        }
        else {
            let filename = self.hex_string();

            if self.keep_file_extension {
                let ex = self.entry.path().extension().unwrap().to_str().unwrap();
                let mut owned_filename = filename.to_owned();
                owned_filename.push_str(ex);
                out_path.push(owned_filename);
            }
            else {
                out_path.push(filename)
            }
        }
        
        return out_path 
    }

    fn md5_hash(&self, file_contents: &Vec<u8>) -> String {
        let mut hasher = Md5::new();
        hasher.update(file_contents);
        let result = hasher.finalize();
        let res = hex::encode(result);
        return res.to_owned()
    }

    fn sha1_hash(&self, file_contents: &Vec<u8>) -> String {
        let mut hasher = Sha1::new();
        hasher.update(file_contents);
        let result = hasher.finalize();
        let res = hex::encode(result); 
        return res.to_owned()
    }
}


fn main() -> io::Result<()> {
    let args = Cli::from_args();

    let dest_dir = match args.dest_dir.to_str() {
        Some(x) => x,
        None    => panic!("Dest dir not specified")
    };

    if !args.dest_dir.exists() {
        std::fs::create_dir_all(args.dest_dir.as_path())?;
    }
   

    for entry in WalkDir::new(args.source_dir).into_iter().filter_map(|e| e.ok()) {
        
        // We can skip directories
        if entry.file_type().is_file() {

            let pt = Pairtree{
                entry: entry, 
                dest_dir: String::from(dest_dir),
                hash_type: args.hash_type,
                use_hash_for_filename:  args.use_hash_for_filename, 
                keep_file_extension:  args.keep_file_extension, 
                origin_path_in_dest_name: args.origin_path_in_dest_name
            };
            
            // Ensure the pairtree directory exists
            std::fs::create_dir_all(pt.dest_path_base())?;
            std::fs::rename(pt.entry.path(), pt.dest_path())?;
        }
    }
    Ok(())
    
}