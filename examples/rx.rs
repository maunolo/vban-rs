use std::{convert::TryFrom, net::UdpSocket};

fn main() -> std::io::Result<()> {
    let sock = UdpSocket::bind("0.0.0.0:6980")?;
    loop {
        let mut buf = [0; vban::MAX_PACKET_SIZE];
        let (amt, src) = sock.recv_from(&mut buf)?;

        // Parse packet
        let pkt = vban::Packet::try_from(&buf[..amt]);

        match pkt {
            Ok(pkt) => {
                // Check IP of src
                println!("IP: {}", src.ip());

                // Check stream name
                if pkt.header().stream_name() == "DemoStream" {
                    let _data_size = amt - vban::HEADER_SIZE;
                    print!("Inbound packet! ");

                    match pkt.header().sub_protocol() {
                        vban::SubProtocol::Audio => match pkt.header().codec() {
                            vban::Codec::PCM => {
                                println!("{} samples", pkt.data.len());
                            }
                            _ => (),
                        },
                        _ => (),
                    }
                }
            }
            Err(e) => println!("Error: {:?}", e),
        }
    }
}
