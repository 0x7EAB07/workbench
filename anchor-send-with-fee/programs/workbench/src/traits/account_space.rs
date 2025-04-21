pub trait AccountSpace {
    fn account_space() -> usize;
}

impl<T: anchor_lang::Space> AccountSpace for T {
    fn account_space() -> usize {
        8 + T::INIT_SPACE
    }
}
