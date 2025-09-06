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

### Milestone 1: Create Repository Abstraction Layer

- **Task:** Create trait-based abstractions for repositories to enable different storage backends.
- **Details:**
    - Define traits for `MessageRepository` and `EmbeddingRepository` with all required methods.
    - Refactor existing Neo4j repositories to implement these traits.
    - Update services to accept repository implementations via dependency injection rather than hardcoded imports.
    - This establishes the foundation for swappable storage backends.
- **Files to create:**
    - `src/repos/traits.rs` (repository trait definitions)
- **Files to modify:**
    - `src/repos/message/neo4j_message.rs` (implement MessageRepository trait)
    - `src/repos/embedding/neo4j_embedding.rs` (implement EmbeddingRepository trait)
    - `src/services/messages.rs` (accept repository implementations)

### Milestone 2: Implement Directory Handling

- **Task:** Create utility functions to resolve and manage XDG-compliant directories.
- **Details:**
    - The utility will resolve XDG environment variables (`XDG_DATA_HOME`, `XDG_CACHE_HOME`, `XDG_CONFIG_HOME`) or fall back to their default locations.
    - It will ensure the creation of the appropriate subdirectories for messages (in data directory) and embeddings (in cache directory).
    - The directory structure within these paths will be `{{partition}}/{{instance}}/`, where `partition` is typically a `userId` and `instance` is a `chatId`.
- **Files to create:**
    - `src/utils/dirs.rs`

### Milestone 3: Implement File-Based Repositories

- **Task:** Create new file-based repository implementations that implement the repository traits.
- **Details:**
    - These repositories will write data to the filesystem as individual JSON files.
    - `Message` and `Embedding` nodes will be stored as `{{content_hash}}.json`.
    - Implement all methods from `MessageRepository` and `EmbeddingRepository` traits.
    - Handle relationships between nodes (e.g., conversation history) through the `MessageNode` structure or additional index files.
- **Files to create:**
    - `src/repos/message/fs.rs` (implements MessageRepository trait)
    - `src/repos/embedding/fs.rs` (implements EmbeddingRepository trait)

### Milestone 4: Update Services to Use File-Based Repositories

- **Task:** Update dependency injection to allow switching between storage backends.
- **Details:**
    - Since services now accept repository trait implementations, create factory functions or dependency injection container to provide the appropriate repository implementations.
    - Services can now work with either Neo4j or file-based repositories without code changes.
- **Files to modify:**
    - `src/main.rs` (setup dependency injection)
    - Any service initialization code

### Milestone 5: Create Command Line Storage Backend Selection

- **Task:** Introduce a command line argument to select the storage backend.
- **Details:**
    - Add a `--storage` argument that accepts `neo4j` (default) or `filesystem` values.
    - This allows users to easily switch between storage backends at runtime without code changes.
    - The selection will be checked in the main application logic to determine which repository implementations to inject into services.
    - Example usage: `reservoir --storage filesystem start` or `reservoir --storage neo4j view 10`
- **Files to modify:**
    - `src/args.rs` (add StorageBackend enum and --storage argument)
    - `src/main.rs` (read storage argument and configure dependency injection accordingly)

## 4. Long-Term Vision

This implementation is the first step towards migrating Reservoir to a fully file-based storage system that follows modern Unix conventions. The XDG-compliant approach provides several benefits:

- **Better User Experience:** Users can easily locate their data (`~/.local/share/reservoir/`) separately from cache (`~/.cache/reservoir/`), making backups and data management more intuitive.
- **System Integration:** The XDG structure integrates well with modern desktop environments and system tools that understand these conventions.
- **Selective Data Management:** Users can safely clear cache directories without losing important message data, and backup tools can focus on data directories.

Future work will involve:

- **User Experience:** The command line interface allows users to choose their preferred storage backend based on their needs:
  - `--storage neo4j`: For users who want advanced graph relationships and vector search capabilities
  - `--storage filesystem`: For users who prefer simple, portable, text-based storage that can be easily backed up and inspected
- **Optimizing file I/O:** Using memory-mapped files (`memmap2`) for better performance.
- **Implementing advanced querying:** Developing a system for querying data across multiple files.
- **Configuration Management:** Adding support for user-configurable settings in `$XDG_CONFIG_HOME/reservoir/`.
- **Migration Tools:** Adding commands to migrate data between storage backends (e.g., `reservoir migrate --from neo4j --to filesystem`).
- **Phasing out Neo4j:** Once the file-based system is stable and feature-complete, we may consider making filesystem the default backend.
