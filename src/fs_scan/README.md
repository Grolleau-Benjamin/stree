# Arena-Based Tree Construction

## Overview

STree builds its internal directory structure using an arena allocator rather than a fully recursive, heap-nested tree.
This design provides high performance, avoids unnecessary cloning, and eliminates the borrow-checking complexity that
comes with recursive data ownership in Rust.

## Benefits for STree

- **Performance**: Large project trees are built and traversed faster due to contiguous memory and zero clone overhead.
- **Safety**: No unsafe code, no cyclic references, and no borrow-checker issues.
- **Simplicity**: Modifying or pruning parts of the tree is straightforward since all nodes live in a single arena.

## Materialization phase
Once the entire arena is constructed, STree performs a materialization pass,  converting arena indices into a standard
recursive Node tree used by renderers (for pretty printing, colorization, or JSON export).

## Benchmark
```
time ./target/release/stree ~/Desktop/COSMY/monorepo/
./target/release/stree ~/Desktop/COSMY/monorepo/  0.00s user 0.01s system 92% cpu 0.016 total

./target/release/stree ~/Desktop/COSMY/monorepo/ | wc -l
925


time ./target/release/stree ~/Desktop/COSMY/monorepo/ --hidden-files --gitignore
./target/release/stree ~/Desktop/COSMY/monorepo/ --hidden-files --gitignore  0.32s user 1.62s system 46% cpu 4.138 total

./target/release/stree ~/Desktop/COSMY/monorepo/ --hidden-files --gitignore | wc -l
225374
```

It took less than 0.01 second on a MacBook Pro M4 to traverse a project containing approximately 925 visible files,
while the standard tree command took about 2 seconds on the same repository.

When running with --hidden-files --gitignore, STree scans the entire monorepo — including .git/, build caches, and hidden dependencies — resulting in more than 225,000 filesystem entries.
Even in this extreme case, STree completes the traversal in roughly 4 seconds, fully parsing the directory structure and producing a complete in-memory tree representation.
