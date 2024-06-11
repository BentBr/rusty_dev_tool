default: help

help:
	@echo ""
	@echo "Please use \033[32mmake \033[32m<target>\033[39m where <target> is one of"
	@echo "  \033[32m help \033[39m                 Shows this help"
	@echo "  \033[32m build-copy \033[39m           Builds and releases the project. Copies the binary to /usr/local/bin/rdt"
	@echo "  \033[32m clippy \033[39m           	Runs a very pedantic linter on the project"
	@echo ""

build-copy:
	cargo build --release && cp target/release/rusty_dev_tool /usr/local/bin/rdt && ls -lah /usr/local/bin/rdt && echo "\033[32mBuilt and copied!\033[39m"

clippy:
	cargo clippy -- -D clippy::all -D warnings -D clippy::pedantic -D clippy::nursery