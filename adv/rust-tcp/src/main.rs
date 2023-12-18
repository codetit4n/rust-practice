use std::net::Ipv4Addr;
use std::{collections::HashMap, io};

mod tcp;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Quad {
    src: (Ipv4Addr, u16),
    dst: (Ipv4Addr, u16),
}

fn main() -> io::Result<()> {
    let connections: HashMap<Quad, tcp::State> = Default::default();

    // Create a new TUN interface in TUN mode
    let nic = tun_tap::Iface::new("tun0", tun_tap::Mode::Tun)?;

    // Define a buffer of size 1504 bytes (max ethernet frame size without CRC)
    // to store the received data
    let mut buf = [0u8; 1504];

    // Loop for continuous receiving of data
    loop {
        // receive data from TUN interface and store the number of bytes received in
        // nbytes
        let nbytes = nic.recv(&mut buf[..])?;
        let _eth_flags = u16::from_be_bytes([buf[0], buf[1]]);
        let eth_proto = u16::from_be_bytes([buf[2], buf[3]]); // https://en.wikipedia.org/wiki/EtherType#Values

        if eth_proto != 0x0800 {
            // ignore everything other than IPv4 packets
            continue;
        }

        match etherparse::Ipv4HeaderSlice::from_slice(&buf[4..nbytes]) {
            Ok(p) => {
                let src = p.source_addr();
                let dst = p.destination_addr();
                let proto = p.protocol(); // ip level protocol https://www.iana.org/assignments/protocol-numbers/protocol-numbers.xhtml

                if proto != 0x06 {
                    //not tcp
                    continue;
                }

                match etherparse::TcpHeaderSlice::from_slice(&buf[4 + p.slice().len()..]) {
                    Ok(p) => {
                        //todo!
                        eprintln!(
                            "{} -> {} {}b of tcp to port {}",
                            src,
                            dst,
                            p.slice().len(),
                            p.destination_port()
                        )
                    }

                    Err(e) => {
                        eprintln!("ignoring weird packet {:?}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("ignoring weird packet {:?}", e);
            }
        }
    }
}
