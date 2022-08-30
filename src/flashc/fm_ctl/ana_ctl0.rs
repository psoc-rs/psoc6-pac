#[doc = "Register `ANA_CTL0` reader"]
pub struct R(crate::R<ANA_CTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ANA_CTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ANA_CTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ANA_CTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ANA_CTL0` writer"]
pub struct W(crate::W<ANA_CTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ANA_CTL0_SPEC>;
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
impl From<crate::W<ANA_CTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ANA_CTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSLDAC` reader - Trimming of common source line DAC."]
pub type CSLDAC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CSLDAC` writer - Trimming of common source line DAC."]
pub type CSLDAC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ANA_CTL0_SPEC, u8, u8, 3, O>;
#[doc = "Field `VCC_SEL` reader - Vcc select: '0': 1.2 V : LP reset value '1': 0.95 V: ULP reset value Note: the flash macro compiler has a configuration option that specifies the default/reset value of this field."]
pub type VCC_SEL_R = crate::BitReader<bool>;
#[doc = "Field `VCC_SEL` writer - Vcc select: '0': 1.2 V : LP reset value '1': 0.95 V: ULP reset value Note: the flash macro compiler has a configuration option that specifies the default/reset value of this field."]
pub type VCC_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, ANA_CTL0_SPEC, bool, O>;
#[doc = "Field `FLIP_AMUXBUS_AB` reader - Flips amuxbusa and amuxbusb '0': amuxbusa, amuxbusb '1': amuxbusb, amuxbusb"]
pub type FLIP_AMUXBUS_AB_R = crate::BitReader<bool>;
#[doc = "Field `FLIP_AMUXBUS_AB` writer - Flips amuxbusa and amuxbusb '0': amuxbusa, amuxbusb '1': amuxbusb, amuxbusb"]
pub type FLIP_AMUXBUS_AB_W<'a, const O: u8> = crate::BitWriter<'a, u32, ANA_CTL0_SPEC, bool, O>;
impl R {
    #[doc = "Bits 8:10 - Trimming of common source line DAC."]
    #[inline(always)]
    pub fn csldac(&self) -> CSLDAC_R {
        CSLDAC_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 24 - Vcc select: '0': 1.2 V : LP reset value '1': 0.95 V: ULP reset value Note: the flash macro compiler has a configuration option that specifies the default/reset value of this field."]
    #[inline(always)]
    pub fn vcc_sel(&self) -> VCC_SEL_R {
        VCC_SEL_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 27 - Flips amuxbusa and amuxbusb '0': amuxbusa, amuxbusb '1': amuxbusb, amuxbusb"]
    #[inline(always)]
    pub fn flip_amuxbus_ab(&self) -> FLIP_AMUXBUS_AB_R {
        FLIP_AMUXBUS_AB_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 8:10 - Trimming of common source line DAC."]
    #[inline(always)]
    pub fn csldac(&mut self) -> CSLDAC_W<8> {
        CSLDAC_W::new(self)
    }
    #[doc = "Bit 24 - Vcc select: '0': 1.2 V : LP reset value '1': 0.95 V: ULP reset value Note: the flash macro compiler has a configuration option that specifies the default/reset value of this field."]
    #[inline(always)]
    pub fn vcc_sel(&mut self) -> VCC_SEL_W<24> {
        VCC_SEL_W::new(self)
    }
    #[doc = "Bit 27 - Flips amuxbusa and amuxbusb '0': amuxbusa, amuxbusb '1': amuxbusb, amuxbusb"]
    #[inline(always)]
    pub fn flip_amuxbus_ab(&mut self) -> FLIP_AMUXBUS_AB_W<27> {
        FLIP_AMUXBUS_AB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Analog control 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ana_ctl0](index.html) module"]
pub struct ANA_CTL0_SPEC;
impl crate::RegisterSpec for ANA_CTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ana_ctl0::R](R) reader structure"]
impl crate::Readable for ANA_CTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ana_ctl0::W](W) writer structure"]
impl crate::Writable for ANA_CTL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ANA_CTL0 to value 0x0400"]
impl crate::Resettable for ANA_CTL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0400
    }
}
