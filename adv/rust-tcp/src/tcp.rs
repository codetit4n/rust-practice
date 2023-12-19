use std::io;

enum State {
    Closed,
    Listen,
    //SynRecv,
    //Estab,
}

pub struct Connection {
    state: State,
    send: SendSequenceSpace,
    recv: ReceiveSequenceSpace,
}

///State of the Send Sequence Space (RFC 793 3.2)
///
/// ```
///            1         2          3          4
///       ----------|----------|----------|----------
///              SND.UNA    SND.NXT    SND.UNA
///                                   +SND.WND
///
/// 1 - old sequence numbers which have been acknowledged
/// 2 - sequence numbers of unacknowledged data
/// 3 - sequence numbers allowed for new data transmission
/// 4 - future sequence numbers which are not yet allowed
///
///  ```

struct SendSequenceSpace {
    /// send unacknowledged
    una: usize,
    /// send next
    nxt: usize,
    /// send window
    wnd: usize,
    /// send urgent pointer
    up: bool,
    /// segment sequence number used for last window update
    wl1: usize,
    /// segment acknowledgment number used for last window update
    wl2: usize,
    /// initial send sequence number
    iss: usize,
}

///  State of the Receive Sequence Space (RFC 793 3.2)
///
///```
///                1          2          3
///            ----------|----------|----------
///                   RCV.NXT    RCV.NXT
///                             +RCV.WND
///
/// 1 - old sequence numbers which have been acknowledged
/// 2 - sequence numbers allowed for new reception
/// 3 - future sequence numbers which are not yet allowed
///
///```

struct ReceiveSequenceSpace {
    /// receive next
    nxt: usize,
    /// receive window
    wnd: usize,
    /// receive urgent pointer
    up: bool,
    /// initial receive sequence number
    irs: usize,
}

impl Default for Connection {
    fn default() -> Self {
        todo!()

        //        Connection {
        //            //State::Closed
        //            state: State::Listen, // for starting out listen always
        //        }
    }
}

impl Connection {
    //'a is the lifetime of the packet itself
    pub fn on_packet<'a>(
        &mut self,
        nic: &mut tun_tap::Iface,
        iph: etherparse::Ipv4HeaderSlice<'a>,
        tcph: etherparse::TcpHeaderSlice<'a>,
        data: &'a [u8],
    ) -> io::Result<(usize)> {
        let mut buf = [0u8; 1500];

        match self.state {
            State::Closed => return Ok(0),
            State::Listen => {
                if !tcph.syn() {
                    // only expected SYN packet
                    return Ok(0);
                }
                // need to start estabilishing a connection
                // send an acknowlegement that you received the packet
                // reverse
                let mut syn_ack =
                    etherparse::TcpHeader::new(tcph.destination_port(), tcph.source_port(), 0, 10);
                syn_ack.acknowledgment_number = tcph.sequence_number() + 1;
                syn_ack.syn = true;
                syn_ack.ack = true;

                // wrap it in an ip packet to send back
                let mut ip = etherparse::Ipv4Header::new(
                    syn_ack.header_len(), // because no payload
                    64,
                    etherparse::ip_number::TCP,
                    [
                        iph.destination()[0],
                        iph.destination()[1],
                        iph.destination()[2],
                        iph.destination()[3],
                    ],
                    [
                        iph.source()[0],
                        iph.source()[1],
                        iph.source()[2],
                        iph.source()[3],
                    ],
                );

                // write out the headers
                let unwritten = {
                    let mut unwritten = &mut buf[..];
                    ip.write(&mut unwritten);
                    syn_ack.write(&mut unwritten);
                    unwritten.len()
                };
                nic.send(&buf[..unwritten])
            }
        }
    }
}
