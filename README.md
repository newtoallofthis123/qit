# Quick Git (Qit)

Qit is a simple CLI tool written in rust that tries to make the git experience a little bit easier. It is _not_ a replacement for git, but rather a wrapper around it. It is meant to be used in conjunction with git, not instead of it.

Qit has a few simple commands that make it easier to do some common git tasks. Like for example, `qit init` will create a git repository and automatically prompt you for some details and create a README.md file, a license file, and a .gitignore file. It will also give you the option to create a remote repository on GitHub and push your local repository to it.

## Installation

### From Source

To install from source, you will need to have [Rust](https://www.rust-lang.org/) installed. Once you have that, you can clone this repository and run `cargo install --path .` from the root of the repository.

```bash
git clone https://github.com/newtoallofthis123/qit.git
cd qit
cargo install --path .
```

### From Binary

To install from a binary, you can download the latest release from the [releases page](/releases). Once you have downloaded the binary, you can run `chmod +x qit` to make it executable, and then move it to a directory in your PATH.

```bash
chmod +x qit
sudo mv qit /usr/local/bin
```

## Usage

I am always working on new commands for `qit`, mostly since I use it a lot myself. If you have any ideas for new commands, feel free to open an issue or a pull request. This README might be a little out of date, so you can always run `qit help` to see the latest commands.

### `qit init`

Qit init is a wrapper around `git init`. It will create a git repository in the current directory, and then prompt you for some details. It will ask you for a project name, a project description, a license, and a .gitignore template. It will then create a README.md file, a license file, and a .gitignore file. It will also give you the option to add a remote repository.

### `qit open`

Qit opens up the current repository in your default browser. It will open the repository on GitHub, GitLab, or BitBucket, depending on which one you have configured with the git remote.

### `qit purge`

Qit purge is not a command I use very often, but it can be useful if you want to delete all of your commits and start over. It will delete the .git directory, and then reinitialize the repository.

### `qit "commit message"`

Qit commit is a wrapper around the following commands:

```bash
git add .
git commit -m "commit message"
git push
```

This is the _core_ of qit. It is the command I use the most. It will add all of the files in the current directory, commit them with the message you provide, and then push them to the remote repository.
Don't worry if you accidentally use it, it will ask you to confirm before it pushes.

### `qit help`

Qit help will print out a list of all of the commands, and a short description of what they do.

## Contributing

If you have any ideas for new commands, feel free to open an issue or a pull request. I am always looking for new ideas. If you find a bug, please open an issue. If you have any questions, you can email me at [noobscience@duck.com](mailto:<noobscience@duck.com>)

## License

Qit is licensed under the [MIT License](/LICENSE)

## Acknowledgements

- [Git](https://git-scm.com/)
- [GitHub](https://github.com)
- [nexxeln/license-generator](<https://github.com/nexxeln/license-generator>)
