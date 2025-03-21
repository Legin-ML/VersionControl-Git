# 6. Stashing Changes for Context Switching

The objectives are to

- Stash changes for future use.

**Stashing**

- Git stash is used for temporarily saving changes in the working directory without committing them. 
- This is useful when we need to switch branches but don't want to commit incomplete or experimental changes.

**Method**

- The changes are done on the respository. Eg:
```bash
echo "pub fn test() { true }" > src/main.rs
```
- The changes are not committed, but the file is edited. `git status` shows that the file is modified but not commited.

- To temporarily save these uncommitted changes, use `git stash`.

- This will:
    - Save the changes to a stash.
    - Revert the working directory to the state of the last commit.

- After this, we can switch to another branch and do the work.

- To reapply the stashed changes, switch back to the branch where made the stash, and pop the changes.
```
git checkout main

git stash pop
```

- The pop command restores the stashed changes and removes them from the stash list.

**Multiple Stashes**

- If  multiple sets have been stashed over time, we can view a list of all the stashes created using `git stash list`


- This will show a list of all stashes, with each stash identified by a name like `stash@{0}`, `stash@{1}`, etc.
Eg:
```
stash@{0}: WIP on main: fb567ut Implement Lint
stash@{1}: WIP on bump1: p65su9m Fixed version
```

- To apply a particular stash
```
git stash apply stash@{<stash-number>}
```

- To delete a particular stash
```
git stash drop stash@{<stash-number>}
```
- To clear all stashes
```
git stash clear
```