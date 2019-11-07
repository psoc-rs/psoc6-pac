#[doc = "Reader of register SEQ_INIT_CNT"]
pub type R = crate::R<u32, super::SEQ_INIT_CNT>;
#[doc = "Writer for register SEQ_INIT_CNT"]
pub type W = crate::W<u32, super::SEQ_INIT_CNT>;
#[doc = "Register SEQ_INIT_CNT `reset()`'s with value 0"]
impl crate::ResetValue for super::SEQ_INIT_CNT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CONV_CNT`"]
pub type CONV_CNT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CONV_CNT`"]
pub struct CONV_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> CONV_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Number of conversion per Initialization sample, if set to 0 the Sample_init state will be skipped."]
    #[inline(always)]
    pub fn conv_cnt(&self) -> CONV_CNT_R {
        CONV_CNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Number of conversion per Initialization sample, if set to 0 the Sample_init state will be skipped."]
    #[inline(always)]
    pub fn conv_cnt(&mut self) -> CONV_CNT_W {
        CONV_CNT_W { w: self }
    }
}
