# Change Proposal Document: Implementing File-Based Storage in Reservoir

**Author(s):** Divan Visagie
**Date:** 2025-07-06
**Status:** Draft

## 1. Summary

This document outlines the plan to implement a file-based storage system in Reservoir as an alternative to the current Neo4j storage system. This is to align more with the envisioned use case of the system which is primarily designed to run on a client machine operated by the user. It also aligns with Sector F's design philosophy of sticking with the original principals devised at Bell Labs of preferring data to be stored as text rather than as binary. This allows flexibility for users who want to explore or use the data produced by reservoir in ways it may be impossible for the original program designers to predict.

This will involve creating new repository implementations that write to the filesystem, adhering to traditional Unix conventions.

## 2. Research

In developing our file-based storage system, we will adhere to a consistent and predictable filesystem standard. Our approach is guided by the following principles:

-   **Filesystem Standard**: We have considered both the modern [XDG Base Directory Specification](https://specifications.freedesktop.org/basedir-spec/basedir-spec-latest.html) and the traditional Unix convention for storing user-specific application data. While the XDG standard aims to reduce clutter in the home directory, we have chosen to adopt the more traditional approach for its simplicity and universal recognition across all Unix-like systems (Linux, macOS, BSD). This convention, where an application stores its data in a single hidden directory (a "dotfile") within the user's home directory, is a long-standing de-facto standard. 

-   **Data Storage**:
    -   **Base Directory**: All Reservoir data will be stored within `~/.reservoir/`.
    -   **Permanent Data (Messages)**: Permanent message data will be stored in `~/.reservoir/messages/`.
    -   **Temporary Data (Embeddings)**: Semi-temporary data like embeddings will be stored in `~/.reservoir/embeddings/`. To support multiple embedding models simultaneously, this directory will contain subdirectories named after the model used to generate the embeddings (e.g., `~/.reservoir/embeddings/text-embedding-ada-002/`).

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

- **Task:** Create utility functions to resolve and manage the `~/.reservoir/` directory.
- **Details:**
    - The utility will expand the `~` character to the user's home directory path.
    - It will also ensure the creation of the `messages` and `embeddings` subdirectories.
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

This implementation is the first step towards migrating Reservoir to a fully file-based storage system. Future work will involve:

- **Optimizing file I/O:** Using memory-mapped files (`memmap2`) for better performance.
- **Implementing advanced querying:** Developing a system for querying data across multiple files.
- **Phasing out Neo4j:** Once the file-based system is stable and feature-complete, we will begin the process of removing the Neo4j dependency.
