#[doc = "Register `MT_DELAY_CFG3` reader"]
pub struct R(crate::R<MT_DELAY_CFG3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MT_DELAY_CFG3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MT_DELAY_CFG3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MT_DELAY_CFG3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MT_DELAY_CFG3` writer"]
pub struct W(crate::W<MT_DELAY_CFG3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MT_DELAY_CFG3_SPEC>;
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
impl From<crate::W<MT_DELAY_CFG3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MT_DELAY_CFG3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XTAL_DISABLE_DELAY` reader - This register specifies the time from switching of logic to Retention LDO in CYBLERD55 to XTAL Disable. This should include the post processing time The delay is in terms of LF Clock cycles. FW has to program this register based on the selected LF clock frequency. At the minimum XTAL_DISABLE_DELAY should be the sum of DIG_LDO_DISABLE_DELAY and the powerdown time of ACTIVE_LDO"]
pub type XTAL_DISABLE_DELAY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `XTAL_DISABLE_DELAY` writer - This register specifies the time from switching of logic to Retention LDO in CYBLERD55 to XTAL Disable. This should include the post processing time The delay is in terms of LF Clock cycles. FW has to program this register based on the selected LF clock frequency. At the minimum XTAL_DISABLE_DELAY should be the sum of DIG_LDO_DISABLE_DELAY and the powerdown time of ACTIVE_LDO"]
pub type XTAL_DISABLE_DELAY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MT_DELAY_CFG3_SPEC, u8, u8, 8, O>;
#[doc = "Field `DIG_LDO_DISABLE_DELAY` reader - This field holds the delay from the time of diabling Digital LDO to the time at which ACTIVE regulator is disabled"]
pub type DIG_LDO_DISABLE_DELAY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DIG_LDO_DISABLE_DELAY` writer - This field holds the delay from the time of diabling Digital LDO to the time at which ACTIVE regulator is disabled"]
pub type DIG_LDO_DISABLE_DELAY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MT_DELAY_CFG3_SPEC, u8, u8, 8, O>;
#[doc = "Field `VDDR_STABLE_DELAY` reader - This field holds the delay after HVLDO Startup to VDDR Stable. Refer to memo AKK-410"]
pub type VDDR_STABLE_DELAY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VDDR_STABLE_DELAY` writer - This field holds the delay after HVLDO Startup to VDDR Stable. Refer to memo AKK-410"]
pub type VDDR_STABLE_DELAY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MT_DELAY_CFG3_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - This register specifies the time from switching of logic to Retention LDO in CYBLERD55 to XTAL Disable. This should include the post processing time The delay is in terms of LF Clock cycles. FW has to program this register based on the selected LF clock frequency. At the minimum XTAL_DISABLE_DELAY should be the sum of DIG_LDO_DISABLE_DELAY and the powerdown time of ACTIVE_LDO"]
    #[inline(always)]
    pub fn xtal_disable_delay(&self) -> XTAL_DISABLE_DELAY_R {
        XTAL_DISABLE_DELAY_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - This field holds the delay from the time of diabling Digital LDO to the time at which ACTIVE regulator is disabled"]
    #[inline(always)]
    pub fn dig_ldo_disable_delay(&self) -> DIG_LDO_DISABLE_DELAY_R {
        DIG_LDO_DISABLE_DELAY_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - This field holds the delay after HVLDO Startup to VDDR Stable. Refer to memo AKK-410"]
    #[inline(always)]
    pub fn vddr_stable_delay(&self) -> VDDR_STABLE_DELAY_R {
        VDDR_STABLE_DELAY_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - This register specifies the time from switching of logic to Retention LDO in CYBLERD55 to XTAL Disable. This should include the post processing time The delay is in terms of LF Clock cycles. FW has to program this register based on the selected LF clock frequency. At the minimum XTAL_DISABLE_DELAY should be the sum of DIG_LDO_DISABLE_DELAY and the powerdown time of ACTIVE_LDO"]
    #[inline(always)]
    pub fn xtal_disable_delay(&mut self) -> XTAL_DISABLE_DELAY_W<0> {
        XTAL_DISABLE_DELAY_W::new(self)
    }
    #[doc = "Bits 8:15 - This field holds the delay from the time of diabling Digital LDO to the time at which ACTIVE regulator is disabled"]
    #[inline(always)]
    pub fn dig_ldo_disable_delay(&mut self) -> DIG_LDO_DISABLE_DELAY_W<8> {
        DIG_LDO_DISABLE_DELAY_W::new(self)
    }
    #[doc = "Bits 16:23 - This field holds the delay after HVLDO Startup to VDDR Stable. Refer to memo AKK-410"]
    #[inline(always)]
    pub fn vddr_stable_delay(&mut self) -> VDDR_STABLE_DELAY_W<16> {
        VDDR_STABLE_DELAY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MT Delay configuration for state transitions\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mt_delay_cfg3](index.html) module"]
pub struct MT_DELAY_CFG3_SPEC;
impl crate::RegisterSpec for MT_DELAY_CFG3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mt_delay_cfg3::R](R) reader structure"]
impl crate::Readable for MT_DELAY_CFG3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mt_delay_cfg3::W](W) writer structure"]
impl crate::Writable for MT_DELAY_CFG3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MT_DELAY_CFG3 to value 0"]
impl crate::Resettable for MT_DELAY_CFG3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
