# Contributing to ScaffoldMC

Thank you for wanting to contribute to ScaffoldMC! This document specifies
guidelines that should be followed when contributing code. Before contributing
code to the project, please first open an issue describing the problem you're
having or the feature you want added.

## Pull Requests

1. Describe the issue your pull request resolves and the changes it makes. Link
   relevant issues.
    - Keep changes focused. Multiple unrelated changes shouldn't be lumped into
      one pull request.
    - Open an issue if the changes your PR makes is not described by an existing
      unresolved issue.

2. Ensure that your changes work before requesting to merge.
    - Changes should compile without warnings and behave as intended
    - Your changes should meet the linter guidelines.

3. Ensure that your code is formatted with the provided formatter configurations
   and is readable
    - Use comments where necessary. Use doc comments for functions.

### Additional Notes

- Pull Requests with mangled git history will be rejected.
    - If your feature branch needs to be updated with upstream changes, use
      `git rebase`
- LLM-produced code should be thoroughly understood, audited, and cleaned up.
  Pull Requests submitted by LLM Agents will be rejected.
