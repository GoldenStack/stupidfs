# stupidfs

### Hide files by storing them in the metadata of other files

---

stupidfs stores information in the 'last modified date' of files in a directory.


The data is stored in the sub-second portion of the timestamp, so in most cases
running stupidfs over a file won't produce any visible change. Even if the
entire date is shown, it's basically useless because of how specific it is.

stupidfs is an undercover storage system: it's impossible to tell whether or not data is stored within a directory with stupidfs.
