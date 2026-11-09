#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// NativeAmount128 is an XDR Struct defined as:
///
/// ```text
/// struct NativeAmount128
/// {
///     hyper hi;
///     unsigned hyper lo;
/// };
/// ```
///
#[cfg_attr(feature = "alloc", derive(Default))]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", cfg_eval::cfg_eval)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
#[cfg_attr(
    all(feature = "serde", feature = "alloc"),
    serde_with::serde_as,
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct NativeAmount128 {
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub hi: i64,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub lo: u64,
}

impl ReadXdr for NativeAmount128 {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                hi: i64::read_xdr(r)?,
                lo: u64::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for NativeAmount128 {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.hi.write_xdr(w)?;
            self.lo.write_xdr(w)?;
            Ok(())
        })
    }
}
