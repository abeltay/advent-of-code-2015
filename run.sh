# !/bin/bash

day=`cat day.txt`
cd $day
cargo run --bin "$day"
