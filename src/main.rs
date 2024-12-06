/*
use std::fs;
fn main() -> std::io::Result<()> {
    // download the target HTML document
    let response = reqwest::blocking::get("https://provbiz.ru/");
    // get the HTML content from the request response
    // and print it
    let html_content = response.unwrap().text().unwrap();
    // fs::write("quote.txt", &html_content)?;
    println!("{html_content}");
    // println!("len == {}", &html_content.len());
    Ok(())
}
*/
use std::fs;
use regex::Regex; // Подключаем библиотеку для работы с регулярными выражениями

fn main() -> std::io::Result<()> {
    // Скачиваем целевой HTML-документ
    let response = reqwest::blocking::get("https://provbiz.ru/")
        .expect("Не удалось выполнить запрос");
    
    // Получаем HTML-содержимое из ответа и выводим его
    let html_content = response.text().expect("Не удалось получить текст");
    // println!("{}", html_content);
    
    // Создаем регулярное выражение для поиска ссылок, начинающихся на https://
    let re = Regex::new(r"https://[^\s]+").expect("Не удалось создать регулярное выражение");
    
    // Ищем все ссылки в HTML-коде и сохраняем их в вектор
    let mut links: Vec<String> = Vec::new();
    for cap in re.captures_iter(&html_content) {
        if let Some(url) = cap.get(0) {
            links.push(url.as_str().to_string());
        }
    }
    
    // Выводим найденные ссылки на экран
    println!("Найденные ссылки:");
    for link in &links {
        println!("{}", link);
    }
    let links_content = links.join("\n");
    fs::write("links.txt", links_content)?;
    
    Ok(())
}

