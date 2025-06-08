# NYX

A project manager in CLI using a lightweight storage system.

## Project Data File

NYX uses a unique file system called NXFS (Nyx File System). NXFS works using a .nxfs directory in the NYX source code. All project data is stored there. There is two object file type in NXFS:

- nxs: it's an index file containing information about all stored projects.
- nxp: a unique file per project. Containing all projects informations.

They are unique binary file format using a strict defined structure. Using a header and a content.

## Possible errors

# Exit Codes

## 💡 Principles:
- **0**: Success.
- **1-2**: General errors.
- **3-9**: Argument and command errors.
- **10-29**: Execution errors (files, access).
- **30-49**: System or environment issues.
- **50+**: Nyx-specific errors.

| Exit Code| Description                            | Example Use                                          |
|----------|----------------------------------------|------------------------------------------------------|
| **0**    | Success                                | Command executed successfully.                       |
| **1**    | General error                          | Undefined or uncategorized error.                    |
| **2**    | Misuse of shell builtins               | Syntax error or incorrect usage.                     |
| **3**    | Invalid argument                       | Unrecognized or incorrectly formatted argument.      |
| **4**    | Missing argument                       | An expected argument is missing.                     |
| **5**    | Unsupported command                    | The specified command or subcommand is not supported.|
| **6**    | Subcommand failure                     | A subcommand called by Nyx has failed.               |
| **7**    | Command conflict                       | Conflict between multiple options or commands.       |
| **10**   | File not found                         | The specified file or project does not exist.        |
| **11**   | Permission denied                      | Unable to access or modify a file.                   |
| **12**   | File system error                      | Error during file read or write.                     |
| **13**   | Invalid file format                    | File is unreadable or corrupted.                     |
| **20**   | Missing environment dependency         | A required tool or binary is missing (e.g., `git`, `pbcopy`). |
| **21**   | Incompatible environment               | Rust version or another component is incorrect.      |
| **30**   | Configuration error                    | Configuration file is unreadable or malformed.       |
| **31**   | Inconsistent state                     | File or directory in an unexpected state.            |
| **40**   | Network error                          | Failed to reach an external resource.                |
| **50**   | Nyx-specific error (generic)           | Internal tool error (e.g., project parsing failure). |
| **51**   | NXFS error                             | Error related to the Nyx File System.                |
| **52**   | TUI error                              | Issue with interactive display or UI rendering.      |
| **60**   | Health check failed                    | An environment check has failed.                     |

## License

This project is licensed under the NYX Custom License.  
- Free for personal and educational use  
- Forking and modifying is allowed  
- Commercial use is not allowed  
- Attribution to the author is required

Check the [LICENSE.md](./LICENSE.md) file for the full text.
