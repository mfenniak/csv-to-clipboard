# csv-to-clipboard

This is a simple utility that quickly copies a CSV file onto your clipboard in a tab-separated format, which allows the contents to be pasted into a spreadsheet at the current cursor.

It doesn't sound like much... but it's kinda magic when so many software development tools provide CSV download/export capabilities.  You download the file, double-click the download, and then paste it into a spreadsheet that you're working on.  Super easy to combine data from a variety of different data sources in that way.

## Windows

You can build a Windows installer.  I ... kinda forget how, but it's here!

## Linux

Using Nix to create a build environment... not the only way for sure, but one easy way.

```
nix-shell -p cargo -p gtk3
cargo build
```

Otherwise, you'd need rust (works w/ 1.61.0) and gtk development libraries.

A `PKGBUILD` for Arch is published in the [Arch User Repository](https://aur.archlinux.org/packages/csv-to-clipboard).

## General

Double-click on a CSV file after you've associated this app with your CSV files.  A message box will pop up confirming that the contents have been copied to the clipboard; you're done!

## Development

Testing app:

```
cargo test
cargo run ../some-csv-file.csv
```

