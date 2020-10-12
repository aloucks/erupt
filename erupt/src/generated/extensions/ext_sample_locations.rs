#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_SAMPLE_LOCATIONS_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const EXT_SAMPLE_LOCATIONS_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_EXT_sample_locations");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_SET_SAMPLE_LOCATIONS_EXT: *const std::os::raw::c_char =
    crate::cstr!("vkCmdSetSampleLocationsEXT");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_PHYSICAL_DEVICE_MULTISAMPLE_PROPERTIES_EXT: *const std::os::raw::c_char =
    crate::cstr!("vkGetPhysicalDeviceMultisamplePropertiesEXT");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetSampleLocationsEXT.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetSampleLocationsEXT = unsafe extern "system" fn(
    command_buffer: crate::vk1_0::CommandBuffer,
    p_sample_locations_info: *const crate::extensions::ext_sample_locations::SampleLocationsInfoEXT,
) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceMultisamplePropertiesEXT.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceMultisamplePropertiesEXT = unsafe extern "system" fn(
    physical_device: crate::vk1_0::PhysicalDevice,
    samples: crate::vk1_0::SampleCountFlagBits,
    p_multisample_properties: *mut crate::extensions::ext_sample_locations::MultisamplePropertiesEXT,
) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSampleLocationEXT.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SampleLocationEXT {
    pub x: std::os::raw::c_float,
    pub y: std::os::raw::c_float,
}
impl Default for SampleLocationEXT {
    fn default() -> Self {
        Self {
            x: Default::default(),
            y: Default::default(),
        }
    }
}
impl std::fmt::Debug for SampleLocationEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SampleLocationEXT")
            .field("x", &self.x)
            .field("y", &self.y)
            .finish()
    }
}
impl SampleLocationEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> SampleLocationEXTBuilder<'a> {
        SampleLocationEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSampleLocationEXT.html) · Builder of [`SampleLocationEXT`](struct.SampleLocationEXT.html)"]
#[repr(transparent)]
pub struct SampleLocationEXTBuilder<'a>(SampleLocationEXT, std::marker::PhantomData<&'a ()>);
impl<'a> SampleLocationEXTBuilder<'a> {
    #[inline]
    pub fn new() -> SampleLocationEXTBuilder<'a> {
        SampleLocationEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn x(mut self, x: std::os::raw::c_float) -> Self {
        self.0.x = x as _;
        self
    }
    #[inline]
    pub fn y(mut self, y: std::os::raw::c_float) -> Self {
        self.0.y = y as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> SampleLocationEXT {
        self.0
    }
}
impl<'a> std::default::Default for SampleLocationEXTBuilder<'a> {
    fn default() -> SampleLocationEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for SampleLocationEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for SampleLocationEXTBuilder<'a> {
    type Target = SampleLocationEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for SampleLocationEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSampleLocationsInfoEXT.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SampleLocationsInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub sample_locations_per_pixel: crate::vk1_0::SampleCountFlagBits,
    pub sample_location_grid_size: crate::vk1_0::Extent2D,
    pub sample_locations_count: u32,
    pub p_sample_locations: *const crate::extensions::ext_sample_locations::SampleLocationEXT,
}
impl Default for SampleLocationsInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::SAMPLE_LOCATIONS_INFO_EXT,
            p_next: std::ptr::null(),
            sample_locations_per_pixel: Default::default(),
            sample_location_grid_size: Default::default(),
            sample_locations_count: Default::default(),
            p_sample_locations: std::ptr::null(),
        }
    }
}
impl std::fmt::Debug for SampleLocationsInfoEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SampleLocationsInfoEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field(
                "sample_locations_per_pixel",
                &self.sample_locations_per_pixel,
            )
            .field("sample_location_grid_size", &self.sample_location_grid_size)
            .field("sample_locations_count", &self.sample_locations_count)
            .field("p_sample_locations", &self.p_sample_locations)
            .finish()
    }
}
impl SampleLocationsInfoEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> SampleLocationsInfoEXTBuilder<'a> {
        SampleLocationsInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSampleLocationsInfoEXT.html) · Builder of [`SampleLocationsInfoEXT`](struct.SampleLocationsInfoEXT.html)"]
#[repr(transparent)]
pub struct SampleLocationsInfoEXTBuilder<'a>(
    SampleLocationsInfoEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> SampleLocationsInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> SampleLocationsInfoEXTBuilder<'a> {
        SampleLocationsInfoEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn sample_locations_per_pixel(
        mut self,
        sample_locations_per_pixel: crate::vk1_0::SampleCountFlagBits,
    ) -> Self {
        self.0.sample_locations_per_pixel = sample_locations_per_pixel as _;
        self
    }
    #[inline]
    pub fn sample_location_grid_size(
        mut self,
        sample_location_grid_size: crate::vk1_0::Extent2D,
    ) -> Self {
        self.0.sample_location_grid_size = sample_location_grid_size as _;
        self
    }
    #[inline]
    pub fn sample_locations(
        mut self,
        sample_locations: &'a [crate::extensions::ext_sample_locations::SampleLocationEXTBuilder],
    ) -> Self {
        self.0.p_sample_locations = sample_locations.as_ptr() as _;
        self.0.sample_locations_count = sample_locations.len() as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> SampleLocationsInfoEXT {
        self.0
    }
}
impl<'a> std::default::Default for SampleLocationsInfoEXTBuilder<'a> {
    fn default() -> SampleLocationsInfoEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for SampleLocationsInfoEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for SampleLocationsInfoEXTBuilder<'a> {
    type Target = SampleLocationsInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for SampleLocationsInfoEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAttachmentSampleLocationsEXT.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AttachmentSampleLocationsEXT {
    pub attachment_index: u32,
    pub sample_locations_info: crate::extensions::ext_sample_locations::SampleLocationsInfoEXT,
}
impl Default for AttachmentSampleLocationsEXT {
    fn default() -> Self {
        Self {
            attachment_index: Default::default(),
            sample_locations_info: Default::default(),
        }
    }
}
impl std::fmt::Debug for AttachmentSampleLocationsEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("AttachmentSampleLocationsEXT")
            .field("attachment_index", &self.attachment_index)
            .field("sample_locations_info", &self.sample_locations_info)
            .finish()
    }
}
impl AttachmentSampleLocationsEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> AttachmentSampleLocationsEXTBuilder<'a> {
        AttachmentSampleLocationsEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAttachmentSampleLocationsEXT.html) · Builder of [`AttachmentSampleLocationsEXT`](struct.AttachmentSampleLocationsEXT.html)"]
#[repr(transparent)]
pub struct AttachmentSampleLocationsEXTBuilder<'a>(
    AttachmentSampleLocationsEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> AttachmentSampleLocationsEXTBuilder<'a> {
    #[inline]
    pub fn new() -> AttachmentSampleLocationsEXTBuilder<'a> {
        AttachmentSampleLocationsEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn attachment_index(mut self, attachment_index: u32) -> Self {
        self.0.attachment_index = attachment_index as _;
        self
    }
    #[inline]
    pub fn sample_locations_info(
        mut self,
        sample_locations_info: crate::extensions::ext_sample_locations::SampleLocationsInfoEXT,
    ) -> Self {
        self.0.sample_locations_info = sample_locations_info as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> AttachmentSampleLocationsEXT {
        self.0
    }
}
impl<'a> std::default::Default for AttachmentSampleLocationsEXTBuilder<'a> {
    fn default() -> AttachmentSampleLocationsEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for AttachmentSampleLocationsEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for AttachmentSampleLocationsEXTBuilder<'a> {
    type Target = AttachmentSampleLocationsEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for AttachmentSampleLocationsEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSubpassSampleLocationsEXT.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SubpassSampleLocationsEXT {
    pub subpass_index: u32,
    pub sample_locations_info: crate::extensions::ext_sample_locations::SampleLocationsInfoEXT,
}
impl Default for SubpassSampleLocationsEXT {
    fn default() -> Self {
        Self {
            subpass_index: Default::default(),
            sample_locations_info: Default::default(),
        }
    }
}
impl std::fmt::Debug for SubpassSampleLocationsEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SubpassSampleLocationsEXT")
            .field("subpass_index", &self.subpass_index)
            .field("sample_locations_info", &self.sample_locations_info)
            .finish()
    }
}
impl SubpassSampleLocationsEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> SubpassSampleLocationsEXTBuilder<'a> {
        SubpassSampleLocationsEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkSubpassSampleLocationsEXT.html) · Builder of [`SubpassSampleLocationsEXT`](struct.SubpassSampleLocationsEXT.html)"]
#[repr(transparent)]
pub struct SubpassSampleLocationsEXTBuilder<'a>(
    SubpassSampleLocationsEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> SubpassSampleLocationsEXTBuilder<'a> {
    #[inline]
    pub fn new() -> SubpassSampleLocationsEXTBuilder<'a> {
        SubpassSampleLocationsEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn subpass_index(mut self, subpass_index: u32) -> Self {
        self.0.subpass_index = subpass_index as _;
        self
    }
    #[inline]
    pub fn sample_locations_info(
        mut self,
        sample_locations_info: crate::extensions::ext_sample_locations::SampleLocationsInfoEXT,
    ) -> Self {
        self.0.sample_locations_info = sample_locations_info as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> SubpassSampleLocationsEXT {
        self.0
    }
}
impl<'a> std::default::Default for SubpassSampleLocationsEXTBuilder<'a> {
    fn default() -> SubpassSampleLocationsEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for SubpassSampleLocationsEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for SubpassSampleLocationsEXTBuilder<'a> {
    type Target = SubpassSampleLocationsEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for SubpassSampleLocationsEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRenderPassSampleLocationsBeginInfoEXT.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RenderPassSampleLocationsBeginInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub attachment_initial_sample_locations_count: u32,
    pub p_attachment_initial_sample_locations:
        *const crate::extensions::ext_sample_locations::AttachmentSampleLocationsEXT,
    pub post_subpass_sample_locations_count: u32,
    pub p_post_subpass_sample_locations:
        *const crate::extensions::ext_sample_locations::SubpassSampleLocationsEXT,
}
impl Default for RenderPassSampleLocationsBeginInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::RENDER_PASS_SAMPLE_LOCATIONS_BEGIN_INFO_EXT,
            p_next: std::ptr::null(),
            attachment_initial_sample_locations_count: Default::default(),
            p_attachment_initial_sample_locations: std::ptr::null(),
            post_subpass_sample_locations_count: Default::default(),
            p_post_subpass_sample_locations: std::ptr::null(),
        }
    }
}
impl std::fmt::Debug for RenderPassSampleLocationsBeginInfoEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RenderPassSampleLocationsBeginInfoEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field(
                "attachment_initial_sample_locations_count",
                &self.attachment_initial_sample_locations_count,
            )
            .field(
                "p_attachment_initial_sample_locations",
                &self.p_attachment_initial_sample_locations,
            )
            .field(
                "post_subpass_sample_locations_count",
                &self.post_subpass_sample_locations_count,
            )
            .field(
                "p_post_subpass_sample_locations",
                &self.p_post_subpass_sample_locations,
            )
            .finish()
    }
}
impl RenderPassSampleLocationsBeginInfoEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> RenderPassSampleLocationsBeginInfoEXTBuilder<'a> {
        RenderPassSampleLocationsBeginInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkRenderPassSampleLocationsBeginInfoEXT.html) · Builder of [`RenderPassSampleLocationsBeginInfoEXT`](struct.RenderPassSampleLocationsBeginInfoEXT.html)"]
#[repr(transparent)]
pub struct RenderPassSampleLocationsBeginInfoEXTBuilder<'a>(
    RenderPassSampleLocationsBeginInfoEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> RenderPassSampleLocationsBeginInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> RenderPassSampleLocationsBeginInfoEXTBuilder<'a> {
        RenderPassSampleLocationsBeginInfoEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn attachment_initial_sample_locations(
        mut self,
        attachment_initial_sample_locations : & 'a [crate :: extensions :: ext_sample_locations :: AttachmentSampleLocationsEXTBuilder],
    ) -> Self {
        self.0.p_attachment_initial_sample_locations =
            attachment_initial_sample_locations.as_ptr() as _;
        self.0.attachment_initial_sample_locations_count =
            attachment_initial_sample_locations.len() as _;
        self
    }
    #[inline]
    pub fn post_subpass_sample_locations(
        mut self,
        post_subpass_sample_locations : & 'a [crate :: extensions :: ext_sample_locations :: SubpassSampleLocationsEXTBuilder],
    ) -> Self {
        self.0.p_post_subpass_sample_locations = post_subpass_sample_locations.as_ptr() as _;
        self.0.post_subpass_sample_locations_count = post_subpass_sample_locations.len() as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> RenderPassSampleLocationsBeginInfoEXT {
        self.0
    }
}
impl<'a> std::default::Default for RenderPassSampleLocationsBeginInfoEXTBuilder<'a> {
    fn default() -> RenderPassSampleLocationsBeginInfoEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for RenderPassSampleLocationsBeginInfoEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for RenderPassSampleLocationsBeginInfoEXTBuilder<'a> {
    type Target = RenderPassSampleLocationsBeginInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for RenderPassSampleLocationsBeginInfoEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineSampleLocationsStateCreateInfoEXT.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PipelineSampleLocationsStateCreateInfoEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub sample_locations_enable: crate::vk1_0::Bool32,
    pub sample_locations_info: crate::extensions::ext_sample_locations::SampleLocationsInfoEXT,
}
impl Default for PipelineSampleLocationsStateCreateInfoEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::PIPELINE_SAMPLE_LOCATIONS_STATE_CREATE_INFO_EXT,
            p_next: std::ptr::null(),
            sample_locations_enable: Default::default(),
            sample_locations_info: Default::default(),
        }
    }
}
impl std::fmt::Debug for PipelineSampleLocationsStateCreateInfoEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PipelineSampleLocationsStateCreateInfoEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field(
                "sample_locations_enable",
                &(self.sample_locations_enable != 0),
            )
            .field("sample_locations_info", &self.sample_locations_info)
            .finish()
    }
}
impl PipelineSampleLocationsStateCreateInfoEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> PipelineSampleLocationsStateCreateInfoEXTBuilder<'a> {
        PipelineSampleLocationsStateCreateInfoEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPipelineSampleLocationsStateCreateInfoEXT.html) · Builder of [`PipelineSampleLocationsStateCreateInfoEXT`](struct.PipelineSampleLocationsStateCreateInfoEXT.html)"]
#[repr(transparent)]
pub struct PipelineSampleLocationsStateCreateInfoEXTBuilder<'a>(
    PipelineSampleLocationsStateCreateInfoEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PipelineSampleLocationsStateCreateInfoEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PipelineSampleLocationsStateCreateInfoEXTBuilder<'a> {
        PipelineSampleLocationsStateCreateInfoEXTBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[inline]
    pub fn sample_locations_enable(mut self, sample_locations_enable: bool) -> Self {
        self.0.sample_locations_enable = sample_locations_enable as _;
        self
    }
    #[inline]
    pub fn sample_locations_info(
        mut self,
        sample_locations_info: crate::extensions::ext_sample_locations::SampleLocationsInfoEXT,
    ) -> Self {
        self.0.sample_locations_info = sample_locations_info as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PipelineSampleLocationsStateCreateInfoEXT {
        self.0
    }
}
impl<'a> std::default::Default for PipelineSampleLocationsStateCreateInfoEXTBuilder<'a> {
    fn default() -> PipelineSampleLocationsStateCreateInfoEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PipelineSampleLocationsStateCreateInfoEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PipelineSampleLocationsStateCreateInfoEXTBuilder<'a> {
    type Target = PipelineSampleLocationsStateCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PipelineSampleLocationsStateCreateInfoEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceSampleLocationsPropertiesEXT.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PhysicalDeviceSampleLocationsPropertiesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub sample_location_sample_counts: crate::vk1_0::SampleCountFlags,
    pub max_sample_location_grid_size: crate::vk1_0::Extent2D,
    pub sample_location_coordinate_range: [std::os::raw::c_float; 2],
    pub sample_location_sub_pixel_bits: u32,
    pub variable_sample_locations: crate::vk1_0::Bool32,
}
impl Default for PhysicalDeviceSampleLocationsPropertiesEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::PHYSICAL_DEVICE_SAMPLE_LOCATIONS_PROPERTIES_EXT,
            p_next: std::ptr::null_mut(),
            sample_location_sample_counts: Default::default(),
            max_sample_location_grid_size: Default::default(),
            sample_location_coordinate_range: unsafe { std::mem::zeroed() },
            sample_location_sub_pixel_bits: Default::default(),
            variable_sample_locations: Default::default(),
        }
    }
}
impl std::fmt::Debug for PhysicalDeviceSampleLocationsPropertiesEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PhysicalDeviceSampleLocationsPropertiesEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field(
                "sample_location_sample_counts",
                &self.sample_location_sample_counts,
            )
            .field(
                "max_sample_location_grid_size",
                &self.max_sample_location_grid_size,
            )
            .field(
                "sample_location_coordinate_range",
                &self.sample_location_coordinate_range,
            )
            .field(
                "sample_location_sub_pixel_bits",
                &self.sample_location_sub_pixel_bits,
            )
            .field(
                "variable_sample_locations",
                &(self.variable_sample_locations != 0),
            )
            .finish()
    }
}
impl PhysicalDeviceSampleLocationsPropertiesEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> PhysicalDeviceSampleLocationsPropertiesEXTBuilder<'a> {
        PhysicalDeviceSampleLocationsPropertiesEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkPhysicalDeviceSampleLocationsPropertiesEXT.html) · Builder of [`PhysicalDeviceSampleLocationsPropertiesEXT`](struct.PhysicalDeviceSampleLocationsPropertiesEXT.html)"]
#[repr(transparent)]
pub struct PhysicalDeviceSampleLocationsPropertiesEXTBuilder<'a>(
    PhysicalDeviceSampleLocationsPropertiesEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> PhysicalDeviceSampleLocationsPropertiesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> PhysicalDeviceSampleLocationsPropertiesEXTBuilder<'a> {
        PhysicalDeviceSampleLocationsPropertiesEXTBuilder(
            Default::default(),
            std::marker::PhantomData,
        )
    }
    #[inline]
    pub fn sample_location_sample_counts(
        mut self,
        sample_location_sample_counts: crate::vk1_0::SampleCountFlags,
    ) -> Self {
        self.0.sample_location_sample_counts = sample_location_sample_counts as _;
        self
    }
    #[inline]
    pub fn max_sample_location_grid_size(
        mut self,
        max_sample_location_grid_size: crate::vk1_0::Extent2D,
    ) -> Self {
        self.0.max_sample_location_grid_size = max_sample_location_grid_size as _;
        self
    }
    #[inline]
    pub fn sample_location_coordinate_range(
        mut self,
        sample_location_coordinate_range: [std::os::raw::c_float; 2],
    ) -> Self {
        self.0.sample_location_coordinate_range = sample_location_coordinate_range as _;
        self
    }
    #[inline]
    pub fn sample_location_sub_pixel_bits(mut self, sample_location_sub_pixel_bits: u32) -> Self {
        self.0.sample_location_sub_pixel_bits = sample_location_sub_pixel_bits as _;
        self
    }
    #[inline]
    pub fn variable_sample_locations(mut self, variable_sample_locations: bool) -> Self {
        self.0.variable_sample_locations = variable_sample_locations as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> PhysicalDeviceSampleLocationsPropertiesEXT {
        self.0
    }
}
impl<'a> std::default::Default for PhysicalDeviceSampleLocationsPropertiesEXTBuilder<'a> {
    fn default() -> PhysicalDeviceSampleLocationsPropertiesEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for PhysicalDeviceSampleLocationsPropertiesEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for PhysicalDeviceSampleLocationsPropertiesEXTBuilder<'a> {
    type Target = PhysicalDeviceSampleLocationsPropertiesEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for PhysicalDeviceSampleLocationsPropertiesEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMultisamplePropertiesEXT.html) · Structure"]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MultisamplePropertiesEXT {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *mut std::ffi::c_void,
    pub max_sample_location_grid_size: crate::vk1_0::Extent2D,
}
impl Default for MultisamplePropertiesEXT {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::MULTISAMPLE_PROPERTIES_EXT,
            p_next: std::ptr::null_mut(),
            max_sample_location_grid_size: Default::default(),
        }
    }
}
impl std::fmt::Debug for MultisamplePropertiesEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("MultisamplePropertiesEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field(
                "max_sample_location_grid_size",
                &self.max_sample_location_grid_size,
            )
            .finish()
    }
}
impl MultisamplePropertiesEXT {
    #[inline]
    pub fn into_builder<'a>(self) -> MultisamplePropertiesEXTBuilder<'a> {
        MultisamplePropertiesEXTBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkMultisamplePropertiesEXT.html) · Builder of [`MultisamplePropertiesEXT`](struct.MultisamplePropertiesEXT.html)"]
#[repr(transparent)]
pub struct MultisamplePropertiesEXTBuilder<'a>(
    MultisamplePropertiesEXT,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> MultisamplePropertiesEXTBuilder<'a> {
    #[inline]
    pub fn new() -> MultisamplePropertiesEXTBuilder<'a> {
        MultisamplePropertiesEXTBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn max_sample_location_grid_size(
        mut self,
        max_sample_location_grid_size: crate::vk1_0::Extent2D,
    ) -> Self {
        self.0.max_sample_location_grid_size = max_sample_location_grid_size as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> MultisamplePropertiesEXT {
        self.0
    }
}
impl<'a> std::default::Default for MultisamplePropertiesEXTBuilder<'a> {
    fn default() -> MultisamplePropertiesEXTBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for MultisamplePropertiesEXTBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for MultisamplePropertiesEXTBuilder<'a> {
    type Target = MultisamplePropertiesEXT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for MultisamplePropertiesEXTBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "Provided by [`extensions::ext_sample_locations`](extensions/ext_sample_locations/index.html)"]
impl crate::DeviceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetSampleLocationsEXT.html) · Function"]
    pub unsafe fn cmd_set_sample_locations_ext(
        &self,
        command_buffer: crate::vk1_0::CommandBuffer,
        sample_locations_info: &crate::extensions::ext_sample_locations::SampleLocationsInfoEXT,
    ) -> () {
        let _function = self
            .cmd_set_sample_locations_ext
            .expect("`cmd_set_sample_locations_ext` is not loaded");
        let _return = _function(command_buffer as _, sample_locations_info as _);
        ()
    }
}
#[doc = "Provided by [`extensions::ext_sample_locations`](extensions/ext_sample_locations/index.html)"]
impl crate::InstanceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceMultisamplePropertiesEXT.html) · Function"]
    pub unsafe fn get_physical_device_multisample_properties_ext(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        samples: crate::vk1_0::SampleCountFlagBits,
        multisample_properties: Option<
            crate::extensions::ext_sample_locations::MultisamplePropertiesEXT,
        >,
    ) -> crate::extensions::ext_sample_locations::MultisamplePropertiesEXT {
        let _function = self
            .get_physical_device_multisample_properties_ext
            .expect("`get_physical_device_multisample_properties_ext` is not loaded");
        let mut multisample_properties = match multisample_properties {
            Some(v) => v,
            None => Default::default(),
        };
        let _return = _function(
            physical_device as _,
            samples as _,
            &mut multisample_properties,
        );
        multisample_properties
    }
}
