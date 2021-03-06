#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_PIPELINE_LIBRARY_SPEC_VERSION")]
pub const KHR_PIPELINE_LIBRARY_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_PIPELINE_LIBRARY_EXTENSION_NAME")]
pub const KHR_PIPELINE_LIBRARY_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_KHR_pipeline_library");
#[doc = "Provided by [`crate::extensions::khr_pipeline_library`]"]
impl crate::vk1_0::PipelineCreateFlagBits {
    pub const LIBRARY_KHR: Self = Self(2048);
}
#[doc = "Provided by [`crate::extensions::khr_pipeline_library`]"]
impl crate::vk1_0::StructureType {
    pub const PIPELINE_LIBRARY_CREATE_INFO_KHR: Self = Self(1000290000);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineLibraryCreateInfoKHR.html) · Structure"]
#[doc(alias = "VkPipelineLibraryCreateInfoKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PipelineLibraryCreateInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub library_count: u32,
    pub p_libraries: *const crate::vk1_0::Pipeline,
}
impl PipelineLibraryCreateInfoKHR {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::PIPELINE_LIBRARY_CREATE_INFO_KHR;
}
impl Default for PipelineLibraryCreateInfoKHR {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), library_count: Default::default(), p_libraries: std::ptr::null() }
    }
}
impl std::fmt::Debug for PipelineLibraryCreateInfoKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PipelineLibraryCreateInfoKHR").field("s_type", &self.s_type).field("p_next", &self.p_next).field("library_count", &self.library_count).field("p_libraries", &self.p_libraries).finish()
    }
}
impl PipelineLibraryCreateInfoKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> PipelineLibraryCreateInfoKHRBuilder<'a> {
        PipelineLibraryCreateInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineLibraryCreateInfoKHR.html) · Builder of [`PipelineLibraryCreateInfoKHR`]"]
#[repr(transparent)]
pub struct PipelineLibraryCreateInfoKHRBuilder<'a>(PipelineLibraryCreateInfoKHR, std::marker::PhantomData<&'a ()>);
impl<'a> PipelineLibraryCreateInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> PipelineLibraryCreateInfoKHRBuilder<'a> {
        PipelineLibraryCreateInfoKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn libraries(mut self, libraries: &'a [crate::vk1_0::Pipeline]) -> Self {
        self.0.p_libraries = libraries.as_ptr() as _;
        self.0.library_count = libraries.len() as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> PipelineLibraryCreateInfoKHR {
        self.0
    }
}
impl<'a> std::default::Default for PipelineLibraryCreateInfoKHRBuilder<'a> {
    fn default() -> PipelineLibraryCreateInfoKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PipelineLibraryCreateInfoKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PipelineLibraryCreateInfoKHRBuilder<'a> {
    type Target = PipelineLibraryCreateInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PipelineLibraryCreateInfoKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
