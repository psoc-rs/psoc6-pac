#[doc = "Reader of register INIT_SCN_ADV_RX_FIFO"]
pub type R = crate::R<u32, super::INIT_SCN_ADV_RX_FIFO>;
#[doc = "Reader of field `ADV_SCAN_RSP_RX_DATA`"]
pub type ADV_SCAN_RSP_RX_DATA_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - IO mapped FIFO of depth 64, to store ADV and SCAN_RSP header and payload received by the scanner. The RSSI value at the time of reception of this packet is also stored. Firmware reads from the same address to read out consecutive words of data. Note: The 16 bit header is first loaded to the advertise channel data receive FIFO followed by the payload data and then 16 bit RSSI."]
    #[inline(always)]
    pub fn adv_scan_rsp_rx_data(&self) -> ADV_SCAN_RSP_RX_DATA_R {
        ADV_SCAN_RSP_RX_DATA_R::new((self.bits & 0xffff) as u16)
    }
}
