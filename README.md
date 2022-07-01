# csv-to-clipboard

This is a simple utility that quickly copies a CSV file onto your clipboard in a tab-separated format, which allows the contents to be pasted into a spreadsheet at the current cursor.

## Linux

Using Nix to create a build environment...

```
nix-shell -p cargo -p gtk3
cargo build
```


## General

Testing app:

```
cargo test
cargo run ../some-csv-file.csv
```

