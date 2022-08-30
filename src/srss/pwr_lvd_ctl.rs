#[doc = "Register `PWR_LVD_CTL` reader"]
pub struct R(crate::R<PWR_LVD_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWR_LVD_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWR_LVD_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWR_LVD_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWR_LVD_CTL` writer"]
pub struct W(crate::W<PWR_LVD_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWR_LVD_CTL_SPEC>;
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
impl From<crate::W<PWR_LVD_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWR_LVD_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HVLVD1_TRIPSEL` reader - Threshold selection for HVLVD1. Disable the LVD (HVLVD1_EN=0) before changing the threshold. 0: rise=1.225V (nom), fall=1.2V (nom) 1: rise=1.425V (nom), fall=1.4V (nom) 2: rise=1.625V (nom), fall=1.6V (nom) 3: rise=1.825V (nom), fall=1.8V (nom) 4: rise=2.025V (nom), fall=2V (nom) 5: rise=2.125V (nom), fall=2.1V (nom) 6: rise=2.225V (nom), fall=2.2V (nom) 7: rise=2.325V (nom), fall=2.3V (nom) 8: rise=2.425V (nom), fall=2.4V (nom) 9: rise=2.525V (nom), fall=2.5V (nom) 10: rise=2.625V (nom), fall=2.6V (nom) 11: rise=2.725V (nom), fall=2.7V (nom) 12: rise=2.825V (nom), fall=2.8V (nom) 13: rise=2.925V (nom), fall=2.9V (nom) 14: rise=3.025V (nom), fall=3.0V (nom) 15: rise=3.125V (nom), fall=3.1V (nom)"]
pub type HVLVD1_TRIPSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HVLVD1_TRIPSEL` writer - Threshold selection for HVLVD1. Disable the LVD (HVLVD1_EN=0) before changing the threshold. 0: rise=1.225V (nom), fall=1.2V (nom) 1: rise=1.425V (nom), fall=1.4V (nom) 2: rise=1.625V (nom), fall=1.6V (nom) 3: rise=1.825V (nom), fall=1.8V (nom) 4: rise=2.025V (nom), fall=2V (nom) 5: rise=2.125V (nom), fall=2.1V (nom) 6: rise=2.225V (nom), fall=2.2V (nom) 7: rise=2.325V (nom), fall=2.3V (nom) 8: rise=2.425V (nom), fall=2.4V (nom) 9: rise=2.525V (nom), fall=2.5V (nom) 10: rise=2.625V (nom), fall=2.6V (nom) 11: rise=2.725V (nom), fall=2.7V (nom) 12: rise=2.825V (nom), fall=2.8V (nom) 13: rise=2.925V (nom), fall=2.9V (nom) 14: rise=3.025V (nom), fall=3.0V (nom) 15: rise=3.125V (nom), fall=3.1V (nom)"]
pub type HVLVD1_TRIPSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PWR_LVD_CTL_SPEC, u8, u8, 4, O>;
#[doc = "Source selection for HVLVD1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum HVLVD1_SRCSEL_A {
    #[doc = "0: Select VDDD"]
    VDDD = 0,
    #[doc = "1: Select AMUXBUSA (VDDD branch)"]
    AMUXBUSA = 1,
    #[doc = "2: N/A"]
    RSVD = 2,
    #[doc = "3: N/A"]
    VDDIO = 3,
    #[doc = "4: Select AMUXBUSB (VDDD branch)"]
    AMUXBUSB = 4,
}
impl From<HVLVD1_SRCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: HVLVD1_SRCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `HVLVD1_SRCSEL` reader - Source selection for HVLVD1"]
pub type HVLVD1_SRCSEL_R = crate::FieldReader<u8, HVLVD1_SRCSEL_A>;
impl HVLVD1_SRCSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<HVLVD1_SRCSEL_A> {
        match self.bits {
            0 => Some(HVLVD1_SRCSEL_A::VDDD),
            1 => Some(HVLVD1_SRCSEL_A::AMUXBUSA),
            2 => Some(HVLVD1_SRCSEL_A::RSVD),
            3 => Some(HVLVD1_SRCSEL_A::VDDIO),
            4 => Some(HVLVD1_SRCSEL_A::AMUXBUSB),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VDDD`"]
    #[inline(always)]
    pub fn is_vddd(&self) -> bool {
        *self == HVLVD1_SRCSEL_A::VDDD
    }
    #[doc = "Checks if the value of the field is `AMUXBUSA`"]
    #[inline(always)]
    pub fn is_amuxbusa(&self) -> bool {
        *self == HVLVD1_SRCSEL_A::AMUXBUSA
    }
    #[doc = "Checks if the value of the field is `RSVD`"]
    #[inline(always)]
    pub fn is_rsvd(&self) -> bool {
        *self == HVLVD1_SRCSEL_A::RSVD
    }
    #[doc = "Checks if the value of the field is `VDDIO`"]
    #[inline(always)]
    pub fn is_vddio(&self) -> bool {
        *self == HVLVD1_SRCSEL_A::VDDIO
    }
    #[doc = "Checks if the value of the field is `AMUXBUSB`"]
    #[inline(always)]
    pub fn is_amuxbusb(&self) -> bool {
        *self == HVLVD1_SRCSEL_A::AMUXBUSB
    }
}
#[doc = "Field `HVLVD1_SRCSEL` writer - Source selection for HVLVD1"]
pub type HVLVD1_SRCSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PWR_LVD_CTL_SPEC, u8, HVLVD1_SRCSEL_A, 3, O>;
impl<'a, const O: u8> HVLVD1_SRCSEL_W<'a, O> {
    #[doc = "Select VDDD"]
    #[inline(always)]
    pub fn vddd(self) -> &'a mut W {
        self.variant(HVLVD1_SRCSEL_A::VDDD)
    }
    #[doc = "Select AMUXBUSA (VDDD branch)"]
    #[inline(always)]
    pub fn amuxbusa(self) -> &'a mut W {
        self.variant(HVLVD1_SRCSEL_A::AMUXBUSA)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn rsvd(self) -> &'a mut W {
        self.variant(HVLVD1_SRCSEL_A::RSVD)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn vddio(self) -> &'a mut W {
        self.variant(HVLVD1_SRCSEL_A::VDDIO)
    }
    #[doc = "Select AMUXBUSB (VDDD branch)"]
    #[inline(always)]
    pub fn amuxbusb(self) -> &'a mut W {
        self.variant(HVLVD1_SRCSEL_A::AMUXBUSB)
    }
}
#[doc = "Field `HVLVD1_EN` reader - Enable HVLVD1 voltage monitor. When the LVD is enabled, it takes 20us for it to settle. There is no hardware stabilization counter, and it may falsely trigger during settling. It is recommended that firmware keep the interrupt masked for at least 8us, write a 1'b1 to the corresponding SRSS_INTR field to any falsely pended interrupt, and then optionally unmask the interrupt. After enabling, it is further recommended to read the related PWR_LVD_STATUS field, since the interrupt only triggers on edges. This bit is cleared (LVD is disabled) when entering DEEPSLEEP to prevent false interrupts during wakeup."]
pub type HVLVD1_EN_R = crate::BitReader<bool>;
#[doc = "Field `HVLVD1_EN` writer - Enable HVLVD1 voltage monitor. When the LVD is enabled, it takes 20us for it to settle. There is no hardware stabilization counter, and it may falsely trigger during settling. It is recommended that firmware keep the interrupt masked for at least 8us, write a 1'b1 to the corresponding SRSS_INTR field to any falsely pended interrupt, and then optionally unmask the interrupt. After enabling, it is further recommended to read the related PWR_LVD_STATUS field, since the interrupt only triggers on edges. This bit is cleared (LVD is disabled) when entering DEEPSLEEP to prevent false interrupts during wakeup."]
pub type HVLVD1_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_LVD_CTL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - Threshold selection for HVLVD1. Disable the LVD (HVLVD1_EN=0) before changing the threshold. 0: rise=1.225V (nom), fall=1.2V (nom) 1: rise=1.425V (nom), fall=1.4V (nom) 2: rise=1.625V (nom), fall=1.6V (nom) 3: rise=1.825V (nom), fall=1.8V (nom) 4: rise=2.025V (nom), fall=2V (nom) 5: rise=2.125V (nom), fall=2.1V (nom) 6: rise=2.225V (nom), fall=2.2V (nom) 7: rise=2.325V (nom), fall=2.3V (nom) 8: rise=2.425V (nom), fall=2.4V (nom) 9: rise=2.525V (nom), fall=2.5V (nom) 10: rise=2.625V (nom), fall=2.6V (nom) 11: rise=2.725V (nom), fall=2.7V (nom) 12: rise=2.825V (nom), fall=2.8V (nom) 13: rise=2.925V (nom), fall=2.9V (nom) 14: rise=3.025V (nom), fall=3.0V (nom) 15: rise=3.125V (nom), fall=3.1V (nom)"]
    #[inline(always)]
    pub fn hvlvd1_tripsel(&self) -> HVLVD1_TRIPSEL_R {
        HVLVD1_TRIPSEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Source selection for HVLVD1"]
    #[inline(always)]
    pub fn hvlvd1_srcsel(&self) -> HVLVD1_SRCSEL_R {
        HVLVD1_SRCSEL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Enable HVLVD1 voltage monitor. When the LVD is enabled, it takes 20us for it to settle. There is no hardware stabilization counter, and it may falsely trigger during settling. It is recommended that firmware keep the interrupt masked for at least 8us, write a 1'b1 to the corresponding SRSS_INTR field to any falsely pended interrupt, and then optionally unmask the interrupt. After enabling, it is further recommended to read the related PWR_LVD_STATUS field, since the interrupt only triggers on edges. This bit is cleared (LVD is disabled) when entering DEEPSLEEP to prevent false interrupts during wakeup."]
    #[inline(always)]
    pub fn hvlvd1_en(&self) -> HVLVD1_EN_R {
        HVLVD1_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Threshold selection for HVLVD1. Disable the LVD (HVLVD1_EN=0) before changing the threshold. 0: rise=1.225V (nom), fall=1.2V (nom) 1: rise=1.425V (nom), fall=1.4V (nom) 2: rise=1.625V (nom), fall=1.6V (nom) 3: rise=1.825V (nom), fall=1.8V (nom) 4: rise=2.025V (nom), fall=2V (nom) 5: rise=2.125V (nom), fall=2.1V (nom) 6: rise=2.225V (nom), fall=2.2V (nom) 7: rise=2.325V (nom), fall=2.3V (nom) 8: rise=2.425V (nom), fall=2.4V (nom) 9: rise=2.525V (nom), fall=2.5V (nom) 10: rise=2.625V (nom), fall=2.6V (nom) 11: rise=2.725V (nom), fall=2.7V (nom) 12: rise=2.825V (nom), fall=2.8V (nom) 13: rise=2.925V (nom), fall=2.9V (nom) 14: rise=3.025V (nom), fall=3.0V (nom) 15: rise=3.125V (nom), fall=3.1V (nom)"]
    #[inline(always)]
    pub fn hvlvd1_tripsel(&mut self) -> HVLVD1_TRIPSEL_W<0> {
        HVLVD1_TRIPSEL_W::new(self)
    }
    #[doc = "Bits 4:6 - Source selection for HVLVD1"]
    #[inline(always)]
    pub fn hvlvd1_srcsel(&mut self) -> HVLVD1_SRCSEL_W<4> {
        HVLVD1_SRCSEL_W::new(self)
    }
    #[doc = "Bit 7 - Enable HVLVD1 voltage monitor. When the LVD is enabled, it takes 20us for it to settle. There is no hardware stabilization counter, and it may falsely trigger during settling. It is recommended that firmware keep the interrupt masked for at least 8us, write a 1'b1 to the corresponding SRSS_INTR field to any falsely pended interrupt, and then optionally unmask the interrupt. After enabling, it is further recommended to read the related PWR_LVD_STATUS field, since the interrupt only triggers on edges. This bit is cleared (LVD is disabled) when entering DEEPSLEEP to prevent false interrupts during wakeup."]
    #[inline(always)]
    pub fn hvlvd1_en(&mut self) -> HVLVD1_EN_W<7> {
        HVLVD1_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Low Voltage Detector (LVD) Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr_lvd_ctl](index.html) module"]
pub struct PWR_LVD_CTL_SPEC;
impl crate::RegisterSpec for PWR_LVD_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwr_lvd_ctl::R](R) reader structure"]
impl crate::Readable for PWR_LVD_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwr_lvd_ctl::W](W) writer structure"]
impl crate::Writable for PWR_LVD_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWR_LVD_CTL to value 0"]
impl crate::Resettable for PWR_LVD_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
