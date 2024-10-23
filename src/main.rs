#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_doc_comments)]
#![allow(unused_attributes)]
#[macro_use]
extern crate text_io;
mod anime;
mod anime_json;
mod anime_episode_json;
mod anime_episode;

use std::io;
use std::num::ParseIntError;
use std::process::Command;
use reqwest::header::{COOKIE, REFERER, USER_AGENT};
use scraper::{Html, Selector};
use tokio::io::stdin;
use crate::anime_episode_json::AnimeEpisodeJson;
use crate::anime_json::AnimeJson;

#[tokio::main]
async fn main() {
    println!("Hello, sad world!");
    let client = reqwest::Client::new();
    let mut searched_anime: String = String::new();
    let mut search_response: String;


    //send the request for the ddg2 cookie
    let ddg2_cookie_value = client.get("https://check.ddos-guard.net/check.js").send().await.unwrap().cookies().next().unwrap().value().to_string();
    let dd2g_cookie = "__ddg2_=".to_owned() + &*ddg2_cookie_value + "; Path=/; Domain=animepahe.ru; HttpOnly;";


    /**Request for the wanted anime**/

    //Take the user input for the searched anime
    println!("Search the anime you want to see?");
    io::stdin().read_line(&mut searched_anime).expect("Read line failed.");
    searched_anime = searched_anime.to_ascii_lowercase().trim_end().to_string();
    println!("Searched Anime: {:?}", searched_anime);

    //Send the get request to the api to get the searched anime
    search_response = client.get("https://animepahe.ru/api?m=search".to_owned()+"&q="+ &*urlencoding::encode(&searched_anime) +"")
        .header(COOKIE, &dd2g_cookie)
        .send().await.unwrap().text().await.unwrap();
    let search_json: AnimeJson = serde_json::from_str(&*search_response).unwrap();
    let searched_anime_list_results = search_json.data();
    let mut cmp: i8 = 1;
    for anime in searched_anime_list_results {
        println!("{}: {:?}", cmp, anime.title());
        cmp += 1;
    }

    //Take the user input for the searched anime
    println!("What anime do you want to watch?");
    let mut index = String::new();

    // Loop to make sure the user enters a valid input
    loop {
        io::stdin().read_line(&mut index).expect("Error: Read line failed.");
        index = index.trim_end().to_string();
        println!("{:?}", index);
        if index.parse::<usize>().is_err(){
            println!("Please enter a valid number from the choice above");
            index.clear();
            let mut cmp: i8 = 1;
            for anime in searched_anime_list_results {
                println!("{}: {:?}", cmp, anime.title());
                cmp += 1;
            }
            continue
        }
        break;
    }

    let index_choice:usize = index.parse::<usize>().unwrap();
    let chosen_anime = &searched_anime_list_results[index_choice - 1];


    //Send the get request for the anime information (Episodes etc..)
    let id = chosen_anime.anime_id();
    search_response = client.get("https://animepahe.ru/api?m=release".to_owned()+"&id="+ id +"&sort=episode_asc&page=1")
        .header(COOKIE, &dd2g_cookie)
        .send().await.unwrap().text().await.unwrap();
    let search_json: AnimeEpisodeJson    = serde_json::from_str(&*search_response).unwrap();
    let anime_episodes = search_json.data();


    /*Request for the wanted episode of the anime*/

    //Ask user to pick the episode of the anime
    cmp = 1;
    for episode in anime_episodes {
        println!("{}: {} - {}", cmp, chosen_anime.title(), episode.episode());
        cmp += 1;
    }
    println!("What episode do you want to watch?");
    index.clear();

    // Loop to make sure the user enters a valid input
    loop {
        io::stdin().read_line(&mut index).expect("Error: Read line failed.");
        index = index.trim_end().to_string();
        println!("{:?}", index);
        if index.parse::<usize>().is_err(){
            println!("Please enter a valid number from the choice above");
            index.clear();
            cmp = 1;
            for episode in anime_episodes {
                println!("{}: {} - {}", cmp, chosen_anime.title(), episode.episode());
                cmp += 1;
            }
            continue
        }
        break;
    }

    let index_choice:usize = index.parse::<usize>().unwrap();
    let chosen_episode = &anime_episodes[index_choice - 1];

    /*Request the wanted episode's page*/

    //Get the kwik.si link where the m3u8 file information are stored
    let id = chosen_anime.anime_id();
    search_response = client.get("https://animepahe.ru/play/".to_owned()+chosen_anime.anime_id()+"/"+chosen_episode.session())
        .header(COOKIE, &dd2g_cookie)
        .send().await.unwrap().text().await.unwrap();

    //Gets the kwik.si/e link to get the m3u8
    let fragment = Html::parse_fragment(&*search_response);
    let selector = Selector::parse("button").unwrap();
    let mut url_kwik = "";
    for element in fragment.select(&selector) {
        if element.text().collect::<Vec<_>>().get(0).unwrap().contains(&"· 1080p") {
            url_kwik = element.attr("data-src").unwrap();
            break;
        }
    }


    //Get the m3u8 file
    search_response = client.get(url_kwik)
        .header(USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/129.0.0.0 Safari/537.36")
        .header(REFERER, "https://animepahe.ru/")
        .send().await.unwrap().text().await.unwrap();

    //Get values for the m3u8 url file
    let mut m3u8_url_info_parts: Vec<&str> = Vec::new();

    //Gets the eu-012.files etc
    let fragment = Html::parse_fragment(&*search_response);
    let mut selector = Selector::parse("link").unwrap();
    for element in fragment.select(&selector) {
        if element.attr("rel").unwrap().eq("preconnect") && element.attr("href").unwrap().contains("nextcdn.org") {
            m3u8_url_info_parts.push(element.attr("href").unwrap());
            m3u8_url_info_parts.push(&element.attr("href").unwrap()[5..7]);
            break;
        }
    }

    //Gets the id
    let fragment = Html::parse_fragment(&*search_response);
    selector = Selector::parse("script").unwrap();
    let mut m3u8_url_info = "";
    for element in fragment.select(&selector) {
        if !element.text().collect::<Vec<_>>().is_empty() {
            m3u8_url_info = element.text().collect::<Vec<_>>().get(0).unwrap();
            break;
        }
    }

    let value_position_checker = "/1C/1B/1A.1z";
    m3u8_url_info = &m3u8_url_info[1783..m3u8_url_info.len()];
    let m3u8_url_info_length: usize = m3u8_url_info.len();

    /** Checks if the kwik contains the String value_position_checker
    * If it's true the value is at the end with the episode's code
    * If not that means the value is at the start
    */
    if m3u8_url_info.contains(value_position_checker) {
        let string = &m3u8_url_info[(m3u8_url_info_length - 121)..(m3u8_url_info_length - 54)];
        let temp = &mut string.split("|").collect::<Vec<&str>>();
        m3u8_url_info_parts.push(temp[1]);
        m3u8_url_info_parts.push(temp[0]);
    } else {
        m3u8_url_info_parts.push(&m3u8_url_info[32..34]);
        m3u8_url_info_parts.push(&m3u8_url_info[(m3u8_url_info_length - 118)..(m3u8_url_info_length - 54)]);
    }

    //Combines all the m3u8 information to make the m3u8 url
    let m3u8_url = "https:".to_owned() + m3u8_url_info_parts[0] + "/stream/"+m3u8_url_info_parts[1]+"/"
        +m3u8_url_info_parts[2]+"/"+m3u8_url_info_parts[3]+"/uwu.m3u8";
    println!("{}", &m3u8_url);

    let mut folder_location:String = String::new();
    println!("Enter the location where you would like to save the file");
    io::stdin().read_line(&mut folder_location).expect("Error: Read line failed.");
    folder_location = folder_location.trim_end().to_string();
    println!("{}\\{}_{}", folder_location,chosen_anime.title().replace(" ", "_"),chosen_episode.episode().to_string());

    // Creates a Command that run ffmpeg to download the episode or episodes selected
    let output = Command::new("ffmpeg")
        .arg("-i")
        .arg(&m3u8_url)
        .arg("-c")
        .arg("copy")
        .arg(folder_location+"\\"+ &*chosen_anime.title().replace(" ", "_")+"_"+ &*chosen_episode.episode().to_string()+".mkv")
        .spawn()
        .expect("Error : FFMPEG command failed to start");

    println!("{:?}", output.wait_with_output());
}

