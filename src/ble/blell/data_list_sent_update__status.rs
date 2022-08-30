#[doc = "Register `DATA_LIST_SENT_UPDATE__STATUS` reader"]
pub struct R(crate::R<DATA_LIST_SENT_UPDATE__STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATA_LIST_SENT_UPDATE__STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATA_LIST_SENT_UPDATE__STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATA_LIST_SENT_UPDATE__STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DATA_LIST_SENT_UPDATE__STATUS` writer"]
pub struct W(crate::W<DATA_LIST_SENT_UPDATE__STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATA_LIST_SENT_UPDATE__STATUS_SPEC>;
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
impl From<crate::W<DATA_LIST_SENT_UPDATE__STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATA_LIST_SENT_UPDATE__STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LIST_INDEX__TX_SENT_3_0` reader - Write:Indicates the buffer index for which the SENT bit is being updated by firmware. The default number of buffers in the IP is 4. The index range is 0-3. Read: Reads TX_SENT\\[3:0\\]. The bits in this field indicate the status of the SENT bit in the hard-ware for each packet buffer. The bit values are 1 - queued 0 - no packet / packet ack received by hardware Example1: If the read value is : 0x03, then packets in buffer 0 and buffer 1 are in the queue to be transmitted. All the other FIFOs are empty or hardware has cleared them after receiving acknowledgement. NOTE: The SENT status bit and ACK status bit have to be taken together to understand the meaning of packet status. The table below describes how the two bits are sequentially updated by either hardware/firmware to complete one data transmission. SENT ACK Description 0 0 Buffer is empty. No packet is queued in the buffer 1 0 Packet is queued by firmware. 1 1 Packet is transmitted by hardware. Hardware is waiting for acknowledgement. 0 1 Hardware has received ACK. Firmware has not yet processed the ACK. 0 0 Firmware has processed the ack. The buffer is again empty."]
pub type LIST_INDEX__TX_SENT_3_0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LIST_INDEX__TX_SENT_3_0` writer - Write:Indicates the buffer index for which the SENT bit is being updated by firmware. The default number of buffers in the IP is 4. The index range is 0-3. Read: Reads TX_SENT\\[3:0\\]. The bits in this field indicate the status of the SENT bit in the hard-ware for each packet buffer. The bit values are 1 - queued 0 - no packet / packet ack received by hardware Example1: If the read value is : 0x03, then packets in buffer 0 and buffer 1 are in the queue to be transmitted. All the other FIFOs are empty or hardware has cleared them after receiving acknowledgement. NOTE: The SENT status bit and ACK status bit have to be taken together to understand the meaning of packet status. The table below describes how the two bits are sequentially updated by either hardware/firmware to complete one data transmission. SENT ACK Description 0 0 Buffer is empty. No packet is queued in the buffer 1 0 Packet is queued by firmware. 1 1 Packet is transmitted by hardware. Hardware is waiting for acknowledgement. 0 1 Hardware has received ACK. Firmware has not yet processed the ACK. 0 0 Firmware has processed the ack. The buffer is again empty."]
pub type LIST_INDEX__TX_SENT_3_0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DATA_LIST_SENT_UPDATE__STATUS_SPEC, u8, u8, 4, O>;
#[doc = "Field `SET_CLEAR` writer - Write: Used to set the SENT bit in hardware for the selected packet buffer. 1 - packet queued When firmware has a packet to send, firmware first loads the next available packet buffer. Then the hardware SENT bit is set by writing 1 to this bit field along with the list_index field that identified the buffer index. This indicates that a packet has been queued in the data buffer for sending. This packet is now ready to be transmitted. The SENT bit in hardware is cleared by hardware only when it has received an acknowledgement from the remote device. Firmware typically does not clear the bit. However, It only clears the bit on its own if it needs to 'flush' a packet from the buffer, without waiting to receive acknowledgement from the remote device, firmware clears BIT7 along with the list_index specified."]
pub type SET_CLEAR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DATA_LIST_SENT_UPDATE__STATUS_SPEC, bool, O>;
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
    pub fn list_index__tx_sent_3_0(&mut self) -> LIST_INDEX__TX_SENT_3_0_W<0> {
        LIST_INDEX__TX_SENT_3_0_W::new(self)
    }
    #[doc = "Bit 7 - Write: Used to set the SENT bit in hardware for the selected packet buffer. 1 - packet queued When firmware has a packet to send, firmware first loads the next available packet buffer. Then the hardware SENT bit is set by writing 1 to this bit field along with the list_index field that identified the buffer index. This indicates that a packet has been queued in the data buffer for sending. This packet is now ready to be transmitted. The SENT bit in hardware is cleared by hardware only when it has received an acknowledgement from the remote device. Firmware typically does not clear the bit. However, It only clears the bit on its own if it needs to 'flush' a packet from the buffer, without waiting to receive acknowledgement from the remote device, firmware clears BIT7 along with the list_index specified."]
    #[inline(always)]
    pub fn set_clear(&mut self) -> SET_CLEAR_W<7> {
        SET_CLEAR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "data list sent update and status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data_list_sent_update__status](index.html) module"]
pub struct DATA_LIST_SENT_UPDATE__STATUS_SPEC;
impl crate::RegisterSpec for DATA_LIST_SENT_UPDATE__STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [data_list_sent_update__status::R](R) reader structure"]
impl crate::Readable for DATA_LIST_SENT_UPDATE__STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [data_list_sent_update__status::W](W) writer structure"]
impl crate::Writable for DATA_LIST_SENT_UPDATE__STATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DATA_LIST_SENT_UPDATE__STATUS to value 0"]
impl crate::Resettable for DATA_LIST_SENT_UPDATE__STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
