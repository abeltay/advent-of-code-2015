value=$1

echo $value > day.txt
cp -r template $value

echo "
[[bin]]
name = \"$value\"
path = \"$value/main.rs\"" >> cargo.toml
