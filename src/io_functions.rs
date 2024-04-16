// Importing required libraries and modules
use std::{env, fs, io::Write, path::Path};
use anyhow::{bail, Result}; // Importing `anyhow` for error handling
use chrono::prelude::Local; // Importing `chrono` for date-time manipulation
use glob::glob; // Importing `glob` for file matching
use whoami::{platform, Platform}; // Importing `whoami` for platform detection

// Fetch HTML data from Google search
pub fn fetch(query: String, lang: String) -> Result<String, ureq::Error> {
    // Make a GET request to Google search
    let x = ureq::get("https://google.com/search")
        .set(
            "User-Agent",
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) QtWebEngine/5.15.2 Chrome/87.0.4280.144 Safari/537.36"
        )
        .query("q", &query)
        .query("hl", &lang)
        .call()? // Perform the request
        .into_string() // Convert the response to a string
        .unwrap(); // Unwrap the result
    Ok(x) // Return the fetched HTML data
}

// Retrieve cached HTML data
pub fn cached_html() -> Result<String> {
    let os_type = platform(); // Detect the current platform
    let files = get_file_list(os_type)?; // Get the list of cached files
    let html = fs::read_to_string(&files[files.len() - 1])?; // Read the last cached file
    Ok(html) // Return the cached HTML data
}

// Save HTML data to cache
pub fn save_html(query: &[&str], html: &str) -> Result<String> {
    let os_type = platform(); // Detect the current platform
    let cache_path = get_cache_path(&os_type)?; // Get the cache path
    let file_date = Local::now().format("%s").to_string(); // Get the current date and time
    let file_query = query.join("_"); // Join query parts with underscores
    let sep = sep_type(&os_type); // Get the separator based on the platform

    // Construct the file path
    let mut x: Vec<&str> = vec![&cache_path, sep, "pooi"];
    if !Path::new(&x.join("")).is_dir() {
        fs::create_dir(&x.join(""))?; // Create the directory if it doesn't exist
    }
    x.push(sep);
    x.push(&file_date);
    x.push("-");
    x.push(&file_query);
    x.push(".html");

    // Create and write to the file
    let full_path = x.join("");
    let mut file = fs::File::create(&full_path)?;
    file.write_all(html.as_bytes())?;
    Ok(full_path) // Return the path of the saved file
}

// Clean the cache directory
pub fn clean_cache() -> Result<String> {
    let os_type = platform(); // Detect the current platform
    let sep = sep_type(&os_type); // Get the separator based on the platform
    let target = [&get_cache_path(&os_type)?, sep, "pooi"].join(""); // Construct the cache directory path
    fs::remove_dir_all(&target)?; // Remove the cache directory and its contents
    Ok(target) // Return the path of the cleaned cache directory
}

// Determine the separator based on the platform
fn sep_type(os_type: &Platform) -> &str {
    match os_type {
        Platform::Windows => "\\",
        _ => "/"
    }
}

// Get the cache path based on the platform
fn get_cache_path(os_type: &Platform) -> Result<String> {
    // Determine cache path based on platform
    let cache_path: String = match os_type {
        Platform::Bsd | Platform::Linux => match env::var("XDG_CACHE_HOME") {
            Ok(x) => x,
            Err(_) => {
                let home_path = env::var("HOME")?;
                let x = [home_path, "/.cache".to_string()];
                x.join("")
            }
        },
        Platform::MacOS => {
            let home_path = env::var("HOME")?;
            let x = [home_path, "/Library/Application Support".to_string()];
            x.join("")
        }
        Platform::Windows => env::var("LOCALAPPDATA")?,
        _ => bail!("This feature is not supported on your platform, sorry!")
    };
    Ok(cache_path) // Return the cache path
}

// Get a list of cached file paths
fn get_file_list(os_type: Platform) -> Result<Vec<String>> {
    let sep = sep_type(&os_type); // Get the separator based on the platform
    let cache_path = [&get_cache_path(&os_type)?, sep, "pooi", sep, "*.html"].join(""); // Construct the glob pattern
    let mut files: Vec<String> = vec![];
    for x in glob(&cache_path).unwrap() {
        match x {
            Ok(x) => files.push(x.display().to_string()), // Add valid paths to the list
            Err(_) => panic!("get_file_list: glob search failed!?!")
        }
    }

    match files.len() {
        0 => bail!("Can't find any cached results, sorry!"), // Return error if no cached files found
        _ => Ok(files) // Return the list of cached file paths
    }
}
