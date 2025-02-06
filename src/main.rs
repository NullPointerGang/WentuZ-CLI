/**
 * Copyright 2025 FlacSy
 *
 * Licensed under the FlacSy Open Use License (FOUL) 1.0
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License by contacting the author at
 *
 *     flacsy.tw@gmail.com
 *
 * This software is provided "as is", without any warranties, express or implied.
 * The author is not responsible for any damages or losses arising from the use of this software.
 * The License governs the permissions and restrictions related to the use, modification,
 * and distribution of this software.
 *
 * Commercial use is only permitted with prior written consent from the author.
 */


 use std::io::{self, Write};
 use std::sync::{Arc, Mutex};
 use std::time::Duration;
 use std::fs;
 
 use WentuZ_Backend::{Player, Track};
 
 
 fn main() {
     let player = Arc::new(Mutex::new(Player::new()));
 
     loop {
         println!("\n--- Music Player Console ---");
         println!("1. Play a track");
         println!("2. Pause");
         println!("3. Resume");
         println!("4. Stop");
         println!("5. Skip");
         println!("6. Previous");
         println!("7. Set volume");
         println!("8. Add track to queue");
         println!("9. Seek");
         println!("10. Show queue");
         println!("0. Exit");
         print!("Choose an option: ");
         io::stdout().flush().unwrap();
 
         let mut input = String::new();
         io::stdin().read_line(&mut input).unwrap();
         let choice = input.trim();
 
         match choice {
             "1" => {
                 let mut player = player.lock().unwrap();
                 player.auto_play();
             }
             "2" => {
                 let player = player.lock().unwrap();
                 player.pause();
                 println!("Playback paused.");
             }
             "3" => {
                 let player = player.lock().unwrap();
                 player.resume();
                 println!("Playback resumed.");
             }
             "4" => {
                 let player = player.lock().unwrap();
                 player.stop_auto_play();
                 println!("Playback stopped.");
             }
             "5" => {
                 let player = player.lock().unwrap();
                 player.play_next();
                 println!("Track skipped.");
             }
             "6" => {
                 let player = player.lock().unwrap();
                 player.play_previous();
                 println!("Previous track.");
             }
             "7" => {
                 println!("Enter volume (0.0 - 1.0): ");
                 io::stdout().flush().unwrap();
                 let mut volume_input = String::new();
                 io::stdin().read_line(&mut volume_input).unwrap();
                 if let Ok(volume) = volume_input.trim().parse::<f32>() {
                     let mut player = player.lock().unwrap();
                     player.set_volume(volume);
                     println!("Volume set to: {}", volume);
                 } else {
                     println!("Invalid volume value.");
                 }
             }
             "8" => {
                 println!("Enter the path to the track: ");
                 io::stdout().flush().unwrap();
                 let mut track_path = String::new();
                 io::stdin().read_line(&mut track_path).unwrap();
                 let track_path = track_path.trim();
                 if let Ok(track) = load_track(track_path) {
                     let player = player.lock().unwrap();
                     player.add_to_queue(track);
                     println!("Added track to queue: {}", track_path);
                 } else {
                     println!("Failed to load track: {}", track_path);
                 }
             }
             "9" => {
                 println!("Enter the seek time in seconds: ");
                 io::stdout().flush().unwrap();
                 let mut seek_input = String::new();
                 io::stdin().read_line(&mut seek_input).unwrap();
                 if let Ok(seek_time) = seek_input.trim().parse::<u64>() {
                     let player = player.lock().unwrap();
                     player.seek(Duration::from_secs(seek_time));
                     println!("Seeked to {} seconds.", seek_time);
                 } else {
                     println!("Invalid seek time.");
                 }
             }
             "10" => {
                 println!("Current queue:");
                 let player = player.lock().unwrap();
                 let queue = player.get_queue().lock().unwrap().clone();
                 for (index, track) in queue.get_tracks().iter().enumerate() {
                     println!("{}: {}", index + 1, track.get_file_path().as_deref().unwrap_or("Unknown"));
                 }
             }
             "0" => {
                 println!("Exiting...");
                 break;
             }
             _ => {
                 println!("Invalid option, please try again.");
             }
         }
     }
 }
 
 fn load_track(file_path: &str) -> Result<Track, String> {
     match fs::read(file_path) {
         Ok(track_data) => Ok(Track::new(
             file_path.to_string(),
             track_data,
             None,
             None,
             Some(Duration::new(240, 0)),
             Some(file_path.to_string()),
         )),
         Err(_) => Err("Failed to read the file.".to_string()),
     }
 }
 