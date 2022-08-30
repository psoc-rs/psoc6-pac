#[doc = "Register `DPLL_CONFIG` reader"]
pub struct R(crate::R<DPLL_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DPLL_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DPLL_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DPLL_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DPLL_CONFIG` writer"]
pub struct W(crate::W<DPLL_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DPLL_CONFIG_SPEC>;
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
impl From<crate::W<DPLL_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DPLL_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DPLL_CORREL_CONFIG` reader - If MXD_IF_OPTION is 0: Not used If CY_CORREL_EN is 1: \\[11:0\\]
CY correl Access address compare mask for LSB 12 bits. Ideal value is 0xFFF \\[15:12\\]
CY correl maximum number of allowed mismatched bits in access address. Ideal value is 0x0."]
pub type DPLL_CORREL_CONFIG_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DPLL_CORREL_CONFIG` writer - If MXD_IF_OPTION is 0: Not used If CY_CORREL_EN is 1: \\[11:0\\]
CY correl Access address compare mask for LSB 12 bits. Ideal value is 0xFFF \\[15:12\\]
CY correl maximum number of allowed mismatched bits in access address. Ideal value is 0x0."]
pub type DPLL_CORREL_CONFIG_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DPLL_CONFIG_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - If MXD_IF_OPTION is 0: Not used If CY_CORREL_EN is 1: \\[11:0\\]
CY correl Access address compare mask for LSB 12 bits. Ideal value is 0xFFF \\[15:12\\]
CY correl maximum number of allowed mismatched bits in access address. Ideal value is 0x0."]
    #[inline(always)]
    pub fn dpll_correl_config(&self) -> DPLL_CORREL_CONFIG_R {
        DPLL_CORREL_CONFIG_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - If MXD_IF_OPTION is 0: Not used If CY_CORREL_EN is 1: \\[11:0\\]
CY correl Access address compare mask for LSB 12 bits. Ideal value is 0xFFF \\[15:12\\]
CY correl maximum number of allowed mismatched bits in access address. Ideal value is 0x0."]
    #[inline(always)]
    pub fn dpll_correl_config(&mut self) -> DPLL_CORREL_CONFIG_W<0> {
        DPLL_CORREL_CONFIG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DPLL & CY Correlator configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dpll_config](index.html) module"]
pub struct DPLL_CONFIG_SPEC;
impl crate::RegisterSpec for DPLL_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dpll_config::R](R) reader structure"]
impl crate::Readable for DPLL_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dpll_config::W](W) writer structure"]
impl crate::Writable for DPLL_CONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DPLL_CONFIG to value 0"]
impl crate::Resettable for DPLL_CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
