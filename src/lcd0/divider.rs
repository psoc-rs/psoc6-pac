#[doc = "Reader of register DIVIDER"]
pub type R = crate::R<u32, super::DIVIDER>;
#[doc = "Writer for register DIVIDER"]
pub type W = crate::W<u32, super::DIVIDER>;
#[doc = "Register DIVIDER `reset()`'s with value 0"]
impl crate::ResetValue for super::DIVIDER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SUBFR_DIV`"]
pub type SUBFR_DIV_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SUBFR_DIV`"]
pub struct SUBFR_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> SUBFR_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `DEAD_DIV`"]
pub type DEAD_DIV_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DEAD_DIV`"]
pub struct DEAD_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> DEAD_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Input clock frequency divide value, to generate the 1/4 sub-frame period. The sub-frame period is 4*(SUBFR_DIV+1) cycles long."]
    #[inline(always)]
    pub fn subfr_div(&self) -> SUBFR_DIV_R {
        SUBFR_DIV_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Length of the dead time period in cycles. When set to zero, no dead time period exists."]
    #[inline(always)]
    pub fn dead_div(&self) -> DEAD_DIV_R {
        DEAD_DIV_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Input clock frequency divide value, to generate the 1/4 sub-frame period. The sub-frame period is 4*(SUBFR_DIV+1) cycles long."]
    #[inline(always)]
    pub fn subfr_div(&mut self) -> SUBFR_DIV_W {
        SUBFR_DIV_W { w: self }
    }
    #[doc = "Bits 16:31 - Length of the dead time period in cycles. When set to zero, no dead time period exists."]
    #[inline(always)]
    pub fn dead_div(&mut self) -> DEAD_DIV_W {
        DEAD_DIV_W { w: self }
    }
}
