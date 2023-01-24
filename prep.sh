day=$1

echo $day > day.txt
cp -r template $day

echo "
[[bin]]
name = \"$day\"
path = \"$day/main.rs\"" >> cargo.toml
