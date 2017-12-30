# git-branch-deleter-rs

A simple CLI tool to help delete local branches.

### installation

Clone this repo and run:

```
cargo install
```

Use the `--force` option to override previous installs if upgrading.

### usage

Navigate to the git directory that you'd like to remove branches from. Then run
the following comand:

```
$ git-bd
```

It will prompt you through series of questions for each branch. You should answer 'Yes'
or 'No' to the prompt. Hitting enter will skip the branch.

It will not delete the branch you are currently on.
