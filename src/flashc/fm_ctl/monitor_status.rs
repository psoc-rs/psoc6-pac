#[doc = "Reader of register MONITOR_STATUS"]
pub type R = crate::R<u32, super::MONITOR_STATUS>;
#[doc = "Reader of field `POS_PUMP_VLO`"]
pub type POS_PUMP_VLO_R = crate::R<bool, bool>;
#[doc = "Reader of field `NEG_PUMP_VHI`"]
pub type NEG_PUMP_VHI_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 1 - POS pump VLO"]
    #[inline(always)]
    pub fn pos_pump_vlo(&self) -> POS_PUMP_VLO_R {
        POS_PUMP_VLO_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - NEG pump VHI"]
    #[inline(always)]
    pub fn neg_pump_vhi(&self) -> NEG_PUMP_VHI_R {
        NEG_PUMP_VHI_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
