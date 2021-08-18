`lopbox` is a small gtk based program for creating button dialogs. It is intended to be used as part of shell scripts.

## Usage

Buttons displayed inside the dialog can be configured by a json string that contains a list of objects. Objects itself can hold the attributes `code` for the designated return value and `label` for the buttons text. Any other keys will be ignored.

`lopbox` will return `0` for a regular close or cancel, `1` for an internal error - every other code is free for use by custom buttons.

Buttons can either be declared by passing json via parameter `-o` or by piping lines in the form `<return_code>;<label>` to stdin.

<p align="center">
    <img src="screenshot.png" alt="example"/>
</p>

``` bash
#!/bin/bash

layout() {
    echo "10;Lock"
    echo "20;Reboot"
    echo "30;Shutdown"
}

layout | lopbox -b "#101010" -f "#d3d3d3" -c

# check on return code 
case $? in
"10")
    echo "lock";
    ;;
"20")
    echo "reboot";
    ;;
"30")
    echo "shutdown";
    ;;
*)
    echo "cancel";
esac
```

## Installation

``` bash
cargo install lopbox
```

## Development requirements

This repo requires you to have `libgtk-3-dev` installed on your system (see [here](https://github.com/gtk-rs/gtk)). 

### Debian or Ubuntu

``` bash
$ sudo apt-get install libgtk-3-dev
```
