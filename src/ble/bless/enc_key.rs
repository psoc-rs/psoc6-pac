#[doc = "Writer for register ENC_KEY[%s]"]
pub type W = crate::W<u32, super::ENC_KEY>;
#[doc = "Register ENC_KEY[%s] `reset()`'s with value 0"]
impl crate::ResetValue for super::ENC_KEY {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `ENC_KEY`"]
pub struct ENC_KEY_W<'a> {
    w: &'a mut W,
}
impl<'a> ENC_KEY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - The encryption key / session key which is used in ECB encryption, CCM encryption and CCM decryption."]
    #[inline(always)]
    pub fn enc_key(&mut self) -> ENC_KEY_W {
        ENC_KEY_W { w: self }
    }
}
