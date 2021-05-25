# Pairtree

A Rust CLI app to move files in an arbitrarily nested directory structure to a [pairtree structure](https://confluence.ucop.edu/display/Curation/PairTree?preview=/14254128/16973838/PairtreeSpec.pdf).


## Installation

TODO

## Usage

```bash

pairtree --help

USAGE:
    pairtree [FLAGS] [OPTIONS] --dest-dir <dest-dir> --source-dir <source-dir>

FLAGS:
        --help                        Prints help information
    -k, --keep-extension              If using the hash as the file name, retain the original file extension.
    -o, --origin-path-in-dest-name    Use the full filepath to the source file as the name of the destination file,
                                      replacing '/' with '_'.
    -u, --use-hash-for-filename       Use the hash as the filename in the destination directory.
    -V, --version                     Prints version information

OPTIONS:
    -d, --dest-dir <dest-dir>        Path to directory where pairtree structure will be created.
    -h, --hash-type <hash-type>      Type of has algorithm to use. Options: md5, sha1, blake3. [default: blake3]
    -s, --source-dir <source-dir>    Path to directory with existing files to be moved.

```

### Example usage

We have a directory with some files at various levels of a nested directory structure:

```bash
tree ./scratch/
scratch/
├── test3
└── top
    ├── middle
    │   ├── bottom
    │   │   └── test.txt
    │   └── test2.txt
    └── test3.txt
```

To move them into a pairtree structure at /var/data using the md5 algorith, you'd run
```bash
pairtree --source-dir ./scratch --dest-dir /var/data --hash-type md5 
```

And then we can see the files moved to the destination directory.

```bash
tree /var/data

var/data/
├── 19
│   └── d0
│       └── test2.txt
├── 9b
│   └── 4b
│       └── test3.txt
├── b3
│   └── bc
│       └── test3
└── bc
    └── 2e
        └── test.txt
```

------

Shoutout to [@demiankatz](https://github.com/demiankatz), [@crhallberg](https://github.com/crhallberg), [@Geoffsc](https://github.com/Geoffsc), and [@nomadicoder](https://github.com/nomadicoder) for the code swarm and review session that helped me fix multiple bugs!

------
License: [Hippocratic License Version Number: 2.1.](license.md)
