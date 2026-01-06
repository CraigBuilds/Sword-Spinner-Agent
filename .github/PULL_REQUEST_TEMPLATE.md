# Pull Request

## Description
<!-- Provide a brief description of your changes -->

## Type of Change
- [ ] Bug fix (non-breaking change which fixes an issue)
- [ ] New feature (non-breaking change which adds functionality)
- [ ] Breaking change (fix or feature that would cause existing functionality to not work as expected)
- [ ] Documentation update
- [ ] Refactoring (no functional changes)
- [ ] Performance improvement
- [ ] Build/CI changes

## Code Quality Checklist
**Run these checks during development to ensure code quality:**

### Required Rust Checks (run locally during development)
- [ ] `cargo check` passes without errors
- [ ] `cargo clippy` passes without warnings (run: `cargo clippy -- -D warnings`)
- [ ] `cargo fmt --check` passes (run: `cargo fmt` to fix)
- [ ] Code builds successfully with `cargo build`

### Testing
- [ ] `cargo test` passes (run: `cargo test --lib`)
- [ ] Manual testing performed (if applicable)
- [ ] Android build tested (if applicable): `cargo apk build --lib`

### Documentation
- [ ] Code comments added for complex logic
- [ ] README.md updated (if needed)
- [ ] SETUP.md updated (if needed)

### Best Practices
- [ ] No unnecessary dependencies added
- [ ] No compiler warnings introduced
- [ ] No `TODO` or `FIXME` comments left unaddressed
- [ ] Code follows existing project patterns and style
- [ ] Security considerations reviewed (no unsafe code without justification)

## Testing Instructions
<!-- Describe how to test your changes -->

## Additional Notes
<!-- Any additional information, context, or screenshots -->
