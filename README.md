# gi: interactive shell for git
Currently, this repository is under development.

## Install

```bash
cargo install gi
```

## Usage
```bash
gi switch
```

then interactive shell is shown as below.

```bash
? Select a branch to switch to
> feat/mv
  main
  origin/main
[↑↓ to move, enter to select, type to filter]
```
You can check the full command list with `gi --help`.

## Docker
To build docker container, use
```
source ./docker/build.sh
```
