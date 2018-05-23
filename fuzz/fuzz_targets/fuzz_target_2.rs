#![no_main]
#[macro_use]
extern crate libfuzzer_sys;

fuzz_target!{
    |u8_data: &[u8]| {
        if let Ok(s) = std::str::from_utf8(u8_data) {
            let char_data: Vec<char> = s.chars().collect();
            let string: String = char_data.iter().collect();
            assert_eq!(s, string);
        }
    }
}
