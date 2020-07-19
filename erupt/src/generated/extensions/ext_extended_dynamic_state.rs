#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_EXTENDED_DYNAMIC_STATE_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_EXTENDED_DYNAMIC_STATE_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_EXT_extended_dynamic_state");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_SET_CULL_MODE_EXT: *const std::os::raw::c_char =
    crate::cstr!("vkCmdSetCullModeEXT");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_SET_FRONT_FACE_EXT: *const std::os::raw::c_char =
    crate::cstr!("vkCmdSetFrontFaceEXT");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_SET_PRIMITIVE_TOPOLOGY_EXT: *const std::os::raw::c_char =
    crate::cstr!("vkCmdSetPrimitiveTopologyEXT");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_SET_VIEWPORT_WITH_COUNT_EXT: *const std::os::raw::c_char =
    crate::cstr!("vkCmdSetViewportWithCountEXT");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_SET_SCISSOR_WITH_COUNT_EXT: *const std::os::raw::c_char =
    crate::cstr!("vkCmdSetScissorWithCountEXT");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_BIND_VERTEX_BUFFERS2_EXT: *const std::os::raw::c_char =
    crate::cstr!("vkCmdBindVertexBuffers2EXT");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_SET_DEPTH_TEST_ENABLE_EXT: *const std::os::raw::c_char =
    crate::cstr!("vkCmdSetDepthTestEnableEXT");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_SET_DEPTH_WRITE_ENABLE_EXT: *const std::os::raw::c_char =
    crate::cstr!("vkCmdSetDepthWriteEnableEXT");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_SET_DEPTH_COMPARE_OP_EXT: *const std::os::raw::c_char =
    crate::cstr!("vkCmdSetDepthCompareOpEXT");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_SET_DEPTH_BOUNDS_TEST_ENABLE_EXT: *const std::os::raw::c_char =
    crate::cstr!("vkCmdSetDepthBoundsTestEnableEXT");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_SET_STENCIL_TEST_ENABLE_EXT: *const std::os::raw::c_char =
    crate::cstr!("vkCmdSetStencilTestEnableEXT");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_SET_STENCIL_OP_EXT: *const std::os::raw::c_char =
    crate::cstr!("vkCmdSetStencilOpEXT");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetCullModeEXT.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetCullModeEXT = unsafe extern "system" fn(
    command_buffer: crate::vk1_0::CommandBuffer,
    cull_mode: crate::vk1_0::CullModeFlags,
) -> std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetFrontFaceEXT.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetFrontFaceEXT = unsafe extern "system" fn(
    command_buffer: crate::vk1_0::CommandBuffer,
    front_face: crate::vk1_0::FrontFace,
) -> std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetPrimitiveTopologyEXT.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetPrimitiveTopologyEXT = unsafe extern "system" fn(
    command_buffer: crate::vk1_0::CommandBuffer,
    primitive_topology: crate::vk1_0::PrimitiveTopology,
) -> std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetViewportWithCountEXT.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetViewportWithCountEXT = unsafe extern "system" fn(
    command_buffer: crate::vk1_0::CommandBuffer,
    viewport_count: u32,
    p_viewports: *const crate::vk1_0::Viewport,
) -> std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetScissorWithCountEXT.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetScissorWithCountEXT = unsafe extern "system" fn(
    command_buffer: crate::vk1_0::CommandBuffer,
    scissor_count: u32,
    p_scissors: *const crate::vk1_0::Rect2D,
) -> std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBindVertexBuffers2EXT.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBindVertexBuffers2EXT = unsafe extern "system" fn(
    command_buffer: crate::vk1_0::CommandBuffer,
    first_binding: u32,
    binding_count: u32,
    p_buffers: *const crate::vk1_0::Buffer,
    p_offsets: *const crate::vk1_0::DeviceSize,
    p_sizes: *const crate::vk1_0::DeviceSize,
    p_strides: *const crate::vk1_0::DeviceSize,
) -> std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetDepthTestEnableEXT.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetDepthTestEnableEXT = unsafe extern "system" fn(
    command_buffer: crate::vk1_0::CommandBuffer,
    depth_test_enable: crate::vk1_0::Bool32,
) -> std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetDepthWriteEnableEXT.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetDepthWriteEnableEXT = unsafe extern "system" fn(
    command_buffer: crate::vk1_0::CommandBuffer,
    depth_write_enable: crate::vk1_0::Bool32,
) -> std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetDepthCompareOpEXT.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetDepthCompareOpEXT = unsafe extern "system" fn(
    command_buffer: crate::vk1_0::CommandBuffer,
    depth_compare_op: crate::vk1_0::CompareOp,
) -> std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetDepthBoundsTestEnableEXT.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetDepthBoundsTestEnableEXT = unsafe extern "system" fn(
    command_buffer: crate::vk1_0::CommandBuffer,
    depth_bounds_test_enable: crate::vk1_0::Bool32,
) -> std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetStencilTestEnableEXT.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetStencilTestEnableEXT = unsafe extern "system" fn(
    command_buffer: crate::vk1_0::CommandBuffer,
    stencil_test_enable: crate::vk1_0::Bool32,
) -> std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetStencilOpEXT.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetStencilOpEXT = unsafe extern "system" fn(
    command_buffer: crate::vk1_0::CommandBuffer,
    face_mask: crate::vk1_0::StencilFaceFlags,
    fail_op: crate::vk1_0::StencilOp,
    pass_op: crate::vk1_0::StencilOp,
    depth_fail_op: crate::vk1_0::StencilOp,
    compare_op: crate::vk1_0::CompareOp,
) -> std::ffi::c_void;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceExtendedDynamicStateFeaturesEXT.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceExtendedDynamicStateFeaturesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub extended_dynamic_state: crate::vk1_0::Bool32,
}
impl Default for PhysicalDeviceExtendedDynamicStateFeaturesEXT {
    fn default() -> Self {
        Self {
            s_type:
                crate::vk1_0::StructureType::PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            extended_dynamic_state: Default::default(),
        }
    }
}
impl std::fmt::Debug for PhysicalDeviceExtendedDynamicStateFeaturesEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceExtendedDynamicStateFeaturesEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field(
                "extended_dynamic_state",
                &(self.extended_dynamic_state != 0),
            )
            .finish()
    }
}
impl PhysicalDeviceExtendedDynamicStateFeaturesEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceExtendedDynamicStateFeaturesEXTBuilder<'a> {
        PhysicalDeviceExtendedDynamicStateFeaturesEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceExtendedDynamicStateFeaturesEXT.html) · Builder of [`PhysicalDeviceExtendedDynamicStateFeaturesEXT`](struct.PhysicalDeviceExtendedDynamicStateFeaturesEXT.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceExtendedDynamicStateFeaturesEXTBuilder<'a>(
    PhysicalDeviceExtendedDynamicStateFeaturesEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceExtendedDynamicStateFeaturesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceExtendedDynamicStateFeaturesEXTBuilder<'a> {
        PhysicalDeviceExtendedDynamicStateFeaturesEXTBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[inline]
    pub fn extended_dynamic_state(mut self, extended_dynamic_state: bool) -> Self {
        self.0.extended_dynamic_state = extended_dynamic_state as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceExtendedDynamicStateFeaturesEXT {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceExtendedDynamicStateFeaturesEXTBuilder<'a> {
    fn default() -> PhysicalDeviceExtendedDynamicStateFeaturesEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceExtendedDynamicStateFeaturesEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceExtendedDynamicStateFeaturesEXTBuilder<'a> {
    type Target = PhysicalDeviceExtendedDynamicStateFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceExtendedDynamicStateFeaturesEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "Provided by [`extensions::ext_extended_dynamic_state`](extensions/ext_extended_dynamic_state/index.html)"]
impl crate::DeviceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetCullModeEXT.html) · Function"]
    pub unsafe fn cmd_set_cull_mode_ext(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        cull_mode: Option<crate::vk1_0::CullModeFlags>,
    ) -> () {
        let _function = self
            .cmd_set_cull_mode_ext
            .expect("`cmd_set_cull_mode_ext` is not loaded");
        let _return = _function(
            command_buffer as _,
            match cull_mode {
                Some(v) => v,
                None => Default::default(),
            },
        );
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetFrontFaceEXT.html) · Function"]
    pub unsafe fn cmd_set_front_face_ext(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        front_face: crate::vk1_0::FrontFace,
    ) -> () {
        let _function = self
            .cmd_set_front_face_ext
            .expect("`cmd_set_front_face_ext` is not loaded");
        let _return = _function(command_buffer as _, front_face as _);
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetPrimitiveTopologyEXT.html) · Function"]
    pub unsafe fn cmd_set_primitive_topology_ext(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        primitive_topology: crate::vk1_0::PrimitiveTopology,
    ) -> () {
        let _function = self
            .cmd_set_primitive_topology_ext
            .expect("`cmd_set_primitive_topology_ext` is not loaded");
        let _return = _function(command_buffer as _, primitive_topology as _);
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetViewportWithCountEXT.html) · Function"]
    pub unsafe fn cmd_set_viewport_with_count_ext(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        viewports: &[crate::vk1_0::ViewportBuilder],
    ) -> () {
        let _function = self
            .cmd_set_viewport_with_count_ext
            .expect("`cmd_set_viewport_with_count_ext` is not loaded");
        let viewport_count = viewports.len();
        let _return = _function(
            command_buffer as _,
            viewport_count as _,
            viewports.as_ptr() as _,
        );
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetScissorWithCountEXT.html) · Function"]
    pub unsafe fn cmd_set_scissor_with_count_ext(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        scissors: &[crate::vk1_0::Rect2DBuilder],
    ) -> () {
        let _function = self
            .cmd_set_scissor_with_count_ext
            .expect("`cmd_set_scissor_with_count_ext` is not loaded");
        let scissor_count = scissors.len();
        let _return = _function(
            command_buffer as _,
            scissor_count as _,
            scissors.as_ptr() as _,
        );
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBindVertexBuffers2EXT.html) · Function"]
    pub unsafe fn cmd_bind_vertex_buffers2_ext(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        first_binding: u32,
        buffers: &[crate::vk1_0::Buffer],
        offsets: &[crate::vk1_0::DeviceSize],
        sizes: &[crate::vk1_0::DeviceSize],
        strides: &[crate::vk1_0::DeviceSize],
    ) -> () {
        let _function = self
            .cmd_bind_vertex_buffers2_ext
            .expect("`cmd_bind_vertex_buffers2_ext` is not loaded");
        let binding_count = buffers
            .len()
            .min(offsets.len())
            .min(sizes.len())
            .min(strides.len());
        let _return = _function(
            command_buffer as _,
            first_binding as _,
            binding_count as _,
            buffers.as_ptr() as _,
            offsets.as_ptr() as _,
            sizes.as_ptr() as _,
            strides.as_ptr() as _,
        );
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetDepthTestEnableEXT.html) · Function"]
    pub unsafe fn cmd_set_depth_test_enable_ext(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        depth_test_enable: bool,
    ) -> () {
        let _function = self
            .cmd_set_depth_test_enable_ext
            .expect("`cmd_set_depth_test_enable_ext` is not loaded");
        let _return = _function(command_buffer as _, depth_test_enable as _);
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetDepthWriteEnableEXT.html) · Function"]
    pub unsafe fn cmd_set_depth_write_enable_ext(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        depth_write_enable: bool,
    ) -> () {
        let _function = self
            .cmd_set_depth_write_enable_ext
            .expect("`cmd_set_depth_write_enable_ext` is not loaded");
        let _return = _function(command_buffer as _, depth_write_enable as _);
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetDepthCompareOpEXT.html) · Function"]
    pub unsafe fn cmd_set_depth_compare_op_ext(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        depth_compare_op: crate::vk1_0::CompareOp,
    ) -> () {
        let _function = self
            .cmd_set_depth_compare_op_ext
            .expect("`cmd_set_depth_compare_op_ext` is not loaded");
        let _return = _function(command_buffer as _, depth_compare_op as _);
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetDepthBoundsTestEnableEXT.html) · Function"]
    pub unsafe fn cmd_set_depth_bounds_test_enable_ext(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        depth_bounds_test_enable: bool,
    ) -> () {
        let _function = self
            .cmd_set_depth_bounds_test_enable_ext
            .expect("`cmd_set_depth_bounds_test_enable_ext` is not loaded");
        let _return = _function(command_buffer as _, depth_bounds_test_enable as _);
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetStencilTestEnableEXT.html) · Function"]
    pub unsafe fn cmd_set_stencil_test_enable_ext(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        stencil_test_enable: bool,
    ) -> () {
        let _function = self
            .cmd_set_stencil_test_enable_ext
            .expect("`cmd_set_stencil_test_enable_ext` is not loaded");
        let _return = _function(command_buffer as _, stencil_test_enable as _);
        ()
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetStencilOpEXT.html) · Function"]
    pub unsafe fn cmd_set_stencil_op_ext(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        face_mask: crate::vk1_0::StencilFaceFlags,
        fail_op: crate::vk1_0::StencilOp,
        pass_op: crate::vk1_0::StencilOp,
        depth_fail_op: crate::vk1_0::StencilOp,
        compare_op: crate::vk1_0::CompareOp,
    ) -> () {
        let _function = self
            .cmd_set_stencil_op_ext
            .expect("`cmd_set_stencil_op_ext` is not loaded");
        let _return = _function(
            command_buffer as _,
            face_mask as _,
            fail_op as _,
            pass_op as _,
            depth_fail_op as _,
            compare_op as _,
        );
        ()
    }
}