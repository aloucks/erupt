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

#[cfg(feature = "loading")]
impl CoreLoader<libloading::Library> {
    /// Load functions using [`libloading`](https://crates.io/crates/libloading)
    ///
    /// Enabled using the `loading` cargo feature
    pub fn new() -> std::result::Result<CoreLoader<libloading::Library>, libloading::Error> {
        // from ash
        #[cfg(all(
            unix,
            not(any(target_os = "macos", target_os = "ios", target_os = "android"))
        ))]
        const LIB_PATH: &str = "libvulkan.so.1";

        // from ash
        #[cfg(target_os = "android")]
        const LIB_PATH: &str = "libvulkan.so";

        // from ash
        #[cfg(any(target_os = "macos", target_os = "ios"))]
        const LIB_PATH: &str = "libvulkan.dylib";

        // from ash
        #[cfg(windows)]
        const LIB_PATH: &str = "vulkan-1.dll";

        Ok(CoreLoader::custom(
            libloading::Library::new(LIB_PATH)?,
            Box::new(|loader, name| unsafe {
                let cstring = std::ffi::CString::new(name).unwrap();
                loader
                    .get(cstring.as_bytes_with_nul())
                    .ok()
                    .map(|symbol| *symbol)
            }),
        ))
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
    pub fn unwrap(self) -> T {
        match self.value {
            Some(value) => value,
            None => panic!("{:?}", self.raw),
        }
    }

    /// Returns the contained value, consuming `self`
    ///
    /// Panics with `msg` and the name of `self.raw` if `self.value` is `None`
    pub fn expect(self, msg: impl Display) -> T {
        match self.value {
            Some(value) => value,
            None => panic!("{:?}: {}", self.raw, msg),
        }
    }

    /// Converts from `&VulkanResult<T>` to `VulkanResult<&T>`
    ///
    /// Clones `self.raw`
    pub fn as_ref(&self) -> VulkanResult<&T> {
        VulkanResult {
            raw: self.raw.clone(),
            value: self.value.as_ref(),
        }
    }

    /// Converts from `&mut VulkanResult<T>` to `VulkanResult<&mut T>`
    ///
    /// Clones `self.raw`
    pub fn as_mut(&mut self) -> VulkanResult<&mut T> {
        VulkanResult {
            raw: self.raw.clone(),
            value: self.value.as_mut(),
        }
    }

    /// Constructs a new `VulkanResult` from `value`
    ///
    /// This will always set `self.raw` to `RawResult::SUCCESS`
    pub fn new_ok(value: T) -> VulkanResult<T> {
        VulkanResult::new(RawResult::SUCCESS, value)
    }

    /// Constructs a new `VulkanResult` from `raw`
    ///
    /// This will always set `self.value` to `None`
    pub fn new_err(raw: RawResult) -> VulkanResult<T> {
        VulkanResult { raw, value: None }
    }

    /// Returns `self.value`, consuming `self` and dropping `self.raw`
    pub fn ok(self) -> Option<T> {
        self.value
    }

    /// Maps `Some(v)` of `self.value` to `Ok(v)` and `None` of `self.value` to `Err(self.raw)`
    pub fn result(self) -> Result<T, RawResult> {
        self.value.ok_or(self.raw)
    }

    /// Returns `true` if `self.value` is `Some`
    pub fn is_ok(&self) -> bool {
        self.value.is_some()
    }

    /// Returns `true` if `self.raw` is positive
    pub fn raw_is_ok(&self) -> bool {
        self.raw.0.is_positive()
    }

    /// Returns `true` if `self.value` is `None`
    pub fn is_err(&self) -> bool {
        self.value.is_none()
    }

    /// Returns `true` if `self.raw` is negative
    pub fn raw_is_err(&self) -> bool {
        self.raw.0.is_negative()
    }
}

impl Display for RawResult {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(self, fmt)
    }
}

impl Error for RawResult {}

impl<T> Debug for VulkanResult<T>
where
    T: Debug,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match &self.value {
            Some(value) => writeln!(fmt, "{:?}: {:?}", self.raw, value),
            None => writeln!(fmt, "{:?}: <no value>", self.raw),
        }
    }
}

impl<T> Display for VulkanResult<T>
where
    T: Display,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match &self.value {
            Some(value) => writeln!(fmt, "{:?}: {}", self.raw, value),
            None => writeln!(fmt, "{:?}: <no value>", self.raw),
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
