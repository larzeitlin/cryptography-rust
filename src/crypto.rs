use hex;
use base64::{Engine as _, engine::general_purpose};
use std::collections::HashMap;
use std::str;

pub fn hex_to_base64(hex_string: &str) -> String {
    let hex_bytes = hex::decode(hex_string);
    general_purpose::STANDARD.encode(hex_bytes.unwrap())
}

pub fn print_hex(bytes_in: &[u8]) {
    println!("{}", hex::encode(bytes_in));
}

pub fn encode_hex(bytes_in: &[u8]) -> String {
    hex::encode(bytes_in)
}


pub fn print_b64(bytes_in: &[u8]) {
    println!("{}", general_purpose::STANDARD.encode(bytes_in));
}

pub fn decode_hex_str(str_in: &str) -> Vec<u8> {
    hex::decode(str_in).unwrap()
}

pub fn decode_b64_str(str_in: &str) -> Vec<u8> {
    general_purpose::STANDARD.decode(str_in).unwrap()
}

pub fn bytes_array_to_str(bytes_in: &[u8]) -> String {
    String::from_utf8_lossy(bytes_in).to_string()
}

pub fn xor_byte_arrays(a: Vec<u8>, b: Vec<u8>) -> Vec<u8> {
    let result = a
	.iter()
	.zip(b.iter())
	.map(|(&x1, &x2)| x1 ^ x2)
	.collect();
    return result
}

pub fn xor_byte_array_vs_byte(v: Vec<u8>, byte: u8) -> Vec<u8> {
    let result: Vec<u8> = v
	.iter()
	.map(|x| x ^ byte)
	.collect();
    return result
}

pub fn repeating_key_xor_encrypt(v: Vec<u8>, key: Vec<u8>) -> Vec<u8> {
    let key_cycle = key.iter().cycle().take(v.len());
    let result = v
	.iter()
	.zip(key_cycle)
	.map(|(x1, x2)| x1 ^ x2)
	.collect();
    return result
}

pub fn eng_letter_freq_score(input_bytes: &[u8]) -> f64 {
    let letter_scores = HashMap::from([
	("#".as_bytes()[0], 7.881711E-6),
	("%".as_bytes()[0], 7.881711E-6),
	("﻿".as_bytes()[0], 7.881711E-6),
	("$".as_bytes()[0], 1.5763422E-5),
	("[".as_bytes()[0], 1.5763422E-5),
	("]".as_bytes()[0], 1.5763422E-5),
	("è".as_bytes()[0], 3.1526844E-5),
	("'".as_bytes()[0], 5.517198E-5),
	("4".as_bytes()[0], 7.09354E-5),
	("6".as_bytes()[0], 7.09354E-5),
	("5".as_bytes()[0], 7.881711E-5),
	("/".as_bytes()[0], 8.669883E-5),
	("3".as_bytes()[0], 8.669883E-5),
	("7".as_bytes()[0], 8.669883E-5),
	("2".as_bytes()[0], 9.458054E-5),
	("8".as_bytes()[0], 1.1034396E-4),
	("9".as_bytes()[0], 1.1822567E-4),
	("0".as_bytes()[0], 1.6551594E-4),
	("\"".as_bytes()[0], 1.7339765E-4),
	("Q".as_bytes()[0], 2.2856962E-4),
	("é".as_bytes()[0], 4.09849E-4),
	("K".as_bytes()[0], 4.1773068E-4),
	("X".as_bytes()[0], 4.7290267E-4),
	("1".as_bytes()[0], 4.807844E-4),
	("*".as_bytes()[0], 6.935906E-4),
	("U".as_bytes()[0], 7.172357E-4),
	("‘".as_bytes()[0], 7.802894E-4),
	("V".as_bytes()[0], 9.063968E-4),
	("(".as_bytes()[0], 0.0011664933),
	(")".as_bytes()[0], 0.0011664933),
	("_".as_bytes()[0], 0.0014344715),
	("z".as_bytes()[0], 0.001694568),
	(":".as_bytes()[0], 0.00207289),
	("J".as_bytes()[0], 0.0022068792),
	("R".as_bytes()[0], 0.0024433306),
	("O".as_bytes()[0], 0.0025852013),
	("G".as_bytes()[0], 0.0030344587),
	("N".as_bytes()[0], 0.0032315017),
	("j".as_bytes()[0], 0.0034206626),
	("F".as_bytes()[0], 0.0034364262),
	("Y".as_bytes()[0], 0.0034521895),
	("E".as_bytes()[0], 0.0039408556),
	("B".as_bytes()[0], 0.0042167157),
	("P".as_bytes()[0], 0.0046029193),
	("W".as_bytes()[0], 0.004752672),
	("q".as_bytes()[0], 0.00502065),
	("x".as_bytes()[0], 0.0052334564),
	("C".as_bytes()[0], 0.006273842),
	("L".as_bytes()[0], 0.0065733474),
	("S".as_bytes()[0], 0.0071093035),
	("?".as_bytes()[0], 0.007196002),
	("!".as_bytes()[0], 0.0075270343),
	("H".as_bytes()[0], 0.0076846685),
	("A".as_bytes()[0], 0.007771367),
	("D".as_bytes()[0], 0.007771367),
	(";".as_bytes()[0], 0.008732936),
	("’".as_bytes()[0], 0.00916643),
	("M".as_bytes()[0], 0.012799899),
	("T".as_bytes()[0], 0.01404521),
	("-".as_bytes()[0], 0.019152557),
	("”".as_bytes()[0], 0.021808695),
	("I".as_bytes()[0], 0.022628393),
	("“".as_bytes()[0], 0.02279391),
	("k".as_bytes()[0], 0.037343547),
	("v".as_bytes()[0], 0.040094264),
	(".".as_bytes()[0], 0.053761154),
	("b".as_bytes()[0], 0.062147293),
	("p".as_bytes()[0], 0.0741669),
	("y".as_bytes()[0], 0.09272045),
	("g".as_bytes()[0], 0.09592043),
	("f".as_bytes()[0], 0.103431694),
	("c".as_bytes()[0], 0.10369968),
	(",".as_bytes()[0], 0.10459819),
	("w".as_bytes()[0], 0.10672625),
	("m".as_bytes()[0], 0.10771935),
	("\n".as_bytes()[0], 0.12835367),
	("\r".as_bytes()[0], 0.12835367),
	("u".as_bytes()[0], 0.13126202),
	("l".as_bytes()[0], 0.16717109),
	("d".as_bytes()[0], 0.21331851),
	("s".as_bytes()[0], 0.28914058),
	("r".as_bytes()[0], 0.29137897),
	("h".as_bytes()[0], 0.2999464),
	("i".as_bytes()[0], 0.30052966),
	("n".as_bytes()[0], 0.3309373),
	("o".as_bytes()[0], 0.36438727),
	("a".as_bytes()[0], 0.37227687),
	("t".as_bytes()[0], 0.41265488),
	("e".as_bytes()[0], 0.58621013),
	(" ".as_bytes()[0], 1.0)
    ]);
    

    let score: f64 = input_bytes
	.iter()
	.fold(0.0, |sum, x| sum + letter_scores.get(x).unwrap_or(&0.0));

    return score
}

