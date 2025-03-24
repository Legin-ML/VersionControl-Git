# 9. Working with Remote Repositories and Collaboration

**Objective:**
Simulate a collaborative workflow with remote repositories.

**Steps:**



- Initialize a local repository:

    ```git init```

- Create a new repository on a remote service (e.g., GitHub, Gitlab, etc.).

- Link the remote repository to the local repository (Github is used as an example):

    ```git remote add origin https://github.com/<username>/<repository>.git```

- Push the local repository to the remote:

    ```git push -u origin main```

**Pull Requests**

- Create a new feature branch:

    ```git checkout -b <feature-branch>```  
- Commit as usual
    ```
    git add .  
    git commit -m "Add feature XYZ"  
    ```

- Push the feature branch to the remote:


    ```git push origin feature-branch```  

- Open a Pull Request (PR) [or Merge Request (MR)] and perform a code review process:

    - Navigate to the remote repository on GitHub/GitLab.

    - Open a PR/MR to merge feature-branch into main.

    - Collaborators review the PR, suggest changes, and approve it after addressing feedback.

    - Merge the feature branch via the remote interface (GitHub/GitLab).


- Pull the latest changes to your local repository:

    ```
    git checkout main  
    git pull origin main  
    ```