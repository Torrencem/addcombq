#!/usr/bin/env bash

ver() {
    cat package-version.txt | tr -d '\n'
}

function prepend() { while read line; do echo "${1}${line}"; done; }

if [[ "$OSTYPE" == "linux-gnu" ]]; then
	BUILD_FILES=$(find . -printf "%P\n" \
    		-type f -o -type l -o -type d \
    		| grep -vf .gitignore \
		| grep -v -e ".git/*" -e ".git" -e "package.sh" -e pf_*.tar.gz )
	
if [ -d "src/build" ]; then
	BUILD_FILES+="$(echo " ")"
	BUILD_FILES+="$(find src/build -printf "%P\n" -type f -o -type l -o -type d | prepend "src/build/")"
fi

elif [[ "$OSTYPE" == "darwin"* ]]; then
	# MacOS
	BUILD_FILES=$(find . -type f -exec stat -f "%N" {} \; \
		| grep -vf .gitignore \
		| grep -v -e ".git/*" -e ".git" -e "package.sh" -e pf_*.tar.gz -e "mac_utils/*"\ )

if [ -d "src/build" ]; then
	BUILD_FILES+="$(echo " ")"
	BUILD_FILES+="$(find src/build -type f -exec stat -f "%N" {} \; | prepend "src/build/")"
fi

fi

echo -e "\n\n"

echo $BUILD_FILES

echo -e "\n\n"

echo "${BUILD_FILES}" | tar cvzf addcombq-$(ver).tar.gz --no-recursion --ignore-failed-read -T -
