#[doc = "Register `ENC_CONFIG` reader"]
pub struct R(crate::R<ENC_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ENC_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ENC_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ENC_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ENC_CONFIG` writer"]
pub struct W(crate::W<ENC_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ENC_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<ENC_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ENC_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `START_PROC` reader - 1 Start the AES processing"]
pub type START_PROC_R = crate::BitReader<bool>;
#[doc = "Field `START_PROC` writer - 1 Start the AES processing"]
pub type START_PROC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ENC_CONFIG_SPEC, bool, O>;
#[doc = "Field `ECB_CCM` reader - 0 - CCM 1 - ECB"]
pub type ECB_CCM_R = crate::BitReader<bool>;
#[doc = "Field `ECB_CCM` writer - 0 - CCM 1 - ECB"]
pub type ECB_CCM_W<'a, const O: u8> = crate::BitWriter<'a, u32, ENC_CONFIG_SPEC, bool, O>;
#[doc = "Field `DEC_ENC` reader - Decryption/Encryption 0 - Encrypt 1 - Decrypt"]
pub type DEC_ENC_R = crate::BitReader<bool>;
#[doc = "Field `DEC_ENC` writer - Decryption/Encryption 0 - Encrypt 1 - Decrypt"]
pub type DEC_ENC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ENC_CONFIG_SPEC, bool, O>;
#[doc = "Field `PAYLOAD_LENGTH_MSB` reader - MS byte of the length of the input data when B0 needs to be completely configurable. Valid only when AES_B0_DATA_OVERRIDE is enabled. When AES_B0_DATA_OVERRIDE is enabled total ENC payload length = {PAYLOAD_LENGTH_MSB, PAYLOAD_LENGTH_MSB, PAYLOAD_LENGTH}"]
pub type PAYLOAD_LENGTH_MSB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PAYLOAD_LENGTH_MSB` writer - MS byte of the length of the input data when B0 needs to be completely configurable. Valid only when AES_B0_DATA_OVERRIDE is enabled. When AES_B0_DATA_OVERRIDE is enabled total ENC payload length = {PAYLOAD_LENGTH_MSB, PAYLOAD_LENGTH_MSB, PAYLOAD_LENGTH}"]
pub type PAYLOAD_LENGTH_MSB_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ENC_CONFIG_SPEC, u8, u8, 8, O>;
#[doc = "Field `B0_FLAGS` reader - LS byte of the input data when B0 needs to be completely configurable. Valid only when AES_B0_DATA_OVERRIDE is enabled."]
pub type B0_FLAGS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `B0_FLAGS` writer - LS byte of the input data when B0 needs to be completely configurable. Valid only when AES_B0_DATA_OVERRIDE is enabled."]
pub type B0_FLAGS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ENC_CONFIG_SPEC, u8, u8, 8, O>;
#[doc = "Field `AES_B0_DATA_OVERRIDE` reader - Configuration to use B0 DATA provided by FW for CCM computation"]
pub type AES_B0_DATA_OVERRIDE_R = crate::BitReader<bool>;
#[doc = "Field `AES_B0_DATA_OVERRIDE` writer - Configuration to use B0 DATA provided by FW for CCM computation"]
pub type AES_B0_DATA_OVERRIDE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ENC_CONFIG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - 1 Start the AES processing"]
    #[inline(always)]
    pub fn start_proc(&self) -> START_PROC_R {
        START_PROC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 0 - CCM 1 - ECB"]
    #[inline(always)]
    pub fn ecb_ccm(&self) -> ECB_CCM_R {
        ECB_CCM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Decryption/Encryption 0 - Encrypt 1 - Decrypt"]
    #[inline(always)]
    pub fn dec_enc(&self) -> DEC_ENC_R {
        DEC_ENC_R::new(((self.bits >> 2) & 1) != 0)
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
        AES_B0_DATA_OVERRIDE_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1 Start the AES processing"]
    #[inline(always)]
    pub fn start_proc(&mut self) -> START_PROC_W<0> {
        START_PROC_W::new(self)
    }
    #[doc = "Bit 1 - 0 - CCM 1 - ECB"]
    #[inline(always)]
    pub fn ecb_ccm(&mut self) -> ECB_CCM_W<1> {
        ECB_CCM_W::new(self)
    }
    #[doc = "Bit 2 - Decryption/Encryption 0 - Encrypt 1 - Decrypt"]
    #[inline(always)]
    pub fn dec_enc(&mut self) -> DEC_ENC_W<2> {
        DEC_ENC_W::new(self)
    }
    #[doc = "Bits 8:15 - MS byte of the length of the input data when B0 needs to be completely configurable. Valid only when AES_B0_DATA_OVERRIDE is enabled. When AES_B0_DATA_OVERRIDE is enabled total ENC payload length = {PAYLOAD_LENGTH_MSB, PAYLOAD_LENGTH_MSB, PAYLOAD_LENGTH}"]
    #[inline(always)]
    pub fn payload_length_msb(&mut self) -> PAYLOAD_LENGTH_MSB_W<8> {
        PAYLOAD_LENGTH_MSB_W::new(self)
    }
    #[doc = "Bits 16:23 - LS byte of the input data when B0 needs to be completely configurable. Valid only when AES_B0_DATA_OVERRIDE is enabled."]
    #[inline(always)]
    pub fn b0_flags(&mut self) -> B0_FLAGS_W<16> {
        B0_FLAGS_W::new(self)
    }
    #[doc = "Bit 24 - Configuration to use B0 DATA provided by FW for CCM computation"]
    #[inline(always)]
    pub fn aes_b0_data_override(&mut self) -> AES_B0_DATA_OVERRIDE_W<24> {
        AES_B0_DATA_OVERRIDE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Encryption Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [enc_config](index.html) module"]
pub struct ENC_CONFIG_SPEC;
impl crate::RegisterSpec for ENC_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [enc_config::R](R) reader structure"]
impl crate::Readable for ENC_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [enc_config::W](W) writer structure"]
impl crate::Writable for ENC_CONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ENC_CONFIG to value 0"]
impl crate::Resettable for ENC_CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
