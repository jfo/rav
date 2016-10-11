extern crate rand;
extern crate byteorder;

use std::io::{ stdout, Write };
use byteorder::{LittleEndian, WriteBytesExt};

const SAMPLE_RATE: u32 = 44100;
const CHANNELS: u32 = 1;
const HEADER_SIZE: u32 = 36;
const SUBCHUNK1_SIZE: u32 = 16;
const AUDIO_FORMAT: u32 = 1; // for PCM
const BIT_DEPTH: u32 = 8;
const BYTE_SIZE: u32 = 8;

#[allow(unused_must_use)]
fn write_header() {
    let stdout = stdout();
    let mut handle = stdout.lock();

    let numsamples = SAMPLE_RATE * 1;

    // ChunkId
    handle.write(b"RIFF");
    // ChunkSize = 36 + subchunk size 2
    handle.write_u32::<LittleEndian>(HEADER_SIZE + numsamples);
    // Format
    // Subchunk1ID
    handle.write(b"WAVEfmt ");
    // Subchunk1size
    handle.write_u32::<LittleEndian>(SUBCHUNK1_SIZE);
    // AudioFormat
    handle.write_u16::<LittleEndian>(AUDIO_FORMAT as u16);
    // Numchannels
    handle.write_u16::<LittleEndian>(CHANNELS as u16);
    // Samplerate
    handle.write_u32::<LittleEndian>(SAMPLE_RATE);
    // Byterate samplerate + num of channels * bits per sample /8
    handle.write_u32::<LittleEndian>(SAMPLE_RATE * CHANNELS * (BIT_DEPTH / BYTE_SIZE));
    // blockalign
    handle.write_u16::<LittleEndian>((CHANNELS * (BIT_DEPTH / BYTE_SIZE)) as u16);
    // bitspersample
    handle.write_u16::<LittleEndian>(BIT_DEPTH as u16);
    // subchunk2 id
    handle.write(b"data");
    // subchunk2size == numsamples * numchannels * bitspersample / 8
    handle.write_u32::<LittleEndian>(numsamples * CHANNELS * (BIT_DEPTH / BYTE_SIZE));
}

fn main() {
    write_header();
    // for x in 0..44100 {
    //     stdout().write(&[ rand::random::<u8>() ]);
    // }
}
