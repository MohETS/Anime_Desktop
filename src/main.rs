#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_doc_comments)]
#![allow(unused_attributes)]
#[macro_use]
extern crate text_io;
mod models;

mod api_jsons;
mod api_caller;
mod user;

use crate::api_caller::{AnimepaheApi, DdosGuardApi, KwikApi};


use crate::models::Anime;
use crate::user::User;
use reqwest::Url;
use std::io;
use std::process::Command;

#[tokio::main]
async fn main() {
    println!("Hello, sad world!");
    let client = reqwest::Client::new();
    let user: User = User::new();
    let ddos_guard: DdosGuardApi = DdosGuardApi::new(user.client().clone());
    let animepahe: AnimepaheApi = AnimepaheApi::new(user.client().clone());
    let mut kwik: KwikApi = KwikApi::new(user.client().clone());


    let mut searched_anime: String = String::new();
    let mut search_response: String;

    //send the request for the ddg2 cookie
    let dd2g_cookie = ddos_guard.get_ddg2_cookie().await;
    user.add_cookie(&*dd2g_cookie, &"https://animepahe.ru".parse::<Url>().unwrap());


    /**Request for the wanted anime**/
    //Take the user input for the searched anime
    println!("Search the anime you want to see?");
    io::stdin().read_line(&mut searched_anime).expect("Read line failed.");
    searched_anime = searched_anime.to_ascii_lowercase().trim_end().to_string();
    println!("Searched Anime: {:?}", searched_anime);

    //Send the get request to the api to get the searched anime
    let anime_results = animepahe.get_searched_animes(&searched_anime).await;
    let mut cmp: i8 = 1;
    for anime in &anime_results {
        println!("{}: {:?}", cmp, anime.title());
        cmp += 1;
    }

    //Take the user input for the searched anime
    println!("What anime do you want to watch?");
    let mut user_choice = read_user_input(&anime_results).await;
    let chosen_anime = &anime_results[user_choice - 1];

    //Send the get request for the anime information (Episodes etc..)
    let anime_episodes_results = animepahe.get_searched_anime_episodes(chosen_anime.anime_id()).await;


    /*Request for the wanted episode of the anime*/
    //Ask user to pick the episode of the anime
    cmp = 1;
    for episode in &anime_episodes_results {
        println!("{}: {} - {}", cmp, chosen_anime.title(), episode.episode());
        cmp += 1;
    }
    println!("What episode do you want to watch?");

    // Loop to make sure the user enters a valid input
    user_choice = read_user_input(&anime_results).await;
    let chosen_episode = &anime_episodes_results[user_choice - 1];

    /*Request the wanted episode's page*/
    //Get the kwik.si link where the m3u8 file information are stored
    let kwik_url = animepahe.get_kwik_episode_url(chosen_anime, chosen_episode).await;

    //Get the m3u8 file
    let m3u8_url = kwik.get_m3u8_url(&*kwik_url).await;
    println!("{}", &m3u8_url);

    let mut folder_location: String = String::new();
    println!("Enter the location where you would like to save the file");
    io::stdin().read_line(&mut folder_location).expect("Error: Read line failed.");
    folder_location = folder_location.trim_end().to_string();
    println!("{}\\{}_{}", folder_location, chosen_anime.title().replace(" ", "_"), chosen_episode.episode().to_string());

    // Creates a Command that run ffmpeg to download the episode or episodes selected
    let output = Command::new("ffmpeg")
        .arg("-i")
        .arg(&m3u8_url)
        .arg("-c")
        .arg("copy")
        .arg(folder_location + "\\" + &*chosen_anime.title().replace(" ", "_") + "_" + &*chosen_episode.episode().to_string() + ".mkv")
        .spawn()
        .expect("Error : FFMPEG command failed to start");

    println!("{:?}", output.wait_with_output());
}

async fn read_user_input(anime_results: &Vec<Anime>) -> usize {
    // Loop to make sure the user enters a valid input
    let mut index = String::new();
    loop {
        io::stdin().read_line(&mut index).expect("Error: Read line failed.");
        index = index.trim_end().to_string();
        println!("{:?}", index);
        if index.parse::<usize>().is_err() {
            println!("Please enter a valid number from the choice above");
            index.clear();
            let mut cmp: i8 = 1;
            for anime in anime_results {
                println!("{}: {:?}", cmp, anime.title());
                cmp += 1;
            }
            continue;
        }
        return index.parse::<usize>().unwrap();
    }
}

