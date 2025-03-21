# 4. Simulating and Resolving Merge Conflict

The objectives are to

- Create a scenario that produces a merge conflict and resolve it.

**Merge Conflicts**

- Merge conflicts happen when two branches modify the same part of the same file in different ways. Here’s how to simulate and resolve such conflicts:

- Steps to simulate a merge conflict

    - Create Two Branches from the Same Commit
    ```bash
    git init
    git checkout -b main
    ```

    - A file is added in the main branch:
    ```bash
    echo "Line 1: package:0.0.1" > sample.txt
    git add sample.txt
    git commit -m "Add line 1 to sample.txt"
    ```
    - Checkout two different branches and do seperate modifications to the same line in the file.
    ```bash
    git checkout -b bump1

    echo "Line 1: package:0.0.2" > example.txt
    git add example.txt
    git commit -m "Modify package hotfix"

    git checkout main

    echo "Line 1: package:0.0.3" > example.txt
    git add example.txt
    git commit -m "Modify line 1 in main branch"
    ```
    - Attempt to Merge the Branches

    ```bash
    git checkout main
    git merge bump1
    ```

    - This will result in a merge conflict.
    ```
    <<<<<<< HEAD
    Line 1: package:0.0.3
    =======
    Line 1: package:0.0.2
    >>>>>>> bump1

    ```


**Resolving the Merge Conflict**

- `git status` lists the current status, including any merge conflicts.

- `git diff` shows the conflicting portions of the file.


- Open the conflicting file (sample.txt) and manually decide which change you want to keep or how to combine both changes. We can:
    - Choose one version completely (either from HEAD or from bump1).
    - Combine parts of both changes.

- After manually editing till the file is as desired, the conflict markers are discarded (<<<<<<<, =======, >>>>>>>).

- Finally, using `git add .` and `git commit` will finally merge the changes.
