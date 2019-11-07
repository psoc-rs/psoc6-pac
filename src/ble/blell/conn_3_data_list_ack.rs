#[doc = "Reader of register CONN_3_DATA_LIST_ACK"]
pub type R = crate::R<u32, super::CONN_3_DATA_LIST_ACK>;
#[doc = "Writer for register CONN_3_DATA_LIST_ACK"]
pub type W = crate::W<u32, super::CONN_3_DATA_LIST_ACK>;
#[doc = "Register CONN_3_DATA_LIST_ACK `reset()`'s with value 0"]
impl crate::ResetValue for super::CONN_3_DATA_LIST_ACK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LIST_INDEX__TX_ACK_3_0_C1`"]
pub type LIST_INDEX__TX_ACK_3_0_C1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LIST_INDEX__TX_ACK_3_0_C1`"]
pub struct LIST_INDEX__TX_ACK_3_0_C1_W<'a> {
    w: &'a mut W,
}
impl<'a> LIST_INDEX__TX_ACK_3_0_C1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Write proxy for field `SET_CLEAR_C1`"]
pub struct SET_CLEAR_C1_W<'a> {
    w: &'a mut W,
}
impl<'a> SET_CLEAR_C1_W<'a> {
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
    #[doc = "Bits 0:3 - Write: Indicates the buffer index for which the ACK bit is being updated by firmware. The default number of buffers in the IP is 5. The index range is 0-3. Read: Reads TX_ACK\\[3:0\\] If a particular bit is set, then the packet in the selected buffer has been transmitted (at least once) by the hardware and hardware is waiting for acknowledgement. Example1 : If the read value is : 0x03, then packets in FIFO-0 and FIFO-1 are acknowledged by the remote device. These acknowledgements are pending to be processed by firmware. Example2 : If the read value is : 0x02, then packet FIFO-1 is acknowledged by the remote device. This acknowledgement is pending to be processed by firmware."]
    #[inline(always)]
    pub fn list_index__tx_ack_3_0_c1(&self) -> LIST_INDEX__TX_ACK_3_0_C1_R {
        LIST_INDEX__TX_ACK_3_0_C1_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Write: Indicates the buffer index for which the ACK bit is being updated by firmware. The default number of buffers in the IP is 5. The index range is 0-3. Read: Reads TX_ACK\\[3:0\\] If a particular bit is set, then the packet in the selected buffer has been transmitted (at least once) by the hardware and hardware is waiting for acknowledgement. Example1 : If the read value is : 0x03, then packets in FIFO-0 and FIFO-1 are acknowledged by the remote device. These acknowledgements are pending to be processed by firmware. Example2 : If the read value is : 0x02, then packet FIFO-1 is acknowledged by the remote device. This acknowledgement is pending to be processed by firmware."]
    #[inline(always)]
    pub fn list_index__tx_ack_3_0_c1(&mut self) -> LIST_INDEX__TX_ACK_3_0_C1_W {
        LIST_INDEX__TX_ACK_3_0_C1_W { w: self }
    }
    #[doc = "Bit 7 - Write: Firmware uses the field to clear and ACK bit in the hardware to indicate that the acknowledgement for the transmit packet has been received and processed by firmware. Firmware clears the ACK bit in the hardware by writing in this register only after the acknowledgement is processed successfully by firmware. For clearing ack for a packet transmitted in fifo-index : '3', firm-ware will write '3' in the 'list-index' field and set this bit (BIT7) to 0. This is the indication that the corresponding packet buffer identi-fied by List-Index is cleared of previous transmission and can be re-used for another packet from now on. The ACK bit in hardware is set by hardware when it has success-fully transmitted a packet."]
    #[inline(always)]
    pub fn set_clear_c1(&mut self) -> SET_CLEAR_C1_W {
        SET_CLEAR_C1_W { w: self }
    }
}
