use std::fs;
fn main() -> std::io::Result<()> {
    // download the target HTML document
    let response = reqwest::blocking::get("https://provbiz.ru/");
    // get the HTML content from the request response
    // and print it
    let html_content = response.unwrap().text().unwrap();
    fs::write("quote.txt", &html_content)?;
    println!("{html_content}");
    println!("len == {}", &html_content.len());
    Ok(())
}
