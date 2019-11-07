#[doc = "Reader of register DEV_PA_ADDR_H"]
pub type R = crate::R<u32, super::DEV_PA_ADDR_H>;
#[doc = "Writer for register DEV_PA_ADDR_H"]
pub type W = crate::W<u32, super::DEV_PA_ADDR_H>;
#[doc = "Register DEV_PA_ADDR_H `reset()`'s with value 0"]
impl crate::ResetValue for super::DEV_PA_ADDR_H {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DEV_PA_ADDR_H`"]
pub type DEV_PA_ADDR_H_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DEV_PA_ADDR_H`"]
pub struct DEV_PA_ADDR_H_W<'a> {
    w: &'a mut W,
}
impl<'a> DEV_PA_ADDR_H_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Higher 16 bit of 48-bit Random Private address of the device."]
    #[inline(always)]
    pub fn dev_pa_addr_h(&self) -> DEV_PA_ADDR_H_R {
        DEV_PA_ADDR_H_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Higher 16 bit of 48-bit Random Private address of the device."]
    #[inline(always)]
    pub fn dev_pa_addr_h(&mut self) -> DEV_PA_ADDR_H_W {
        DEV_PA_ADDR_H_W { w: self }
    }
}
