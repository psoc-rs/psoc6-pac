#[doc = "Reader of register CLK_TRIM_CCO_CTL2"]
pub type R = crate::R<u32, super::CLK_TRIM_CCO_CTL2>;
#[doc = "Writer for register CLK_TRIM_CCO_CTL2"]
pub type W = crate::W<u32, super::CLK_TRIM_CCO_CTL2>;
#[doc = "Register CLK_TRIM_CCO_CTL2 `reset()`'s with value 0x0088_4110"]
impl crate::ResetValue for super::CLK_TRIM_CCO_CTL2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0088_4110
    }
}
#[doc = "Reader of field `CCO_FCTRIM1`"]
pub type CCO_FCTRIM1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CCO_FCTRIM1`"]
pub struct CCO_FCTRIM1_W<'a> {
    w: &'a mut W,
}
impl<'a> CCO_FCTRIM1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Reader of field `CCO_FCTRIM2`"]
pub type CCO_FCTRIM2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CCO_FCTRIM2`"]
pub struct CCO_FCTRIM2_W<'a> {
    w: &'a mut W,
}
impl<'a> CCO_FCTRIM2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 5)) | (((value as u32) & 0x1f) << 5);
        self.w
    }
}
#[doc = "Reader of field `CCO_FCTRIM3`"]
pub type CCO_FCTRIM3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CCO_FCTRIM3`"]
pub struct CCO_FCTRIM3_W<'a> {
    w: &'a mut W,
}
impl<'a> CCO_FCTRIM3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 10)) | (((value as u32) & 0x1f) << 10);
        self.w
    }
}
#[doc = "Reader of field `CCO_FCTRIM4`"]
pub type CCO_FCTRIM4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CCO_FCTRIM4`"]
pub struct CCO_FCTRIM4_W<'a> {
    w: &'a mut W,
}
impl<'a> CCO_FCTRIM4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 15)) | (((value as u32) & 0x1f) << 15);
        self.w
    }
}
#[doc = "Reader of field `CCO_FCTRIM5`"]
pub type CCO_FCTRIM5_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CCO_FCTRIM5`"]
pub struct CCO_FCTRIM5_W<'a> {
    w: &'a mut W,
}
impl<'a> CCO_FCTRIM5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 20)) | (((value as u32) & 0x1f) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - CCO frequency 1st range calibration"]
    #[inline(always)]
    pub fn cco_fctrim1(&self) -> CCO_FCTRIM1_R {
        CCO_FCTRIM1_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - CCO frequency 2nd range calibration"]
    #[inline(always)]
    pub fn cco_fctrim2(&self) -> CCO_FCTRIM2_R {
        CCO_FCTRIM2_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - CCO frequency 3rd range calibration"]
    #[inline(always)]
    pub fn cco_fctrim3(&self) -> CCO_FCTRIM3_R {
        CCO_FCTRIM3_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - CCO frequency 4th range calibration"]
    #[inline(always)]
    pub fn cco_fctrim4(&self) -> CCO_FCTRIM4_R {
        CCO_FCTRIM4_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 20:24 - CCO frequency 5th range calibration"]
    #[inline(always)]
    pub fn cco_fctrim5(&self) -> CCO_FCTRIM5_R {
        CCO_FCTRIM5_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - CCO frequency 1st range calibration"]
    #[inline(always)]
    pub fn cco_fctrim1(&mut self) -> CCO_FCTRIM1_W {
        CCO_FCTRIM1_W { w: self }
    }
    #[doc = "Bits 5:9 - CCO frequency 2nd range calibration"]
    #[inline(always)]
    pub fn cco_fctrim2(&mut self) -> CCO_FCTRIM2_W {
        CCO_FCTRIM2_W { w: self }
    }
    #[doc = "Bits 10:14 - CCO frequency 3rd range calibration"]
    #[inline(always)]
    pub fn cco_fctrim3(&mut self) -> CCO_FCTRIM3_W {
        CCO_FCTRIM3_W { w: self }
    }
    #[doc = "Bits 15:19 - CCO frequency 4th range calibration"]
    #[inline(always)]
    pub fn cco_fctrim4(&mut self) -> CCO_FCTRIM4_W {
        CCO_FCTRIM4_W { w: self }
    }
    #[doc = "Bits 20:24 - CCO frequency 5th range calibration"]
    #[inline(always)]
    pub fn cco_fctrim5(&mut self) -> CCO_FCTRIM5_W {
        CCO_FCTRIM5_W { w: self }
    }
}
