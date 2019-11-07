#[doc = "Reader of register CONN_INDEX"]
pub type R = crate::R<u32, super::CONN_INDEX>;
#[doc = "Writer for register CONN_INDEX"]
pub type W = crate::W<u32, super::CONN_INDEX>;
#[doc = "Register CONN_INDEX `reset()`'s with value 0"]
impl crate::ResetValue for super::CONN_INDEX {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CONN_INDEX`"]
pub type CONN_INDEX_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CONN_INDEX`"]
pub struct CONN_INDEX_W<'a> {
    w: &'a mut W,
}
impl<'a> CONN_INDEX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - This field is used to index the multiple connections existing. Range is 0 to maximum number of connections supported. For a single connection device, conn_index is 0."]
    #[inline(always)]
    pub fn conn_index(&self) -> CONN_INDEX_R {
        CONN_INDEX_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This field is used to index the multiple connections existing. Range is 0 to maximum number of connections supported. For a single connection device, conn_index is 0."]
    #[inline(always)]
    pub fn conn_index(&mut self) -> CONN_INDEX_W {
        CONN_INDEX_W { w: self }
    }
}
