use std::fs;
use std::io::Error;
use std::path::Path;
use std::process::Command;

fn main() {
    //新建一个文件夹
    if let Ok(_) = mk_dir() {
        println!("成功");
    } else {
        eprintln!("エラー発生.");
    }
    //设置输入输出路径
    let input_path = Path::new("./weishu.mp4").to_str().unwrap();
    let output_path = Path::new("./out/weishu.mp3").to_str().unwrap();
    //提取音频
    if let Ok(_) = extract_audio(input_path, output_path) {
        println!("音声を抽出成功.");
    } else {
        eprintln!("エラー発生.");
    }
}

/// 提取音频函数
fn extract_audio(input_path: &str, output_path: &str) -> Result<(), Error> {
    Command::new("ffmpeg")
        //オプションで入力ファイル（ビデオ）を指定します
        .arg("-i")
        .arg(input_path)
        //オプションでビデオを無視し、音声のみを抽出します
        .arg("-vn")
        //オプションで音声のコーデックを指定します
        .arg("-acodec")
        //libmp3lameを指定してMP3形式で保存します
        .arg("libmp3lame")
        .arg(output_path)
        .output()?;

    Ok(())
}
/// 创建文件夹函数
fn mk_dir() -> Result<(), Error> {
    let dir = "./out";
    if !Path::new(dir).exists() {
        fs::create_dir(dir)?;
    }
    Ok(())
}

use std::fs::File;
use std::io::Write;
use tikv_client::{Config, Key, RawClient, Value};

struct Segment {
    start: String,
    end: String,
    text: String,
}

fn reformat_time(time: &str) -> String {
    // Implement this function to reformat the time string.
    // This is a placeholder implementation.
    String::from(time)
}

fn write_srt(seg: &[Segment], srt_path: &Path) -> std::io::Result<()> {
    let mut file = File::create(srt_path)?;

    for (n, i) in seg.iter().enumerate() {
        let write_content = format!(
            "{}\n{} --> {}\n{}\n\n",
            n + 1,
            reformat_time(&i.start),
            reformat_time(&i.end),
            i.text
        );
        file.write_all(write_content.as_bytes())?;
    }

    Ok(())
}

fn transcribe_audio(
    audio_path: &Path,
    srt_path: &Path,
    language: &str,
    model_path: &str,
    device: &str,
) {
    // This function should implement the audio transcription.
    // As there's no direct equivalent in Rust for the Python's whisper library,
    // you might need to use an external service like Google Speech-to-Text API.
    // The transcribed text should be segmented and each segment should be converted into a Segment struct.
    // Then, you can call the write_srt function to write the segments into an SRT file,
    // and the save_to_tikv function to save the segments into a TiKV store.
}
