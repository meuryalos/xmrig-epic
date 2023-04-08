use bigint::uint::U256;

#[cxx::bridge]
mod ffi {
    extern "Rust" {
        fn check_diff(data: &[u8], diff: u64) -> bool;
    }
}

fn check_diff(data: &[u8], diff: u64) -> bool {
    let num_hash = U256::from(data);
    let result = U256::max_value() / num_hash;
    result.low_u64() >= diff
}

