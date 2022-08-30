#[doc = "Register `MT_DELAY_CFG` reader"]
pub struct R(crate::R<MT_DELAY_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MT_DELAY_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MT_DELAY_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MT_DELAY_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MT_DELAY_CFG` writer"]
pub struct W(crate::W<MT_DELAY_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MT_DELAY_CFG_SPEC>;
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
impl From<crate::W<MT_DELAY_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MT_DELAY_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HVLDO_STARTUP_DELAY` reader - This register specifies the startup delay for the HVLDO interms of number of LF Clock cycles. FW has to program this register based on the selected LF clock frequency"]
pub type HVLDO_STARTUP_DELAY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HVLDO_STARTUP_DELAY` writer - This register specifies the startup delay for the HVLDO interms of number of LF Clock cycles. FW has to program this register based on the selected LF clock frequency"]
pub type HVLDO_STARTUP_DELAY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MT_DELAY_CFG_SPEC, u8, u8, 8, O>;
#[doc = "Field `ISOLATE_DEASSERT_DELAY` reader - This register specifies the time from switching the CYBLERD55 logic to Active regulator to removal of ISOLATE_N"]
pub type ISOLATE_DEASSERT_DELAY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ISOLATE_DEASSERT_DELAY` writer - This register specifies the time from switching the CYBLERD55 logic to Active regulator to removal of ISOLATE_N"]
pub type ISOLATE_DEASSERT_DELAY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MT_DELAY_CFG_SPEC, u8, u8, 8, O>;
#[doc = "Field `ACT_TO_SWITCH_DELAY` reader - This register specifies the time from assertion of ISOLATE_N to switching the CYBLERD55 logic to Retention LDO"]
pub type ACT_TO_SWITCH_DELAY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ACT_TO_SWITCH_DELAY` writer - This register specifies the time from assertion of ISOLATE_N to switching the CYBLERD55 logic to Retention LDO"]
pub type ACT_TO_SWITCH_DELAY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MT_DELAY_CFG_SPEC, u8, u8, 8, O>;
#[doc = "Field `HVLDO_DISABLE_DELAY` reader - This register specifies the time from disabling XTAL to switching of the HVLDO."]
pub type HVLDO_DISABLE_DELAY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HVLDO_DISABLE_DELAY` writer - This register specifies the time from disabling XTAL to switching of the HVLDO."]
pub type HVLDO_DISABLE_DELAY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MT_DELAY_CFG_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - This register specifies the startup delay for the HVLDO interms of number of LF Clock cycles. FW has to program this register based on the selected LF clock frequency"]
    #[inline(always)]
    pub fn hvldo_startup_delay(&self) -> HVLDO_STARTUP_DELAY_R {
        HVLDO_STARTUP_DELAY_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - This register specifies the time from switching the CYBLERD55 logic to Active regulator to removal of ISOLATE_N"]
    #[inline(always)]
    pub fn isolate_deassert_delay(&self) -> ISOLATE_DEASSERT_DELAY_R {
        ISOLATE_DEASSERT_DELAY_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - This register specifies the time from assertion of ISOLATE_N to switching the CYBLERD55 logic to Retention LDO"]
    #[inline(always)]
    pub fn act_to_switch_delay(&self) -> ACT_TO_SWITCH_DELAY_R {
        ACT_TO_SWITCH_DELAY_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - This register specifies the time from disabling XTAL to switching of the HVLDO."]
    #[inline(always)]
    pub fn hvldo_disable_delay(&self) -> HVLDO_DISABLE_DELAY_R {
        HVLDO_DISABLE_DELAY_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - This register specifies the startup delay for the HVLDO interms of number of LF Clock cycles. FW has to program this register based on the selected LF clock frequency"]
    #[inline(always)]
    pub fn hvldo_startup_delay(&mut self) -> HVLDO_STARTUP_DELAY_W<0> {
        HVLDO_STARTUP_DELAY_W::new(self)
    }
    #[doc = "Bits 8:15 - This register specifies the time from switching the CYBLERD55 logic to Active regulator to removal of ISOLATE_N"]
    #[inline(always)]
    pub fn isolate_deassert_delay(&mut self) -> ISOLATE_DEASSERT_DELAY_W<8> {
        ISOLATE_DEASSERT_DELAY_W::new(self)
    }
    #[doc = "Bits 16:23 - This register specifies the time from assertion of ISOLATE_N to switching the CYBLERD55 logic to Retention LDO"]
    #[inline(always)]
    pub fn act_to_switch_delay(&mut self) -> ACT_TO_SWITCH_DELAY_W<16> {
        ACT_TO_SWITCH_DELAY_W::new(self)
    }
    #[doc = "Bits 24:31 - This register specifies the time from disabling XTAL to switching of the HVLDO."]
    #[inline(always)]
    pub fn hvldo_disable_delay(&mut self) -> HVLDO_DISABLE_DELAY_W<24> {
        HVLDO_DISABLE_DELAY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MT Delay configuration for state transitions\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mt_delay_cfg](index.html) module"]
pub struct MT_DELAY_CFG_SPEC;
impl crate::RegisterSpec for MT_DELAY_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mt_delay_cfg::R](R) reader structure"]
impl crate::Readable for MT_DELAY_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mt_delay_cfg::W](W) writer structure"]
impl crate::Writable for MT_DELAY_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MT_DELAY_CFG to value 0"]
impl crate::Resettable for MT_DELAY_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
