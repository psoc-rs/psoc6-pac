#[doc = "Reader of register IV_SLAVE0"]
pub type R = crate::R<u32, super::IV_SLAVE0>;
#[doc = "Writer for register IV_SLAVE0"]
pub type W = crate::W<u32, super::IV_SLAVE0>;
#[doc = "Register IV_SLAVE0 `reset()`'s with value 0"]
impl crate::ResetValue for super::IV_SLAVE0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IV_SLAVE`"]
pub type IV_SLAVE_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `IV_SLAVE`"]
pub struct IV_SLAVE_W<'a> {
    w: &'a mut W,
}
impl<'a> IV_SLAVE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - This is the IVs field, which contains the slave's portion of the initialization vector."]
    #[inline(always)]
    pub fn iv_slave(&self) -> IV_SLAVE_R {
        IV_SLAVE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - This is the IVs field, which contains the slave's portion of the initialization vector."]
    #[inline(always)]
    pub fn iv_slave(&mut self) -> IV_SLAVE_W {
        IV_SLAVE_W { w: self }
    }
}
