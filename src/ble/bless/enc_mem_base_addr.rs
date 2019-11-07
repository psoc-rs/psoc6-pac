#[doc = "Reader of register ENC_MEM_BASE_ADDR"]
pub type R = crate::R<u32, super::ENC_MEM_BASE_ADDR>;
#[doc = "Writer for register ENC_MEM_BASE_ADDR"]
pub type W = crate::W<u32, super::ENC_MEM_BASE_ADDR>;
#[doc = "Register ENC_MEM_BASE_ADDR `reset()`'s with value 0"]
impl crate::ResetValue for super::ENC_MEM_BASE_ADDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ENC_MEM`"]
pub type ENC_MEM_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ENC_MEM`"]
pub struct ENC_MEM_W<'a> {
    w: &'a mut W,
}
impl<'a> ENC_MEM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Data values written to Enc memory are written as 16-bit wide data. This memory is valid only if DLE is set."]
    #[inline(always)]
    pub fn enc_mem(&self) -> ENC_MEM_R {
        ENC_MEM_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data values written to Enc memory are written as 16-bit wide data. This memory is valid only if DLE is set."]
    #[inline(always)]
    pub fn enc_mem(&mut self) -> ENC_MEM_W {
        ENC_MEM_W { w: self }
    }
}
