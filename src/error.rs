pub enum Chip8Error {
    OutOfBoundsAccess,
    StackUnderflow,
    StackOverflow,
}

pub type Result<T> = std::result::Result<T, Chip8Error>;
