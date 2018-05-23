#![no_main]
#[macro_use]
extern crate libfuzzer_sys;

fuzz_target!(|u8_data: &[u8]| {
    if u8_data.len() % 4 != 0 {
        return;
    }

    let u8_ptr = u8_data.as_ptr();
    let u32_ptr = u8_ptr as *const u32;
    let u32_len = u8_data.len() / 4;
    let u32_data: &[u32] = unsafe { std::slice::from_raw_parts(u32_ptr, u32_len) };
    let char_data: Vec<char> = u32_data
        .iter()
        .filter_map(|c| std::char::from_u32(*c))
        .collect();
    let string: String = char_data.iter().collect();
    let char_data_new: Vec<char> = string.chars().collect();
    assert_eq!(char_data, char_data_new);
});
