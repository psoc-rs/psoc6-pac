#[doc = "Register `TRIM_LDO_3` reader"]
pub struct R(crate::R<TRIM_LDO_3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRIM_LDO_3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRIM_LDO_3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRIM_LDO_3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRIM_LDO_3` writer"]
pub struct W(crate::W<TRIM_LDO_3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRIM_LDO_3_SPEC>;
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
impl From<crate::W<TRIM_LDO_3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRIM_LDO_3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LVDET` reader - To trim the trip points of the LV-Detect block"]
pub type LVDET_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LVDET` writer - To trim the trip points of the LV-Detect block"]
pub type LVDET_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TRIM_LDO_3_SPEC, u8, u8, 5, O>;
#[doc = "Field `SLOPE_SB_BMULT` reader - To trim standby regulator beta-multiplier temp-co slope"]
pub type SLOPE_SB_BMULT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SLOPE_SB_BMULT` writer - To trim standby regulator beta-multiplier temp-co slope"]
pub type SLOPE_SB_BMULT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TRIM_LDO_3_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:4 - To trim the trip points of the LV-Detect block"]
    #[inline(always)]
    pub fn lvdet(&self) -> LVDET_R {
        LVDET_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:6 - To trim standby regulator beta-multiplier temp-co slope"]
    #[inline(always)]
    pub fn slope_sb_bmult(&self) -> SLOPE_SB_BMULT_R {
        SLOPE_SB_BMULT_R::new(((self.bits >> 5) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - To trim the trip points of the LV-Detect block"]
    #[inline(always)]
    pub fn lvdet(&mut self) -> LVDET_W<0> {
        LVDET_W::new(self)
    }
    #[doc = "Bits 5:6 - To trim standby regulator beta-multiplier temp-co slope"]
    #[inline(always)]
    pub fn slope_sb_bmult(&mut self) -> SLOPE_SB_BMULT_W<5> {
        SLOPE_SB_BMULT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LDO Trim register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trim_ldo_3](index.html) module"]
pub struct TRIM_LDO_3_SPEC;
impl crate::RegisterSpec for TRIM_LDO_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [trim_ldo_3::R](R) reader structure"]
impl crate::Readable for TRIM_LDO_3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [trim_ldo_3::W](W) writer structure"]
impl crate::Writable for TRIM_LDO_3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TRIM_LDO_3 to value 0x10"]
impl crate::Resettable for TRIM_LDO_3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x10
    }
}
