#!/bin/bash

pwd=$(pwd)
debug_target="$pwd/target/debug/football-rpg"
release_target="$pwd/target/release/football-rpg"
target=$debug_target

usage() { echo "Usage: $0 [options]"; exit 1; }

while getopts "rh" o; do 
    case "${o}" in
        r)
            target=$release_target
            ;;
        h) 
            usage
            ;;
    esac
done

options="--maximize"

# start the CLI program in a new terminal
gnome-terminal $options -- bash -ic "export TARGET=$target; $target"