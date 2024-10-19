#!/bin/zsh
set -e;

export DOT_FILES=${DOT_FILES:-~/.dotfiles};

REPO=${REPO:-pavandv/dot-files}
REMOTE=${REMOTE:-https://github.com/${REPO}.git}
BRANCH=${BRANCH:-master}

if [ -d $DOT_FILES ]; then
	rm -rf $DOT_FILES;
fi;

if ! type "brew" >&/dev/null; then
	echo "Installing HomeBrew..."
	/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/master/install.sh)"

	BREW_STATUS=$?;
	[ $BREW_STATUS -eq 0 ] || exit 1;
fi;

#echo "Installing Git Credential Manager Core..."
#brew tap microsoft/git && \
#brew install --cask git-credential-manager

echo "cloning $REMOTE to ($DOT_FILES)..."
git clone --depth=1 --branch "$BRANCH" "$REMOTE" "$DOT_FILES";
