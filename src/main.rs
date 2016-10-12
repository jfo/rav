extern crate rand;
extern crate byteorder;

use std::io::{ Write, Error };
use byteorder::{ LittleEndian, WriteBytesExt };
use std::fs::File;


const SAMPLE_RATE: u32 = 44100;
const CHANNELS: u32 = 1;
const HEADER_SIZE: u32 = 36;
const SUBCHUNK1_SIZE: u32 = 16;
const AUDIO_FORMAT: u32 = 1;
const BIT_DEPTH: u32 = 8;
const BYTE_SIZE: u32 = 8;

fn write_header<T: Write>(seconds: u32, handle: &mut T) -> Result<(), Error> {

    let numsamples = SAMPLE_RATE * seconds;

    try!(handle.write(b"RIFF"));
    try!(handle.write_u32::<LittleEndian>(HEADER_SIZE + numsamples));

    try!(handle.write(b"WAVEfmt "));
    try!(handle.write_u32::<LittleEndian>(SUBCHUNK1_SIZE));
    try!(handle.write_u16::<LittleEndian>(AUDIO_FORMAT as u16));
    try!(handle.write_u16::<LittleEndian>(CHANNELS as u16));

    try!(handle.write_u32::<LittleEndian>(SAMPLE_RATE));
    try!(handle.write_u32::<LittleEndian>(SAMPLE_RATE * CHANNELS * (BIT_DEPTH / BYTE_SIZE)));
    try!(handle.write_u16::<LittleEndian>((CHANNELS * (BIT_DEPTH / BYTE_SIZE)) as u16));
    try!(handle.write_u16::<LittleEndian>(BIT_DEPTH as u16));

    try!(handle.write(b"data"));
    try!(handle.write_u32::<LittleEndian>(numsamples * CHANNELS * (BIT_DEPTH / BYTE_SIZE)));

    Ok(())
}

fn make_some_noise<T: Write>(seconds: u32, handle: &mut T) -> Result<(), Error > {

    for _ in 0..seconds * SAMPLE_RATE {
       try!(handle.write(&[ rand::random::<u8>() ]));
    }

    Ok(())
}

fn make_a_random_ass_sawtooth<T: Write>(seconds: u32, handle: &mut T) -> Result<(), Error > {

    for x in 0..seconds * SAMPLE_RATE {
       try!(handle.write(&[ ((x + 1) % 255) as u8 ]));
    }

    Ok(())
}

fn main() {
    let duration = 1;

    let mut fp = File::create("out.wav").unwrap();

    write_header(duration * 4, &mut fp).unwrap();

    make_a_random_ass_sawtooth(duration, &mut fp).unwrap();
    make_some_noise(duration, &mut fp).unwrap();
    make_a_random_ass_sawtooth(duration, &mut fp).unwrap();
    make_some_noise(duration, &mut fp).unwrap();
}
