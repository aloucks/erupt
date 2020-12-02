#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_MEMORY_BUDGET_SPEC_VERSION")]
pub const EXT_MEMORY_BUDGET_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_MEMORY_BUDGET_EXTENSION_NAME")]
pub const EXT_MEMORY_BUDGET_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_EXT_memory_budget");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceMemoryBudgetPropertiesEXT.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceMemoryBudgetPropertiesEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceMemoryBudgetPropertiesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub heap_budget: [crate::vk1_0::DeviceSize; 16],
    pub heap_usage: [crate::vk1_0::DeviceSize; 16],
}
impl Default for PhysicalDeviceMemoryBudgetPropertiesEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_MEMORY_BUDGET_PROPERTIES_EXT,
            p_next: std::ptr::null_mut(),
            heap_budget: unsafe { std::mem::zeroed() },
            heap_usage: unsafe { std::mem::zeroed() },
        }
    }
}
impl std::fmt::Debug for PhysicalDeviceMemoryBudgetPropertiesEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceMemoryBudgetPropertiesEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("heap_budget", &self.heap_budget)
            .field("heap_usage", &self.heap_usage)
            .finish()
    }
}
impl PhysicalDeviceMemoryBudgetPropertiesEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceMemoryBudgetPropertiesEXTBuilder<'a> {
        PhysicalDeviceMemoryBudgetPropertiesEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceMemoryBudgetPropertiesEXT.html) · Builder of [`PhysicalDeviceMemoryBudgetPropertiesEXT`](struct.PhysicalDeviceMemoryBudgetPropertiesEXT.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceMemoryBudgetPropertiesEXTBuilder<'a>(
    PhysicalDeviceMemoryBudgetPropertiesEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceMemoryBudgetPropertiesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceMemoryBudgetPropertiesEXTBuilder<'a> {
        PhysicalDeviceMemoryBudgetPropertiesEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn heap_budget(mut self, heap_budget: [crate::vk1_0::DeviceSize; 16]) -> Self {
        self.0.heap_budget = heap_budget as _;
        self
    }
    #[inline]
    pub fn heap_usage(mut self, heap_usage: [crate::vk1_0::DeviceSize; 16]) -> Self {
        self.0.heap_usage = heap_usage as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceMemoryBudgetPropertiesEXT {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceMemoryBudgetPropertiesEXTBuilder<'a> {
    fn default() -> PhysicalDeviceMemoryBudgetPropertiesEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceMemoryBudgetPropertiesEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceMemoryBudgetPropertiesEXTBuilder<'a> {
    type Target = PhysicalDeviceMemoryBudgetPropertiesEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceMemoryBudgetPropertiesEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
