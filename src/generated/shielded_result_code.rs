#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ShieldedResultCode is an XDR Enum defined as:
///
/// ```text
/// enum ShieldedResultCode
/// {
///     // codes considered as "success" for the operation
///     SHIELDED_SUCCESS = 0,
///
///     // codes considered as "failure" for the operation
///     SHIELDED_MALFORMED = -1,             // invalid op body / bundle bounds
///     SHIELDED_GATE_CLOSED = -2,           // ledgerVersion < SHIELDED_V1
///     SHIELDED_VERIFY_FAILED = -3,         // halo2/signature verification failed
///     SHIELDED_NULLIFIER_DUPLICATE = -4,   // note already spent
///     SHIELDED_WITHDRAWAL_MISMATCH = -5,   // transparentOutput vs bundle delta
///     SHIELDED_INSUFFICIENT_BALANCE = -6,  // deposit: source cannot pay amount
///     SHIELDED_POOL_OVERFLOW = -7,         // withdrawal credit overflow
///     SHIELDED_VERIFIER_UNAVAILABLE = -8   // kernel FFI not loadable (fail-closed)
/// };
/// ```
///
// enum
#[cfg_attr(feature = "alloc", derive(Default))]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[repr(i32)]
pub enum ShieldedResultCode {
    #[cfg_attr(feature = "alloc", default)]
    Success = 0,
    Malformed = -1,
    GateClosed = -2,
    VerifyFailed = -3,
    NullifierDuplicate = -4,
    WithdrawalMismatch = -5,
    InsufficientBalance = -6,
    PoolOverflow = -7,
    VerifierUnavailable = -8,
}

impl ShieldedResultCode {
    const _VARIANTS: &[ShieldedResultCode] = &[
        ShieldedResultCode::Success,
        ShieldedResultCode::Malformed,
        ShieldedResultCode::GateClosed,
        ShieldedResultCode::VerifyFailed,
        ShieldedResultCode::NullifierDuplicate,
        ShieldedResultCode::WithdrawalMismatch,
        ShieldedResultCode::InsufficientBalance,
        ShieldedResultCode::PoolOverflow,
        ShieldedResultCode::VerifierUnavailable,
    ];
    pub const VARIANTS: [ShieldedResultCode; Self::_VARIANTS.len()] = {
        let mut arr = [Self::_VARIANTS[0]; Self::_VARIANTS.len()];
        let mut i = 1;
        while i < Self::_VARIANTS.len() {
            arr[i] = Self::_VARIANTS[i];
            i += 1;
        }
        arr
    };
    const _VARIANTS_STR: &[&str] = &[
        "Success",
        "Malformed",
        "GateClosed",
        "VerifyFailed",
        "NullifierDuplicate",
        "WithdrawalMismatch",
        "InsufficientBalance",
        "PoolOverflow",
        "VerifierUnavailable",
    ];
    pub const VARIANTS_STR: [&'static str; Self::_VARIANTS_STR.len()] = {
        let mut arr = [Self::_VARIANTS_STR[0]; Self::_VARIANTS_STR.len()];
        let mut i = 1;
        while i < Self::_VARIANTS_STR.len() {
            arr[i] = Self::_VARIANTS_STR[i];
            i += 1;
        }
        arr
    };

    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Success => "Success",
            Self::Malformed => "Malformed",
            Self::GateClosed => "GateClosed",
            Self::VerifyFailed => "VerifyFailed",
            Self::NullifierDuplicate => "NullifierDuplicate",
            Self::WithdrawalMismatch => "WithdrawalMismatch",
            Self::InsufficientBalance => "InsufficientBalance",
            Self::PoolOverflow => "PoolOverflow",
            Self::VerifierUnavailable => "VerifierUnavailable",
        }
    }

    #[must_use]
    pub const fn variants() -> [ShieldedResultCode; Self::_VARIANTS.len()] {
        Self::VARIANTS
    }
}

impl Name for ShieldedResultCode {
    #[must_use]
    fn name(&self) -> &'static str {
        Self::name(self)
    }
}

impl Variants<ShieldedResultCode> for ShieldedResultCode {
    fn variants() -> slice::Iter<'static, ShieldedResultCode> {
        Self::VARIANTS.iter()
    }
}

impl Enum for ShieldedResultCode {}

impl fmt::Display for ShieldedResultCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl TryFrom<i32> for ShieldedResultCode {
    type Error = Error;

    fn try_from(i: i32) -> Result<Self, Error> {
        let e = match i {
            0 => ShieldedResultCode::Success,
            -1 => ShieldedResultCode::Malformed,
            -2 => ShieldedResultCode::GateClosed,
            -3 => ShieldedResultCode::VerifyFailed,
            -4 => ShieldedResultCode::NullifierDuplicate,
            -5 => ShieldedResultCode::WithdrawalMismatch,
            -6 => ShieldedResultCode::InsufficientBalance,
            -7 => ShieldedResultCode::PoolOverflow,
            -8 => ShieldedResultCode::VerifierUnavailable,
            #[allow(unreachable_patterns)]
            _ => return Err(Error::Invalid),
        };
        Ok(e)
    }
}

impl From<ShieldedResultCode> for i32 {
    #[must_use]
    fn from(e: ShieldedResultCode) -> Self {
        e as Self
    }
}

impl ReadXdr for ShieldedResultCode {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            let e = i32::read_xdr(r)?;
            let v: Self = e.try_into()?;
            Ok(v)
        })
    }
}

impl WriteXdr for ShieldedResultCode {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            let i: i32 = (*self).into();
            i.write_xdr(w)
        })
    }
}
