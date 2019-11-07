#[doc = "Reader of register DATA_LIST_SENT_UPDATE__STATUS"]
pub type R = crate::R<u32, super::DATA_LIST_SENT_UPDATE__STATUS>;
#[doc = "Writer for register DATA_LIST_SENT_UPDATE__STATUS"]
pub type W = crate::W<u32, super::DATA_LIST_SENT_UPDATE__STATUS>;
#[doc = "Register DATA_LIST_SENT_UPDATE__STATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::DATA_LIST_SENT_UPDATE__STATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LIST_INDEX__TX_SENT_3_0`"]
pub type LIST_INDEX__TX_SENT_3_0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LIST_INDEX__TX_SENT_3_0`"]
pub struct LIST_INDEX__TX_SENT_3_0_W<'a> {
    w: &'a mut W,
}
impl<'a> LIST_INDEX__TX_SENT_3_0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Write proxy for field `SET_CLEAR`"]
pub struct SET_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> SET_CLEAR_W<'a> {
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
    #[doc = "Bits 0:3 - Write:Indicates the buffer index for which the SENT bit is being updated by firmware. The default number of buffers in the IP is 4. The index range is 0-3. Read: Reads TX_SENT\\[3:0\\]. The bits in this field indicate the status of the SENT bit in the hard-ware for each packet buffer. The bit values are 1 - queued 0 - no packet / packet ack received by hardware Example1: If the read value is : 0x03, then packets in buffer 0 and buffer 1 are in the queue to be transmitted. All the other FIFOs are empty or hardware has cleared them after receiving acknowledgement. NOTE: The SENT status bit and ACK status bit have to be taken together to understand the meaning of packet status. The table below describes how the two bits are sequentially updated by either hardware/firmware to complete one data transmission. SENT ACK Description 0 0 Buffer is empty. No packet is queued in the buffer 1 0 Packet is queued by firmware. 1 1 Packet is transmitted by hardware. Hardware is waiting for acknowledgement. 0 1 Hardware has received ACK. Firmware has not yet processed the ACK. 0 0 Firmware has processed the ack. The buffer is again empty."]
    #[inline(always)]
    pub fn list_index__tx_sent_3_0(&self) -> LIST_INDEX__TX_SENT_3_0_R {
        LIST_INDEX__TX_SENT_3_0_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Write:Indicates the buffer index for which the SENT bit is being updated by firmware. The default number of buffers in the IP is 4. The index range is 0-3. Read: Reads TX_SENT\\[3:0\\]. The bits in this field indicate the status of the SENT bit in the hard-ware for each packet buffer. The bit values are 1 - queued 0 - no packet / packet ack received by hardware Example1: If the read value is : 0x03, then packets in buffer 0 and buffer 1 are in the queue to be transmitted. All the other FIFOs are empty or hardware has cleared them after receiving acknowledgement. NOTE: The SENT status bit and ACK status bit have to be taken together to understand the meaning of packet status. The table below describes how the two bits are sequentially updated by either hardware/firmware to complete one data transmission. SENT ACK Description 0 0 Buffer is empty. No packet is queued in the buffer 1 0 Packet is queued by firmware. 1 1 Packet is transmitted by hardware. Hardware is waiting for acknowledgement. 0 1 Hardware has received ACK. Firmware has not yet processed the ACK. 0 0 Firmware has processed the ack. The buffer is again empty."]
    #[inline(always)]
    pub fn list_index__tx_sent_3_0(&mut self) -> LIST_INDEX__TX_SENT_3_0_W {
        LIST_INDEX__TX_SENT_3_0_W { w: self }
    }
    #[doc = "Bit 7 - Write: Used to set the SENT bit in hardware for the selected packet buffer. 1 - packet queued When firmware has a packet to send, firmware first loads the next available packet buffer. Then the hardware SENT bit is set by writing 1 to this bit field along with the list_index field that identified the buffer index. This indicates that a packet has been queued in the data buffer for sending. This packet is now ready to be transmitted. The SENT bit in hardware is cleared by hardware only when it has received an acknowledgement from the remote device. Firmware typically does not clear the bit. However, It only clears the bit on its own if it needs to 'flush' a packet from the buffer, without waiting to receive acknowledgement from the remote device, firmware clears BIT7 along with the list_index specified."]
    #[inline(always)]
    pub fn set_clear(&mut self) -> SET_CLEAR_W {
        SET_CLEAR_W { w: self }
    }
}
