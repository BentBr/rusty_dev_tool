# Changelog

## [0.6.0](https://github.com/BentBr/rusty_dev_tool/compare/v0.5.2...v0.6.0) (2024-06-12)


### Features

* adding a little flag if compose is being used locally - maybe to set only aliases for easier working ([54d9bf0](https://github.com/BentBr/rusty_dev_tool/commit/54d9bf0af31cfc25cdc5c910f660123ad2f3b016))


### Bug Fixes

* updating readme for testing purposes of test pipeline ([#53](https://github.com/BentBr/rusty_dev_tool/issues/53)) ([135fb0c](https://github.com/BentBr/rusty_dev_tool/commit/135fb0c46c26263a7a01c9ece418be48c44282ac))
* updating tests to publish results ([10e86c3](https://github.com/BentBr/rusty_dev_tool/commit/10e86c3f8018c9ba83260844fe7469dd992a9d89))

## [0.5.2](https://github.com/BentBr/rusty_dev_tool/compare/v0.5.1...v0.5.2) (2024-06-11)


### Bug Fixes

* now, the real fix for broken download as the file override only happens if the download worked -.- ([4e94aad](https://github.com/BentBr/rusty_dev_tool/commit/4e94aad3dc2c3e05585e4486635303e3789f6233))

## [0.5.1](https://github.com/BentBr/rusty_dev_tool/compare/v0.5.0...v0.5.1) (2024-06-11)


### Bug Fixes

* removed the main branch target for PRs running for clippy ([b1cead2](https://github.com/BentBr/rusty_dev_tool/commit/b1cead24bcfd0939d7562c85b8c2ec2aad2acf51))
* Updated check for download of new tool: checking content length ([f69a4f5](https://github.com/BentBr/rusty_dev_tool/commit/f69a4f5439aa62f0924f4fa0e6b2669cf589e6b4))

## [0.5.0](https://github.com/BentBr/rusty_dev_tool/compare/v0.4.0...v0.5.0) (2024-06-09)


### Features

* added the possibility to use arguments (single ones) for all commands. Has to be added to the command implemantation ([a92f576](https://github.com/BentBr/rusty_dev_tool/commit/a92f576834b03d3a10286f24fe5d4bf2df3570e3))

## [0.4.0](https://github.com/BentBr/rusty_dev_tool/compare/v0.3.3...v0.4.0) (2024-06-09)


### Features

* Custom commands ([#46](https://github.com/BentBr/rusty_dev_tool/issues/46)) ([bc97fcd](https://github.com/BentBr/rusty_dev_tool/commit/bc97fcdfd16d547b44e5c1766d996a50411e8549))

## [0.3.3](https://github.com/BentBr/rusty_dev_tool/compare/v0.3.2...v0.3.3) (2024-06-08)


### Bug Fixes

* checking if download of new version works and failing accordingly ([2e34c8b](https://github.com/BentBr/rusty_dev_tool/commit/2e34c8b32b8f95c9e43da31f819ef62918c84189))

## [0.3.2](https://github.com/BentBr/rusty_dev_tool/compare/v0.3.1...v0.3.2) (2024-06-08)


### Bug Fixes

* changing when clippy runs ([6f26809](https://github.com/BentBr/rusty_dev_tool/commit/6f26809a24a6a4f5b3d27dd89c183170fd7aca62))
* trying official clippy documentation ([#41](https://github.com/BentBr/rusty_dev_tool/issues/41)) ([47640ff](https://github.com/BentBr/rusty_dev_tool/commit/47640ffce1f59ca411ecd1b1ca82f04884599d3c))

## [0.3.1](https://github.com/BentBr/rusty_dev_tool/compare/v0.3.0...v0.3.1) (2024-06-08)


### Bug Fixes

* fixing pipeline by renaming folders without colon : ([df161d5](https://github.com/BentBr/rusty_dev_tool/commit/df161d52322b7c1f8cbf724e15ddc9881a591b80))
* updated clippy run ([6c1ab13](https://github.com/BentBr/rusty_dev_tool/commit/6c1ab13c7f03952e6ecf6fba94a1a9f1ae5295f1))

## [0.3.0](https://github.com/BentBr/rusty_dev_tool/compare/v0.2.0...v0.3.0) (2024-06-08)


### Features

* added self-updating command with config ([d81c145](https://github.com/BentBr/rusty_dev_tool/commit/d81c145978dec8223dc351a7257ca2562f3a6aee))
* adding clippy to workflows ([c0e93d4](https://github.com/BentBr/rusty_dev_tool/commit/c0e93d451cfc4a3c58c8cf4116c3c049462cc684))
* adding differentiation to run different main containers (with install during start). ([60b821c](https://github.com/BentBr/rusty_dev_tool/commit/60b821cb538ced8b61a2480438b0dd7dc7a81238))


### Bug Fixes

* adding language_framework retrieval to chown as well ([43c2087](https://github.com/BentBr/rusty_dev_tool/commit/43c2087f703079b7d35afacbd45529ace2d6b8ec))
* removing the idea of removing assets from the realeases ([34323a9](https://github.com/BentBr/rusty_dev_tool/commit/34323a97ed7c2543d3058d997d8e93c84b539a10))
* updating shell command to always shell into the main one ([7d2aa71](https://github.com/BentBr/rusty_dev_tool/commit/7d2aa718c6fe8b54441d6a28ee14c15de463269f))

## [0.2.0](https://github.com/BentBr/rusty_dev_tool/compare/v0.1.17...v0.2.0) (2024-06-06)


### Features

* added correct version to clap ([9dda7b0](https://github.com/BentBr/rusty_dev_tool/commit/9dda7b0b331bc0cca5ad6c7b6dbb778a11d20bfe))


### Bug Fixes

* fixing the repo deletion asset job ([047f996](https://github.com/BentBr/rusty_dev_tool/commit/047f996cb4c5cd93f5ea6e864206e4d3b32a32a4))

## [0.1.17](https://github.com/BentBr/rusty_dev_tool/compare/v0.1.16...v0.1.17) (2024-06-06)


### Bug Fixes

* trying to debug the asset ids... by exporting ([6dc2aa8](https://github.com/BentBr/rusty_dev_tool/commit/6dc2aa86bcbeea99b5a82344b7d2501d43a24ef6))

## [0.1.16](https://github.com/BentBr/rusty_dev_tool/compare/v0.1.15...v0.1.16) (2024-06-06)


### Bug Fixes

* new debug for asset ids (I want to delete) ([5b5ebd3](https://github.com/BentBr/rusty_dev_tool/commit/5b5ebd35fd59c8fb94a30491fbf13c567df2e8be))

## [0.1.15](https://github.com/BentBr/rusty_dev_tool/compare/v0.1.14...v0.1.15) (2024-06-06)


### Bug Fixes

* Finally found proper release pathes ([bdfbb53](https://github.com/BentBr/rusty_dev_tool/commit/bdfbb53cb4c72b406722b3be935f7016cdd7aad0))

## [0.1.14](https://github.com/BentBr/rusty_dev_tool/compare/v0.1.13...v0.1.14) (2024-06-06)


### Bug Fixes

* more and deeper verbosity ([b42b552](https://github.com/BentBr/rusty_dev_tool/commit/b42b55299ff5295a012cb62f3d996ef010b7fa93))

## [0.1.13](https://github.com/BentBr/rusty_dev_tool/compare/v0.1.12...v0.1.13) (2024-06-06)


### Bug Fixes

* deeper ls ([efdb16f](https://github.com/BentBr/rusty_dev_tool/commit/efdb16ff7f791653118edd07abedc39538b5c1c9))

## [0.1.12](https://github.com/BentBr/rusty_dev_tool/compare/v0.1.11...v0.1.12) (2024-06-06)


### Bug Fixes

* updating ls command for insights ([d637290](https://github.com/BentBr/rusty_dev_tool/commit/d6372907a0ca8ef5ebc8609649b3d6ff51128493))

## [0.1.11](https://github.com/BentBr/rusty_dev_tool/compare/v0.1.10...v0.1.11) (2024-06-06)


### Bug Fixes

* adding more verbosity ([c1d679f](https://github.com/BentBr/rusty_dev_tool/commit/c1d679f47977312775e02ca095ae65f443c0ec85))

## [0.1.10](https://github.com/BentBr/rusty_dev_tool/compare/v0.1.9...v0.1.10) (2024-06-06)


### Bug Fixes

* getting info of files to upload ([9e20299](https://github.com/BentBr/rusty_dev_tool/commit/9e202997f71b5a284f677c9c5e2a9e57fa48db52))

## [0.1.9](https://github.com/BentBr/rusty_dev_tool/compare/v0.1.8...v0.1.9) (2024-06-06)


### Bug Fixes

* fixing upload namings ([#25](https://github.com/BentBr/rusty_dev_tool/issues/25)) ([d0af515](https://github.com/BentBr/rusty_dev_tool/commit/d0af5155097275d1dff842b348702e07a756bf8e))

## [0.1.8](https://github.com/BentBr/rusty_dev_tool/compare/v0.1.7...v0.1.8) (2024-06-06)


### Bug Fixes

* Fixed proper tag naming ([#23](https://github.com/BentBr/rusty_dev_tool/issues/23)) ([8acfccc](https://github.com/BentBr/rusty_dev_tool/commit/8acfccc97095eaff3d851f24e295a479054b74db))

## [0.1.7](https://github.com/BentBr/rusty_dev_tool/compare/v0.1.6...v0.1.7) (2024-06-06)


### Bug Fixes

* all outputs verbosity ([#21](https://github.com/BentBr/rusty_dev_tool/issues/21)) ([0d68867](https://github.com/BentBr/rusty_dev_tool/commit/0d688676ebca6b3ef0a119faa6cce7cac88e3e4a))
* next try verbosity ([7c0071b](https://github.com/BentBr/rusty_dev_tool/commit/7c0071b0e17683a19b04e561ea534464cf6d0e4c))
* next try verbosity 2 ([058a93e](https://github.com/BentBr/rusty_dev_tool/commit/058a93ed53761ea0608c363dd6ce9a20aa053aa0))

## [0.1.6](https://github.com/BentBr/rusty_dev_tool/compare/v0.1.5...v0.1.6) (2024-06-06)


### Bug Fixes

* Added all outputs ([#19](https://github.com/BentBr/rusty_dev_tool/issues/19)) ([8ca7bf5](https://github.com/BentBr/rusty_dev_tool/commit/8ca7bf5824b6ab3e571a27c93d8d35ac87be5958))

## [0.1.5](https://github.com/BentBr/rusty_dev_tool/compare/v0.1.4...v0.1.5) (2024-06-06)


### Bug Fixes

* updated the r-p action to v4 ([#17](https://github.com/BentBr/rusty_dev_tool/issues/17)) ([1abc336](https://github.com/BentBr/rusty_dev_tool/commit/1abc3365783116ea5f681763e20749944226f7e0))

### [0.1.4](https://www.github.com/BentBr/rusty_dev_tool/compare/v0.1.3...v0.1.4) (2024-06-06)


### Bug Fixes

* adding something ([#15](https://www.github.com/BentBr/rusty_dev_tool/issues/15)) ([38d7be9](https://www.github.com/BentBr/rusty_dev_tool/commit/38d7be96a72f4c8a0b2ee2399d49d33a45d93230))

### [0.1.3](https://www.github.com/BentBr/rusty_dev_tool/compare/v0.1.2...v0.1.3) (2024-06-06)


### Bug Fixes

* Made sure to properly set the input ([#12](https://www.github.com/BentBr/rusty_dev_tool/issues/12)) ([10d0bc5](https://www.github.com/BentBr/rusty_dev_tool/commit/10d0bc54b6f92116652bb84a68f8cd81f6435c72))
* removing zipped source from release + trying to add release files manually with set tags ([#11](https://www.github.com/BentBr/rusty_dev_tool/issues/11)) ([ded6196](https://www.github.com/BentBr/rusty_dev_tool/commit/ded619678a3d3cc7c9ad7322f5acdb9776e42f8a))

### [0.1.2](https://www.github.com/BentBr/rusty_dev_tool/compare/v0.1.1...v0.1.2) (2024-06-06)


### Bug Fixes

* Fixing missing trigger for releases ([#9](https://www.github.com/BentBr/rusty_dev_tool/issues/9)) ([c93cb39](https://www.github.com/BentBr/rusty_dev_tool/commit/c93cb3985d9e358d0d3fb6248b89fcecff363aff))

### [0.1.1](https://www.github.com/BentBr/rusty_dev_tool/compare/v0.1.0...v0.1.1) (2024-06-06)


### Bug Fixes

* removing not existing param from release-please job ([#6](https://www.github.com/BentBr/rusty_dev_tool/issues/6)) ([9dff6ff](https://www.github.com/BentBr/rusty_dev_tool/commit/9dff6ff24adcbc00eda0f554697803de750c7b7d))
