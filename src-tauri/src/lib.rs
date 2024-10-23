mod ffmpeg;

use ffmpeg::install_ffmpeg;
use pv_recorder::PvRecorderBuilder;
use reqwest::Client;
use serde_json::Value;
use std::env;
use std::fs;
use std::path::Path;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// Main function to transcribe audio
#[tauri::command]
async fn transcribe_audio(
    file_path: String,
    output_folder: Option<String>,
    service: String,
) -> Result<String, String> {
    let api_key = get_api_key(&service)?;
    let file_content = read_audio_file(&file_path)?;
    let transcript = upload_audio(&file_content, &api_key, &service).await?;
    let output_path = save_transcript(&transcript, output_folder, &file_path)?;

    Ok(output_path)
}

// Function to get the API key based on the service
fn get_api_key(service: &str) -> Result<String, String> {
    match service {
        "assemblyai" => {
            env::var("ASSEMBLYAI_API_KEY").map_err(|_| "ASSEMBLYAI_API_KEY not set".to_string())
        }
        "openai" => env::var("OPENAI_API_KEY").map_err(|_| "OPENAI_API_KEY not set".to_string()),
        _ => Err("Invalid service specified".to_string()),
    }
}

// Function to read the audio file
fn read_audio_file(file_path: &str) -> Result<Vec<u8>, String> {
    fs::read(file_path).map_err(|e| e.to_string())
}

// Function to upload audio and get the transcript
async fn upload_audio(file_content: &[u8], api_key: &str, service: &str) -> Result<String, String> {
    let client = Client::new();
    let url = match service {
        "assemblyai" => "https://api.assemblyai.com/v2/upload",
        "openai" => "https://api.openai.com/v1/audio/transcriptions",
        _ => return Err("Invalid service specified".to_string()),
    };

    let response = client
        .post(url)
        .header("authorization", api_key)
        .header("content-type", "application/json")
        .body(file_content.to_vec())
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !response.status().is_success() {
        return Err(format!(
            "Failed to upload audio: {}",
            response.text().await.unwrap_or_default()
        ));
    }

    let transcript_response: Value = response.json().await.map_err(|e| e.to_string())?;
    transcript_response["text"]
        .as_str()
        .map(|s| s.to_string())
        .ok_or("Transcript not found".to_string())
}

// Function to save the transcript to a file
fn save_transcript(
    transcript: &str,
    output_folder: Option<String>,
    file_path: &str,
) -> Result<String, String> {
    let output_path = output_folder.unwrap_or_else(|| {
        Path::new(file_path)
            .parent()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
    });
    let output_file = format!("{}/transcript.txt", output_path);
    fs::write(&output_file, transcript).map_err(|e| e.to_string())?;

    Ok(output_file)
}

#[tauri::command]
fn show_audio_devices() -> Result<Vec<String>, String> {
    let audio_devices = PvRecorderBuilder::default()
        .get_available_devices()
        .map_err(|e| e.to_string())?;
    Ok(audio_devices)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Call the FFmpeg installation function on startup
    install_ffmpeg();

    tauri::Builder::default()
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            transcribe_audio,
            show_audio_devices
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
