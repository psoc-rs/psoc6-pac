#[doc = "Reader of register SAMPLE_TIME23"]
pub type R = crate::R<u32, super::SAMPLE_TIME23>;
#[doc = "Writer for register SAMPLE_TIME23"]
pub type W = crate::W<u32, super::SAMPLE_TIME23>;
#[doc = "Register SAMPLE_TIME23 `reset()`'s with value 0x0003_0003"]
impl crate::ResetValue for super::SAMPLE_TIME23 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0003_0003
    }
}
#[doc = "Reader of field `SAMPLE_TIME2`"]
pub type SAMPLE_TIME2_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SAMPLE_TIME2`"]
pub struct SAMPLE_TIME2_W<'a> {
    w: &'a mut W,
}
impl<'a> SAMPLE_TIME2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
#[doc = "Reader of field `SAMPLE_TIME3`"]
pub type SAMPLE_TIME3_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SAMPLE_TIME3`"]
pub struct SAMPLE_TIME3_W<'a> {
    w: &'a mut W,
}
impl<'a> SAMPLE_TIME3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 16)) | (((value as u32) & 0x03ff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - Sample time2"]
    #[inline(always)]
    pub fn sample_time2(&self) -> SAMPLE_TIME2_R {
        SAMPLE_TIME2_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Sample time3"]
    #[inline(always)]
    pub fn sample_time3(&self) -> SAMPLE_TIME3_R {
        SAMPLE_TIME3_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Sample time2"]
    #[inline(always)]
    pub fn sample_time2(&mut self) -> SAMPLE_TIME2_W {
        SAMPLE_TIME2_W { w: self }
    }
    #[doc = "Bits 16:25 - Sample time3"]
    #[inline(always)]
    pub fn sample_time3(&mut self) -> SAMPLE_TIME3_W {
        SAMPLE_TIME3_W { w: self }
    }
}
