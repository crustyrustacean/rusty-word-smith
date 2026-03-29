// src/main.rs

// dependencies
use std::fs;

fn write_html(input_file: &str) -> std::io::Result<()> {
    // read the input file
    let markdown_input = fs::read_to_string(input_file)?;

    // Create parser with example Markdown text.
    let parser = pulldown_cmark::Parser::new(&markdown_input);

    // Write to a new String buffer.
    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);

    // Write the resulting HTML file to disk.
    fs::write("index.html", html_output.as_bytes())?;
    println!("Wrote: {} to index.html", html_output);

    Ok(())
}

fn main() -> std::io::Result<()> {
    let input_file = "_index.md";
    println!("Read: {}", input_file);

    write_html(input_file)?;

    Ok(())
}
