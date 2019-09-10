#!/usr/bin/env bash

ver() {
    cat package-version.txt | tr -d '\n'
}

if [[ "$OSTYPE" == "linux-gnu" ]]; then
	find . -printf "%P\n" \
    		-type f -o -type l -o -type d \
    		| grep -vf .gitignore \
    		| grep -v -e ".git/*" -e ".git" -e "package.sh" -e pf_*.tar.gz \
    		| tar cvzf addcombq-$(ver).tar.gz --no-recursion -T -
elif [[ "$OSTYPE" == "darwin"* ]]; then
	# MacOS
	find . -type f -exec stat -f "%N" {} \; \
		| grep -vf .gitignore \
		| grep -v -e ".git/*" -e ".git" -e "package.sh" -e pf_*.tar.gz -e "mac_utils/*" \
		| tar cvzf addcombq-$(ver).tar.gz --no-recursion -T -
fi
