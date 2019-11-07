#[doc = "Reader of register STATUS"]
pub type R = crate::R<u32, super::STATUS>;
#[doc = "Reader of field `CUR_CHAN`"]
pub type CUR_CHAN_R = crate::R<u8, u8>;
#[doc = "Reader of field `SW_VREF_NEG`"]
pub type SW_VREF_NEG_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUSY`"]
pub type BUSY_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:4 - current channel being sampled (channel 16 indicates the injection channel), only valid if BUSY."]
    #[inline(always)]
    pub fn cur_chan(&self) -> CUR_CHAN_R {
        CUR_CHAN_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 30 - the current switch status, including DSI and sequencer controls, of the switch in the SARADC that shorts NEG with VREF input (see NEG_SEL)."]
    #[inline(always)]
    pub fn sw_vref_neg(&self) -> SW_VREF_NEG_R {
        SW_VREF_NEG_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - If high then the SAR is busy with a conversion. This bit is always high when CONTINUOUS is set. Firmware should wait for this bit to be low before putting the SAR in power down."]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
