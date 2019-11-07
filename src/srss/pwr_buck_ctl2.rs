#[doc = "Reader of register PWR_BUCK_CTL2"]
pub type R = crate::R<u32, super::PWR_BUCK_CTL2>;
#[doc = "Writer for register PWR_BUCK_CTL2"]
pub type W = crate::W<u32, super::PWR_BUCK_CTL2>;
#[doc = "Register PWR_BUCK_CTL2 `reset()`'s with value 0"]
impl crate::ResetValue for super::PWR_BUCK_CTL2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BUCK_OUT2_SEL`"]
pub type BUCK_OUT2_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BUCK_OUT2_SEL`"]
pub struct BUCK_OUT2_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> BUCK_OUT2_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `BUCK_OUT2_HW_SEL`"]
pub type BUCK_OUT2_HW_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BUCK_OUT2_HW_SEL`"]
pub struct BUCK_OUT2_HW_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> BUCK_OUT2_HW_SEL_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `BUCK_OUT2_EN`"]
pub type BUCK_OUT2_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BUCK_OUT2_EN`"]
pub struct BUCK_OUT2_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> BUCK_OUT2_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Voltage output selection for vccbuck2 output. When increasing the voltage, it can take up to 200us for the output voltage to settle. When decreasing the voltage, the settling time depends on the load current. 0: 1.15V 1: 1.20V 2: 1.25V 3: 1.30V 4: 1.35V 5: 1.40V 6: 1.45V 7: 1.50V"]
    #[inline(always)]
    pub fn buck_out2_sel(&self) -> BUCK_OUT2_SEL_R {
        BUCK_OUT2_SEL_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 30 - Hardware control for vccbuck2 output. When this bit is set, the value in BUCK_OUT2_EN is ignored and a hardware signal is used instead. If the product has supporting hardware, it can directly control the enable signal for vccbuck2. The same charging time in BUCK_OUT2_EN applies."]
    #[inline(always)]
    pub fn buck_out2_hw_sel(&self) -> BUCK_OUT2_HW_SEL_R {
        BUCK_OUT2_HW_SEL_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Enable for vccbuck2 output. The value in this register is ignored unless PWR_BUCK_CTL.BUCK_EN==1. The regulator takes up to 600us to charge the external capacitor. If there is additional load current while charging, this will increase the startup time."]
    #[inline(always)]
    pub fn buck_out2_en(&self) -> BUCK_OUT2_EN_R {
        BUCK_OUT2_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Voltage output selection for vccbuck2 output. When increasing the voltage, it can take up to 200us for the output voltage to settle. When decreasing the voltage, the settling time depends on the load current. 0: 1.15V 1: 1.20V 2: 1.25V 3: 1.30V 4: 1.35V 5: 1.40V 6: 1.45V 7: 1.50V"]
    #[inline(always)]
    pub fn buck_out2_sel(&mut self) -> BUCK_OUT2_SEL_W {
        BUCK_OUT2_SEL_W { w: self }
    }
    #[doc = "Bit 30 - Hardware control for vccbuck2 output. When this bit is set, the value in BUCK_OUT2_EN is ignored and a hardware signal is used instead. If the product has supporting hardware, it can directly control the enable signal for vccbuck2. The same charging time in BUCK_OUT2_EN applies."]
    #[inline(always)]
    pub fn buck_out2_hw_sel(&mut self) -> BUCK_OUT2_HW_SEL_W {
        BUCK_OUT2_HW_SEL_W { w: self }
    }
    #[doc = "Bit 31 - Enable for vccbuck2 output. The value in this register is ignored unless PWR_BUCK_CTL.BUCK_EN==1. The regulator takes up to 600us to charge the external capacitor. If there is additional load current while charging, this will increase the startup time."]
    #[inline(always)]
    pub fn buck_out2_en(&mut self) -> BUCK_OUT2_EN_W {
        BUCK_OUT2_EN_W { w: self }
    }
}
