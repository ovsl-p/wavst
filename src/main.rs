use std::{
    fs::File,
    io::{BufReader, Read},
};

fn main() {
    let buffer = BufReader::new(File::open("./path").unwrap());
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = hound::WavWriter::create("test.wav", spec).unwrap();

    let mut array_count = 0;
    let mut bytes: Vec<u8> = vec![];
    for byte_or_error in buffer.bytes() {
        let byte = byte_or_error.unwrap();
        bytes.push(byte);
        if array_count != 3 {
            array_count += 1;
            continue;
        }

        let byte_array: [u8; 4] = bytes[0..4].try_into().expect("Needed 4 bytes for a float");
        let sample: f32 = f32::from_be_bytes(byte_array);
        let amplitude = i16::MAX as f32;
        writer.write_sample((sample * amplitude) as i16).unwrap();
        bytes = vec![];
        array_count = 0;
    }
    writer.finalize().unwrap();
}
