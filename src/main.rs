mod crypto;
use std::fs;
use itertools::iproduct;

fn challange1() {
    assert_eq!(crypto::hex_to_base64("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"),
	      "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t" )
}

fn challange2() {
    let hex_arr1 = crypto::decode_hex_str("1c0111001f010100061a024b53535009181c");
    let hex_arr2 = crypto::decode_hex_str("686974207468652062756c6c277320657965");
    let result = crypto::xor_byte_vecs(hex_arr1, hex_arr2);
    assert_eq!(result, crypto::decode_hex_str("746865206b696420646f6e277420706c6179"))

}

fn challange3() {
    let hex_arr = crypto::decode_hex_str("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");
    let mut max_score = 0.0;
    let mut max_score_byte = "0".as_bytes()[0];
    let mut result = "0".as_bytes().to_vec(); 
    for char_num in 0..255 {
	let test_byte = u8::try_from(char_num).unwrap();
	let xor_array = crypto::xor_byte_vec_vs_byte(hex_arr.clone(), test_byte);
 	let score = crypto::eng_letter_freq_score(&xor_array);
	if score > max_score {
	    max_score = score;
	    max_score_byte = test_byte;
	    result = xor_array;
	}
    }
    println!("{}", crypto::bytes_array_to_str(&result));
    println!("key byte {:?}", max_score_byte);
}

fn challange4() {
    let c4_input_file = fs::read_to_string("resources/4.txt").unwrap();
    let cypher_hex_strs = c4_input_file.split_whitespace();
    let mut max_score = 0.0;
    let mut max_score_byte = "0".as_bytes()[0];
    let mut result = "0".as_bytes().to_vec(); 
    for hex_str in cypher_hex_strs {
	let hex_arr = crypto::decode_hex_str(hex_str);
	for char_num in 0..255 {
	    let test_byte = u8::try_from(char_num).unwrap();
	    let xor_array = crypto::xor_byte_vec_vs_byte(hex_arr.clone(), test_byte);
 	    let score = crypto::eng_letter_freq_score(&xor_array);
	    if score > max_score {
		max_score = score;
		max_score_byte = test_byte;
		result = xor_array;
	    }
	}
    }
    println!("{}", crypto::bytes_array_to_str(&result));
    println!("key byte {:?}", max_score_byte);
}

fn challange5() {
    let plaintext = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
    let key = "ICE";
    let cypher = crypto::repeating_key_xor_encrypt(plaintext.as_bytes().to_vec(), key.as_bytes().to_vec());
    assert_eq!(
	"0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f",
	crypto::encode_hex(&cypher)	
    );
}

fn challange6() {
    let mut input_file = fs::read_to_string("resources/6.txt").unwrap();
    input_file.retain(|c| !c.is_whitespace());
    let input_bytes_vec: Vec<u8> = crypto::decode_b64_str(&input_file);

    
    let mut lowest_dist = 999999999.99;
    let mut lowest_keysize = 0;
    for keysize in 2..41 {
	let chunks = input_bytes_vec
	    .chunks_exact(keysize)
	    .collect::<Vec<&[u8]>>();
	
	let chunks2 = chunks.clone();
	
	let distances: Vec<u32> = iproduct!(chunks, chunks2)
	    .map(|x| crypto::hamming_distance(x.0.to_vec(), x.1.to_vec()))
	    .collect();

	let n_distances = distances.len();

	let sum_dist: u32 = distances
	    .iter()
	    .sum();

	let mean_dist = sum_dist as f64 / n_distances as f64;
	let dist = mean_dist / keysize as f64;

	if dist < lowest_dist {
	    lowest_dist = dist;
	    lowest_keysize = keysize;
	}
    }

    let mut key_bytes = vec![];
    
    for key_byte_idx in 0..lowest_keysize {
	let mut max_score = 0.0;
	let mut max_score_byte = u8::try_from(0).unwrap();
	for char_num in 0..256 {
	    let cypher_nth_bytes = input_bytes_vec
		.iter()
		.skip(key_byte_idx)
		.step_by(lowest_keysize)
		.map(|x| *x)
		.collect::<Vec<u8>>();
	    
	    let test_byte = u8::try_from(char_num).unwrap();
	    let xor_array = crypto::xor_byte_vec_vs_byte(cypher_nth_bytes, test_byte);
 	    let score = crypto::eng_letter_freq_score(&xor_array);
	    if score > max_score {
		max_score = score;
		max_score_byte = test_byte;
	    }
	}
	key_bytes.push(max_score_byte);
    } 
    println!("keyword_bytes: {:?}", crypto::bytes_array_to_str(&key_bytes));
}
 
pub(crate) fn main() {
    challange1();
    challange2();
    challange3(); 
    challange4();
    challange5();
    challange6();
}

