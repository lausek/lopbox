#! /usr/bin

# change this path according to your build directory
BUILDPATH=~/Projects/opbox/target/debug/opbox
OPBOX=$(realpath $BUILDPATH)
FLAGS="-f #d3d3d3 -b #101010"

BUTTONS='[{"code":10,"label":"Lock"}, {"code":20,"label":"Reboot"}, {"code":30,"label":"Shutdown"}]' 

function confirm {
    $OPBOX $FLAGS -i "Do you really want to $1?" -o "$(echo '[{"code":10,"label":"Yes"}, {"code":20,"label":"No"}]' | tr '"' '\"')"
}

# tr: escape json string 
$OPBOX -c $FLAGS -o "$(echo $BUTTONS | tr '"' '\"')" 

# check on return code 
case $? in
"10")
    echo "lock";
    ;;
"20")
    confirm reboot
    if [ $? -eq "10" ]; then
        echo "reboot";
    fi
    ;;
"30")
    confirm shutdown
    if [ $? -eq "10" ]; then
        echo "shutdown";
    fi
    ;;
*)
    echo "cancel";
esac
