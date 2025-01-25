# Little Tools

[![Test](https://github.com/snltd/little_tools/actions/workflows/test.yml/badge.svg)](https://github.com/snltd/little_tools/actions/workflows/test.yml)

A bunch of little command-line tools that I find useful. They are Rust rewrites,
done primarily as a learning exercise, so you trust them at your peril.

## `alsort`

Sorts files into directories based on the first letter of their name. The
directories it creates will always be lower case, so `MY_DOS_FILE.TXT` goes into
`m/`. If you supply the `--group` option, rather than using the single initial
as the target directory name, it will pick one of `abc`, `def`, etc. Numbers go
in `0-9`, everything else goes in `symbols`.

## `cf`

Counts files in directories, presenting info like `wc`, so it's easy to sort.

```
$ cf /etc /bin
        186     /etc
        942     /bin
```

Optionally recurses down trees, and can omit directories from the counts.

## `cs`

Flattens fancy filenames in to `lowercase_ascii_with_underscores`.

```
$ ls
90°.hot  'This Is A File.TXT'
$ cs *
90°.hot -> 90.hot
This Is A File.TXT -> this_is_a_file.txt
$ ls
90.hot  this_is_a_file.txt
```

## `fseq`

Renames files to follow a pattern, with sequence numbers.

## `mmv`

Batch renamer. Takes a find-and-replace pair, and subs `find` with `replace` in
the names of the given files. Supports regular expressions and capture groups,
and has a no-op mode for safe experimentation.

```
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

Has clobber-protection, multi-replace, and various levels of verbosity. `--help`
explains.

`--git` prints out `git mv` commands, which you can paste back into your shell.

## `randos`

Randomly selects a given number of files from a list or directory tree, and
either symlinks, hard links, copies, or moves them to some other directory.

You can filter the source files by file extension, age, or a regular expression.

The new files can have new names, specified by the `-s` option. 
* `-s plain`: the target filename is the same as the source filename.
* `-s hash`: the target filename is a SHA1 hash of the source file's full path.
* `-s expand`: the target filename is the source file's full path, but with
`/` replaced by `-`.
* `-s seq`: the targets are named sequentially, from `00000001` upwards. The
source file's extension (if any) is preserved.


