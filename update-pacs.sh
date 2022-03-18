for D in */
do
	echo "working on $D"
	pushd $D
	./generate.sh
	popd
done
