#[doc = "Reader of register ENC_CONFIG"]
pub type R = crate::R<u32, super::ENC_CONFIG>;
#[doc = "Writer for register ENC_CONFIG"]
pub type W = crate::W<u32, super::ENC_CONFIG>;
#[doc = "Register ENC_CONFIG `reset()`'s with value 0"]
impl crate::ResetValue for super::ENC_CONFIG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `START_PROC`"]
pub type START_PROC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `START_PROC`"]
pub struct START_PROC_W<'a> {
    w: &'a mut W,
}
impl<'a> START_PROC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `ECB_CCM`"]
pub type ECB_CCM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ECB_CCM`"]
pub struct ECB_CCM_W<'a> {
    w: &'a mut W,
}
impl<'a> ECB_CCM_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `DEC_ENC`"]
pub type DEC_ENC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DEC_ENC`"]
pub struct DEC_ENC_W<'a> {
    w: &'a mut W,
}
impl<'a> DEC_ENC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `PAYLOAD_LENGTH_MSB`"]
pub type PAYLOAD_LENGTH_MSB_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PAYLOAD_LENGTH_MSB`"]
pub struct PAYLOAD_LENGTH_MSB_W<'a> {
    w: &'a mut W,
}
impl<'a> PAYLOAD_LENGTH_MSB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `B0_FLAGS`"]
pub type B0_FLAGS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `B0_FLAGS`"]
pub struct B0_FLAGS_W<'a> {
    w: &'a mut W,
}
impl<'a> B0_FLAGS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `AES_B0_DATA_OVERRIDE`"]
pub type AES_B0_DATA_OVERRIDE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AES_B0_DATA_OVERRIDE`"]
pub struct AES_B0_DATA_OVERRIDE_W<'a> {
    w: &'a mut W,
}
impl<'a> AES_B0_DATA_OVERRIDE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - 1 Start the AES processing"]
    #[inline(always)]
    pub fn start_proc(&self) -> START_PROC_R {
        START_PROC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - 0 - CCM 1 - ECB"]
    #[inline(always)]
    pub fn ecb_ccm(&self) -> ECB_CCM_R {
        ECB_CCM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Decryption/Encryption 0 - Encrypt 1 - Decrypt"]
    #[inline(always)]
    pub fn dec_enc(&self) -> DEC_ENC_R {
        DEC_ENC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - MS byte of the length of the input data when B0 needs to be completely configurable. Valid only when AES_B0_DATA_OVERRIDE is enabled. When AES_B0_DATA_OVERRIDE is enabled total ENC payload length = {PAYLOAD_LENGTH_MSB, PAYLOAD_LENGTH_MSB, PAYLOAD_LENGTH}"]
    #[inline(always)]
    pub fn payload_length_msb(&self) -> PAYLOAD_LENGTH_MSB_R {
        PAYLOAD_LENGTH_MSB_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - LS byte of the input data when B0 needs to be completely configurable. Valid only when AES_B0_DATA_OVERRIDE is enabled."]
    #[inline(always)]
    pub fn b0_flags(&self) -> B0_FLAGS_R {
        B0_FLAGS_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Configuration to use B0 DATA provided by FW for CCM computation"]
    #[inline(always)]
    pub fn aes_b0_data_override(&self) -> AES_B0_DATA_OVERRIDE_R {
        AES_B0_DATA_OVERRIDE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1 Start the AES processing"]
    #[inline(always)]
    pub fn start_proc(&mut self) -> START_PROC_W {
        START_PROC_W { w: self }
    }
    #[doc = "Bit 1 - 0 - CCM 1 - ECB"]
    #[inline(always)]
    pub fn ecb_ccm(&mut self) -> ECB_CCM_W {
        ECB_CCM_W { w: self }
    }
    #[doc = "Bit 2 - Decryption/Encryption 0 - Encrypt 1 - Decrypt"]
    #[inline(always)]
    pub fn dec_enc(&mut self) -> DEC_ENC_W {
        DEC_ENC_W { w: self }
    }
    #[doc = "Bits 8:15 - MS byte of the length of the input data when B0 needs to be completely configurable. Valid only when AES_B0_DATA_OVERRIDE is enabled. When AES_B0_DATA_OVERRIDE is enabled total ENC payload length = {PAYLOAD_LENGTH_MSB, PAYLOAD_LENGTH_MSB, PAYLOAD_LENGTH}"]
    #[inline(always)]
    pub fn payload_length_msb(&mut self) -> PAYLOAD_LENGTH_MSB_W {
        PAYLOAD_LENGTH_MSB_W { w: self }
    }
    #[doc = "Bits 16:23 - LS byte of the input data when B0 needs to be completely configurable. Valid only when AES_B0_DATA_OVERRIDE is enabled."]
    #[inline(always)]
    pub fn b0_flags(&mut self) -> B0_FLAGS_W {
        B0_FLAGS_W { w: self }
    }
    #[doc = "Bit 24 - Configuration to use B0 DATA provided by FW for CCM computation"]
    #[inline(always)]
    pub fn aes_b0_data_override(&mut self) -> AES_B0_DATA_OVERRIDE_W {
        AES_B0_DATA_OVERRIDE_W { w: self }
    }
}
