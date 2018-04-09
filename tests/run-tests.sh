#!/bin/bash

T_TEST_FILE="$(readlink -e $0)"
T_TEST_DIR="$(dirname $T_TEST_FILE)"
T_BASE_DIR="$(dirname $T_TEST_DIR)"
T_BINARY="${T_BASE_DIR}/target/debug/earmms_keyderiv"
source "${T_TEST_DIR}/util.sh"

if [ -z "$LISTEN_FD" ]; then
	s_die "Tests must be run under catflap (or something that provides LISTEN_FD)"
fi

if [ ! -f "${T_BINARY}" ]; then
	s_die "The earmms_keyderiv binary was not found at ${T_BINARY}!"
fi

FAILED_TESTS=0
TESTS_TO_RUN=$(ls "${T_TEST_DIR}" | grep -Pxe '^test-\d{2}-[A-Za-z0-9_]+\.sh$' | tr '\n' ' ')
for i in $TESTS_TO_RUN; do
	if bash -c "export T_BINARY='${T_BINARY}'; cd ${T_TEST_DIR}; source util.sh; source $i; s_run_test || exit 1"; then
		s_info "${i}: success"
	else
		FAILED_TESTS=$(($FAILED_TESTS + 1))
		s_error "${i}: failure"
	fi
done

s_status "Test run complete, ${FAILED_TESTS} failed."
if [ $FAILED_TESTS -ne 0 ]; then
	s_die "Some tests failed!"
fi
