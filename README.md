# Qmetadata

Basic metadata fetcher for the MFEQ project. It interrogates UFO fonts, for now mostly fontinfo.plist&mdash;also determines a list of glyphs.

## Building

This MFEQ module uses `git` submodules. So, after you clone, you have to run:

```bash
git submodule init
git submodule update
```

Before you call `cargo build`.
