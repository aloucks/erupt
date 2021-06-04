#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_EXTERNAL_FENCE_SPEC_VERSION")]
pub const KHR_EXTERNAL_FENCE_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_EXTERNAL_FENCE_EXTENSION_NAME")]
pub const KHR_EXTERNAL_FENCE_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_KHR_external_fence");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkFenceImportFlagsKHR.html) · Alias"]
#[doc(alias = "VkFenceImportFlagsKHR")]
#[allow(non_camel_case_types)]
pub type FenceImportFlagsKHR = crate::vk1_1::FenceImportFlags;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkFenceImportFlagBitsKHR.html) · Alias"]
#[doc(alias = "VkFenceImportFlagBitsKHR")]
#[allow(non_camel_case_types)]
pub type FenceImportFlagBitsKHR = crate::vk1_1::FenceImportFlagBits;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExportFenceCreateInfoKHR.html) · Alias"]
#[doc(alias = "VkExportFenceCreateInfoKHR")]
#[allow(non_camel_case_types)]
pub type ExportFenceCreateInfoKHR = crate::vk1_1::ExportFenceCreateInfo;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkExportFenceCreateInfoKHR.html) · Alias"]
#[doc(alias = "VkExportFenceCreateInfoKHR")]
#[allow(non_camel_case_types)]
pub type ExportFenceCreateInfoKHRBuilder<'a> = crate::vk1_1::ExportFenceCreateInfoBuilder<'a>;
#[doc = "Provided by [`crate::extensions::khr_external_fence`]"]
impl crate::vk1_0::StructureType {
    pub const EXPORT_FENCE_CREATE_INFO_KHR: Self = Self::EXPORT_FENCE_CREATE_INFO;
}
#[doc = "Provided by [`crate::extensions::khr_external_fence`]"]
impl crate::vk1_1::FenceImportFlagBits {
    pub const TEMPORARY_KHR: Self = Self::TEMPORARY;
}
