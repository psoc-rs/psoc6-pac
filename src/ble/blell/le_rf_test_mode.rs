#[doc = "Reader of register LE_RF_TEST_MODE"]
pub type R = crate::R<u32, super::LE_RF_TEST_MODE>;
#[doc = "Writer for register LE_RF_TEST_MODE"]
pub type W = crate::W<u32, super::LE_RF_TEST_MODE>;
#[doc = "Register LE_RF_TEST_MODE `reset()`'s with value 0"]
impl crate::ResetValue for super::LE_RF_TEST_MODE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TEST_FREQUENCY`"]
pub type TEST_FREQUENCY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TEST_FREQUENCY`"]
pub struct TEST_FREQUENCY_W<'a> {
    w: &'a mut W,
}
impl<'a> TEST_FREQUENCY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Reader of field `DTM_STATUS__DTM_CONT_RXEN`"]
pub type DTM_STATUS__DTM_CONT_RXEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DTM_STATUS__DTM_CONT_RXEN`"]
pub struct DTM_STATUS__DTM_CONT_RXEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DTM_STATUS__DTM_CONT_RXEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `PKT_PAYLOAD`"]
pub type PKT_PAYLOAD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PKT_PAYLOAD`"]
pub struct PKT_PAYLOAD_W<'a> {
    w: &'a mut W,
}
impl<'a> PKT_PAYLOAD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 7)) | (((value as u32) & 0x07) << 7);
        self.w
    }
}
#[doc = "Reader of field `DTM_CONT_TXEN`"]
pub type DTM_CONT_TXEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DTM_CONT_TXEN`"]
pub struct DTM_CONT_TXEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DTM_CONT_TXEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `DTM_DATA_2MBPS`"]
pub type DTM_DATA_2MBPS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DTM_DATA_2MBPS`"]
pub struct DTM_DATA_2MBPS_W<'a> {
    w: &'a mut W,
}
impl<'a> DTM_DATA_2MBPS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - N = (F - 2402) / 2 Range: 0x00 - 0x27. Frequency Range : 2402 MHz to 2480 MHz"]
    #[inline(always)]
    pub fn test_frequency(&self) -> TEST_FREQUENCY_R {
        TEST_FREQUENCY_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - This bit is overloaded. The read operation returns the staus of the DTM 1 - DTM test ON 0 - DTM test OFF The write operation contrls the DTM RX mode 0: DTM run at normal DTMRX burst mode 1: DTM run at continuous RX DTM mode"]
    #[inline(always)]
    pub fn dtm_status__dtm_cont_rxen(&self) -> DTM_STATUS__DTM_CONT_RXEN_R {
        DTM_STATUS__DTM_CONT_RXEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 7:9 - N/A"]
    #[inline(always)]
    pub fn pkt_payload(&self) -> PKT_PAYLOAD_R {
        PKT_PAYLOAD_R::new(((self.bits >> 7) & 0x07) as u8)
    }
    #[doc = "Bit 13 - 0: DTM run at normal DTMTX burst mode 1: DTM run at continuous TX DTM mode"]
    #[inline(always)]
    pub fn dtm_cont_txen(&self) -> DTM_CONT_TXEN_R {
        DTM_CONT_TXEN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 15 - 0: DTM run at 1M bps data rate 1: DTM run at 2M bps data rate"]
    #[inline(always)]
    pub fn dtm_data_2mbps(&self) -> DTM_DATA_2MBPS_R {
        DTM_DATA_2MBPS_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - N = (F - 2402) / 2 Range: 0x00 - 0x27. Frequency Range : 2402 MHz to 2480 MHz"]
    #[inline(always)]
    pub fn test_frequency(&mut self) -> TEST_FREQUENCY_W {
        TEST_FREQUENCY_W { w: self }
    }
    #[doc = "Bit 6 - This bit is overloaded. The read operation returns the staus of the DTM 1 - DTM test ON 0 - DTM test OFF The write operation contrls the DTM RX mode 0: DTM run at normal DTMRX burst mode 1: DTM run at continuous RX DTM mode"]
    #[inline(always)]
    pub fn dtm_status__dtm_cont_rxen(&mut self) -> DTM_STATUS__DTM_CONT_RXEN_W {
        DTM_STATUS__DTM_CONT_RXEN_W { w: self }
    }
    #[doc = "Bits 7:9 - N/A"]
    #[inline(always)]
    pub fn pkt_payload(&mut self) -> PKT_PAYLOAD_W {
        PKT_PAYLOAD_W { w: self }
    }
    #[doc = "Bit 13 - 0: DTM run at normal DTMTX burst mode 1: DTM run at continuous TX DTM mode"]
    #[inline(always)]
    pub fn dtm_cont_txen(&mut self) -> DTM_CONT_TXEN_W {
        DTM_CONT_TXEN_W { w: self }
    }
    #[doc = "Bit 15 - 0: DTM run at 1M bps data rate 1: DTM run at 2M bps data rate"]
    #[inline(always)]
    pub fn dtm_data_2mbps(&mut self) -> DTM_DATA_2MBPS_W {
        DTM_DATA_2MBPS_W { w: self }
    }
}
