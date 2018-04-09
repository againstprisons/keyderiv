index_form_data='{"mode":"index","table":"test","column":"test"}'
encrypt_form_data='{"mode":"encrypt","table":"test","column":"test","row":"test"}'
curl_opts=("-s" "-o" "/dev/null" "-H" 'Content-Type: application/json' "-w" '%{http_code}')

s_run_test() {
	# unset keys, just in case
	unset INDEX_KEY
	unset ENCRYPT_KEY

	# start keyderiv
	$T_BINARY >/dev/null 2>/dev/null &
	pid=$!

	# check that index key gen fails
	output="$(curl "${curl_opts[@]}" -d "${index_form_data}" localhost:5000)"
	if [ "$output" -ne 500 ]; then
		kill -9 $pid
		s_error "index: server didn't return a 500"
		return 1
	fi

	# check that encryption key gen fails
	output="$(curl "${curl_opts[@]}" -d "${encrypt_form_data}" localhost:5000)"
	if [ "$output" -ne 500 ]; then
		kill -9 $pid
		s_error "encrypt: server didn't return a 500"
		return 1
	fi

	# clean up
	kill -9 $pid
	return 0
}
