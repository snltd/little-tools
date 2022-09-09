# `mmv`

`mmv` is a batch renamer. It takes a find-and-replace pair, and subs `find`
with `replace` in the names of the given files. It supports regular
expressions and capture groups, and has a noop for safe experimentation.

```
USAGE:
    mmv [OPTIONS] <PATTERN> <REPLACE> [FILES]...

ARGS:
    <PATTERN>     pattern to replace. Supports Rust regexes
    <REPLACE>     string that should replace <pattern>. Supports Rust capture groups, like ${1}
    <FILES>...    files to rename

OPTIONS:
    -a, --all                    replace all occurrences of pattern
    -c, --clobber                overwrite existing files
    -f, --full                   show fully qualified pathnames in verbose output
    -h, --help                   Print help information
    -m, --match <REPLACE_NTH>    only replace the nth match (starts at 0)
    -n, --noop                   just print the rename operations
    -t, --terse                  with -n, on print the target basenames
    -v, --verbose                be verbose
    -V, --version                Print version information
```

`mmv` is one of the first things I have written in Rust, so you might be smart
not to trust it. It's a bit sketchy.
