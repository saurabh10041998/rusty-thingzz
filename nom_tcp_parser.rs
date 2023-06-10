use nom::bits;
use nom::error::{Error, ErrorKind};
use nom::number;
use nom::sequence;
use nom::Needed;
use nom::{Err, IResult};

use indoc::writedoc;

const END_OF_OPTIONS: u8 = 0;
const NO_OP: u8 = 1;
const MSS: u8 = 2;
const WINDOW_SCALE: u8 = 3;
const SACK_PERMITTED: u8 = 4;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum TcpOption {
    EndOfOptions,
    NoOperation,
    MaximumSegmentSize(MaximumSegmentSize),
    WindowScaling(WindowScaling),
    SackPermitted,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct MaximumSegmentSize {
    mss: u16,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct WindowScaling {
    scaling: u8,
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
struct TcpHeader {
    source_port: u16,
    dest_port: u16,
    sequence_no: u32,
    ack_no: u32,
    hlen: u8,
    reserved: u8,
    flag_urg: bool,
    flag_ack: bool,
    flag_psh: bool,
    flag_rst: bool,
    flag_syn: bool,
    flag_fin: bool,
    window: u16,
    checksum: u16,
    urgent_pointer: u16,
    options: Option<Vec<TcpOption>>,
}

fn hlen_res_flags(input: &[u8]) -> IResult<&[u8], (u8, u8, u8)> {
    bits::bits::<_, _, Error<_>, _, _>(sequence::tuple((
        bits::streaming::take(4u8),
        bits::streaming::take(6u8),
        bits::streaming::take(6u8),
    )))(input)
}

fn parse_tcp(input: &[u8]) -> IResult<&[u8], TcpHeader> {
    let (input, source_port) = number::streaming::be_u16(input)?;
    let (input, dest_port) = number::streaming::be_u16(input)?;
    let (input, sequence_no) = number::streaming::be_u32(input)?;
    let (input, ack_no) = number::streaming::be_u32(input)?;
    let (input, hlen_res_flags) = hlen_res_flags(input)?;
    let (input, window) = number::streaming::be_u16(input)?;
    let (input, checksum) = number::streaming::be_u16(input)?;
    let (input, urgent_pointer) = number::streaming::be_u16(input)?;

    let header = TcpHeader {
        source_port,
        dest_port,
        sequence_no,
        ack_no,
        hlen: hlen_res_flags.0,
        reserved: hlen_res_flags.1,
        flag_urg: hlen_res_flags.2 & 0b10_0000 == 0b10_0000,
        flag_ack: hlen_res_flags.2 & 0b01_0000 == 0b01_0000,
        flag_psh: hlen_res_flags.2 & 0b00_1000 == 0b00_1000,
        flag_rst: hlen_res_flags.2 & 0b00_0100 == 0b00_0100,
        flag_syn: hlen_res_flags.2 & 0b00_0010 == 0b00_0010,
        flag_fin: hlen_res_flags.2 & 0b00_0001 == 0b00_0001,
        window,
        checksum,
        urgent_pointer,
        options: None,
    };

    return Ok((input, header));
}

fn parse_tcp_option(input: &[u8]) -> IResult<&[u8], TcpOption> {
    match number::streaming::be_u8(input)? {
        (input, END_OF_OPTIONS) => Ok((input, TcpOption::EndOfOptions)),
        (input, NO_OP) => Ok((input, TcpOption::NoOperation)),
        (input, MSS) => {
            let (input, _len) = number::streaming::be_u8(input)?;
            let (input, mss) = number::streaming::be_u16(input)?;
            Ok((
                input,
                TcpOption::MaximumSegmentSize(MaximumSegmentSize { mss }),
            ))
        }
        (input, WINDOW_SCALE) => {
            let (input, _len) = number::streaming::be_u8(input)?;
            let (input, scaling) = number::streaming::be_u8(input)?;
            Ok((input, TcpOption::WindowScaling(WindowScaling { scaling })))
        }
        (input, SACK_PERMITTED) => {
            let (input, _len) = number::streaming::be_u8(input)?;
            Ok((input, TcpOption::SackPermitted))
        }
        _ => Err(Err::Failure(Error::new(input, ErrorKind::Switch))),
    }
}

fn parse_tcp_options(input: &[u8]) -> IResult<&[u8], Vec<TcpOption>> {
    let mut left = input;
    let mut options = vec![];
    loop {
        match parse_tcp_option(left) {
            Ok((l, opt)) => {
                left = l;
                options.push(opt);
                if let TcpOption::EndOfOptions = opt {
                    break;
                }
            }
            Err(e) => return Err(e),
        }
    }
    Ok((left, options))
}

fn parse_tcp_header(input: &[u8]) -> IResult<&[u8], TcpHeader> {
    match parse_tcp(input) {
        Ok((left, mut header)) => {
            if header.hlen > 5 {
                let options_length = ((header.hlen - 5) * 4) as usize;
                if options_length < left.len() {
                    if let Ok((_, options)) = parse_tcp_options(&left[0..options_length]) {
                        header.options = Some(options);
                        return Ok((&left[options_length..], header));
                    }
                    return Ok((&left[options_length..], header));
                } else {
                    return Err(Err::Incomplete(Needed::new(options_length - left.len())));
                }
            } else {
                Ok((left, header))
            }
        }
        e => e,
    }
}

impl std::fmt::Display for TcpHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut flags_str = String::new();

        flags_str.push_str(if self.flag_urg { "U" } else { "" });
        flags_str.push_str(if self.flag_ack { "A" } else { "" });
        flags_str.push_str(if self.flag_psh { "P" } else { "" });
        flags_str.push_str(if self.flag_rst { "R" } else { "" });
        flags_str.push_str(if self.flag_syn { "S" } else { "" });
        flags_str.push_str(if self.flag_fin { "F" } else { "" });

        writedoc!(
            f,
            r#"
            ##[TCP]##
                sport     = {sport}
                dport     = {dport}
                seq       = {seq}
                ack       = {ack}
                dataofs   = {dataofs}
                reserved  = {reserved}
                flags     = {flags}
                window    = {window}
                checksum  = {checksum}
                urgptr    = {urgptr}
                options   = {options}
            "#,
            sport = self.source_port,
            dport = self.dest_port,
            seq = self.sequence_no,
            ack = self.ack_no,
            dataofs = self.hlen,
            reserved = self.reserved,
            flags = flags_str,
            window = self.window,
            checksum = self.checksum,
            urgptr = self.urgent_pointer,
            options = "[]",
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EMPTY_SLICE: &'static [u8] = &[];

    #[test]
    fn test_tcp_parse() {
        let bytes = [
            0xc2, 0x1f, /* Source port */
            0x00, 0x50, /* Dest port */
            0x0f, 0xd8, 0x7f, 0x4c, /* Seq no */
            0xeb, 0x2f, 0x05, 0xc8, /* Ack no */
            0x50, 0x18, 0x01, 0x00, /* Window + header_len + flags + reserved */
            0x7c, 0x29, /* Checksum */
            0x00, 0x00, /* Urgent pointer */
        ];
        let expections = TcpHeader {
            source_port: 49695,
            dest_port: 80,
            sequence_no: 0x0fd87f4c,
            ack_no: 0xeb2f05c8,
            hlen: 5,
            reserved: 0,
            flag_urg: false,
            flag_ack: true,
            flag_psh: true,
            flag_rst: false,
            flag_syn: false,
            flag_fin: false,
            window: 256,
            checksum: 0x7c29,
            urgent_pointer: 0,
            options: None,
        };
        assert_eq!(parse_tcp_header(&bytes), Ok((EMPTY_SLICE, expections)));
    }
}

fn main() {
    let bytes = [
        0xc2, 0x1f, /* Source port */
        0x00, 0x50, /* Dest port */
        0x0f, 0xd8, 0x7f, 0x4c, /* Seq no */
        0xeb, 0x2f, 0x05, 0xc8, /* Ack no */
        0x50, 0x18, 0x01, 0x00, /* Window + header_len + flags + reserved */
        0x7c, 0x29, /* Checksum */
        0x00, 0x00, /* Urgent pointer */
    ];

    let (_, header) = parse_tcp_header(&bytes).unwrap();

    println!("{}", header);
}
