# Little Tools

A bunch of little command-line tools that I find useful. They are Rust
rewrites, done primarily as a learning exercise, so you trust them at your
peril.

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

## `cf`

Counts files in directories, presenting info like `wc`, so it's easy to sort.

```
$ cf /etc /bin
        186     /etc
        942     /bin
```

## `mmv`

Batch renamer. Takes a find-and-replace pair, and subs `find` with `replace`
in the names of the given files. Supports regular expressions and capture
groups, and has a no-op mode for safe experimentation.

```
$ ls
file1.txt  file2.txt  file3.txt
$ mmv file renamed_file *
$ ls
renamed_file1.txt  renamed_file2.txt  renamed_file3.txt
$ mmv "re(\w+)(\d).txt" "number_\${2}_\${1}.text" *
$ ls
number_1_named_file.text  number_2_named_file.text  number_3_named_file.text
```
