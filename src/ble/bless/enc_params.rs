#[doc = "Reader of register ENC_PARAMS"]
pub type R = crate::R<u32, super::ENC_PARAMS>;
#[doc = "Writer for register ENC_PARAMS"]
pub type W = crate::W<u32, super::ENC_PARAMS>;
#[doc = "Register ENC_PARAMS `reset()`'s with value 0"]
impl crate::ResetValue for super::ENC_PARAMS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DATA_PDU_HEADER`"]
pub type DATA_PDU_HEADER_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATA_PDU_HEADER`"]
pub struct DATA_PDU_HEADER_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_PDU_HEADER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `PAYLOAD_LENGTH_LSB`"]
pub type PAYLOAD_LENGTH_LSB_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PAYLOAD_LENGTH_LSB`"]
pub struct PAYLOAD_LENGTH_LSB_W<'a> {
    w: &'a mut W,
}
impl<'a> PAYLOAD_LENGTH_LSB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 2)) | (((value as u32) & 0x1f) << 2);
        self.w
    }
}
#[doc = "Reader of field `DIRECTION`"]
pub type DIRECTION_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIRECTION`"]
pub struct DIRECTION_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRECTION_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `PAYLOAD_LENGTH_LSB_EXT`"]
pub type PAYLOAD_LENGTH_LSB_EXT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PAYLOAD_LENGTH_LSB_EXT`"]
pub struct PAYLOAD_LENGTH_LSB_EXT_W<'a> {
    w: &'a mut W,
}
impl<'a> PAYLOAD_LENGTH_LSB_EXT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `MEM_LATENCY_HIDE`"]
pub type MEM_LATENCY_HIDE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MEM_LATENCY_HIDE`"]
pub struct MEM_LATENCY_HIDE_W<'a> {
    w: &'a mut W,
}
impl<'a> MEM_LATENCY_HIDE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - LLID of the packet."]
    #[inline(always)]
    pub fn data_pdu_header(&self) -> DATA_PDU_HEADER_R {
        DATA_PDU_HEADER_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:6 - Length of the input data."]
    #[inline(always)]
    pub fn payload_length_lsb(&self) -> PAYLOAD_LENGTH_LSB_R {
        PAYLOAD_LENGTH_LSB_R::new(((self.bits >> 2) & 0x1f) as u8)
    }
    #[doc = "Bit 7 - The directionBit shall be set to '1' for Data Channel PDUs sent by the master and set to '0' for Data Channel PDUs sent by the slave."]
    #[inline(always)]
    pub fn direction(&self) -> DIRECTION_R {
        DIRECTION_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - 3 Most significant bits of the LS byte of the length of the input data. Valid only when DLE is enabled. When DLE is enabled total ENC payload length = {PAYLOAD_LENGTH_LSB_EXT, PAYLOAD_LENGTH_LSB}"]
    #[inline(always)]
    pub fn payload_length_lsb_ext(&self) -> PAYLOAD_LENGTH_LSB_EXT_R {
        PAYLOAD_LENGTH_LSB_EXT_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 11 - Controls the encryption memory access mode. Valid only when DLE is enabled. 0 - The AES is idle while memory fetch/store in progress. 1- The AES is pipelined while memory fetch/store in progress."]
    #[inline(always)]
    pub fn mem_latency_hide(&self) -> MEM_LATENCY_HIDE_R {
        MEM_LATENCY_HIDE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - LLID of the packet."]
    #[inline(always)]
    pub fn data_pdu_header(&mut self) -> DATA_PDU_HEADER_W {
        DATA_PDU_HEADER_W { w: self }
    }
    #[doc = "Bits 2:6 - Length of the input data."]
    #[inline(always)]
    pub fn payload_length_lsb(&mut self) -> PAYLOAD_LENGTH_LSB_W {
        PAYLOAD_LENGTH_LSB_W { w: self }
    }
    #[doc = "Bit 7 - The directionBit shall be set to '1' for Data Channel PDUs sent by the master and set to '0' for Data Channel PDUs sent by the slave."]
    #[inline(always)]
    pub fn direction(&mut self) -> DIRECTION_W {
        DIRECTION_W { w: self }
    }
    #[doc = "Bits 8:10 - 3 Most significant bits of the LS byte of the length of the input data. Valid only when DLE is enabled. When DLE is enabled total ENC payload length = {PAYLOAD_LENGTH_LSB_EXT, PAYLOAD_LENGTH_LSB}"]
    #[inline(always)]
    pub fn payload_length_lsb_ext(&mut self) -> PAYLOAD_LENGTH_LSB_EXT_W {
        PAYLOAD_LENGTH_LSB_EXT_W { w: self }
    }
    #[doc = "Bit 11 - Controls the encryption memory access mode. Valid only when DLE is enabled. 0 - The AES is idle while memory fetch/store in progress. 1- The AES is pipelined while memory fetch/store in progress."]
    #[inline(always)]
    pub fn mem_latency_hide(&mut self) -> MEM_LATENCY_HIDE_W {
        MEM_LATENCY_HIDE_W { w: self }
    }
}
