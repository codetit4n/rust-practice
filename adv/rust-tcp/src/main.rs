use std::io;

fn main() -> io::Result<()> {
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

        eprintln!("read {} bytes: {:x?}", nbytes, &buf[..nbytes]);
    }

    #[allow(unreachable_code)]
    Ok(())
}
