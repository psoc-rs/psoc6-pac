#[doc = "Register `VDD_INTR_MASKED` reader"]
pub struct R(crate::R<VDD_INTR_MASKED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VDD_INTR_MASKED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VDD_INTR_MASKED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VDD_INTR_MASKED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `VDDIO_ACTIVE` reader - Supply transition detected AND masked '0': Interrupt was not forwarded to CPU '1': Interrupt occurred and was forwarded to CPU"]
pub type VDDIO_ACTIVE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `VDDA_ACTIVE` reader - Same as VDDIO_ACTIVE for the analog supply VDDA."]
pub type VDDA_ACTIVE_R = crate::BitReader<bool>;
#[doc = "Field `VDDD_ACTIVE` reader - Same as VDDIO_ACTIVE for the digital supply VDDD."]
pub type VDDD_ACTIVE_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:15 - Supply transition detected AND masked '0': Interrupt was not forwarded to CPU '1': Interrupt occurred and was forwarded to CPU"]
    #[inline(always)]
    pub fn vddio_active(&self) -> VDDIO_ACTIVE_R {
        VDDIO_ACTIVE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 30 - Same as VDDIO_ACTIVE for the analog supply VDDA."]
    #[inline(always)]
    pub fn vdda_active(&self) -> VDDA_ACTIVE_R {
        VDDA_ACTIVE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Same as VDDIO_ACTIVE for the digital supply VDDD."]
    #[inline(always)]
    pub fn vddd_active(&self) -> VDDD_ACTIVE_R {
        VDDD_ACTIVE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Supply detection interrupt masked register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vdd_intr_masked](index.html) module"]
pub struct VDD_INTR_MASKED_SPEC;
impl crate::RegisterSpec for VDD_INTR_MASKED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vdd_intr_masked::R](R) reader structure"]
impl crate::Readable for VDD_INTR_MASKED_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets VDD_INTR_MASKED to value 0"]
impl crate::Resettable for VDD_INTR_MASKED_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
