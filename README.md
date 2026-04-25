# Little Tools

[![Test](https://github.com/snltd/little_tools/actions/workflows/test.yml/badge.svg)](https://github.com/snltd/little_tools/actions/workflows/test.yml)

A bunch of little command-line tools that I find useful. Most of them are
rewrites of shell or Ruby scripts I've had kicking around for years.

Install them with `cargo install --path <name>`, or do the whole lot in one go
with `./install.sh`.

Everything has `--help`, but, briefly:

## `align-times`

Recursively makes timestamps of files in `source/` match those in `dest/`. You
don't need it until you need it.

## `alsort`

Sorts files into directories based on the lower-cased first letter of their
name: so `smart_unix.txt` and `SUCKYDOS.TXT` both go in `s/`. `--group` makes it
use `abc`, `def`... rather than the initial. (Numbers go in `0-9`, everything
else goes in `symbols`.)

## `cf`

Counts files in directories. Ouptut is like `wc`, so it's easy to sort.

```sh
$ cf /etc /bin
        186     /etc
        942     /bin
```

Optionally recurses down trees, and can omit directories from the counts.

## `cs`

Flattens fancy filenames to `lowercase_ascii_with_underscores`. Reject the
advances of unicode like a boss.

```sh
$ ls
90°.hot  'This Is A File.TXT'
$ cs *
90°.hot -> 90.hot
This Is A File.TXT -> this_is_a_file.txt
$ ls
90.hot  this_is_a_file.txt
```

## `flink`

Links files as home-directory dotfiles.

- `zshrc` -> `~/.zshrc`
- `config-helix/config.toml` -> `~/.config/helix/config.toml`

## `fseq`

Renames files to follow a pattern, with sequence numbers.

## `mixup`

Mixes up bodies of text with granularity `char`, `word`, `line`, or `file`. If
you give multiple files, the `-i` option will mix the files together first, then
mix the result up with the specified granularity. Sort of like a psychotic
`shuf`.

## `mmv`

Batch renamer. Takes a find-and-replace pair, and subs `find` with `replace` in
the names of the given files. Supports Rust regular expressions and capture
groups; has a no-op mode for safe experimentation; has clobber-protection,
multi-replace, and various levels of verbosity.

```sh
$ ls
file1.txt  file2.txt  file3.txt
$ mmv -v file renamed_file *
file1.txt file1.txt -> renamed_file1.txt
file2.txt file2.txt -> renamed_file2.txt
file3.txt file3.txt -> renamed_file3.txt
$ ls
renamed_file1.txt  renamed_file2.txt  renamed_file3.txt
$ mmv "re(\w+)(\d).txt" "number_\${2}_\${1}.text" *
$ ls
number_1_named_file.text  number_2_named_file.text  number_3_named_file.text
```

`--git` prints out `git mv` commands, which you can paste back into your shell.

## `randos`

Randomly selects a given number of files from a list or directory tree, and
either symlinks, hard links, copies, or moves them to some other directory.

You can filter the source files by file extension, age, or a regular expression.

The new files can have new names, specified by the `-s` option.

- `-s plain`: the target filename is the same as the source filename.
- `-s hash`: the target filename is a SHA1 hash of the source file's full path.
- `-s expand`: the target filename is the source file's full path, but with `/`
  replaced by `-`.
- `-s seq`: the targets are named sequentially, from `00000001` upwards. The
  source file's extension (if any) is preserved.
