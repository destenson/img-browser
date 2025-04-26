# img-browser

A comprehensive media categorization and management suite with specialized tools for AI-generated content.

## Vision

img-browser aims to be more than just an image viewer - it's designed to be a complete solution for organizing, categorizing, and improving your media collection. With a special focus on AI-generated content, it provides tools for grouping similar media, identifying and correcting defects, and managing large collections efficiently.

## Key Features (Planned)

### Media Management
- Fast, efficient browsing of images, videos, and other media
- Advanced categorization and tagging system
- Powerful search and filtering capabilities
- Duplicate and similarity detection
- Metadata extraction and management

### AI-Generated Content Tools
- Similarity detection for grouping related AI-generated content
- Defect identification and correction for AI artifacts
- Prompt and model tracking
- Style analysis and categorization
- Batch processing for AI content improvement

### Technical Foundation
- Built in Rust for performance and safety
- Native platform integration for optimal performance
- Efficient memory usage for large collections
- Extensible architecture with plugin support
- Cross-platform compatibility (planned)

## Current Status

The project is in early conceptual and architectural planning stages. The foundation is being built with a focus on Windows integration initially, with plans to expand to other platforms in the future.

## Getting Started

### Prerequisites

- Rust toolchain (1.70.0 or newer recommended)
- Windows 10 or newer
- Git

### Building from Source

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/img-browser.git
   cd img-browser
   ```

2. Build the project:
   ```bash
   cargo build
   ```

3. Run the application:
   ```bash
   cargo run
   ```

### Examples

The project includes several examples demonstrating different aspects of the application:

- `win32-imageview.rs`: Basic image viewing functionality
- `win32-bitblt.rs`: Demonstrates BitBlt operations for image rendering
- `win32-dcomp.rs`: Shows DirectComposition usage
- `win32-enumvols.rs`: Volume enumeration example
- `win32-fileinfo.rs`: File information retrieval
- `win32-msgbox.rs`: Simple message box example

Run an example with:
```bash
cargo run --example win32-imageview
```

## Project Structure

- `src/`: Main application source code
  - `app/`: Core application logic
  - `platform/`: Platform-specific implementations
  - `logging/`: Logging utilities
- `examples/`: Example applications
- `docs/`: Project documentation
- `vendor/`: Third-party dependencies and reference implementations

## Documentation

- [Architecture](docs/ARCHITECTURE.md): Technical architecture and design
- [Development Guide](docs/DEVELOPMENT.md): Setup and development practices
- [Roadmap](docs/ROADMAP.md): Future plans and feature roadmap
- [Coding Guidelines](docs/CODING_GUIDELINES.md): Standards for code quality

## Contributing

Contributions are welcome! The project is in early stages, so there are many opportunities to help shape its direction. See the [Development Guide](docs/DEVELOPMENT.md) for information on getting started.

## License

[MIT License](LICENSE)
