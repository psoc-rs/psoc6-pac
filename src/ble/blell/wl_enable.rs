#[doc = "Reader of register WL_ENABLE"]
pub type R = crate::R<u32, super::WL_ENABLE>;
#[doc = "Writer for register WL_ENABLE"]
pub type W = crate::W<u32, super::WL_ENABLE>;
#[doc = "Register WL_ENABLE `reset()`'s with value 0"]
impl crate::ResetValue for super::WL_ENABLE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WL_ENABLE`"]
pub type WL_ENABLE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `WL_ENABLE`"]
pub struct WL_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> WL_ENABLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Stores the valid entry bit corresponding to each of the eight device address stored in the whitelist. 1 - White list entry is Valid 0 - White list entry is Invalid"]
    #[inline(always)]
    pub fn wl_enable(&self) -> WL_ENABLE_R {
        WL_ENABLE_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Stores the valid entry bit corresponding to each of the eight device address stored in the whitelist. 1 - White list entry is Valid 0 - White list entry is Invalid"]
    #[inline(always)]
    pub fn wl_enable(&mut self) -> WL_ENABLE_W {
        WL_ENABLE_W { w: self }
    }
}
