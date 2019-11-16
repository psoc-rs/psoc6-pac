#[doc = "Reader of register OA_RES1_CTRL"]
pub type R = crate::R<u32, super::OA_RES1_CTRL>;
#[doc = "Writer for register OA_RES1_CTRL"]
pub type W = crate::W<u32, super::OA_RES1_CTRL>;
#[doc = "Register OA_RES1_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::OA_RES1_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OA1_PWR_MODE`"]
pub type OA1_PWR_MODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OA1_PWR_MODE`"]
pub struct OA1_PWR_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> OA1_PWR_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `OA1_DRIVE_STR_SEL`"]
pub type OA1_DRIVE_STR_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OA1_DRIVE_STR_SEL`"]
pub struct OA1_DRIVE_STR_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> OA1_DRIVE_STR_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `OA1_COMP_EN`"]
pub type OA1_COMP_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OA1_COMP_EN`"]
pub struct OA1_COMP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> OA1_COMP_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `OA1_HYST_EN`"]
pub type OA1_HYST_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OA1_HYST_EN`"]
pub struct OA1_HYST_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> OA1_HYST_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `OA1_BYPASS_DSI_SYNC`"]
pub type OA1_BYPASS_DSI_SYNC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OA1_BYPASS_DSI_SYNC`"]
pub struct OA1_BYPASS_DSI_SYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> OA1_BYPASS_DSI_SYNC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `OA1_DSI_LEVEL`"]
pub type OA1_DSI_LEVEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OA1_DSI_LEVEL`"]
pub struct OA1_DSI_LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> OA1_DSI_LEVEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Opamp1 comparator edge detect for interrupt and pulse mode of DSI (trigger)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OA1_COMPINT_A {
    #[doc = "0: Disabled, no interrupts will be detected"]
    DISABLE = 0,
    #[doc = "1: Rising edge"]
    RISING = 1,
    #[doc = "2: Falling edge"]
    FALLING = 2,
    #[doc = "3: Both rising and falling edges"]
    BOTH = 3,
}
impl From<OA1_COMPINT_A> for u8 {
    #[inline(always)]
    fn from(variant: OA1_COMPINT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `OA1_COMPINT`"]
pub type OA1_COMPINT_R = crate::R<u8, OA1_COMPINT_A>;
impl OA1_COMPINT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OA1_COMPINT_A {
        match self.bits {
            0 => OA1_COMPINT_A::DISABLE,
            1 => OA1_COMPINT_A::RISING,
            2 => OA1_COMPINT_A::FALLING,
            3 => OA1_COMPINT_A::BOTH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == OA1_COMPINT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == OA1_COMPINT_A::RISING
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == OA1_COMPINT_A::FALLING
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == OA1_COMPINT_A::BOTH
    }
}
#[doc = "Write proxy for field `OA1_COMPINT`"]
pub struct OA1_COMPINT_W<'a> {
    w: &'a mut W,
}
impl<'a> OA1_COMPINT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OA1_COMPINT_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Disabled, no interrupts will be detected"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(OA1_COMPINT_A::DISABLE)
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(OA1_COMPINT_A::RISING)
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(OA1_COMPINT_A::FALLING)
    }
    #[doc = "Both rising and falling edges"]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(OA1_COMPINT_A::BOTH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `OA1_PUMP_EN`"]
pub type OA1_PUMP_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OA1_PUMP_EN`"]
pub struct OA1_PUMP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> OA1_PUMP_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `OA1_BOOST_EN`"]
pub type OA1_BOOST_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OA1_BOOST_EN`"]
pub struct OA1_BOOST_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> OA1_BOOST_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Opamp1 power level: see description of OA0_PWR_MODE"]
    #[inline(always)]
    pub fn oa1_pwr_mode(&self) -> OA1_PWR_MODE_R {
        OA1_PWR_MODE_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 3 - Opamp1 output strength select 0=1x, 1=10x This setting sets specific requirements for OA1_BOOST_EN and OA1_COMP_TRIM"]
    #[inline(always)]
    pub fn oa1_drive_str_sel(&self) -> OA1_DRIVE_STR_SEL_R {
        OA1_DRIVE_STR_SEL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Opamp1 comparator enable"]
    #[inline(always)]
    pub fn oa1_comp_en(&self) -> OA1_COMP_EN_R {
        OA1_COMP_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Opamp1 hysteresis enable (10mV)"]
    #[inline(always)]
    pub fn oa1_hyst_en(&self) -> OA1_HYST_EN_R {
        OA1_HYST_EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Opamp1 bypass comparator output synchronization for DSI output: 0=synchronize, 1=bypass"]
    #[inline(always)]
    pub fn oa1_bypass_dsi_sync(&self) -> OA1_BYPASS_DSI_SYNC_R {
        OA1_BYPASS_DSI_SYNC_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Opamp1 comparator DSI (trigger) out level : 0=pulse, each time an edge is detected (see OA1_COMPINT) a pulse is sent out on DSI 1=level, DSI output is a synchronized version of the comparator output"]
    #[inline(always)]
    pub fn oa1_dsi_level(&self) -> OA1_DSI_LEVEL_R {
        OA1_DSI_LEVEL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Opamp1 comparator edge detect for interrupt and pulse mode of DSI (trigger)"]
    #[inline(always)]
    pub fn oa1_compint(&self) -> OA1_COMPINT_R {
        OA1_COMPINT_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 11 - Opamp1 pump enable"]
    #[inline(always)]
    pub fn oa1_pump_en(&self) -> OA1_PUMP_EN_R {
        OA1_PUMP_EN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Opamp1 gain booster enable for class A output, for risk mitigation only, not user selectable. Value depends on the drive strength setting - 1x mode: set to 1; 10x mode: set to 0"]
    #[inline(always)]
    pub fn oa1_boost_en(&self) -> OA1_BOOST_EN_R {
        OA1_BOOST_EN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Opamp1 power level: see description of OA0_PWR_MODE"]
    #[inline(always)]
    pub fn oa1_pwr_mode(&mut self) -> OA1_PWR_MODE_W {
        OA1_PWR_MODE_W { w: self }
    }
    #[doc = "Bit 3 - Opamp1 output strength select 0=1x, 1=10x This setting sets specific requirements for OA1_BOOST_EN and OA1_COMP_TRIM"]
    #[inline(always)]
    pub fn oa1_drive_str_sel(&mut self) -> OA1_DRIVE_STR_SEL_W {
        OA1_DRIVE_STR_SEL_W { w: self }
    }
    #[doc = "Bit 4 - Opamp1 comparator enable"]
    #[inline(always)]
    pub fn oa1_comp_en(&mut self) -> OA1_COMP_EN_W {
        OA1_COMP_EN_W { w: self }
    }
    #[doc = "Bit 5 - Opamp1 hysteresis enable (10mV)"]
    #[inline(always)]
    pub fn oa1_hyst_en(&mut self) -> OA1_HYST_EN_W {
        OA1_HYST_EN_W { w: self }
    }
    #[doc = "Bit 6 - Opamp1 bypass comparator output synchronization for DSI output: 0=synchronize, 1=bypass"]
    #[inline(always)]
    pub fn oa1_bypass_dsi_sync(&mut self) -> OA1_BYPASS_DSI_SYNC_W {
        OA1_BYPASS_DSI_SYNC_W { w: self }
    }
    #[doc = "Bit 7 - Opamp1 comparator DSI (trigger) out level : 0=pulse, each time an edge is detected (see OA1_COMPINT) a pulse is sent out on DSI 1=level, DSI output is a synchronized version of the comparator output"]
    #[inline(always)]
    pub fn oa1_dsi_level(&mut self) -> OA1_DSI_LEVEL_W {
        OA1_DSI_LEVEL_W { w: self }
    }
    #[doc = "Bits 8:9 - Opamp1 comparator edge detect for interrupt and pulse mode of DSI (trigger)"]
    #[inline(always)]
    pub fn oa1_compint(&mut self) -> OA1_COMPINT_W {
        OA1_COMPINT_W { w: self }
    }
    #[doc = "Bit 11 - Opamp1 pump enable"]
    #[inline(always)]
    pub fn oa1_pump_en(&mut self) -> OA1_PUMP_EN_W {
        OA1_PUMP_EN_W { w: self }
    }
    #[doc = "Bit 12 - Opamp1 gain booster enable for class A output, for risk mitigation only, not user selectable. Value depends on the drive strength setting - 1x mode: set to 1; 10x mode: set to 0"]
    #[inline(always)]
    pub fn oa1_boost_en(&mut self) -> OA1_BOOST_EN_W {
        OA1_BOOST_EN_W { w: self }
    }
}
