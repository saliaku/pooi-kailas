// Import required modules and libraries
use std::{env, io::Error};

// Import clap_generate crate for completion file generation
use clap_generate::{
    generate_to,
    generators::{Bash, Fish, PowerShell, Zsh}
};

// Import the build function from src/cli.rs to create the CLI app
include!("src/cli.rs");

fn main() -> Result<(), Error> {
    // Define the list of selectors that can be used with the 'pooi' command
    let selectors = [
        "basic1", 
        "basic2", 
        "clock", 
        "conversions", 
        "currency",
        "define", 
        "holidays", 
        "lists", 
        "lyrics", 
        "maths",
        "pronounce", 
        "snippets1", 
        "snippets2", 
        "sports", 
        "summary",
        "translate", 
        "weather"
    ];

    // Build the CLI app using the build function from src/cli.rs
    let mut app = build(&selectors);

    // Determine the output directory for the generated completion files
    let outdir = match env::var_os("OUT_DIR") {
        None => return Ok(()),  // If OUT_DIR environment variable is not set, return Ok
        Some(outdir) => outdir  // Otherwise, use the specified output directory
    };

    // Generate Bash completion file and print a warning message
    let bash = generate_to(Bash, &mut app, "pooi", &outdir)?;
    println!("cargo:warning=completion file is generated: {:?}", bash);

    // Generate Fish completion file and print a warning message
    let fish = generate_to(Fish, &mut app, "pooi", &outdir)?;
    println!("cargo:warning=completion file is generated: {:?}", fish);

    // Generate PowerShell completion file and print a warning message
    let powershell = generate_to(PowerShell, &mut app, "pooi", &outdir)?;
    println!("cargo:warning=completion file is generated: {:?}", powershell);

    // Generate Zsh completion file and print a warning message
    let zsh = generate_to(Zsh, &mut app, "pooi", &outdir)?;
    println!("cargo:warning=completion file is generated: {:?}", zsh);

    Ok(())  // Return Ok to indicate successful completion
}
