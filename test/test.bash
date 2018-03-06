#! /usr/bin

# change this path according to your build directory
BUILDPATH=~/Projects/opbox/target/debug/opbox
OPBOX=$(realpath $BUILDPATH)
FLAGS="-f #d3d3d3 -b #101010"

BUTTONS='[{"code":10,"label":"Lock"}, {"code":20,"label":"Reboot"}, {"code":30,"label":"Shutdown"}]' 

# tr: escape json string 
$OPBOX -c $FLAGS -o "$(echo $BUTTONS | tr '"' '\"')" 

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
