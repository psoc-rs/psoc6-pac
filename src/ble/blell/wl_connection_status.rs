#[doc = "Reader of register WL_CONNECTION_STATUS"]
pub type R = crate::R<u32, super::WL_CONNECTION_STATUS>;
#[doc = "Writer for register WL_CONNECTION_STATUS"]
pub type W = crate::W<u32, super::WL_CONNECTION_STATUS>;
#[doc = "Register WL_CONNECTION_STATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::WL_CONNECTION_STATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WL_ENTRY_CONNECTED`"]
pub type WL_ENTRY_CONNECTED_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `WL_ENTRY_CONNECTED`"]
pub struct WL_ENTRY_CONNECTED_W<'a> {
    w: &'a mut W,
}
impl<'a> WL_ENTRY_CONNECTED_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Stores the connection status of each of the sixteen device address stored in the whitelist. 1 - White list entry is already in a connection 0 - White list entry is not in a connection"]
    #[inline(always)]
    pub fn wl_entry_connected(&self) -> WL_ENTRY_CONNECTED_R {
        WL_ENTRY_CONNECTED_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Stores the connection status of each of the sixteen device address stored in the whitelist. 1 - White list entry is already in a connection 0 - White list entry is not in a connection"]
    #[inline(always)]
    pub fn wl_entry_connected(&mut self) -> WL_ENTRY_CONNECTED_W {
        WL_ENTRY_CONNECTED_W { w: self }
    }
}
