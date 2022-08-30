#[doc = "Register `CONN_3_DATA_LIST_SENT` reader"]
pub struct R(crate::R<CONN_3_DATA_LIST_SENT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONN_3_DATA_LIST_SENT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONN_3_DATA_LIST_SENT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONN_3_DATA_LIST_SENT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONN_3_DATA_LIST_SENT` writer"]
pub struct W(crate::W<CONN_3_DATA_LIST_SENT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONN_3_DATA_LIST_SENT_SPEC>;
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
impl From<crate::W<CONN_3_DATA_LIST_SENT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONN_3_DATA_LIST_SENT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LIST_INDEX__TX_SENT_3_0_C1` reader - Write:Indicates the buffer index for which the SENT bit is being updated by firmware. The default number of buffers in the IP is 5. The index range is 0-3. Read: Reads TX_SENT\\[3:0\\]. The bits in this field indicate the status of the SENT bit in the hard-ware for each packet buffer. The bit values are 1 - queued 0 - no packet / packet ack received by hardware Example1: If the read value is : 0x03, then packets in buffer 0 and buffer 1 are in the queue to be transmitted. All the other FIFOs are empty or hardware has cleared them after receiving acknowledgement."]
pub type LIST_INDEX__TX_SENT_3_0_C1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LIST_INDEX__TX_SENT_3_0_C1` writer - Write:Indicates the buffer index for which the SENT bit is being updated by firmware. The default number of buffers in the IP is 5. The index range is 0-3. Read: Reads TX_SENT\\[3:0\\]. The bits in this field indicate the status of the SENT bit in the hard-ware for each packet buffer. The bit values are 1 - queued 0 - no packet / packet ack received by hardware Example1: If the read value is : 0x03, then packets in buffer 0 and buffer 1 are in the queue to be transmitted. All the other FIFOs are empty or hardware has cleared them after receiving acknowledgement."]
pub type LIST_INDEX__TX_SENT_3_0_C1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONN_3_DATA_LIST_SENT_SPEC, u8, u8, 4, O>;
#[doc = "Field `SET_CLEAR_C1` writer - Write: Used to set the SENT bit in hardware for the selected packet buffer. 1 - packet queued When firmware has a packet to send, firmware first loads the next available packet buffer. Then the hardware SENT bit is set by writing 1 to this bit field along with the list_index field that identified the buffer index. This indicates that a packet has been queued in the data buffer for sending. This packet is now ready to be transmitted. The SENT bit in hardware is cleared by hardware only when it has received an acknowledgement from the remote device. Firmware typically does not clear the bit. However, It only clears the bit on its own if it needs to 'flush' a packet from the buffer, without waiting to receive acknowledgement from the remote device, firmware clears BIT7 along with the list_index specified."]
pub type SET_CLEAR_C1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CONN_3_DATA_LIST_SENT_SPEC, bool, O>;
#[doc = "Field `BUFFER_NUM_TX_SENT_3_0_C1` reader - Write: Indicates the buffer number for which SENT bit is updated by firmware. This is the mapping of the list index to the physical transmit buffer. The total number of transmit buffers is 16, can be shared with up to 8 connections"]
pub type BUFFER_NUM_TX_SENT_3_0_C1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BUFFER_NUM_TX_SENT_3_0_C1` writer - Write: Indicates the buffer number for which SENT bit is updated by firmware. This is the mapping of the list index to the physical transmit buffer. The total number of transmit buffers is 16, can be shared with up to 8 connections"]
pub type BUFFER_NUM_TX_SENT_3_0_C1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONN_3_DATA_LIST_SENT_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Write:Indicates the buffer index for which the SENT bit is being updated by firmware. The default number of buffers in the IP is 5. The index range is 0-3. Read: Reads TX_SENT\\[3:0\\]. The bits in this field indicate the status of the SENT bit in the hard-ware for each packet buffer. The bit values are 1 - queued 0 - no packet / packet ack received by hardware Example1: If the read value is : 0x03, then packets in buffer 0 and buffer 1 are in the queue to be transmitted. All the other FIFOs are empty or hardware has cleared them after receiving acknowledgement."]
    #[inline(always)]
    pub fn list_index__tx_sent_3_0_c1(&self) -> LIST_INDEX__TX_SENT_3_0_C1_R {
        LIST_INDEX__TX_SENT_3_0_C1_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Write: Indicates the buffer number for which SENT bit is updated by firmware. This is the mapping of the list index to the physical transmit buffer. The total number of transmit buffers is 16, can be shared with up to 8 connections"]
    #[inline(always)]
    pub fn buffer_num_tx_sent_3_0_c1(&self) -> BUFFER_NUM_TX_SENT_3_0_C1_R {
        BUFFER_NUM_TX_SENT_3_0_C1_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Write:Indicates the buffer index for which the SENT bit is being updated by firmware. The default number of buffers in the IP is 5. The index range is 0-3. Read: Reads TX_SENT\\[3:0\\]. The bits in this field indicate the status of the SENT bit in the hard-ware for each packet buffer. The bit values are 1 - queued 0 - no packet / packet ack received by hardware Example1: If the read value is : 0x03, then packets in buffer 0 and buffer 1 are in the queue to be transmitted. All the other FIFOs are empty or hardware has cleared them after receiving acknowledgement."]
    #[inline(always)]
    pub fn list_index__tx_sent_3_0_c1(&mut self) -> LIST_INDEX__TX_SENT_3_0_C1_W<0> {
        LIST_INDEX__TX_SENT_3_0_C1_W::new(self)
    }
    #[doc = "Bit 7 - Write: Used to set the SENT bit in hardware for the selected packet buffer. 1 - packet queued When firmware has a packet to send, firmware first loads the next available packet buffer. Then the hardware SENT bit is set by writing 1 to this bit field along with the list_index field that identified the buffer index. This indicates that a packet has been queued in the data buffer for sending. This packet is now ready to be transmitted. The SENT bit in hardware is cleared by hardware only when it has received an acknowledgement from the remote device. Firmware typically does not clear the bit. However, It only clears the bit on its own if it needs to 'flush' a packet from the buffer, without waiting to receive acknowledgement from the remote device, firmware clears BIT7 along with the list_index specified."]
    #[inline(always)]
    pub fn set_clear_c1(&mut self) -> SET_CLEAR_C1_W<7> {
        SET_CLEAR_C1_W::new(self)
    }
    #[doc = "Bits 8:11 - Write: Indicates the buffer number for which SENT bit is updated by firmware. This is the mapping of the list index to the physical transmit buffer. The total number of transmit buffers is 16, can be shared with up to 8 connections"]
    #[inline(always)]
    pub fn buffer_num_tx_sent_3_0_c1(&mut self) -> BUFFER_NUM_TX_SENT_3_0_C1_W<8> {
        BUFFER_NUM_TX_SENT_3_0_C1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "data list sent update and status for connection 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conn_3_data_list_sent](index.html) module"]
pub struct CONN_3_DATA_LIST_SENT_SPEC;
impl crate::RegisterSpec for CONN_3_DATA_LIST_SENT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [conn_3_data_list_sent::R](R) reader structure"]
impl crate::Readable for CONN_3_DATA_LIST_SENT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [conn_3_data_list_sent::W](W) writer structure"]
impl crate::Writable for CONN_3_DATA_LIST_SENT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONN_3_DATA_LIST_SENT to value 0"]
impl crate::Resettable for CONN_3_DATA_LIST_SENT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
