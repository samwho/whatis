# whatis

`whatis` aims to be a modern replacement for the `file` tool, but as well as
telling you what the file is it will also tell you information about the file.
If it's an image, it will tell you the dimensions and pull the EXIF data. If
it's an audio file, it will tell you the bitrate and duration. And so on.

**Note: This is very much unfinished, and will probably not ever be complete
enough to be a true replacement to `file`.**

# Installation and usage

As it stands, this project isn't packaged anywhere. To install and run it, you
will need to do so from source.

```
git clone https://github.com/samwho/whatis
cd whatis
cargo run -- path/to/file
```

# Contributing

If you want to contribute, what follows is a small guide on writing code to
recognise file formats you care about.

The program is split in to two distinct phases: matching and extracting.

- **Matching:** determines what file type the file in question may be.
- **Extracting:** extracts information about the file.

Matching is intended to be quick and dirty, in order to reduce the number of
extractors that run. Because of this, extractors need to be written in a way
that doesn't assume the file they're working on is actually of the correct
type. Some random binary file may be misinterpreted as a JPEG because it happens
to have the right magic number at the start.

To make `whatis` support a file format, you will need to write both a matcher
and an extractor.

## Writing a matcher

Matchers live in `src/whatis/matchers`. Each matcher lives in its own file, so
it is reasonable to copy an existing matcher to use as a starting point. All
matcher files should expose a function called `matcher`. This function is then
added to the `ALL` static that lives in `src/whatis/matchers/mod.rs`.

Note that if your file type is unsupported at the moment, you will also have to
add an enum entry for it in `src/whatis/file_type.rs`.

## Writing an extractor

Extractors live in `src/whatis/extractors`. Similar to matchers, each extractor
lives in its own file. All extractor files should expose a function called
`extractor`. This function is then called from `src/whatis/extractors/mod.rs`,
in the `impl Extractor` block.

The return type of the `extractor` function is
`Result<Option<Map<String, Value>>>`. This is quite intimidating so let's break
it down. The `Result` is for unrecoverable errors. These terminate all execution
and will be reported back to the user. Not being able to parse the file type
is not an unrecoverable error. For that you should return `Ok(None)`. The
`Option` is to signal whether or not the extractor was able to get any
information about the file. The `Map<String, Value>` is the data about the file
you were able to extract. It uses `serde_json` types as JSON is an output format
of the program.

Unlike matchers, it's reasonable to run multiple extractors on one file. For
example, the exif extractors runs over multiple file types. To facilitate this,
`src/whatis/extractors/mod.rs` has a function called `combine` that you will
see used in places in that file. Use it if it makes sense to do so.
