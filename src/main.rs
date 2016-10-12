extern crate rand;
extern crate byteorder;

use std::io::{ Write, Error };
use byteorder::{ LittleEndian, WriteBytesExt };
use std::fs::File;
use std::f64::consts::PI;


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

fn sine_wave<T: Write>(seconds: u32, handle: &mut T, freq: f64) -> Result<(), Error > {
    for x in 0..seconds * SAMPLE_RATE {
       let x = x as f64;
       try!(handle.write(&[ ((((((x * 2f64 * PI) / SAMPLE_RATE as f64) * freq).sin() + 1f64 )/ 2f64) * 255f64) as u8 ]));
    }
    Ok(())
}

fn main() {
    let duration = 1;

    let mut fp = File::create("out.wav").unwrap();

    write_header(duration * 9, &mut fp).unwrap();
    sine_wave(duration, &mut fp, 523.25_f64).unwrap();
    sine_wave(duration, &mut fp, 493.88_f64).unwrap();
    sine_wave(duration, &mut fp, 440_f64).unwrap();
    sine_wave(duration, &mut fp, 415.30_f64).unwrap();
    sine_wave(duration, &mut fp, 392_f64).unwrap();
    sine_wave(duration, &mut fp, 349.23_f64).unwrap();
    sine_wave(duration, &mut fp, 329.63_f64).unwrap();
    sine_wave(duration, &mut fp, 293.66_f64).unwrap();
    sine_wave(duration, &mut fp, 261.63_f64).unwrap();
}
