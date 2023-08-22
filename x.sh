#!/usr/bin/env bash

# Colors.
readonly WHITE="\033[1;97m" GREEN="\033[1;92m" RED="\033[1;91m" YELLOW="\033[1;93m" BLUE="\033[1;94m" OFF="\033[0m" TITLE="==============================================================>"

# Logging.
title() {
	printf "\n${BLUE}${TITLE}${WHITE} %s${OFF}\n" "$1"
}
fail() {
	printf "${RED}${TITLE}${WHITE} %s${OFF}\n" "$1"
	exit 1
}
ok() {
	printf "${GREEN}${TITLE}${WHITE} %s${OFF}\n" "$1"
}
finish() {
	printf "\n\n\n${GREEN}${TITLE}${WHITE} %s${OFF}\n" "Festival Build OK."
}

# Help message.
help() {
	echo "./x.sh [ARG]"
	echo ""
	echo "Lint/test/build all packages in the Festival repo."
	echo "Builds are done with --release mode."
	echo ""
	echo "Arguments:"
	echo "    c | clippy    lint all packages"
	echo "    t | test      test all packages"
	echo "    b | build     build all packages"
	echo "    a | all       do all the above"
	echo "    h | help      print help"
}

# Clippy.
clippy() {
	for i in {festival-gui,festivald,festival-cli}; do
		title "Clippy [${i}]"
		if cargo clippy -r -p ${i} -p shukusai --no-deps; then
			ok "Clippy [${i}] OK"
		else
			fail "Clippy [${i}] FAIL"
		fi
	done

	i=rpc
	title "Clippy [${i}]"
	if cargo clippy -r -p ${i} --no-deps; then
		ok "Clippy [${i}] OK"
	else
		fail "Clippy [${i}] FAIL"
	fi
}

# Test.
test() {
	for i in {festival-gui,festivald,festival-cli}; do
		title "Test [${i}]"
		if cargo test -r -p ${i} -p shukusai; then
			ok "Test [${i}] OK"
		else
			fail "Test [${i}] FAIL"
		fi
	done

	# Special cases
	for i in {'rpc','festival-gui -- --ignored --exact watch::watch::tests::signals','festival-cli -- --ignored'}; do
		title "Test [${i}]"
		if cargo test -r -p ${i}; then
			ok "Test [${i}] OK"
		else
			fail "Test [${i}] FAIL"
		fi
	done
}

# Build.
build() {
	for i in {festival-gui,festivald,festival-cli}; do
		title "Build [${i}]"
		if cargo build -r -p ${i} -p shukusai; then
			ok "Build [${i}] OK"
		else
			fail "Build [${i}] FAIL"
		fi
	done

	finish
	ls -al --color=always target/release/festival{"",d,-cli}
}

# Do everything.
all() {
	clippy
	test
	build
}

# Subcommands.
case $1 in
	'a'|'all') all;;
	'c'|'clippy') clippy;;
	't'|'test') test;;
	'b'|'build') build;;
	*) help;;
esac
