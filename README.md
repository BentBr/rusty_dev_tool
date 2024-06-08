# Rusty Dev Tool
This is a little helper for developers using docker-compose (or mutagen-compose) for local development and kubectl for remote development.
The idea is to have common and repetitive tasks simplified.
Furthermore, it should reduce the complexity for new developers or developers that are not familiar with the setup (or are no backend developers / devops).

## TL;DR

Run the following commands to interact with your docker-compose setup or shell into your remote kubernetes setup. 
- `rdt start`
- `rdt stop`
- `rdt shell`

## Installation and Setup
### Pre-Requirements for installation
In the most basic setup you need to have the following tools installed:
- [docker](https://www.docker.com/products/docker-desktop/) (you don't need the desktop app. [Docker engine](https://docs.docker.com/engine/install/) is enough)
- [docker-compose](https://docs.docker.com/compose/install/)
- [local routing](docs/local-routing-setup.md) setup (to install a local domain to be used for your projects) \
Strictly spoken not needed but highly recommended as web projects are often using domains and it's very convenient for developrs.


The following tools are optional but recommended for further functionality:
- [mutagen](https://mutagen.io/documentation/introduction/installation) + [mutagen-compose](https://mutagen.io/documentation/orchestration/compose) (for faster file sync on older macOS versions)
- [kubectl](https://kubernetes.io/docs/reference/kubectl/) (for remote development)
- ~~SequelAce for database access within your containers~~ Todo: Find a better solution for all OS available ?!


### General Configuration of RDT
When you first run RDT it will add the default configuration into your home directory.
This configuration file is located at `~/.rusty-dev-tool/config.toml`.
If by accident you delete this file, you can recreate it by running `rdt init-config`.

In your current project RDT will check for a local config in `%project-root%/.rusty-dev-tool/config.toml`. Those entries will override the ones in the global config.


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
`docker-compose down`

### shell
For shelling into a container locally. \
`docker-compose exec php bash`

## Arguments
### --self-update
Updates the tool to the latest version.

### --help
Same as the help subcommand.

### --generate-completions
NOT YET IN
Generates shell completions for the tool. Available shells are: bash, zsh, fish, powershell... TBD

## OS
Currently, Linux and MacOS for amd64 / arm64 are supported.
Windows is not supported at the moment. The build is just included for future use.

## Examples
See the [examples](examples/) folder for some example setups.
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

Furthermore, please make sure to add a proper description to your commit message. PRs must successfully past tests + clippy checks. Make sure to cover your accordingly.

##
##
##
##

############################# not yet implemented #############################
## Environments
- config for different envs
  - local
    - docker-compose
    - mutagen-compose
  - remote (with different urls + namings)
    - kubectl
    - ssh
    - oidc
  - tests (separate runs)
    - stan
    - phpunit
  - update path for builds
  - custom scripts

## Commands

- shell (with default option)
  - local
  - remote
    - multiple ones
- start
  - local
- stop
  - local
- clear removes db
  - local
- fix
- self-update
- help (list)

