#[doc = "Reader of register MIC_IN0"]
pub type R = crate::R<u32, super::MIC_IN0>;
#[doc = "Writer for register MIC_IN0"]
pub type W = crate::W<u32, super::MIC_IN0>;
#[doc = "Register MIC_IN0 `reset()`'s with value 0"]
impl crate::ResetValue for super::MIC_IN0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MIC_IN`"]
pub type MIC_IN_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `MIC_IN`"]
pub struct MIC_IN_W<'a> {
    w: &'a mut W,
}
impl<'a> MIC_IN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - This is the MIC field used for CCM decryption."]
    #[inline(always)]
    pub fn mic_in(&self) -> MIC_IN_R {
        MIC_IN_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - This is the MIC field used for CCM decryption."]
    #[inline(always)]
    pub fn mic_in(&mut self) -> MIC_IN_W {
        MIC_IN_W { w: self }
    }
}
