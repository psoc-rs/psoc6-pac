#[doc = "Reader of register INTR_CAUSE"]
pub type R = crate::R<u32, super::INTR_CAUSE>;
#[doc = "Reader of field `CTB0_INT`"]
pub type CTB0_INT_R = crate::R<bool, bool>;
#[doc = "Reader of field `CTB1_INT`"]
pub type CTB1_INT_R = crate::R<bool, bool>;
#[doc = "Reader of field `CTB2_INT`"]
pub type CTB2_INT_R = crate::R<bool, bool>;
#[doc = "Reader of field `CTB3_INT`"]
pub type CTB3_INT_R = crate::R<bool, bool>;
#[doc = "Reader of field `CTDAC0_INT`"]
pub type CTDAC0_INT_R = crate::R<bool, bool>;
#[doc = "Reader of field `CTDAC1_INT`"]
pub type CTDAC1_INT_R = crate::R<bool, bool>;
#[doc = "Reader of field `CTDAC2_INT`"]
pub type CTDAC2_INT_R = crate::R<bool, bool>;
#[doc = "Reader of field `CTDAC3_INT`"]
pub type CTDAC3_INT_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - CTB0 interrupt pending"]
    #[inline(always)]
    pub fn ctb0_int(&self) -> CTB0_INT_R {
        CTB0_INT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CTB1 interrupt pending"]
    #[inline(always)]
    pub fn ctb1_int(&self) -> CTB1_INT_R {
        CTB1_INT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - CTB2 interrupt pending"]
    #[inline(always)]
    pub fn ctb2_int(&self) -> CTB2_INT_R {
        CTB2_INT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - CTB3 interrupt pending"]
    #[inline(always)]
    pub fn ctb3_int(&self) -> CTB3_INT_R {
        CTB3_INT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - CTDAC0 interrupt pending"]
    #[inline(always)]
    pub fn ctdac0_int(&self) -> CTDAC0_INT_R {
        CTDAC0_INT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - CTDAC1 interrupt pending"]
    #[inline(always)]
    pub fn ctdac1_int(&self) -> CTDAC1_INT_R {
        CTDAC1_INT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - CTDAC2 interrupt pending"]
    #[inline(always)]
    pub fn ctdac2_int(&self) -> CTDAC2_INT_R {
        CTDAC2_INT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - CTDAC3 interrupt pending"]
    #[inline(always)]
    pub fn ctdac3_int(&self) -> CTDAC3_INT_R {
        CTDAC3_INT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
