# Overview

Dynamic aliases based on the directory you are currently in.

Ever wanted to type something like `server` in whole bunch of different directories and your computer just knows what you're thinking?

Now you can!

## Why you want this

Already know you want this? Skip to [Installation](#installation).

Bash aliases are cool but limited, they are globals and have limited configurability.

One downside of standard bash aliases is that they don't take arguments, to counter this many people (myself included) do thing like create bash functions like this:

```
function ll() {
  ls -la "$@"
}
```

This downside to this is it's hard to tell where these functions are coming from, you can't just type `which ll` and find the function.

You also end up writing a whole lot of functions that are really similar, plus none of these are dynamic or contextual

So I created `aliases` to make my life easier and more fun.

## Usage

To create aliases for the current directory run:

```
aliases init
```

This creates a `.aliases` file in the current directory with a commented out alias structure that looks like below:

```yaml
# alias_name:
#   command: ./super_command.sh                         # required
#   confirm: true                                       # optional - You will be asked to confirm before execution
#   confirmation_message: Are you sure you are sure??   # optional - If confirm is set to true then you this is your confirmation message
#   conditional: /bin/true                              # optional - A bash command that needs to be successful for the alias to run
#   backout_seconds: 3                                  # optional - Give's you a backout option (ctrl + c) before the alias is executed
#   unit_test: '[ true = true ]'                        # optional - A bash command that tells whether the alias is doing what you want
#   quiet: false                                        # optional - default 'false', when false full evaluated command is printed to stdout before running
```

Edit the file and then run `aliases rehash` to make the alias available.

Here's an example of some aliases:

```yaml
l:
  command: ls
gc:
  command: git commit
deploy_production:
  command: bundle exec cap production deploy
  backout_seconds: 3
  conditional: [ `git rev-parse --abbrev-ref HEAD` == "master" ]
deploy_staging:
  command: bundle exec cap staging deploy
```

To list all aliases available type:

```
aliases
```

The `.aliases` file should be checked in to your repo, spread the alias love with the people you are working with.

### Global Aliases

Global aliases are created but running `aliases init` in your home directory.

Global aliases are overridden by local aliases if they exist.

## Installation

### OSX

```
brew tap sebglazebrook/aliases
brew install aliases
```

### Linux

TODO

### Compile from source

TODO


## Contributing

Do the normal things, fork the code and make a PR.


## Bugs to fix

- Need to escape forwarding arguments to alias commands as they could contain quotes
  i.e gc -m "My commit message" doesn't work when gc == "git commit" but gc -m \"My commit message\" does work
- Being able to actually run the unit tests :-)


## Small improvements to come

- Add info in aliases file and point to repo
- Sort aliases lists better and make it more obvious which ones are local
- Colors for user interactions


## Possible future features

- Having custom aliases i.e. .superman-aliases files etc
- Allow better filtering when listing aliases
- aliases that take params?
- Autocompletion for aliases
- clean uninstall, removing shims etc
- Allow parent aliases that are not global??
