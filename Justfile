install:
    #!/bin/sh
    for tool in $(grep , Cargo.toml | cut -d \" -f2 | grep -v common)
    do
        echo "installing $tool"
        cargo install $1 --path $tool >/dev/null 2>&1
    done
