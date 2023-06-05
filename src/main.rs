use std::io::{self, Write, Seek};
use std::fs::OpenOptions;
use std::ops::Sub;
use std::time::{Instant, Duration};
use log::{info, Level};
use rand::prelude::*;
use rand::{Rng, SeedableRng};
use clap::Parser;
use nix::ioctl_read_bad;
use std::os::unix::io::AsRawFd;

ioctl_read_bad!(ioctl_dkiocgetblockcount, 0x40086419, u64); // DKIOCGETBLOCKCOUNT
ioctl_read_bad!(ioctl_dkiocgetblocksize, 0x40046418, u32); // DKIOCGETBLOCKSIZE

fn get_block_device_size(device: &std::fs::File) -> nix::Result<u64> {
    let mut block_count: u64 = 0;
    let mut block_size: u32 = 0;

    unsafe {
        info!("Calling ioctls");
        ioctl_dkiocgetblockcount(device.as_raw_fd(), &mut block_count)?;
        ioctl_dkiocgetblocksize(device.as_raw_fd(), &mut block_size)?;
    }

    Ok(block_count * block_size as u64)
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    device: String,

    #[arg(short, long, default_value_t = 33554432)] //32MB default
    block_size: usize
}

fn main() -> io::Result<()> {
    simple_logger::init_with_level(Level::Info).unwrap();
    let args = Args::parse();

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(args.device)?;
        
    let mut rng = SmallRng::from_entropy();
    let mut block = vec![0u8; args.block_size];
    let mut total_blocks = 0u64;
    let origstart = Instant::now();
    let mut start = Instant::now();
    let block_device_size = get_block_device_size(&file)?;

    info!("Block size: {}", args.block_size);
    info!("Block device size: {}", block_device_size);
    let block_count = block_device_size / args.block_size as u64;

    loop {
        rng.fill_bytes(&mut block);
        let block_offset = rng.gen_range(0..block_count);
        let offset = block_offset * args.block_size as u64;
        file.seek(std::io::SeekFrom::Start(offset))?;
        file.write_all(&block)?;

        total_blocks += 1;
        if start.elapsed() > Duration::from_secs(10) {
            start = Instant::now();
            let bytes_written = total_blocks * args.block_size as u64;
            let elapsed_seconds = start.sub(origstart).as_secs();
            let megabytes_per_second = bytes_written / elapsed_seconds / (1024*1024);
            let pct = (bytes_written as f64 / block_device_size as f64) * 100.0;
            info!("{} bytes written over {} randomly seeked locations in {} seconds ({} MB/s)\n{}% of disk overwritten", bytes_written, total_blocks, elapsed_seconds, megabytes_per_second, pct);
        }
    }
}
