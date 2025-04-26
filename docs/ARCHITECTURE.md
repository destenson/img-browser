# Architecture

This document outlines the architectural design of the img-browser media management suite.

## Overview

img-browser follows a modular, layered architecture with clear separation of concerns. The application is designed to be platform-agnostic at its core, with platform-specific implementations provided through abstraction layers. This architecture supports the project's ambitious goals of comprehensive media management with specialized AI-generated content tools.

## Architectural Layers

The architecture is organized into several distinct layers, each with specific responsibilities:

### 1. Core Application Layer (`app/`)

The application core contains the platform-independent business logic:

- **App**: The main application struct that coordinates all components
- **Config**: Configuration management and command-line argument parsing
- **State**: Application state management
- **Error**: Error handling and result types
- **Settings**: User settings and preferences

### 2. Domain Layer (Planned)

The domain layer will contain the core business entities and logic:

- **Media**: Representations of different media types (images, videos, etc.)
- **Catalog**: Media organization and categorization
- **Tags**: Tagging and metadata system
- **Search**: Search and filtering capabilities
- **AI Models**: Interfaces for AI-based analysis and processing

### 3. Platform Layer (`platform/`)

The platform layer provides abstractions for platform-specific functionality:

- **Platform Trait**: Defines the interface for platform-specific implementations
- **Window**: Abstraction for window management
- **Event Handling**: Processing of platform events
- **Rendering**: Platform-specific rendering capabilities
- **File System**: Platform-specific file operations

Currently, the application primarily targets Windows through the Win32 API.

### 4. Data Layer (Planned)

The data layer will handle persistence and data management:

- **Database**: Storage for media metadata and organization
- **File Management**: Handling of media files on disk
- **Import/Export**: Data interchange with other systems
- **Caching**: Performance optimization for media access
- **Synchronization**: Multi-device data synchronization

### 5. Infrastructure Layer

Cross-cutting concerns that support the entire application:

- **Logging**: Centralized logging system with configurable levels
- **Configuration**: Application-wide configuration management
- **Performance Monitoring**: Tracking and optimization of performance
- **Security**: Access control and data protection
- **Internationalization**: Support for multiple languages

## Core Workflows

### Application Startup Flow

1. The application starts in `main.rs`
2. Command-line arguments are parsed into a `Config` object
3. The `App` is initialized with the `Config` and default `State`
4. The platform-specific implementation is selected based on the target platform
5. Core services are initialized (logging, database, etc.)
6. The platform layer creates a window and starts the event loop
7. The initial view is rendered and the application is ready for user interaction

### Event Processing Flow

1. User interactions and system events are captured by the platform layer
2. Events are translated into platform-agnostic actions
3. Actions are dispatched to appropriate handlers in the application core
4. The application core updates its state based on the actions
5. UI updates are requested as needed
6. The platform layer handles the actual rendering

### Media Management Flow (Planned)

1. Media Discovery: Scanning directories and identifying media files
2. Metadata Extraction: Reading file metadata and generating thumbnails
3. Cataloging: Organizing media into collections and categories
4. Tagging: Applying user-defined and automated tags
5. Searching: Finding media based on various criteria
6. Viewing: Displaying media with appropriate viewers

### AI Content Processing Flow (Planned)

1. Analysis: Examining media for AI-generated characteristics
2. Grouping: Clustering similar AI-generated content
3. Defect Detection: Identifying common AI artifacts and issues
4. Enhancement: Applying corrections to improve quality
5. Metadata Management: Tracking prompts, models, and generation parameters

## Media Processing Pipeline

1. Loading: Files are loaded using appropriate libraries (`image` crate for images, etc.)
2. Decoding: Media is decoded into standard internal formats
3. Analysis: Content is analyzed for metadata, features, and potential issues
4. Processing: Optional transformations are applied (resize, enhance, etc.)
5. Rendering: The processed media is rendered to the screen using platform-specific APIs

For Windows image rendering, this involves:
- Creating a compatible bitmap
- Setting the bitmap bits from the image data
- Using BitBlt or DirectX to render the bitmap to the window

## Key Components and Technologies

### Core Libraries and Dependencies

- **image**: For loading and processing various image formats
- **windows**: For Windows API integration
- **log/env_logger**: For logging functionality
- **defer**: For resource cleanup
- **chrono**: For time-related functionality
- **opencv**: For advanced image processing and computer vision
- **rusqlite/diesel**: For database management (planned)
- **rayon**: For parallel processing (planned)
- **serde**: For serialization and deserialization (planned)
- **tokio**: For asynchronous operations (planned)

### AI and Machine Learning (Planned)

- **tensorflow/pytorch**: For machine learning model integration
- **onnxruntime**: For running pre-trained models
- **image-similarity**: For finding similar images
- **image-quality**: For detecting and measuring image defects
- **clustering-algorithms**: For grouping similar content

### User Interface (Planned)

- **egui/iced**: For cross-platform UI components
- **wgpu/vulkan**: For hardware-accelerated rendering
- **custom-widgets**: For specialized media management interfaces
- **theming-system**: For customizable appearance

## Extensibility

The architecture is designed with extensibility in mind:

### Plugin System (Planned)

- **Media Handlers**: For supporting additional media formats
- **Processors**: For custom media processing operations
- **Analyzers**: For specialized content analysis
- **Exporters/Importers**: For integration with other systems
- **UI Extensions**: For custom interface components

### API Layer (Planned)

- **REST API**: For remote access to the media catalog
- **WebSocket**: For real-time updates and notifications
- **SDK**: For third-party integration

## Future Architecture Considerations

- **Distributed Processing**: Enable processing across multiple machines for large collections
- **Cloud Integration**: Seamless integration with cloud storage and processing
- **Federated Catalogs**: Connecting multiple instances for collaborative work
- **AI Model Training**: Custom model training for specialized media analysis
- **Real-time Collaboration**: Multi-user simultaneous access and editing
- **Containerization**: Deployment as containerized services for scalability
