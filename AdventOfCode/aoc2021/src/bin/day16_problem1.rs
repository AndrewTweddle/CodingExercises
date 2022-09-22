use std::fs;
use std::time::Instant;

#[derive(PartialEq, Eq, Clone, Debug)]
enum LengthMode {
    NumberOfBits,
    NumberOfPackets,
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct OperatorData {
    #[allow(dead_code)]
    length_mode: LengthMode,
    #[allow(dead_code)]
    length: usize,
    sub_packets: Vec<Packet>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
enum Data {
    Literal(String),
    Operator(OperatorData),
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct Packet {
    version: u8,
    #[allow(dead_code)]
    type_id: u8,
    data: Data,
}

#[allow(dead_code)]
impl Packet {
    fn parse_literal_data_and_remaining_str(data_to_parse: &str) -> (Data, &str) {
        fn recursively_parse_data_string_and_rem_str(data_str: &str) -> (String, &str) {
            let mut literal_data = data_str[1..5].to_string();
            if data_str.starts_with('1') {
                let (rem_literal_string, remaining_str) =
                    recursively_parse_data_string_and_rem_str(&data_str[5..]);
                literal_data += rem_literal_string.as_str();
                (literal_data, remaining_str)
            } else {
                (literal_data, &data_str[5..])
            }
        }

        let (data_string, remaining_str) = recursively_parse_data_string_and_rem_str(data_to_parse);
        (Data::Literal(data_string), remaining_str)
    }

    fn parse_operator_with_number_of_packets(data_to_parse: &str) -> (Data, &str) {
        let length = usize::from_str_radix(&data_to_parse[0..11], 2).unwrap();
        let mut sub_packets: Vec<Packet> = Vec::with_capacity(length);
        let mut rem_str = &data_to_parse[11..];
        for _ in 0..length {
            let (sub_packet, new_rem_str) = Self::parse(rem_str);
            sub_packets.push(sub_packet);
            rem_str = new_rem_str;
        }
        let operator_data = OperatorData {
            length_mode: LengthMode::NumberOfPackets,
            length,
            sub_packets,
        };
        (Data::Operator(operator_data), rem_str)
    }

    fn parse_operator_with_number_of_bits(data_to_parse: &str) -> (Data, &str) {
        let length = usize::from_str_radix(&data_to_parse[0..15], 2).unwrap();
        let mut sub_packets: Vec<Packet> = Vec::new();
        let pos_of_remainder = 15 + length;
        let mut rem_to_parse_str = &data_to_parse[15..pos_of_remainder];
        while rem_to_parse_str.len() > 7 {
            let (sub_packet, new_rem_to_parse_str) = Self::parse(rem_to_parse_str);
            sub_packets.push(sub_packet);
            rem_to_parse_str = new_rem_to_parse_str;
        }
        let operator_data = OperatorData {
            length_mode: LengthMode::NumberOfBits,
            length,
            sub_packets,
        };
        (
            Data::Operator(operator_data),
            &data_to_parse[pos_of_remainder..],
        )
    }

    fn parse(transmission: &str) -> (Self, &str) {
        let version = u8::from_str_radix(&transmission[0..3], 2).expect("Error parsing version");
        let type_id = u8::from_str_radix(&transmission[3..6], 2).expect("Error parsing type id");
        let (data, remaining_str) = if type_id == 4 {
            Self::parse_literal_data_and_remaining_str(&transmission[6..])
        } else if transmission.chars().nth(6).unwrap() == '1' {
            Self::parse_operator_with_number_of_packets(&transmission[7..])
        } else {
            Self::parse_operator_with_number_of_bits(&transmission[7..])
        };
        let packet = Packet {
            version,
            type_id,
            data,
        };
        (packet, remaining_str)
    }

    fn get_sum_of_all_version_numbers(&self) -> usize {
        match &self.data {
            Data::Literal(_) => self.version as usize,
            Data::Operator(operator_data) => {
                let sub_packet_version_sum: usize = operator_data
                    .sub_packets
                    .iter()
                    .map(|sub_packet| sub_packet.get_sum_of_all_version_numbers())
                    .sum();
                sub_packet_version_sum + self.version as usize
            }
        }
    }

    fn get_literal_data(&self) -> Option<&String> {
        match &self.data {
            Data::Literal(data_string) => Some(data_string),
            Data::Operator(_) => None,
        }
    }

    fn get_operator_data(&self) -> Option<&OperatorData> {
        match &self.data {
            Data::Literal(_) => None,
            Data::Operator(operator_data) => Some(operator_data),
        }
    }
}

fn main() {
    let start_time = Instant::now();
    let contents = fs::read_to_string("data/day16_input.txt").unwrap();
    let bytes = parse_hex_chars_as_nibbles(contents.trim());
    let bits = convert_nibbles_to_bitstring(bytes.as_slice());
    let (packet, _remaining_str) = Packet::parse(&bits);
    let part1_answer = packet.get_sum_of_all_version_numbers();
    let duration = start_time.elapsed();
    println!("Part 1 answer: {}", part1_answer);
    println!("Duration: {:?}", duration);
}

fn parse_hex_chars_as_nibbles(hex_str: &str) -> Vec<u8> {
    hex_str
        .chars()
        .map(convert_hex_char_to_u8)
        .collect::<Vec<u8>>()
}

fn convert_hex_char_to_u8(hex_char: char) -> u8 {
    if !hex_char.is_ascii_hexdigit() {
        panic!("Unrecognized hex character: {}", hex_char);
    }
    if hex_char >= 'A' {
        hex_char as u8 - b'A' + 10
    } else {
        hex_char as u8 - b'0'
    }
}

fn convert_nibbles_to_bitstring(nibbles: &[u8]) -> String {
    let bits: String = nibbles
        .iter()
        .map(|nibble| format!("{:04b}", nibble))
        .collect();
    bits
}

#[cfg(test)]
mod tests {
    use crate::{
        convert_nibbles_to_bitstring, parse_hex_chars_as_nibbles, Data, LengthMode, Packet,
    };

    #[test]
    fn test_hex_conversion() {
        let nibbles = parse_hex_chars_as_nibbles("D2FE28");
        let bit_string: String = nibbles
            .iter()
            .map(|nibble| format!("{:04b}", nibble))
            .collect();
        assert_eq!(bit_string, "110100101111111000101000");
    }

    #[test]
    fn test_bitstring_conversion() {
        let nibbles = parse_hex_chars_as_nibbles("D2FE28");
        let bit_string = convert_nibbles_to_bitstring(&nibbles);
        assert_eq!(bit_string, "110100101111111000101000");
    }

    #[test]
    fn test_parsing_literal() {
        let nibbles = parse_hex_chars_as_nibbles("D2FE28");
        let bit_string = convert_nibbles_to_bitstring(&nibbles);
        let (packet, rem_str) = Packet::parse(bit_string.as_str());
        assert_eq!(packet.version, 6);
        assert_eq!(packet.type_id, 4);
        assert_eq!(packet.data, Data::Literal("011111100101".to_string()));
        assert_eq!(rem_str, "000");
    }

    #[test]
    fn test_parsing_sub_literal_10() {
        let literal_bits = "11010001010";
        let (packet, rem_str) = Packet::parse(literal_bits);
        assert_eq!(packet.data, Data::Literal("1010".to_string()));
        assert_eq!(rem_str, "");
    }

    #[test]
    fn test_parsing_sub_literal_20() {
        let literal_bits = "01010010001001000000000";
        let (packet, rem_str) = Packet::parse(literal_bits);
        assert_eq!(packet.data, Data::Literal("00010100".to_string()));
        assert_eq!(rem_str, "0000000");
    }

    #[test]
    fn test_parsing_operator_with_length_type_id_0() {
        let nibbles = parse_hex_chars_as_nibbles("38006F45291200");
        let bit_string = convert_nibbles_to_bitstring(&nibbles);
        let (packet, rem_str) = Packet::parse(bit_string.as_str());
        assert_eq!(packet.version, 1);
        assert_eq!(packet.type_id, 6);
        match packet.data {
            Data::Literal(_) => panic!("Expected Operator, not Literal"),
            Data::Operator(operator_data) => {
                assert_eq!(operator_data.length, 27);
                assert_eq!(operator_data.length_mode, LengthMode::NumberOfBits);
                assert_eq!(operator_data.sub_packets.len(), 2);
                assert_eq!(
                    operator_data.sub_packets[0].data,
                    Data::Literal("1010".to_string())
                );
                assert_eq!(
                    operator_data.sub_packets[1].data,
                    Data::Literal("00010100".to_string())
                );
            }
        }
        assert_eq!(rem_str, "0000000");
    }

    #[test]
    fn test_parsing_operator_with_length_type_id_1() {
        let nibbles = parse_hex_chars_as_nibbles("EE00D40C823060");
        let bit_string = convert_nibbles_to_bitstring(&nibbles);
        let (packet, rem_str) = Packet::parse(bit_string.as_str());
        assert_eq!(packet.version, 7);
        assert_eq!(packet.type_id, 3);
        match packet.data {
            Data::Literal(_) => panic!("Expected Operator, not Literal"),
            Data::Operator(operator_data) => {
                assert_eq!(operator_data.length, 3);
                assert_eq!(operator_data.length_mode, LengthMode::NumberOfPackets);
                assert_eq!(operator_data.sub_packets.len(), 3);
                assert_eq!(
                    operator_data.sub_packets[0].data,
                    Data::Literal("0001".to_string())
                );
                assert_eq!(
                    operator_data.sub_packets[1].data,
                    Data::Literal("0010".to_string())
                );
                assert_eq!(
                    operator_data.sub_packets[2].data,
                    Data::Literal("0011".to_string())
                );
            }
        }
        assert_eq!(rem_str, "00000");
    }

    #[test]
    fn test_nested_operator_packet_with_version_sum_16() {
        let nibbles = parse_hex_chars_as_nibbles("8A004A801A8002F478");
        let bit_string = convert_nibbles_to_bitstring(&nibbles);
        let (packet1, _) = Packet::parse(bit_string.as_str());
        assert_eq!(packet1.version, 4);
        let operator_data_1 = packet1.get_operator_data().unwrap();
        let packet2 = &operator_data_1.sub_packets[0];
        assert_eq!(packet2.version, 1);
        let packet3 = &packet2.get_operator_data().unwrap().sub_packets[0];
        assert_eq!(packet3.version, 5);
        let packet4 = &packet3.get_operator_data().unwrap().sub_packets[0];
        assert_eq!(packet4.version, 6);
        assert!(packet4.get_literal_data() != None);
        assert_eq!(packet1.get_sum_of_all_version_numbers(), 16);
    }

    #[test]
    fn test_nested_operator_packet_with_multiple_sub_packets_and_version_sum_12() {
        let nibbles = parse_hex_chars_as_nibbles("620080001611562C8802118E34");
        let bit_string = convert_nibbles_to_bitstring(&nibbles);
        let (packet, _) = Packet::parse(bit_string.as_str());
        assert_eq!(packet.version, 3);
        assert_eq!(packet.get_sum_of_all_version_numbers(), 12);
    }

    #[test]
    fn test_nested_operator_packet_with_multiple_sub_packets_and_version_sum_23() {
        let nibbles = parse_hex_chars_as_nibbles("C0015000016115A2E0802F182340");
        let bit_string = convert_nibbles_to_bitstring(&nibbles);
        let (packet, _) = Packet::parse(bit_string.as_str());
        assert_eq!(packet.get_sum_of_all_version_numbers(), 23);
    }

    #[test]
    fn test_nested_operator_packet_with_version_sum_31() {
        let nibbles = parse_hex_chars_as_nibbles("A0016C880162017C3686B18A3D4780");
        let bit_string = convert_nibbles_to_bitstring(&nibbles);
        let (packet, _) = Packet::parse(bit_string.as_str());
        let lowest_operator_packet = packet.get_operator_data().unwrap().sub_packets[0]
            .get_operator_data()
            .unwrap()
            .sub_packets[0]
            .get_operator_data()
            .unwrap();
        assert_eq!(lowest_operator_packet.sub_packets.len(), 5);
        assert_eq!(packet.get_sum_of_all_version_numbers(), 31);
    }
}
