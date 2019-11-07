#[doc = "Reader of register SEQ_TIME"]
pub type R = crate::R<u32, super::SEQ_TIME>;
#[doc = "Writer for register SEQ_TIME"]
pub type W = crate::W<u32, super::SEQ_TIME>;
#[doc = "Register SEQ_TIME `reset()`'s with value 0"]
impl crate::ResetValue for super::SEQ_TIME {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `AZ_TIME`"]
pub type AZ_TIME_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AZ_TIME`"]
pub struct AZ_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> AZ_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Define Auto-Zero time in csd_sense cycles -1."]
    #[inline(always)]
    pub fn az_time(&self) -> AZ_TIME_R {
        AZ_TIME_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Define Auto-Zero time in csd_sense cycles -1."]
    #[inline(always)]
    pub fn az_time(&mut self) -> AZ_TIME_W {
        AZ_TIME_W { w: self }
    }
}
