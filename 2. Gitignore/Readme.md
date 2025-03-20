# 2. Using the .gitignore file

The objectives are to

- Setting up .gitignore file
- Verify the working of the gitignore file

**Gitignore patterns**

- A `.gitignore` file is used to prevent staging of certain file and folder patterns when adding them to the staging area.
- This is especially useful in situations where 
    - Build artifacts 
    - Private files (API Keys used for local testing)
    - Temporary files   *and similar.*

- There are many ways of including patterns in gitignore.
    - `<filename.extension>` The standard way of ignoring a single file. Eg: `error.log`
    - `*.extension`. This is a wildcard entry. This implies to ignore *all* files with the specified file extension Eg: `*.log` (Ignores all .log files)
    - `<folder>/`. This pattern discards everything inside the specified folder. (Adding `**/` before the folder makes it search all subdirectories too)

**Demonstration**

- A rust application has been initialized in the current directory.
- The required files are src/*, and Cargo.toml.
- But, when the application is run using `cargo run`, It generates a `target/` folder, containing the build artifact and intermediatory steps, which is not desired in a repository.
- Hence, the unwanted folder is removed using the `.gitignore` file created, that contains `target/`. This makes the target folder excluded from the git staging process.

