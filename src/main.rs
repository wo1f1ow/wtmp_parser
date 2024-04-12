use std::env;
use std::fs::File;
use std::io::{BufReader, Read};
use std::mem;

const WTMP_REC_SIZE: usize = 384;
const WTMP_REC_TYPES: [&str; 10] = [
    "Empty",
    "Run Lvl",
    "Boot",
    "New Time",
    "Old Time",
    "Init",
    "Login",
    "Normal",
    "Term",
    "Account",
];

#[derive(Debug)]
struct WtmpRecord {
    rec_type: u32,
    pid: u32,
    line: [u8; 32],
    inittab: u32,
    user: [u8; 32],
    host: [u8; 256],
    t1: u32,
    t2: u32,
    t3: u32,
    t4: u32,
    t5: u32,
}

fn main() {
    let mut wtmp_file = "/var/log/wtmp";
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && args[1] == "-f" {
        if args.len() == 3 {
            wtmp_file = &args[2];
        } else {
            eprintln!("Usage: {} [-f wtmp_file]", args[0]);
            return;
        }
    }

    let file = File::open(wtmp_file).expect("Failed to open wtmp file");
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();

    reader.read_to_end(&mut buffer).expect("Failed to read wtmp file");

    for record in buffer.chunks(WTMP_REC_SIZE) {
        if record.len() == WTMP_REC_SIZE {
            let mut wtmp_rec: WtmpRecord = unsafe { mem::zeroed() };
            wtmp_rec.rec_type = u32::from_le_bytes(record[0..4].try_into().unwrap());
            wtmp_rec.pid = u32::from_le_bytes(record[4..8].try_into().unwrap());
            wtmp_rec.line = record[8..40].try_into().unwrap();
            wtmp_rec.inittab = u32::from_le_bytes(record[40..44].try_into().unwrap());
            wtmp_rec.user = record[44..76].try_into().unwrap();
            wtmp_rec.host = record[76..332].try_into().unwrap();
            wtmp_rec.t1 = u32::from_le_bytes(record[332..336].try_into().unwrap());
            wtmp_rec.t2 = u32::from_le_bytes(record[336..340].try_into().unwrap());
            wtmp_rec.t3 = u32::from_le_bytes(record[340..344].try_into().unwrap());
            wtmp_rec.t4 = u32::from_le_bytes(record[344..348].try_into().unwrap());
            wtmp_rec.t5 = u32::from_le_bytes(record[348..352].try_into().unwrap());

            if wtmp_rec.line.iter().any(|&b| b != 0) {
                let line = String::from_utf8_lossy(&wtmp_rec.line).trim_end_matches(char::from(0)).to_string();
                let host = String::from_utf8_lossy(&wtmp_rec.host).trim_end_matches(char::from(0)).to_string();
                let user = String::from_utf8_lossy(&wtmp_rec.user).trim_end_matches(char::from(0)).to_string();
                let rec_type_str = WTMP_REC_TYPES.get(wtmp_rec.rec_type as usize).unwrap_or(&"Unknown");

                println!(
                    "{} {:-8} {:-12} {:-10} {:-45}",
                    chrono::NaiveDateTime::from_timestamp(wtmp_rec.t3 as i64, 0),
                    rec_type_str,
                    user,
                    line,
                    host
                );
            }
        }
    }
}