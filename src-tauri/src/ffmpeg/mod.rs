use ffmpeg_sidecar::command::FfmpegCommand;
use ffmpeg_sidecar::download::auto_download;

pub fn install_ffmpeg() {
    if let Err(e) = auto_download() {
        eprintln!("Failed to install FFmpeg: {}", e);
    }
}

pub fn ffmpeg_list_devices() {
    // List audio devices using ffmpeg
    let mut ffmpeg_cmd = FfmpegCommand::new()
        .args(&["-f", "dshow", "-list_devices", "true", "-i", "dummy"])
        .spawn()
        .unwrap();

    let output = ffmpeg_cmd.take_stdout().unwrap();
    Ok(output);
}
