pub struct Ipv4Addr {
    inner: c::in_addr,
}

impl Ipv4Addr {
    pub const fn new(a: u8, b: u8, c: u8, d: u8) -> Ipv4Addr {
        // `s_addr` is stored as BE on all machine and the array is in BE order.
        // So the native endian conversion method is used so that it's never swapped.
        Ipv4Addr { inner: c::in_addr { s_addr: u32::from_ne_bytes([a, b, c, d]) } }
    }

    pub const fn octets(&self) -> [u8; 4] {
        // This returns the order we want because s_addr is stored in big-endian.
        self.inner.s_addr.to_ne_bytes()
    }
}
