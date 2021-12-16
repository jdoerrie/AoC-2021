struct Header {
    version: u8,
    type_id: u8,
}

enum Packet {
    Literal(Header, u64),
    Operator(Header, Vec<Packet>),
}

struct BitsRead(usize);

fn parse_header(data: &str) -> (Header, BitsRead) {
    let version = u8::from_str_radix(&data[0..3], 2).unwrap();
    let type_id = u8::from_str_radix(&data[3..6], 2).unwrap();
    (Header { version, type_id }, BitsRead(6))
}

fn parse_literal(mut data: &str) -> (u64, BitsRead) {
    let mut result = 0;
    let mut bits_read = 0;
    loop {
        bits_read += 5;
        result = (result << 4) + u64::from_str_radix(&data[1..5], 2).unwrap();
        if data.as_bytes()[0] == b'0' {
            break;
        }

        data = &data[5..];
    }

    (result, BitsRead(bits_read))
}

fn parse_operator(mut data: &str) -> (Vec<Packet>, BitsRead) {
    let mut packets = Vec::new();
    let mut read_bits = 0;

    let length_type_id = data.as_bytes()[0];
    if length_type_id == b'0' {
        let len_packets = usize::from_str_radix(&data[1..16], 2).unwrap();
        data = &data[16..];

        while read_bits < len_packets {
            let (packet, BitsRead(bits)) = parse_packet(data);
            packets.push(packet);
            read_bits += bits;
            data = &data[bits..];
        }

        assert_eq!(read_bits, len_packets);
        read_bits += 16;
    } else {
        let num_packets = usize::from_str_radix(&data[1..12], 2).unwrap();
        data = &data[12..];

        for _ in 0..num_packets {
            let (packet, BitsRead(bits)) = parse_packet(data);
            packets.push(packet);
            read_bits += bits;
            data = &data[bits..];
        }

        read_bits += 12;
    }

    (packets, BitsRead(read_bits))
}

fn parse_packet(data: &str) -> (Packet, BitsRead) {
    let (header, _) = parse_header(&data[0..6]);
    let data = &data[6..];
    match header.type_id {
        4 => {
            let (literal, BitsRead(n)) = parse_literal(data);
            (Packet::Literal(header, literal), BitsRead(n + 6))
        }
        _ => {
            let (packets, BitsRead(n)) = parse_operator(data);
            (Packet::Operator(header, packets), BitsRead(n + 6))
        }
    }
}

fn sum_versions(packet: &Packet) -> u64 {
    match packet {
        Packet::Literal(header, _) => header.version as u64,
        Packet::Operator(header, packets) => {
            packets.iter().fold(header.version as u64, |acc, packet| {
                acc + sum_versions(packet)
            })
        }
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let bit_string = input
        .trim()
        .chars()
        .map(|c| format!("{:04b}", u8::from_str_radix(&c.to_string(), 16).unwrap()))
        .collect::<Vec<_>>()
        .join("");
    println!("{}", sum_versions(&parse_packet(&bit_string).0));
}
