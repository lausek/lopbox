`opbox` is a small gtk based program for creating button dialogs. It is intended to be used as part of shell scripts.

## Usage

Buttons displayed inside the dialog can be configured by a json string that contains a list of objects. Objects itself can hold the attributes `code` for the designated return value and `label` for the buttons text. Any other keys will be ignored.

`opbox` will return `0` for a regular close or cancel, `1` for an internal error - every other code is free for use by custom buttons.

<p align="center">
    <img src="screenshot.png" alt="example"/>
</p>

`test.bash`

``` bash
#!/usr/bin

BUTTONS='[{"code":10,"label":"Lock"}, {"code":20,"label":"Reboot"}, {"code":30,"label":"Shutdown"}]' 

opbox -c $FLAGS -o "$(echo $BUTTONS | tr '"' '\"')" 

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

This repo requires you to have `libgtk-3-dev` installed on your system (see [here](https://github.com/gtk-rs/gtk)). 

### Debian or Ubuntu

``` bash
$ sudo apt-get install libgtk-3-dev
```

## Background

Do you know the moment when you are lying in your bed and the only thing you have in scope is your wireless mouse? Too bad that you use a tiled window manager and your setup isn't meant to be shut down by mouse... 
