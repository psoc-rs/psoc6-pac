#[doc = "Reader of register IV_MASTER0"]
pub type R = crate::R<u32, super::IV_MASTER0>;
#[doc = "Writer for register IV_MASTER0"]
pub type W = crate::W<u32, super::IV_MASTER0>;
#[doc = "Register IV_MASTER0 `reset()`'s with value 0"]
impl crate::ResetValue for super::IV_MASTER0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IV_MASTER`"]
pub type IV_MASTER_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `IV_MASTER`"]
pub struct IV_MASTER_W<'a> {
    w: &'a mut W,
}
impl<'a> IV_MASTER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - This is the IVm field, which contains the master's portion of the initialization vector."]
    #[inline(always)]
    pub fn iv_master(&self) -> IV_MASTER_R {
        IV_MASTER_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - This is the IVm field, which contains the master's portion of the initialization vector."]
    #[inline(always)]
    pub fn iv_master(&mut self) -> IV_MASTER_W {
        IV_MASTER_W { w: self }
    }
}
