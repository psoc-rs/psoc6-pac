#[doc = "Reader of register PWR_BUCK_CTL"]
pub type R = crate::R<u32, super::PWR_BUCK_CTL>;
#[doc = "Writer for register PWR_BUCK_CTL"]
pub type W = crate::W<u32, super::PWR_BUCK_CTL>;
#[doc = "Register PWR_BUCK_CTL `reset()`'s with value 0x05"]
impl crate::ResetValue for super::PWR_BUCK_CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x05
    }
}
#[doc = "Reader of field `BUCK_OUT1_SEL`"]
pub type BUCK_OUT1_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BUCK_OUT1_SEL`"]
pub struct BUCK_OUT1_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> BUCK_OUT1_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `BUCK_EN`"]
pub type BUCK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BUCK_EN`"]
pub struct BUCK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> BUCK_EN_W<'a> {
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
#[doc = "Reader of field `BUCK_OUT1_EN`"]
pub type BUCK_OUT1_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BUCK_OUT1_EN`"]
pub struct BUCK_OUT1_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> BUCK_OUT1_EN_W<'a> {
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
    #[doc = "Bits 0:2 - Voltage output selection for vccbuck1 output. This register is only reset by XRES/POR/BOD/OVP/HIBERNATE. When increasing the voltage, it can take up to 200us for the output voltage to settle. When decreasing the voltage, the settling time depends on the load current. 0: 0.85V 1: 0.875V 2: 0.90V 3: 0.95V 4: 1.05V 5: 1.10V 6: 1.15V 7: 1.20V"]
    #[inline(always)]
    pub fn buck_out1_sel(&self) -> BUCK_OUT1_SEL_R {
        BUCK_OUT1_SEL_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 30 - Master enable for buck converter. This register is only reset by XRES/POR/BOD/OVP/HIBERNATE."]
    #[inline(always)]
    pub fn buck_en(&self) -> BUCK_EN_R {
        BUCK_EN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Enable for vccbuck1 output. The value in this register is ignored unless PWR_BUCK_CTL.BUCK_EN==1. This register is only reset by XRES/POR/BOD/OVP/HIBERNATE. The regulator takes up to 600us to charge the external capacitor. If there is additional load current while charging, this will increase the startup time. The SAS specifies the required sequence when transitioning vccd from the LDO to SIMO Buck output #1."]
    #[inline(always)]
    pub fn buck_out1_en(&self) -> BUCK_OUT1_EN_R {
        BUCK_OUT1_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Voltage output selection for vccbuck1 output. This register is only reset by XRES/POR/BOD/OVP/HIBERNATE. When increasing the voltage, it can take up to 200us for the output voltage to settle. When decreasing the voltage, the settling time depends on the load current. 0: 0.85V 1: 0.875V 2: 0.90V 3: 0.95V 4: 1.05V 5: 1.10V 6: 1.15V 7: 1.20V"]
    #[inline(always)]
    pub fn buck_out1_sel(&mut self) -> BUCK_OUT1_SEL_W {
        BUCK_OUT1_SEL_W { w: self }
    }
    #[doc = "Bit 30 - Master enable for buck converter. This register is only reset by XRES/POR/BOD/OVP/HIBERNATE."]
    #[inline(always)]
    pub fn buck_en(&mut self) -> BUCK_EN_W {
        BUCK_EN_W { w: self }
    }
    #[doc = "Bit 31 - Enable for vccbuck1 output. The value in this register is ignored unless PWR_BUCK_CTL.BUCK_EN==1. This register is only reset by XRES/POR/BOD/OVP/HIBERNATE. The regulator takes up to 600us to charge the external capacitor. If there is additional load current while charging, this will increase the startup time. The SAS specifies the required sequence when transitioning vccd from the LDO to SIMO Buck output #1."]
    #[inline(always)]
    pub fn buck_out1_en(&mut self) -> BUCK_OUT1_EN_W {
        BUCK_OUT1_EN_W { w: self }
    }
}
