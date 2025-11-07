## ⚖️ Comparison with Eza

[Eza](https://github.com/eza-community/eza) is a modern replacement for `ls` that includes a `-T` flag to display directories in a tree-like format. While this overlaps with Arbor’s visual output, the goals of both tools are fundamentally different:

| Feature                        | eza -T                               | Arbor                                 |
| ------------------------------ | ------------------------------------ | ------------------------------------- |
| Focus                          | File listing with optional tree mode | Full project structure exploration    |
| `.gitignore` handling          | ❌                                    | ✅                                     |
| Git status integration         | ✅ (basic)                            | ✅ (detailed: branch, status, ignored) |
| Configurable depth (`--depth`) | ✅                                    | ✅                                     |
| Icons                          | ✅                                    | ✅                                     |
| Hidden files toggle            | ✅                                    | ✅                                     |
| JSON export                    | ❌                                    | ✅                                     |
| Count mode (`--count`)         | ❌                                    | ✅                                     |
| Performance on large repos     | ⚙️ Optimized for listings             | ⚙️ Optimized for traversal             |
| Primary use case               | Better `ls`                          | Smarter `tree`                        |

Arbor focuses on project-level insight, combining `tree` clarity and `git` awareness,
whereas Eza’s `-T` is designed for quick directory overviews inside a file listing tool.

## ⚙️ Performance benchmark

A quick performance comparison between **Arbor** and **Eza** using `gtime` on macOS M4 Pro (averaged over 3 runs):

| Command                                  | Elapsed | CPU   | Memory |
| ---------------------------------------- | ------- | ----- | ------ |
| `arbor` (default)                        | 0.00    | 66 %  | 3MB    |
| `arbor --show-gitignored --show-hiddens` | 0.17 s  | 98 %  | 11 MB  |
| `eza -T`                                 | 0.26 s  | 193 % | 62 MB  |

> [!NOTE]
>
> Arbor’s traversal engine is optimized for filesystem scanning and Git awareness. It consumes up to 5× less memory and completes 30 % faster than `eza -T` in equivalent tree mode.
