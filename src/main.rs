use std::fs;
use std::io::Error;
use std::path::Path;
use std::process::Command;

fn main() {
    //新建一个文件夹
    mk_dir().unwrap();
    //设置输入输出路径
    let input_path = Path::new("./weishu.mp4").to_str().unwrap();
    let output_path = Path::new("./out/weishu.mp3").to_str().unwrap();
    //提取音频
    extract_audio(input_path, output_path).unwrap();
}

/// 提取音频函数
fn extract_audio(input_path: &str, output_path: &str) -> Result<(), Error> {
    Command::new("ffmpeg")
        .arg("-i")
        .arg(input_path)
        .arg("-vn")
        .arg("-acodec")
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
