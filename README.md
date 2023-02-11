[![Library Tests](https://img.shields.io/github/actions/workflow/status/KennethTrecy/chearmyp_parser/library.yml?style=for-the-badge)](https://github.com/KennethTrecy/chearmyp_parser/actions/workflows/library.yml)
![GitHub lines](https://img.shields.io/github/license/KennethTrecy/chearmyp_parser?style=for-the-badge)
![GitHub release (latest SemVer)](https://img.shields.io/github/v/release/KennethTrecy/chearmyp_parser?style=for-the-badge&display_name=tag&sort=semver)
![GitHub closed issues count](https://img.shields.io/github/issues-closed/KennethTrecy/chearmyp_parser?style=for-the-badge)
![GitHub pull request count](https://img.shields.io/github/issues-pr-closed/KennethTrecy/chearmyp_parser?style=for-the-badge)
![Commits since latest version](https://img.shields.io/github/commits-since/KennethTrecy/chearmyp_parser/latest?style=for-the-badge)
![Lines of code](https://img.shields.io/tokei/lines/github/KennethTrecy/chearmyp_parser?style=for-the-badge)
![GitHub code size in bytes](https://img.shields.io/github/repo-size/KennethTrecy/chearmyp_parser?style=for-the-badge)

# Chearmyp
This is an experimental, general purpose, human-readable, language. In this context, general purpose
means that it can be used as a markup, programming, command, and more.

## Origin
Some parts of the repository was based from [`filled_bare_metal`] branch of [Feo Template].

The mechanism of the parser was based on [Chearmyp reference].

## Parser
This repository contains a parser library for Chearmyp which represents the source as syntax tree.

## Lexer
The lexer has been forked. Visit the [repository of the lexer] for more details.

## Usage

### Installation
Add it to the dependencies:
```
[dependencies.chearmyp_parser]
git = "https://github.com/KennethTrecy/chearmyp_parser"
tag = "v1.0.0"
```

You may also activate all the features:
```
[dependencies.chearmyp_parser]
git = "https://github.com/KennethTrecy/chearmyp_parser"
tag = "v1.0.0"
features = ["no_std"]
```

You generate the documentation by the running following code below:
```
cargo doc --all-features --open
```

### Initialization
If you want to contribute, this repository should be initialized to adhere in [Conventional Commits specification] for organize
commits and automated generation of change log.

#### Prerequisites
- [Node.js and NPM]
- [pnpm] (optional)

#### Instructions
By running the command below, all your commits will be linted to follow the [Conventional Commits
specification].
```
$ npm install
```

Or if you have installed [pnpm], run the following command:
```
$ pnpm install
```

To generate the change log automatically, run the command below:
```
$ npx changelogen --from=[tag name or branch name or commit itself] --to=master
```

## Notes

### License
The repository is licensed under [MIT].

### Want to contribute?
Read the [contributing guide] for different ways to contribute in the project.

### Author
Chearmyp Parser was created by Kenneth Trecy Tobias.

[`filled_bare_metal`]: https://github.com/KennethTrecy/feo_template/tree/filled_bare_metal
[Feo Template]: https://github.com/KennethTrecy/feo_template
[repository of the lexer]: https://github.com/KennethTrecy/chearmyp_lexer
[MIT]: https://github.com/KennethTrecy/chearmyp_parser/blob/master/LICENSE
[Node.js and NPM]: https://nodejs.org/en/
[pnpm]: https://pnpm.io/installation
[Conventional Commits specification]: https://www.conventionalcommits.org/en/v1.0.0/
[contributing guide]: ./CONTRIBUTING.md
[Chearmyp reference]: https://github.com/KennethTrecy/chearmyp_reference
