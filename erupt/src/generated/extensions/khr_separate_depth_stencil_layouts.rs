#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_SEPARATE_DEPTH_STENCIL_LAYOUTS_SPEC_VERSION")]
pub const KHR_SEPARATE_DEPTH_STENCIL_LAYOUTS_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_SEPARATE_DEPTH_STENCIL_LAYOUTS_EXTENSION_NAME")]
pub const KHR_SEPARATE_DEPTH_STENCIL_LAYOUTS_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_KHR_separate_depth_stencil_layouts");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceSeparateDepthStencilLayoutsFeaturesKHR.html) · Alias"]
#[doc(alias = "VkPhysicalDeviceSeparateDepthStencilLayoutsFeaturesKHR")]
#[allow(non_camel_case_types)]
pub type PhysicalDeviceSeparateDepthStencilLayoutsFeaturesKHR = crate::vk1_2::PhysicalDeviceSeparateDepthStencilLayoutsFeatures;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceSeparateDepthStencilLayoutsFeaturesKHR.html) · Alias"]
#[doc(alias = "VkPhysicalDeviceSeparateDepthStencilLayoutsFeaturesKHR")]
#[allow(non_camel_case_types)]
pub type PhysicalDeviceSeparateDepthStencilLayoutsFeaturesKHRBuilder<'a> = crate::vk1_2::PhysicalDeviceSeparateDepthStencilLayoutsFeaturesBuilder<'a>;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAttachmentReferenceStencilLayoutKHR.html) · Alias"]
#[doc(alias = "VkAttachmentReferenceStencilLayoutKHR")]
#[allow(non_camel_case_types)]
pub type AttachmentReferenceStencilLayoutKHR = crate::vk1_2::AttachmentReferenceStencilLayout;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAttachmentReferenceStencilLayoutKHR.html) · Alias"]
#[doc(alias = "VkAttachmentReferenceStencilLayoutKHR")]
#[allow(non_camel_case_types)]
pub type AttachmentReferenceStencilLayoutKHRBuilder<'a> = crate::vk1_2::AttachmentReferenceStencilLayoutBuilder<'a>;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAttachmentDescriptionStencilLayoutKHR.html) · Alias"]
#[doc(alias = "VkAttachmentDescriptionStencilLayoutKHR")]
#[allow(non_camel_case_types)]
pub type AttachmentDescriptionStencilLayoutKHR = crate::vk1_2::AttachmentDescriptionStencilLayout;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAttachmentDescriptionStencilLayoutKHR.html) · Alias"]
#[doc(alias = "VkAttachmentDescriptionStencilLayoutKHR")]
#[allow(non_camel_case_types)]
pub type AttachmentDescriptionStencilLayoutKHRBuilder<'a> = crate::vk1_2::AttachmentDescriptionStencilLayoutBuilder<'a>;
#[doc = "Provided by [`crate::extensions::khr_separate_depth_stencil_layouts`]"]
impl crate::vk1_0::ImageLayout {
    pub const DEPTH_ATTACHMENT_OPTIMAL_KHR: Self = Self::DEPTH_ATTACHMENT_OPTIMAL;
    pub const DEPTH_READ_ONLY_OPTIMAL_KHR: Self = Self::DEPTH_READ_ONLY_OPTIMAL;
    pub const STENCIL_ATTACHMENT_OPTIMAL_KHR: Self = Self::STENCIL_ATTACHMENT_OPTIMAL;
    pub const STENCIL_READ_ONLY_OPTIMAL_KHR: Self = Self::STENCIL_READ_ONLY_OPTIMAL;
}
#[doc = "Provided by [`crate::extensions::khr_separate_depth_stencil_layouts`]"]
impl crate::vk1_0::StructureType {
    pub const PHYSICAL_DEVICE_SEPARATE_DEPTH_STENCIL_LAYOUTS_FEATURES_KHR: Self = Self::PHYSICAL_DEVICE_SEPARATE_DEPTH_STENCIL_LAYOUTS_FEATURES;
    pub const ATTACHMENT_REFERENCE_STENCIL_LAYOUT_KHR: Self = Self::ATTACHMENT_REFERENCE_STENCIL_LAYOUT;
    pub const ATTACHMENT_DESCRIPTION_STENCIL_LAYOUT_KHR: Self = Self::ATTACHMENT_DESCRIPTION_STENCIL_LAYOUT;
}
