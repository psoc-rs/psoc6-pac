#[doc = "Reader of register DEV_PA_ADDR_L"]
pub type R = crate::R<u32, super::DEV_PA_ADDR_L>;
#[doc = "Writer for register DEV_PA_ADDR_L"]
pub type W = crate::W<u32, super::DEV_PA_ADDR_L>;
#[doc = "Register DEV_PA_ADDR_L `reset()`'s with value 0x3412"]
impl crate::ResetValue for super::DEV_PA_ADDR_L {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x3412
    }
}
#[doc = "Reader of field `DEV_PA_ADDR_L`"]
pub type DEV_PA_ADDR_L_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DEV_PA_ADDR_L`"]
pub struct DEV_PA_ADDR_L_W<'a> {
    w: &'a mut W,
}
impl<'a> DEV_PA_ADDR_L_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Lower 16 bit of 48-bit Random Private address of the device."]
    #[inline(always)]
    pub fn dev_pa_addr_l(&self) -> DEV_PA_ADDR_L_R {
        DEV_PA_ADDR_L_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Lower 16 bit of 48-bit Random Private address of the device."]
    #[inline(always)]
    pub fn dev_pa_addr_l(&mut self) -> DEV_PA_ADDR_L_W {
        DEV_PA_ADDR_L_W { w: self }
    }
}
