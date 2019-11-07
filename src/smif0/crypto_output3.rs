#[doc = "Reader of register CRYPTO_OUTPUT3"]
pub type R = crate::R<u32, super::CRYPTO_OUTPUT3>;
#[doc = "Writer for register CRYPTO_OUTPUT3"]
pub type W = crate::W<u32, super::CRYPTO_OUTPUT3>;
#[doc = "Register CRYPTO_OUTPUT3 `reset()`'s with value 0"]
impl crate::ResetValue for super::CRYPTO_OUTPUT3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OUTPUT`"]
pub type OUTPUT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `OUTPUT`"]
pub struct OUTPUT_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTPUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Four Bytes of the ciphertext CT\\[127:96\\] = CRYPTO_OUTPUT3.OUTPUT\\[31:0\\]."]
    #[inline(always)]
    pub fn output(&self) -> OUTPUT_R {
        OUTPUT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Four Bytes of the ciphertext CT\\[127:96\\] = CRYPTO_OUTPUT3.OUTPUT\\[31:0\\]."]
    #[inline(always)]
    pub fn output(&mut self) -> OUTPUT_W {
        OUTPUT_W { w: self }
    }
}
