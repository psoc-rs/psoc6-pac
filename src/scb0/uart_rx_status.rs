#[doc = "Reader of register UART_RX_STATUS"]
pub type R = crate::R<u32, super::UART_RX_STATUS>;
#[doc = "Reader of field `BR_COUNTER`"]
pub type BR_COUNTER_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:11 - Amount of SCB clock periods that constitute the transmission of a 0x55 data frame (sent least signficant bit first) as determined by the receiver. BR_COUNTER / 8 is the amount of SCB clock periods that constitute a bit period. This field has valid data when INTR_RX.BAUD_DETECT is set to '1'."]
    #[inline(always)]
    pub fn br_counter(&self) -> BR_COUNTER_R {
        BR_COUNTER_R::new((self.bits & 0x0fff) as u16)
    }
}
