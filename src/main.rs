mod onnsei2tekisuto;

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
    let input_path = Path::new("./resource/weishu.mp4").to_str().unwrap();
    let output_path = Path::new("./out/weishu.wav").to_str().unwrap();
    //提取音频
    if let Ok(_) = extract_audio(input_path, output_path) {
        println!("音声を抽出成功.");
    } else {
        eprintln!("エラー発生.");
    }
    //オディオファイルをテキストに変換
    if let Ok(_) = crate::onnsei2tekisuto::onnsei2tekisuto() {
        println!("成功");
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
        //mp3を指定してMP3形式で保存します
        // .arg("libmp3lame")
        //WAVを指定してMP3形式で保存します
        .arg("pcm_s16le")
        //オプションで音声のサンプリングレートを指定します
        .arg("-ar")
        .arg("16000")
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
