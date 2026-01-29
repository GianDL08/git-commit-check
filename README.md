# git-commit-check

Validate Git commit messages against a minimal Conventional Commits rule set.

## Installation

```bash
cargo install --path .
```

## License

MIT. See `LICENSE`.

## Examples

Valid:

- `feat: add login flow`
- `fix(auth): handle token refresh`
- `docs(readme): update usage`

Invalid:

- `Feat: add login flow` (type must be lowercase)
- `fix: Add login flow` (description must be lowercase)
- `chore: tidy up.` (description must not end with a period)
- `refactor: this subject line is intentionally made longer than seventy-two characters to fail`

## Pre-commit hook setup

An example hook is provided at `scripts/pre-commit` and expects the commit message file path
as the first argument.

Copy or symlink it into your repo:

```bash
cp scripts/pre-commit .git/hooks/pre-commit
# or
ln -s ../../scripts/pre-commit .git/hooks/pre-commit
```

Make sure the hook is executable:

```bash
chmod +x .git/hooks/pre-commit
```

## Walkthrough

1) Build the CLI:

```bash
cargo build
```

2) Try a valid message (should print success and exit 0):

```bash
printf "feat: add login flow\n" | ./target/debug/git-commit-check /dev/stdin
```

3) Try an invalid message (should print errors and exit 1):

```bash
printf "fix: Add login flow\n" | ./target/debug/git-commit-check /dev/stdin
```

4) Install the hook and test in Git:

```bash
chmod +x scripts/pre-commit
cp scripts/pre-commit .git/hooks/pre-commit

git commit --allow-empty -m "fix: add check"
git commit --allow-empty -m "fix: Add check"
```
