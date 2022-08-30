#[doc = "Register `ENC_PARAMS` reader"]
pub struct R(crate::R<ENC_PARAMS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ENC_PARAMS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ENC_PARAMS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ENC_PARAMS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ENC_PARAMS` writer"]
pub struct W(crate::W<ENC_PARAMS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ENC_PARAMS_SPEC>;
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
impl From<crate::W<ENC_PARAMS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ENC_PARAMS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA_PDU_HEADER` reader - LLID of the packet."]
pub type DATA_PDU_HEADER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA_PDU_HEADER` writer - LLID of the packet."]
pub type DATA_PDU_HEADER_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ENC_PARAMS_SPEC, u8, u8, 2, O>;
#[doc = "Field `PAYLOAD_LENGTH_LSB` reader - Length of the input data."]
pub type PAYLOAD_LENGTH_LSB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PAYLOAD_LENGTH_LSB` writer - Length of the input data."]
pub type PAYLOAD_LENGTH_LSB_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ENC_PARAMS_SPEC, u8, u8, 5, O>;
#[doc = "Field `DIRECTION` reader - The directionBit shall be set to '1' for Data Channel PDUs sent by the master and set to '0' for Data Channel PDUs sent by the slave."]
pub type DIRECTION_R = crate::BitReader<bool>;
#[doc = "Field `DIRECTION` writer - The directionBit shall be set to '1' for Data Channel PDUs sent by the master and set to '0' for Data Channel PDUs sent by the slave."]
pub type DIRECTION_W<'a, const O: u8> = crate::BitWriter<'a, u32, ENC_PARAMS_SPEC, bool, O>;
#[doc = "Field `PAYLOAD_LENGTH_LSB_EXT` reader - 3 Most significant bits of the LS byte of the length of the input data. Valid only when DLE is enabled. When DLE is enabled total ENC payload length = {PAYLOAD_LENGTH_LSB_EXT, PAYLOAD_LENGTH_LSB}"]
pub type PAYLOAD_LENGTH_LSB_EXT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PAYLOAD_LENGTH_LSB_EXT` writer - 3 Most significant bits of the LS byte of the length of the input data. Valid only when DLE is enabled. When DLE is enabled total ENC payload length = {PAYLOAD_LENGTH_LSB_EXT, PAYLOAD_LENGTH_LSB}"]
pub type PAYLOAD_LENGTH_LSB_EXT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ENC_PARAMS_SPEC, u8, u8, 3, O>;
#[doc = "Field `MEM_LATENCY_HIDE` reader - Controls the encryption memory access mode. Valid only when DLE is enabled. 0 - The AES is idle while memory fetch/store in progress. 1- The AES is pipelined while memory fetch/store in progress."]
pub type MEM_LATENCY_HIDE_R = crate::BitReader<bool>;
#[doc = "Field `MEM_LATENCY_HIDE` writer - Controls the encryption memory access mode. Valid only when DLE is enabled. 0 - The AES is idle while memory fetch/store in progress. 1- The AES is pipelined while memory fetch/store in progress."]
pub type MEM_LATENCY_HIDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ENC_PARAMS_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - LLID of the packet."]
    #[inline(always)]
    pub fn data_pdu_header(&self) -> DATA_PDU_HEADER_R {
        DATA_PDU_HEADER_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:6 - Length of the input data."]
    #[inline(always)]
    pub fn payload_length_lsb(&self) -> PAYLOAD_LENGTH_LSB_R {
        PAYLOAD_LENGTH_LSB_R::new(((self.bits >> 2) & 0x1f) as u8)
    }
    #[doc = "Bit 7 - The directionBit shall be set to '1' for Data Channel PDUs sent by the master and set to '0' for Data Channel PDUs sent by the slave."]
    #[inline(always)]
    pub fn direction(&self) -> DIRECTION_R {
        DIRECTION_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - 3 Most significant bits of the LS byte of the length of the input data. Valid only when DLE is enabled. When DLE is enabled total ENC payload length = {PAYLOAD_LENGTH_LSB_EXT, PAYLOAD_LENGTH_LSB}"]
    #[inline(always)]
    pub fn payload_length_lsb_ext(&self) -> PAYLOAD_LENGTH_LSB_EXT_R {
        PAYLOAD_LENGTH_LSB_EXT_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - Controls the encryption memory access mode. Valid only when DLE is enabled. 0 - The AES is idle while memory fetch/store in progress. 1- The AES is pipelined while memory fetch/store in progress."]
    #[inline(always)]
    pub fn mem_latency_hide(&self) -> MEM_LATENCY_HIDE_R {
        MEM_LATENCY_HIDE_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - LLID of the packet."]
    #[inline(always)]
    pub fn data_pdu_header(&mut self) -> DATA_PDU_HEADER_W<0> {
        DATA_PDU_HEADER_W::new(self)
    }
    #[doc = "Bits 2:6 - Length of the input data."]
    #[inline(always)]
    pub fn payload_length_lsb(&mut self) -> PAYLOAD_LENGTH_LSB_W<2> {
        PAYLOAD_LENGTH_LSB_W::new(self)
    }
    #[doc = "Bit 7 - The directionBit shall be set to '1' for Data Channel PDUs sent by the master and set to '0' for Data Channel PDUs sent by the slave."]
    #[inline(always)]
    pub fn direction(&mut self) -> DIRECTION_W<7> {
        DIRECTION_W::new(self)
    }
    #[doc = "Bits 8:10 - 3 Most significant bits of the LS byte of the length of the input data. Valid only when DLE is enabled. When DLE is enabled total ENC payload length = {PAYLOAD_LENGTH_LSB_EXT, PAYLOAD_LENGTH_LSB}"]
    #[inline(always)]
    pub fn payload_length_lsb_ext(&mut self) -> PAYLOAD_LENGTH_LSB_EXT_W<8> {
        PAYLOAD_LENGTH_LSB_EXT_W::new(self)
    }
    #[doc = "Bit 11 - Controls the encryption memory access mode. Valid only when DLE is enabled. 0 - The AES is idle while memory fetch/store in progress. 1- The AES is pipelined while memory fetch/store in progress."]
    #[inline(always)]
    pub fn mem_latency_hide(&mut self) -> MEM_LATENCY_HIDE_W<11> {
        MEM_LATENCY_HIDE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Encryption Parameter register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [enc_params](index.html) module"]
pub struct ENC_PARAMS_SPEC;
impl crate::RegisterSpec for ENC_PARAMS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [enc_params::R](R) reader structure"]
impl crate::Readable for ENC_PARAMS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [enc_params::W](W) writer structure"]
impl crate::Writable for ENC_PARAMS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ENC_PARAMS to value 0"]
impl crate::Resettable for ENC_PARAMS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
