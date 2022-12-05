#!/bin/sh
# Cleanup temp file
cleanup () {
  trap - EXIT
  if [ -e "$tmpfile" ] ; then rm -rf "$tmpfile"; fi
}
trap 'cleanup' EXIT
trap 'cleanup HUP' HUP
trap 'cleanup TERM' TERM
trap 'cleanup INT' INT

# Load SESSION_TOKEN variable
eval $(cat $(dirname $0)/.env)

if [ -z $SESSION_TOKEN ]; then
    echo "SESSION_TOKEN variable not defined in $(dirname $0)/.env"
    exit 1
fi

# Read inputs and set day
day=$(( $(date +%d) ))
part=$(($1))
answer=$2

if [ -z $part ] || [ -z $answer ]; then
    echo "Usage: ${0} PART ANSWER"
    exit 2
elif [[ $part != 1 ]] && [[ $parr != 2 ]]; then
    echo "PART must be 1 or 2"
    exit 2
fi

url="https://adventofcode.com/2022/day/${day}/answer"

# Do the stuff
tmp_file=$(mktemp -t aoc_ans.XXXXX)

curl -b session=${SESSION_TOKEN} -X POST -H "Content-Type: application/x-www-form-urlencoded" --data "level=${part}&answer=${answer}" ${url} > $tmp_file

if grep "That's not the right answer" ${tmpfile}; then
    echo "The answer (${answer}) is wrong!"
elif grep "too high" ${tmp_file}; then
    echo "The answer (${answer}) is too high!"
elif grep "too low" ${tmp_file}; then
    echo "The answer (${answer}) is too low!"
else
    echo "TODO get 'right answer' grep pattern"
fi

