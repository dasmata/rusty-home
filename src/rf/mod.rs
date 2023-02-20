mod sx126x;

pub fn init_rf() -> ! {
    let mut adapter: sx126x::Sx126x = sx126x::Sx126x::new(
        true,
        868,
        "/dev/ttyS0",
        sx126x::POWER_22 as u8,
    );

    adapter.init();
    adapter.read()
}
