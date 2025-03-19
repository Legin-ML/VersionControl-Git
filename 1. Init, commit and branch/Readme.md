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
- The commit message can be given directly as `git commit -m "<message>"`, or It can be edited via a text editor, if just `git commit` is given as the command.
- `git log` lists the commits made.

**Branching and Merging**

- Git branches are a very powerful way of seperating the development process of code, without disturbing the actual codebase, leading to parallel and faster development.
- Each branch, contains the code *until* the creation of the branch. After the changes are commited, the branch can be *merged* back to the parent branch. This feature enables proper testing and validation of code, before it is merged, leading to more difficulty to track down.

- First, `git branch <branch-name>` is used to create a branch, and `git checkout <branch-name>` is used to switch to the branch. (Note: `git checkout -b <branch-name>` creates, and moves to a branch. As of git v2.23, there is a new command to reduce the ambiguity of git checkout. It is `git switch <branch-name>`).

- Changes are done, and committed using `git commit`. This change is in the new branch. 

- Eg. 
    - `git checkout -b dependency-update`
    - In Cargo.toml, a new dependency is added to parse json (serde_json)
    - `git add .`
    - `git commit -m "feat: added serde_json"`

- To **merge** the changes, First, switch to the parent branch. `git checkout -b <parent-branch>`. Then, the merge command is used. `git merge <feature-branch>`.

- Eg.(contd.)
    - `git checkout main`
    - `git merge dependency-update`

- This adds the code changes done by the feature branch to the parent branch. 