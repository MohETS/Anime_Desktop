use std::io::{BufRead, BufReader};
use std::process::{ChildStderr, Command, Stdio};

pub struct FFMPEGInstance {
    anime_title: String,
    anime_episode: String,
    folder_location: String,
    m3u8_url: String,
    instance: Command,
    buf_reader: Option<BufReader<ChildStderr>>,
    duration: f32,
    progress: f32,
}

impl FFMPEGInstance {
    pub fn new(anime_title: String, anime_episode: String, folder_location: String, m3u8_url: String, duration: f32) -> Self {
        FFMPEGInstance {
            anime_title,
            anime_episode,
            folder_location,
            m3u8_url,
            instance: Command::new("ffmpeg"),
            buf_reader: None,
            duration,
            progress: 0.0,
        }
    }

    pub fn execute_instance(&mut self) {
        let ffmpeg = self.instance
            .arg("-hide_banner")
            .arg("-y")
            .arg("-i")
            .arg(&self.m3u8_url)
            .arg("-c")
            .arg("copy")
            .arg(self.folder_location.to_string() + &*self.anime_title + "_" + &*self.anime_episode + ".mkv")
            .stderr(Stdio::piped())
            .spawn()
            .expect("Error : FFMPEG command failed to start");

        self.buf_reader = Option::from(BufReader::new(ffmpeg.stderr.unwrap()));
    }

    pub fn download_over(&mut self) -> bool {
        let mut line = String::new();

        self.buf_reader.as_mut().unwrap().read_line(&mut line).expect("Read line failed.");
        if line == "" {
            self.progress = 100.0;
            println!("FINISHED {} Episode {} - Progress: {:.2}", self.anime_title, self.anime_episode, self.progress);
            return true;
        }

        if line.contains("time=") {
            let video_time: Vec<&str> = line.split('=').collect::<Vec<&str>>()[5][0..8].split(':').collect::<Vec<&str>>();
            let time = video_time[2].parse::<f32>().unwrap() + video_time[1].parse::<f32>().unwrap() * 60f32 + (video_time[0].parse::<f32>().unwrap() * 60f32 * 60f32);
            self.progress = (time / self.duration) * 100f32;
            println!("{} Episode {} - Progress: {:.2}", self.anime_title, self.anime_episode, self.progress);
        }
        false
    }
}