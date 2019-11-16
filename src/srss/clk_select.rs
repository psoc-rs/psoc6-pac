#[doc = "Reader of register CLK_SELECT"]
pub type R = crate::R<u32, super::CLK_SELECT>;
#[doc = "Writer for register CLK_SELECT"]
pub type W = crate::W<u32, super::CLK_SELECT>;
#[doc = "Register CLK_SELECT `reset()`'s with value 0"]
impl crate::ResetValue for super::CLK_SELECT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Select source for LFCLK. Note that not all products support all clock sources. Selecting a clock source that is not supported will result in undefined behavior. Writes to this field are ignored unless the WDT is unlocked using WDT_LOCK register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LFCLK_SEL_A {
    #[doc = "0: ILO - Internal Low-speed Oscillator"]
    ILO = 0,
    #[doc = "1: WCO - Watch-Crystal Oscillator.  Requires Backup domain to be present and properly configured (including external watch crystal, if used)."]
    WCO = 1,
    #[doc = "2: ALTLF - Alternate Low-Frequency Clock.  Capability is product-specific"]
    ALTLF = 2,
    #[doc = "3: PILO - Precision ILO. If present, it works in DEEPSLEEP and higher modes.  Does not work in HIBERNATE mode."]
    PILO = 3,
}
impl From<LFCLK_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: LFCLK_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LFCLK_SEL`"]
pub type LFCLK_SEL_R = crate::R<u8, LFCLK_SEL_A>;
impl LFCLK_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LFCLK_SEL_A {
        match self.bits {
            0 => LFCLK_SEL_A::ILO,
            1 => LFCLK_SEL_A::WCO,
            2 => LFCLK_SEL_A::ALTLF,
            3 => LFCLK_SEL_A::PILO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ILO`"]
    #[inline(always)]
    pub fn is_ilo(&self) -> bool {
        *self == LFCLK_SEL_A::ILO
    }
    #[doc = "Checks if the value of the field is `WCO`"]
    #[inline(always)]
    pub fn is_wco(&self) -> bool {
        *self == LFCLK_SEL_A::WCO
    }
    #[doc = "Checks if the value of the field is `ALTLF`"]
    #[inline(always)]
    pub fn is_altlf(&self) -> bool {
        *self == LFCLK_SEL_A::ALTLF
    }
    #[doc = "Checks if the value of the field is `PILO`"]
    #[inline(always)]
    pub fn is_pilo(&self) -> bool {
        *self == LFCLK_SEL_A::PILO
    }
}
#[doc = "Write proxy for field `LFCLK_SEL`"]
pub struct LFCLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> LFCLK_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LFCLK_SEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "ILO - Internal Low-speed Oscillator"]
    #[inline(always)]
    pub fn ilo(self) -> &'a mut W {
        self.variant(LFCLK_SEL_A::ILO)
    }
    #[doc = "WCO - Watch-Crystal Oscillator. Requires Backup domain to be present and properly configured (including external watch crystal, if used)."]
    #[inline(always)]
    pub fn wco(self) -> &'a mut W {
        self.variant(LFCLK_SEL_A::WCO)
    }
    #[doc = "ALTLF - Alternate Low-Frequency Clock. Capability is product-specific"]
    #[inline(always)]
    pub fn altlf(self) -> &'a mut W {
        self.variant(LFCLK_SEL_A::ALTLF)
    }
    #[doc = "PILO - Precision ILO. If present, it works in DEEPSLEEP and higher modes. Does not work in HIBERNATE mode."]
    #[inline(always)]
    pub fn pilo(self) -> &'a mut W {
        self.variant(LFCLK_SEL_A::PILO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `PUMP_SEL`"]
pub type PUMP_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PUMP_SEL`"]
pub struct PUMP_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PUMP_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Division ratio for PUMPCLK. Uses selected PUMP_SEL clock as the source.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PUMP_DIV_A {
    #[doc = "0: Transparent mode, feed through selected clock source w/o dividing."]
    NO_DIV = 0,
    #[doc = "1: Divide selected clock source by 2"]
    DIV_BY_2 = 1,
    #[doc = "2: Divide selected clock source by 4"]
    DIV_BY_4 = 2,
    #[doc = "3: Divide selected clock source by 8"]
    DIV_BY_8 = 3,
    #[doc = "4: Divide selected clock source by 16"]
    DIV_BY_16 = 4,
}
impl From<PUMP_DIV_A> for u8 {
    #[inline(always)]
    fn from(variant: PUMP_DIV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PUMP_DIV`"]
pub type PUMP_DIV_R = crate::R<u8, PUMP_DIV_A>;
impl PUMP_DIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PUMP_DIV_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PUMP_DIV_A::NO_DIV),
            1 => Val(PUMP_DIV_A::DIV_BY_2),
            2 => Val(PUMP_DIV_A::DIV_BY_4),
            3 => Val(PUMP_DIV_A::DIV_BY_8),
            4 => Val(PUMP_DIV_A::DIV_BY_16),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NO_DIV`"]
    #[inline(always)]
    pub fn is_no_div(&self) -> bool {
        *self == PUMP_DIV_A::NO_DIV
    }
    #[doc = "Checks if the value of the field is `DIV_BY_2`"]
    #[inline(always)]
    pub fn is_div_by_2(&self) -> bool {
        *self == PUMP_DIV_A::DIV_BY_2
    }
    #[doc = "Checks if the value of the field is `DIV_BY_4`"]
    #[inline(always)]
    pub fn is_div_by_4(&self) -> bool {
        *self == PUMP_DIV_A::DIV_BY_4
    }
    #[doc = "Checks if the value of the field is `DIV_BY_8`"]
    #[inline(always)]
    pub fn is_div_by_8(&self) -> bool {
        *self == PUMP_DIV_A::DIV_BY_8
    }
    #[doc = "Checks if the value of the field is `DIV_BY_16`"]
    #[inline(always)]
    pub fn is_div_by_16(&self) -> bool {
        *self == PUMP_DIV_A::DIV_BY_16
    }
}
#[doc = "Write proxy for field `PUMP_DIV`"]
pub struct PUMP_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> PUMP_DIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PUMP_DIV_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Transparent mode, feed through selected clock source w/o dividing."]
    #[inline(always)]
    pub fn no_div(self) -> &'a mut W {
        self.variant(PUMP_DIV_A::NO_DIV)
    }
    #[doc = "Divide selected clock source by 2"]
    #[inline(always)]
    pub fn div_by_2(self) -> &'a mut W {
        self.variant(PUMP_DIV_A::DIV_BY_2)
    }
    #[doc = "Divide selected clock source by 4"]
    #[inline(always)]
    pub fn div_by_4(self) -> &'a mut W {
        self.variant(PUMP_DIV_A::DIV_BY_4)
    }
    #[doc = "Divide selected clock source by 8"]
    #[inline(always)]
    pub fn div_by_8(self) -> &'a mut W {
        self.variant(PUMP_DIV_A::DIV_BY_8)
    }
    #[doc = "Divide selected clock source by 16"]
    #[inline(always)]
    pub fn div_by_16(self) -> &'a mut W {
        self.variant(PUMP_DIV_A::DIV_BY_16)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
#[doc = "Reader of field `PUMP_ENABLE`"]
pub type PUMP_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PUMP_ENABLE`"]
pub struct PUMP_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> PUMP_ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Select source for LFCLK. Note that not all products support all clock sources. Selecting a clock source that is not supported will result in undefined behavior. Writes to this field are ignored unless the WDT is unlocked using WDT_LOCK register."]
    #[inline(always)]
    pub fn lfclk_sel(&self) -> LFCLK_SEL_R {
        LFCLK_SEL_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 8:11 - Selects clock PATH<k>, where k=PUMP_SEL. The output of this mux goes to the PUMP_DIV to make PUMPCLK Each product has a specific number of available clock paths. Selecting a path that is not implemented on a product will result in undefined behavior. Note that this is not a glitch free mux."]
    #[inline(always)]
    pub fn pump_sel(&self) -> PUMP_SEL_R {
        PUMP_SEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - Division ratio for PUMPCLK. Uses selected PUMP_SEL clock as the source."]
    #[inline(always)]
    pub fn pump_div(&self) -> PUMP_DIV_R {
        PUMP_DIV_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bit 15 - Enable the pump clock. PUMP_ENABLE and the PUMP_SEL mux are not glitch-free to minimize side-effects, avoid changing the PUMP_SEL and PUMP_DIV while changing PUMP_ENABLE. To change the settings, do the following: 1) If the pump clock is enabled, write PUMP_ENABLE=0 without changing PUMP_SEL and PUMP_DIV. 2) Change PUMP_SEL and PUMP_DIV to desired settings with PUMP_ENABLE=0. 3) Write PUMP_ENABLE=1 without changing PUMP_SEL and PUMP_DIV."]
    #[inline(always)]
    pub fn pump_enable(&self) -> PUMP_ENABLE_R {
        PUMP_ENABLE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Select source for LFCLK. Note that not all products support all clock sources. Selecting a clock source that is not supported will result in undefined behavior. Writes to this field are ignored unless the WDT is unlocked using WDT_LOCK register."]
    #[inline(always)]
    pub fn lfclk_sel(&mut self) -> LFCLK_SEL_W {
        LFCLK_SEL_W { w: self }
    }
    #[doc = "Bits 8:11 - Selects clock PATH<k>, where k=PUMP_SEL. The output of this mux goes to the PUMP_DIV to make PUMPCLK Each product has a specific number of available clock paths. Selecting a path that is not implemented on a product will result in undefined behavior. Note that this is not a glitch free mux."]
    #[inline(always)]
    pub fn pump_sel(&mut self) -> PUMP_SEL_W {
        PUMP_SEL_W { w: self }
    }
    #[doc = "Bits 12:14 - Division ratio for PUMPCLK. Uses selected PUMP_SEL clock as the source."]
    #[inline(always)]
    pub fn pump_div(&mut self) -> PUMP_DIV_W {
        PUMP_DIV_W { w: self }
    }
    #[doc = "Bit 15 - Enable the pump clock. PUMP_ENABLE and the PUMP_SEL mux are not glitch-free to minimize side-effects, avoid changing the PUMP_SEL and PUMP_DIV while changing PUMP_ENABLE. To change the settings, do the following: 1) If the pump clock is enabled, write PUMP_ENABLE=0 without changing PUMP_SEL and PUMP_DIV. 2) Change PUMP_SEL and PUMP_DIV to desired settings with PUMP_ENABLE=0. 3) Write PUMP_ENABLE=1 without changing PUMP_SEL and PUMP_DIV."]
    #[inline(always)]
    pub fn pump_enable(&mut self) -> PUMP_ENABLE_W {
        PUMP_ENABLE_W { w: self }
    }
}
