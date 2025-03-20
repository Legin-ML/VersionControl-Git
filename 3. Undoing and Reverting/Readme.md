# 3. Undoing and Reverting

The objectives are to

- Undoing editing a file
- Undoing commits

**Undoing modification of tracked files**

- Git has a way of quickly reverting a file's local changes, back to the latest commit. This can be achieved via two commands, both having the same functionality

  - `git checkout -- <filename>` (Ambiguous, as checkout is used to switch branches as well, but supported in older versions)
  - `git revert <filename>` (Newer command, but has only one functionality, hence more consice).

- Eg: In the latest commit, the file `test.txt` has the text `"Hello world"`.
  - The file is modified as `"Hello everyone"`.
  - `git revert test.txt` is executed, changing the data inside `test.txt` to `"Hello world"`.

**Undoing Commits**

- Undoing commits can be achieved by two ways, `git revert` and `git reset`

- `git revert`

  - Usage: `git revert <commit-hash>`
  - It will ask for a commit message text file, just like `git commit`. Once given correctly, It will create a _new_ commit, that is the same as the given commit-hash.
  - This is a **non-destructive** method of reverting back to a particular commit.

- `git reset`

  - Usage: `git reset --<type> <commit-hash>`
  - This method will move the header HEAD to the specified commit hash. The next operation depends on the <type> parameter
  - `--soft` : This moves the HEAD pointer, but leaves the staged files and working directory untouched.
  - `--mixed`: (Default) This moves the HEAD pointer and unstages the files. But the working directory is untouched.
  - `--hard` : This moves the HEAD pointer, unstages the files, and resets all the files in the working directory to the ones in the commit. (Destructive).

- Usage:
  - `git revert`: Shared repositories, where history needs to be maintained.
  - `git reset`: To clean up unwanted commits in local repo before commiting changes, or need for drastic changes.
