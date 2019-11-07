#[doc = "Reader of register CLK_FLL_CONFIG2"]
pub type R = crate::R<u32, super::CLK_FLL_CONFIG2>;
#[doc = "Writer for register CLK_FLL_CONFIG2"]
pub type W = crate::W<u32, super::CLK_FLL_CONFIG2>;
#[doc = "Register CLK_FLL_CONFIG2 `reset()`'s with value 0x0002_0001"]
impl crate::ResetValue for super::CLK_FLL_CONFIG2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0002_0001
    }
}
#[doc = "Reader of field `FLL_REF_DIV`"]
pub type FLL_REF_DIV_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `FLL_REF_DIV`"]
pub struct FLL_REF_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> FLL_REF_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1fff) | ((value as u32) & 0x1fff);
        self.w
    }
}
#[doc = "Reader of field `LOCK_TOL`"]
pub type LOCK_TOL_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `LOCK_TOL`"]
pub struct LOCK_TOL_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_TOL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 16)) | (((value as u32) & 0x01ff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:12 - Control bits for reference divider. Set the divide value before enabling the FLL, and do not change it while FLL is enabled. 0: illegal (undefined behavior) 1: divide by 1 ... 8191: divide by 8191"]
    #[inline(always)]
    pub fn fll_ref_div(&self) -> FLL_REF_DIV_R {
        FLL_REF_DIV_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:24 - Lock tolerance sets the error threshold for when the FLL output is considered locked to the reference input. A high tolerance can be used to lock more quickly or to track a less accurate source. The tolerance should be set so that the FLL does not unlock under normal conditions. The tolerance is the allowed difference between the count value for the ideal formula and the measured value. 0: tolerate error of 1 count value 1: tolerate error of 2 count values ... 511: tolerate error of 512 count values"]
    #[inline(always)]
    pub fn lock_tol(&self) -> LOCK_TOL_R {
        LOCK_TOL_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - Control bits for reference divider. Set the divide value before enabling the FLL, and do not change it while FLL is enabled. 0: illegal (undefined behavior) 1: divide by 1 ... 8191: divide by 8191"]
    #[inline(always)]
    pub fn fll_ref_div(&mut self) -> FLL_REF_DIV_W {
        FLL_REF_DIV_W { w: self }
    }
    #[doc = "Bits 16:24 - Lock tolerance sets the error threshold for when the FLL output is considered locked to the reference input. A high tolerance can be used to lock more quickly or to track a less accurate source. The tolerance should be set so that the FLL does not unlock under normal conditions. The tolerance is the allowed difference between the count value for the ideal formula and the measured value. 0: tolerate error of 1 count value 1: tolerate error of 2 count values ... 511: tolerate error of 512 count values"]
    #[inline(always)]
    pub fn lock_tol(&mut self) -> LOCK_TOL_W {
        LOCK_TOL_W { w: self }
    }
}
