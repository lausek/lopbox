#! /usr/bin

BUTTONS="[{\"code\":10,\"label\":\"Lock\"}, {\"code\":20,\"label\":\"Reboot\"}, {\"code\":30,\"label\":\"Shutdown\"}]"

../target/debug/opbox -c -b "$BUTTONS" 

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
