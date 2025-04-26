# Development Guide

This document provides guidelines and best practices for developing the Image Browser application.

## Development Environment Setup

### Required Tools

1. **Rust Toolchain**
   - Install using [rustup](https://rustup.rs/)
   - Use the latest stable version (1.70.0+)
   - Required components: `rustc`, `cargo`, `rustfmt`, `clippy`

2. **IDE/Editor**
   - Recommended: Visual Studio Code with rust-analyzer extension
   - Alternatives: CLion, Vim/Neovim with rust plugins, or any editor with Rust support

3. **Git**
   - For version control
   - Configure with your name and email:
     ```
     git config --global user.name "Your Name"
     git config --global user.email "your.email@example.com"
     ```

4. **Windows SDK**
   - Required for Windows API development
   - Install the latest Windows SDK from Microsoft

### Setting Up the Project

1. Clone the repository:
   ```
   git clone https://github.com/yourusername/img-browser.git
   cd img-browser
   ```

2. Build the project:
   ```
   cargo build
   ```

3. Run tests:
   ```
   cargo test
   ```

4. Run the application:
   ```
   cargo run
   ```

## Coding Standards

### Rust Style Guidelines

- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `rustfmt` to format your code:
  ```
  cargo fmt
  ```
- Use `clippy` to catch common mistakes:
  ```
  cargo clippy
  ```

### Project-Specific Guidelines

1. **Error Handling**
   - Use the custom `Result` and `Error` types from `app::error`
   - Propagate errors with `?` operator when appropriate
   - Provide meaningful error messages

2. **Logging**
   - Use the `log` crate macros: `error!`, `warn!`, `info!`, `debug!`, `trace!`
   - Choose appropriate log levels based on message importance
   - Include relevant context in log messages

3. **Documentation**
   - Document all public APIs with doc comments
   - Include examples where appropriate
   - Keep documentation up-to-date with code changes

4. **Testing**
   - Write unit tests for all new functionality
   - Use integration tests for platform-specific code
   - Aim for high test coverage

## Architecture Guidelines

1. **Separation of Concerns**
   - Keep platform-specific code in the `platform` module
   - Keep business logic in the `app` module
   - Use traits for abstraction between layers

2. **Resource Management**
   - Use RAII principles for resource management
   - Consider using the `defer` crate for cleanup in complex scenarios
   - Properly handle Windows resources (HANDLES, DCs, etc.)

3. **Performance Considerations**
   - Minimize allocations in performance-critical paths
   - Use appropriate data structures for the task
   - Consider memory usage when dealing with large images

## Git Workflow

1. **Branching Strategy**
   - `main`: Stable, production-ready code
   - `develop`: Integration branch for features
   - Feature branches: `feature/feature-name`
   - Bug fix branches: `fix/bug-description`

2. **Commit Guidelines**
   - Write clear, concise commit messages
   - Use present tense ("Add feature" not "Added feature")
   - Reference issue numbers when applicable

3. **Pull Requests**
   - Create a PR for each feature or bug fix
   - Include a description of changes
   - Ensure all tests pass
   - Request code review from team members

## Building and Testing

### Debug Builds

```
cargo build
```

### Release Builds

```
cargo build --release
```

### Running Tests

```
cargo test
```

### Running Examples

```
cargo run --example win32-imageview
```

## Troubleshooting Common Issues

1. **Windows API Errors**
   - Check MSDN documentation for specific error codes
   - Use `GetLastError()` to get detailed error information
   - Enable debug logging for more information

2. **Build Errors**
   - Ensure you have the latest Rust toolchain
   - Check for missing dependencies
   - Verify Windows SDK installation

3. **Runtime Errors**
   - Check log output for error messages
   - Use a debugger to step through problematic code
   - Verify resource cleanup in error paths

## Additional Resources

- [Rust Documentation](https://doc.rust-lang.org/book/)
- [Windows API Documentation](https://docs.microsoft.com/en-us/windows/win32/api/)
- [Rust Windows Crate Documentation](https://docs.rs/windows/latest/windows/)
