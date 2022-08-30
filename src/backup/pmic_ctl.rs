#[doc = "Register `PMIC_CTL` reader"]
pub struct R(crate::R<PMIC_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMIC_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PMIC_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PMIC_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PMIC_CTL` writer"]
pub struct W(crate::W<PMIC_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMIC_CTL_SPEC>;
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
impl From<crate::W<PMIC_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PMIC_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UNLOCK` reader - This byte must be set to 0x3A for PMIC to be disabled. When the UNLOCK code is not present: writes to PMIC_EN field are ignored and the hardware ignores the value in PMIC_EN. Do not change PMIC_EN in the same write cycle as setting/clearing the UNLOCK code; do these in separate write cycles."]
pub type UNLOCK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UNLOCK` writer - This byte must be set to 0x3A for PMIC to be disabled. When the UNLOCK code is not present: writes to PMIC_EN field are ignored and the hardware ignores the value in PMIC_EN. Do not change PMIC_EN in the same write cycle as setting/clearing the UNLOCK code; do these in separate write cycles."]
pub type UNLOCK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PMIC_CTL_SPEC, u8, u8, 8, O>;
#[doc = "Field `POLARITY` reader - N/A"]
pub type POLARITY_R = crate::BitReader<bool>;
#[doc = "Field `POLARITY` writer - N/A"]
pub type POLARITY_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMIC_CTL_SPEC, bool, O>;
#[doc = "Field `PMIC_EN_OUTEN` reader - Output enable for the output driver in the PMIC_EN pad. 0: Output pad is tristate for PMIC_EN pin. This can allow this pin to be used for another purpose. Tristate condition is kept only if the UNLOCK key (0x3A) is present 1: Output pad is enabled for PMIC_EN pin."]
pub type PMIC_EN_OUTEN_R = crate::BitReader<bool>;
#[doc = "Field `PMIC_EN_OUTEN` writer - Output enable for the output driver in the PMIC_EN pad. 0: Output pad is tristate for PMIC_EN pin. This can allow this pin to be used for another purpose. Tristate condition is kept only if the UNLOCK key (0x3A) is present 1: Output pad is enabled for PMIC_EN pin."]
pub type PMIC_EN_OUTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMIC_CTL_SPEC, bool, O>;
#[doc = "Field `PMIC_ALWAYSEN` reader - Override normal PMIC controls to prevent accidentally turning off the PMIC by errant firmware. 0: Normal operation, PMIC_EN and PMIC_OUTEN work as described 1: PMIC_EN and PMIC_OUTEN are ignored and the output pad is forced enabled. Note: This bit is a write-once bit until the next backup reset."]
pub type PMIC_ALWAYSEN_R = crate::BitReader<bool>;
#[doc = "Field `PMIC_ALWAYSEN` writer - Override normal PMIC controls to prevent accidentally turning off the PMIC by errant firmware. 0: Normal operation, PMIC_EN and PMIC_OUTEN work as described 1: PMIC_EN and PMIC_OUTEN are ignored and the output pad is forced enabled. Note: This bit is a write-once bit until the next backup reset."]
pub type PMIC_ALWAYSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMIC_CTL_SPEC, bool, O>;
#[doc = "Field `PMIC_EN` reader - Enable for external PMIC that supplies vddd (if present). This bit will only clear if UNLOCK was written correctly in a previous write operation and PMIC_ALWAYSEN=0. When PMIC_EN=0, the system functions normally until vddd is no longer present (OFF w/Backup mode). Firmware can set this bit, if it does so before vddd is actually removed. This bit is also set by any RTC alarm or PMIC pin wakeup event regardless of UNLOCK setting."]
pub type PMIC_EN_R = crate::BitReader<bool>;
#[doc = "Field `PMIC_EN` writer - Enable for external PMIC that supplies vddd (if present). This bit will only clear if UNLOCK was written correctly in a previous write operation and PMIC_ALWAYSEN=0. When PMIC_EN=0, the system functions normally until vddd is no longer present (OFF w/Backup mode). Firmware can set this bit, if it does so before vddd is actually removed. This bit is also set by any RTC alarm or PMIC pin wakeup event regardless of UNLOCK setting."]
pub type PMIC_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMIC_CTL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 8:15 - This byte must be set to 0x3A for PMIC to be disabled. When the UNLOCK code is not present: writes to PMIC_EN field are ignored and the hardware ignores the value in PMIC_EN. Do not change PMIC_EN in the same write cycle as setting/clearing the UNLOCK code; do these in separate write cycles."]
    #[inline(always)]
    pub fn unlock(&self) -> UNLOCK_R {
        UNLOCK_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - N/A"]
    #[inline(always)]
    pub fn polarity(&self) -> POLARITY_R {
        POLARITY_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 29 - Output enable for the output driver in the PMIC_EN pad. 0: Output pad is tristate for PMIC_EN pin. This can allow this pin to be used for another purpose. Tristate condition is kept only if the UNLOCK key (0x3A) is present 1: Output pad is enabled for PMIC_EN pin."]
    #[inline(always)]
    pub fn pmic_en_outen(&self) -> PMIC_EN_OUTEN_R {
        PMIC_EN_OUTEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Override normal PMIC controls to prevent accidentally turning off the PMIC by errant firmware. 0: Normal operation, PMIC_EN and PMIC_OUTEN work as described 1: PMIC_EN and PMIC_OUTEN are ignored and the output pad is forced enabled. Note: This bit is a write-once bit until the next backup reset."]
    #[inline(always)]
    pub fn pmic_alwaysen(&self) -> PMIC_ALWAYSEN_R {
        PMIC_ALWAYSEN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable for external PMIC that supplies vddd (if present). This bit will only clear if UNLOCK was written correctly in a previous write operation and PMIC_ALWAYSEN=0. When PMIC_EN=0, the system functions normally until vddd is no longer present (OFF w/Backup mode). Firmware can set this bit, if it does so before vddd is actually removed. This bit is also set by any RTC alarm or PMIC pin wakeup event regardless of UNLOCK setting."]
    #[inline(always)]
    pub fn pmic_en(&self) -> PMIC_EN_R {
        PMIC_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 8:15 - This byte must be set to 0x3A for PMIC to be disabled. When the UNLOCK code is not present: writes to PMIC_EN field are ignored and the hardware ignores the value in PMIC_EN. Do not change PMIC_EN in the same write cycle as setting/clearing the UNLOCK code; do these in separate write cycles."]
    #[inline(always)]
    pub fn unlock(&mut self) -> UNLOCK_W<8> {
        UNLOCK_W::new(self)
    }
    #[doc = "Bit 16 - N/A"]
    #[inline(always)]
    pub fn polarity(&mut self) -> POLARITY_W<16> {
        POLARITY_W::new(self)
    }
    #[doc = "Bit 29 - Output enable for the output driver in the PMIC_EN pad. 0: Output pad is tristate for PMIC_EN pin. This can allow this pin to be used for another purpose. Tristate condition is kept only if the UNLOCK key (0x3A) is present 1: Output pad is enabled for PMIC_EN pin."]
    #[inline(always)]
    pub fn pmic_en_outen(&mut self) -> PMIC_EN_OUTEN_W<29> {
        PMIC_EN_OUTEN_W::new(self)
    }
    #[doc = "Bit 30 - Override normal PMIC controls to prevent accidentally turning off the PMIC by errant firmware. 0: Normal operation, PMIC_EN and PMIC_OUTEN work as described 1: PMIC_EN and PMIC_OUTEN are ignored and the output pad is forced enabled. Note: This bit is a write-once bit until the next backup reset."]
    #[inline(always)]
    pub fn pmic_alwaysen(&mut self) -> PMIC_ALWAYSEN_W<30> {
        PMIC_ALWAYSEN_W::new(self)
    }
    #[doc = "Bit 31 - Enable for external PMIC that supplies vddd (if present). This bit will only clear if UNLOCK was written correctly in a previous write operation and PMIC_ALWAYSEN=0. When PMIC_EN=0, the system functions normally until vddd is no longer present (OFF w/Backup mode). Firmware can set this bit, if it does so before vddd is actually removed. This bit is also set by any RTC alarm or PMIC pin wakeup event regardless of UNLOCK setting."]
    #[inline(always)]
    pub fn pmic_en(&mut self) -> PMIC_EN_W<31> {
        PMIC_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PMIC control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmic_ctl](index.html) module"]
pub struct PMIC_CTL_SPEC;
impl crate::RegisterSpec for PMIC_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pmic_ctl::R](R) reader structure"]
impl crate::Readable for PMIC_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pmic_ctl::W](W) writer structure"]
impl crate::Writable for PMIC_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PMIC_CTL to value 0xa000_0000"]
impl crate::Resettable for PMIC_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xa000_0000
    }
}
