#!/bin/sh

day=`cat day.txt`
url_day=$(echo $day | sed 's/^0*//')
cookie=`cat cookie.txt`

curl --location --request GET "https://adventofcode.com/2015/day/$url_day/input" \
	--header "Cookie: $cookie" --output "$day/input.txt"

