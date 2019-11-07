#[doc = "Reader of register RANGE_THRES"]
pub type R = crate::R<u32, super::RANGE_THRES>;
#[doc = "Writer for register RANGE_THRES"]
pub type W = crate::W<u32, super::RANGE_THRES>;
#[doc = "Register RANGE_THRES `reset()`'s with value 0"]
impl crate::ResetValue for super::RANGE_THRES {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RANGE_LOW`"]
pub type RANGE_LOW_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RANGE_LOW`"]
pub struct RANGE_LOW_W<'a> {
    w: &'a mut W,
}
impl<'a> RANGE_LOW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `RANGE_HIGH`"]
pub type RANGE_HIGH_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RANGE_HIGH`"]
pub struct RANGE_HIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> RANGE_HIGH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Low threshold for range detect."]
    #[inline(always)]
    pub fn range_low(&self) -> RANGE_LOW_R {
        RANGE_LOW_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - High threshold for range detect."]
    #[inline(always)]
    pub fn range_high(&self) -> RANGE_HIGH_R {
        RANGE_HIGH_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Low threshold for range detect."]
    #[inline(always)]
    pub fn range_low(&mut self) -> RANGE_LOW_W {
        RANGE_LOW_W { w: self }
    }
    #[doc = "Bits 16:31 - High threshold for range detect."]
    #[inline(always)]
    pub fn range_high(&mut self) -> RANGE_HIGH_W {
        RANGE_HIGH_W { w: self }
    }
}
