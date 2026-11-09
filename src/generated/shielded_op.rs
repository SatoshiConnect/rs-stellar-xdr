#[allow(unused_imports, clippy::wildcard_imports)]
use super::*;

/// ShieldedOp is an XDR Struct defined as:
///
/// ```text
/// struct ShieldedOp
/// {
///     opaque bundleZip225<262144>;
///     int64 transparentOutput;
///     AccountID withdrawalTarget;
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
pub struct ShieldedOp {
    pub bundle_zip225: BytesM<262144>,
    #[cfg_attr(
        all(feature = "serde", feature = "alloc"),
        serde_as(as = "NumberOrString")
    )]
    pub transparent_output: i64,
    pub withdrawal_target: AccountId,
}

impl ReadXdr for ShieldedOp {
    #[cfg(feature = "std")]
    fn read_xdr<R: Read>(r: &mut Limited<R>) -> Result<Self, Error> {
        r.with_limited_depth(|r| {
            Ok(Self {
                bundle_zip225: BytesM::<262144>::read_xdr(r)?,
                transparent_output: i64::read_xdr(r)?,
                withdrawal_target: AccountId::read_xdr(r)?,
            })
        })
    }
}

impl WriteXdr for ShieldedOp {
    #[cfg(feature = "std")]
    fn write_xdr<W: Write>(&self, w: &mut Limited<W>) -> Result<(), Error> {
        w.with_limited_depth(|w| {
            self.bundle_zip225.write_xdr(w)?;
            self.transparent_output.write_xdr(w)?;
            self.withdrawal_target.write_xdr(w)?;
            Ok(())
        })
    }
}
