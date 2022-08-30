#[doc = "Register `LL_PKT_RSSI_CH_ENERGY` reader"]
pub struct R(crate::R<LL_PKT_RSSI_CH_ENERGY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LL_PKT_RSSI_CH_ENERGY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LL_PKT_RSSI_CH_ENERGY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LL_PKT_RSSI_CH_ENERGY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RSSI` reader - This field captures the RSSI of the packet when a packet reception is complete or gives the Channel energy when a Receive cycle is over without packet reception."]
pub type RSSI_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RX_CHANNEL` reader - This field indicates the last channel for which the RSSI is captured"]
pub type RX_CHANNEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PKT_RSSI_OR_CH_ENERGY` reader - This field indicates if the captured RSSI is for a received packet or is the channel energy"]
pub type PKT_RSSI_OR_CH_ENERGY_R = crate::BitReader<bool>;
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
        PKT_RSSI_OR_CH_ENERGY_R::new(((self.bits >> 22) & 1) != 0)
    }
}
#[doc = "Link Layer Last Received packet RSSI/Channel energy and channel number\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ll_pkt_rssi_ch_energy](index.html) module"]
pub struct LL_PKT_RSSI_CH_ENERGY_SPEC;
impl crate::RegisterSpec for LL_PKT_RSSI_CH_ENERGY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ll_pkt_rssi_ch_energy::R](R) reader structure"]
impl crate::Readable for LL_PKT_RSSI_CH_ENERGY_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LL_PKT_RSSI_CH_ENERGY to value 0"]
impl crate::Resettable for LL_PKT_RSSI_CH_ENERGY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
