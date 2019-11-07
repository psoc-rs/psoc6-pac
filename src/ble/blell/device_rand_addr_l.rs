#[doc = "Reader of register DEVICE_RAND_ADDR_L"]
pub type R = crate::R<u32, super::DEVICE_RAND_ADDR_L>;
#[doc = "Writer for register DEVICE_RAND_ADDR_L"]
pub type W = crate::W<u32, super::DEVICE_RAND_ADDR_L>;
#[doc = "Register DEVICE_RAND_ADDR_L `reset()`'s with value 0"]
impl crate::ResetValue for super::DEVICE_RAND_ADDR_L {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DEVICE_RAND_ADDR_L`"]
pub type DEVICE_RAND_ADDR_L_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DEVICE_RAND_ADDR_L`"]
pub struct DEVICE_RAND_ADDR_L_W<'a> {
    w: &'a mut W,
}
impl<'a> DEVICE_RAND_ADDR_L_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Lower 16 bit of 48-bit random address of the device."]
    #[inline(always)]
    pub fn device_rand_addr_l(&self) -> DEVICE_RAND_ADDR_L_R {
        DEVICE_RAND_ADDR_L_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Lower 16 bit of 48-bit random address of the device."]
    #[inline(always)]
    pub fn device_rand_addr_l(&mut self) -> DEVICE_RAND_ADDR_L_W {
        DEVICE_RAND_ADDR_L_W { w: self }
    }
}
