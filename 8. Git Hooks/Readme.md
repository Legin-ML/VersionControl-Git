# 8. Using Git Hooks for Automated Checks
Objective:

Set up a Git hook to run scripts before commits are finalized.

**Git Hooks:**
- Git hooks are custom scripts that we can execute at specific points in our Git workflow. 
- These hooks help automate tasks such as running linters or unit tests before making commits.

Example: Pre-commit hook that runs cargo fmt for formatting and cargo test for unit tests.

**Steps:**
- Create the pre-commit Hook in the .git/hooks Directory
Navigate to the Git hooks directory:


    ```cd .git/hooks```
- Create a new file named pre-commit:

    ```
    touch pre-commit
    chmod +x pre-commit
    ```
- Write a Simple Script to Run Rust Checks:
*in pre-commit*
```bash
#!/bin/bash

# Run cargo fmt to ensure the code is formatted
echo "Running cargo fmt..."
cargo fmt -- --check
if [ $? -ne 0 ]; then
  echo "Formatting issues detected. Commit aborted."
  exit 1
fi

# Run cargo test to ensure all tests pass
echo "Running cargo test..."
cargo test --quiet
if [ $? -ne 0 ]; then
  echo "Tests failed. Commit aborted."
  exit 1
fi

# If both checks pass, allow the commit
exit 0
```


- When a commit is invoked (`git commit`), this script is run.

**Other hooks**
- applypatch-msg       
- commit-msg           
- post-update          
- pre-applypatch       
- pre-commit
- pre-push
- pre-rebase
- prepare-commit-msg
- update
