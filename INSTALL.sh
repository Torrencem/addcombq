#!/usr/bin/env bash

if [ $EUID != 0 ]; then
    sudo "$0" "$@"
    exit $?
fi

ver() {
    cat package-version.txt | tr -d '\n'
}

./package.sh

SAGE_MAIN=$(which sage)

if [ -z "$SAGE_MAIN" ];
then
    echo "Sage installation not found (in path)"
    echo "make sure sage is installed as 'sage'"
    exit 1
fi

if [[ "$OSTYPE" == "linux-gnu" ]]; then
	SAGE=$(readlink -f $SAGE_MAIN)
elif [[ "$OSTYPE" == "darwin"* ]]; then
	SAGE=$(./mac_utils/greadlink.sh $SAGE_MAIN)
fi

SAGE_DIR="$(dirname "$SAGE")"

echo Installing to sage installed in directory $SAGE_DIR
read -p "Do you want to continue?" yn

case $yn in
    [Nn]* ) exit;;
    * ) echo "Installing...";;
esac

rm $SAGE_DIR/upstream/addcombq-$(ver).tar.gz
cp ./addcombq-$(ver).tar.gz $SAGE_DIR/upstream/addcombq-$(ver).tar.gz

rm -r $SAGE_DIR/build/pkgs/addcombq

mkdir $SAGE_DIR/build/pkgs/addcombq
tar -C $SAGE_DIR/build/pkgs/addcombq -zxvf ./addcombq-$(ver).tar.gz

sage --package fix-checksum

if [ "$1" == "test" ];
then
    sage -p -c addcombq-0.1
else
    sage -p addcombq-0.1
fi

echo "Done installing to sage"
