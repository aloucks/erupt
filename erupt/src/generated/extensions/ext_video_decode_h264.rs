//! ## Versioning Warning ⚠️
//!
//! This is a Vulkan **provisional/beta** extension and **must** be used with
//! caution. Its API/behaviour has not been finalized yet and _may_ therefore
//! change in ways that break backwards compatibility between revisions, and
//! before final release of a non-provisional version of this extension.
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_VIDEO_DECODE_H264_SPEC_VERSION")]
pub const EXT_VIDEO_DECODE_H264_SPEC_VERSION: u32 = 3;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_VIDEO_DECODE_H264_EXTENSION_NAME")]
pub const EXT_VIDEO_DECODE_H264_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_EXT_video_decode_h264");
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoDecodeH264CreateFlagsEXT.html) · Bitmask of [`VideoDecodeH264CreateFlagBitsEXT`]"] # [doc (alias = "VkVideoDecodeH264CreateFlagsEXT")] # [derive (Default)] # [repr (transparent)] pub struct VideoDecodeH264CreateFlagsEXT : u32 { # [cfg (empty_bitflag_workaround)] const EMPTY_BITFLAG_WORKAROUND = 0 ; } }
#[doc = "<s>Vulkan Manual Page</s> · Bits enum of [`VideoDecodeH264CreateFlagsEXT`]"]
#[doc(alias = "VkVideoDecodeH264CreateFlagBitsEXT")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct VideoDecodeH264CreateFlagBitsEXT(pub u32);
impl VideoDecodeH264CreateFlagBitsEXT {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> VideoDecodeH264CreateFlagsEXT {
        VideoDecodeH264CreateFlagsEXT::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for VideoDecodeH264CreateFlagBitsEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::ext_video_decode_h264`]"]
impl crate::vk1_0::StructureType {
    pub const VIDEO_DECODE_H264_CAPABILITIES_EXT: Self = Self(1000040000);
    pub const VIDEO_DECODE_H264_SESSION_CREATE_INFO_EXT: Self = Self(1000040001);
    pub const VIDEO_DECODE_H264_PICTURE_INFO_EXT: Self = Self(1000040002);
    pub const VIDEO_DECODE_H264_MVC_EXT: Self = Self(1000040003);
    pub const VIDEO_DECODE_H264_PROFILE_EXT: Self = Self(1000040004);
    pub const VIDEO_DECODE_H264_SESSION_PARAMETERS_CREATE_INFO_EXT: Self = Self(1000040005);
    pub const VIDEO_DECODE_H264_SESSION_PARAMETERS_ADD_INFO_EXT: Self = Self(1000040006);
    pub const VIDEO_DECODE_H264_DPB_SLOT_INFO_EXT: Self = Self(1000040007);
}
#[doc = "Provided by [`crate::extensions::ext_video_decode_h264`]"]
impl crate::extensions::khr_video_queue::VideoCodecOperationFlagBitsKHR {
    pub const DECODE_H264_EXT: Self = Self(1);
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoDecodeH264PictureLayoutFlagsEXT.html) · Bitmask of [`VideoDecodeH264PictureLayoutFlagBitsEXT`]"] # [doc (alias = "VkVideoDecodeH264PictureLayoutFlagsEXT")] # [derive (Default)] # [repr (transparent)] pub struct VideoDecodeH264PictureLayoutFlagsEXT : u32 { const PROGRESSIVE_EXT = VideoDecodeH264PictureLayoutFlagBitsEXT :: PROGRESSIVE_EXT . 0 ; const INTERLACED_INTERLEAVED_LINES_EXT = VideoDecodeH264PictureLayoutFlagBitsEXT :: INTERLACED_INTERLEAVED_LINES_EXT . 0 ; const INTERLACED_SEPARATE_PLANES_EXT = VideoDecodeH264PictureLayoutFlagBitsEXT :: INTERLACED_SEPARATE_PLANES_EXT . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoDecodeH264PictureLayoutFlagBitsEXT.html) · Bits enum of [`VideoDecodeH264PictureLayoutFlagsEXT`]"]
#[doc(alias = "VkVideoDecodeH264PictureLayoutFlagBitsEXT")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct VideoDecodeH264PictureLayoutFlagBitsEXT(pub u32);
impl VideoDecodeH264PictureLayoutFlagBitsEXT {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> VideoDecodeH264PictureLayoutFlagsEXT {
        VideoDecodeH264PictureLayoutFlagsEXT::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for VideoDecodeH264PictureLayoutFlagBitsEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::PROGRESSIVE_EXT => "PROGRESSIVE_EXT",
            &Self::INTERLACED_INTERLEAVED_LINES_EXT => "INTERLACED_INTERLEAVED_LINES_EXT",
            &Self::INTERLACED_SEPARATE_PLANES_EXT => "INTERLACED_SEPARATE_PLANES_EXT",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::ext_video_decode_h264`]"]
impl crate::extensions::ext_video_decode_h264::VideoDecodeH264PictureLayoutFlagBitsEXT {
    pub const PROGRESSIVE_EXT: Self = Self(0);
    pub const INTERLACED_INTERLEAVED_LINES_EXT: Self = Self(1);
    pub const INTERLACED_SEPARATE_PLANES_EXT: Self = Self(2);
}
impl<'a> crate::ExtendableFrom<'a, VideoDecodeH264ProfileEXT> for crate::vk1_0::BufferCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, VideoDecodeH264ProfileEXTBuilder<'_>> for crate::vk1_0::BufferCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, VideoDecodeH264ProfileEXT> for crate::vk1_0::ImageCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, VideoDecodeH264ProfileEXTBuilder<'_>> for crate::vk1_0::ImageCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, VideoDecodeH264ProfileEXT> for crate::vk1_0::ImageViewCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, VideoDecodeH264ProfileEXTBuilder<'_>> for crate::vk1_0::ImageViewCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, VideoDecodeH264ProfileEXT> for crate::vk1_0::QueryPoolCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, VideoDecodeH264ProfileEXTBuilder<'_>> for crate::vk1_0::QueryPoolCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, VideoDecodeH264ProfileEXT> for crate::vk1_1::FormatProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, VideoDecodeH264ProfileEXTBuilder<'_>> for crate::vk1_1::FormatProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, VideoDecodeH264ProfileEXT> for crate::extensions::khr_video_queue::VideoProfileKHRBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, VideoDecodeH264ProfileEXTBuilder<'_>> for crate::extensions::khr_video_queue::VideoProfileKHRBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, VideoDecodeH264CapabilitiesEXT> for crate::extensions::khr_video_queue::VideoCapabilitiesKHRBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, VideoDecodeH264CapabilitiesEXTBuilder<'_>> for crate::extensions::khr_video_queue::VideoCapabilitiesKHRBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, VideoDecodeH264DpbSlotInfoEXT> for crate::extensions::khr_video_queue::VideoReferenceSlotKHRBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, VideoDecodeH264DpbSlotInfoEXTBuilder<'_>> for crate::extensions::khr_video_queue::VideoReferenceSlotKHRBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, VideoDecodeH264PictureInfoEXT> for crate::extensions::khr_video_decode_queue::VideoDecodeInfoKHRBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, VideoDecodeH264PictureInfoEXTBuilder<'_>> for crate::extensions::khr_video_decode_queue::VideoDecodeInfoKHRBuilder<'a> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoDecodeH264ProfileEXT.html) · Structure"]
#[doc(alias = "VkVideoDecodeH264ProfileEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VideoDecodeH264ProfileEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub std_profile_idc: crate::external::vk_video::StdVideoH264ProfileIdc,
    pub picture_layout: crate::extensions::ext_video_decode_h264::VideoDecodeH264PictureLayoutFlagsEXT,
}
impl VideoDecodeH264ProfileEXT {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::VIDEO_DECODE_H264_PROFILE_EXT;
}
impl Default for VideoDecodeH264ProfileEXT {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), std_profile_idc: Default::default(), picture_layout: Default::default() }
    }
}
impl std::fmt::Debug for VideoDecodeH264ProfileEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("VideoDecodeH264ProfileEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("std_profile_idc", &self.std_profile_idc).field("picture_layout", &self.picture_layout).finish()
    }
}
impl VideoDecodeH264ProfileEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> VideoDecodeH264ProfileEXTBuilder<'a> {
        VideoDecodeH264ProfileEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoDecodeH264ProfileEXT.html) · Builder of [`VideoDecodeH264ProfileEXT`]"]
#[repr(transparent)]
pub struct VideoDecodeH264ProfileEXTBuilder<'a>(VideoDecodeH264ProfileEXT, std::marker::PhantomData<&'a ()>);
impl<'a> VideoDecodeH264ProfileEXTBuilder<'a> {
    #[inline]
    pub fn new() -> VideoDecodeH264ProfileEXTBuilder<'a> {
        VideoDecodeH264ProfileEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn std_profile_idc(mut self, std_profile_idc: crate::external::vk_video::StdVideoH264ProfileIdc) -> Self {
        self.0.std_profile_idc = std_profile_idc as _;
        self
    }
    #[inline]
    pub fn picture_layout(mut self, picture_layout: crate::extensions::ext_video_decode_h264::VideoDecodeH264PictureLayoutFlagsEXT) -> Self {
        self.0.picture_layout = picture_layout as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> VideoDecodeH264ProfileEXT {
        self.0
    }
}
impl<'a> std::default::Default for VideoDecodeH264ProfileEXTBuilder<'a> {
    fn default() -> VideoDecodeH264ProfileEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for VideoDecodeH264ProfileEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for VideoDecodeH264ProfileEXTBuilder<'a> {
    type Target = VideoDecodeH264ProfileEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for VideoDecodeH264ProfileEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoDecodeH264CapabilitiesEXT.html) · Structure"]
#[doc(alias = "VkVideoDecodeH264CapabilitiesEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VideoDecodeH264CapabilitiesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub max_level: u32,
    pub field_offset_granularity: crate::vk1_0::Offset2D,
    pub std_extension_version: crate::vk1_0::ExtensionProperties,
}
impl VideoDecodeH264CapabilitiesEXT {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::VIDEO_DECODE_H264_CAPABILITIES_EXT;
}
impl Default for VideoDecodeH264CapabilitiesEXT {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null_mut(), max_level: Default::default(), field_offset_granularity: Default::default(), std_extension_version: Default::default() }
    }
}
impl std::fmt::Debug for VideoDecodeH264CapabilitiesEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("VideoDecodeH264CapabilitiesEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("max_level", &self.max_level).field("field_offset_granularity", &self.field_offset_granularity).field("std_extension_version", &self.std_extension_version).finish()
    }
}
impl VideoDecodeH264CapabilitiesEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> VideoDecodeH264CapabilitiesEXTBuilder<'a> {
        VideoDecodeH264CapabilitiesEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoDecodeH264CapabilitiesEXT.html) · Builder of [`VideoDecodeH264CapabilitiesEXT`]"]
#[repr(transparent)]
pub struct VideoDecodeH264CapabilitiesEXTBuilder<'a>(VideoDecodeH264CapabilitiesEXT, std::marker::PhantomData<&'a ()>);
impl<'a> VideoDecodeH264CapabilitiesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> VideoDecodeH264CapabilitiesEXTBuilder<'a> {
        VideoDecodeH264CapabilitiesEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn max_level(mut self, max_level: u32) -> Self {
        self.0.max_level = max_level as _;
        self
    }
    #[inline]
    pub fn field_offset_granularity(mut self, field_offset_granularity: crate::vk1_0::Offset2D) -> Self {
        self.0.field_offset_granularity = field_offset_granularity as _;
        self
    }
    #[inline]
    pub fn std_extension_version(mut self, std_extension_version: crate::vk1_0::ExtensionProperties) -> Self {
        self.0.std_extension_version = std_extension_version as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> VideoDecodeH264CapabilitiesEXT {
        self.0
    }
}
impl<'a> std::default::Default for VideoDecodeH264CapabilitiesEXTBuilder<'a> {
    fn default() -> VideoDecodeH264CapabilitiesEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for VideoDecodeH264CapabilitiesEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for VideoDecodeH264CapabilitiesEXTBuilder<'a> {
    type Target = VideoDecodeH264CapabilitiesEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for VideoDecodeH264CapabilitiesEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoDecodeH264SessionCreateInfoEXT.html) · Structure"]
#[doc(alias = "VkVideoDecodeH264SessionCreateInfoEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VideoDecodeH264SessionCreateInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::extensions::ext_video_decode_h264::VideoDecodeH264CreateFlagsEXT,
    pub p_std_extension_version: *const crate::vk1_0::ExtensionProperties,
}
impl VideoDecodeH264SessionCreateInfoEXT {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::VIDEO_DECODE_H264_SESSION_CREATE_INFO_EXT;
}
impl Default for VideoDecodeH264SessionCreateInfoEXT {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), flags: Default::default(), p_std_extension_version: std::ptr::null() }
    }
}
impl std::fmt::Debug for VideoDecodeH264SessionCreateInfoEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("VideoDecodeH264SessionCreateInfoEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("flags", &self.flags).field("p_std_extension_version", &self.p_std_extension_version).finish()
    }
}
impl VideoDecodeH264SessionCreateInfoEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> VideoDecodeH264SessionCreateInfoEXTBuilder<'a> {
        VideoDecodeH264SessionCreateInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoDecodeH264SessionCreateInfoEXT.html) · Builder of [`VideoDecodeH264SessionCreateInfoEXT`]"]
#[repr(transparent)]
pub struct VideoDecodeH264SessionCreateInfoEXTBuilder<'a>(VideoDecodeH264SessionCreateInfoEXT, std::marker::PhantomData<&'a ()>);
impl<'a> VideoDecodeH264SessionCreateInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> VideoDecodeH264SessionCreateInfoEXTBuilder<'a> {
        VideoDecodeH264SessionCreateInfoEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn flags(mut self, flags: crate::extensions::ext_video_decode_h264::VideoDecodeH264CreateFlagsEXT) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    pub fn std_extension_version(mut self, std_extension_version: &'a crate::vk1_0::ExtensionProperties) -> Self {
        self.0.p_std_extension_version = std_extension_version as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> VideoDecodeH264SessionCreateInfoEXT {
        self.0
    }
}
impl<'a> std::default::Default for VideoDecodeH264SessionCreateInfoEXTBuilder<'a> {
    fn default() -> VideoDecodeH264SessionCreateInfoEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for VideoDecodeH264SessionCreateInfoEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for VideoDecodeH264SessionCreateInfoEXTBuilder<'a> {
    type Target = VideoDecodeH264SessionCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for VideoDecodeH264SessionCreateInfoEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoDecodeH264SessionParametersAddInfoEXT.html) · Structure"]
#[doc(alias = "VkVideoDecodeH264SessionParametersAddInfoEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VideoDecodeH264SessionParametersAddInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub sps_std_count: u32,
    pub p_sps_std: *const crate::external::vk_video::StdVideoH264SequenceParameterSet,
    pub pps_std_count: u32,
    pub p_pps_std: *const crate::external::vk_video::StdVideoH264PictureParameterSet,
}
impl VideoDecodeH264SessionParametersAddInfoEXT {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::VIDEO_DECODE_H264_SESSION_PARAMETERS_ADD_INFO_EXT;
}
impl Default for VideoDecodeH264SessionParametersAddInfoEXT {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), sps_std_count: Default::default(), p_sps_std: std::ptr::null(), pps_std_count: Default::default(), p_pps_std: std::ptr::null() }
    }
}
impl std::fmt::Debug for VideoDecodeH264SessionParametersAddInfoEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("VideoDecodeH264SessionParametersAddInfoEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("sps_std_count", &self.sps_std_count).field("p_sps_std", &self.p_sps_std).field("pps_std_count", &self.pps_std_count).field("p_pps_std", &self.p_pps_std).finish()
    }
}
impl VideoDecodeH264SessionParametersAddInfoEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> VideoDecodeH264SessionParametersAddInfoEXTBuilder<'a> {
        VideoDecodeH264SessionParametersAddInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoDecodeH264SessionParametersAddInfoEXT.html) · Builder of [`VideoDecodeH264SessionParametersAddInfoEXT`]"]
#[repr(transparent)]
pub struct VideoDecodeH264SessionParametersAddInfoEXTBuilder<'a>(VideoDecodeH264SessionParametersAddInfoEXT, std::marker::PhantomData<&'a ()>);
impl<'a> VideoDecodeH264SessionParametersAddInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> VideoDecodeH264SessionParametersAddInfoEXTBuilder<'a> {
        VideoDecodeH264SessionParametersAddInfoEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn sps_std(mut self, sps_std: &'a [crate::external::vk_video::StdVideoH264SequenceParameterSetBuilder]) -> Self {
        self.0.p_sps_std = sps_std.as_ptr() as _;
        self.0.sps_std_count = sps_std.len() as _;
        self
    }
    #[inline]
    pub fn pps_std(mut self, pps_std: &'a [crate::external::vk_video::StdVideoH264PictureParameterSetBuilder]) -> Self {
        self.0.p_pps_std = pps_std.as_ptr() as _;
        self.0.pps_std_count = pps_std.len() as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> VideoDecodeH264SessionParametersAddInfoEXT {
        self.0
    }
}
impl<'a> std::default::Default for VideoDecodeH264SessionParametersAddInfoEXTBuilder<'a> {
    fn default() -> VideoDecodeH264SessionParametersAddInfoEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for VideoDecodeH264SessionParametersAddInfoEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for VideoDecodeH264SessionParametersAddInfoEXTBuilder<'a> {
    type Target = VideoDecodeH264SessionParametersAddInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for VideoDecodeH264SessionParametersAddInfoEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoDecodeH264SessionParametersCreateInfoEXT.html) · Structure"]
#[doc(alias = "VkVideoDecodeH264SessionParametersCreateInfoEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VideoDecodeH264SessionParametersCreateInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub max_sps_std_count: u32,
    pub max_pps_std_count: u32,
    pub p_parameters_add_info: *const crate::extensions::ext_video_decode_h264::VideoDecodeH264SessionParametersAddInfoEXT,
}
impl VideoDecodeH264SessionParametersCreateInfoEXT {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::VIDEO_DECODE_H264_SESSION_PARAMETERS_CREATE_INFO_EXT;
}
impl Default for VideoDecodeH264SessionParametersCreateInfoEXT {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), max_sps_std_count: Default::default(), max_pps_std_count: Default::default(), p_parameters_add_info: std::ptr::null() }
    }
}
impl std::fmt::Debug for VideoDecodeH264SessionParametersCreateInfoEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("VideoDecodeH264SessionParametersCreateInfoEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("max_sps_std_count", &self.max_sps_std_count).field("max_pps_std_count", &self.max_pps_std_count).field("p_parameters_add_info", &self.p_parameters_add_info).finish()
    }
}
impl VideoDecodeH264SessionParametersCreateInfoEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> VideoDecodeH264SessionParametersCreateInfoEXTBuilder<'a> {
        VideoDecodeH264SessionParametersCreateInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoDecodeH264SessionParametersCreateInfoEXT.html) · Builder of [`VideoDecodeH264SessionParametersCreateInfoEXT`]"]
#[repr(transparent)]
pub struct VideoDecodeH264SessionParametersCreateInfoEXTBuilder<'a>(VideoDecodeH264SessionParametersCreateInfoEXT, std::marker::PhantomData<&'a ()>);
impl<'a> VideoDecodeH264SessionParametersCreateInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> VideoDecodeH264SessionParametersCreateInfoEXTBuilder<'a> {
        VideoDecodeH264SessionParametersCreateInfoEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn max_sps_std_count(mut self, max_sps_std_count: u32) -> Self {
        self.0.max_sps_std_count = max_sps_std_count as _;
        self
    }
    #[inline]
    pub fn max_pps_std_count(mut self, max_pps_std_count: u32) -> Self {
        self.0.max_pps_std_count = max_pps_std_count as _;
        self
    }
    #[inline]
    pub fn parameters_add_info(mut self, parameters_add_info: &'a crate::extensions::ext_video_decode_h264::VideoDecodeH264SessionParametersAddInfoEXT) -> Self {
        self.0.p_parameters_add_info = parameters_add_info as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> VideoDecodeH264SessionParametersCreateInfoEXT {
        self.0
    }
}
impl<'a> std::default::Default for VideoDecodeH264SessionParametersCreateInfoEXTBuilder<'a> {
    fn default() -> VideoDecodeH264SessionParametersCreateInfoEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for VideoDecodeH264SessionParametersCreateInfoEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for VideoDecodeH264SessionParametersCreateInfoEXTBuilder<'a> {
    type Target = VideoDecodeH264SessionParametersCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for VideoDecodeH264SessionParametersCreateInfoEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoDecodeH264PictureInfoEXT.html) · Structure"]
#[doc(alias = "VkVideoDecodeH264PictureInfoEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VideoDecodeH264PictureInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub p_std_picture_info: *const crate::external::vk_video::StdVideoDecodeH264PictureInfo,
    pub slices_count: u32,
    pub p_slices_data_offsets: *const u32,
}
impl VideoDecodeH264PictureInfoEXT {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::VIDEO_DECODE_H264_PICTURE_INFO_EXT;
}
impl Default for VideoDecodeH264PictureInfoEXT {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), p_std_picture_info: std::ptr::null(), slices_count: Default::default(), p_slices_data_offsets: std::ptr::null() }
    }
}
impl std::fmt::Debug for VideoDecodeH264PictureInfoEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("VideoDecodeH264PictureInfoEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("p_std_picture_info", &self.p_std_picture_info).field("slices_count", &self.slices_count).field("p_slices_data_offsets", &self.p_slices_data_offsets).finish()
    }
}
impl VideoDecodeH264PictureInfoEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> VideoDecodeH264PictureInfoEXTBuilder<'a> {
        VideoDecodeH264PictureInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
impl<'a> crate::ExtendableFrom<'a, VideoDecodeH264MvcEXT> for crate::extensions::ext_video_decode_h264::VideoDecodeH264PictureInfoEXTBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, VideoDecodeH264MvcEXTBuilder<'_>> for crate::extensions::ext_video_decode_h264::VideoDecodeH264PictureInfoEXTBuilder<'a> {}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoDecodeH264PictureInfoEXT.html) · Builder of [`VideoDecodeH264PictureInfoEXT`]"]
#[repr(transparent)]
pub struct VideoDecodeH264PictureInfoEXTBuilder<'a>(VideoDecodeH264PictureInfoEXT, std::marker::PhantomData<&'a ()>);
impl<'a> VideoDecodeH264PictureInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> VideoDecodeH264PictureInfoEXTBuilder<'a> {
        VideoDecodeH264PictureInfoEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn std_picture_info(mut self, std_picture_info: &'a crate::external::vk_video::StdVideoDecodeH264PictureInfo) -> Self {
        self.0.p_std_picture_info = std_picture_info as _;
        self
    }
    #[inline]
    pub fn slices_data_offsets(mut self, slices_data_offsets: &'a [u32]) -> Self {
        self.0.p_slices_data_offsets = slices_data_offsets.as_ptr() as _;
        self.0.slices_count = slices_data_offsets.len() as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> VideoDecodeH264PictureInfoEXT {
        self.0
    }
}
impl<'a> std::default::Default for VideoDecodeH264PictureInfoEXTBuilder<'a> {
    fn default() -> VideoDecodeH264PictureInfoEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for VideoDecodeH264PictureInfoEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for VideoDecodeH264PictureInfoEXTBuilder<'a> {
    type Target = VideoDecodeH264PictureInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for VideoDecodeH264PictureInfoEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoDecodeH264DpbSlotInfoEXT.html) · Structure"]
#[doc(alias = "VkVideoDecodeH264DpbSlotInfoEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VideoDecodeH264DpbSlotInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub p_std_reference_info: *const crate::external::vk_video::StdVideoDecodeH264ReferenceInfo,
}
impl VideoDecodeH264DpbSlotInfoEXT {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::VIDEO_DECODE_H264_DPB_SLOT_INFO_EXT;
}
impl Default for VideoDecodeH264DpbSlotInfoEXT {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), p_std_reference_info: std::ptr::null() }
    }
}
impl std::fmt::Debug for VideoDecodeH264DpbSlotInfoEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("VideoDecodeH264DpbSlotInfoEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("p_std_reference_info", &self.p_std_reference_info).finish()
    }
}
impl VideoDecodeH264DpbSlotInfoEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> VideoDecodeH264DpbSlotInfoEXTBuilder<'a> {
        VideoDecodeH264DpbSlotInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoDecodeH264DpbSlotInfoEXT.html) · Builder of [`VideoDecodeH264DpbSlotInfoEXT`]"]
#[repr(transparent)]
pub struct VideoDecodeH264DpbSlotInfoEXTBuilder<'a>(VideoDecodeH264DpbSlotInfoEXT, std::marker::PhantomData<&'a ()>);
impl<'a> VideoDecodeH264DpbSlotInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> VideoDecodeH264DpbSlotInfoEXTBuilder<'a> {
        VideoDecodeH264DpbSlotInfoEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn std_reference_info(mut self, std_reference_info: &'a crate::external::vk_video::StdVideoDecodeH264ReferenceInfo) -> Self {
        self.0.p_std_reference_info = std_reference_info as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> VideoDecodeH264DpbSlotInfoEXT {
        self.0
    }
}
impl<'a> std::default::Default for VideoDecodeH264DpbSlotInfoEXTBuilder<'a> {
    fn default() -> VideoDecodeH264DpbSlotInfoEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for VideoDecodeH264DpbSlotInfoEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for VideoDecodeH264DpbSlotInfoEXTBuilder<'a> {
    type Target = VideoDecodeH264DpbSlotInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for VideoDecodeH264DpbSlotInfoEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoDecodeH264MvcEXT.html) · Structure"]
#[doc(alias = "VkVideoDecodeH264MvcEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VideoDecodeH264MvcEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub p_std_mvc: *const crate::external::vk_video::StdVideoDecodeH264Mvc,
}
impl VideoDecodeH264MvcEXT {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::VIDEO_DECODE_H264_MVC_EXT;
}
impl Default for VideoDecodeH264MvcEXT {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), p_std_mvc: std::ptr::null() }
    }
}
impl std::fmt::Debug for VideoDecodeH264MvcEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("VideoDecodeH264MvcEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("p_std_mvc", &self.p_std_mvc).finish()
    }
}
impl VideoDecodeH264MvcEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> VideoDecodeH264MvcEXTBuilder<'a> {
        VideoDecodeH264MvcEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoDecodeH264MvcEXT.html) · Builder of [`VideoDecodeH264MvcEXT`]"]
#[repr(transparent)]
pub struct VideoDecodeH264MvcEXTBuilder<'a>(VideoDecodeH264MvcEXT, std::marker::PhantomData<&'a ()>);
impl<'a> VideoDecodeH264MvcEXTBuilder<'a> {
    #[inline]
    pub fn new() -> VideoDecodeH264MvcEXTBuilder<'a> {
        VideoDecodeH264MvcEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn std_mvc(mut self, std_mvc: &'a crate::external::vk_video::StdVideoDecodeH264Mvc) -> Self {
        self.0.p_std_mvc = std_mvc as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> VideoDecodeH264MvcEXT {
        self.0
    }
}
impl<'a> std::default::Default for VideoDecodeH264MvcEXTBuilder<'a> {
    fn default() -> VideoDecodeH264MvcEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for VideoDecodeH264MvcEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for VideoDecodeH264MvcEXTBuilder<'a> {
    type Target = VideoDecodeH264MvcEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for VideoDecodeH264MvcEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl<'a> crate::ExtendableFrom<'a, VideoDecodeH264SessionCreateInfoEXT> for crate::extensions::khr_video_queue::VideoSessionCreateInfoKHRBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, VideoDecodeH264SessionCreateInfoEXTBuilder<'_>> for crate::extensions::khr_video_queue::VideoSessionCreateInfoKHRBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, VideoDecodeH264SessionParametersCreateInfoEXT> for crate::extensions::khr_video_queue::VideoSessionParametersCreateInfoKHRBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, VideoDecodeH264SessionParametersCreateInfoEXTBuilder<'_>> for crate::extensions::khr_video_queue::VideoSessionParametersCreateInfoKHRBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, VideoDecodeH264SessionParametersAddInfoEXT> for crate::extensions::khr_video_queue::VideoSessionParametersUpdateInfoKHRBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, VideoDecodeH264SessionParametersAddInfoEXTBuilder<'_>> for crate::extensions::khr_video_queue::VideoSessionParametersUpdateInfoKHRBuilder<'a> {}
