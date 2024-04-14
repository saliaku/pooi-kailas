#!/bin/sh

# Check if the system is Linux
if [ $(uname) != 'Linux' ]; then
    echo "This script is designed for linux only, sorry!"
    exit 1
fi

set -e

# Define text formatting variables
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

# Check for cargo (Rust package manager)
if !command -v cargo >/dev/null 2>&1; then
    printf '%berror:%b can not find %bcargo%b in your $PATH, please ensure it is correctly installed\n' $RED $RESET $BOLD $RESET
    exit 1
fi

# Determine the available privilege escalation command (sudo or doas)
if command -v sudo >/dev/null 2>&1; then
    PRIV_ESC='sudo'
elif command -v doas >/dev/null 2>&1; then
    PRIV_ESC='doas'
else
    printf '%berror:%b can not find %bsudo%b or %bdoas%b in your $PATH, one of these is required\n' $RED $RESET $BOLD $RESET $BOLD $RESET
    exit 1
fi

# Navigate to the script's directory
cd "$(dirname "$0")"

# Build the binary using cargo
printf '%bSTEP 1:%b %bbuilding the binary%b (this may take a few minutes)\n\n' $GREEN $RESET $BOLD $RESET
cargo build --release
# Strip debug symbols from the binary if strip is available
command -v strip >/dev/null 2>&1 && strip -s ./target/release/pooi

# Copy necessary files to system directories
printf '\n%bSTEP 2:%b %bcopying files%b (elevated privileges are required)\n\n' $GREEN $RESET $BOLD $RESET
$PRIV_ESC install -Dvm755 ./target/release/pooi /usr/local/bin/pooi
$PRIV_ESC install -Dvm644 ./etc/completions/_oi /usr/share/zsh/site-functions/_oi
$PRIV_ESC install -Dvm644 ./etc/completions/pooi.bash /usr/share/bash-completion/completions/pooi
$PRIV_ESC install -Dvm644 ./etc/completions/pooi.fish /usr/share/fish/vendor_completions.d/pooi.fish

# Print completion message
printf '\n%bDONE:%b %bthanks for testing! %b<3%b (this repo is no longer needed and can be deleted)\n' $GREEN $RESET $BOLD $RED $RESET
