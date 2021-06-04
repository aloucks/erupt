#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_IMG_FORMAT_PVRTC_SPEC_VERSION")]
pub const IMG_FORMAT_PVRTC_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_IMG_FORMAT_PVRTC_EXTENSION_NAME")]
pub const IMG_FORMAT_PVRTC_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_IMG_format_pvrtc");
#[doc = "Provided by [`crate::extensions::img_format_pvrtc`]"]
impl crate::vk1_0::Format {
    pub const PVRTC1_2BPP_UNORM_BLOCK_IMG: Self = Self(1000054000);
    pub const PVRTC1_4BPP_UNORM_BLOCK_IMG: Self = Self(1000054001);
    pub const PVRTC2_2BPP_UNORM_BLOCK_IMG: Self = Self(1000054002);
    pub const PVRTC2_4BPP_UNORM_BLOCK_IMG: Self = Self(1000054003);
    pub const PVRTC1_2BPP_SRGB_BLOCK_IMG: Self = Self(1000054004);
    pub const PVRTC1_4BPP_SRGB_BLOCK_IMG: Self = Self(1000054005);
    pub const PVRTC2_2BPP_SRGB_BLOCK_IMG: Self = Self(1000054006);
    pub const PVRTC2_4BPP_SRGB_BLOCK_IMG: Self = Self(1000054007);
}
