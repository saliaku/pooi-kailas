// Importing required libraries and modules

// Import standard library modules
use std::{io::stdout, process::exit};

// Import crossterm library modules for terminal handling and styling
use crossterm::{style::Stylize, terminal::size, tty::IsTty};

// Import scraper library modules for HTML parsing
use scraper::{Html, Selector};

// Import whoami library module for language detection
use whoami::lang;

// Import custom modules from the project directory
mod cli;            // Module containing the Command Line Interface configuration and options
mod io_functions;  // Module containing I/O functions like file reading, writing, etc.
mod selectors;     // Module containing selectors for scraping and querying



fn main() {
    // Get the main selector details
    let main_array = selectors::details(); //details funtion is in /selectors/mod.rs
                                                                           //which contain the details of all the selectors
    let mut selector_list = vec![]; //it is a vector, similar to an array. It is used to store the names of the selectors
    
    // Populate the selector list from the main array
    for x in &main_array {
        selector_list.push(x.0) //fetches all the names of the selectors and stores the names of these in this selector_list
    }
    
    // Parse command-line arguments
    let args = cli::build(&selector_list).get_matches();//constructs the cli with the selector list and gets the matches

    // Handle 'list' flag
    if args.is_present("list") {
        selectors::print_list(main_array);
        exit(0); // Exit the application after listing selectors
    }

// Handle 'clean' flag to clean cache
if args.is_present("clean") {
    // Check if the 'clean' flag is present in the command-line arguments

    match io_functions::clean_cache() {
        // Call the clean_cache function from io_functions module
        // clean_cache function likely removes cache files or directories

        Ok(r) => {
            // If cleaning is successful
            println!(
                "{} The directory {} and it's contents have been removed!",
                "success:".green().bold(),
                r.blue()
            );
            // Print a success message indicating which directory and its contents were removed
            exit(0); // Exit the application after cleaning cache with success code
        }
        Err(e) => {
            // If cleaning encounters an error
            eprintln!("{} {}", "error:".red().bold(), e);
            // Print an error message indicating the nature of the error
            exit(1); // Exit the application with an error code
        }
    }
}


    // Determine if cache should be used and get query
let use_cache = args.is_present("cache");
// Check if the 'cache' flag is present in the command-line arguments and store the result in use_cache

let query: Vec<&str> = match use_cache {
    true => vec![],
    // If use_cache is true (meaning the 'cache' flag is present)
    // set query to an empty vector since we don't need to execute a query when using cache

    false => args.values_of("query").unwrap().collect()
    // If use_cache is false (meaning the 'cache' flag is not present)
    // get the values associated with the 'query' flag from the command-line arguments
    // and collect them into a vector of string slices (&str)
};


// Check for missing query and display error
if query.len() == 1 && query[0] == "-" {
    // Check if there is exactly one query and its value is "-"
    
    println!(
        "{} The following required arguments were not provided:\n    {}\n\nUSAGE:\n    pooi \
         <query>...\n\nFor more information try {}\nThank you",
        "error:".red().bold(),
        "<query>...".green(),
        "--help".green()
    );
    // If the condition is met, print an error message indicating that the required arguments are missing
    // The message also provides usage information and suggests using '--help' for more details

    exit(1);
    // Exit the application with an error code
}


    // Determine terminal type and size
    let mut tty = stdout().is_tty();
    let tty_size = size().unwrap_or((0, 0));
    let w: usize = match tty_size.0 {
        0 if tty => panic!("main: can't determine terminal size"),
        0 => 0,
        1..=100 => tty_size.0.into(),
        _ => 100
    };

    // Override tty setting if 'raw' flag is present
    if args.is_present("raw") {
        tty = false;
    }

    // Determine verbosity based on tty
    let quiet = match tty {
        true => args.is_present("quiet"),
        false => true
    };

   // Fetch HTML data based on cache and query
let html = match use_cache {
    // If the "cache" flag is present in the command-line arguments
    true => {
        // Attempt to retrieve cached HTML data
        match io_functions::cached_html() {
            Ok(r) => {
                // If successful, return the cached HTML
                r
            },
            Err(e) => {
                // If an error occurs while fetching cached HTML, print the error and exit with an error code
                eprintln!("{} {}", "error:".red().bold(), e);
                exit(1);
            }
        }
    },
    // If the "cache" flag is not present
    false => {
        // Determine the language for the query
        let lang = match args.is_present("language") {
            // If the "language" flag is present
            true => {
                // Get the value provided for the "language" flag and convert it to a String
                args.value_of("language").unwrap().to_string()
            },
            // If the "language" flag is not present
            false => {
                // Use the default language, or "en-US" if no default is available
                lang().next().unwrap_or_else(|| "en-US".to_string())
            }
        };

        // Fetch HTML data from Google based on the query and language
        match io_functions::fetch(query.join(" "), lang) {
            Ok(r) => {
                // If successful, return the fetched HTML
                r
            },
            Err(_) => {
                // If an error occurs while fetching HTML from Google, print an error message and exit with an error code
                eprintln!("{} No response from google, sorry!", "error:".red().bold());
                exit(1);
            }
        }
    }
};


    // Save HTML data if 'save' flag is present
    if args.is_present("save") {
        match io_functions::save_html(&query, &html) {
            Ok(r) => match tty {
                true => println!(
                    "{}\n    {}\n",
                    "HTML for the query has been saved to the following path:".dark_grey(),
                    r.blue()
                ),
                false => {}
            },
            Err(e) => eprintln!("{} {}\n", "error:".red().bold(), e)
        }
    }

    // Get selectors based on command line or default list
let mut selectors = match args.is_present("selectors") {
    // If the "selectors" flag is present in the command-line arguments
    true => {
        // Collect all the values provided for the "selectors" flag into a Vec<&str>
        args.values_of("selectors").unwrap().collect()
    },
    // If the "selectors" flag is not present
    false => {
        // Use the default `selector_list` as the selectors
        selector_list.clone()
    }
};

// Ensure that the "corrections" selector is always included
selectors.push("corrections");


// Parse HTML data into a format that can be searched
let data = Html::parse_document(&html);

// Initialize an empty vector to store the answers
let mut answers = vec![];

// Iterate over the selectors to find matches in the parsed HTML data
for x in &selectors {
    // Determine the selector ID based on the selector name
    let y = match *x {
        "corrections" => {
            // Get the selector ID for "corrections"
            selectors::name_to_id("corrections")
        },
        _ => {
            // Find the position of the current selector in the default selector list
            let p = selector_list.iter().position(|&r| r == *x).unwrap();
            // Get the selector ID from the main_array using the found position
            main_array[p].3
        }
    };

    // Check if the selector is present in the parsed HTML data
    if data.select(&Selector::parse(y).unwrap()).next().is_some() {
        // Handle special case for "holidays" to prevent false positives
        match *x == "holidays" {
            // If the selector is "holidays" and matches the special case condition
            true if data
                .select(&Selector::parse("div.wDYxhc").unwrap())
                .nth(1)
                .unwrap()
                .value()
                .attr("data-attrid")
                .unwrap()
                == "kc:/public_events:holidays_for_date" =>
            {
                // Add "holidays" to the answers vector
                answers.push(*x)
            },
            // If the selector is "holidays" but doesn't match the special case condition
            true => {},
            // For all other selectors
            false => {
                // Add the selector to the answers vector
                answers.push(*x)
            }
        }
    }
}


    // Get the total number of answers found
let total = answers.len();

// Match on the total number of answers
match total {
    // If there are no answers found
    0 => {
        // Call the no_result function to handle cases with no results
        // Passes parameters for terminal type (`tty`), width (`w`), parsed HTML data (`data`), 
        // quiet mode (`quiet`), and a flag indicating no actual results (`false`)
        no_result(tty, w, data, quiet, false);
        
        // Exit the application with an error code
        exit(1);
    },
    
    // If there is exactly one answer found and it is "corrections"
    1 if answers[0] == "corrections" => {
        // Call the no_result function to handle cases with no results
        // Passes parameters for terminal type (`tty`), width (`w`), parsed HTML data (`data`), 
        // quiet mode (`quiet`), and a flag indicating it's for "corrections" (`true`)
        no_result(tty, w, data, quiet, true);
        
        // Exit the application with an error code
        exit(1);
    },
    
    // For all other cases (more than one answer found)
    _ => {}
}


   // Initialize a flag to track if corrections are handled
let mut corrected = false;

// Check if the last answer is "corrections"
if answers[total - 1] == "corrections" {
    // If the last answer is "corrections", set the corrected flag to true
    corrected = true;

    // Check if the quiet mode is not enabled
    if !quiet {
        // Call the corrections function to handle corrections
        corrections(&data);
    }

    // Remove "corrections" from the answers list
    answers.pop();
}

// Clone the answers to preserve the original list for filtering
let matches = answers.clone();

// Check if the 'all' flag is not present and there are more than one answer
if !args.is_present("all") && total > 1 {
    // Determine the reference query based on whether corrections were applied or not
    let r_query = match corrected {
        // If corrections were applied, get the text associated with the corrections selector
        true => data
            .select(&Selector::parse(selectors::name_to_id("corrections")).unwrap())
            .next()
            .unwrap()
            .text()
            .collect::<Vec<&str>>()
            .join(""),

        // If no corrections were applied, determine the reference query based on cache usage
        false => match use_cache {
            // If cache was used, use the title from the cached HTML data
            true => {
                // Extract the title from the HTML data
                let x = data
                    .select(&Selector::parse("title").unwrap())
                    .next()
                    .unwrap()
                    .text()
                    .collect::<Vec<&str>>()
                    .join("");

                // Split the title into words and remove the last three words
                let mut y = x.split(' ').collect::<Vec<&str>>();
                for _ in 0..3 {
                    y.pop();
                }
                y.join(" ")
            },

            // If cache was not used, use the provided query
            false => query.join(" ")
        }
    };

    // Filter the answers based on the reference query
    answers = selectors::filter(answers, r_query);
}

    // Print the final answers
    selectors::print_answer(&data, answers, &tty, w, &quiet, matches);

    // Print URLs if 'urls' flag is present
    if args.is_present("urls") {
        print_urls(w, data);
    }
}

// Handle cases with no results
fn no_result(tty: bool, w: usize, data: scraper::Html, quiet: bool, corrected: bool) {
    match tty {
        true => match quiet {
            true => println!("{} Sorry about that!", "No result:".red().bold()),
            false => {
                if corrected {
                    corrections(&data)
                }
                println!(
                    "{} Perhaps one of these links might help?",
                    "No result:".bold().red()
                );
                print_urls(w, data)
            }
        },
        false => eprintln!("No result!")
    }
}

// Handle corrections
fn corrections(data: &scraper::Html) {
    let x = data
        .select(&Selector::parse(selectors::name_to_id("corrections")).unwrap())
        .next()
        .unwrap();
    let foo = x.inner_html();
    let bar = x.text().collect::<Vec<&str>>().join("");

    let html = foo.split(' ').collect::<Vec<&str>>();
    let text = bar.split(' ').collect::<Vec<&str>>();
    let total = html.len();

    assert_eq!(total, text.len());

    print!("{}", "I'll assume you meant this: ".dark_grey());
    for i in 0..total {
        match html[i] == text[i] {
            true => print!("{} ", text[i]),
            false => print!("{} ", text[i].bold().cyan())
        }
    }
    println!();
}

// Print URLs
fn print_urls(w: usize, data: scraper::Html) {
    let selector = Selector::parse(selectors::name_to_id("url_block")).unwrap();
    let mut url_blocks = data.select(&selector);

    if url_blocks.next().is_some() {
        for x in url_blocks {
            let title_selector = Selector::parse(selectors::name_to_id("title")).unwrap();
            let title = x.select(&title_selector).next().unwrap().text().collect::<Vec<&str>>();

            let url_selector = Selector::parse(selectors::name_to_id("url")).unwrap();
            let url_element = x.select(&url_selector).next().unwrap();
            let href = url_element
                .first_child()
                .and_then(|element| element.value().as_element())
                .and_then(|element| element.attr("href"))
                .unwrap_or("No URL available");

            let desc_selector = Selector::parse(selectors::name_to_id("desc")).unwrap();
            let desc_check = x.select(&desc_selector).next();
            let description = match desc_check {
                Some(y) => y.text().collect::<Vec<&str>>().join(""),
                None => "No description available, sorry!".to_string(),
            };

            println!(
                "\n{}\n{}\n{}",
                title.join("").bold().blue(),
                href,
                format_desc(w, description).dark_grey()
            );
        }
    } else {
        println!("{}", "jk, there are no links!".dark_grey());
    }
}

// Format description to fit terminal width
fn format_desc(length_max: usize, desc: String) -> String {
    if desc.len() < length_max {
        return desc;
    }
    let mut length = 0;
    let mut desc_build = vec![];
    let mut r: Vec<String> = vec![];

    let desc_words: Vec<&str> = desc.split(' ').collect();
    for x in &desc_words {
        match (x.len() + length) >= length_max {
            true => {
                r.push(desc_build.join(" "));
                desc_build.clear();
                length = x.len() + 1;
            }
            false => length += x.len() + 1
        }
        desc_build.push(*x);
    }
    r.push(desc_build.join(" "));
    r.join("\n")
}
