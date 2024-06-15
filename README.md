# Rusty Dev Tool
This is a little helper for developers using docker-compose (or mutagen-compose) for local development and kubectl for remote interaction.
The idea is to have common and repetitive tasks simplified.
Furthermore, it should reduce the complexity for new developers or developers that are not familiar with the setup (or are no backend developers / devops).

The general expected (and suggested) tech stack is something locally with docker-compose (so no language or framework installation locally - everything via docker images) and a remote k8s cluster. \
The crucial idea is to **NO PROJECT SPECIFIC CONFIG DONE LOCALLY**. Everything should be done via the project's docker-compose file and must be run-able after a _git checkout_.

## TL;DR

Run the following commands to interact with your docker-compose setup or shell into your remote kubernetes setup. 
- `rdt start` (starting and installing docker-compose setup)
- `rdt stop` (stopping docker-compose setup)
- `rdt shell` (shelling into docker-compose setup)
- `rdt db` (getting a local dump into your container)
- `rdt chown` (chowning inside the container for debugging)
- `rdt build` (building docker image locally)

After starting, you can simply run your local web project via a proper domain: `http://my-project.docker` (if setup correctly 😜 )\
This tool is not only for web projects but can be used for any kind of project which is using docker-compose.

## Installation and Setup
### Installation
Get the latest release fitting to your platform from [here](https://github.com/BentBr/rusty_dev_tool/releases/latest). \
After downloading the binary, make it executable and move it to a location in your PATH.

#### Unix
`cp ~/Downloads/rdt-macos-%bitness%-v%release% /usr/local/bin/rdt` \
We are using here the default path for the local user: `/usr/local/bin/`.

### Uninstallation
Just remove the binary from your PATH. \
`rm /usr/local/bin/rdt`

### Pre-Requirements for usage
In the most basic setup you need to have the following tools installed:
- [docker](https://www.docker.com/products/docker-desktop/) (you don't need the desktop app. [Docker engine](https://docs.docker.com/engine/install/) is enough)
- [docker-compose](https://docs.docker.com/compose/install/)
- [local routing](docs/local-routing-setup.md) setup (to install a local domain to be used for your projects) \
Strictly spoken not needed but highly recommended as web projects are often using domains, and it's very convenient for develops.

The following tools are optional but recommended for further functionality:
- [mutagen](https://mutagen.io/documentation/introduction/installation) + [mutagen-compose](https://mutagen.io/documentation/orchestration/compose) (for faster file sync on older macOS versions)
- [kubectl](https://kubernetes.io/docs/reference/kubectl/) (for remote development)
- ~~SequelAce for database access within your containers~~ Todo: Find a better solution for all OS available ?!

### General Configuration of RDT
When you first run RDT it will add the default configuration into your home directory.
This configuration file is located at `~/.rusty-dev-tool/config.toml`.
If by accident you delete this file, you can recreate it by running `rdt --config-restore`.

In your current project RDT will check for a local config in `%project-root%/.rusty-dev-tool/config.toml`. Those entries will override the ones in the global config.

See the example config files for the [home config](examples/configs/home_config.toml) and the [project config](examples/configs/project_config.toml).
#### Naming
It's not needed to have RDT named as `rdt`. You can rename the binary to whatever you like. \
Just make sure to update the home config: `rdt_name="mfc"` if you work for _my fancy company_ and decided to have your internal tool used as such. \
On Unix systems you only need to rename the file in respective binary folder. \
`mv /usr/local/bin/rdt /usr/local/bin/mfc`

Now, you can use rdt like: `mfc start` or `mfc shell`.

#### Updating from custom fork / repo
If you don't want to use the official repository for updates, you can set the following in your config: \
`download_path="https://my-own-repo.com/rdt/releases/download"` \
and \
`meta_path="https://api.my-own-repo.com/repos/rdt/releases/latest"`

The download path builds the key like: https://github.com/BentBr/rusty_dev_tool/releases/download/v0.2.0/rdt-macos-aarch64-v0.2.0 \
And the meta path checks for the `tag_name` in the json response.

Please check the [release workflow](.github/workflows/release.yaml) for it.

## Available Commands
All commands which are running _docker-compose_ do check before if x-mutagen is configured and will run _mutagen-compose_ if so.

### start
Starting the setup for local development. This will start the _docker-compose_ setup and the _mutagen-compose_ sync.

Under the hood it will run the following commands: \
`docker-compose pull && docker-compose up -d --build && docker-compose exec -T php composer install`

It is checking for the environment in compose.yaml: `MAIN_SERVICE=php|node|rust` \
(The list gets extended as needed)

### stop
Stopping the setup for local development. \
Directly runs `docker-compose down`

### shell
For shelling into a container locally. \
Mostly runs `docker-compose exec php|rust|node bash`

An optional argument can be passed to shell into a different container. \
`rdt shell node`

### db
Bringing in a local dump into the container's database. \
`rdt db` fetches dump.sql from the project's root folder. \
`rdt db my-dump/file.sql.gz` fetches the specified file relative to the current working directory.

It automatically checks if the dump is zipped (.gz) and unzips it before importing.

### chown
Chowning the project folder to the user and group of the container. \
Especially handy if different actions (IDE vs webserver) are running with different users. 

As per default it's chowning to the www-data:www-data user and group. \
It takes an optional argument such as `rdt chown root:root` to chown to the root user and group.

### build
Building the docker image locally. \
Runs `docker buildx build .`

### help
Find out what commands exist and what you can do with this tool.

## Arguments
### --self-update
Updates the tool to the latest version.

### --help
Same as the help subcommand.

### --generate-completions
Generates an auto-completion script for RDT in your current terminal. Available shells are: bash, zsh, fish, PowerShell... \
All official commands are supported. Additionally, those from the [current config](examples/configs/home_config.toml) will be taken as well (If you are in a project dir with local commands those will be taken as well).

It takes into account if you changed the name of your binary and reads from the home config file the `rdt_name`.

#### Usage
`rdt --generate-completions bash > ~/.zsh/auto_completion.sh` \
Then, you have to source this script as it is being loaded on shell start: `source ~/.zsh/auto_completion.sh` \
Add this command to your shell's profile file to have it loaded on every shell start (like: `~/.zprofile, ~/.bash_profile, ~/.zshrc, etc.`.

Don't forget to adapt to your current shell which you can find out via `echo $0` (Unix).

## OS
Currently, Linux and macOS for amd64 / arm64 are supported (Unix in general).
Windows is not supported at the moment. The build is just included for future use.

## Examples
See the [examples](examples) folder for some example setups.
Please provide your own examples as well as this tool is meant to be used for different setups.

## Contribution
❤️ If you want to contribute to this project, feel free to open a pull request. ❤️

All commits to this repository must follow the [conventional commits](https://www.conventionalcommits.org/) standard. Version numbers for this tool are generated automatically based on the commit types used.
Please use the following commit types:
- `feat`: for new features
- `fix`: for bug fixes
- `chore`: for code refactoring
- `docs`: for documentation changes
- `test`: for adding and enhancing tests

Furthermore, please make sure to add a proper description to your commit message. PRs must successfully pass tests + clippy checks. Make sure to cover your changes accordingly with tests.

See the custom commands in config toml for some helpers. \
We are using our beloved tool for local building and linting.
`rdt help` for details.

### Test coverage
Install grcov::
- `cargo install grcov` Install the coverage report generator
- `rustup component add llvm-tools-preview` component to check usages during test
- `rustup install nightly && rustup default nightly` nightly build for instrument-coverage usage

Then, you can run the tests with coverage: `rdt test-coverage`

## Official todos on the roadmap (the next ones)
- Adding more and basic tests (WIP)
- Adding support for environments (shelling into remote k8s setups)
- Removing the command list hashmap (as it doubles the registry). Add command descriptions to Executors
- Adding support for windows... (maybe). Dunno what works atm and what not ;) \
  Known things that will break:
  - Colours in terminal
  - fetching the correct console (so no exec works)
  - auto-complete generation
- Update the logic and fields for custom commands:
  - command alias in toml node
  - command as is
  - description additional (refactor with the registry)
- ~~Adding a check to run via current terminal and not always sh(sh, bash, zsh, fish, PowerShell, cmd)~~ ✅
- ~~Adding support for auto completions on terminals~~ ✅ 
- ~~Sorting of all commands for help menu~~ ✅
- ~~Adding some generic db connection option(s)~~ ✅
- ~~Adding docker build alias for local Dockerfile (to only build the image)~~ ✅
- ~~Add an option (config) to only have aliases (custom commands) without compose.yaml config mandatory~~  ✅
- ~~Adding basic commands: start, stop, shell~~ ✅
- ~~Adding update functionality~~ ✅
- ~~Testing during CI and have those being a blocking factor~~ ✅
- ~~Adding support for custom commands~~ ✅
- ~~Adding arguments for certain commands (such as chown)~~ ✅
- ~~fix self-update if release is not built but tag published... -> leads to empty binary file~~ ✅
