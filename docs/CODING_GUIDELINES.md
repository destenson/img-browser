# Coding Guidelines

This document provides specific guidelines for writing high-quality code for the Image Browser project.

## General Principles

1. **Clarity over Cleverness**
   - Write code that is easy to understand and maintain
   - Avoid overly complex or clever solutions unless absolutely necessary
   - Comment complex sections of code to explain the reasoning

2. **Consistency**
   - Follow established patterns in the codebase
   - Be consistent with naming, formatting, and organization
   - Use the same approach to solve similar problems

3. **Correctness**
   - Ensure code works correctly under all expected conditions
   - Handle edge cases and error conditions gracefully
   - Write tests to verify correctness

4. **Performance**
   - Be mindful of performance implications, especially in image processing code
   - Optimize only when necessary and after profiling
   - Document performance considerations and trade-offs

## Rust-Specific Guidelines

### Type Safety

- Leverage Rust's type system to prevent errors at compile time
- Use strong types rather than primitive types where appropriate
- Create custom types to represent domain concepts
- Use newtypes to add semantic meaning to primitive types

```rust
// Instead of this
fn process_image(width: u32, height: u32) { /* ... */ }

// Do this
struct ImageDimensions {
    width: u32,
    height: u32,
}

fn process_image(dimensions: ImageDimensions) { /* ... */ }
```

### Error Handling

- Use the Result type for operations that can fail
- Provide meaningful error messages and context
- Use the `?` operator for error propagation
- Avoid panicking in library code

```rust
// Good error handling
fn load_image(path: &Path) -> Result<Image, Error> {
    let file = File::open(path).map_err(|e| Error::FileOpen {
        path: path.to_path_buf(),
        source: e,
    })?;
    
    // Process file...
    Ok(image)
}
```

### Memory Management

- Use Rust's ownership model effectively
- Prefer stack allocation over heap allocation when possible
- Use references instead of cloning when appropriate
- Be careful with `unsafe` code, document and isolate it

### Concurrency

- Use message passing (channels) for communication between threads
- Prefer higher-level abstractions like `rayon` for parallel processing
- Be explicit about synchronization and document thread safety
- Use `Arc` and `Mutex` judiciously

## Project-Specific Guidelines

### Platform Abstraction

- Keep platform-specific code behind the Platform trait
- Don't leak platform details into the core application logic
- Test platform-specific code separately from core logic
- Document platform-specific assumptions and requirements

```rust
// Good platform abstraction
trait Platform {
    fn create_window(&self, config: WindowConfig) -> Result<Window>;
    fn process_events(&self, app: &mut App) -> Result<()>;
}

// Implementation for specific platform
struct Win32Platform;

impl Platform for Win32Platform {
    fn create_window(&self, config: WindowConfig) -> Result<Window> {
        // Windows-specific implementation
    }
    
    fn process_events(&self, app: &mut App) -> Result<()> {
        // Windows-specific event processing
    }
}
```

### Image Processing

- Handle different image formats consistently
- Be careful with pixel format conversions
- Document assumptions about color spaces
- Consider memory usage for large images
- Use appropriate algorithms for scaling and transformations

### UI Code

- Separate UI logic from business logic
- Use consistent patterns for event handling
- Document UI state transitions
- Consider accessibility in UI design
- Handle high DPI displays properly

## Code Organization

### File Structure

- Group related functionality in modules
- Keep files focused on a single responsibility
- Use clear, descriptive file names
- Organize tests alongside the code they test

### Module Structure

- Start modules with public exports
- Group related functions and types
- Use private helper functions for implementation details
- Document module purpose and usage

```rust
// Good module structure
pub mod image_processing {
    // Public exports
    pub use self::loader::load_image;
    pub use self::processor::{resize_image, rotate_image};
    
    // Public types
    pub struct ImageProcessor { /* ... */ }
    
    // Submodules
    mod loader { /* ... */ }
    mod processor { /* ... */ }
    
    // Private helpers
    fn validate_dimensions(width: u32, height: u32) -> bool { /* ... */ }
}
```

## Documentation

### Code Comments

- Document the "why" not just the "what"
- Use doc comments (`///`) for public APIs
- Use regular comments (`//`) for implementation details
- Keep comments up-to-date with code changes

### API Documentation

- Document all public APIs with examples
- Specify preconditions and postconditions
- Document error conditions and handling
- Use Markdown formatting in doc comments

```rust
/// Resizes an image to the specified dimensions.
///
/// # Arguments
///
/// * `image` - The source image to resize
/// * `width` - The target width in pixels
/// * `height` - The target height in pixels
/// * `filter` - The resampling filter to use
///
/// # Returns
///
/// A new `Image` with the specified dimensions.
///
/// # Errors
///
/// Returns an error if the dimensions are invalid or if the resize operation fails.
///
/// # Examples
///
/// ```
/// let img = load_image("input.jpg")?;
/// let resized = resize_image(img, 800, 600, Filter::Lanczos3)?;
/// ```
pub fn resize_image(image: &Image, width: u32, height: u32, filter: Filter) -> Result<Image> {
    // Implementation...
}
```

## Testing

### Unit Tests

- Test each function or method in isolation
- Use mocks or stubs for dependencies
- Test both success and failure cases
- Test edge cases and boundary conditions

### Integration Tests

- Test interactions between components
- Test end-to-end workflows
- Use realistic test data
- Test performance characteristics

### Test Organization

- Keep tests close to the code they test
- Use descriptive test names
- Structure tests with Arrange-Act-Assert pattern
- Use test helpers to reduce duplication

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn resize_image_maintains_aspect_ratio() {
        // Arrange
        let image = create_test_image(1000, 500);
        
        // Act
        let resized = resize_image(&image, 500, 250, Filter::Lanczos3).unwrap();
        
        // Assert
        assert_eq!(resized.width(), 500);
        assert_eq!(resized.height(), 250);
    }
    
    #[test]
    fn resize_image_handles_zero_dimensions() {
        // Arrange
        let image = create_test_image(100, 100);
        
        // Act
        let result = resize_image(&image, 0, 0, Filter::Lanczos3);
        
        // Assert
        assert!(result.is_err());
        assert_matches!(result.unwrap_err(), Error::InvalidDimensions { .. });
    }
}
```

## Performance Considerations

### Image Loading

- Load images asynchronously to avoid blocking the UI
- Consider using a thread pool for parallel loading
- Implement caching for frequently accessed images
- Use memory mapping for large files when appropriate

### Rendering

- Minimize redraws and repaints
- Use hardware acceleration when available
- Implement double buffering to reduce flickering
- Consider using DirectX or other graphics APIs for better performance

### Memory Usage

- Be mindful of memory usage for large images
- Implement progressive loading for very large images
- Release resources when they're no longer needed
- Monitor memory usage during development

## Windows-Specific Guidelines

### Win32 API

- Properly initialize and clean up Windows resources
- Check return values from all Win32 API calls
- Use RAII patterns to manage Windows handles
- Document Win32-specific assumptions and requirements

### GDI/GDI+

- Be careful with device contexts (DCs)
- Always select objects out of DCs before deleting them
- Use appropriate pixel formats for different operations
- Be mindful of coordinate systems and transformations

### COM Interfaces

- Properly initialize and uninitialize COM
- Release COM interfaces when done with them
- Use smart pointers to manage COM interface lifetimes
- Be careful with threading models

## Conclusion

Following these guidelines will help ensure that the Image Browser codebase remains maintainable, performant, and correct. Remember that these guidelines are not rigid rules but rather principles to guide development. Use your judgment and adapt these guidelines to specific situations as needed.
