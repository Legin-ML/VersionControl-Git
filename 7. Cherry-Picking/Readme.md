# 7. Cherry-Picking Commits Between Branches
The objectives are to:

 - Selectively apply a commit from one branch to another using git cherry-pick.

**Cherry-Picking**

Git Cherry-Pick is used to apply a specific commit from one branch onto another. This is useful when we want to move a particular change or bug fix from one branch to another *without* merging the entire branch.

**Steps**

 - Create Two Branches with Distinct Commits

    Example:

    ```
    git checkout -b issue32
    echo "Version mismatch check function" > file.txt
    git add .
    git commit -m "fix mismatch"

    git checkout -b issue44
    echo "Version update check function" > file.txt
    git add .
    git commit -m "fix check"
    ```
- `git log` is used to find the commit hash that we want to apply to another branch.

    ```git log issue32```

 - Pick the commit hash to cherry-pick. 
- Switch to the branch where you want to apply the commit. 
    ```git checkout issue44```


- Apply the selected commit using the git cherry-pick command:


    ```git cherry-pick <commit-hash>```

 - Handle Conflicts (if any). After closing the file, continue the cherry pick process.

    ```    
    git add <resolved-file>
    git cherry-pick --continue
    ```
- `git log` can be used to verify the process completion.

