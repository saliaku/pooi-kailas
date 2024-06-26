#!/bin/sh

# Check if the system is Linux
if [ $(uname) != 'Linux' ]; then
    echo "This uninstall script is designed for linux only, sorry!"
    exit 1
fi

set -e  # Exit immediately if any command exits with a non-zero status

# Define text formatting variables for colorful output
RED='[1;31m'
GREEN='[1;32m'
YELLOW='[1;33m'
CYAN='[1;36m'
BOLD='[1m'
RESET='[0m'

# Print a cool banner
printf '\n%b' $CYAN && cat << 'EOF'
      ▪  ▄▄
▪     ██ ██▌
 ▄█▀▄ ▐█·▐█·
▐█▌.▐▌▐█▌.▀
 ▀█▄▀▪▀▀▀ ▀
EOF
printf '%b' $RESET

# Check if running as root
if [ $(id -u) = 0 ]; then
    printf "%bwarning:%b please don't run random scripts you find on the internet as root!\n" $YELLOW $RESET
    printf '%bsudo or doas will be used when elevated privileges are required%b\n' $BOLD $RESET
    exit 1
fi

# Check if 'pooi' is in the system's PATH
if ! command -v pooi >/dev/null ; then
    printf '%berror:%b can not find %boi%b in your $PATH, are you sure that it is installed?\n' $RED $RESET $BOLD $RESET
    exit 1
fi

# Prompt user for confirmation before proceeding with uninstallation
printf "%balert!%b are you sure that you wish to remove pooi from your system? [Y/n] " $YELLOW $RESET
while true; do
    read yn
    case $yn in
        [Yy]* ) break;;
        [Nn]* ) exit;;
        * ) echo "Please answer Y/y or N/n";;
    esac
done

# Determine the available privilege escalation command (sudo or doas)
if command -v sudo >/dev/null 2>&1; then
    PRIV_ESC='sudo'
elif command -v doas >/dev/null 2>&1; then
    PRIV_ESC='doas'
else
    printf '%berror:%b can not find %bsudo%b or %bdoas%b in your $PATH, one of these is required\n' $RED $RESET $BOLD $RESET $BOLD $RESET
    exit 1
fi

# Get the location of 'pooi' executable
LOC=$(command -v pooi)

# Remove files associated with 'pooi' using elevated privileges
printf '\n%bremoving files%b (elevated privileges are required)\n\n' $GREEN $RESET
$PRIV_ESC rm -v $LOC
$PRIV_ESC rm -v /usr/share/zsh/site-functions/_oi
$PRIV_ESC rm -v /usr/share/bash-completion/completions/pooi
$PRIV_ESC rm -v /usr/share/fish/vendor_completions.d/pooi.fish

# Check if the removal was successful
if [ ! -f $LOC ]; then
    printf '\n%bDONE:%b done removing files!\n' $GREEN $RESET
else
    printf '\n%bERROR:%b could not remove the executable at $LOC, you may need to remove it manually\n' $RED $RESET
fi
