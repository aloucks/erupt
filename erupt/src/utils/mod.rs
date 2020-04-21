//! Utilities to aid your usage of this crate

#[cfg(feature = "loading")]
pub mod loading;
#[cfg(feature = "surface")]
pub mod surface;

use crate::{vk1_0::Result as RawResult, vk1_1::*, CoreLoader};
use std::{
    convert::TryInto,
    error::Error,
    fmt::{self, Debug, Display},
    io,
};

// https://www.khronos.org/registry/spir-v/specs/unified1/SPIRV.html#_a_id_magic_a_magic_number
const SPV_MAGIC_NUMBER_LE: u32 = 0x07230203;
const SPV_MAGIC_NUMBER_BE: u32 = SPV_MAGIC_NUMBER_LE.swap_bytes();

impl<T> CoreLoader<T> {
    /// # Important notice
    /// `enumerate_instance_version` is only available on Vulkan 1.1+, this calls that function
    /// if it is available, otherwise it returns `erupt::make_version(1, 0, 0)`
    pub fn instance_version(&mut self) -> u32 {
        if self.vk1_1.is_none() {
            let _ = self.load_vk1_1();
        }

        if self.vk1_1.is_some() {
            unsafe { self.enumerate_instance_version(None) }.unwrap()
        } else {
            crate::make_version(1, 0, 0)
        }
    }
}

/// Idiomatic wrapper around a Vulkan Result
#[must_use = "this `VulkanResult` may be an error, which should be handled"]
#[derive(Copy, Clone, Default)]
pub struct VulkanResult<T> {
    pub raw: RawResult,
    pub value: Option<T>,
}

impl<T> VulkanResult<T> {
    /// Construct a new `VulkanResult` from `raw` and `value`
    ///
    /// This will not populate `self.value` if the raw result is negative (Error)
    #[inline]
    pub fn new(raw: RawResult, value: T) -> VulkanResult<T> {
        let value = if raw.0.is_negative() {
            None
        } else {
            Some(value)
        };

        VulkanResult { raw, value }
    }

    /// Returns the contained value, consuming `self`
    ///
    /// Panics with the name of `self.raw` if `self.value` is `None`
    #[inline]
    pub fn unwrap(self) -> T {
        match self.value {
            Some(value) => value,
            None => panic!("{:?}", self.raw),
        }
    }

    /// Returns the contained value, consuming `self`
    ///
    /// Panics with `msg` and the name of `self.raw` if `self.value` is `None`
    #[inline]
    pub fn expect(self, msg: impl Display) -> T {
        match self.value {
            Some(value) => value,
            None => panic!("{:?}: {}", self.raw, msg),
        }
    }

    /// Converts from `&VulkanResult<T>` to `VulkanResult<&T>`
    ///
    /// Clones `self.raw`
    #[inline]
    pub fn as_ref(&self) -> VulkanResult<&T> {
        VulkanResult {
            raw: self.raw.clone(),
            value: self.value.as_ref(),
        }
    }

    /// Converts from `&mut VulkanResult<T>` to `VulkanResult<&mut T>`
    ///
    /// Clones `self.raw`
    #[inline]
    pub fn as_mut(&mut self) -> VulkanResult<&mut T> {
        VulkanResult {
            raw: self.raw.clone(),
            value: self.value.as_mut(),
        }
    }

    /// Constructs a new `VulkanResult` from `value`
    ///
    /// This will always set `self.raw` to `RawResult::SUCCESS`
    #[inline]
    pub fn new_ok(value: T) -> VulkanResult<T> {
        VulkanResult::new(RawResult::SUCCESS, value)
    }

    /// Constructs a new `VulkanResult` from `raw`
    ///
    /// This will always set `self.value` to `None`
    #[inline]
    pub fn new_err(raw: RawResult) -> VulkanResult<T> {
        VulkanResult { raw, value: None }
    }

    /// Returns `self.value`, consuming `self` and dropping `self.raw`
    #[inline]
    pub fn ok(self) -> Option<T> {
        self.value
    }

    /// Maps `Some(v)` of `self.value` to `Ok(v)` and `None` of `self.value` to `Err(self.raw)`
    #[inline]
    pub fn result(self) -> Result<T, RawResult> {
        self.value.ok_or(self.raw)
    }

    /// Returns `true` if `self.value` is `Some`
    #[inline]
    pub fn is_ok(&self) -> bool {
        self.value.is_some()
    }

    /// Returns `true` if `self.raw` is positive
    #[inline]
    pub fn raw_is_ok(&self) -> bool {
        self.raw.0.is_positive()
    }

    /// Returns `true` if `self.value` is `None`
    #[inline]
    pub fn is_err(&self) -> bool {
        self.value.is_none()
    }

    /// Returns `true` if `self.raw` is negative
    #[inline]
    pub fn raw_is_err(&self) -> bool {
        self.raw.0.is_negative()
    }
}

impl Display for RawResult {
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(self, fmt)
    }
}

impl Error for RawResult {}

impl<T> Debug for VulkanResult<T>
where
    T: Debug,
{
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match &self.value {
            Some(value) => writeln!(fmt, "{:?}: {:?}", self.raw, value),
            None => writeln!(fmt, "{:?}: (no value)", self.raw),
        }
    }
}

impl<T> Display for VulkanResult<T>
where
    T: Display,
{
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match &self.value {
            Some(value) => writeln!(fmt, "{:?}: {}", self.raw, value),
            None => writeln!(fmt, "{:?}: (no value)", self.raw),
        }
    }
}

// inspired by https://docs.rs/ash/0.30.0/ash/util/fn.read_spv.html
/// Safely decode arbitrary SPIR-V data to it's correct word representation.
///
/// SPIR-V data consists of words (32-bit) not bytes (8-bit) and therefore
/// requires an alignment of 4 bytes. The byte data will be rejected if it is
/// not aligned. Additionally, it is possible that the bytes have wrong
/// endianness, which is also accounted for with this function.
pub fn decode_spv(bytes: &[u8]) -> io::Result<Vec<u32>> {
    if bytes.len() % 4 != 0 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "`bytes` is not 4-byte aligned",
        ));
    }

    let mut words: Vec<_> = bytes
        .chunks(4)
        .map(|word| u32::from_le_bytes(word.try_into().unwrap()))
        .collect();

    match words.get(0) {
        Some(&SPV_MAGIC_NUMBER_LE) => (),
        Some(&SPV_MAGIC_NUMBER_BE) => {
            for word in &mut words {
                *word = word.swap_bytes();
            }
        }
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "`bytes` is not valid SPIR-V data",
            ))
        }
    }

    Ok(words)
}
