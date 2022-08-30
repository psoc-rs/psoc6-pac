#[doc = "Register `PWR_TRIM_BODOVP_CTL` reader"]
pub struct R(crate::R<PWR_TRIM_BODOVP_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWR_TRIM_BODOVP_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWR_TRIM_BODOVP_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWR_TRIM_BODOVP_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWR_TRIM_BODOVP_CTL` writer"]
pub struct W(crate::W<PWR_TRIM_BODOVP_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWR_TRIM_BODOVP_CTL_SPEC>;
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
impl From<crate::W<PWR_TRIM_BODOVP_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWR_TRIM_BODOVP_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HVPORBOD_TRIPSEL` reader - HVPORBOD trip point selection. Monitors vddd. This register is only reset by XRES/POR/BOD/HIBERNATE."]
pub type HVPORBOD_TRIPSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HVPORBOD_TRIPSEL` writer - HVPORBOD trip point selection. Monitors vddd. This register is only reset by XRES/POR/BOD/HIBERNATE."]
pub type HVPORBOD_TRIPSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PWR_TRIM_BODOVP_CTL_SPEC, u8, u8, 3, O>;
#[doc = "Field `HVPORBOD_OFSTRIM` reader - HVPORBOD offset trim. This register is only reset by XRES/POR/BOD/HIBERNATE."]
pub type HVPORBOD_OFSTRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HVPORBOD_OFSTRIM` writer - HVPORBOD offset trim. This register is only reset by XRES/POR/BOD/HIBERNATE."]
pub type HVPORBOD_OFSTRIM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PWR_TRIM_BODOVP_CTL_SPEC, u8, u8, 3, O>;
#[doc = "Field `HVPORBOD_ITRIM` reader - HVPORBOD current trim. This register is only reset by XRES/POR/BOD/HIBERNATE."]
pub type HVPORBOD_ITRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HVPORBOD_ITRIM` writer - HVPORBOD current trim. This register is only reset by XRES/POR/BOD/HIBERNATE."]
pub type HVPORBOD_ITRIM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PWR_TRIM_BODOVP_CTL_SPEC, u8, u8, 3, O>;
#[doc = "Field `LVPORBOD_TRIPSEL` reader - LVPORBOD trip point selection. Monitors vccd. This register is only reset by XRES/POR/BOD/HIBERNATE."]
pub type LVPORBOD_TRIPSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LVPORBOD_TRIPSEL` writer - LVPORBOD trip point selection. Monitors vccd. This register is only reset by XRES/POR/BOD/HIBERNATE."]
pub type LVPORBOD_TRIPSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PWR_TRIM_BODOVP_CTL_SPEC, u8, u8, 3, O>;
#[doc = "Field `LVPORBOD_OFSTRIM` reader - LVPORBOD offset trim. This register is only reset by XRES/POR/BOD/HIBERNATE."]
pub type LVPORBOD_OFSTRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LVPORBOD_OFSTRIM` writer - LVPORBOD offset trim. This register is only reset by XRES/POR/BOD/HIBERNATE."]
pub type LVPORBOD_OFSTRIM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PWR_TRIM_BODOVP_CTL_SPEC, u8, u8, 3, O>;
#[doc = "Field `LVPORBOD_ITRIM` reader - LVPORBOD current trim. This register is only reset by XRES/POR/BOD/HIBERNATE."]
pub type LVPORBOD_ITRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LVPORBOD_ITRIM` writer - LVPORBOD current trim. This register is only reset by XRES/POR/BOD/HIBERNATE."]
pub type LVPORBOD_ITRIM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PWR_TRIM_BODOVP_CTL_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:2 - HVPORBOD trip point selection. Monitors vddd. This register is only reset by XRES/POR/BOD/HIBERNATE."]
    #[inline(always)]
    pub fn hvporbod_tripsel(&self) -> HVPORBOD_TRIPSEL_R {
        HVPORBOD_TRIPSEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - HVPORBOD offset trim. This register is only reset by XRES/POR/BOD/HIBERNATE."]
    #[inline(always)]
    pub fn hvporbod_ofstrim(&self) -> HVPORBOD_OFSTRIM_R {
        HVPORBOD_OFSTRIM_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 7:9 - HVPORBOD current trim. This register is only reset by XRES/POR/BOD/HIBERNATE."]
    #[inline(always)]
    pub fn hvporbod_itrim(&self) -> HVPORBOD_ITRIM_R {
        HVPORBOD_ITRIM_R::new(((self.bits >> 7) & 7) as u8)
    }
    #[doc = "Bits 10:12 - LVPORBOD trip point selection. Monitors vccd. This register is only reset by XRES/POR/BOD/HIBERNATE."]
    #[inline(always)]
    pub fn lvporbod_tripsel(&self) -> LVPORBOD_TRIPSEL_R {
        LVPORBOD_TRIPSEL_R::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bits 14:16 - LVPORBOD offset trim. This register is only reset by XRES/POR/BOD/HIBERNATE."]
    #[inline(always)]
    pub fn lvporbod_ofstrim(&self) -> LVPORBOD_OFSTRIM_R {
        LVPORBOD_OFSTRIM_R::new(((self.bits >> 14) & 7) as u8)
    }
    #[doc = "Bits 17:19 - LVPORBOD current trim. This register is only reset by XRES/POR/BOD/HIBERNATE."]
    #[inline(always)]
    pub fn lvporbod_itrim(&self) -> LVPORBOD_ITRIM_R {
        LVPORBOD_ITRIM_R::new(((self.bits >> 17) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - HVPORBOD trip point selection. Monitors vddd. This register is only reset by XRES/POR/BOD/HIBERNATE."]
    #[inline(always)]
    pub fn hvporbod_tripsel(&mut self) -> HVPORBOD_TRIPSEL_W<0> {
        HVPORBOD_TRIPSEL_W::new(self)
    }
    #[doc = "Bits 4:6 - HVPORBOD offset trim. This register is only reset by XRES/POR/BOD/HIBERNATE."]
    #[inline(always)]
    pub fn hvporbod_ofstrim(&mut self) -> HVPORBOD_OFSTRIM_W<4> {
        HVPORBOD_OFSTRIM_W::new(self)
    }
    #[doc = "Bits 7:9 - HVPORBOD current trim. This register is only reset by XRES/POR/BOD/HIBERNATE."]
    #[inline(always)]
    pub fn hvporbod_itrim(&mut self) -> HVPORBOD_ITRIM_W<7> {
        HVPORBOD_ITRIM_W::new(self)
    }
    #[doc = "Bits 10:12 - LVPORBOD trip point selection. Monitors vccd. This register is only reset by XRES/POR/BOD/HIBERNATE."]
    #[inline(always)]
    pub fn lvporbod_tripsel(&mut self) -> LVPORBOD_TRIPSEL_W<10> {
        LVPORBOD_TRIPSEL_W::new(self)
    }
    #[doc = "Bits 14:16 - LVPORBOD offset trim. This register is only reset by XRES/POR/BOD/HIBERNATE."]
    #[inline(always)]
    pub fn lvporbod_ofstrim(&mut self) -> LVPORBOD_OFSTRIM_W<14> {
        LVPORBOD_OFSTRIM_W::new(self)
    }
    #[doc = "Bits 17:19 - LVPORBOD current trim. This register is only reset by XRES/POR/BOD/HIBERNATE."]
    #[inline(always)]
    pub fn lvporbod_itrim(&mut self) -> LVPORBOD_ITRIM_W<17> {
        LVPORBOD_ITRIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BOD/OVP Trim Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr_trim_bodovp_ctl](index.html) module"]
pub struct PWR_TRIM_BODOVP_CTL_SPEC;
impl crate::RegisterSpec for PWR_TRIM_BODOVP_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwr_trim_bodovp_ctl::R](R) reader structure"]
impl crate::Readable for PWR_TRIM_BODOVP_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwr_trim_bodovp_ctl::W](W) writer structure"]
impl crate::Writable for PWR_TRIM_BODOVP_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWR_TRIM_BODOVP_CTL to value 0x0004_0d04"]
impl crate::Resettable for PWR_TRIM_BODOVP_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0004_0d04
    }
}
