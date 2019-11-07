#[doc = "Reader of register SIE_EP5_CNT1"]
pub type R = crate::R<u32, super::SIE_EP5_CNT1>;
#[doc = "Writer for register SIE_EP5_CNT1"]
pub type W = crate::W<u32, super::SIE_EP5_CNT1>;
#[doc = "Register SIE_EP5_CNT1 `reset()`'s with value 0"]
impl crate::ResetValue for super::SIE_EP5_CNT1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DATA_COUNT`"]
pub type DATA_COUNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATA_COUNT`"]
pub struct DATA_COUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_COUNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - These bits are the 8 LSb of a 11-bit counter. The 3 MSb bits are in the CNT0 register. The 11-bit count indicates the number of data bytes in a transaction."]
    #[inline(always)]
    pub fn data_count(&self) -> DATA_COUNT_R {
        DATA_COUNT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - These bits are the 8 LSb of a 11-bit counter. The 3 MSb bits are in the CNT0 register. The 11-bit count indicates the number of data bytes in a transaction."]
    #[inline(always)]
    pub fn data_count(&mut self) -> DATA_COUNT_W {
        DATA_COUNT_W { w: self }
    }
}
