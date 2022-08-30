#[doc = "Register `TRIM_LDO_0` reader"]
pub struct R(crate::R<TRIM_LDO_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRIM_LDO_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRIM_LDO_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRIM_LDO_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRIM_LDO_0` writer"]
pub struct W(crate::W<TRIM_LDO_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRIM_LDO_0_SPEC>;
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
impl From<crate::W<TRIM_LDO_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRIM_LDO_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ACT_LDO_VREG` reader - To trim the regulated voltage in steps of 25mV typically"]
pub type ACT_LDO_VREG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ACT_LDO_VREG` writer - To trim the regulated voltage in steps of 25mV typically"]
pub type ACT_LDO_VREG_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TRIM_LDO_0_SPEC, u8, u8, 4, O>;
#[doc = "Field `ACT_LDO_ITAIL` reader - To trim the bias currents for all the active mode blocks"]
pub type ACT_LDO_ITAIL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ACT_LDO_ITAIL` writer - To trim the bias currents for all the active mode blocks"]
pub type ACT_LDO_ITAIL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TRIM_LDO_0_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - To trim the regulated voltage in steps of 25mV typically"]
    #[inline(always)]
    pub fn act_ldo_vreg(&self) -> ACT_LDO_VREG_R {
        ACT_LDO_VREG_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - To trim the bias currents for all the active mode blocks"]
    #[inline(always)]
    pub fn act_ldo_itail(&self) -> ACT_LDO_ITAIL_R {
        ACT_LDO_ITAIL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - To trim the regulated voltage in steps of 25mV typically"]
    #[inline(always)]
    pub fn act_ldo_vreg(&mut self) -> ACT_LDO_VREG_W<0> {
        ACT_LDO_VREG_W::new(self)
    }
    #[doc = "Bits 4:7 - To trim the bias currents for all the active mode blocks"]
    #[inline(always)]
    pub fn act_ldo_itail(&mut self) -> ACT_LDO_ITAIL_W<4> {
        ACT_LDO_ITAIL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LDO Trim register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trim_ldo_0](index.html) module"]
pub struct TRIM_LDO_0_SPEC;
impl crate::RegisterSpec for TRIM_LDO_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [trim_ldo_0::R](R) reader structure"]
impl crate::Readable for TRIM_LDO_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [trim_ldo_0::W](W) writer structure"]
impl crate::Writable for TRIM_LDO_0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TRIM_LDO_0 to value 0x58"]
impl crate::Resettable for TRIM_LDO_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x58
    }
}
