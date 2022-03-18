for D in */
do
	echo "working on $D"
	pushd $D
	./generate.sh
	# cargo build --release
	cargo publish
	popd
done
