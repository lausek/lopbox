`opbox` is a small gtk based program for creating button dialogs.

## Installation

This repo requires you to have `libgtk-3-dev` installed on your system (see [here](https://github.com/gtk-rs/gtk)). 

### Debian or Ubuntu

```
> sudo apt-get install libgtk-3-dev
```

## Usage

At the moment `opbox` is hardcoded with system control commands. Buttons and optical tweaks will be configurable later via command line.

The program should be used as a part of bash scripts. `opbox` will return `0` for a regular close or cancel, `1` for an internal error - every other code is free for use by custom buttons.

`test.bash`

``` bash
#! /usr/bin

BUTTONS="[{\"code\":10,\"label\":\"Lock\"}, {\"code\":20,\"label\":\"Reboot\"}, {\"code\":30,\"label\":\"Shutdown\"}]"

opbox -c -o "$BUTTONS" 

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

## Background

Do you know the moment when you are lying in your bed and the only thing you have in scope is your wireless mouse? Too bad that you use a tiled window manager and your setup isn't meant to be shut down by mouse... 
