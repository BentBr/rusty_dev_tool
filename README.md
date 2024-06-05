# Rusty Dev Tool
This is a little helper for developers using docker-compose (or mutagen-compose) for local development and kubectl for remote development.
The idea is to have common and repetitive tasks simplified.
Furthermore, it should reduce the complexity for new developers or developers that are not familiar with the setup (or are no backend developers / devops).

## Installation and Setup
### Pre-Requirements for installation
In the most basic setup you need to have the following tools installed:
- docker
- docker-compose

The following tools are optional but recommended:
- mutagen + mutagen-compose (for faster file sync on older macOS versions)
- kubectl (for remote development)
- local routing setup (see here how to set up)
- SequelAce for database access within your containers


### General Configuration
When you first run RDT it will add the default configuration into your home directory.
This configuration file is located at `~/.rusty-dev-tool/config.toml`.
If by accident you delete this file, you can recreate it by running `rdt init-config`.

In your current project RDT will check for a local config in `%project-root%/.rusty-dev-tool/config.toml`. Those entries will override the ones in the global config.

### Mac

### Linux


## Available Commands

## OS
Currently Linux and MacOS for amd64 / arm64 are supported.
Windows is not supported at the moment. The build is just included for future use.

## Contribution
❤️ If you want to contribute to this project, feel free to open a pull request. ❤️

All commits to this repository must follow the [conventional commits](https://www.conventionalcommits.org/) standard. Version numbers for this tool are generated automatically based on the commit types used.
Please use the following commit types:
- `feat`: for new features
- `fix`: for bug fixes
- `chore`: for code refactoring
- `docs`: for documentation changes

Furthermore, please make sure to add a proper description to your commit message. PRs must successfully past tests + clippy checks. Make sure to cover your accordingly.



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

