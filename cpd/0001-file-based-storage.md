# Change Proposal Document: Implementing File-Based Storage in Reservoir

**Author(s):** Divan Visagie
**Date:** 2025-07-06
**Updated:** 2025-09-06
**Status:** Draft

## 1. Summary

This document outlines the plan to implement a file-based storage system in Reservoir as an alternative to the current Neo4j storage system. This is to align more with the envisioned use case of the system which is primarily designed to run on a client machine operated by the user. It also aligns with Sector F's design philosophy of sticking with the original principals devised at Bell Labs of preferring data to be stored as text rather than as binary. This allows flexibility for users who want to explore or use the data produced by reservoir in ways it may be impossible for the original program designers to predict.

This will involve creating new repository implementations that write to the filesystem, adhering to traditional Unix conventions.

## 2. Research

In developing our file-based storage system, we will adhere to a consistent and predictable filesystem standard. Our approach is guided by the following principles:

-   **Filesystem Standard**: We will adopt the modern [XDG Base Directory Specification](https://specifications.freedesktop.org/basedir-spec/basedir-spec-latest.html) for organizing Reservoir's data. This standard provides a clean separation of different types of data and aligns with modern desktop environments on Linux and other Unix-like systems. The XDG specification defines environment variables for different data types, with sensible defaults when these variables are not set.

-   **Data Storage**:
    -   **User Data Directory**: Permanent message data will be stored in `$XDG_DATA_HOME/reservoir/` (defaults to `~/.local/share/reservoir/`).
    -   **Cache Directory**: Semi-temporary data like embeddings will be stored in `$XDG_CACHE_HOME/reservoir/` (defaults to `~/.cache/reservoir/`). To support multiple embedding models simultaneously, this directory will contain subdirectories named after the model used to generate the embeddings (e.g., `$XDG_CACHE_HOME/reservoir/embeddings/text-embedding-ada-002/`).
    -   **Configuration Directory**: Any future configuration files will be stored in `$XDG_CONFIG_HOME/reservoir/` (defaults to `~/.config/reservoir/`).

-   **File Naming**: To facilitate quick mapping between messages and their corresponding embeddings, we will use a content-based hashing mechanism for filenames. The filename for both a message and its corresponding embedding will be a SHA-256 hash of the message content. This ensures a direct and predictable link between the two.

-   **Data Structure**: The `MessageNode` struct is the primary data structure for messages. When serialized to JSON, it will have the following format:

    ```rust
    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct MessageNode {
        pub id: Option<i64>,
        pub trace_id: String,
        pub partition: String,
        pub instance: String,
        pub content: Option<String>,
        pub role: String,
        pub embedding: Vec<f32>,
        pub url: Option<String>,
        pub timestamp: i64,
    }
    ```

## 3. Implementation Plan

### Milestone 1: Implement Directory Handling

- **Task:** Create utility functions to resolve and manage XDG-compliant directories.
- **Details:**
    - The utility will resolve XDG environment variables (`XDG_DATA_HOME`, `XDG_CACHE_HOME`, `XDG_CONFIG_HOME`) or fall back to their default locations.
    - It will ensure the creation of the appropriate subdirectories for messages (in data directory) and embeddings (in cache directory).
    - The directory structure within these paths will be `{{partition}}/{{instance}}/`, where `partition` is typically a `userId` and `instance` is a `chatId`.
- **Files to create:**
    - `src/utils/dirs.rs`

### Milestone 2: Implement File-Based Repositories

- **Task:** Create new repository implementations for `Message` and `Embedding` nodes.
- **Details:**
    - These repositories will write data to the filesystem as individual JSON files.
    - `Message` and `Embedding` nodes will be stored as `{{content_hash}}.json`.
    - Relationships between nodes (e.g., conversation history) will be stored in the `MessageNode` itself or handled by the services that reconstruct conversations.
- **Files to create:**
    - `src/repos/message/fs.rs`
    - `src/repos/embedding/fs.rs`

### Milestone 3: Update Services to Use File-Based Repositories

- **Task:** Modify the existing services to use the new file-based repositories.
- **Details:**
    - This will likely involve creating a new set of services (e.g., `services/fs/messages.rs`) or adding a feature flag to the existing services to switch between repository implementations.
    - For now, we will focus on creating parallel services to avoid disrupting the existing functionality.
- **Files to create:**
    - `src/services/fs/mod.rs`
    - `src/services/fs/messages.rs`

### Milestone 4: Create a Feature Flag for Storage Options

- **Task:** Introduce a configuration flag to enable or disable the file-based storage system.
- **Details:**
    - This will allow us to easily switch between the Neo4j and file-based storage systems for testing and development.
    - The flag will be checked in the main application logic to determine which services to use.
- **Files to modify:**
    - `src/args.rs`
    - `src/main.rs`

## 4. Long-Term Vision

This implementation is the first step towards migrating Reservoir to a fully file-based storage system that follows modern Unix conventions. The XDG-compliant approach provides several benefits:

- **Better User Experience:** Users can easily locate their data (`~/.local/share/reservoir/`) separately from cache (`~/.cache/reservoir/`), making backups and data management more intuitive.
- **System Integration:** The XDG structure integrates well with modern desktop environments and system tools that understand these conventions.
- **Selective Data Management:** Users can safely clear cache directories without losing important message data, and backup tools can focus on data directories.

Future work will involve:

- **Optimizing file I/O:** Using memory-mapped files (`memmap2`) for better performance.
- **Implementing advanced querying:** Developing a system for querying data across multiple files.
- **Configuration Management:** Adding support for user-configurable settings in `$XDG_CONFIG_HOME/reservoir/`.
- **Phasing out Neo4j:** Once the file-based system is stable and feature-complete, we will begin the process of removing the Neo4j dependency.
