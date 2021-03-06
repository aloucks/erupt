//! ## Versioning Warning ⚠️
//!
//! This is a Vulkan **provisional/beta** extension and **must** be used with
//! caution. Its API/behaviour has not been finalized yet and _may_ therefore
//! change in ways that break backwards compatibility between revisions, and
//! before final release of a non-provisional version of this extension.
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_VIDEO_DECODE_QUEUE_SPEC_VERSION")]
pub const KHR_VIDEO_DECODE_QUEUE_SPEC_VERSION: u32 = 2;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_KHR_VIDEO_DECODE_QUEUE_EXTENSION_NAME")]
pub const KHR_VIDEO_DECODE_QUEUE_EXTENSION_NAME: *const std::os::raw::c_char = crate::cstr!("VK_KHR_video_decode_queue");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_CMD_DECODE_VIDEO_KHR: *const std::os::raw::c_char = crate::cstr!("vkCmdDecodeVideoKHR");
#[doc = "Provided by [`crate::extensions::khr_video_decode_queue`]"]
impl crate::extensions::khr_synchronization2::AccessFlagBits2KHR {
    pub const VIDEO_DECODE_READ_KHR: Self = Self(34359738368);
    pub const VIDEO_DECODE_WRITE_KHR: Self = Self(68719476736);
}
#[doc = "Provided by [`crate::extensions::khr_video_decode_queue`]"]
impl crate::extensions::khr_synchronization2::PipelineStageFlagBits2KHR {
    pub const VIDEO_DECODE_KHR: Self = Self(67108864);
}
#[doc = "Provided by [`crate::extensions::khr_video_decode_queue`]"]
impl crate::extensions::khr_format_feature_flags2::FormatFeatureFlagBits2KHR {
    pub const VIDEO_DECODE_OUTPUT_KHR: Self = Self(33554432);
    pub const VIDEO_DECODE_DPB_KHR: Self = Self(67108864);
}
#[doc = "Provided by [`crate::extensions::khr_video_decode_queue`]"]
impl crate::vk1_0::BufferUsageFlagBits {
    pub const VIDEO_DECODE_SRC_KHR: Self = Self(8192);
    pub const VIDEO_DECODE_DST_KHR: Self = Self(16384);
}
#[doc = "Provided by [`crate::extensions::khr_video_decode_queue`]"]
impl crate::vk1_0::FormatFeatureFlagBits {
    pub const VIDEO_DECODE_OUTPUT_KHR: Self = Self(33554432);
    pub const VIDEO_DECODE_DPB_KHR: Self = Self(67108864);
}
#[doc = "Provided by [`crate::extensions::khr_video_decode_queue`]"]
impl crate::vk1_0::ImageLayout {
    pub const VIDEO_DECODE_DST_KHR: Self = Self(1000024000);
    pub const VIDEO_DECODE_SRC_KHR: Self = Self(1000024001);
    pub const VIDEO_DECODE_DPB_KHR: Self = Self(1000024002);
}
#[doc = "Provided by [`crate::extensions::khr_video_decode_queue`]"]
impl crate::vk1_0::ImageUsageFlagBits {
    pub const VIDEO_DECODE_DST_KHR: Self = Self(1024);
    pub const VIDEO_DECODE_SRC_KHR: Self = Self(2048);
    pub const VIDEO_DECODE_DPB_KHR: Self = Self(4096);
}
#[doc = "Provided by [`crate::extensions::khr_video_decode_queue`]"]
impl crate::vk1_0::QueueFlagBits {
    pub const VIDEO_DECODE_KHR: Self = Self(32);
}
#[doc = "Provided by [`crate::extensions::khr_video_decode_queue`]"]
impl crate::vk1_0::StructureType {
    pub const VIDEO_DECODE_INFO_KHR: Self = Self(1000024000);
}
bitflags::bitflags! { # [doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoDecodeFlagsKHR.html) · Bitmask of [`VideoDecodeFlagBitsKHR`]"] # [doc (alias = "VkVideoDecodeFlagsKHR")] # [derive (Default)] # [repr (transparent)] pub struct VideoDecodeFlagsKHR : u32 { const DEFAULT_KHR = VideoDecodeFlagBitsKHR :: DEFAULT_KHR . 0 ; const RESERVED_0_KHR = VideoDecodeFlagBitsKHR :: RESERVED_0_KHR . 0 ; } }
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoDecodeFlagBitsKHR.html) · Bits enum of [`VideoDecodeFlagsKHR`]"]
#[doc(alias = "VkVideoDecodeFlagBitsKHR")]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct VideoDecodeFlagBitsKHR(pub u32);
impl VideoDecodeFlagBitsKHR {
    #[inline]
    #[doc = "Converts this enum variant to the corresponding bitmask"]
    pub const fn bitmask(&self) -> VideoDecodeFlagsKHR {
        VideoDecodeFlagsKHR::from_bits_truncate(self.0)
    }
}
impl std::fmt::Debug for VideoDecodeFlagBitsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(match self {
            &Self::DEFAULT_KHR => "DEFAULT_KHR",
            &Self::RESERVED_0_KHR => "RESERVED_0_KHR",
            _ => "(unknown variant)",
        })
    }
}
#[doc = "Provided by [`crate::extensions::khr_video_decode_queue`]"]
impl crate::extensions::khr_video_decode_queue::VideoDecodeFlagBitsKHR {
    pub const DEFAULT_KHR: Self = Self(0);
    pub const RESERVED_0_KHR: Self = Self(1);
}
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDecodeVideoKHR.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDecodeVideoKHR = unsafe extern "system" fn(command_buffer: crate::vk1_0::CommandBuffer, p_frame_info: *const crate::extensions::khr_video_decode_queue::VideoDecodeInfoKHR) -> ();
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoDecodeInfoKHR.html) · Structure"]
#[doc(alias = "VkVideoDecodeInfoKHR")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VideoDecodeInfoKHR {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub flags: crate::extensions::khr_video_decode_queue::VideoDecodeFlagsKHR,
    pub coded_offset: crate::vk1_0::Offset2D,
    pub coded_extent: crate::vk1_0::Extent2D,
    pub src_buffer: crate::vk1_0::Buffer,
    pub src_buffer_offset: crate::vk1_0::DeviceSize,
    pub src_buffer_range: crate::vk1_0::DeviceSize,
    pub dst_picture_resource: crate::extensions::khr_video_queue::VideoPictureResourceKHR,
    pub p_setup_reference_slot: *const crate::extensions::khr_video_queue::VideoReferenceSlotKHR,
    pub reference_slot_count: u32,
    pub p_reference_slots: *const crate::extensions::khr_video_queue::VideoReferenceSlotKHR,
}
impl VideoDecodeInfoKHR {
    pub const STRUCTURE_TYPE: crate::vk1_0::StructureType = crate::vk1_0::StructureType::VIDEO_DECODE_INFO_KHR;
}
impl Default for VideoDecodeInfoKHR {
    fn default() -> Self {
        Self { s_type: Self::STRUCTURE_TYPE, p_next: std::ptr::null(), flags: Default::default(), coded_offset: Default::default(), coded_extent: Default::default(), src_buffer: Default::default(), src_buffer_offset: Default::default(), src_buffer_range: Default::default(), dst_picture_resource: Default::default(), p_setup_reference_slot: std::ptr::null(), reference_slot_count: Default::default(), p_reference_slots: std::ptr::null() }
    }
}
impl std::fmt::Debug for VideoDecodeInfoKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("VideoDecodeInfoKHR").field("s_type", &self.s_type).field("p_next", &self.p_next).field("flags", &self.flags).field("coded_offset", &self.coded_offset).field("coded_extent", &self.coded_extent).field("src_buffer", &self.src_buffer).field("src_buffer_offset", &self.src_buffer_offset).field("src_buffer_range", &self.src_buffer_range).field("dst_picture_resource", &self.dst_picture_resource).field("p_setup_reference_slot", &self.p_setup_reference_slot).field("reference_slot_count", &self.reference_slot_count).field("p_reference_slots", &self.p_reference_slots).finish()
    }
}
impl VideoDecodeInfoKHR {
    #[inline]
    pub fn into_builder<'a>(self) -> VideoDecodeInfoKHRBuilder<'a> {
        VideoDecodeInfoKHRBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkVideoDecodeInfoKHR.html) · Builder of [`VideoDecodeInfoKHR`]"]
#[repr(transparent)]
pub struct VideoDecodeInfoKHRBuilder<'a>(VideoDecodeInfoKHR, std::marker::PhantomData<&'a ()>);
impl<'a> VideoDecodeInfoKHRBuilder<'a> {
    #[inline]
    pub fn new() -> VideoDecodeInfoKHRBuilder<'a> {
        VideoDecodeInfoKHRBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn flags(mut self, flags: crate::extensions::khr_video_decode_queue::VideoDecodeFlagsKHR) -> Self {
        self.0.flags = flags as _;
        self
    }
    #[inline]
    pub fn coded_offset(mut self, coded_offset: crate::vk1_0::Offset2D) -> Self {
        self.0.coded_offset = coded_offset as _;
        self
    }
    #[inline]
    pub fn coded_extent(mut self, coded_extent: crate::vk1_0::Extent2D) -> Self {
        self.0.coded_extent = coded_extent as _;
        self
    }
    #[inline]
    pub fn src_buffer(mut self, src_buffer: crate::vk1_0::Buffer) -> Self {
        self.0.src_buffer = src_buffer as _;
        self
    }
    #[inline]
    pub fn src_buffer_offset(mut self, src_buffer_offset: crate::vk1_0::DeviceSize) -> Self {
        self.0.src_buffer_offset = src_buffer_offset as _;
        self
    }
    #[inline]
    pub fn src_buffer_range(mut self, src_buffer_range: crate::vk1_0::DeviceSize) -> Self {
        self.0.src_buffer_range = src_buffer_range as _;
        self
    }
    #[inline]
    pub fn dst_picture_resource(mut self, dst_picture_resource: crate::extensions::khr_video_queue::VideoPictureResourceKHR) -> Self {
        self.0.dst_picture_resource = dst_picture_resource as _;
        self
    }
    #[inline]
    pub fn setup_reference_slot(mut self, setup_reference_slot: &'a crate::extensions::khr_video_queue::VideoReferenceSlotKHR) -> Self {
        self.0.p_setup_reference_slot = setup_reference_slot as _;
        self
    }
    #[inline]
    pub fn reference_slots(mut self, reference_slots: &'a [crate::extensions::khr_video_queue::VideoReferenceSlotKHRBuilder]) -> Self {
        self.0.p_reference_slots = reference_slots.as_ptr() as _;
        self.0.reference_slot_count = reference_slots.len() as _;
        self
    }
    #[inline]
    #[doc = r" Discards all lifetime information."]
    #[doc = r" Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build_dangling(self) -> VideoDecodeInfoKHR {
        self.0
    }
}
impl<'a> std::default::Default for VideoDecodeInfoKHRBuilder<'a> {
    fn default() -> VideoDecodeInfoKHRBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for VideoDecodeInfoKHRBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for VideoDecodeInfoKHRBuilder<'a> {
    type Target = VideoDecodeInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for VideoDecodeInfoKHRBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[doc = "Provided by [`crate::extensions::khr_video_decode_queue`]"]
impl crate::DeviceLoader {
    #[inline]
    #[track_caller]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDecodeVideoKHR.html) · Function"]
    #[doc(alias = "vkCmdDecodeVideoKHR")]
    pub unsafe fn cmd_decode_video_khr(&self, command_buffer: crate::vk1_0::CommandBuffer, frame_info: &crate::extensions::khr_video_decode_queue::VideoDecodeInfoKHR) -> () {
        let _function = self.cmd_decode_video_khr.expect(crate::NOT_LOADED_MESSAGE);
        let _return = _function(command_buffer as _, frame_info as _);
        ()
    }
}
