#![no_main]
use libfuzzer_sys::fuzz_target;

use waitmap::WaitMap;

fuzz_target!(|data: &[u8]| {
    // fuzzed code goes here
    let mapping = WaitMap::new();
    let mut count = 0;
    for chunk in data.chunks(2) {
        if chunk.len() == 2 {
            let key = chunk[0];
            let val = chunk[1];
            if mapping.get(&key).is_none() {
                count = count + 1;
            }
            mapping.insert(key, val);
        }
    }
    let _ = count;
});
