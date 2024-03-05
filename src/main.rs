use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::process::Command;
fn main() {
    let input_path = Path::new("weishu.mp4");
    let output_path = Path::new("weishu.mp3");
    audio_extract(input_path, output_path).unwrap();
}

fn audio_extract(input_path: &Path, output_path: &Path) -> std::io::Result<()> {
    let mut file = File::open(input_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    Command::new("ffmpeg")
        .arg("-i")
        .arg("pipe:0")
        .arg("-vn")
        .arg(output_path.to_str().unwrap())
        .output()
        .expect("Failed to execute command");

    Ok(())
}
