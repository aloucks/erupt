#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_NV_ACQUIRE_WINRT_DISPLAY_SPEC_VERSION")]
pub const NV_ACQUIRE_WINRT_DISPLAY_SPEC_VERSION: u32 = 1;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_NV_ACQUIRE_WINRT_DISPLAY_EXTENSION_NAME")]
pub const NV_ACQUIRE_WINRT_DISPLAY_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_NV_acquire_winrt_display");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_ACQUIRE_WINRT_DISPLAY_NV: *const std::os::raw::c_char =
    crate::cstr!("vkAcquireWinrtDisplayNV");
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
pub const FN_GET_WINRT_DISPLAY_NV: *const std::os::raw::c_char =
    crate::cstr!("vkGetWinrtDisplayNV");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkAcquireWinrtDisplayNV.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkAcquireWinrtDisplayNV = unsafe extern "system" fn(
    physical_device: crate::vk1_0::PhysicalDevice,
    display: crate::extensions::khr_display::DisplayKHR,
) -> crate::vk1_0::Result;
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetWinrtDisplayNV.html) · Function"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetWinrtDisplayNV = unsafe extern "system" fn(
    physical_device: crate::vk1_0::PhysicalDevice,
    device_relative_id: u32,
    p_display: *mut crate::extensions::khr_display::DisplayKHR,
) -> crate::vk1_0::Result;
#[doc = "Provided by [`crate::extensions::nv_acquire_winrt_display`]"]
impl crate::InstanceLoader {
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkAcquireWinrtDisplayNV.html) · Function"]
    #[doc(alias = "vkAcquireWinrtDisplayNV")]
    pub unsafe fn acquire_winrt_display_nv(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        display: crate::extensions::khr_display::DisplayKHR,
    ) -> crate::utils::VulkanResult<()> {
        let _function = self
            .acquire_winrt_display_nv
            .expect("`acquire_winrt_display_nv` is not loaded");
        let _return = _function(physical_device as _, display as _);
        crate::utils::VulkanResult::new(_return, ())
    }
    #[inline]
    #[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetWinrtDisplayNV.html) · Function"]
    #[doc(alias = "vkGetWinrtDisplayNV")]
    pub unsafe fn get_winrt_display_nv(
        &self,
        physical_device: crate::vk1_0::PhysicalDevice,
        device_relative_id: u32,
        display: Option<crate::extensions::khr_display::DisplayKHR>,
    ) -> crate::utils::VulkanResult<crate::extensions::khr_display::DisplayKHR> {
        let _function = self
            .get_winrt_display_nv
            .expect("`get_winrt_display_nv` is not loaded");
        let mut display = match display {
            Some(v) => v,
            None => Default::default(),
        };
        let _return = _function(physical_device as _, device_relative_id as _, &mut display);
        crate::utils::VulkanResult::new(_return, display)
    }
}
