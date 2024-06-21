# findme

**findme** is a command-line tool written in Rust as a simple alternative to the Linux `find` command, designed for searching files with fuzzy name matching and filtering by extensions.

## Features

- **Fuzzy matching**: Search for files with names that approximate the given search string.
- **Extension filtering**: Narrow down search results to files with specific extensions.
- **Depth control**: Specify the maximum depth to search within directories.
- **Predefined paths**: Use convenient shortcuts for common directories.

## Command-Line Arguments

- **-n, --name <FILE_NAME>**  
  Name of the file to search for (supports fuzzy matching).

- **-p, --path <SEARCH_PATH>**  
  Directory path to start the search. Use predefined paths like `_desktop`, `_documents`, etc., or provide a custom path. **Required**.

- **-e, --ext <EXTENSIONS>**  
  Optional file extensions to filter by (comma-separated).

- **-d, --depth <MAX_DEPTH>**  
  Maximum depth to search within the directory (optional, default is 2).

## Example

Search for files with names containing "example" in the desktop directory (_desktop) and filter to include only .txt and .rs files:

```bash
findme --name example --path _desktop --ext txt,rs -d 4
