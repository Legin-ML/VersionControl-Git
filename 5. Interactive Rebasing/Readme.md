# 5. Interactive Rebasing for Clean Commit History

The objectives are to

- Use interactive rebase to tidy up commit history.

**Interactive Rebase Overview**

- Interactive rebase is a powerful tool in Git that enables users to rewrite commit history. It gives the flexibility to:

    - *Squash* multiple commits into one (combine several commits into a single, meaningful commit).
    - *Reorder* commits to ensure logical order.
     - *Edit* commit messages to correct typos or clarify changes.

**How to Perform Interactive Rebase**

- Start an Interactive Rebase

    - Use `git rebase -i HEAD~n`, where *n* is the number of commits to go back in history (e.g., `HEAD~3` to rebase the last `3` commits).   Eg: `git rebase -i HEAD~5`
    - Choose actions for each commit in the editor that opens. A list of recent commits and their hashes will be listed. Each commit will be prefixed by the word `pick`. This can be changed to one of these five commands:

        - pick (keep the commit as is)
        - reword (edit the commit message)
        - squash (combine the commit with the previous one)
        - edit (edit the commit’s content)
        - drop (remove the commit) 
    ```
    pick 1705923 Fix mod dep
    squash fa91db3 Fix mismatch
    ```
    - After choosing the desired actions, save and close the editor. Git will start the rebase and may prompt to resolve conflicts or edit commit messages.

    - If any conflicts arise, Git will pause and prompt for resolving commits. Once resolved, use `git rebase --continue` to proceed.