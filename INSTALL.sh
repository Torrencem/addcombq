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

SAGE=$(readlink -f $SAGE_MAIN)

SAGE_DIR=$(dirname $SAGE)

echo Installing to sage installed in directory $SAGE_DIR
read -p "Do you want to continue?" yn

case $yn in
    [Nn]* ) exit;;
    * ) echo "Installing...";;
esac

rm $SAGE_DIR/upstream/pf_addcomb-$(ver).tar.gz
cp ./pf_addcomb-$(ver).tar.gz $SAGE_DIR/upstream/pf_addcomb-$(ver).tar.gz

rm -r $SAGE_DIR/build/pkgs/pf_addcomb

mkdir $SAGE_DIR/build/pkgs/pf_addcomb
tar -C $SAGE_DIR/build/pkgs/pf_addcomb -zxvf ./pf_addcomb-$(ver).tar.gz

sage --package fix-checksum

sage -p pf_addcomb-0.1

echo "Done installing to sage"