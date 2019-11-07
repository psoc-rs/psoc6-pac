#[doc = "Reader of register INTR_MASKED"]
pub type R = crate::R<u32, super::INTR_MASKED>;
#[doc = "Reader of field `EDGE0`"]
pub type EDGE0_R = crate::R<bool, bool>;
#[doc = "Reader of field `EDGE1`"]
pub type EDGE1_R = crate::R<bool, bool>;
#[doc = "Reader of field `EDGE2`"]
pub type EDGE2_R = crate::R<bool, bool>;
#[doc = "Reader of field `EDGE3`"]
pub type EDGE3_R = crate::R<bool, bool>;
#[doc = "Reader of field `EDGE4`"]
pub type EDGE4_R = crate::R<bool, bool>;
#[doc = "Reader of field `EDGE5`"]
pub type EDGE5_R = crate::R<bool, bool>;
#[doc = "Reader of field `EDGE6`"]
pub type EDGE6_R = crate::R<bool, bool>;
#[doc = "Reader of field `EDGE7`"]
pub type EDGE7_R = crate::R<bool, bool>;
#[doc = "Reader of field `FLT_EDGE`"]
pub type FLT_EDGE_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Edge detected AND masked on IO pin 0 '0': Interrupt was not forwarded to CPU '1': Interrupt occurred and was forwarded to CPU"]
    #[inline(always)]
    pub fn edge0(&self) -> EDGE0_R {
        EDGE0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Edge detected and masked on IO pin 1"]
    #[inline(always)]
    pub fn edge1(&self) -> EDGE1_R {
        EDGE1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Edge detected and masked on IO pin 2"]
    #[inline(always)]
    pub fn edge2(&self) -> EDGE2_R {
        EDGE2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Edge detected and masked on IO pin 3"]
    #[inline(always)]
    pub fn edge3(&self) -> EDGE3_R {
        EDGE3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Edge detected and masked on IO pin 4"]
    #[inline(always)]
    pub fn edge4(&self) -> EDGE4_R {
        EDGE4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Edge detected and masked on IO pin 5"]
    #[inline(always)]
    pub fn edge5(&self) -> EDGE5_R {
        EDGE5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Edge detected and masked on IO pin 6"]
    #[inline(always)]
    pub fn edge6(&self) -> EDGE6_R {
        EDGE6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Edge detected and masked on IO pin 7"]
    #[inline(always)]
    pub fn edge7(&self) -> EDGE7_R {
        EDGE7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Edge detected and masked on filtered pin selected by INTR_CFG.FLT_SEL"]
    #[inline(always)]
    pub fn flt_edge(&self) -> FLT_EDGE_R {
        FLT_EDGE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
