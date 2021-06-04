#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_NV_SHADER_SUBGROUP_PARTITIONED_SPEC_VERSION")]
pub const NV_SHADER_SUBGROUP_PARTITIONED_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_NV_SHADER_SUBGROUP_PARTITIONED_EXTENSION_NAME")]
pub const NV_SHADER_SUBGROUP_PARTITIONED_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_NV_shader_subgroup_partitioned");
#[doc = "Provided by [`crate::extensions::nv_shader_subgroup_partitioned`]"]
impl crate::vk1_1::SubgroupFeatureFlagBits {
    pub const PARTITIONED_NV: Self = Self(256);
}
