use amiquip::{Connection, Exchange, Publish, Result};
use clap::Parser;
use uuid::Uuid;
use std::time::Instant;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    queue: String,

    /// Number of messages to send
    #[arg(short, long, default_value_t = 10)]
    messages: u64,

    /// Stop after
    #[arg(short, long, default_value_t = 0)]
    time: u64,

    /// Server to connect to
    #[arg(short, long)]
    server: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Open connection.
    let mut connection = Connection::insecure_open(&args.server)?;
    // Open a channel - None says let the library choose the channel ID.
    let channel = connection.open_channel(None)?;
    // Get a handle to the direct exchange on our channel.
    let exchange = Exchange::direct(&channel);

    match args.time {
        0 => {
            for _ in 0..args.messages {
                // Publish a message to the queue.
                exchange.publish(Publish::new(Uuid::new_v4().to_string().as_bytes(), &args.queue))?;
            }
        }
        _ => {
            let start = Instant::now();
            while start.elapsed().as_secs() < args.time {
                // Publish a message to the queue.
                exchange.publish(Publish::new(Uuid::new_v4().to_string().as_bytes(), &args.queue))?;
            }
        }
    };

    connection.close()
}
