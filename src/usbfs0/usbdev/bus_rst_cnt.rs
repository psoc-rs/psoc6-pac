#[doc = "Reader of register BUS_RST_CNT"]
pub type R = crate::R<u32, super::BUS_RST_CNT>;
#[doc = "Writer for register BUS_RST_CNT"]
pub type W = crate::W<u32, super::BUS_RST_CNT>;
#[doc = "Register BUS_RST_CNT `reset()`'s with value 0x0a"]
impl crate::ResetValue for super::BUS_RST_CNT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0a
    }
}
#[doc = "Reader of field `BUS_RST_CNT`"]
pub type BUS_RST_CNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BUS_RST_CNT`"]
pub struct BUS_RST_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> BUS_RST_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Bus Reset Count Length"]
    #[inline(always)]
    pub fn bus_rst_cnt(&self) -> BUS_RST_CNT_R {
        BUS_RST_CNT_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Bus Reset Count Length"]
    #[inline(always)]
    pub fn bus_rst_cnt(&mut self) -> BUS_RST_CNT_W {
        BUS_RST_CNT_W { w: self }
    }
}
