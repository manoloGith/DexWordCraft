#!/usr/bin/env bash
# Fetch WordCraft's build input craft-i18n into .build-inputs/ (gitignored), at the revision
# upstream pins (upstream/craft-i18n.rev), and print the environment variable that makes the build
# embed it: the interface translations and the spelling and hyphenation data. Without it the
# interface is English only and spelling falls back to a small English word list.
#
# craft-fonts, WordCraft's other optional input, is left out on purpose: its faces (CJK, Arabic,
# Hebrew, Thai, Devanagari, Georgian, symbols) would add about 45 MB to the library, and Android
# ships the same Noto families in /system/fonts, where the fork's font search looks.
#
#   eval "$(scripts/fetch-build-inputs.sh)"
set -euo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
dir="$root/.build-inputs/craft-i18n"
rev="$(tr -d '[:space:]' < "$root/upstream/craft-i18n.rev")"
if [ "$(git -C "$dir" rev-parse HEAD 2>/dev/null || true)" != "$rev" ]; then
    rm -rf "$dir"
    git init --quiet "$dir"
    git -C "$dir" fetch --quiet --depth 1 https://github.com/storytold/craft-i18n "$rev"
    git -C "$dir" checkout --quiet FETCH_HEAD
fi
echo "export CRAFT_I18N_DIR=\"$dir\""
