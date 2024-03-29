// CLI news headlines display

use std::io;
use std::error::Error;
use dotenv::dotenv;
use colour::{yellow, blue};
use newsapi::{Articles, get_articles};

// ------------------------------------------------------------------------------------------------------------------------------------------------------

fn readline_string_clean() -> String {
    let mut s: String = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read line");
    if let Some('\n')= s.chars().next_back() {
        s.pop();
    }
    if let Some('\r')= s.chars().next_back() {
        s.pop();
    }
    return s;
}   

fn render_articles(articles: &Articles) {
    for a in &articles.articles {
        yellow!("> {}\n", a.title);
        blue!("> {}\n\n", a.url);
    }
}

fn top_headlines() -> String {
    println!("Enter 2-letter ISO 3166-1 country code:  ");
    let country_code: String = readline_string_clean();
    let url: String = format!("https://newsapi.org/v2/top-headlines?country={}&apiKey=", country_code);
    return url;
}

fn search_headlines() -> String {
    println!("Enter 2-letter ISO-639-1 language code:  ");
    let lang_code: String = readline_string_clean();

    println!("Enter search term:  ");
    let search_term: String = readline_string_clean();

    println!("Enter number of results:  ");
    let num_results: usize = readline_string_clean().parse().unwrap();

    let url: String = format!("https://newsapi.org/v2/everything?q={}&searchin=title,description&language={}&pageSize={}&apiKey=", search_term, lang_code, num_results);
    return url;
}

// ------------------------------------------------------------------------------------------------------------------------------------------------------

fn main() -> Result<(), Box<dyn Error>>{
    dotenv()?;
    let api_key: String = std::env::var("API_KEY")?;
    
    println!("Enter corresponding number to choose:\n\n1) View top headlines\n2) Search for results");
    let url: String = loop {
        let choice: String = readline_string_clean();
        let choice: u8 = match choice.trim().parse() {
            Ok(num) => {num}
            Err(_) => {
                println!("Enter a valid number");
                continue;
            }
        };

        if choice > 2 {
            println!("Enter a valid number");
            continue;
        } else {
            let url: String = {
                if choice == 1 {
                    top_headlines()
                } else {
                    search_headlines()
                }
            };
            break url;
        }
    };
        
    let url: String = format!("{}{}", url, api_key);
    let articles: Articles = get_articles(&url)?;
    render_articles(&articles);
    Ok(())
}
