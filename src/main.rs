#![allow(dead_code)]
// #![allow(unused_variables)]
// #![allow(unused_imports)]
// #![allow(unused_doc_comments)]
#![allow(unused_attributes)]
extern crate core;
extern crate text_io;

mod models;
mod api_jsons;
mod api_caller;
mod user;

use crate::api_caller::{AnimepaheApi, DdosGuardApi, KwikApi};
use crate::models::{AnimeEpisode, FFMPEGInstance};
use crate::user::User;

use reqwest::Url;
use std::io;
use regex::{Regex};

#[tokio::main]
async fn main() {
    let user: User = User::new();
    let ddos_guard: DdosGuardApi = DdosGuardApi::new(user.client().clone());
    let animepahe: AnimepaheApi = AnimepaheApi::new(user.client().clone());
    let mut kwik: KwikApi = KwikApi::new(user.client().clone());


    //send the request for the ddg2 cookie
    let dd2g_cookie = ddos_guard.get_ddg2_cookie().await;
    user.add_cookie(&*dd2g_cookie, &"https://animepahe.ru".parse::<Url>().unwrap());


    let mut searched_anime: String = String::new();

    /**Request for the wanted anime**/

    //Take the user input for the searched anime
    println!("Search the anime you want to see?");
    io::stdin().read_line(&mut searched_anime).expect("Read line failed.");
    searched_anime = searched_anime.to_ascii_lowercase().trim_end().to_string();
    println!("Searched Anime: {}", searched_anime);

    //Send the get request to the api to get the searched anime
    let anime_results = animepahe.get_searched_anime_results(&searched_anime).await;
    let mut cmp: i8 = 1;
    for anime in &anime_results {
        println!("{}: {}", cmp, anime.title());
        cmp += 1;
    }

    //Take the user input for the searched anime
    println!("What anime do you want to watch?");
    let mut user_choice = read_user_input(anime_results.len());
    let chosen_anime = &anime_results[user_choice - 1];

    //Send the get request for the anime information (Episodes etc..)
    let anime_episodes_results = animepahe.get_searched_anime_episodes(chosen_anime.anime_id()).await;


    /*Request for the wanted episode of the anime*/
    //Ask user to pick the episode of the anime
    println!("What option do you want ?");
    println!("1. Select a specific episode");
    println!("2. Select a specific set of episodes");
    //TODO: println!(" 3. Download all the season episode (Only for seasonal animes. HxH, One piece are not valid anime))");
    user_choice = read_user_input(3);

    //Show every episodes available
    let mut cmp = 1;
    for episode in &anime_episodes_results {
        println!("{}: {} - {}", cmp, chosen_anime.title(), episode.episode());
        cmp += 1;
    }


    let episode_list: Vec<&AnimeEpisode>;
    match user_choice {
        1 => episode_list = single_episode(&anime_episodes_results),
        2 => { episode_list = multiple_episode(&anime_episodes_results)}
        3 => { todo!() }
        _ => { todo!() }
    };

    // Loop to make sure the user enters a valid input
    let mut folder_location: String = String::new();
    println!("Enter the location where you would like to save the file");
    io::stdin().read_line(&mut folder_location).expect("Error: Read line failed.");
    folder_location = folder_location.trim_end().to_string() + "\\";

    let mut ffmpeg_instances: Vec<FFMPEGInstance> = Vec::new();
    for anime_episode in episode_list {
        /*Request the wanted episode's page*/
        //Get the kwik.si link where the m3u8 file information are stored
        let kwik_url = animepahe.get_kwik_episode_url(chosen_anime, anime_episode).await;

        //Get the m3u8 file
        let m3u8_url = kwik.get_m3u8_url(&*kwik_url).await;

        let video_duration: Vec<String> = anime_episode.duration().split(":").map(|s| s.to_string()).collect();
        let duration: f32 = (video_duration[2].parse::<f32>().unwrap() - 1.0) + video_duration[1].parse::<f32>().unwrap() * 60f32 + (video_duration[0].parse::<f32>().unwrap() * 60f32 * 60f32);

        let mut instance = FFMPEGInstance::new(chosen_anime.title().to_string(), anime_episode.episode().to_string(), folder_location.clone(), m3u8_url, duration);
        instance.execute_instance();
        ffmpeg_instances.push(instance);
    }

    let number_downloads = ffmpeg_instances.len();
    let mut finished_downloads: usize = 0;
    let mut i;
    loop {
        i = 0;
        while i < ffmpeg_instances.len() {
            if ffmpeg_instances[i].download_over() {
                finished_downloads += 1;
                ffmpeg_instances.remove(i);
            }
            i += 1;
        }
        if finished_downloads == number_downloads {
            break;
        }
    }

}

fn read_user_input(max_option: usize) -> usize {
    // Loop to make sure the user enters a valid input
    let mut index = String::new();
    let number_reg:Regex = Regex::new(r"^\d*$").unwrap();
    loop {
        io::stdin().read_line(&mut index).expect("Error: Read line failed.");
        index = index.trim_end().to_string();

        if !Regex::is_match(&number_reg, &*index) || index.parse::<usize>().unwrap() > max_option{
            println!("Please enter a valid number from the choice above");
            index.clear();
            continue;
        }

        return index.parse::<usize>().unwrap();
    }
}

fn single_episode(anime_episodes_results: &Vec<AnimeEpisode>) -> Vec<&AnimeEpisode> {
    println!("What episode do you want to watch?");
    let user_choice = read_user_input(anime_episodes_results.len());

    let chosen_episode = &anime_episodes_results[user_choice - 1];
    let mut episodes: Vec<&AnimeEpisode> = Vec::new();
    episodes.push(chosen_episode);
    episodes
}

fn multiple_episode(anime_episodes_results: &Vec<AnimeEpisode>) -> Vec<&AnimeEpisode> {
    println!("What episodes do you want to watch? (Two input possible)");
    println!("- x..y will select all the episode from x to y");
    println!("- x-y-z will select every episode seperated by -");
    let mut episodes: Vec<&AnimeEpisode> = Vec::new();
    let mut user_choice: String = String::new();
    io::stdin().read_line(&mut user_choice).expect("Error: Read line failed.");
    user_choice = user_choice.trim_end().to_string();
    if user_choice.contains("..") {
        let indexes = user_choice.split("..").collect::<Vec<&str>>();
        let start_index = indexes[0].parse::<usize>().expect("Error: Invalid index number.");
        let end_index = indexes[1].parse::<usize>().expect("Error: Invalid index number.");
        for i in start_index..end_index + 1
        {
            // println!("{:?}", &anime_episodes_results[i - 1]);
            episodes.push(&anime_episodes_results[i - 1]);
        }
    }

    if user_choice.contains("-") {
        let indexes = user_choice.split("-").collect::<Vec<&str>>();
        for i in 0..indexes.len() {
            // println!("{:?}", &anime_episodes_results[indexes[i].parse::<usize>().unwrap() - 1]);
            episodes.push(&anime_episodes_results[indexes[i].parse::<usize>().unwrap() - 1]);
        }
    }

    episodes
}