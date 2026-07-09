//! HTTP-like response status codes.

/// Simplified HTTP status for API gateway exercises.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HttpStatus {
    Ok,
    Created,
    BadRequest,
    Unauthorized,
    Forbidden,
    NotFound,
    InternalServerError,
}

impl HttpStatus {
    /// Map a numeric status code to the enum, if recognized.
    #[must_use]
    pub const fn from_code(code: u16) -> Option<Self> {
        match code {
            200 => Some(Self::Ok),
            201 => Some(Self::Created),
            400 => Some(Self::BadRequest),
            401 => Some(Self::Unauthorized),
            403 => Some(Self::Forbidden),
            404 => Some(Self::NotFound),
            500 => Some(Self::InternalServerError),
            _ => None,
        }
    }

    /// Numeric code for this status.
    #[must_use]
    pub const fn code(self) -> u16 {
        match self {
            Self::Ok => 200,
            Self::Created => 201,
            Self::BadRequest => 400,
            Self::Unauthorized => 401,
            Self::Forbidden => 403,
            Self::NotFound => 404,
            Self::InternalServerError => 500,
        }
    }

    /// Whether the status is in the 2xx success class.
    #[must_use]
    pub const fn is_success(self) -> bool {
        matches!(self, Self::Ok | Self::Created)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_code_maps_common_values() {
        assert_eq!(HttpStatus::from_code(200), Some(HttpStatus::Ok));
        assert_eq!(HttpStatus::from_code(403), Some(HttpStatus::Forbidden));
        assert_eq!(HttpStatus::from_code(418), None);
    }

    #[test]
    fn round_trip_codes() {
        for code in [200u16, 201, 400, 401, 403, 404, 500] {
            let status = HttpStatus::from_code(code);
            assert!(status.is_some());
            if let Some(s) = status {
                assert_eq!(s.code(), code);
            }
        }
    }
}
