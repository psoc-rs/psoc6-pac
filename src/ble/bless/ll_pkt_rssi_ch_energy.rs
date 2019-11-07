#[doc = "Reader of register LL_PKT_RSSI_CH_ENERGY"]
pub type R = crate::R<u32, super::LL_PKT_RSSI_CH_ENERGY>;
#[doc = "Reader of field `RSSI`"]
pub type RSSI_R = crate::R<u16, u16>;
#[doc = "Reader of field `RX_CHANNEL`"]
pub type RX_CHANNEL_R = crate::R<u8, u8>;
#[doc = "Reader of field `PKT_RSSI_OR_CH_ENERGY`"]
pub type PKT_RSSI_OR_CH_ENERGY_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:15 - This field captures the RSSI of the packet when a packet reception is complete or gives the Channel energy when a Receive cycle is over without packet reception."]
    #[inline(always)]
    pub fn rssi(&self) -> RSSI_R {
        RSSI_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:21 - This field indicates the last channel for which the RSSI is captured"]
    #[inline(always)]
    pub fn rx_channel(&self) -> RX_CHANNEL_R {
        RX_CHANNEL_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 22 - This field indicates if the captured RSSI is for a received packet or is the channel energy"]
    #[inline(always)]
    pub fn pkt_rssi_or_ch_energy(&self) -> PKT_RSSI_OR_CH_ENERGY_R {
        PKT_RSSI_OR_CH_ENERGY_R::new(((self.bits >> 22) & 0x01) != 0)
    }
}
