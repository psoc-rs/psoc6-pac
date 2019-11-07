#[doc = "Reader of register CONN_NI_STATUS"]
pub type R = crate::R<u32, super::CONN_NI_STATUS>;
#[doc = "Reader of field `CONN_NI`"]
pub type CONN_NI_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - HW updates this register with the next Connection Instant for current serviced connection, granularity is 625us. The reset value is 0x0000. After reset deassertion, then the very next clock, the value assigned to the registers is 0xFFFF."]
    #[inline(always)]
    pub fn conn_ni(&self) -> CONN_NI_R {
        CONN_NI_R::new((self.bits & 0xffff) as u16)
    }
}
