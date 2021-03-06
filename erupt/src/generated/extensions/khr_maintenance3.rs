#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_MAINTENANCE_3_SPEC_VERSION")]
pub const KHR_MAINTENANCE_3_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_MAINTENANCE_3_EXTENSION_NAME")]
pub const KHR_MAINTENANCE_3_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_KHR_maintenance3");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_MAINTENANCE3_SPEC_VERSION")]
pub const KHR_MAINTENANCE3_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_MAINTENANCE3_EXTENSION_NAME")]
pub const KHR_MAINTENANCE3_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_KHR_maintenance3");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_DESCRIPTOR_SET_LAYOUT_SUPPORT_KHR: *const std::os::raw::c_char = crate::cstr!("vkGetDescriptorSetLayoutSupportKHR");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceMaintenance3PropertiesKHR.html) · Alias"]
#[doc(alias = "VkPhysicalDeviceMaintenance3PropertiesKHR")]
#[allow(non_camel_case_types)]
pub type PhysicalDeviceMaintenance3PropertiesKHR = crate::vk1_1::PhysicalDeviceMaintenance3Properties;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceMaintenance3PropertiesKHR.html) · Alias"]
#[doc(alias = "VkPhysicalDeviceMaintenance3PropertiesKHR")]
#[allow(non_camel_case_types)]
pub type PhysicalDeviceMaintenance3PropertiesKHRBuilder<'a> = crate::vk1_1::PhysicalDeviceMaintenance3PropertiesBuilder<'a>;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorSetLayoutSupportKHR.html) · Alias"]
#[doc(alias = "VkDescriptorSetLayoutSupportKHR")]
#[allow(non_camel_case_types)]
pub type DescriptorSetLayoutSupportKHR = crate::vk1_1::DescriptorSetLayoutSupport;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkDescriptorSetLayoutSupportKHR.html) · Alias"]
#[doc(alias = "VkDescriptorSetLayoutSupportKHR")]
#[allow(non_camel_case_types)]
pub type DescriptorSetLayoutSupportKHRBuilder<'a> = crate::vk1_1::DescriptorSetLayoutSupportBuilder<'a>;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDescriptorSetLayoutSupportKHR.html) · Alias"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetDescriptorSetLayoutSupportKHR = crate::vk1_1::PFN_vkGetDescriptorSetLayoutSupport;
#[doc = "Provided by [`crate::extensions::khr_maintenance3`]"]
impl crate::vk1_0::StructureType {
    pub const PHYSICAL_DEVICE_MAINTENANCE_3_PROPERTIES_KHR: Self = Self::PHYSICAL_DEVICE_MAINTENANCE_3_PROPERTIES;
    pub const DESCRIPTOR_SET_LAYOUT_SUPPORT_KHR: Self = Self::DESCRIPTOR_SET_LAYOUT_SUPPORT;
}
#[doc = "Provided by [`crate::extensions::khr_maintenance3`]"]
impl crate::DeviceLoader {
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDescriptorSetLayoutSupportKHR.html) · Function"]
    #[doc(alias = "vkGetDescriptorSetLayoutSupportKHR")]
    pub unsafe fn get_descriptor_set_layout_support_khr(&self, create_info: &crate::vk1_0::DescriptorSetLayoutCreateInfo, support: Option<crate::vk1_1::DescriptorSetLayoutSupport>) -> crate::vk1_1::DescriptorSetLayoutSupport {
        let _function = self.get_descriptor_set_layout_support_khr.expect(crate::NOT_LOADED_MESSAGE);
        let mut support = match support {
            Some(v) => v,
            None => Default::default(),
        };
        let _return = _function(self.handle, create_info as _, &mut support);
        support
    }
}
