use tokio::process::Command;
use crate::error::Result;
use std::path::Path;

pub async fn transcode_to_hls(filepath: String) -> Result<()> {
    // 确保输入文件存在
    if !Path::new(&filepath).exists() {
        return Err(crate::error::AppError::Io(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Input file not found: {}", filepath)
        )));
    }

    let output_dir = filepath.trim_end_matches(".mp4");
    println!("Creating output directory: {}", output_dir);
    tokio::fs::create_dir_all(output_dir).await?;

    // 构建 ffmpeg 命令
    let args = [
        "-i", &filepath,
        "-profile:v", "baseline",
        "-level", "3.0",
        "-start_number", "0",
        "-hls_time", "10",
        "-hls_list_size", "0",
        "-f", "hls",
        "-hls_segment_type", "mpegts",
        "-hls_flags", "independent_segments",
        &format!("{}/index.m3u8", output_dir)
    ];

    println!("Executing ffmpeg command with args: {:?}", args);

    let cmd = Command::new("ffmpeg")
        .args(&args)
        .output()
        .await?;

    if !cmd.status.success() {
        let error_output = String::from_utf8_lossy(&cmd.stderr);
        println!("FFmpeg error output: {}", error_output);
        return Err(crate::error::AppError::Io(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Transcoding failed: {}", error_output)
        )));
    }

    // 验证输出文件
    let m3u8_path = format!("{}/index.m3u8", output_dir);
    if !Path::new(&m3u8_path).exists() {
        return Err(crate::error::AppError::Io(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Output file not created: {}", m3u8_path)
        )));
    }

    println!("Transcoding completed successfully");
    Ok(())
}