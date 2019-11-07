#[doc = "Reader of register CLK_TRIM_PILO_CTL2"]
pub type R = crate::R<u32, super::CLK_TRIM_PILO_CTL2>;
#[doc = "Writer for register CLK_TRIM_PILO_CTL2"]
pub type W = crate::W<u32, super::CLK_TRIM_PILO_CTL2>;
#[doc = "Register CLK_TRIM_PILO_CTL2 `reset()`'s with value 0x00da_10e0"]
impl crate::ResetValue for super::CLK_TRIM_PILO_CTL2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x00da_10e0
    }
}
#[doc = "Reader of field `PILO_VREF_TRIM`"]
pub type PILO_VREF_TRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PILO_VREF_TRIM`"]
pub struct PILO_VREF_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> PILO_VREF_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `PILO_IREFBM_TRIM`"]
pub type PILO_IREFBM_TRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PILO_IREFBM_TRIM`"]
pub struct PILO_IREFBM_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> PILO_IREFBM_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "Reader of field `PILO_IREF_TRIM`"]
pub type PILO_IREF_TRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PILO_IREF_TRIM`"]
pub struct PILO_IREF_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> PILO_IREF_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Trim for voltage reference"]
    #[inline(always)]
    pub fn pilo_vref_trim(&self) -> PILO_VREF_TRIM_R {
        PILO_VREF_TRIM_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:12 - Trim for beta-multiplier current reference"]
    #[inline(always)]
    pub fn pilo_irefbm_trim(&self) -> PILO_IREFBM_TRIM_R {
        PILO_IREFBM_TRIM_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:23 - Trim for current reference"]
    #[inline(always)]
    pub fn pilo_iref_trim(&self) -> PILO_IREF_TRIM_R {
        PILO_IREF_TRIM_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Trim for voltage reference"]
    #[inline(always)]
    pub fn pilo_vref_trim(&mut self) -> PILO_VREF_TRIM_W {
        PILO_VREF_TRIM_W { w: self }
    }
    #[doc = "Bits 8:12 - Trim for beta-multiplier current reference"]
    #[inline(always)]
    pub fn pilo_irefbm_trim(&mut self) -> PILO_IREFBM_TRIM_W {
        PILO_IREFBM_TRIM_W { w: self }
    }
    #[doc = "Bits 16:23 - Trim for current reference"]
    #[inline(always)]
    pub fn pilo_iref_trim(&mut self) -> PILO_IREF_TRIM_W {
        PILO_IREF_TRIM_W { w: self }
    }
}
