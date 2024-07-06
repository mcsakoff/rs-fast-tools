use std::fs::File;
use std::io::{BufRead, BufReader, Read, stdin};
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use anyhow::{anyhow, Result};
use clap::Parser;
use humantime::format_duration;
use log::{error, info};

use fast_tools::message::NullMessageFactory;
use fast_tools::packet::Packet;
use fastlib::{Decoder, Error, JsonMessageFactory, TextMessageFactory};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// XML file with template definitions
    #[arg(short, long, value_name = "TEMPLATES")]
    templates: Option<PathBuf>,

    /// Quiet mode; produce no message output
    #[arg(short, long)]
    quiet: bool,

    /// Output messages in JSON
    #[arg(short, long)]
    json: bool,

    /// Parse messages wrapped in packets
    #[arg(short, long)]
    packet: bool,

    /// Data file with raw packets
    #[arg()]
    data: Option<PathBuf>,
}

fn main() -> Result<()> {
    env_logger::init_from_env(
        env_logger::Env::default()
            .filter_or("LOG_LEVEL", "info")
            .write_style_or("LOG_STYLE", "always")
    );
    decode(Args::parse()).or_else(|err| {
        error!("{err}");
        Err(anyhow!(err))
    })
}

enum OutputMode {
    Null(NullMessageFactory),
    Text(TextMessageFactory),
    Json(JsonMessageFactory),
}

fn decode(args: Args) -> Result<()> {
    let mut decoder = load_templates(args.templates.as_deref())?;
    let mut data = get_data_reader(args.data.as_deref())?;

    let mut output: OutputMode;
    if args.quiet {
        output = OutputMode::Null(NullMessageFactory::new());
    } else if args.json {
        output = OutputMode::Json(JsonMessageFactory::new());
    } else {
        output = OutputMode::Text(TextMessageFactory::new());
    }

    if args.packet {
        read_all_packets(&mut decoder, &mut data, &mut output)?;
    } else {
        read_all_messages(&mut decoder, &mut data, &mut output)?;
    }
    Ok(())
}

fn load_templates(templates: Option<&Path>) -> Result<Decoder> {
    let templates = match templates {
        None => {
            info!("Using default templates...");
            include_str!("../../templates.xml").to_string()
        }
        Some(path) => {
            info!("Reading templates from {} ...", path.display());
            String::from_utf8(std::fs::read(path)?)?
        }
    };
    Ok(Decoder::new_from_xml(&templates)?)
}

fn get_data_reader(data: Option<&Path>) -> Result<Box<dyn BufRead>> {
    match data {
        None => {
            info!("Reading data from stdin...");
            Ok(Box::new(BufReader::new(stdin())))
        }
        Some(path) => {
            info!("Reading data from {} ...", path.display());
            Ok(Box::new(BufReader::new(File::open(path)?)))
        }
    }
}

fn read_all_packets(decoder: &mut Decoder, data: &mut dyn Read, output: &mut OutputMode) -> Result<()> {
    let mut packet_count = 0u64;
    let mut message_count = 0u64;
    let start = SystemTime::now();
    'main: loop {
        let packet = match Packet::read(data)? {
            None => break 'main,
            Some(p) => p,
        };
        match output {
            OutputMode::Null(msg) => {
                decoder.decode_vec(packet.payload, msg)?;
            }
            OutputMode::Text(msg) => {
                decoder.decode_vec(packet.payload, msg)?;
                println!("{}", &msg.text);
            }
            OutputMode::Json(msg) => {
                decoder.decode_vec(packet.payload, msg)?;
                println!("{}", &msg.text);
            }
        }
        packet_count += 1;
        message_count += 1;
    }
    let duration = start.elapsed()?;
    let usec_per_message = duration.as_micros() as f64 / message_count as f64;
    info!("{packet_count} packets ({message_count} messages) processed in {} ({usec_per_message:.2}us/msg)", format_duration(duration));
    Ok(())
}

fn read_all_messages(decoder: &mut Decoder, data: &mut dyn Read, output: &mut OutputMode) -> Result<()> {
    let mut message_count = 0u64;
    let start = SystemTime::now();
    'main: loop {
        let res = match output {
            OutputMode::Null(msg) => {
                decoder.decode_stream(data, msg)
            }
            OutputMode::Text(msg) => {
                decoder.decode_stream(data, msg)
            }
            OutputMode::Json(msg) => {
                decoder.decode_stream(data, msg)
            }
        };
        match res {
            Ok(_) => {}
            Err(Error::Eof) => { break 'main; }
            Err(e) => {
                return Err(anyhow!(e))
            }
        }
        match output {
            OutputMode::Null(_) => {}
            OutputMode::Text(msg) => {
                println!("{}", &msg.text);
            }
            OutputMode::Json(msg) => {
                println!("{}", &msg.text);
            }
        }
        message_count += 1;
    }
    let duration = start.elapsed()?;
    let usec_per_message = duration.as_micros() as f64 / message_count as f64;
    info!("{message_count} messages processed in {} ({usec_per_message:.2}us/msg)", format_duration(duration));
    Ok(())
}
