use std::path::PathBuf;
use clap::{App, Arg};
use walkdir::WalkDir;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;

struct PredefinedPaths {
    paths: std::collections::HashMap<&'static str, PathBuf>,
}

impl PredefinedPaths {
    fn new() -> Self {
        let mut paths = std::collections::HashMap::new();
        paths.insert("_home", dirs::home_dir().unwrap_or_default());
        paths.get_mut("_home").unwrap().push(".config");
        paths.insert("_etc", PathBuf::from("/etc"));
        paths.insert("_desktop", dirs::desktop_dir().unwrap_or_default());
        paths.insert("_documents", dirs::document_dir().unwrap_or_default());
        paths.insert("_downloads", dirs::download_dir().unwrap_or_default());
        paths.insert("_config", dirs::config_dir().unwrap_or_default());
        // idk what more to add 

        Self { paths }
    }

    fn resolve_path(&self, key: &str) -> Option<PathBuf> {
        self.paths.get(key).cloned()
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let predefined_paths = PredefinedPaths::new();

    let matches = App::new("findme")
        .version("1.0")
        .author("Yassine")
        .about("Search for files with a fuzzy match on the name and filter by extensions.")
        .arg(Arg::with_name("name")
            .short('n')
            .long("name")
            .value_name("FILE_NAME")
            .help("Name of the file to search for (supports fuzzy matching)")
            .required(true)
            .takes_value(true))
        .arg(Arg::with_name("path")
            .short('p')
            .long("path")
            .value_name("SEARCH_PATH")
            .help("Directory path to start the search (use predefined paths like _desktop)")
            .takes_value(true))
        .arg(Arg::with_name("ext")
            .short('e')
            .long("ext")
            .value_name("EXTENSIONS")
            .help("Optional file extensions to filter by (comma-separated, e.g., txt,rs)")
            .takes_value(true))
        .arg(Arg::with_name("depth")
            .short('d')
            .long("depth")
            .value_name("MAX_DEPTH")
            .help("Maximum depth to search within the directory (optional)")
            .takes_value(true))
        .get_matches();

    let file_name = matches.value_of("name").unwrap();
    let search_path = match matches.value_of("path") {
        Some(path) => {
            if let Some(predefined_path) = predefined_paths.resolve_path(path) {
                predefined_path
            } else {
                PathBuf::from(path)
            }
        },
        None => {
            eprintln!("Error: Missing required argument '--path'");
            return Ok(());
        }
    };

    let max_depth = matches.value_of("depth").map(|d| d.parse::<usize>().unwrap_or(2)).unwrap_or(2); 
    let file_extensions = matches.value_of("ext")
        .map(|exts| exts.split(',').map(|s| s.trim().to_lowercase()).collect::<Vec<String>>());

    let matcher = SkimMatcherV2::default();
    let mut file_count = 0;

    for entry in WalkDir::new(&search_path).max_depth(max_depth).into_iter().filter_map(Result::ok) {
        let path = entry.path();
        if path.is_file() {
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if matcher.fuzzy_match(name, file_name).is_some() {
                    if let Some(exts) = &file_extensions {
                        if exts.is_empty() || exts.iter().any(|ext| {
                            if let Some(file_ext) = path.extension().and_then(|e| e.to_str()) {
                                ext == &file_ext.to_lowercase()
                            } else {
                                false
                            }
                        }) {
                            println!("Found file: {}", path.display());
                            file_count += 1;
                        }
                    } else {
                        println!("Found file: {}", path.display());
                        file_count += 1;
                    }
                }
            }
        }
    }

    if file_count > 0 {
        println!("Total files found: {}", file_count);
    } else {
        println!("No result found");
    }

    Ok(())
}
