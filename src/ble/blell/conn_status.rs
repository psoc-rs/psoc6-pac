#[doc = "Register `CONN_STATUS` reader"]
pub struct R(crate::R<CONN_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONN_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONN_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONN_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RECEIVE_PACKET_COUNT` reader - This field stores the count for the number of receive packets in the receive FIFO that are still not ready by firmware. The counter value is incremented by hardware for every good packet it stores in the FIFO. After firmware reads a packet, it decrements the counter by issuing the PACKET_RECEIVED command from the commander."]
pub type RECEIVE_PACKET_COUNT_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 12:15 - This field stores the count for the number of receive packets in the receive FIFO that are still not ready by firmware. The counter value is incremented by hardware for every good packet it stores in the FIFO. After firmware reads a packet, it decrements the counter by issuing the PACKET_RECEIVED command from the commander."]
    #[inline(always)]
    pub fn receive_packet_count(&self) -> RECEIVE_PACKET_COUNT_R {
        RECEIVE_PACKET_COUNT_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
#[doc = "Connection channel status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conn_status](index.html) module"]
pub struct CONN_STATUS_SPEC;
impl crate::RegisterSpec for CONN_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [conn_status::R](R) reader structure"]
impl crate::Readable for CONN_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CONN_STATUS to value 0"]
impl crate::Resettable for CONN_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
