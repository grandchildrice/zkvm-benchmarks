#![cfg_attr(feature = "guest", no_std)]
#![no_main]

#[jolt::provable(stack_size = 1000_000, memory_size = 10000000, max_input_size = 10000000)]
fn transfer_eth_n_times(n: usize) -> bool {
    revm_utils::transfer_eth_n_times(n)
}
