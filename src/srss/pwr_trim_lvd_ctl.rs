#[doc = "Register `PWR_TRIM_LVD_CTL` reader"]
pub struct R(crate::R<PWR_TRIM_LVD_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWR_TRIM_LVD_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWR_TRIM_LVD_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWR_TRIM_LVD_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWR_TRIM_LVD_CTL` writer"]
pub struct W(crate::W<PWR_TRIM_LVD_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWR_TRIM_LVD_CTL_SPEC>;
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
impl From<crate::W<PWR_TRIM_LVD_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWR_TRIM_LVD_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HVLVD1_OFSTRIM` reader - HVLVD1 offset trim"]
pub type HVLVD1_OFSTRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HVLVD1_OFSTRIM` writer - HVLVD1 offset trim"]
pub type HVLVD1_OFSTRIM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PWR_TRIM_LVD_CTL_SPEC, u8, u8, 3, O>;
#[doc = "Field `HVLVD1_ITRIM` reader - HVLVD1 current trim"]
pub type HVLVD1_ITRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HVLVD1_ITRIM` writer - HVLVD1 current trim"]
pub type HVLVD1_ITRIM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PWR_TRIM_LVD_CTL_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:2 - HVLVD1 offset trim"]
    #[inline(always)]
    pub fn hvlvd1_ofstrim(&self) -> HVLVD1_OFSTRIM_R {
        HVLVD1_OFSTRIM_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - HVLVD1 current trim"]
    #[inline(always)]
    pub fn hvlvd1_itrim(&self) -> HVLVD1_ITRIM_R {
        HVLVD1_ITRIM_R::new(((self.bits >> 4) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - HVLVD1 offset trim"]
    #[inline(always)]
    pub fn hvlvd1_ofstrim(&mut self) -> HVLVD1_OFSTRIM_W<0> {
        HVLVD1_OFSTRIM_W::new(self)
    }
    #[doc = "Bits 4:6 - HVLVD1 current trim"]
    #[inline(always)]
    pub fn hvlvd1_itrim(&mut self) -> HVLVD1_ITRIM_W<4> {
        HVLVD1_ITRIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LVD Trim Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr_trim_lvd_ctl](index.html) module"]
pub struct PWR_TRIM_LVD_CTL_SPEC;
impl crate::RegisterSpec for PWR_TRIM_LVD_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwr_trim_lvd_ctl::R](R) reader structure"]
impl crate::Readable for PWR_TRIM_LVD_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwr_trim_lvd_ctl::W](W) writer structure"]
impl crate::Writable for PWR_TRIM_LVD_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWR_TRIM_LVD_CTL to value 0x20"]
impl crate::Resettable for PWR_TRIM_LVD_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x20
    }
}
