# fetch-preview

A utility to fetch preview images from given links.

This tool can come handy when you kept a large list, and you don't want to know the content by clicking.

The initiative of this project is to provide a preview for my MPV playlist that I use as a [music player](https://codeberg.org/iff/dotfiles/src/branch/main/shell/ms).

## Usage

Example:
```
cargo run --release - --file path/to/file/with/links
```

Usages:
```
Usage: fetch-preview [OPTION]...
Fetches a preview image for a link or a list of links in a file.

Options:
	-f, --file FILE         Fetches a preview image for each link in FILE.
	-l, --link LINK         Fetches a preview image for LINK.
	-o, --output_dir DIR    Sets the output directory to DIR. (default: ./preview)
	-h, --help              Shows this help.
```
