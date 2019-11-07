#[doc = "Reader of register VREF_TRIM3"]
pub type R = crate::R<u32, super::VREF_TRIM3>;
#[doc = "Writer for register VREF_TRIM3"]
pub type W = crate::W<u32, super::VREF_TRIM3>;
#[doc = "Register VREF_TRIM3 `reset()`'s with value 0"]
impl crate::ResetValue for super::VREF_TRIM3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `VREF_ATTEN_TRIM`"]
pub type VREF_ATTEN_TRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VREF_ATTEN_TRIM`"]
pub struct VREF_ATTEN_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> VREF_ATTEN_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Obsolete"]
    #[inline(always)]
    pub fn vref_atten_trim(&self) -> VREF_ATTEN_TRIM_R {
        VREF_ATTEN_TRIM_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Obsolete"]
    #[inline(always)]
    pub fn vref_atten_trim(&mut self) -> VREF_ATTEN_TRIM_W {
        VREF_ATTEN_TRIM_W { w: self }
    }
}
