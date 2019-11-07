#[doc = "Reader of register TRIM_LDO_1"]
pub type R = crate::R<u32, super::TRIM_LDO_1>;
#[doc = "Writer for register TRIM_LDO_1"]
pub type W = crate::W<u32, super::TRIM_LDO_1>;
#[doc = "Register TRIM_LDO_1 `reset()`'s with value 0x08"]
impl crate::ResetValue for super::TRIM_LDO_1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x08
    }
}
#[doc = "Reader of field `ACT_REF_BGR`"]
pub type ACT_REF_BGR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ACT_REF_BGR`"]
pub struct ACT_REF_BGR_W<'a> {
    w: &'a mut W,
}
impl<'a> ACT_REF_BGR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `SB_BGRES`"]
pub type SB_BGRES_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SB_BGRES`"]
pub struct SB_BGRES_W<'a> {
    w: &'a mut W,
}
impl<'a> SB_BGRES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - To trim active regulator reference voltage"]
    #[inline(always)]
    pub fn act_ref_bgr(&self) -> ACT_REF_BGR_R {
        ACT_REF_BGR_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - To trim standby regulator reference voltage"]
    #[inline(always)]
    pub fn sb_bgres(&self) -> SB_BGRES_R {
        SB_BGRES_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - To trim active regulator reference voltage"]
    #[inline(always)]
    pub fn act_ref_bgr(&mut self) -> ACT_REF_BGR_W {
        ACT_REF_BGR_W { w: self }
    }
    #[doc = "Bits 4:7 - To trim standby regulator reference voltage"]
    #[inline(always)]
    pub fn sb_bgres(&mut self) -> SB_BGRES_W {
        SB_BGRES_W { w: self }
    }
}
