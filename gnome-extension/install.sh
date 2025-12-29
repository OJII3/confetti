#!/bin/sh
set -e

EXTENSION_DIR="$HOME/.local/share/gnome-shell/extensions/confetti@ojii3.github.com"

mkdir -p "$EXTENSION_DIR"
cp extension.js "$EXTENSION_DIR/"
cp metadata.json "$EXTENSION_DIR/"

echo "Installed to $EXTENSION_DIR"
echo ""
echo "To enable the extension:"
echo "  gnome-extensions enable confetti@ojii3.github.com"
echo ""
echo "After enabling, fire confetti with:"
echo "  ./confetti"
echo "  or"
echo "  gdbus call --session --dest com.github.ojii3.Confetti --object-path /com/github/ojii3/Confetti --method com.github.ojii3.Confetti.Fire"
