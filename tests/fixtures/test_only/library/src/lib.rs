pub fn integration_only() {}

#[cfg(test)]
pub fn test_compiled_only() {}

#[cfg(test)]
mod tests {
    #[test]
    fn uses_test_compiled_declaration() {
        super::test_compiled_only();
    }
}
