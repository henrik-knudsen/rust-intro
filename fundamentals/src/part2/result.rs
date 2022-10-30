use std::num::ParseIntError;

fn try_parse_number(x: &str) -> Result<usize, ParseIntError> {
    unimplemented!()
}

/// Different types of version for some API.
#[derive(Debug, PartialEq, Eq)]
enum Version {
    Version1,
    Version2,
}

/// Error types
#[derive(Debug, PartialEq, Eq)]
enum VersionParseError {
    InvalidVersion,
    InvalidHeaderLength,
}

/// Parses API version from header (bytes)
fn parse_version(header: &[u8]) -> Result<Version, VersionParseError> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_value_or_zero() {
        assert_eq!(try_parse_number("3"), Ok(3));
        assert_eq!(try_parse_number("a").is_err(), true);
    }

    #[test]
    fn test_parse_version() {
        assert_eq!(parse_version(&[1, 255, 124, 100]), Ok(Version::Version1));
        assert_eq!(parse_version(&[2, 2, 3, 4]), Ok(Version::Version2));
        assert_eq!(
            parse_version(&[3, 255, 124, 100]),
            Err(VersionParseError::InvalidVersion)
        );
        assert_eq!(
            parse_version(&[]),
            Err(VersionParseError::InvalidHeaderLength)
        );
    }
}
