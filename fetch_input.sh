#!/bin/sh

day=$(TZ='EST5' date +%d)
out="$(dirname $0)/inputs/input${day}.txt"
url="https://adventofcode.com/2022/day/$((10#$day))/input"

eval $(cat $(dirname $0)/.env) # .env should contain the line `SESSION_TOKEN=yoursessiontoken`
curl -b session=${SESSION_TOKEN} ${url} > $out && {
    echo -e "\nFetched input for day $((10#$day)) to ${out}:"
    head $out
    echo ...
}

