#!/bin/sh

day_padded=$(TZ='EST5' date +%d)
out="$(dirname $0)/inputs/input${day_padded}.txt"
day=$((10#$day_padded))
url="https://adventofcode.com/2022/day/${day}"

# Download personal input
eval $(cat $(dirname $0)/.env) # .env should contain the line `SESSION_TOKEN=yoursessiontoken`
curl -b session=${SESSION_TOKEN} "${url}/input" > $out && {
    echo -e "\nFetched input for day ${day} to ${out}.\nPreview:"
    head $out
    echo
}

# Get day's title
title=$(curl -s ${url} | grep -m 1 "<h2>--- Day" | sed -E "s/^.*<h2>--- Day [0-9]: (.*) ---<\/h2>.*$/\1/")
echo $title

# Create source code file from template and add day to Answers.md
new_file="$(dirname $0)/src/bin/day${day}.rs"
echo "Creating new file ${new_file} from template..."
if [ ! -e $new_file ]; then
    cp "$(dirname $0)/template.rs" ${new_file}
    sed -i -E "s/#(.*)\{Title\}/${day}\1${title}/" ${new_file}
    echo -e "### Day ${day}: ${title}\n[[Description]](${url}) [[Input]](inputs/input${day_padded}.txt)  \n**Answer 1:**   \n**Answer 2:**   \n" >> \
        $(dirname $0)/Answers.md
else
    echo "${new_file} already exists."
fi

