use std::io::{BufRead, Write};
use std::path::PathBuf;
use std::time::SystemTime;

use anyhow::{anyhow, Result};
use clap::Parser;
use humantime::format_duration;
use log::{error, info};

use fast_tools::{get_data_reader, get_data_writer, load_templates};
use fast_tools::packet::Packet;
use fastlib::{Encoder, TextMessageVisitor};

/// FAST (FIX Adapted for STreaming) protocol encoding tool
#[derive(Parser)]
#[command(author, version, name = "fast-encode")]
struct Args {
    /// XML file with template definitions
    #[arg(short, long, value_name = "TEMPLATES")]
    templates: Option<PathBuf>,

    /// Output file
    #[arg(short, long, value_name = "BIN_DATA")]
    output: Option<PathBuf>,

    /// Write messages wrapped in packets
    #[arg(short, long)]
    packet: bool,

    /// Data file with text messages, one message per line
    #[arg()]
    data: Option<PathBuf>,
}

fn main() -> Result<()> {
    env_logger::init_from_env(
        env_logger::Env::default()
            .filter_or("LOG_LEVEL", "info")
            .write_style_or("LOG_STYLE", "always")
    );
    encode(Args::parse()).or_else(|err| {
        error!("{err}");
        Err(anyhow!(err))
    })
}

fn encode(args: Args) -> Result<()> {
    let mut encoder = Encoder::new_from_xml(
        &load_templates(args.templates.as_deref())?
    )?;
    let mut input = get_data_reader(args.data.as_deref())?;
    let mut output = get_data_writer(args.output.as_deref())?;
    if args.packet {
        write_all_packets(&mut encoder, &mut input, &mut output)?;
    } else {
        write_all_messages(&mut encoder, &mut input, &mut output)?;
    }
    Ok(())
}

fn write_all_packets(encoder: &mut Encoder, input: &mut dyn BufRead, output: &mut dyn Write) -> Result<()> {
    let mut seq_num = 1u32;
    let mut packet_count = 0u64;
    let mut message_count = 0u64;
    let start = SystemTime::now();

    for line in input.lines() {
        let mut message = TextMessageVisitor::from_text(&line?)?;
        let packet = Packet {
            seq_num,
            sub_channel: 0,
            payload: encoder.encode_vec(&mut message)?,
        };
        packet.write(output)?;

        seq_num += 1;
        packet_count += 1;
        message_count += 1;
    }

    let duration = start.elapsed()?;
    let usec_per_message = duration.as_micros() as f64 / message_count as f64;
    info!("{packet_count} packets ({message_count} messages) processed in {} ({usec_per_message:.2}us/msg)", format_duration(duration));
    Ok(())
}

fn write_all_messages(encoder: &mut Encoder, input: &mut dyn BufRead, output: &mut dyn Write) -> Result<()> {
    let mut message_count = 0u64;
    let start = SystemTime::now();
    for line in input.lines() {
        let mut message = TextMessageVisitor::from_text(&line?)?;
        encoder.encode_stream(output, &mut message)?;
        message_count += 1;
    }
    let duration = start.elapsed()?;
    let usec_per_message = duration.as_micros() as f64 / message_count as f64;
    info!("{message_count} messages processed in {} ({usec_per_message:.2}us/msg)", format_duration(duration));
    Ok(())
}
