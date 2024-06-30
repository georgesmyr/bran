# Introduction

These notes are for me to be think how to structure my implementation of git in Rust.

# `ObjectKind` 

`ObjectKind` is an enum that holds information about what kind of object is an object. It can be either Tree, Blob or Commit.

# `EntryMode` 

Entry mode is an enum that holds information about the tree entry mode, i.e. if it's a directory, executable, non-executable etc. I think this provides convenience because of the methods, `from_metadata` which reads the metadata of a direntry and deduces the entry mode, or `from_str` which is the way git writes down the mode. Also, it implements how its formatted in display and debug.

# `ObjectID`

`ObjectID` is a struct that holds information about the id of an object, either in hash string (hex encoded) or in bytes.

# `Object`

Object is a trait that all objects should implement, i.e. Blobs, Trees, and Commits. It has three methods that objects should implement, `kind`, `size` and `content` which gives back a reader of the contents of the object. There are other methods already implemented that write the object in the database and return an ObjectID.

# `Blob`
`Blob` is a struct that represents a blob object, and implements the `Object` trait.

# `Tree`
`Tree` is a struct that represents a tree object, and implements the `Object` trait.