extern crate rand;
extern crate byteorder;

use std::io::{ stdout, Write, StdoutLock};
use byteorder::{LittleEndian, WriteBytesExt};

const SAMPLE_RATE: u32 = 44100;
const CHANNELS: u32 = 1;
const HEADER_SIZE: u32 = 36;
const SUBCHUNK1_SIZE: u32 = 16;
const AUDIO_FORMAT: u32 = 1;
const BIT_DEPTH: u32 = 8;
const BYTE_SIZE: u32 = 8;

#[allow(unused_must_use)]
fn write_header(seconds: u32, mut handle: StdoutLock) {

    let numsamples = SAMPLE_RATE * seconds;

    handle.write(b"RIFF");
    handle.write_u32::<LittleEndian>(HEADER_SIZE + numsamples);
    handle.write(b"WAVEfmt ");
    handle.write_u32::<LittleEndian>(SUBCHUNK1_SIZE);
    handle.write_u16::<LittleEndian>(AUDIO_FORMAT as u16);
    handle.write_u16::<LittleEndian>(CHANNELS as u16);
    handle.write_u32::<LittleEndian>(SAMPLE_RATE);
    handle.write_u32::<LittleEndian>(SAMPLE_RATE * CHANNELS * (BIT_DEPTH / BYTE_SIZE));
    handle.write_u16::<LittleEndian>((CHANNELS * (BIT_DEPTH / BYTE_SIZE)) as u16);
    handle.write_u16::<LittleEndian>(BIT_DEPTH as u16);
    handle.write(b"data");
    handle.write_u32::<LittleEndian>(numsamples * CHANNELS * (BIT_DEPTH / BYTE_SIZE));
}

fn main() {

    let duration = 1;

    let stdout = stdout();
    write_header(duration, stdout.lock());
    // for x in 0..duration * SAMPLE_RATE {
    //     stdout().write(&[ rand::random::<u8>() ]);
    // }

}
