Simple alternative to linux find command.

Features:
Fuzzy matching
Extension filtering
Depth control
Predefined paths

Command-Line Args
-n, --name <FILE_NAME>
Name of the file to search for (supports fuzzy matching). 

-p, --path <SEARCH_PATH>
Directory path to start the search. Use predefined paths like _desktop, _documents, etc., or provide a custom path. Required.

-e, --ext <EXTENSIONS>
Optional file extensions to filter by (comma-separated).

-d, --depth <MAX_DEPTH>
Maximum depth to search within the directory (optional, 2 is the default).

Example
Search for files with names containing "example" in the desktop directory (_desktop) and filter to include only .txt and .rs files:

findme --name example --path _desktop --ext txt,rs -d 4
