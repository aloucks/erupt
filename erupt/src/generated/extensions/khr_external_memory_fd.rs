#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_EXTERNAL_MEMORY_FD_SPEC_VERSION")]
pub const KHR_EXTERNAL_MEMORY_FD_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_EXTERNAL_MEMORY_FD_EXTENSION_NAME")]
pub const KHR_EXTERNAL_MEMORY_FD_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_KHR_external_memory_fd");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_MEMORY_FD_KHR: *const std::os::raw::c_char = crate::cstr!("vkGetMemoryFdKHR");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_MEMORY_FD_PROPERTIES_KHR: *const std::os::raw::c_char =
    crate::cstr!("vkGetMemoryFdPropertiesKHR");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetMemoryFdKHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetMemoryFdKHR = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    p_get_fd_info: *const crate::extensions::khr_external_memory_fd::MemoryGetFdInfoKHR,
    p_fd: *mut std::os::raw::c_int,
) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetMemoryFdPropertiesKHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetMemoryFdPropertiesKHR = unsafe extern "system" fn(
    device: crate::vk1_0::Device,
    handle_type: crate::vk1_1::ExternalMemoryHandleTypeFlagBits,
    fd: std::os::raw::c_int,
    p_memory_fd_properties: *mut crate::extensions::khr_external_memory_fd::MemoryFdPropertiesKHR,
) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImportMemoryFdInfoKHR.html) · Structure"]
#[doc(alias = "VkImportMemoryFdInfoKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ImportMemoryFdInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub handle_type: crate::vk1_1::ExternalMemoryHandleTypeFlagBits,
    pub fd: std::os::raw::c_int,
}
impl Default for ImportMemoryFdInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::IMPORT_MEMORY_FD_INFO_KHR,
            p_next: std::ptr::null(),
            handle_type: Default::default(),
            fd: Default::default(),
        }
    }
}
impl std::fmt::Debug for ImportMemoryFdInfoKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ImportMemoryFdInfoKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("handle_type", &self.handle_type)
            .field("fd", &self.fd)
            .finish()
    }
}
impl ImportMemoryFdInfoKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> ImportMemoryFdInfoKHRBuilder<'a> {
        ImportMemoryFdInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkImportMemoryFdInfoKHR.html) · Builder of [`ImportMemoryFdInfoKHR`]"]
#[repr(transparent)]
pub struct ImportMemoryFdInfoKHRBuilder<'a>(
    ImportMemoryFdInfoKHR,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> ImportMemoryFdInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> ImportMemoryFdInfoKHRBuilder<'a> {
        ImportMemoryFdInfoKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn handle_type(
        mut self,
        handle_type: crate::vk1_1::ExternalMemoryHandleTypeFlagBits,
    ) -> Self {
        self.0.handle_type = handle_type as _;
        self
    }
    #[inline]
    pub fn fd(mut self, fd: std::os::raw::c_int) -> Self {
        self.0.fd = fd as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> ImportMemoryFdInfoKHR {
        self.0
    }
}
impl<'a> std::default::Default for ImportMemoryFdInfoKHRBuilder<'a> {
    fn default() -> ImportMemoryFdInfoKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for ImportMemoryFdInfoKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for ImportMemoryFdInfoKHRBuilder<'a> {
    type Target = ImportMemoryFdInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for ImportMemoryFdInfoKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryFdPropertiesKHR.html) · Structure"]
#[doc(alias = "VkMemoryFdPropertiesKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MemoryFdPropertiesKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub memory_type_bits: u32,
}
impl Default for MemoryFdPropertiesKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::MEMORY_FD_PROPERTIES_KHR,
            p_next: std::ptr::null_mut(),
            memory_type_bits: Default::default(),
        }
    }
}
impl std::fmt::Debug for MemoryFdPropertiesKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("MemoryFdPropertiesKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("memory_type_bits", &self.memory_type_bits)
            .finish()
    }
}
impl MemoryFdPropertiesKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> MemoryFdPropertiesKHRBuilder<'a> {
        MemoryFdPropertiesKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryFdPropertiesKHR.html) · Builder of [`MemoryFdPropertiesKHR`]"]
#[repr(transparent)]
pub struct MemoryFdPropertiesKHRBuilder<'a>(
    MemoryFdPropertiesKHR,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> MemoryFdPropertiesKHRBuilder<'a> {
    #[inline]
    pub fn new() -> MemoryFdPropertiesKHRBuilder<'a> {
        MemoryFdPropertiesKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn memory_type_bits(mut self, memory_type_bits: u32) -> Self {
        self.0.memory_type_bits = memory_type_bits as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> MemoryFdPropertiesKHR {
        self.0
    }
}
impl<'a> std::default::Default for MemoryFdPropertiesKHRBuilder<'a> {
    fn default() -> MemoryFdPropertiesKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for MemoryFdPropertiesKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for MemoryFdPropertiesKHRBuilder<'a> {
    type Target = MemoryFdPropertiesKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for MemoryFdPropertiesKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryGetFdInfoKHR.html) · Structure"]
#[doc(alias = "VkMemoryGetFdInfoKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MemoryGetFdInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub memory: crate::vk1_0::DeviceMemory,
    pub handle_type: crate::vk1_1::ExternalMemoryHandleTypeFlagBits,
}
impl Default for MemoryGetFdInfoKHR {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::MEMORY_GET_FD_INFO_KHR,
            p_next: std::ptr::null(),
            memory: Default::default(),
            handle_type: Default::default(),
        }
    }
}
impl std::fmt::Debug for MemoryGetFdInfoKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("MemoryGetFdInfoKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("memory", &self.memory)
            .field("handle_type", &self.handle_type)
            .finish()
    }
}
impl MemoryGetFdInfoKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> MemoryGetFdInfoKHRBuilder<'a> {
        MemoryGetFdInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMemoryGetFdInfoKHR.html) · Builder of [`MemoryGetFdInfoKHR`]"]
#[repr(transparent)]
pub struct MemoryGetFdInfoKHRBuilder<'a>(MemoryGetFdInfoKHR, std::marker::PhantomData<&'a ()>);
impl<'a> MemoryGetFdInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> MemoryGetFdInfoKHRBuilder<'a> {
        MemoryGetFdInfoKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn memory(mut self, memory: crate::vk1_0::DeviceMemory) -> Self {
        self.0.memory = memory as _;
        self
    }
    #[inline]
    pub fn handle_type(
        mut self,
        handle_type: crate::vk1_1::ExternalMemoryHandleTypeFlagBits,
    ) -> Self {
        self.0.handle_type = handle_type as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> MemoryGetFdInfoKHR {
        self.0
    }
}
impl<'a> std::default::Default for MemoryGetFdInfoKHRBuilder<'a> {
    fn default() -> MemoryGetFdInfoKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for MemoryGetFdInfoKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for MemoryGetFdInfoKHRBuilder<'a> {
    type Target = MemoryGetFdInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for MemoryGetFdInfoKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "Provided by [`crate::extensions::khr_external_memory_fd`]"]
impl crate::DeviceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetMemoryFdKHR.html) · Function"]
    #[doc(alias = "vkGetMemoryFdKHR")]
    pub unsafe fn get_memory_fd_khr(
        &self,
        get_fd_info: &crate::extensions::khr_external_memory_fd::MemoryGetFdInfoKHR,
        fd: Option<std::os::raw::c_int>,
    ) -> crate::utils::VulkanResult<std::os::raw::c_int> {
        let _function = self
            .get_memory_fd_khr
            .expect("`get_memory_fd_khr` is not loaded");
        let mut fd = match fd {
            Some(v) => v,
            None => Default::default(),
        };
        let _return = _function(self.handle, get_fd_info as _, &mut fd);
        crate::utils::VulkanResult::new(_return, fd)
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetMemoryFdPropertiesKHR.html) · Function"]
    #[doc(alias = "vkGetMemoryFdPropertiesKHR")]
    pub unsafe fn get_memory_fd_properties_khr(
        &self,
        handle_type: crate::vk1_1::ExternalMemoryHandleTypeFlagBits,
        fd: std::os::raw::c_int,
        memory_fd_properties: Option<
            crate::extensions::khr_external_memory_fd::MemoryFdPropertiesKHR,
        >,
    ) -> crate::utils::VulkanResult<crate::extensions::khr_external_memory_fd::MemoryFdPropertiesKHR>
    {
        let _function = self
            .get_memory_fd_properties_khr
            .expect("`get_memory_fd_properties_khr` is not loaded");
        let mut memory_fd_properties = match memory_fd_properties {
            Some(v) => v,
            None => Default::default(),
        };
        let _return = _function(
            self.handle,
            handle_type as _,
            fd as _,
            &mut memory_fd_properties,
        );
        crate::utils::VulkanResult::new(_return, memory_fd_properties)
    }
}
