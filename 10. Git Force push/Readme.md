# 10. Comprehensive Workflow with Forced Pushes and Recovery

**Objective:**
- Simulate an advanced Git scenario that includes forced pushes, recovering lost commits, and a multi-branch workflow.

**Steps:**


- Create a new repository and add feature branches:

    ```
    git init  
    git remote add origin <git remote>
    <adding changes and commiting to main branch>
    git checkout -b feature-branch  
    git checkout -b bugfix-branch  
    git checkout -b release-branch  
    ```


- After making some changes, perform an interactive rebase to clean up commits:

    ```
    git rebase -i HEAD~5  
    ```

- Push the rewritten history (forced push):

    ```
    git push --force origin feature-branch 
    ```
 
 **NOTE:**
- This changes the local commit history, and it becomes out of sync with the remote repository. This leads to conflicts, and hence, git will not allow pushing data that overwrites older commits to remote. But, `git push --force` overwrites that and changes the commit history. 

- This *WILL* lead to data loss if improperly applied or applied to a main branch, where others have made changes *before* the rewritten commit history.
Use git reflog to recover lost commits after a mistaken force push:

**Reflog**


- Use `git reflog` to find the commit history:

- `git reflog` shows the history of the repository's HEAD, allowing us to find and recover commits that might have been lost due to a force push, reset, or rebase.
 
- Checkout the lost commit:
    `git checkout <commit-hash>` or `git checkout HEAD@{<pointer>}`  
- Create a new branch from the lost commit:

    `git checkout -b recovered-branch`  

- `git reflog <branch-name>` provides reflogs for other branches as well.

**Best Practices for Collaboration when Using Force Pushes:**

- Only use force pushes when absolutely necessary (e.g., to fix commit history).

- Communicate with team members when rewriting history to avoid conflicts.

- Always create backups before performing any destructive Git operations.