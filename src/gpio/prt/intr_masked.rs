#[doc = "Register `INTR_MASKED` reader"]
pub struct R(crate::R<INTR_MASKED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_MASKED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_MASKED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_MASKED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EDGE0` reader - Edge detected AND masked on IO pin 0 '0': Interrupt was not forwarded to CPU '1': Interrupt occurred and was forwarded to CPU"]
pub type EDGE0_R = crate::BitReader<bool>;
#[doc = "Field `EDGE1` reader - Edge detected and masked on IO pin 1"]
pub type EDGE1_R = crate::BitReader<bool>;
#[doc = "Field `EDGE2` reader - Edge detected and masked on IO pin 2"]
pub type EDGE2_R = crate::BitReader<bool>;
#[doc = "Field `EDGE3` reader - Edge detected and masked on IO pin 3"]
pub type EDGE3_R = crate::BitReader<bool>;
#[doc = "Field `EDGE4` reader - Edge detected and masked on IO pin 4"]
pub type EDGE4_R = crate::BitReader<bool>;
#[doc = "Field `EDGE5` reader - Edge detected and masked on IO pin 5"]
pub type EDGE5_R = crate::BitReader<bool>;
#[doc = "Field `EDGE6` reader - Edge detected and masked on IO pin 6"]
pub type EDGE6_R = crate::BitReader<bool>;
#[doc = "Field `EDGE7` reader - Edge detected and masked on IO pin 7"]
pub type EDGE7_R = crate::BitReader<bool>;
#[doc = "Field `FLT_EDGE` reader - Edge detected and masked on filtered pin selected by INTR_CFG.FLT_SEL"]
pub type FLT_EDGE_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Edge detected AND masked on IO pin 0 '0': Interrupt was not forwarded to CPU '1': Interrupt occurred and was forwarded to CPU"]
    #[inline(always)]
    pub fn edge0(&self) -> EDGE0_R {
        EDGE0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Edge detected and masked on IO pin 1"]
    #[inline(always)]
    pub fn edge1(&self) -> EDGE1_R {
        EDGE1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Edge detected and masked on IO pin 2"]
    #[inline(always)]
    pub fn edge2(&self) -> EDGE2_R {
        EDGE2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Edge detected and masked on IO pin 3"]
    #[inline(always)]
    pub fn edge3(&self) -> EDGE3_R {
        EDGE3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Edge detected and masked on IO pin 4"]
    #[inline(always)]
    pub fn edge4(&self) -> EDGE4_R {
        EDGE4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Edge detected and masked on IO pin 5"]
    #[inline(always)]
    pub fn edge5(&self) -> EDGE5_R {
        EDGE5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Edge detected and masked on IO pin 6"]
    #[inline(always)]
    pub fn edge6(&self) -> EDGE6_R {
        EDGE6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Edge detected and masked on IO pin 7"]
    #[inline(always)]
    pub fn edge7(&self) -> EDGE7_R {
        EDGE7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Edge detected and masked on filtered pin selected by INTR_CFG.FLT_SEL"]
    #[inline(always)]
    pub fn flt_edge(&self) -> FLT_EDGE_R {
        FLT_EDGE_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "Port interrupt masked status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_masked](index.html) module"]
pub struct INTR_MASKED_SPEC;
impl crate::RegisterSpec for INTR_MASKED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_masked::R](R) reader structure"]
impl crate::Readable for INTR_MASKED_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INTR_MASKED to value 0"]
impl crate::Resettable for INTR_MASKED_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
