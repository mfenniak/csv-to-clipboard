# csv-to-clipboard

This is a simple utility that quickly copies a CSV file onto your clipboard in a tab-separated format, which allows the contents to be pasted into a spreadsheet at the current cursor.

## Linux

Using Nix to create a build environment... not the only way for sure, but one easy way.

```
nix-shell -p cargo -p gtk3
cargo build
```

Otherwise, you'd need rust (works w/ 1.61.0) and gtk development libraries.

A `PKGBUILD` for Arch is published in the Arch User Repository.


## General

Testing app:

```
cargo test
cargo run ../some-csv-file.csv
```

