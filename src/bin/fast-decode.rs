use std::io::{Read, Write};
use std::path::PathBuf;
use std::time::SystemTime;

use anyhow::{anyhow, Result};
use clap::Parser;
use humantime::format_duration;
use log::{error, info};

use fast_tools::{get_data_reader, get_data_writer, load_templates};
use fast_tools::message::NullMessageFactory;
use fast_tools::packet::Packet;
use fastlib::{Decoder, Error, JsonMessageFactory, TextMessageFactory};

/// FAST (FIX Adapted for STreaming) protocol decoding tool
#[derive(Parser)]
#[command(author, version, name = "fast-decode")]
struct Args {
    /// XML file with template definitions
    #[arg(short, long, value_name = "TEMPLATES")]
    templates: Option<PathBuf>,

    /// Output file
    #[arg(short, long, value_name = "TEXT_DATA")]
    output: Option<PathBuf>,

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
    let mut decoder = Decoder::new_from_xml(
        &load_templates(args.templates.as_deref())?
    )?;
    let mut input = get_data_reader(args.data.as_deref())?;
    let mut output = get_data_writer(args.output.as_deref())?;

    let mut mode: OutputMode;
    if args.quiet {
        mode = OutputMode::Null(NullMessageFactory::new());
    } else if args.json {
        mode = OutputMode::Json(JsonMessageFactory::new());
    } else {
        mode = OutputMode::Text(TextMessageFactory::new());
    }

    if args.packet {
        read_all_packets(&mut decoder, &mut input, &mut output, &mut mode)?;
    } else {
        read_all_messages(&mut decoder, &mut input, &mut output, &mut mode)?;
    }
    Ok(())
}

fn read_all_packets(decoder: &mut Decoder, data: &mut dyn Read, output: &mut dyn Write, mode: &mut OutputMode) -> Result<()> {
    let mut packet_count = 0u64;
    let mut message_count = 0u64;
    let start = SystemTime::now();

    'main: loop {
        let packet = match Packet::read(data)? {
            None => break 'main,
            Some(p) => p,
        };
        match mode {
            OutputMode::Null(msg) => {
                decoder.decode_vec(packet.payload, msg)?;
            }
            OutputMode::Text(msg) => {
                decoder.decode_vec(packet.payload, msg)?;
                output.write_all(msg.text.as_bytes())?;
                output.write_all(b"\n")?;
                output.flush()?;
            }
            OutputMode::Json(msg) => {
                decoder.decode_vec(packet.payload, msg)?;
                output.write_all(msg.json.as_bytes())?;
                output.write_all(b"\n")?;
                output.flush()?;
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

fn read_all_messages(decoder: &mut Decoder, data: &mut dyn Read, output: &mut dyn Write, mode: &mut OutputMode) -> Result<()> {
    let mut message_count = 0u64;
    let start = SystemTime::now();

    'main: loop {
        let res = match mode {
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
        match mode {
            OutputMode::Null(_) => {}
            OutputMode::Text(msg) => {
                output.write_all(msg.text.as_bytes())?;
                output.write_all(b"\n")?;
                output.flush()?;
            }
            OutputMode::Json(msg) => {
                output.write_all(msg.json.as_bytes())?;
                output.write_all(b"\n")?;
                output.flush()?;
            }
        }
        message_count += 1;
    }

    let duration = start.elapsed()?;
    let usec_per_message = duration.as_micros() as f64 / message_count as f64;
    info!("{message_count} messages processed in {} ({usec_per_message:.2}us/msg)", format_duration(duration));
    Ok(())
}
