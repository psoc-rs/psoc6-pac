#[doc = "Reader of register ANA_CTL0"]
pub type R = crate::R<u32, super::ANA_CTL0>;
#[doc = "Writer for register ANA_CTL0"]
pub type W = crate::W<u32, super::ANA_CTL0>;
#[doc = "Register ANA_CTL0 `reset()`'s with value 0x0400"]
impl crate::ResetValue for super::ANA_CTL0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0400
    }
}
#[doc = "Reader of field `CSLDAC`"]
pub type CSLDAC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CSLDAC`"]
pub struct CSLDAC_W<'a> {
    w: &'a mut W,
}
impl<'a> CSLDAC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `VCC_SEL`"]
pub type VCC_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VCC_SEL`"]
pub struct VCC_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> VCC_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `FLIP_AMUXBUS_AB`"]
pub type FLIP_AMUXBUS_AB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FLIP_AMUXBUS_AB`"]
pub struct FLIP_AMUXBUS_AB_W<'a> {
    w: &'a mut W,
}
impl<'a> FLIP_AMUXBUS_AB_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:10 - Trimming of common source line DAC."]
    #[inline(always)]
    pub fn csldac(&self) -> CSLDAC_R {
        CSLDAC_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 24 - Vcc select: '0': 1.2 V : LP reset value '1': 0.95 V: ULP reset value Note: the flash macro compiler has a configuration option that specifies the default/reset value of this field."]
    #[inline(always)]
    pub fn vcc_sel(&self) -> VCC_SEL_R {
        VCC_SEL_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Flips amuxbusa and amuxbusb '0': amuxbusa, amuxbusb '1': amuxbusb, amuxbusb"]
    #[inline(always)]
    pub fn flip_amuxbus_ab(&self) -> FLIP_AMUXBUS_AB_R {
        FLIP_AMUXBUS_AB_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 8:10 - Trimming of common source line DAC."]
    #[inline(always)]
    pub fn csldac(&mut self) -> CSLDAC_W {
        CSLDAC_W { w: self }
    }
    #[doc = "Bit 24 - Vcc select: '0': 1.2 V : LP reset value '1': 0.95 V: ULP reset value Note: the flash macro compiler has a configuration option that specifies the default/reset value of this field."]
    #[inline(always)]
    pub fn vcc_sel(&mut self) -> VCC_SEL_W {
        VCC_SEL_W { w: self }
    }
    #[doc = "Bit 27 - Flips amuxbusa and amuxbusb '0': amuxbusa, amuxbusb '1': amuxbusb, amuxbusb"]
    #[inline(always)]
    pub fn flip_amuxbus_ab(&mut self) -> FLIP_AMUXBUS_AB_W {
        FLIP_AMUXBUS_AB_W { w: self }
    }
}
