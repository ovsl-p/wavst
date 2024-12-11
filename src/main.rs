use clap::Parser;
use std::{
    fs::File,
    io::{BufReader, Read},
};

#[derive(Parser)]
#[command(name = "wavst")]
#[command(version = "1.0")]
struct Cli {
    input_file: Option<String>,
    output_path: Option<String>,
    name: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    let input_file = match cli.input_file {
        Some(input_file) => input_file,
        None => {
            println!("input_fileを入力してください。");
            return;
        }
    };
    let output_path = match cli.output_path {
        Some(output_path) => output_path,
        None => {
            println!("output_pathを入力してください。");
            return;
        }
    };
    let name = match cli.name {
        Some(name) => name,
        None => "test".to_string(),
    };
    let buffer = BufReader::new(File::open(input_file).expect("変換対象のファイルが開けません。"));
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    const WAV_SUFFIX: &str = ".wav";
    let mut writer = hound::WavWriter::create(format!("{}{}{WAV_SUFFIX}", output_path, name), spec)
        .expect("ファイルの作成に失敗しました。");

    let mut bytes_index = 0;
    let mut bytes: Vec<u8> = vec![0, 0];
    for byte_or_error in buffer.bytes() {
        let byte = byte_or_error.unwrap();
        bytes[bytes_index] = byte;
        if bytes_index < 1 {
            bytes_index += 1;
            continue;
        }

        let sample: i16 = ((bytes[1] as i16) << 8) | (bytes[0] as i16);
        writer.write_sample(sample).unwrap();
        bytes = vec![0, 0];
        bytes_index = 0;
    }
    writer.finalize().unwrap();
}
