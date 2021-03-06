#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_FORMAT_FEATURE_FLAGS_2_SPEC_VERSION")]
pub const KHR_FORMAT_FEATURE_FLAGS_2_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_FORMAT_FEATURE_FLAGS_2_EXTENSION_NAME")]
pub const KHR_FORMAT_FEATURE_FLAGS_2_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_KHR_format_feature_flags2");
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkFormatFeatureFlags2KHR.html) · Bitmask of [`FormatFeatureFlagBits2KHR`]"] # [doc (alias = "VkFormatFeatureFlags2KHR")] # [derive (Default)] # [repr (transparent)] pub struct FormatFeatureFlags2KHR : u64 { const SAMPLED_IMAGE_KHR = FormatFeatureFlagBits2KHR :: SAMPLED_IMAGE_KHR . 0 ; const STORAGE_IMAGE_KHR = FormatFeatureFlagBits2KHR :: STORAGE_IMAGE_KHR . 0 ; const STORAGE_IMAGE_ATOMIC_KHR = FormatFeatureFlagBits2KHR :: STORAGE_IMAGE_ATOMIC_KHR . 0 ; const UNIFORM_TEXEL_BUFFER_KHR = FormatFeatureFlagBits2KHR :: UNIFORM_TEXEL_BUFFER_KHR . 0 ; const STORAGE_TEXEL_BUFFER_KHR = FormatFeatureFlagBits2KHR :: STORAGE_TEXEL_BUFFER_KHR . 0 ; const STORAGE_TEXEL_BUFFER_ATOMIC_KHR = FormatFeatureFlagBits2KHR :: STORAGE_TEXEL_BUFFER_ATOMIC_KHR . 0 ; const VERTEX_BUFFER_KHR = FormatFeatureFlagBits2KHR :: VERTEX_BUFFER_KHR . 0 ; const COLOR_ATTACHMENT_KHR = FormatFeatureFlagBits2KHR :: COLOR_ATTACHMENT_KHR . 0 ; const COLOR_ATTACHMENT_BLEND_KHR = FormatFeatureFlagBits2KHR :: COLOR_ATTACHMENT_BLEND_KHR . 0 ; const DEPTH_STENCIL_ATTACHMENT_KHR = FormatFeatureFlagBits2KHR :: DEPTH_STENCIL_ATTACHMENT_KHR . 0 ; const BLIT_SRC_KHR = FormatFeatureFlagBits2KHR :: BLIT_SRC_KHR . 0 ; const BLIT_DST_KHR = FormatFeatureFlagBits2KHR :: BLIT_DST_KHR . 0 ; const SAMPLED_IMAGE_FILTER_LINEAR_KHR = FormatFeatureFlagBits2KHR :: SAMPLED_IMAGE_FILTER_LINEAR_KHR . 0 ; const SAMPLED_IMAGE_FILTER_CUBIC_EXT = FormatFeatureFlagBits2KHR :: SAMPLED_IMAGE_FILTER_CUBIC_EXT . 0 ; const TRANSFER_SRC_KHR = FormatFeatureFlagBits2KHR :: TRANSFER_SRC_KHR . 0 ; const TRANSFER_DST_KHR = FormatFeatureFlagBits2KHR :: TRANSFER_DST_KHR . 0 ; const SAMPLED_IMAGE_FILTER_MINMAX_KHR = FormatFeatureFlagBits2KHR :: SAMPLED_IMAGE_FILTER_MINMAX_KHR . 0 ; const MIDPOINT_CHROMA_SAMPLES_KHR = FormatFeatureFlagBits2KHR :: MIDPOINT_CHROMA_SAMPLES_KHR . 0 ; const SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_KHR = FormatFeatureFlagBits2KHR :: SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_KHR . 0 ; const SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_KHR = FormatFeatureFlagBits2KHR :: SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_KHR . 0 ; const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_KHR = FormatFeatureFlagBits2KHR :: SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_KHR . 0 ; const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_KHR = FormatFeatureFlagBits2KHR :: SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_KHR . 0 ; const DISJOINT_KHR = FormatFeatureFlagBits2KHR :: DISJOINT_KHR . 0 ; const COSITED_CHROMA_SAMPLES_KHR = FormatFeatureFlagBits2KHR :: COSITED_CHROMA_SAMPLES_KHR . 0 ; const STORAGE_READ_WITHOUT_FORMAT_KHR = FormatFeatureFlagBits2KHR :: STORAGE_READ_WITHOUT_FORMAT_KHR . 0 ; const STORAGE_WRITE_WITHOUT_FORMAT_KHR = FormatFeatureFlagBits2KHR :: STORAGE_WRITE_WITHOUT_FORMAT_KHR . 0 ; const SAMPLED_IMAGE_DEPTH_COMPARISON_KHR = FormatFeatureFlagBits2KHR :: SAMPLED_IMAGE_DEPTH_COMPARISON_KHR . 0 ; const VIDEO_DECODE_OUTPUT_KHR = FormatFeatureFlagBits2KHR :: VIDEO_DECODE_OUTPUT_KHR . 0 ; const VIDEO_DECODE_DPB_KHR = FormatFeatureFlagBits2KHR :: VIDEO_DECODE_DPB_KHR . 0 ; const ACCELERATION_STRUCTURE_VERTEX_BUFFER_KHR = FormatFeatureFlagBits2KHR :: ACCELERATION_STRUCTURE_VERTEX_BUFFER_KHR . 0 ; const FRAGMENT_DENSITY_MAP_EXT = FormatFeatureFlagBits2KHR :: FRAGMENT_DENSITY_MAP_EXT . 0 ; const FRAGMENT_SHADING_RATE_ATTACHMENT_KHR = FormatFeatureFlagBits2KHR :: FRAGMENT_SHADING_RATE_ATTACHMENT_KHR . 0 ; const VIDEO_ENCODE_INPUT_KHR = FormatFeatureFlagBits2KHR :: VIDEO_ENCODE_INPUT_KHR . 0 ; const VIDEO_ENCODE_DPB_KHR = FormatFeatureFlagBits2KHR :: VIDEO_ENCODE_DPB_KHR . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkFormatFeatureFlagBits2KHR.html) · Bits enum of [`FormatFeatureFlags2KHR`]"]
#[doc(alias = "VkFormatFeatureFlagBits2KHR")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct FormatFeatureFlagBits2KHR(pub u64);
impl FormatFeatureFlagBits2KHR {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> FormatFeatureFlags2KHR {
        FormatFeatureFlags2KHR::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for FormatFeatureFlagBits2KHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::SAMPLED_IMAGE_KHR => "SAMPLED_IMAGE_KHR",
            &Self::STORAGE_IMAGE_KHR => "STORAGE_IMAGE_KHR",
            &Self::STORAGE_IMAGE_ATOMIC_KHR => "STORAGE_IMAGE_ATOMIC_KHR",
            &Self::UNIFORM_TEXEL_BUFFER_KHR => "UNIFORM_TEXEL_BUFFER_KHR",
            &Self::STORAGE_TEXEL_BUFFER_KHR => "STORAGE_TEXEL_BUFFER_KHR",
            &Self::STORAGE_TEXEL_BUFFER_ATOMIC_KHR => "STORAGE_TEXEL_BUFFER_ATOMIC_KHR",
            &Self::VERTEX_BUFFER_KHR => "VERTEX_BUFFER_KHR",
            &Self::COLOR_ATTACHMENT_KHR => "COLOR_ATTACHMENT_KHR",
            &Self::COLOR_ATTACHMENT_BLEND_KHR => "COLOR_ATTACHMENT_BLEND_KHR",
            &Self::DEPTH_STENCIL_ATTACHMENT_KHR => "DEPTH_STENCIL_ATTACHMENT_KHR",
            &Self::BLIT_SRC_KHR => "BLIT_SRC_KHR",
            &Self::BLIT_DST_KHR => "BLIT_DST_KHR",
            &Self::SAMPLED_IMAGE_FILTER_LINEAR_KHR => "SAMPLED_IMAGE_FILTER_LINEAR_KHR",
            &Self::SAMPLED_IMAGE_FILTER_CUBIC_EXT => "SAMPLED_IMAGE_FILTER_CUBIC_EXT",
            &Self::TRANSFER_SRC_KHR => "TRANSFER_SRC_KHR",
            &Self::TRANSFER_DST_KHR => "TRANSFER_DST_KHR",
            &Self::SAMPLED_IMAGE_FILTER_MINMAX_KHR => "SAMPLED_IMAGE_FILTER_MINMAX_KHR",
            &Self::MIDPOINT_CHROMA_SAMPLES_KHR => "MIDPOINT_CHROMA_SAMPLES_KHR",
            &Self::SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_KHR => "SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_KHR",
            &Self::SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_KHR => "SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_KHR",
            &Self::SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_KHR => "SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_KHR",
            &Self::SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_KHR => "SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_KHR",
            &Self::DISJOINT_KHR => "DISJOINT_KHR",
            &Self::COSITED_CHROMA_SAMPLES_KHR => "COSITED_CHROMA_SAMPLES_KHR",
            &Self::STORAGE_READ_WITHOUT_FORMAT_KHR => "STORAGE_READ_WITHOUT_FORMAT_KHR",
            &Self::STORAGE_WRITE_WITHOUT_FORMAT_KHR => "STORAGE_WRITE_WITHOUT_FORMAT_KHR",
            &Self::SAMPLED_IMAGE_DEPTH_COMPARISON_KHR => "SAMPLED_IMAGE_DEPTH_COMPARISON_KHR",
            &Self::VIDEO_DECODE_OUTPUT_KHR => "VIDEO_DECODE_OUTPUT_KHR",
            &Self::VIDEO_DECODE_DPB_KHR => "VIDEO_DECODE_DPB_KHR",
            &Self::ACCELERATION_STRUCTURE_VERTEX_BUFFER_KHR => "ACCELERATION_STRUCTURE_VERTEX_BUFFER_KHR",
            &Self::FRAGMENT_DENSITY_MAP_EXT => "FRAGMENT_DENSITY_MAP_EXT",
            &Self::FRAGMENT_SHADING_RATE_ATTACHMENT_KHR => "FRAGMENT_SHADING_RATE_ATTACHMENT_KHR",
            &Self::VIDEO_ENCODE_INPUT_KHR => "VIDEO_ENCODE_INPUT_KHR",
            &Self::VIDEO_ENCODE_DPB_KHR => "VIDEO_ENCODE_DPB_KHR",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::khr_format_feature_flags2`]"]
impl crate::extensions::khr_format_feature_flags2::FormatFeatureFlagBits2KHR {
    pub const SAMPLED_IMAGE_KHR: Self = Self(1);
    pub const STORAGE_IMAGE_KHR: Self = Self(2);
    pub const STORAGE_IMAGE_ATOMIC_KHR: Self = Self(4);
    pub const UNIFORM_TEXEL_BUFFER_KHR: Self = Self(8);
    pub const STORAGE_TEXEL_BUFFER_KHR: Self = Self(16);
    pub const STORAGE_TEXEL_BUFFER_ATOMIC_KHR: Self = Self(32);
    pub const VERTEX_BUFFER_KHR: Self = Self(64);
    pub const COLOR_ATTACHMENT_KHR: Self = Self(128);
    pub const COLOR_ATTACHMENT_BLEND_KHR: Self = Self(256);
    pub const DEPTH_STENCIL_ATTACHMENT_KHR: Self = Self(512);
    pub const BLIT_SRC_KHR: Self = Self(1024);
    pub const BLIT_DST_KHR: Self = Self(2048);
    pub const SAMPLED_IMAGE_FILTER_LINEAR_KHR: Self = Self(4096);
    pub const SAMPLED_IMAGE_FILTER_CUBIC_EXT: Self = Self(8192);
    pub const TRANSFER_SRC_KHR: Self = Self(16384);
    pub const TRANSFER_DST_KHR: Self = Self(32768);
    pub const SAMPLED_IMAGE_FILTER_MINMAX_KHR: Self = Self(65536);
    pub const MIDPOINT_CHROMA_SAMPLES_KHR: Self = Self(131072);
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_KHR: Self = Self(262144);
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_KHR: Self = Self(524288);
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_KHR: Self = Self(1048576);
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_KHR: Self = Self(2097152);
    pub const DISJOINT_KHR: Self = Self(4194304);
    pub const COSITED_CHROMA_SAMPLES_KHR: Self = Self(8388608);
    pub const STORAGE_READ_WITHOUT_FORMAT_KHR: Self = Self(2147483648);
    pub const STORAGE_WRITE_WITHOUT_FORMAT_KHR: Self = Self(4294967296);
    pub const SAMPLED_IMAGE_DEPTH_COMPARISON_KHR: Self = Self(8589934592);
}
#[doc = "Provided by [`crate::extensions::khr_format_feature_flags2`]"]
impl crate::vk1_0::StructureType {
    pub const FORMAT_PROPERTIES_3_KHR: Self = Self(1000360000);
}
impl<'a> crate::ExtendableFrom<'a, FormatProperties3KHR> for crate::vk1_1::FormatProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, FormatProperties3KHRBuilder<'_>> for crate::vk1_1::FormatProperties2Builder<'a> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkFormatProperties3KHR.html) · Structure"]
#[doc(alias = "VkFormatProperties3KHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FormatProperties3KHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub linear_tiling_features: crate::extensions::khr_format_feature_flags2::FormatFeatureFlags2KHR,
    pub optimal_tiling_features: crate::extensions::khr_format_feature_flags2::FormatFeatureFlags2KHR,
    pub buffer_features: crate::extensions::khr_format_feature_flags2::FormatFeatureFlags2KHR,
}
impl FormatProperties3KHR {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::FORMAT_PROPERTIES_3_KHR;
}
impl Default for FormatProperties3KHR {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null_mut(), linear_tiling_features: Default::default(), optimal_tiling_features: Default::default(), buffer_features: Default::default() }
    }
}
impl std::fmt::Debug for FormatProperties3KHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("FormatProperties3KHR").field("s_type", &self.s_type).field("p_next", &self.p_next).field("linear_tiling_features", &self.linear_tiling_features).field("optimal_tiling_features", &self.optimal_tiling_features).field("buffer_features", &self.buffer_features).finish()
    }
}
impl FormatProperties3KHR {
    #[inline]
    pub fn into_builder<'a>(self) -> FormatProperties3KHRBuilder<'a> {
        FormatProperties3KHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkFormatProperties3KHR.html) · Builder of [`FormatProperties3KHR`]"]
#[repr(transparent)]
pub struct FormatProperties3KHRBuilder<'a>(FormatProperties3KHR, std::marker::PhantomData<&'a ()>);
impl<'a> FormatProperties3KHRBuilder<'a> {
    #[inline]
    pub fn new() -> FormatProperties3KHRBuilder<'a> {
        FormatProperties3KHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn linear_tiling_features(mut self, linear_tiling_features: crate::extensions::khr_format_feature_flags2::FormatFeatureFlags2KHR) -> Self {
        self.0.linear_tiling_features = linear_tiling_features as _;
        self
    }
    #[inline]
    pub fn optimal_tiling_features(mut self, optimal_tiling_features: crate::extensions::khr_format_feature_flags2::FormatFeatureFlags2KHR) -> Self {
        self.0.optimal_tiling_features = optimal_tiling_features as _;
        self
    }
    #[inline]
    pub fn buffer_features(mut self, buffer_features: crate::extensions::khr_format_feature_flags2::FormatFeatureFlags2KHR) -> Self {
        self.0.buffer_features = buffer_features as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> FormatProperties3KHR {
        self.0
    }
}
impl<'a> std::default::Default for FormatProperties3KHRBuilder<'a> {
    fn default() -> FormatProperties3KHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for FormatProperties3KHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for FormatProperties3KHRBuilder<'a> {
    type Target = FormatProperties3KHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for FormatProperties3KHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
