#[doc = "Reader of register HOST_ERR"]
pub type R = crate::R<u32, super::HOST_ERR>;
#[doc = "Writer for register HOST_ERR"]
pub type W = crate::W<u32, super::HOST_ERR>;
#[doc = "Register HOST_ERR `reset()`'s with value 0x03"]
impl crate::ResetValue for super::HOST_ERR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x03
    }
}
#[doc = "These flags indicate the status of a handshake packet to be sent or received. These flags are set to 'NULL' when no handshake occurs due to an error or when a SOF token has been ended with the TKNEN bit of the Host Token Endpoint Register (HOST_TOKEN). These bits are updated when sending or receiving has been ended. HS bits change values '11' under the following condition. However, if HS bits are written except the following conditions, the values are ignored. - HS bits indicate values except '11' and write the value '11' to HS bits. Note: This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HS_A {
    #[doc = "0: Acknowledge Packet"]
    ACK,
    #[doc = "1: Non-Acknowledge Packet"]
    NAK,
    #[doc = "2: Stall Packet"]
    STALL,
    #[doc = "3: Null Packet"]
    NULL,
}
impl From<HS_A> for u8 {
    #[inline(always)]
    fn from(variant: HS_A) -> Self {
        match variant {
            HS_A::ACK => 0,
            HS_A::NAK => 1,
            HS_A::STALL => 2,
            HS_A::NULL => 3,
        }
    }
}
#[doc = "Reader of field `HS`"]
pub type HS_R = crate::R<u8, HS_A>;
impl HS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HS_A {
        match self.bits {
            0 => HS_A::ACK,
            1 => HS_A::NAK,
            2 => HS_A::STALL,
            3 => HS_A::NULL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ACK`"]
    #[inline(always)]
    pub fn is_ack(&self) -> bool {
        *self == HS_A::ACK
    }
    #[doc = "Checks if the value of the field is `NAK`"]
    #[inline(always)]
    pub fn is_nak(&self) -> bool {
        *self == HS_A::NAK
    }
    #[doc = "Checks if the value of the field is `STALL`"]
    #[inline(always)]
    pub fn is_stall(&self) -> bool {
        *self == HS_A::STALL
    }
    #[doc = "Checks if the value of the field is `NULL`"]
    #[inline(always)]
    pub fn is_null(&self) -> bool {
        *self == HS_A::NULL
    }
}
#[doc = "Write proxy for field `HS`"]
pub struct HS_W<'a> {
    w: &'a mut W,
}
impl<'a> HS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Acknowledge Packet"]
    #[inline(always)]
    pub fn ack(self) -> &'a mut W {
        self.variant(HS_A::ACK)
    }
    #[doc = "Non-Acknowledge Packet"]
    #[inline(always)]
    pub fn nak(self) -> &'a mut W {
        self.variant(HS_A::NAK)
    }
    #[doc = "Stall Packet"]
    #[inline(always)]
    pub fn stall(self) -> &'a mut W {
        self.variant(HS_A::STALL)
    }
    #[doc = "Null Packet"]
    #[inline(always)]
    pub fn null(self) -> &'a mut W {
        self.variant(HS_A::NULL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `STUFF`"]
pub type STUFF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STUFF`"]
pub struct STUFF_W<'a> {
    w: &'a mut W,
}
impl<'a> STUFF_W<'a> {
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
#[doc = "Reader of field `TGERR`"]
pub type TGERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TGERR`"]
pub struct TGERR_W<'a> {
    w: &'a mut W,
}
impl<'a> TGERR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `CRC`"]
pub type CRC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CRC`"]
pub struct CRC_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `TOUT`"]
pub type TOUT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TOUT`"]
pub struct TOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `RERR`"]
pub type RERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RERR`"]
pub struct RERR_W<'a> {
    w: &'a mut W,
}
impl<'a> RERR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `LSTSOF`"]
pub type LSTSOF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LSTSOF`"]
pub struct LSTSOF_W<'a> {
    w: &'a mut W,
}
impl<'a> LSTSOF_W<'a> {
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
impl R {
    #[doc = "Bits 0:1 - These flags indicate the status of a handshake packet to be sent or received. These flags are set to 'NULL' when no handshake occurs due to an error or when a SOF token has been ended with the TKNEN bit of the Host Token Endpoint Register (HOST_TOKEN). These bits are updated when sending or receiving has been ended. HS bits change values '11' under the following condition. However, if HS bits are written except the following conditions, the values are ignored. - HS bits indicate values except '11' and write the value '11' to HS bits. Note: This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn hs(&self) -> HS_R {
        HS_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - If this bit is set to '1', it means that a bit stuffing error is detected. When this bit is '0', it means that no stuffing error is detected. If a stuffing error is detected, bit5 (Timeout) of this register is also set to '1'. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : No stuffing error. '1' : Stuffing error occurs. Note: - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn stuff(&self) -> STUFF_R {
        STUFF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - If this bit is set to '1', it means that the data of this bit does not match the value of the received toggle data. When this bit is '0', it means that no toggle error is detected. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : No toggle error. '1' : Toggle error occurs. Note: - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn tgerr(&self) -> TGERR_R {
        TGERR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - If this bit is set to '1', it means that a CRC error is detected in the USB Host. When this bit is '0', it means that no CRC error is detected. If a CRC error is detected, bit5 (Timeout) of this register is also set to '1'. When this bit is '0', it means that no CRC error is detected. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : No CRC error. '1' : CRC error occurs. Note: - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn crc(&self) -> CRC_R {
        CRC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - If this bit is set to '1', it means that no response is returned from the device within the specified time after a token has been sent in the USB Host. When this bit is '0', it means that no timeout is detected. When this bit is '0', it means that no error occurs. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : No timeout. '1' : Timeout occurs. Note: - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn tout(&self) -> TOUT_R {
        TOUT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - When this bit is set to '1', it means that the received data exceeds the specified maximum number of packets in the USB Host. If a receive error is detected, bit5 (Timeout) of this register is also set to '1'. When this bit is '0', it means that no error occurs. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : No receive error. '1' : Maximum packet receive error. - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn rerr(&self) -> RERR_R {
        RERR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - If this bit is set to '1', it means that the SOF token can't be sent in the USB Host because other token is in process. When this bit is '0', it means that no lost SOF error is detected. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : Sends SOF. '1' : SOF sending error. Note: - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn lstsof(&self) -> LSTSOF_R {
        LSTSOF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - These flags indicate the status of a handshake packet to be sent or received. These flags are set to 'NULL' when no handshake occurs due to an error or when a SOF token has been ended with the TKNEN bit of the Host Token Endpoint Register (HOST_TOKEN). These bits are updated when sending or receiving has been ended. HS bits change values '11' under the following condition. However, if HS bits are written except the following conditions, the values are ignored. - HS bits indicate values except '11' and write the value '11' to HS bits. Note: This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn hs(&mut self) -> HS_W {
        HS_W { w: self }
    }
    #[doc = "Bit 2 - If this bit is set to '1', it means that a bit stuffing error is detected. When this bit is '0', it means that no stuffing error is detected. If a stuffing error is detected, bit5 (Timeout) of this register is also set to '1'. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : No stuffing error. '1' : Stuffing error occurs. Note: - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn stuff(&mut self) -> STUFF_W {
        STUFF_W { w: self }
    }
    #[doc = "Bit 3 - If this bit is set to '1', it means that the data of this bit does not match the value of the received toggle data. When this bit is '0', it means that no toggle error is detected. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : No toggle error. '1' : Toggle error occurs. Note: - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn tgerr(&mut self) -> TGERR_W {
        TGERR_W { w: self }
    }
    #[doc = "Bit 4 - If this bit is set to '1', it means that a CRC error is detected in the USB Host. When this bit is '0', it means that no CRC error is detected. If a CRC error is detected, bit5 (Timeout) of this register is also set to '1'. When this bit is '0', it means that no CRC error is detected. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : No CRC error. '1' : CRC error occurs. Note: - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn crc(&mut self) -> CRC_W {
        CRC_W { w: self }
    }
    #[doc = "Bit 5 - If this bit is set to '1', it means that no response is returned from the device within the specified time after a token has been sent in the USB Host. When this bit is '0', it means that no timeout is detected. When this bit is '0', it means that no error occurs. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : No timeout. '1' : Timeout occurs. Note: - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn tout(&mut self) -> TOUT_W {
        TOUT_W { w: self }
    }
    #[doc = "Bit 6 - When this bit is set to '1', it means that the received data exceeds the specified maximum number of packets in the USB Host. If a receive error is detected, bit5 (Timeout) of this register is also set to '1'. When this bit is '0', it means that no error occurs. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : No receive error. '1' : Maximum packet receive error. - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn rerr(&mut self) -> RERR_W {
        RERR_W { w: self }
    }
    #[doc = "Bit 7 - If this bit is set to '1', it means that the SOF token can't be sent in the USB Host because other token is in process. When this bit is '0', it means that no lost SOF error is detected. If this bit is written with '1', it is set to '0'. However, if this bit is written with '0', its value is ignored. '0' : Sends SOF. '1' : SOF sending error. Note: - This bit is set to the initial value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn lstsof(&mut self) -> LSTSOF_W {
        LSTSOF_W { w: self }
    }
}
