# MFEKmetadata — UFO font metadata fetcher

Basic metadata fetcher for the MFEK project. It interrogates UFO fonts, for now mostly fontinfo.plist&mdash;also determines a list of glyphs.

```
MFEKmetadata 0.0.1-beta1
Fredrick Brennan <copypaste@kittens.ph>
Basic metadata fetcher for the MFEK project. It interrogates UFO fonts, for now mostly fontinfo.plist

USAGE:
    MFEKmetadata <PATH> <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <PATH>    Sets the input file (glif/UFO/rarely plist) to use

SUBCOMMANDS:
    arbitrary         Dumps key values
    glyph             Dumps a single font glyph in the format of `MFEKmetadata glyphs`
    glyphpathlen      Show length of contours in a glyph (.glif) on separate lines
    glyphs            Dumps the font's glyphs
    glyphslen         Show number of glyphs in font
    help              Prints this message or the help of the given subcommand(s)
    write_metainfo    
```

```
MFEKmetadata-arbitrary 0.0.1-beta1
Dumps key values

USAGE:
    MFEKmetadata <PATH> arbitrary [OPTIONS] --key <keys>...

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -k, --key <keys>...                  List of key values to display, one per line, in order requested
    -v, --value <values>...              List of values to write, in order requested
    -X, --xml-redirect <xml-redirect>    Redirect XML to this path instead. Use /dev/stdout or /dev/stderr if that's
                                         what you want, `-` not recognized.
```

```
MFEKmetadata-glyphs 
Dumps the font's glyphs

USAGE:
    MFEKmetadata <UFO_OR_GLIF> glyphs

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
```

```
MFEKmetadata-glyphslen 
Show number of glyphs in font

USAGE:
    MFEKmetadata glyphslen

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
```

```
MFEKmetadata-glyphpathlen
Show length of contours in a glyph (.glif) on separate lines

USAGE:
    MFEKmetadata glyphpathlen [FLAGS] [OPTIONS]

FLAGS:
    -s, --segmentwise    Display length of each segment separated by spaces
    -j, --joined         Display one line: sum of joined path
    -J, --json           Output JSON instead
    -h, --help           Prints help information
    -V, --version        Prints version information

OPTIONS:
        --accuracy <accuracy>    Precision of length calculation [default: 0.01]
```

## Building

This MFEK module uses `git` submodules. So, after you clone, you have to run:

```bash
git submodule init
git submodule update
```

Before you call `cargo build`.

## License

```
Copyright 2020–2021 Fredrick R. Brennan & MFEK Authors

Licensed under the Apache License, Version 2.0 (the "License"); you may not use
this software or any of the provided source code files except in compliance
with the License.  You may obtain a copy of the License at

  http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software distributed
under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
CONDITIONS OF ANY KIND, either express or implied.  See the License for the
specific language governing permissions and limitations under the License.
```

**By contributing you release your contribution under the terms of the license.**
