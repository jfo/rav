extern crate rand;
extern crate byteorder;

use std::io::{ stdout, Write };
use byteorder::{LittleEndian, WriteBytesExt};

const SAMPLE_RATE: u32 = 44100;

#[allow(unused_must_use)]
fn write_header() {
    let stdout = stdout();
    let mut handle = stdout.lock();

    let numsamples = SAMPLE_RATE * 1;

    // ChunkId
    handle.write(b"RIFF");
    // ChunkSize = 36 + subchunk size 2
    handle.write_u32::<LittleEndian>(36 + numsamples);
    // Format
    // Subchunk1ID
    handle.write(b"WAVEfmt ");
    // Subchunk1size
    handle.write_u32::<LittleEndian>(16);
    // AudioFormat
    handle.write_u16::<LittleEndian>(1);
    // Numchannels
    handle.write_u16::<LittleEndian>(1);
    // Samplerate
    handle.write_u32::<LittleEndian>(SAMPLE_RATE);
    // Byterate samplerate + num of channels * bits per sample /8
    handle.write_u32::<LittleEndian>(SAMPLE_RATE * 1 * (8 / 8));
    // blockalign
    handle.write_u16::<LittleEndian>(1);
    // bitspersample
    handle.write_u16::<LittleEndian>(8);
    // subchunk2 id
    handle.write(b"data");
    // subchunk2size == numsamples * numchannels * bitspersample / 8
    handle.write_u32::<LittleEndian>(numsamples * 1 * (8 / 8));
}

fn main() {
    write_header();
    // for x in 0..44100 {
    //     stdout().write(&[ rand::random::<u8>() ]);
    // }
}
