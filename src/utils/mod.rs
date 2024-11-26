pub trait To1DIndex {
    fn to_1d_idx(&self) -> usize;
}

impl To1DIndex for (usize, usize, usize, usize) {
    fn to_1d_idx(&self) -> usize {
        let (x, y, z, size) = self;
        x + y * size + z * size.pow(2)
    }
}
