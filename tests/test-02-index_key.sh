index_form_data='{"mode":"index","table":"test","column":"test"}'
curl_opts=("-qs" "-H" 'Content-Type: application/json')

s_run_test() {
	# unset keys, just in case
	unset INDEX_KEY
	unset ENCRYPT_KEY

	# set keys
	export INDEX_KEY="3c480bd683b364580c01f2d44411e0d7ecc70e95f512e2eb876463d42d4d17d8"
	expected="25aabf0426baa2b2321e5ce9b7264e1f0bff5ae7d7354572e7b8331a536f8564"

	# start keyderiv
	$T_BINARY >/dev/null 2>/dev/null &
	pid=$!

	# check that index key gen works
	output="$(curl "${curl_opts[@]}" -d "${index_form_data}" localhost:5000)"
	if [ "$output" != "$expected" ]; then
		kill -9 $pid
		s_error "index: key mismatch"
		return 1
	fi

	# clean up
	kill -9 $pid
	return 0
}
