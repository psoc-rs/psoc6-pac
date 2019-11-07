#[doc = "Reader of register PWR_TRIM_PWRSYS_CTL"]
pub type R = crate::R<u32, super::PWR_TRIM_PWRSYS_CTL>;
#[doc = "Writer for register PWR_TRIM_PWRSYS_CTL"]
pub type W = crate::W<u32, super::PWR_TRIM_PWRSYS_CTL>;
#[doc = "Register PWR_TRIM_PWRSYS_CTL `reset()`'s with value 0x17"]
impl crate::ResetValue for super::PWR_TRIM_PWRSYS_CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x17
    }
}
#[doc = "Reader of field `ACT_REG_TRIM`"]
pub type ACT_REG_TRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ACT_REG_TRIM`"]
pub struct ACT_REG_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> ACT_REG_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Reader of field `ACT_REG_BOOST`"]
pub type ACT_REG_BOOST_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ACT_REG_BOOST`"]
pub struct ACT_REG_BOOST_W<'a> {
    w: &'a mut W,
}
impl<'a> ACT_REG_BOOST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Trim for the Active-Regulator. This sets the output voltage level. This register is only reset by XRES/POR/BOD/OVP/HIBERNATE. The nominal output voltage is vccd=812.5mV + ACT_REG_TRIM*12.5mV. The actual output voltage will vary depending on conditions and load. The following settings are explicitly shown for convenience, and other values may be calculated using the formula: 5'h07: 900mV (nominal) 5'h17: 1100mV (nominal)"]
    #[inline(always)]
    pub fn act_reg_trim(&self) -> ACT_REG_TRIM_R {
        ACT_REG_TRIM_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 30:31 - Controls the tradeoff between output current and internal operating current for the Active Regulator. The maximum output current depends on the silicon implementation, but an application may limit its maximum current to less than that. This may allow a reduction in the internal operating current of the regulator. The regulator internal operating current depends on the boost setting: 2'b00: 50uA 2'b01: 100uA 2'b10: 150uA 2'b11: 200uA The allowed setting is a lookup table based on the chip-specific maximum (set in factory) and an application-specific maximum (set by customer). The defaults are set assuming the application consumes the maximum allowed by the chip. 50mA chip: 2'b00 (default); 100mA chip: 2'b00 (default); 150mA chip: 50..100mA app => 2'b00, 150mA app => 2'b01 (default); 200mA chip: 50mA app => 2'b00, 100..150mA app => 2'b01, 200mA app => 2'b10 (default); 250mA chip: 50mA app => 2'b00, 100..150mA app => 2'b01, 200..250mA app => 2'b10 (default); 300mA chip: 50mA app => 2'b00, 100..150mA app => 2'b01, 200..250mA app => 2'b10, 300mA app => 2'b11 (default); This register is only reset by XRES/POR/BOD/OVP/HIBERNATE."]
    #[inline(always)]
    pub fn act_reg_boost(&self) -> ACT_REG_BOOST_R {
        ACT_REG_BOOST_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Trim for the Active-Regulator. This sets the output voltage level. This register is only reset by XRES/POR/BOD/OVP/HIBERNATE. The nominal output voltage is vccd=812.5mV + ACT_REG_TRIM*12.5mV. The actual output voltage will vary depending on conditions and load. The following settings are explicitly shown for convenience, and other values may be calculated using the formula: 5'h07: 900mV (nominal) 5'h17: 1100mV (nominal)"]
    #[inline(always)]
    pub fn act_reg_trim(&mut self) -> ACT_REG_TRIM_W {
        ACT_REG_TRIM_W { w: self }
    }
    #[doc = "Bits 30:31 - Controls the tradeoff between output current and internal operating current for the Active Regulator. The maximum output current depends on the silicon implementation, but an application may limit its maximum current to less than that. This may allow a reduction in the internal operating current of the regulator. The regulator internal operating current depends on the boost setting: 2'b00: 50uA 2'b01: 100uA 2'b10: 150uA 2'b11: 200uA The allowed setting is a lookup table based on the chip-specific maximum (set in factory) and an application-specific maximum (set by customer). The defaults are set assuming the application consumes the maximum allowed by the chip. 50mA chip: 2'b00 (default); 100mA chip: 2'b00 (default); 150mA chip: 50..100mA app => 2'b00, 150mA app => 2'b01 (default); 200mA chip: 50mA app => 2'b00, 100..150mA app => 2'b01, 200mA app => 2'b10 (default); 250mA chip: 50mA app => 2'b00, 100..150mA app => 2'b01, 200..250mA app => 2'b10 (default); 300mA chip: 50mA app => 2'b00, 100..150mA app => 2'b01, 200..250mA app => 2'b10, 300mA app => 2'b11 (default); This register is only reset by XRES/POR/BOD/OVP/HIBERNATE."]
    #[inline(always)]
    pub fn act_reg_boost(&mut self) -> ACT_REG_BOOST_W {
        ACT_REG_BOOST_W { w: self }
    }
}
