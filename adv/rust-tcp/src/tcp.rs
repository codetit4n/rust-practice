use std::{cmp::Ordering, io};

enum State {
    //Listen,
    SynRecv,
    Estab,
    FinWait1,
    FinWait2,
    TimeWait,
}

impl State {
    fn is_synchronized(&self) -> bool {
        match *self {
            State::SynRecv => false,
            State::Estab | State::FinWait1 | State::FinWait2 | State::TimeWait => true,
        }
    }
}

pub struct Connection {
    state: State,
    send: SendSequenceSpace,
    recv: ReceiveSequenceSpace,
    ip: etherparse::Ipv4Header,
    tcp: etherparse::TcpHeader,
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
    /// send unacknowledged - oldest unacknowledged sequence number
    una: u32,
    /// send next
    nxt: u32,
    /// send window
    wnd: u16,
    /// send urgent pointer
    up: bool,
    /// segment sequence number used for last window update
    wl1: usize,
    /// segment acknowledgment number used for last window update
    wl2: usize,
    /// initial send sequence number
    iss: u32,
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
    nxt: u32,
    /// receive window
    wnd: u16,
    /// receive urgent pointer
    up: bool,
    /// initial receive sequence number
    irs: u32,
}

impl Connection {
    //'a is the lifetime of the packet itself
    pub fn accept<'a>(
        nic: &mut tun_tap::Iface,
        iph: etherparse::Ipv4HeaderSlice<'a>,
        tcph: etherparse::TcpHeaderSlice<'a>,
        data: &'a [u8],
    ) -> io::Result<Option<Self>> {
        let mut buf = [0u8; 1500];

        if !tcph.syn() {
            // only expected SYN packet
            return Ok(None);
        }

        let iss = 0;
        let wnd = 1024;

        let mut c = Connection {
            state: State::SynRecv,
            send: SendSequenceSpace {
                // decide on stuff we're sending them
                iss: 0,
                una: iss,
                nxt: iss,
                wnd,
                up: false,
                wl1: 0,
                wl2: 0,
            },
            recv: ReceiveSequenceSpace {
                // keep track of sender info
                irs: tcph.sequence_number(),
                nxt: tcph.sequence_number() + 1,
                wnd: tcph.window_size(),
                up: false,
            },
            ip: etherparse::Ipv4Header::new(
                0,
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
            ),
            tcp: etherparse::TcpHeader::new(tcph.destination_port(), tcph.source_port(), iss, wnd),
        };

        // need to start estabilishing a connection
        // send an acknowlegement that you received the packet
        // reverse
        c.tcp.syn = true;
        c.tcp.ack = true;

        c.write(nic, &[])?;

        Ok(Some(c))
    }

    fn write(&mut self, nic: &mut tun_tap::Iface, payload: &[u8]) -> io::Result<usize> {
        let mut buf = [0u8; 1500];
        self.tcp.sequence_number = self.send.nxt;
        self.tcp.acknowledgment_number = self.recv.nxt;

        // because we cannot write out more than the size of buffer
        let size = std::cmp::min(
            buf.len(),
            self.tcp.header_len() as usize + self.ip.header_len() as usize + payload.len(),
        );

        self.ip
            .set_payload_len(size - self.ip.header_len() as usize);

        self.tcp.checksum = self
            .tcp
            .calc_checksum_ipv4(&self.ip, &[])
            .expect("Failed to compute checksum!");

        // write out the headers
        use std::io::Write;
        let mut unwritten = &mut buf[..];
        self.ip.write(&mut unwritten);
        self.tcp.write(&mut unwritten);
        let payload_bytes = unwritten.write(payload)?;
        let unwritten = unwritten.len();
        self.send.nxt.wrapping_add(payload_bytes as u32);
        if self.tcp.syn {
            self.send.nxt = self.send.nxt.wrapping_add(1 as u32);
            self.tcp.syn = false;
        }
        if self.tcp.fin {
            self.send.nxt = self.send.nxt.wrapping_add(1 as u32);
            self.tcp.fin = false;
        }
        nic.send(&buf[..buf.len() - unwritten])?;
        Ok(payload_bytes)
    }

    fn send_rst(&mut self, nic: &mut tun_tap::Iface) -> io::Result<()> {
        // todo: fix sequence numbers here
        // If the incoming segment has an ACK field, the reset takes its
        // sequence number from the ACK field of the segment, otherwise the
        // reset has sequence number zero and the ACK field is set to the sum
        // of the sequence number and segment length of the incoming segment.
        // The connection remains in the same state.
        //
        // todo: handle synchornized RST
        // If the connection is in a synchronized state (ESTABLISHED,
        // FIN-WAIT-1, FIN-WAIT-2, CLOSE-WAIT, CLOSING, LAST-ACK, TIME-WAIT),
        // any unacceptable segment (out of window sequence number or
        // unacceptible acknowledgment number) must elicit only an empty
        // acknowledgment segment containing the current send-sequence number
        // and an acknowledgment indicating the next sequence number expected
        // to be received, and the connection remains in the same state.
        self.tcp.rst = true;
        self.tcp.sequence_number = 0;
        self.tcp.acknowledgment_number = 0;
        self.write(nic, &[])?;
        Ok(())
    }

    pub fn on_packet<'a>(
        &mut self,
        nic: &mut tun_tap::Iface,
        iph: etherparse::Ipv4HeaderSlice<'a>,
        tcph: etherparse::TcpHeaderSlice<'a>,
        data: &'a [u8],
    ) -> io::Result<()> {
        // first check that sequence numbers are valid (RFC 793, 3.3)
        //
        // valid segment check. ok if it acks atleast one byte, which means atleast one of the
        // following is true:
        //
        // RCV.NXT =< SEG.SEQ < RCV.NXT + RCV.WND
        // RCV.NXT =< SEG.SEQ + SEG.LEN - 1 < RCV.NXT + RCV.WND
        //

        // Zero length segments and zero windows have their own rules: From RFC 793 Pg 26:
        // Due to zero windows and zero length segments, we have four cases for the
        // acceptability of an incoming segment:

        //  Segment Receive  Test
        //  Length  Window
        //  ------- -------  -------------------------------------------

        //     0       0     SEG.SEQ = RCV.NXT

        //     0      >0     RCV.NXT =< SEG.SEQ < RCV.NXT+RCV.WND

        //    >0       0     not acceptable

        //    >0      >0     RCV.NXT =< SEG.SEQ < RCV.NXT+RCV.WND
        //                or RCV.NXT =< SEG.SEQ+SEG.LEN-1 < RCV.NXT+RCV.WND
        //
        let seqn = tcph.sequence_number();
        let mut slen = data.len() as u32;
        if tcph.fin() {
            slen += 1;
        };
        if tcph.syn() {
            slen += 1;
        };
        let wend = self.recv.nxt.wrapping_add(self.recv.wnd as u32);
        let okay = if slen == 0 && !tcph.syn() && !tcph.fin() {
            if self.recv.wnd == 0 {
                if seqn != self.recv.nxt {
                    false
                } else {
                    true
                }
            } else if !is_between_wrapped(self.recv.nxt.wrapping_sub(1), seqn, wend) {
                false
            } else {
                true
            }
        } else {
            if self.recv.wnd == 0 {
                false
            } else if !is_between_wrapped(self.recv.nxt.wrapping_sub(1), seqn, wend)
                && !is_between_wrapped(self.recv.nxt.wrapping_sub(1), seqn + slen - 1, wend)
            {
                false
            } else {
                true
            }
        };

        if !okay {
            self.write(nic, &[])?;
            return Ok(());
        }

        self.recv.nxt = seqn.wrapping_add(slen);
        // TODO: If not acceptale send ACK
        // <SEQ=SND.NXT><ACK=RCV.NXT><CTL=ACK>

        if !tcph.ack() {
            return Ok(());
        }

        let ackn = tcph.acknowledgment_number();
        if let State::SynRecv = self.state {
            if is_between_wrapped(
                self.send.una.wrapping_sub(1),
                ackn,
                self.send.nxt.wrapping_add(1),
            ) {
                // must have ACKed our SYN, since we detected at least one acked byte, and we have
                // only sent one byte (the SYN)
                self.state = State::Estab;
            } else {
                //TODO: <SEQ=SEG.ACK><CTL=RST>
            }
        };

        if let State::Estab | State::FinWait1 | State::FinWait2 = self.state {
            if !is_between_wrapped(self.send.una, ackn, self.send.nxt.wrapping_add(1)) {
                return Ok(());
            }

            self.send.una = ackn;
            //TODO
            assert!(data.is_empty());

            if let State::Estab = self.state {
                // now let`s terminate the connection - closing means i have no more data to send
                // but you can still receive
                // todo: needs to be stored in the restarsmission que
                self.tcp.fin = true;
                self.write(nic, &[])?;
                self.state = State::FinWait1;
            }
        }
        if let State::FinWait1 = self.state {
            if self.send.una == self.send.iss + 2 {
                // our FIN has been ACKed
                self.state = State::FinWait2;
            }
        }

        if tcph.fin() {
            match self.state {
                State::FinWait2 => {
                    //we're done with the connection
                    self.write(nic, &[])?;
                    self.state = State::TimeWait;
                }
                _ => unimplemented!(),
            }
        }
        Ok(())
    }
}

fn is_between_wrapped(start: u32, x: u32, end: u32) -> bool {
    match start.cmp(&x) {
        Ordering::Equal => return false,
        Ordering::Less => {
            // we have:
            //
            // 0 |------------S-----------X-------------------| (wraparound)
            //
            // X is between S and E (S < X < E) in these cases:
            //
            // 0 |------------S-----------X----E--------------| (wraparound)
            //
            // 0 |-------E----S-----------X-------------------| (wraparound)
            //
            // but *not* in these cases
            //
            // 0 |------------S-----E-----X-------------------|(wraparound)
            //
            // 0 |------------|-----------X-------------------|(wraparound)
            //               ^-S+E
            //
            // 0 |------------S-----------|-------------------|(wraparound)
            //                          ^-X+E
            //
            // or, in other words, iff !(S <= E <= X)
            if end >= start && end <= x {
                return false;
            }
        }

        Ordering::Greater => {
            // we have, opposite of above:
            //
            // 0 |------------X-----------S-------------------| (wraparound)
            //
            // X is between S and E (S < X < E) *only* in this case:
            //
            // 0 |------------X----E------S-------------------| (wraparound)
            //
            //
            // but *not* in these cases
            //
            // 0 |------------X-----S-----E-------------------|(wraparound)
            //
            // 0 |--------E---X-----S-------------------------|(wraparound)
            //
            // 0 |------------|-----------S-------------------|(wraparound)
            //               ^-X+E
            //
            // 0 |------------X-----------|-------------------|(wraparound)
            //                          ^-S+E
            //
            //or, in other words, iff S < E < X
            if end < start && end > x {
            } else {
                return false;
            }
        }
    };
    true
}
