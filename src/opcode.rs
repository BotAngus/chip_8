#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct Opcode(u16);

impl From<u16> for Opcode {
    fn from(value: u16) -> Self {
        Self(value)
    }
}

impl From<(u8, u8)> for Opcode {
    fn from(bytes: (u8, u8)) -> Self {
        Self {
            0: ((bytes.0 as u16) << 8) | bytes.1 as u16,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::Opcode;


    #[test]
    fn opcode_test() {
        let from: (u8, u8) = (0xFF, 0xFF);
        let result: Opcode = from.into();

        assert_eq!(result, Opcode::from(0xFFFF))
    }
}