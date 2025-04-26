# Database Design

This document outlines the design decisions and implementation details for the img-browser media database system.

## Overview

The img-browser application uses a hybrid database approach for storing media metadata, tags, and user preferences. This approach balances ease of use, portability, and flexibility.

## Database Structure

### Core Components

1. **Global Database**
   - Default storage location for all media metadata
   - Hidden from the user in the application data directory
   - Contains all tags, favorites, and viewing history
   - Automatically loaded at application startup

2. **Project Databases**
   - Optional directory-specific databases
   - Override the global database for specific directories
   - Useful for project-based workflows and sharing
   - Explicitly created by the user

3. **Exportable Database Files**
   - Shareable snapshots of database content
   - Can be imported into other instances of the application
   - JSON format for human readability and editability

## Storage Locations

### Windows
- Global Database: `%APPDATA%\img-browser\global_db.json`
- Settings: `%APPDATA%\img-browser\settings.json`
- Project Databases: `<project_directory>\.img-browser\project_db.json`

### Linux (Planned)
- Global Database: `~/.config/img-browser/global_db.json`
- Settings: `~/.config/img-browser/settings.json`
- Project Databases: `<project_directory>/.img-browser/project_db.json`

### macOS (Planned)
- Global Database: `~/Library/Application Support/img-browser/global_db.json`
- Settings: `~/Library/Application Support/img-browser/settings.json`
- Project Databases: `<project_directory>/.img-browser/project_db.json`

## Database Content

The database stores the following information:

1. **Media Files**
   - File paths
   - Metadata (size, dimensions, format, etc.)
   - User-added tags
   - Favorite status
   - View history

2. **Tags**
   - Tag names
   - Tag hierarchies (planned)
   - Tag usage statistics

3. **Collections**
   - Named groups of media files
   - Smart collections based on rules (planned)

4. **AI-Related Data** (planned)
   - Prompt information
   - Model information
   - Generation parameters
   - Detected defects

## Implementation Details

### File Format
- JSON for human readability and editability
- Structured to minimize redundancy
- Versioned schema for future compatibility

### Loading Strategy
- Global database loaded at startup
- Project databases loaded when entering relevant directories
- Automatic detection of database changes

### Saving Strategy
- Automatic saving on significant changes
- Periodic background saving
- Explicit save option in UI
- Backup of previous version before saving

### Conflict Resolution
- Timestamp-based resolution (newer wins)
- Option to merge conflicting databases
- User notification of conflicts

## User Interface

The application will provide the following database-related UI features:

1. **Database Management**
   - Create new project database
   - Export database to file
   - Import database from file
   - Reset to global database

2. **Project Switching**
   - Indicator of current database source
   - Quick switching between recent projects

3. **Synchronization**
   - Manual sync between global and project databases
   - Conflict resolution interface

## Future Considerations

1. **Cloud Synchronization**
   - Sync databases across devices
   - Versioning and conflict resolution

2. **Multi-User Collaboration**
   - Shared project databases
   - Change tracking and merging

3. **Advanced Database Features**
   - Full-text search
   - Complex queries
   - Performance optimizations for large collections
