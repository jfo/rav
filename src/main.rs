extern crate byteorder;
extern crate rand;

use byteorder::LittleEndian;
use byteorder::WriteBytesExt;
use std::f64::consts::PI;
use std::fs::File;
use std::io::Error;
use std::io::Write;

const SAMPLE_RATE: u32 = 44100;
const CHANNELS: u32 = 1;
const HEADER_SIZE: u32 = 36;
const SUBCHUNK1_SIZE: u32 = 16;
const AUDIO_FORMAT: u32 = 1;
const BIT_DEPTH: u32 = 8;
const BYTE_SIZE: u32 = 8;

fn write_header<T: Write>(seconds: u32, handle: &mut T) -> Result<(), Error> {
    let numsamples = SAMPLE_RATE * seconds;

    handle.write(b"RIFF")?;
    handle.write_u32::<LittleEndian>(HEADER_SIZE + numsamples)?;

    handle.write(b"WAVEfmt ")?;
    handle.write_u32::<LittleEndian>(SUBCHUNK1_SIZE)?;
    handle.write_u16::<LittleEndian>(AUDIO_FORMAT as u16)?;
    handle.write_u16::<LittleEndian>(CHANNELS as u16)?;

    handle.write_u32::<LittleEndian>(SAMPLE_RATE)?;
    handle.write_u32::<LittleEndian>(SAMPLE_RATE * CHANNELS * (BIT_DEPTH / BYTE_SIZE))?;
    handle.write_u16::<LittleEndian>((CHANNELS * (BIT_DEPTH / BYTE_SIZE)) as u16)?;
    handle.write_u16::<LittleEndian>(BIT_DEPTH as u16)?;

    handle.write(b"data")?;
    handle.write_u32::<LittleEndian>(numsamples * CHANNELS * (BIT_DEPTH / BYTE_SIZE))?;

    Ok(())
}

fn sine_wave<T: Write>(seconds: u32, handle: &mut T, freq: f64) -> Result<(), Error> {
    for x in 0..seconds * SAMPLE_RATE {
        let x = x as f64;
        handle.write(&[
            ((((((x * 2f64 * PI) / SAMPLE_RATE as f64) * freq).sin() + 1f64) / 2f64) * 255f64)
                as u8
        ])?;
    }
    Ok(())
}

fn main() {
    let duration = 1;

    let mut fp = File::create("out.wav").unwrap();

    write_header(duration * 31, &mut fp).unwrap();
    sine_wave(duration, &mut fp, 196.00_f64).unwrap();
    sine_wave(duration, &mut fp, 220.00_f64).unwrap();
    sine_wave(duration, &mut fp, 261.63_f64).unwrap();
    sine_wave(duration, &mut fp, 220.00_f64).unwrap();

    sine_wave(duration, &mut fp, 329.63_f64).unwrap();
    sine_wave(duration, &mut fp, 329.63_f64).unwrap();
    sine_wave(duration * 2, &mut fp, 293.66_f64).unwrap();

    sine_wave(duration, &mut fp, 196.00_f64).unwrap();
    sine_wave(duration, &mut fp, 220.00_f64).unwrap();
    sine_wave(duration, &mut fp, 261.63_f64).unwrap();
    sine_wave(duration, &mut fp, 220.00_f64).unwrap();

    sine_wave(duration, &mut fp, 293.66_f64).unwrap();
    sine_wave(duration, &mut fp, 293.66_f64).unwrap();
    sine_wave(duration * 2, &mut fp, 261.63_f64).unwrap();

    sine_wave(duration, &mut fp, 196.00_f64).unwrap();
    sine_wave(duration, &mut fp, 220.00_f64).unwrap();
    sine_wave(duration, &mut fp, 261.63_f64).unwrap();
    sine_wave(duration, &mut fp, 220.00_f64).unwrap();

    sine_wave(duration, &mut fp, 261.63_f64).unwrap();
    sine_wave(duration, &mut fp, 293.66_f64).unwrap();
    sine_wave(duration, &mut fp, 246.94_f64).unwrap();

    sine_wave(duration, &mut fp, 196.00_f64).unwrap();
    sine_wave(duration, &mut fp, 196.00_f64).unwrap();

    sine_wave(duration * 2, &mut fp, 293.66_f64).unwrap();
    sine_wave(duration * 3, &mut fp, 261.66_f64).unwrap();
}

