#!/bin/bash
# This script moves the git hooks in this directory to the .git/hooks directory.

echo "Moving git hooks to .git/hooks".
if [ -d "../.git/hooks" ]; then
	echo "Moving git hooks to .git/hooks"
	mv git-hooks/* .git/hooks/
else
	if [ -d ".git/hooks" ]; then
		echo "Moving git hooks to .git/hooks"
		mv git-hooks/* .git/hooks/
	else
		echo "No .git/hooks directory found, please, make sure that you're in the root/git-hooks directory when using this script."
		exit 1
	fi
fi