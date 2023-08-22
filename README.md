# Quick Git

A Simple CLI to quickly commit and manage git stuff.

This is a beginner project in rust and is mostly aimed for beginners who end up doing the same git commands over and over again.

```
git add .
git commit -m <commit_msg>
git push origin <branch_name>
```

So, I decided to make a simple CLI to do all of this in one command.
By default, it will push to the current branch you are on.
You can also specify the branch name to push to by using the `-b` flag.
Other than that, it is a simple CLI written in rust.

## Installation

Installing it on your system is very easy, just run the following command in your terminal.

```bash
curl -o "qit.exe" https://www.noobscience.rocks/go/tqV0YZ
```
That's it, you are done. Now you can run `qit` in your terminal to use it. I recommend adding it to your path so that you can use it from anywhere.

### Local Installation

If you want to install it locally, you can clone the repo and run the following command in the root directory of the repo.

```bash
git clone https://github.com/newtoallofthis123/qit
cargo build --release
cp target/release/qit /usr/local/bin
```

That should do it. Now you can run `qit` in your terminal to use it.

## Usage

```bash
qit <commit_msg>
```

It is that simple and the rich CLI will guide you through the rest.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.