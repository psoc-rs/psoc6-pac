#[doc = "Register `PWR_BUCK_CTL2` reader"]
pub struct R(crate::R<PWR_BUCK_CTL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWR_BUCK_CTL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWR_BUCK_CTL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWR_BUCK_CTL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWR_BUCK_CTL2` writer"]
pub struct W(crate::W<PWR_BUCK_CTL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWR_BUCK_CTL2_SPEC>;
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
impl From<crate::W<PWR_BUCK_CTL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWR_BUCK_CTL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BUCK_OUT2_SEL` reader - Voltage output selection for vccbuck2 output. When increasing the voltage, it can take up to 200us for the output voltage to settle. When decreasing the voltage, the settling time depends on the load current. 0: 1.15V 1: 1.20V 2: 1.25V 3: 1.30V 4: 1.35V 5: 1.40V 6: 1.45V 7: 1.50V"]
pub type BUCK_OUT2_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BUCK_OUT2_SEL` writer - Voltage output selection for vccbuck2 output. When increasing the voltage, it can take up to 200us for the output voltage to settle. When decreasing the voltage, the settling time depends on the load current. 0: 1.15V 1: 1.20V 2: 1.25V 3: 1.30V 4: 1.35V 5: 1.40V 6: 1.45V 7: 1.50V"]
pub type BUCK_OUT2_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PWR_BUCK_CTL2_SPEC, u8, u8, 3, O>;
#[doc = "Field `BUCK_OUT2_HW_SEL` reader - Hardware control for vccbuck2 output. When this bit is set, the value in BUCK_OUT2_EN is ignored and a hardware signal is used instead. If the product has supporting hardware, it can directly control the enable signal for vccbuck2. The same charging time in BUCK_OUT2_EN applies."]
pub type BUCK_OUT2_HW_SEL_R = crate::BitReader<bool>;
#[doc = "Field `BUCK_OUT2_HW_SEL` writer - Hardware control for vccbuck2 output. When this bit is set, the value in BUCK_OUT2_EN is ignored and a hardware signal is used instead. If the product has supporting hardware, it can directly control the enable signal for vccbuck2. The same charging time in BUCK_OUT2_EN applies."]
pub type BUCK_OUT2_HW_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PWR_BUCK_CTL2_SPEC, bool, O>;
#[doc = "Field `BUCK_OUT2_EN` reader - Enable for vccbuck2 output. The value in this register is ignored unless PWR_BUCK_CTL.BUCK_EN==1. The regulator takes up to 600us to charge the external capacitor. If there is additional load current while charging, this will increase the startup time."]
pub type BUCK_OUT2_EN_R = crate::BitReader<bool>;
#[doc = "Field `BUCK_OUT2_EN` writer - Enable for vccbuck2 output. The value in this register is ignored unless PWR_BUCK_CTL.BUCK_EN==1. The regulator takes up to 600us to charge the external capacitor. If there is additional load current while charging, this will increase the startup time."]
pub type BUCK_OUT2_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_BUCK_CTL2_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - Voltage output selection for vccbuck2 output. When increasing the voltage, it can take up to 200us for the output voltage to settle. When decreasing the voltage, the settling time depends on the load current. 0: 1.15V 1: 1.20V 2: 1.25V 3: 1.30V 4: 1.35V 5: 1.40V 6: 1.45V 7: 1.50V"]
    #[inline(always)]
    pub fn buck_out2_sel(&self) -> BUCK_OUT2_SEL_R {
        BUCK_OUT2_SEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 30 - Hardware control for vccbuck2 output. When this bit is set, the value in BUCK_OUT2_EN is ignored and a hardware signal is used instead. If the product has supporting hardware, it can directly control the enable signal for vccbuck2. The same charging time in BUCK_OUT2_EN applies."]
    #[inline(always)]
    pub fn buck_out2_hw_sel(&self) -> BUCK_OUT2_HW_SEL_R {
        BUCK_OUT2_HW_SEL_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable for vccbuck2 output. The value in this register is ignored unless PWR_BUCK_CTL.BUCK_EN==1. The regulator takes up to 600us to charge the external capacitor. If there is additional load current while charging, this will increase the startup time."]
    #[inline(always)]
    pub fn buck_out2_en(&self) -> BUCK_OUT2_EN_R {
        BUCK_OUT2_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Voltage output selection for vccbuck2 output. When increasing the voltage, it can take up to 200us for the output voltage to settle. When decreasing the voltage, the settling time depends on the load current. 0: 1.15V 1: 1.20V 2: 1.25V 3: 1.30V 4: 1.35V 5: 1.40V 6: 1.45V 7: 1.50V"]
    #[inline(always)]
    pub fn buck_out2_sel(&mut self) -> BUCK_OUT2_SEL_W<0> {
        BUCK_OUT2_SEL_W::new(self)
    }
    #[doc = "Bit 30 - Hardware control for vccbuck2 output. When this bit is set, the value in BUCK_OUT2_EN is ignored and a hardware signal is used instead. If the product has supporting hardware, it can directly control the enable signal for vccbuck2. The same charging time in BUCK_OUT2_EN applies."]
    #[inline(always)]
    pub fn buck_out2_hw_sel(&mut self) -> BUCK_OUT2_HW_SEL_W<30> {
        BUCK_OUT2_HW_SEL_W::new(self)
    }
    #[doc = "Bit 31 - Enable for vccbuck2 output. The value in this register is ignored unless PWR_BUCK_CTL.BUCK_EN==1. The regulator takes up to 600us to charge the external capacitor. If there is additional load current while charging, this will increase the startup time."]
    #[inline(always)]
    pub fn buck_out2_en(&mut self) -> BUCK_OUT2_EN_W<31> {
        BUCK_OUT2_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Buck Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr_buck_ctl2](index.html) module"]
pub struct PWR_BUCK_CTL2_SPEC;
impl crate::RegisterSpec for PWR_BUCK_CTL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwr_buck_ctl2::R](R) reader structure"]
impl crate::Readable for PWR_BUCK_CTL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwr_buck_ctl2::W](W) writer structure"]
impl crate::Writable for PWR_BUCK_CTL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWR_BUCK_CTL2 to value 0"]
impl crate::Resettable for PWR_BUCK_CTL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
