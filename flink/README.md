`flink` links files.

I have a bunch of files in an NFS directory, which I want to be symlinked into
my `$HOME`.

```sh
$ ls ~/work/_DOTFILES
aur.toml              config-zellij         gitconfig             mame                  vimrc
clojure               dircolors             hushlogin             ppd                   wezterm.lua
config-helix          editrc                inputrc               README.md             XCompose
config-starship.toml  gemrc                 lein                  vim                   zshrc
$ flink ~work/_DOTFILES
$ ls -al ~
total 1207
drwxr-xr-x  14 rob      nobody        71 Nov  3 16:31 .
drwxr-xr-x   7 root     root           7 Mar 12  2025 ..
lrwxrwxrwx   1 rob      sysadmin      23 Nov  3 16:30 .aur.toml -> work/_DOTFILES/aur.toml
drwxr-xr-x   5 rob      nobody        11 Nov  3 16:28 .cargo
lrwxrwxrwx   1 rob      sysadmin      22 Nov  3 16:30 .clojure -> work/_DOTFILES/clojure
drwxr-xr-x   2 rob      nobody         5 Nov  3 16:30 .config
lrwxrwxrwx   1 rob      sysadmin      24 Nov  3 16:30 .dircolors -> work/_DOTFILES/dircolors
...
```

You can specify as many source directories as you wish.

Any source file whose name which begins with a dot, or with `README` is ignored,
so you can keep your dotfile directories in git or whatever, and say what they
are.

The link which is created (in your home dir by default, but `--root` lets you
use others) is prefixed with a dot.

If a source file name contains dashes, they are converted to slashes. To
`config-helix` is linked from `~/.config/helix`.
