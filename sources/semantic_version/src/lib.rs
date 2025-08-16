#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Version {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // unimplemented!();
    }
}