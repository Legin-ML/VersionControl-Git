# 1. Initialize, Commit and Branch Basics

The objectives are to

- Initialize a new Git repository.
- Create a few files and commit them.
- Create a new branch, make changes, and merge it back to the main branch.

**Initializing the repo**

- `git init` was used, to create a empty repository. This created a hidden `.git/` folder that contains all of the necessary info about the repository and its branches.
- `cargo init` was used, to create a default rust application. These acts as the sample files for the project.

**Committing the files**

- `git add` command is used to move the files to a staging area. This acts as an intermediatory between committing the code, as it marks the files as "ready to be committed".
- `git add <filename>` is used to add indiviudal files (Eg: `git add src/main.rs`). Alternatively, `git add .` can be used to add all of the files and folders in the current repository to staging.
- `git commit` takes all of the *staged* files, and creates a save, which is referred to as a commit. A commit is a snapshot of the current state of the project's staged files, containing
    - Hash: A unique randomized ID used to refernce a commit
    - Timestamp: Records when the commit is created
    - Authors info: Note of the Author's Name and Email
    - Commit message: A short and long message about the commit, given by the author<br>

- It is necessary to give a short commit message, but the long message is optional.
- The commit message can be given directly as `git commit -m "<message>"`, or It can be edited via a test editor, if just `git commit` is given as the command.
- `git log` lists the commits made.

**Branching and Merging**