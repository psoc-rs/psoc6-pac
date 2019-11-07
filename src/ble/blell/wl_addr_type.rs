#[doc = "Reader of register WL_ADDR_TYPE"]
pub type R = crate::R<u32, super::WL_ADDR_TYPE>;
#[doc = "Writer for register WL_ADDR_TYPE"]
pub type W = crate::W<u32, super::WL_ADDR_TYPE>;
#[doc = "Register WL_ADDR_TYPE `reset()`'s with value 0"]
impl crate::ResetValue for super::WL_ADDR_TYPE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WL_ADDR_TYPE`"]
pub type WL_ADDR_TYPE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `WL_ADDR_TYPE`"]
pub struct WL_ADDR_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> WL_ADDR_TYPE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - 8 address type bits corresponding to the device address stored. 1 - Address type is random. 0 - Address type is public."]
    #[inline(always)]
    pub fn wl_addr_type(&self) -> WL_ADDR_TYPE_R {
        WL_ADDR_TYPE_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 8 address type bits corresponding to the device address stored. 1 - Address type is random. 0 - Address type is public."]
    #[inline(always)]
    pub fn wl_addr_type(&mut self) -> WL_ADDR_TYPE_W {
        WL_ADDR_TYPE_W { w: self }
    }
}
