#[doc = "Register `VDD_INTR_MASK` reader"]
pub struct R(crate::R<VDD_INTR_MASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VDD_INTR_MASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VDD_INTR_MASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VDD_INTR_MASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VDD_INTR_MASK` writer"]
pub struct W(crate::W<VDD_INTR_MASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VDD_INTR_MASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<VDD_INTR_MASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VDD_INTR_MASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VDDIO_ACTIVE` reader - Masks supply interrupt on VDDIO. '0': VDDIO interrupt forwarding disabled '1': VDDIO interrupt forwarding enabled"]
pub type VDDIO_ACTIVE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `VDDIO_ACTIVE` writer - Masks supply interrupt on VDDIO. '0': VDDIO interrupt forwarding disabled '1': VDDIO interrupt forwarding enabled"]
pub type VDDIO_ACTIVE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, VDD_INTR_MASK_SPEC, u16, u16, 16, O>;
#[doc = "Field `VDDA_ACTIVE` reader - Same as VDDIO_ACTIVE for the analog supply VDDA."]
pub type VDDA_ACTIVE_R = crate::BitReader<bool>;
#[doc = "Field `VDDA_ACTIVE` writer - Same as VDDIO_ACTIVE for the analog supply VDDA."]
pub type VDDA_ACTIVE_W<'a, const O: u8> = crate::BitWriter<'a, u32, VDD_INTR_MASK_SPEC, bool, O>;
#[doc = "Field `VDDD_ACTIVE` reader - Same as VDDIO_ACTIVE for the digital supply VDDD."]
pub type VDDD_ACTIVE_R = crate::BitReader<bool>;
#[doc = "Field `VDDD_ACTIVE` writer - Same as VDDIO_ACTIVE for the digital supply VDDD."]
pub type VDDD_ACTIVE_W<'a, const O: u8> = crate::BitWriter<'a, u32, VDD_INTR_MASK_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:15 - Masks supply interrupt on VDDIO. '0': VDDIO interrupt forwarding disabled '1': VDDIO interrupt forwarding enabled"]
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
impl W {
    #[doc = "Bits 0:15 - Masks supply interrupt on VDDIO. '0': VDDIO interrupt forwarding disabled '1': VDDIO interrupt forwarding enabled"]
    #[inline(always)]
    pub fn vddio_active(&mut self) -> VDDIO_ACTIVE_W<0> {
        VDDIO_ACTIVE_W::new(self)
    }
    #[doc = "Bit 30 - Same as VDDIO_ACTIVE for the analog supply VDDA."]
    #[inline(always)]
    pub fn vdda_active(&mut self) -> VDDA_ACTIVE_W<30> {
        VDDA_ACTIVE_W::new(self)
    }
    #[doc = "Bit 31 - Same as VDDIO_ACTIVE for the digital supply VDDD."]
    #[inline(always)]
    pub fn vddd_active(&mut self) -> VDDD_ACTIVE_W<31> {
        VDDD_ACTIVE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Supply detection interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vdd_intr_mask](index.html) module"]
pub struct VDD_INTR_MASK_SPEC;
impl crate::RegisterSpec for VDD_INTR_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vdd_intr_mask::R](R) reader structure"]
impl crate::Readable for VDD_INTR_MASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vdd_intr_mask::W](W) writer structure"]
impl crate::Writable for VDD_INTR_MASK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets VDD_INTR_MASK to value 0"]
impl crate::Resettable for VDD_INTR_MASK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
