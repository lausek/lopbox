#! /usr/bin

# change this path according to your build directory
BUILDPATH=~/Projects/opbox/target/debug/opbox
OPBOX=$(realpath $BUILDPATH)

BUTTONS="[{\"code\":10,\"label\":\"Lock\"}, {\"code\":20,\"label\":\"Reboot\"}, {\"code\":30,\"label\":\"Shutdown\"}]"

$OPBOX -c -o "$BUTTONS" 

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
