encrypt_form_data='{"mode":"encrypt","table":"test","column":"test","row":"test"}'
curl_opts=("-qs" "-H" 'Content-Type: application/json')

s_run_test() {
	# unset keys, just in case
	unset INDEX_KEY
	unset ENCRYPT_KEY

	# set keys
	export ENCRYPT_KEY="45347dc09ea6826c844df48ab6d989ea75030402165348be27a7a313cf3a9869"
	expected="7869589844fc734d10014ce3773d701b14ba46d3df4ddc4ccca4d0975a0fae30"

	# start keyderiv
	$T_BINARY >/dev/null 2>/dev/null &
	pid=$!

	# check that index key gen works
	output="$(curl "${curl_opts[@]}" -d "${encrypt_form_data}" localhost:5000)"
	if [ "$output" != "$expected" ]; then
		kill -9 $pid
		s_error "encrypt: key mismatch"
		return 1
	fi

	# clean up
	kill -9 $pid
	return 0
}
