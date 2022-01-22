mod msg;

fn main() -> Result<(),Box<dyn std::error::Error>>{
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("missing url")
    }

    let url = &args[1];
    let output = "rust.md";

    println!("Fetching url: {}", url);
    let body = reqwest::blocking::get(url)?.text()?;

    println!("Converting html to markdown...");
    let md = html2md::parse_html(&body);

    std::fs::write(output, md.as_bytes())?;
    println!("Converted markdown has been saved in {}", output);

    Ok(())
}

