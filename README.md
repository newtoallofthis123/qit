# Qit

Qit is a simple and easy to use command line wrapper for git. It is aimed at beginners who want to use git, but sometimes mess up the commands. Qit is a wrapper around git, so you can use all the git commands you already know.

## Installation

With the following command you can install qit:

```bash
git clone https://github.com/newtoallofthis123/qit
cd qit
cargo build --release
```

I am currently working on publishing it to crates.io, so you can install it with cargo, however, till then, the "recommended" way to install it is to clone the repository and build it yourself.
It is quite a small binary, so it should not take too long to build.

### Windows

I primarily work on a windows machine, so I can guarantee that it works on windows. However, I have not tested it on linux or mac, so I cannot guarantee that it works on those platforms.

You can also get the windows binary from the [releases](/releases) page.

## Why qit?

Well, I initially made this for myself, because I sometimes mess up the git commands, and I wanted to make it easier for myself. 

Qit solves this problem intuitively by exposing only the most used git commands, and hiding the rest. This makes it easier for beginners to use git, without having to remember all the commands.

All the possible places you might mess up, like not adding a commit message, or commiting with unstaged changes, are handled by qit, so you don't have to worry about them.

Moreover, it provides selection menus for all the commands, so you don't have to remember the commands, and can just select the option you want.

## Usage

Qit is a wrapper around git, so you can use all the git commands you already know. However, qit also provides some additional commands, which are not available in git.

### Commands

> ❗ - Not implemented yet

#### `qit init`

Initializes a new git repository in the current directory with the following files:

- `.gitignore`
- `README.md`
- `LICENSE`
- `commit.sh`

#### `qit add`

Adds the specified files to the staging area. If no files are specified, it adds all the files to the staging area.

#### `qit commit`

Commits the staged changes. If there are no staged changes, it will ask you to stage the changes first.

#### `qit push`

Pushes the changes to the remote repository. If there are no changes to push, it will ask you to commit the changes first.

#### ❗`qit pull`

Pulls the changes from the remote repository. If there are any local changes, it will ask you to commit them first.

#### ❗`qit status`

Shows the status of the repository.

#### ❗`qit log`

Shows the commit log.

#### `qit branch` || `qit checkout`

Shows the current branch and also allows you to switch branches.

#### ❗`qit merge`

Merges the specified branch into the current branch.

#### ❗`qit clone`

Clones the specified repository.

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

## License

Qit is licensed under the [MIT License](/LICENSE).