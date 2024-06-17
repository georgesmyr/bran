# bran
![Image Description](assets/bran_v0.1.0.webp)

Blazing fast partial Implementation of Git in Rust. Why? Why not?


## Initialization of repository

Similar to git, one can initialize a repository by going to the root directory of the repository, say `georgesmyr/repos/new_repo`, and run
```shell
bran init
```
If there is not an initialized repo in this directory, the response will be
```
Initialized empty Git repository in "georgesmyr/repos/test_repo/.git"
```
Alternatlively, one can initialize the repo in the desired directory by providing the path as an argument, i.e.
```shell
bran init --path georgesmyr\repos\new_repo
```
```
Initialized empty Git repository in "georgesmyr/repostest_repo/.git"
```

## `hash-object` 

Returns the hash of an object. If the flag `-w` or `--write` is included, the object is written in the database.
```shell
> cat hello.txt
> hello
```
```shell
> bran hash-object -w hello.txt
> ce013625030ba8dba906f756967f9e9ca394464a
```

## `cat-file`

Reveals the contents of an object with specified hash. The `-p` or `--pretty-print` flag must be provided for now. For example
```shell
> bran cat-file -p ce013625030ba8dba906f756967f9e9ca394464a
> hello
```