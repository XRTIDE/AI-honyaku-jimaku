use std::process::Command;

fn main() {
    let output = Command::new("ffmpeg")
        .arg("-i")
        .arg("input.mp3")
        .arg("output.wav")
        .output()
        .expect("Failed to execute command");

    if !output.status.success() {
        eprintln!("未成功: {:?}", output.stderr);
    }
}
