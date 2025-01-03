# stupidfs

### More files per file: hide files by storing them in the metadata of other files

---

Want to hide files inside other files? Want to make the most out of your
filesystem?

stupidfs stores files in the metadata of your files, allowing your filesystem to
hold more files per file.

This can be used as a stealthy way to hide data, or as a useless waste of time.

## How does it work?

stupidfs stores information in the 'last modified date' of files in the target
directory. The data is stored in the sub-second portion of the timestamp, so it
shouldn't have any visible effect when applied to a directory.

stupidfs is a good way to store files inconspicuously, since it's quite hard to
tell if a directory contains data stored under stupidfs. in fact, the data
itself is somewhat volatile: on my machine, copy pasting the folder updates the
last modified date, rendering useless the idea of copying data from an existing
drive for later use.

## Usage

First, run `cargo install stupidfs` or compile it from source.

To read, give stupidfs a directory and read the data from it.
```sh
stupidfs -o ./data
```

To write, give stupidfs a directory and pipe data into it.

```sh
echo "Hello, world!" | stupidfs -i ./data
```

Some notes:
- Output mode is enabled by default, so the `-o` is optional.
- When writing, extra data will be ignored.
- One file is required for every three bytes of input.


stupidfs might not work on filesystems with less than nanosecond granularity,
but it works on my ext4 filesystem that presumably has large enough inodes.


## Visibility
stupidfs stores data in the last 24 bits of the timestamp of the last
modification date of each file. The first few bits of the sub-second portion are
not touched, meaning the actual change in the timestamp is very small (up to
16.778ms).

## Accuracy
A filesystem supporting nanosecond resolution on timestamps does not necessarily
mean the kernel and hardware support timestamps of such a resolution. If the
hardware doesn't support this resolution or the kernel doesn't update the system
time that often, machine times might end up clustered around (or exactly on)
some greatest common divisor (e.g. microseconds). If this is the case, it'll be
pretty easy to figure out which files were artificially modified.

## Information
Technically more than 3 bytes of information is stored per file, as other file
metadata (e.g. filenames) is used to determine where a given file's bytes are
situated within the actual stored data. This isn't particularly important
though, since it's not correlated with actual file data and can't be used to
reconstruct anything.
