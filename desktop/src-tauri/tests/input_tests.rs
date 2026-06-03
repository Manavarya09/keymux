#[cfg(test)]
mod tests {
    use crate::input::enumerate_devices;

    #[test]
    fn enumerate_returns_ok() {
        let res = enumerate_devices();
        assert!(res.is_ok());
    }
}
