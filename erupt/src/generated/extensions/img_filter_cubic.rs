#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_IMG_FILTER_CUBIC_SPEC_VERSION")]
pub const IMG_FILTER_CUBIC_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_IMG_FILTER_CUBIC_EXTENSION_NAME")]
pub const IMG_FILTER_CUBIC_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_IMG_filter_cubic");
#[doc = "Provided by [`crate::extensions::img_filter_cubic`]"]
impl crate::vk1_0::FormatFeatureFlagBits {
    pub const SAMPLED_IMAGE_FILTER_CUBIC_IMG: Self = Self(8192);
}
#[doc = "Provided by [`crate::extensions::img_filter_cubic`]"]
impl crate::vk1_0::Filter {
    pub const CUBIC_IMG: Self = Self(1000015000);
}
