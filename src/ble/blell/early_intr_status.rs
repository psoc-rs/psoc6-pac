#[doc = "Reader of register EARLY_INTR_STATUS"]
pub type R = crate::R<u32, super::EARLY_INTR_STATUS>;
#[doc = "Reader of field `CONN_INDEX_FOR_EARLY_INTR`"]
pub type CONN_INDEX_FOR_EARLY_INTR_R = crate::R<u8, u8>;
#[doc = "Reader of field `CONN_TYPE_FOR_EARLY_INTR`"]
pub type CONN_TYPE_FOR_EARLY_INTR_R = crate::R<bool, bool>;
#[doc = "Reader of field `US_FOR_EARLY_INTR`"]
pub type US_FOR_EARLY_INTR_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:4 - Connection Index for which early interrupt is raised"]
    #[inline(always)]
    pub fn conn_index_for_early_intr(&self) -> CONN_INDEX_FOR_EARLY_INTR_R {
        CONN_INDEX_FOR_EARLY_INTR_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - Connection type for which early interrupt is raised."]
    #[inline(always)]
    pub fn conn_type_for_early_intr(&self) -> CONN_TYPE_FOR_EARLY_INTR_R {
        CONN_TYPE_FOR_EARLY_INTR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:15 - US offset when early interrupt is raised"]
    #[inline(always)]
    pub fn us_for_early_intr(&self) -> US_FOR_EARLY_INTR_R {
        US_FOR_EARLY_INTR_R::new(((self.bits >> 6) & 0x03ff) as u16)
    }
}
