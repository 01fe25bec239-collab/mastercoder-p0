#!/bin/sh
# PROTOTYPE — DISPOSABLE — NOT PRODUCTION (Mastercoder freeze-closure).
set -eu
base=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd -L)
{ IFS= read -r label; IFS= read -r interpreter; } < "$base/interpreter.txt"
case "$interpreter" in /*) ;; *) echo 'Invalid configured interpreter' >&2; exit 1 ;; esac
/usr/bin/codesign --verify --strict "$base/VMOwner"
exec "$interpreter" -I -B "$base/native_control.py" prebegin-host "$base/config.json"
