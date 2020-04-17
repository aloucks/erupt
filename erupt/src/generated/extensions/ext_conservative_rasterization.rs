# ! [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_EXT_conservative_rasterization.html)\n\n## Extends\n- [`StructureType`](../../vk1_0/struct.StructureType.html)" ]#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_CONSERVATIVE_RASTERIZATION_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_CONSERVATIVE_RASTERIZATION_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_EXT_conservative_rasterization");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceConservativeRasterizationPropertiesEXT.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceConservativeRasterizationPropertiesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub primitive_overestimation_size: f32,
    pub max_extra_primitive_overestimation_size: f32,
    pub extra_primitive_overestimation_size_granularity: f32,
    pub primitive_underestimation: crate::vk1_0::Bool32,
    pub conservative_point_and_line_rasterization: crate::vk1_0::Bool32,
    pub degenerate_triangles_rasterized: crate::vk1_0::Bool32,
    pub degenerate_lines_rasterized: crate::vk1_0::Bool32,
    pub fully_covered_fragment_shader_input_variable: crate::vk1_0::Bool32,
    pub conservative_rasterization_post_depth_coverage: crate::vk1_0::Bool32,
}
impl PhysicalDeviceConservativeRasterizationPropertiesEXT {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByPhysicalDeviceConservativeRasterizationPropertiesEXT,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> PhysicalDeviceConservativeRasterizationPropertiesEXTBuilder<'a> {
        PhysicalDeviceConservativeRasterizationPropertiesEXTBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PhysicalDeviceConservativeRasterizationPropertiesEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PhysicalDeviceConservativeRasterizationPropertiesEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field(
                "primitive_overestimation_size",
                &self.primitive_overestimation_size,
            )
            .field(
                "max_extra_primitive_overestimation_size",
                &self.max_extra_primitive_overestimation_size,
            )
            .field(
                "extra_primitive_overestimation_size_granularity",
                &self.extra_primitive_overestimation_size_granularity,
            )
            .field(
                "primitive_underestimation",
                &(self.primitive_underestimation != 0),
            )
            .field(
                "conservative_point_and_line_rasterization",
                &(self.conservative_point_and_line_rasterization != 0),
            )
            .field(
                "degenerate_triangles_rasterized",
                &(self.degenerate_triangles_rasterized != 0),
            )
            .field(
                "degenerate_lines_rasterized",
                &(self.degenerate_lines_rasterized != 0),
            )
            .field(
                "fully_covered_fragment_shader_input_variable",
                &(self.fully_covered_fragment_shader_input_variable != 0),
            )
            .field(
                "conservative_rasterization_post_depth_coverage",
                &(self.conservative_rasterization_post_depth_coverage != 0),
            )
            .finish()
    }
}
impl Default for PhysicalDeviceConservativeRasterizationPropertiesEXT {
    fn default() -> PhysicalDeviceConservativeRasterizationPropertiesEXT {
        PhysicalDeviceConservativeRasterizationPropertiesEXT { s_type : crate :: vk1_0 :: StructureType :: PHYSICAL_DEVICE_CONSERVATIVE_RASTERIZATION_PROPERTIES_EXT , p_next : std :: ptr :: null_mut ( ) , primitive_overestimation_size : Default :: default ( ) , max_extra_primitive_overestimation_size : Default :: default ( ) , extra_primitive_overestimation_size_granularity : Default :: default ( ) , primitive_underestimation : Default :: default ( ) , conservative_point_and_line_rasterization : Default :: default ( ) , degenerate_triangles_rasterized : Default :: default ( ) , degenerate_lines_rasterized : Default :: default ( ) , fully_covered_fragment_shader_input_variable : Default :: default ( ) , conservative_rasterization_post_depth_coverage : Default :: default ( ) , }
    }
}
#[doc = "Used by [`PhysicalDeviceConservativeRasterizationPropertiesEXT::extend`](struct.PhysicalDeviceConservativeRasterizationPropertiesEXT.html#method.extend)"]
pub trait ExtendableByPhysicalDeviceConservativeRasterizationPropertiesEXT {}
impl ExtendableByPhysicalDeviceConservativeRasterizationPropertiesEXT
    for crate::vk1_1::PhysicalDeviceProperties2
{
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PhysicalDeviceConservativeRasterizationPropertiesEXT`](struct.PhysicalDeviceConservativeRasterizationPropertiesEXT.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceConservativeRasterizationPropertiesEXTBuilder<'a>(
    PhysicalDeviceConservativeRasterizationPropertiesEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceConservativeRasterizationPropertiesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceConservativeRasterizationPropertiesEXTBuilder<'a> {
        PhysicalDeviceConservativeRasterizationPropertiesEXTBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn primitive_overestimation_size(mut self, primitive_overestimation_size: f32) -> Self {
        self.0.primitive_overestimation_size = primitive_overestimation_size;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn max_extra_primitive_overestimation_size(
        mut self,
        max_extra_primitive_overestimation_size: f32,
    ) -> Self {
        self.0.max_extra_primitive_overestimation_size = max_extra_primitive_overestimation_size;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn extra_primitive_overestimation_size_granularity(
        mut self,
        extra_primitive_overestimation_size_granularity: f32,
    ) -> Self {
        self.0.extra_primitive_overestimation_size_granularity =
            extra_primitive_overestimation_size_granularity;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn primitive_underestimation(mut self, primitive_underestimation: bool) -> Self {
        self.0.primitive_underestimation = primitive_underestimation as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn conservative_point_and_line_rasterization(
        mut self,
        conservative_point_and_line_rasterization: bool,
    ) -> Self {
        self.0.conservative_point_and_line_rasterization =
            conservative_point_and_line_rasterization as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn degenerate_triangles_rasterized(
        mut self,
        degenerate_triangles_rasterized: bool,
    ) -> Self {
        self.0.degenerate_triangles_rasterized = degenerate_triangles_rasterized as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn degenerate_lines_rasterized(mut self, degenerate_lines_rasterized: bool) -> Self {
        self.0.degenerate_lines_rasterized = degenerate_lines_rasterized as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn fully_covered_fragment_shader_input_variable(
        mut self,
        fully_covered_fragment_shader_input_variable: bool,
    ) -> Self {
        self.0.fully_covered_fragment_shader_input_variable =
            fully_covered_fragment_shader_input_variable as u32;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn conservative_rasterization_post_depth_coverage(
        mut self,
        conservative_rasterization_post_depth_coverage: bool,
    ) -> Self {
        self.0.conservative_rasterization_post_depth_coverage =
            conservative_rasterization_post_depth_coverage as u32;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PhysicalDeviceConservativeRasterizationPropertiesEXT {
        self.0
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceConservativeRasterizationPropertiesEXTBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceConservativeRasterizationPropertiesEXTBuilder<'a> {
    type Target = PhysicalDeviceConservativeRasterizationPropertiesEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceConservativeRasterizationPropertiesEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineRasterizationConservativeStateCreateInfoEXT.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PipelineRasterizationConservativeStateCreateInfoEXT { pub s_type : crate :: vk1_0 :: StructureType , pub p_next : * const std :: ffi :: c_void , pub flags : crate :: extensions :: ext_conservative_rasterization :: PipelineRasterizationConservativeStateCreateFlagsEXT , pub conservative_rasterization_mode : crate :: extensions :: ext_conservative_rasterization :: ConservativeRasterizationModeEXT , pub extra_primitive_overestimation_size : f32 , }
impl PipelineRasterizationConservativeStateCreateInfoEXT {
    #[inline]
    #[doc = "Appends `self` to `other` pointer chain"]
    pub unsafe fn extend<T>(&mut self, other: &mut T)
    where
        T: ExtendableByPipelineRasterizationConservativeStateCreateInfoEXT,
    {
        crate::append_ptr_chain(other as *mut T as _, self as *mut Self as _);
    }
    #[inline]
    pub fn builder<'a>(self) -> PipelineRasterizationConservativeStateCreateInfoEXTBuilder<'a> {
        PipelineRasterizationConservativeStateCreateInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
impl std::fmt::Debug for PipelineRasterizationConservativeStateCreateInfoEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("PipelineRasterizationConservativeStateCreateInfoEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("flags", &self.flags)
            .field(
                "conservative_rasterization_mode",
                &self.conservative_rasterization_mode,
            )
            .field(
                "extra_primitive_overestimation_size",
                &self.extra_primitive_overestimation_size,
            )
            .finish()
    }
}
impl Default for PipelineRasterizationConservativeStateCreateInfoEXT {
    fn default() -> PipelineRasterizationConservativeStateCreateInfoEXT {
        PipelineRasterizationConservativeStateCreateInfoEXT { s_type : crate :: vk1_0 :: StructureType :: PIPELINE_RASTERIZATION_CONSERVATIVE_STATE_CREATE_INFO_EXT , p_next : std :: ptr :: null ( ) , flags : Default :: default ( ) , conservative_rasterization_mode : Default :: default ( ) , extra_primitive_overestimation_size : Default :: default ( ) , }
    }
}
#[doc = "Used by [`PipelineRasterizationConservativeStateCreateInfoEXT::extend`](struct.PipelineRasterizationConservativeStateCreateInfoEXT.html#method.extend)"]
pub trait ExtendableByPipelineRasterizationConservativeStateCreateInfoEXT {}
impl ExtendableByPipelineRasterizationConservativeStateCreateInfoEXT
    for crate::vk1_0::PipelineRasterizationStateCreateInfo
{
}
#[derive(Copy, Clone)]
#[doc = "Builder of [`PipelineRasterizationConservativeStateCreateInfoEXT`](struct.PipelineRasterizationConservativeStateCreateInfoEXT.html)"]
#[repr(transparent)]
pub struct PipelineRasterizationConservativeStateCreateInfoEXTBuilder<'a>(
    PipelineRasterizationConservativeStateCreateInfoEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PipelineRasterizationConservativeStateCreateInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PipelineRasterizationConservativeStateCreateInfoEXTBuilder<'a> {
        PipelineRasterizationConservativeStateCreateInfoEXTBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn flags(
        mut self,
        flags : crate :: extensions :: ext_conservative_rasterization :: PipelineRasterizationConservativeStateCreateFlagsEXT,
    ) -> Self {
        self.0.flags = flags;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn conservative_rasterization_mode(
        mut self,
        conservative_rasterization_mode : crate :: extensions :: ext_conservative_rasterization :: ConservativeRasterizationModeEXT,
    ) -> Self {
        self.0.conservative_rasterization_mode = conservative_rasterization_mode;
        self
    }
    #[allow(unused_mut)]
    #[inline]
    pub fn extra_primitive_overestimation_size(
        mut self,
        extra_primitive_overestimation_size: f32,
    ) -> Self {
        self.0.extra_primitive_overestimation_size = extra_primitive_overestimation_size;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub unsafe fn discard(self) -> PipelineRasterizationConservativeStateCreateInfoEXT {
        self.0
    }
}
impl<'a> std::fmt::Debug for PipelineRasterizationConservativeStateCreateInfoEXTBuilder<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, fmt)
    }
}
impl<'a> std::ops::Deref for PipelineRasterizationConservativeStateCreateInfoEXTBuilder<'a> {
    type Target = PipelineRasterizationConservativeStateCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PipelineRasterizationConservativeStateCreateInfoEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "<s>Vulkan Manual Page</s> · Flag Bits of [`PipelineRasterizationConservativeStateCreateFlagsEXT`](struct.PipelineRasterizationConservativeStateCreateFlagsEXT.html)"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct PipelineRasterizationConservativeStateCreateFlagBitsEXT(pub u32);
impl PipelineRasterizationConservativeStateCreateFlagBitsEXT {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> PipelineRasterizationConservativeStateCreateFlagsEXT {
        PipelineRasterizationConservativeStateCreateFlagsEXT::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for PipelineRasterizationConservativeStateCreateFlagBitsEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            _ => "Unknown enum variant",
        })
    }
}
bitflags::bitflags! { # [ doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineRasterizationConservativeStateCreateFlagsEXT.html) · Flags of [`PipelineRasterizationConservativeStateCreateFlagBitsEXT`](struct.PipelineRasterizationConservativeStateCreateFlagBitsEXT.html)" ] # [ derive ( Default ) ] # [ repr ( transparent ) ] pub struct PipelineRasterizationConservativeStateCreateFlagsEXT : u32 { # [ cfg ( empty_bitflag_workaround ) ] const EMPTY_BITFLAG_WORKAROUND = 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkConservativeRasterizationModeEXT.html) · Enum"]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ConservativeRasterizationModeEXT(pub i32);
#[doc = "[Part of `extensions::ext_conservative_rasterization`](../../extensions/ext_conservative_rasterization/index.html)"]
impl ConservativeRasterizationModeEXT {
    pub const DISABLED_EXT: Self = Self(0);
    pub const OVERESTIMATE_EXT: Self = Self(1);
    pub const UNDERESTIMATE_EXT: Self = Self(2);
}
impl std::fmt::Debug for ConservativeRasterizationModeEXT {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            &Self::DISABLED_EXT => "DISABLED_EXT",
            &Self::OVERESTIMATE_EXT => "OVERESTIMATE_EXT",
            &Self::UNDERESTIMATE_EXT => "UNDERESTIMATE_EXT",
            _ => "Unknown enum variant",
        })
    }
}
