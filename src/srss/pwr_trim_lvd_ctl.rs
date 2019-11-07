#[doc = "Reader of register PWR_TRIM_LVD_CTL"]
pub type R = crate::R<u32, super::PWR_TRIM_LVD_CTL>;
#[doc = "Writer for register PWR_TRIM_LVD_CTL"]
pub type W = crate::W<u32, super::PWR_TRIM_LVD_CTL>;
#[doc = "Register PWR_TRIM_LVD_CTL `reset()`'s with value 0x20"]
impl crate::ResetValue for super::PWR_TRIM_LVD_CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x20
    }
}
#[doc = "Reader of field `HVLVD1_OFSTRIM`"]
pub type HVLVD1_OFSTRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HVLVD1_OFSTRIM`"]
pub struct HVLVD1_OFSTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> HVLVD1_OFSTRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `HVLVD1_ITRIM`"]
pub type HVLVD1_ITRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HVLVD1_ITRIM`"]
pub struct HVLVD1_ITRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> HVLVD1_ITRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - HVLVD1 offset trim"]
    #[inline(always)]
    pub fn hvlvd1_ofstrim(&self) -> HVLVD1_OFSTRIM_R {
        HVLVD1_OFSTRIM_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:6 - HVLVD1 current trim"]
    #[inline(always)]
    pub fn hvlvd1_itrim(&self) -> HVLVD1_ITRIM_R {
        HVLVD1_ITRIM_R::new(((self.bits >> 4) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - HVLVD1 offset trim"]
    #[inline(always)]
    pub fn hvlvd1_ofstrim(&mut self) -> HVLVD1_OFSTRIM_W {
        HVLVD1_OFSTRIM_W { w: self }
    }
    #[doc = "Bits 4:6 - HVLVD1 current trim"]
    #[inline(always)]
    pub fn hvlvd1_itrim(&mut self) -> HVLVD1_ITRIM_W {
        HVLVD1_ITRIM_W { w: self }
    }
}
