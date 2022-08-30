#[doc = "Register `PWR_BUCK_CTL` reader"]
pub struct R(crate::R<PWR_BUCK_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWR_BUCK_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWR_BUCK_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWR_BUCK_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWR_BUCK_CTL` writer"]
pub struct W(crate::W<PWR_BUCK_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWR_BUCK_CTL_SPEC>;
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
impl From<crate::W<PWR_BUCK_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWR_BUCK_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BUCK_OUT1_SEL` reader - Voltage output selection for vccbuck1 output. This register is only reset by XRES/POR/BOD/HIBERNATE. When increasing the voltage, it can take up to 200us for the output voltage to settle. When decreasing the voltage, the settling time depends on the load current. 0: 0.85V 1: 0.875V 2: 0.90V 3: 0.95V 4: 1.05V 5: 1.10V 6: 1.15V 7: 1.20V"]
pub type BUCK_OUT1_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BUCK_OUT1_SEL` writer - Voltage output selection for vccbuck1 output. This register is only reset by XRES/POR/BOD/HIBERNATE. When increasing the voltage, it can take up to 200us for the output voltage to settle. When decreasing the voltage, the settling time depends on the load current. 0: 0.85V 1: 0.875V 2: 0.90V 3: 0.95V 4: 1.05V 5: 1.10V 6: 1.15V 7: 1.20V"]
pub type BUCK_OUT1_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PWR_BUCK_CTL_SPEC, u8, u8, 3, O>;
#[doc = "Field `BUCK_EN` reader - Master enable for buck converter. This register is only reset by XRES/POR/BOD/HIBERNATE."]
pub type BUCK_EN_R = crate::BitReader<bool>;
#[doc = "Field `BUCK_EN` writer - Master enable for buck converter. This register is only reset by XRES/POR/BOD/HIBERNATE."]
pub type BUCK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_BUCK_CTL_SPEC, bool, O>;
#[doc = "Field `BUCK_OUT1_EN` reader - Enable for vccbuck1 output. The value in this register is ignored unless PWR_BUCK_CTL.BUCK_EN==1. This register is only reset by XRES/POR/BOD/HIBERNATE. The regulator takes up to 600us to charge the external capacitor. If there is additional load current while charging, this will increase the startup time. The TRM specifies the required sequence when transitioning vccd from the LDO to SIMO Buck output #1."]
pub type BUCK_OUT1_EN_R = crate::BitReader<bool>;
#[doc = "Field `BUCK_OUT1_EN` writer - Enable for vccbuck1 output. The value in this register is ignored unless PWR_BUCK_CTL.BUCK_EN==1. This register is only reset by XRES/POR/BOD/HIBERNATE. The regulator takes up to 600us to charge the external capacitor. If there is additional load current while charging, this will increase the startup time. The TRM specifies the required sequence when transitioning vccd from the LDO to SIMO Buck output #1."]
pub type BUCK_OUT1_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_BUCK_CTL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - Voltage output selection for vccbuck1 output. This register is only reset by XRES/POR/BOD/HIBERNATE. When increasing the voltage, it can take up to 200us for the output voltage to settle. When decreasing the voltage, the settling time depends on the load current. 0: 0.85V 1: 0.875V 2: 0.90V 3: 0.95V 4: 1.05V 5: 1.10V 6: 1.15V 7: 1.20V"]
    #[inline(always)]
    pub fn buck_out1_sel(&self) -> BUCK_OUT1_SEL_R {
        BUCK_OUT1_SEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 30 - Master enable for buck converter. This register is only reset by XRES/POR/BOD/HIBERNATE."]
    #[inline(always)]
    pub fn buck_en(&self) -> BUCK_EN_R {
        BUCK_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable for vccbuck1 output. The value in this register is ignored unless PWR_BUCK_CTL.BUCK_EN==1. This register is only reset by XRES/POR/BOD/HIBERNATE. The regulator takes up to 600us to charge the external capacitor. If there is additional load current while charging, this will increase the startup time. The TRM specifies the required sequence when transitioning vccd from the LDO to SIMO Buck output #1."]
    #[inline(always)]
    pub fn buck_out1_en(&self) -> BUCK_OUT1_EN_R {
        BUCK_OUT1_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Voltage output selection for vccbuck1 output. This register is only reset by XRES/POR/BOD/HIBERNATE. When increasing the voltage, it can take up to 200us for the output voltage to settle. When decreasing the voltage, the settling time depends on the load current. 0: 0.85V 1: 0.875V 2: 0.90V 3: 0.95V 4: 1.05V 5: 1.10V 6: 1.15V 7: 1.20V"]
    #[inline(always)]
    pub fn buck_out1_sel(&mut self) -> BUCK_OUT1_SEL_W<0> {
        BUCK_OUT1_SEL_W::new(self)
    }
    #[doc = "Bit 30 - Master enable for buck converter. This register is only reset by XRES/POR/BOD/HIBERNATE."]
    #[inline(always)]
    pub fn buck_en(&mut self) -> BUCK_EN_W<30> {
        BUCK_EN_W::new(self)
    }
    #[doc = "Bit 31 - Enable for vccbuck1 output. The value in this register is ignored unless PWR_BUCK_CTL.BUCK_EN==1. This register is only reset by XRES/POR/BOD/HIBERNATE. The regulator takes up to 600us to charge the external capacitor. If there is additional load current while charging, this will increase the startup time. The TRM specifies the required sequence when transitioning vccd from the LDO to SIMO Buck output #1."]
    #[inline(always)]
    pub fn buck_out1_en(&mut self) -> BUCK_OUT1_EN_W<31> {
        BUCK_OUT1_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Buck Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr_buck_ctl](index.html) module"]
pub struct PWR_BUCK_CTL_SPEC;
impl crate::RegisterSpec for PWR_BUCK_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwr_buck_ctl::R](R) reader structure"]
impl crate::Readable for PWR_BUCK_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwr_buck_ctl::W](W) writer structure"]
impl crate::Writable for PWR_BUCK_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWR_BUCK_CTL to value 0x05"]
impl crate::Resettable for PWR_BUCK_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x05
    }
}
