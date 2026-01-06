## Git Commits

- Never mention Claude Code in commit messages
- Never include the generated footer or Co-Authored-By trailer
- Use imperative mood ("Add feature" not "Added feature")
- Keep commits small and focused
- Before committing update CHANGELOG.md based on the commit messages

## Changelog

Follow [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) format:

### Categories (use only these headings)
- **Added** - New features
- **Changed** - Modifications to existing functionality
- **Deprecated** - Features marked for future removal
- **Removed** - Eliminated features
- **Fixed** - Bug fixes
- **Security** - Vulnerability patches

### Entry Guidelines
- Write user-focused descriptions (what changed for the user, not implementation details)
- Use imperative mood ("Add filter option" not "Added filter option")
- Link to issues/PRs when relevant: `([#123](https://github.com/ametel01/zed-aztec/issues/123))`
- Add entries to `[Unreleased]` section during development
- Move entries to versioned section when releasing
- Update version comparison links at bottom of file when releasing
