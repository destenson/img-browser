# TODO List

This document tracks immediate tasks and priorities for the img-browser media management suite.

## Foundation Phase (Immediate Priority)

- [x] **Core Architecture Implementation**
  - [x] Complete the implementation of the Platform trait
  - [x] Fix the platform module structure to properly inject platform implementations
  - [x] Resolve the incomplete code in `platform/mod.rs`
  - [ ] Design and implement the domain model for media management

- [x] **Basic UI Framework**
  - [x] Implement a flexible window management system
  - [ ] Create a basic layout system for media display
  - [ ] Design UI components for media browsing
  - [ ] Implement responsive UI that works with different screen sizes

- [x] **Media Loading and Display**
  - [x] Implement efficient media loading pipeline
  - [x] Support for common image formats (JPEG, PNG, WebP, etc.)
  - [ ] Basic video format support
  - [ ] Thumbnail generation and caching

- [x] **File System Integration**
  - [x] Directory scanning and indexing
  - [x] File metadata extraction
  - [ ] Watch for file system changes
  - [ ] Basic file operations (copy, move, delete)

## Media Management Phase (High Priority)

- [ ] **Database Design and Implementation**
  - [x] Schema design for media catalog
  - [ ] Implement global user-hidden database as default storage
  - [ ] Add ability to export sharable database files
  - [ ] Support directory-based project databases that override global database
  - [ ] Efficient storage of metadata
  - [ ] Query optimization for large collections
  - [ ] Migration system for schema updates

- [ ] **Tagging and Categorization System**
  - Hierarchical tag structure
  - Automatic tag suggestions
  - Tag management UI
  - Smart collections based on rules

- [ ] **Search and Filter Capabilities**
  - Full-text search across metadata
  - Advanced filtering options
  - Saved searches
  - Search result visualization

- [ ] **Media Organization**
  - Collections and albums
  - Smart grouping based on time, location, etc.
  - Duplicate detection
  - Batch operations on multiple files

## AI Integration Phase (Medium Priority)

- [ ] **Similarity Detection**
  - Research and implement image similarity algorithms
  - Clustering of similar images
  - UI for browsing similar content
  - Threshold adjustment for similarity matching

- [ ] **AI Defect Detection**
  - Identify common AI generation artifacts
  - Classification of defect types
  - Severity rating system
  - Visualization of detected issues

- [ ] **Enhancement Tools**
  - Basic correction tools for common defects
  - Integration with external enhancement tools
  - Batch processing for corrections
  - Before/after comparison

- [ ] **Prompt Management**
  - Extraction of prompts from metadata (if available)
  - Prompt organization and tagging
  - Prompt effectiveness analysis
  - Prompt library and sharing

## Technical Improvements (Ongoing)

- [ ] **Performance Optimization**
  - Profile application performance
  - Optimize media loading and rendering
  - Implement multi-threaded processing
  - Memory usage optimization

- [x] **Error Handling and Logging**
  - [x] Enhance the Error type with more specific error variants
  - [x] Implement comprehensive logging system
  - [ ] User-friendly error messages
  - [ ] Crash recovery mechanisms

- [ ] **Testing Infrastructure**
  - Unit test framework
  - Integration tests for core functionality
  - Performance benchmarks
  - UI testing automation

- [ ] **Documentation**
  - API documentation
  - User guides
  - Developer documentation
  - Example workflows

## UI Improvements (Medium Priority)

- [ ] **Dialog Enhancements**
  - Add recursive folder scanning toggle to the folder open dialog (challenging due to Windows COM interfaces)
  - Implement confirmation dialogs for destructive operations
  - Add progress dialogs for long-running operations

- [ ] **Navigation and Interaction**
  - Implement keyboard navigation for media browsing
  - Add drag-and-drop support for files and folders
  - Create customizable interface themes
  - Implement zooming and panning controls for images

- [ ] **Layout and Organization**
  - Add customizable grid/list views
  - Implement collapsible panels for metadata
  - Create resizable thumbnail sizes
  - Add sorting and grouping options in the UI

## Completed

- [x] Initial project structure setup
- [x] Basic Windows integration exploration
- [x] Proof-of-concept image loading and display
- [x] Project documentation framework
- [x] Platform abstraction layer implementation
- [x] Windows-specific implementation with proper resource management
- [x] Basic window management system with menus
- [x] File and folder dialog integration
- [x] Image loading and display capabilities
- [x] Proper error handling and logging system

## Notes

- Focus on building a solid foundation before adding advanced features
- Consider using a modular architecture to allow for future extensibility
- Research existing media management solutions for inspiration and best practices
- Prioritize user experience and performance from the beginning
- Consider AI integration points throughout the architecture
