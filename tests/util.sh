#!/bin/bash

S_COLOR_RED="\033[31m"
S_COLOR_GREEN="\033[32m"
S_COLOR_WHITE="\033[37m"
S_COLOR_RESET="\033[0m"

s_error() {
	echo -n -e "${S_COLOR_WHITE}[ ${S_COLOR_RED}!!${S_COLOR_WHITE} ]${S_COLOR_RESET} "
	echo "$@"
}

s_die() {
	s_error "$@"
	exit 1
}

s_info() {
	echo -n -e "${S_COLOR_WHITE}[    ]${S_COLOR_RESET} "
	echo "$@"
}

s_status() {
	echo -n -e "${S_COLOR_WHITE}[ ${S_COLOR_GREEN}**${S_COLOR_WHITE} ]${S_COLOR_RESET} "
	echo "$@"
}
