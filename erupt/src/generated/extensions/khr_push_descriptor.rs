#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_PUSH_DESCRIPTOR_SPEC_VERSION")]
pub const KHR_PUSH_DESCRIPTOR_SPEC_VERSION: u32 = 2;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_PUSH_DESCRIPTOR_EXTENSION_NAME")]
pub const KHR_PUSH_DESCRIPTOR_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_KHR_push_descriptor");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_PUSH_DESCRIPTOR_SET_KHR: *const std::os::raw::c_char = crate::cstr!("vkCmdPushDescriptorSetKHR");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_PUSH_DESCRIPTOR_SET_WITH_TEMPLATE_KHR: *const std::os::raw::c_char = crate::cstr!("vkCmdPushDescriptorSetWithTemplateKHR");
#[doc = "Provided by [`crate::extensions::khr_push_descriptor`]"]
impl crate::vk1_0::DescriptorSetLayoutCreateFlagBits {
    pub const PUSH_DESCRIPTOR_KHR: Self = Self(1);
}
#[doc = "Provided by [`crate::extensions::khr_push_descriptor`]"]
impl crate::vk1_0::StructureType {
    pub const PHYSICAL_DEVICE_PUSH_DESCRIPTOR_PROPERTIES_KHR: Self = Self(1000080000);
}
#[doc = "Provided by [`crate::extensions::khr_push_descriptor`]"]
impl crate::vk1_1::DescriptorUpdateTemplateType {
    pub const PUSH_DESCRIPTORS_KHR: Self = Self(1);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdPushDescriptorSetKHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdPushDescriptorSetKHR = unsafe extern "system" fn(command_buffer: crate::vk1_0::CommandBuffer, pipeline_bind_point: crate::vk1_0::PipelineBindPoint, layout: crate::vk1_0::PipelineLayout, set: u32, descriptor_write_count: u32, p_descriptor_writes: *const crate::vk1_0::WriteDescriptorSet) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdPushDescriptorSetWithTemplateKHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdPushDescriptorSetWithTemplateKHR = unsafe extern "system" fn(command_buffer: crate::vk1_0::CommandBuffer, descriptor_update_template: crate::vk1_1::DescriptorUpdateTemplate, layout: crate::vk1_0::PipelineLayout, set: u32, p_data: *const std::ffi::c_void) -> ();
impl<'a> crate::ExtendableFromMut<'a, PhysicalDevicePushDescriptorPropertiesKHR> for crate::vk1_1::PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFromMut<'a, PhysicalDevicePushDescriptorPropertiesKHRBuilder<'_>> for crate::vk1_1::PhysicalDeviceProperties2Builder<'a> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDevicePushDescriptorPropertiesKHR.html) · Structure"]
#[doc(alias = "VkPhysicalDevicePushDescriptorPropertiesKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDevicePushDescriptorPropertiesKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub max_push_descriptors: u32,
}
impl Default for PhysicalDevicePushDescriptorPropertiesKHR {
    fn default() -> Self {
        Self { s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_PUSH_DESCRIPTOR_PROPERTIES_KHR, p_next: std::ptr::null_mut(), max_push_descriptors: Default::default() }
    }
}
impl std::fmt::Debug for PhysicalDevicePushDescriptorPropertiesKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDevicePushDescriptorPropertiesKHR").field("s_type", &self.s_type).field("p_next", &self.p_next).field("max_push_descriptors", &self.max_push_descriptors).finish()
    }
}
impl PhysicalDevicePushDescriptorPropertiesKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDevicePushDescriptorPropertiesKHRBuilder<'a> {
        PhysicalDevicePushDescriptorPropertiesKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDevicePushDescriptorPropertiesKHR.html) · Builder of [`PhysicalDevicePushDescriptorPropertiesKHR`]"]
#[repr(transparent)]
pub struct PhysicalDevicePushDescriptorPropertiesKHRBuilder<'a>(PhysicalDevicePushDescriptorPropertiesKHR, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDevicePushDescriptorPropertiesKHRBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDevicePushDescriptorPropertiesKHRBuilder<'a> {
        PhysicalDevicePushDescriptorPropertiesKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn max_push_descriptors(mut self, max_push_descriptors: u32) -> Self {
        self.0.max_push_descriptors = max_push_descriptors as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDevicePushDescriptorPropertiesKHR {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDevicePushDescriptorPropertiesKHRBuilder<'a> {
    fn default() -> PhysicalDevicePushDescriptorPropertiesKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDevicePushDescriptorPropertiesKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDevicePushDescriptorPropertiesKHRBuilder<'a> {
    type Target = PhysicalDevicePushDescriptorPropertiesKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDevicePushDescriptorPropertiesKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "Provided by [`crate::extensions::khr_push_descriptor`]"]
impl crate::DeviceLoader {
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdPushDescriptorSetKHR.html) · Function"]
    #[doc(alias = "vkCmdPushDescriptorSetKHR")]
    pub unsafe fn cmd_push_descriptor_set_khr(&self, command_buffer: crate::vk1_0::CommandBuffer, pipeline_bind_point: crate::vk1_0::PipelineBindPoint, layout: crate::vk1_0::PipelineLayout, set: u32, descriptor_writes: &[crate::vk1_0::WriteDescriptorSetBuilder]) -> () {
        let _function = self.cmd_push_descriptor_set_khr.expect("tried to call a function that isn't loaded");
        let descriptor_write_count = descriptor_writes.len();
        let _return = _function(command_buffer as _, pipeline_bind_point as _, layout as _, set as _, descriptor_write_count as _, descriptor_writes.as_ptr() as _);
        ()
    }
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdPushDescriptorSetWithTemplateKHR.html) · Function"]
    #[doc(alias = "vkCmdPushDescriptorSetWithTemplateKHR")]
    pub unsafe fn cmd_push_descriptor_set_with_template_khr(&self, command_buffer: crate::vk1_0::CommandBuffer, descriptor_update_template: crate::vk1_1::DescriptorUpdateTemplate, layout: crate::vk1_0::PipelineLayout, set: u32, data: *const std::ffi::c_void) -> () {
        let _function = self.cmd_push_descriptor_set_with_template_khr.expect("tried to call a function that isn't loaded");
        let _return = _function(command_buffer as _, descriptor_update_template as _, layout as _, set as _, data);
        ()
    }
}
