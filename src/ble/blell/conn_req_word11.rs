#[doc = "Reader of register CONN_REQ_WORD11"]
pub type R = crate::R<u32, super::CONN_REQ_WORD11>;
#[doc = "Writer for register CONN_REQ_WORD11"]
pub type W = crate::W<u32, super::CONN_REQ_WORD11>;
#[doc = "Register CONN_REQ_WORD11 `reset()`'s with value 0"]
impl crate::ResetValue for super::CONN_REQ_WORD11 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HOP_INCREMENT_2`"]
pub type HOP_INCREMENT_2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HOP_INCREMENT_2`"]
pub struct HOP_INCREMENT_2_W<'a> {
    w: &'a mut W,
}
impl<'a> HOP_INCREMENT_2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Reader of field `SCA_2`"]
pub type SCA_2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SCA_2`"]
pub struct SCA_2_W<'a> {
    w: &'a mut W,
}
impl<'a> SCA_2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | (((value as u32) & 0x07) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - This field is used for the data channel selection process."]
    #[inline(always)]
    pub fn hop_increment_2(&self) -> HOP_INCREMENT_2_R {
        HOP_INCREMENT_2_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:7 - This field defines the sleep clock accuracies given in ppm."]
    #[inline(always)]
    pub fn sca_2(&self) -> SCA_2_R {
        SCA_2_R::new(((self.bits >> 5) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - This field is used for the data channel selection process."]
    #[inline(always)]
    pub fn hop_increment_2(&mut self) -> HOP_INCREMENT_2_W {
        HOP_INCREMENT_2_W { w: self }
    }
    #[doc = "Bits 5:7 - This field defines the sleep clock accuracies given in ppm."]
    #[inline(always)]
    pub fn sca_2(&mut self) -> SCA_2_W {
        SCA_2_W { w: self }
    }
}
