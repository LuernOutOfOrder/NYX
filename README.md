# NYX

A project manager in CLI using a lightweight storage system.

## Project Data File

NYX uses a unique file system called NXFS (Nyx File System). NXFS works using a .nxfs directory in the NYX source code. All project data is stored there. There is two object file type in NXFS:

- nxs: it's an index file containing information about all stored projects.
- nxp: a unique file per project. Containing all projects informations.

They are unique binary file format using a strict defined structure. Using a header and a content.
