use libc;
use optee_utee_sys as raw;
use std::fmt;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, PartialEq)]
pub struct Error {
    code: u32,
}

#[derive(PartialEq, Eq, Clone, Debug, Copy)]
pub enum ErrorCode {
    CorruptObject = 0xF0100001,
    CorruptObject2 = 0xF0100002,
    StorageNotAvailable = 0xF0100003,
    StorageNotAvailable2 = 0xF0100004,
    Generic = 0xFFFF0000,
    AccessDenied = 0xFFFF0001,
    Cancel = 0xFFFF0002,
    AccessConflict = 0xFFFF0003,
    ExcessData = 0xFFFF0004,
    BadFormat = 0xFFFF0005,
    BadParameters = 0xFFFF0006,
    BadState = 0xFFFF0007,
    ItemNotFound = 0xFFFF0008,
    NotImplemented = 0xFFFF0009,
    NotSupported = 0xFFFF000A,
    NoData = 0xFFFF000B,
    OutOfMEmory = 0xFFFF000C,
    Busy = 0xFFFF000D,
    Communication = 0xFFFF000E,
    Security = 0xFFFF000F,
    ShortBuffer = 0xFFFF0010,
    ExternalCancel = 0xFFFF0011,
    Overflow = 0xFFFF300F,
    TargetDead = 0xFFFF3024,
    StorageNoSpace = 0xFFFF3041,
    MacInvalid = 0xFFFF3071,
    SignatureInvalid = 0xFFFF3072,
    TimeNotSet = 0xFFFF5000,
    TimeNeedsReset = 0xFFFF5001,
    Unknown,
}

impl ErrorCode {
    pub fn as_str(&self) -> &'static str {
        match *self {
            ErrorCode::CorruptObject => "Object corruption.",
            ErrorCode::CorruptObject2 => "Persistent object corruption.",
            ErrorCode::StorageNotAvailable => "Object storage is not available.",
            ErrorCode::StorageNotAvailable2 => "Persistent object storage is not available.",
            ErrorCode::Generic => "Non-specific cause.",
            ErrorCode::AccessDenied => "Access privileges are not sufficient.",
            ErrorCode::Cancel => "The operation was canceled.",
            ErrorCode::AccessConflict => "Concurrent accesses caused conflict.",
            ErrorCode::ExcessData => "Too much data for the requested operation was passed.",
            ErrorCode::BadFormat => "Input data was of invalid format.",
            ErrorCode::BadParameters => "Input parameters were invalid.",
            ErrorCode::BadState => "Operation is not valid in the current state.",
            ErrorCode::ItemNotFound => "The requested data item is not found.",
            ErrorCode::NotImplemented => {
                "The requested operation should exist but is not yet implemented."
            }
            ErrorCode::NotSupported => {
                "The requested operation is valid but is not supported in this implementation."
            }
            ErrorCode::NoData => "Expected data was missing.",
            ErrorCode::OutOfMEmory => "System ran out of resources.",
            ErrorCode::Busy => "The system is busy working on something else.",
            ErrorCode::Communication => "Communication with a remote party failed.",
            ErrorCode::Security => "A security fault was detected.",
            ErrorCode::ShortBuffer => "The supplied buffer is too short for the generated output.",
            ErrorCode::ExternalCancel => "Undocumented.",
            ErrorCode::Overflow => "Data overflow",
            ErrorCode::TargetDead => "Trusted Application has panicked during the operation.",
            ErrorCode::StorageNoSpace => "Storage no space",
            ErrorCode::MacInvalid => "Mac is invalid.",
            ErrorCode::SignatureInvalid => "Signature is invalid.",
            ErrorCode::TimeNotSet => "Time is not set.",
            ErrorCode::TimeNeedsReset => "Time needs to be reset.",
            ErrorCode::Unknown => "Unknown error.",
        }
    }
}

impl Error {
    pub fn from_raw_error(code: u32) -> Error {
        Error { code }
    }

    pub fn code(&self) -> ErrorCode {
        match self.code as libc::uint32_t {
            raw::TEE_ERROR_CORRUPT_OBJECT => ErrorCode::CorruptObject,
            raw::TEE_ERROR_CORRUPT_OBJECT_2 => ErrorCode::CorruptObject2,
            raw::TEE_ERROR_STORAGE_NOT_AVAILABLE => ErrorCode::StorageNotAvailable,
            raw::TEE_ERROR_STORAGE_NOT_AVAILABLE_2 => ErrorCode::StorageNotAvailable2,
            raw::TEE_ERROR_GENERIC => ErrorCode::Generic,
            raw::TEE_ERROR_ACCESS_DENIED => ErrorCode::AccessDenied,
            raw::TEE_ERROR_CANCEL => ErrorCode::Cancel,
            raw::TEE_ERROR_ACCESS_CONFLICT => ErrorCode::AccessConflict,
            raw::TEE_ERROR_EXCESS_DATA => ErrorCode::ExcessData,
            raw::TEE_ERROR_BAD_FORMAT => ErrorCode::BadFormat,
            raw::TEE_ERROR_BAD_PARAMETERS => ErrorCode::BadParameters,
            raw::TEE_ERROR_BAD_STATE => ErrorCode::BadState,
            raw::TEE_ERROR_ITEM_NOT_FOUND => ErrorCode::ItemNotFound,
            raw::TEE_ERROR_NOT_IMPLEMENTED => ErrorCode::NotImplemented,
            raw::TEE_ERROR_NOT_SUPPORTED => ErrorCode::NotSupported,
            raw::TEE_ERROR_NO_DATA => ErrorCode::NoData,
            raw::TEE_ERROR_OUT_OF_MEMORY => ErrorCode::OutOfMEmory,
            raw::TEE_ERROR_BUSY => ErrorCode::Busy,
            raw::TEE_ERROR_COMMUNICATION => ErrorCode::Communication,
            raw::TEE_ERROR_SECURITY => ErrorCode::Security,
            raw::TEE_ERROR_SHORT_BUFFER => ErrorCode::ShortBuffer,
            raw::TEE_ERROR_EXTERNAL_CANCEL => ErrorCode::ExternalCancel,
            raw::TEE_ERROR_OVERFLOW => ErrorCode::Overflow,
            raw::TEE_ERROR_TARGET_DEAD => ErrorCode::TargetDead,
            raw::TEE_ERROR_STORAGE_NO_SPACE => ErrorCode::StorageNoSpace,
            raw::TEE_ERROR_MAC_INVALID => ErrorCode::MacInvalid,
            raw::TEE_ERROR_SIGNATURE_INVALID => ErrorCode::SignatureInvalid,
            raw::TEE_ERROR_TIME_NOT_SET => ErrorCode::TimeNotSet,
            raw::TEE_ERROR_TIME_NEEDS_RESET => ErrorCode::TimeNeedsReset,
            _ => ErrorCode::Unknown,
        }
    }

    pub fn raw_code(&self) -> libc::uint32_t {
        self.code as libc::uint32_t
    }

    pub fn message(&self) -> &str {
        self.code().as_str()
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        self.message()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.message())
    }
}
