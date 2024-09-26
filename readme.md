some notes:



package (the one that has cargo.toml) contains creates (the one with .rs files) contains modules (also .rs files use the mod keyword) see below for the differences


special file names are

- main.rs -> mean binary crate
- lib.rs -> mean library crate
- other than that ex: util.rs -> mean modules that are meant to be used in these crates


some modules cheatsheet:
https://rust-book.cs.brown.edu/ch07-02-defining-modules-to-control-scope-and-privacy.html
