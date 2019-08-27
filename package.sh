#!/usr/bin/env bash

ver() {
    cat package-version.txt | tr -d '\n'
}

find . -printf "%P\n" \
    -type f -o -type l -o -type d \
    | grep -vf .gitignore \
    | grep -v -e ".git/*" -e ".git" -e "package.sh" -e pf_*.tar.gz \
    | tar cvzf pf_addcomb-$(ver).tar.gz --no-recursion -T -