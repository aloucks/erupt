#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_SUBGROUP_SIZE_CONTROL_SPEC_VERSION")]
pub const EXT_SUBGROUP_SIZE_CONTROL_SPEC_VERSION: u32 = 2;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_EXT_SUBGROUP_SIZE_CONTROL_EXTENSION_NAME")]
pub const EXT_SUBGROUP_SIZE_CONTROL_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_EXT_subgroup_size_control");
#[doc = "Provided by [`crate::extensions::ext_subgroup_size_control`]"]
impl crate::vk1_0::PipelineShaderStageCreateFlagBits {
    pub const ALLOW_VARYING_SUBGROUP_SIZE_EXT: Self = Self(1);
    pub const REQUIRE_FULL_SUBGROUPS_EXT: Self = Self(2);
}
#[doc = "Provided by [`crate::extensions::ext_subgroup_size_control`]"]
impl crate::vk1_0::StructureType {
    pub const PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_PROPERTIES_EXT: Self = Self(1000225000);
    pub const PIPELINE_SHADER_STAGE_REQUIRED_SUBGROUP_SIZE_CREATE_INFO_EXT: Self = Self(1000225001);
    pub const PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_FEATURES_EXT: Self = Self(1000225002);
}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceSubgroupSizeControlFeaturesEXT> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceSubgroupSizeControlFeaturesEXTBuilder<'_>> for crate::vk1_0::DeviceCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PipelineShaderStageRequiredSubgroupSizeCreateInfoEXT> for crate::vk1_0::PipelineShaderStageCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PipelineShaderStageRequiredSubgroupSizeCreateInfoEXTBuilder<'_>> for crate::vk1_0::PipelineShaderStageCreateInfoBuilder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceSubgroupSizeControlFeaturesEXT> for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceSubgroupSizeControlFeaturesEXTBuilder<'_>> for crate::vk1_1::PhysicalDeviceFeatures2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceSubgroupSizeControlPropertiesEXT> for crate::vk1_1::PhysicalDeviceProperties2Builder<'a> {}
impl<'a> crate::ExtendableFrom<'a, PhysicalDeviceSubgroupSizeControlPropertiesEXTBuilder<'_>> for crate::vk1_1::PhysicalDeviceProperties2Builder<'a> {}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceSubgroupSizeControlFeaturesEXT.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceSubgroupSizeControlFeaturesEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceSubgroupSizeControlFeaturesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub subgroup_size_control: crate::vk1_0::Bool32,
    pub compute_full_subgroups: crate::vk1_0::Bool32,
}
impl PhysicalDeviceSubgroupSizeControlFeaturesEXT {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_FEATURES_EXT;
}
impl Default for PhysicalDeviceSubgroupSizeControlFeaturesEXT {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null_mut(), subgroup_size_control: Default::default(), compute_full_subgroups: Default::default() }
    }
}
impl std::fmt::Debug for PhysicalDeviceSubgroupSizeControlFeaturesEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceSubgroupSizeControlFeaturesEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("subgroup_size_control", &(self.subgroup_size_control != 0)).field("compute_full_subgroups", &(self.compute_full_subgroups != 0)).finish()
    }
}
impl PhysicalDeviceSubgroupSizeControlFeaturesEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceSubgroupSizeControlFeaturesEXTBuilder<'a> {
        PhysicalDeviceSubgroupSizeControlFeaturesEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceSubgroupSizeControlFeaturesEXT.html) · Builder of [`PhysicalDeviceSubgroupSizeControlFeaturesEXT`]"]
#[repr(transparent)]
pub struct PhysicalDeviceSubgroupSizeControlFeaturesEXTBuilder<'a>(PhysicalDeviceSubgroupSizeControlFeaturesEXT, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceSubgroupSizeControlFeaturesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceSubgroupSizeControlFeaturesEXTBuilder<'a> {
        PhysicalDeviceSubgroupSizeControlFeaturesEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn subgroup_size_control(mut self, subgroup_size_control: bool) -> Self {
        self.0.subgroup_size_control = subgroup_size_control as _;
        self
    }
    #[inline]
    pub fn compute_full_subgroups(mut self, compute_full_subgroups: bool) -> Self {
        self.0.compute_full_subgroups = compute_full_subgroups as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> PhysicalDeviceSubgroupSizeControlFeaturesEXT {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceSubgroupSizeControlFeaturesEXTBuilder<'a> {
    fn default() -> PhysicalDeviceSubgroupSizeControlFeaturesEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceSubgroupSizeControlFeaturesEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceSubgroupSizeControlFeaturesEXTBuilder<'a> {
    type Target = PhysicalDeviceSubgroupSizeControlFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceSubgroupSizeControlFeaturesEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceSubgroupSizeControlPropertiesEXT.html) · Structure"]
#[doc(alias = "VkPhysicalDeviceSubgroupSizeControlPropertiesEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceSubgroupSizeControlPropertiesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub min_subgroup_size: u32,
    pub max_subgroup_size: u32,
    pub max_compute_workgroup_subgroups: u32,
    pub required_subgroup_size_stages: crate::vk1_0::ShaderStageFlags,
}
impl PhysicalDeviceSubgroupSizeControlPropertiesEXT {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_PROPERTIES_EXT;
}
impl Default for PhysicalDeviceSubgroupSizeControlPropertiesEXT {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null_mut(), min_subgroup_size: Default::default(), max_subgroup_size: Default::default(), max_compute_workgroup_subgroups: Default::default(), required_subgroup_size_stages: Default::default() }
    }
}
impl std::fmt::Debug for PhysicalDeviceSubgroupSizeControlPropertiesEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceSubgroupSizeControlPropertiesEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("min_subgroup_size", &self.min_subgroup_size).field("max_subgroup_size", &self.max_subgroup_size).field("max_compute_workgroup_subgroups", &self.max_compute_workgroup_subgroups).field("required_subgroup_size_stages", &self.required_subgroup_size_stages).finish()
    }
}
impl PhysicalDeviceSubgroupSizeControlPropertiesEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceSubgroupSizeControlPropertiesEXTBuilder<'a> {
        PhysicalDeviceSubgroupSizeControlPropertiesEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceSubgroupSizeControlPropertiesEXT.html) · Builder of [`PhysicalDeviceSubgroupSizeControlPropertiesEXT`]"]
#[repr(transparent)]
pub struct PhysicalDeviceSubgroupSizeControlPropertiesEXTBuilder<'a>(PhysicalDeviceSubgroupSizeControlPropertiesEXT, std::marker::PhantomData<&'a ()>);
impl<'a> PhysicalDeviceSubgroupSizeControlPropertiesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceSubgroupSizeControlPropertiesEXTBuilder<'a> {
        PhysicalDeviceSubgroupSizeControlPropertiesEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn min_subgroup_size(mut self, min_subgroup_size: u32) -> Self {
        self.0.min_subgroup_size = min_subgroup_size as _;
        self
    }
    #[inline]
    pub fn max_subgroup_size(mut self, max_subgroup_size: u32) -> Self {
        self.0.max_subgroup_size = max_subgroup_size as _;
        self
    }
    #[inline]
    pub fn max_compute_workgroup_subgroups(mut self, max_compute_workgroup_subgroups: u32) -> Self {
        self.0.max_compute_workgroup_subgroups = max_compute_workgroup_subgroups as _;
        self
    }
    #[inline]
    pub fn required_subgroup_size_stages(mut self, required_subgroup_size_stages: crate::vk1_0::ShaderStageFlags) -> Self {
        self.0.required_subgroup_size_stages = required_subgroup_size_stages as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> PhysicalDeviceSubgroupSizeControlPropertiesEXT {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceSubgroupSizeControlPropertiesEXTBuilder<'a> {
    fn default() -> PhysicalDeviceSubgroupSizeControlPropertiesEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceSubgroupSizeControlPropertiesEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceSubgroupSizeControlPropertiesEXTBuilder<'a> {
    type Target = PhysicalDeviceSubgroupSizeControlPropertiesEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceSubgroupSizeControlPropertiesEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineShaderStageRequiredSubgroupSizeCreateInfoEXT.html) · Structure"]
#[doc(alias = "VkPipelineShaderStageRequiredSubgroupSizeCreateInfoEXT")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PipelineShaderStageRequiredSubgroupSizeCreateInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub required_subgroup_size: u32,
}
impl PipelineShaderStageRequiredSubgroupSizeCreateInfoEXT {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::PIPELINE_SHADER_STAGE_REQUIRED_SUBGROUP_SIZE_CREATE_INFO_EXT;
}
impl Default for PipelineShaderStageRequiredSubgroupSizeCreateInfoEXT {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null_mut(), required_subgroup_size: Default::default() }
    }
}
impl std::fmt::Debug for PipelineShaderStageRequiredSubgroupSizeCreateInfoEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PipelineShaderStageRequiredSubgroupSizeCreateInfoEXT").field("s_type", &self.s_type).field("p_next", &self.p_next).field("required_subgroup_size", &self.required_subgroup_size).finish()
    }
}
impl PipelineShaderStageRequiredSubgroupSizeCreateInfoEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> PipelineShaderStageRequiredSubgroupSizeCreateInfoEXTBuilder<'a> {
        PipelineShaderStageRequiredSubgroupSizeCreateInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineShaderStageRequiredSubgroupSizeCreateInfoEXT.html) · Builder of [`PipelineShaderStageRequiredSubgroupSizeCreateInfoEXT`]"]
#[repr(transparent)]
pub struct PipelineShaderStageRequiredSubgroupSizeCreateInfoEXTBuilder<'a>(PipelineShaderStageRequiredSubgroupSizeCreateInfoEXT, std::marker::PhantomData<&'a ()>);
impl<'a> PipelineShaderStageRequiredSubgroupSizeCreateInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PipelineShaderStageRequiredSubgroupSizeCreateInfoEXTBuilder<'a> {
        PipelineShaderStageRequiredSubgroupSizeCreateInfoEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn required_subgroup_size(mut self, required_subgroup_size: u32) -> Self {
        self.0.required_subgroup_size = required_subgroup_size as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> PipelineShaderStageRequiredSubgroupSizeCreateInfoEXT {
        self.0
    }
}
impl<'a> std::default::Default for PipelineShaderStageRequiredSubgroupSizeCreateInfoEXTBuilder<'a> {
    fn default() -> PipelineShaderStageRequiredSubgroupSizeCreateInfoEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PipelineShaderStageRequiredSubgroupSizeCreateInfoEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PipelineShaderStageRequiredSubgroupSizeCreateInfoEXTBuilder<'a> {
    type Target = PipelineShaderStageRequiredSubgroupSizeCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PipelineShaderStageRequiredSubgroupSizeCreateInfoEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
