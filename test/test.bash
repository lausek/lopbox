#! /usr/bin

./opbox
case $? in
"1")
    echo "lock";
    ;;
"2")
    echo "reboot";
    ;;
"3")
    echo "shutdown";
    ;;
*)
    echo "cancel";
esac
