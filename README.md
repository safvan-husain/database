# Simple Rust Indexing System

This is a small personal Rust project to experiment with how a basic indexing system could work in a database-like setting. The goal is to write and read content to and from storage using manually managed offsets, with a focus on serialization, indexing, and basic memory reuse.

## ✨ Features

- ✅ **Indexing content** into a storage file using offset and length.
- ✅ **Retrieving content** via an index.
- ✅ **Updating content**, marking the old space as free.
- ✅ **Reusing freed space** when possible (basic free space management).
- ✅ **Serialization and deserialization** of custom index data.

## 🗂 Structure

The core logic is housed under the `utils` module, with components like:

- `index.rs`: Handles creating, updating, retrieving, and managing indexes.
- `database.rs`: A simple wrapper for working with data stored in files.
- `lib.rs`: This file contain test for the functionalities.

## 🧺as Overview

### `serialization_des`

Tests if the `Index` struct can be correctly serialized into bytes and deserialized back without data loss.

### `save`

This test does the following:

1. Initializes empty databases for index and content.
2. Saves two identical content strings, confirming they are stored as separate entries with different indexes.
3. Retrieves and asserts their values, offsets, and lengths.
4. Updates one of the entries with a longer string.
5. Verifies that the original index is marked as free.
6. Adds new content and checks if:
   - It reuses free space where appropriate.
   - Index IDs reflect the reuse or continuation logic.

## 🔍 Example Use Case

This kind of system can be useful when:

- You want more control over how data is stored (e.g., embedded systems or minimal databases).
- You want to learn about memory management and file handling in a low-level language.
- You're building a toy DB or experimenting with efficient file-backed storage.

## 🚧 Disclaimer

This project is a learning experiment and not production-ready. It assumes single-threaded access and lacks many safeguards you'd expect in a real system (like crash safety or concurrent access).

