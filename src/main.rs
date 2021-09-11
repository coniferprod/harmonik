use std::collections::HashMap;
use std::fmt;

use structopt::StructOpt;
use rand::Rng;
use ksynth::k5000::harmonic::{Levels, Envelope};
use syxpack::{Message, Manufacturer, ManufacturerId};

const MAX_LEVEL: u8 = 127;
const HARMONIC_COUNT: usize = 64;

// Converts the amplitude to the K5000 harmonic level setting.
fn get_level(a: f64) -> u8 {
    ((a.abs().log2() * 8.0) + MAX_LEVEL as f64).floor() as u8
}

fn get_sine_levels() -> [u8; HARMONIC_COUNT] {
    // Initialize first harmonic to max level, all others to zero
    let mut levels = [0u8; HARMONIC_COUNT];
    levels[0] = MAX_LEVEL;
    levels
}

fn get_saw_levels() -> [u8; HARMONIC_COUNT] {
    let mut levels = [0u8; HARMONIC_COUNT];

    // Get level corresponding to 1/n for each harmonic.
    for i in 0..HARMONIC_COUNT {
        let n = i + 1; // harmonic numbers start at 1
        let a = 1.0 / n as f64;
        println!("{}: {}", n, a);
        levels[i] = get_level(a);
    }

    levels
}

fn get_square_levels() -> [u8; HARMONIC_COUNT] {
    let mut levels = [0u8; HARMONIC_COUNT];

    // Get the sawtooth levels and take out the even harmonics to get square levels.
    let saw_levels = get_saw_levels();
    for i in 0..HARMONIC_COUNT {
        let n = i + 1; // harmonic numbers start at 1
        levels[i] = if n % 2 != 0 { saw_levels[i] } else { 0 }
    }

    levels
}

fn get_triangle_levels() -> [u8; HARMONIC_COUNT] {
    let mut levels = [0u8; HARMONIC_COUNT];

    // Get levels for amplitude 1/n^2 for each harmonic n.
    let mut is_negative = false;  // true if the current harmonic is negative
    for i in 0..HARMONIC_COUNT {
        let n = i + 1; // harmonic numbers start at 1
        let mut level = 0;
        if n % 2 != 0 { // using only odd harmonics
            let mut a = 1.0 / ((n * n) as f64);
            if is_negative {
                a = -a;
                is_negative = !is_negative;
            }
            level = get_level(a)
        }
        levels[i] = level;
    }

    levels
}

#[derive(Debug)]
struct Parameters {
    a: f64,
    b: f64,
    c: f64,
    xp: f64,
    d: f64,
    e: f64,
    yp: f64,
}

use std::f64::consts::PI;

fn get_custom_level(n: usize, params: &Parameters) -> u8 {
    fn compute(n: usize, params: &Parameters) -> f64 {
        // (a, b, c, xp, d, e, yp) = waveform_params

        let x = n as f64 * PI * params.xp;
        let y = n as f64 * PI * params.yp;
        //eprintln!("    x = {}, y = {}", x, y);

        let module1 = 1.0 / params.a.powi(n as i32);
        let module2 = (x.sin()).powf(params.b) * (x.cos()).powf(params.c);
        let module3 = (y.sin()).powf(params.d) * (y.cos()).powf(params.e);

        module1 * module2 * module3
    }

    let max_level = 127f64;
    let a_max = 1.0;
    let a = compute(n, params);
    let v = ((a / a_max).abs()).log2();
    eprintln!("{}: a = {}, v = {}", n, a, v);
    let level = max_level + 8.0 * v;
    if level >= 0.0 {
        level.floor() as u8
    }
    else {
        0
    }
}

fn get_custom_levels(params: &Parameters) -> [u8; HARMONIC_COUNT] {
    let mut levels = [0u8; HARMONIC_COUNT];

    for i in 0..HARMONIC_COUNT {
        let n = i + 1; // harmonic numbers start at 1
        levels[i] = get_custom_level(n, &params);
    }

    levels
}

fn get_random_levels() -> [u8; HARMONIC_COUNT] {
    let mut rng = rand::thread_rng();
    let mut levels = [0u8; HARMONIC_COUNT];
    for i in 0..HARMONIC_COUNT {
        levels[i] = rng.gen_range(0..128)
    }
    levels
}

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(short, long)]
    waveform: String,

    #[structopt(name = "params", required_if("waveform", "custom"))]
    params: Option<String>,

    #[structopt(default_value = "MIDI Out", short, long)]
    device: String,

    #[structopt(default_value = "1", short, long)]
    channel: u8,
}

fn main() {
    let cli = Cli::from_args();
    //println!("{:?}", cli);

    let device = cli.device;
    let channel = cli.channel - 1;  // adjust channel to zero-based

    match cli.waveform.as_str() {
        "custom" => {
            if let Some(values) = cli.params {
                let numbers: Vec<f64> = values.split(",").map(|s| s.parse::<f64>().unwrap()).collect();
                eprintln!("numbers = {:?}", numbers);
                let params = Parameters {
                    a: numbers[0],
                    b: numbers[1],
                    c: numbers[2],
                    xp: numbers[3],
                    d: numbers[4],
                    e: numbers[5],
                    yp: numbers[6],
                };
                eprintln!("params = {:?}", params);
                let levels = get_custom_levels(&params);
                println!("{}", make_graph(&levels));
                make_sysex_messages(&device, &levels, channel, 0, 0);
            };
        },
        "sine" => {
            let levels = get_sine_levels();
            println!("{}", make_graph(&levels));
            make_sysex_messages(&device, &levels, channel, 0, 0);
        },
        "saw" => {
            let levels = get_saw_levels();
            println!("{}", make_graph(&levels));
            make_sysex_messages(&device, &levels, channel, 0, 0);
        },
        "square" => {
            let levels = get_square_levels();
            println!("{}", make_graph(&levels));
            make_sysex_messages(&device, &levels, channel, 0, 0);
        },
        "triangle" => {
            let levels = get_triangle_levels();
            println!("{}", make_graph(&levels));
            make_sysex_messages(&device, &levels, channel, 0, 0);
        },
        "random" => {
            let levels = get_random_levels();
            println!("{}", make_graph(&levels));
            make_sysex_messages(&device, &levels, channel, 0, 0);
        }
        _ => {
            eprintln!("Unknown waveform");
        }
    }

    /*
    let mut custom_params = HashMap::<&str, Parameters>::new();
    custom_params.insert("saw", Parameters { a: 1.0, b: 0.0, c: 0.0, xp: 0.0, d: 0.0, e: 0.0, yp: 0.0 });
    custom_params.insert("square", Parameters { a: 1.0, b: 1.0, c: 0.0, xp: 0.5, d: 0.0, e: 0.0, yp: 0.0 });
    custom_params.insert("triangle", Parameters { a: 2.0, b: 1.0, c: 0.0, xp: 0.5, d: 0.0, e: 0.0, yp: 0.0 });
    custom_params.insert("pulse20", Parameters { a: 1.0, b: 1.0, c: 0.0, xp: 0.2, d: 0.0, e: 0.0, yp: 0.0 });

    for (k, v) in custom_params.into_iter() {
        println!("custom / {}: {:?}", k, get_custom_levels(&v));
    }
    */
}

// Makes a graph of the harmonic levels as a string to print out.
fn make_graph(levels: &[u8]) -> String {
    make_vertical_graph(levels)
}

fn make_horizontal_graph(levels: &[u8]) -> String {
    let mut result = String::new();


    result
}

fn make_vertical_graph(levels: &[u8]) -> String {
    let mut result = String::new();

    let mut index = 1;
    // First just make 64 lines that have a vertical bar that approximates the harmonic level.
    for level in levels {
        // Levels are 0...127; use that divided by 10 to get max 12 characters,
        // then scale up to max line width of 80
        let count = (*level as usize / 10) * 6;
        result.push_str(&format!("{:2}: ", index));
        result.push_str(&"*".repeat(count));
        result.push('\n');
        index += 1;
    }

    result
}

fn make_harmonic_sysex(harmonic_num: u32, channel: u8, level: u8, group_num: u32, source_num: u32) -> Message {
    Message::new(
        Manufacturer::from_id(ManufacturerId::Standard(0x40)),
        vec![
            channel,  // MIDI channel 0...15
            0x10, // function number
            0x00, // synth group
            0x0a, // machine number
            0x02, // "Single Tone ADD Wave Parameter"
            0x40 + group_num as u8,
            source_num as u8,  // 00h ... 05h
            harmonic_num as u8, // harmonic number 0...63
            0,
            0,
            level,
        ]
    )
}

fn make_sysex_messages(device: &str, levels: &[u8], channel: u8, group_num: u32, source_num: u32) {
    for i in 0..HARMONIC_COUNT {
        let message = make_harmonic_sysex(i as u32, channel, levels[i], group_num, source_num);
        print!("sendmidi dev \"{}\" hex syx ", device);
        let mut bytes = message.to_bytes();
        let original_len = bytes.len();
        bytes.remove(0);  // remove first element (the SysEx initiator)
        bytes.remove(bytes.len() - 1); // remove last element (the SysEx terminator)
        assert!(bytes.len() == original_len - 2);
        for b in bytes {
            print!("{:02x} ", b);
        }
        println!("");
    }
}
