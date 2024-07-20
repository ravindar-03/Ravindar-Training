#![no_std]
#![no_main]

use aya_ebpf::{bindings::xdp_action, macros::xdp, programs::XdpContext};
use aya_log_ebpf::info;

#[xdp]
pub fn block_packet(ctx: XdpContext) -> u32 {
    match unsafe { try_block_packet(ctx) } {
        Ok(_) => xdp_action::XDP_DROP,
        Err(_) => xdp_action::XDP_ABORTED,
    }
}

unsafe fn try_block_packet(ctx: XdpContext) -> Result<u32, u32> {
    info!(&ctx, "received a packet (blocking)");
    Ok(xdp_action::XDP_DROP)  // Block the packet here
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { core::hint::unreachable_unchecked() }
}
