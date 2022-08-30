#[doc = "Register `CLK_SELECT` reader"]
pub struct R(crate::R<CLK_SELECT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_SELECT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_SELECT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_SELECT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_SELECT` writer"]
pub struct W(crate::W<CLK_SELECT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_SELECT_SPEC>;
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
impl From<crate::W<CLK_SELECT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_SELECT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Select source for LFCLK. Note that not all products support all clock sources. Selecting a clock source that is not supported will result in undefined behavior. Writes to this field are ignored unless the WDT is unlocked using WDT_LOCK register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LFCLK_SEL_A {
    #[doc = "0: ILO - Internal Low-speed Oscillator"]
    ILO = 0,
    #[doc = "1: WCO - Watch-Crystal Oscillator. Requires Backup domain to be present and properly configured (including external watch crystal, if used)."]
    WCO = 1,
    #[doc = "2: ALTLF - Alternate Low-Frequency Clock. Capability is product-specific"]
    ALTLF = 2,
    #[doc = "3: PILO - Precision ILO. If present, it works in DEEPSLEEP and higher modes. Does not work in HIBERNATE mode."]
    PILO = 3,
}
impl From<LFCLK_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: LFCLK_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `LFCLK_SEL` reader - Select source for LFCLK. Note that not all products support all clock sources. Selecting a clock source that is not supported will result in undefined behavior. Writes to this field are ignored unless the WDT is unlocked using WDT_LOCK register."]
pub type LFCLK_SEL_R = crate::FieldReader<u8, LFCLK_SEL_A>;
impl LFCLK_SEL_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `LFCLK_SEL` writer - Select source for LFCLK. Note that not all products support all clock sources. Selecting a clock source that is not supported will result in undefined behavior. Writes to this field are ignored unless the WDT is unlocked using WDT_LOCK register."]
pub type LFCLK_SEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CLK_SELECT_SPEC, u8, LFCLK_SEL_A, 2, O>;
impl<'a, const O: u8> LFCLK_SEL_W<'a, O> {
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
}
#[doc = "Field `PUMP_SEL` reader - Selects clock PATH<k>, where k=PUMP_SEL. The output of this mux goes to the PUMP_DIV to make PUMPCLK Each product has a specific number of available clock paths. Selecting a path that is not implemented on a product will result in undefined behavior. Note that this is not a glitch free mux."]
pub type PUMP_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PUMP_SEL` writer - Selects clock PATH<k>, where k=PUMP_SEL. The output of this mux goes to the PUMP_DIV to make PUMPCLK Each product has a specific number of available clock paths. Selecting a path that is not implemented on a product will result in undefined behavior. Note that this is not a glitch free mux."]
pub type PUMP_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLK_SELECT_SPEC, u8, u8, 4, O>;
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
#[doc = "Field `PUMP_DIV` reader - Division ratio for PUMPCLK. Uses selected PUMP_SEL clock as the source."]
pub type PUMP_DIV_R = crate::FieldReader<u8, PUMP_DIV_A>;
impl PUMP_DIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PUMP_DIV_A> {
        match self.bits {
            0 => Some(PUMP_DIV_A::NO_DIV),
            1 => Some(PUMP_DIV_A::DIV_BY_2),
            2 => Some(PUMP_DIV_A::DIV_BY_4),
            3 => Some(PUMP_DIV_A::DIV_BY_8),
            4 => Some(PUMP_DIV_A::DIV_BY_16),
            _ => None,
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
#[doc = "Field `PUMP_DIV` writer - Division ratio for PUMPCLK. Uses selected PUMP_SEL clock as the source."]
pub type PUMP_DIV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLK_SELECT_SPEC, u8, PUMP_DIV_A, 3, O>;
impl<'a, const O: u8> PUMP_DIV_W<'a, O> {
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
}
#[doc = "Field `PUMP_ENABLE` reader - Enable the pump clock. PUMP_ENABLE and the PUMP_SEL mux are not glitch-free to minimize side-effects, avoid changing the PUMP_SEL and PUMP_DIV while changing PUMP_ENABLE. To change the settings, do the following: 1) If the pump clock is enabled, write PUMP_ENABLE=0 without changing PUMP_SEL and PUMP_DIV. 2) Change PUMP_SEL and PUMP_DIV to desired settings with PUMP_ENABLE=0. 3) Write PUMP_ENABLE=1 without changing PUMP_SEL and PUMP_DIV."]
pub type PUMP_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `PUMP_ENABLE` writer - Enable the pump clock. PUMP_ENABLE and the PUMP_SEL mux are not glitch-free to minimize side-effects, avoid changing the PUMP_SEL and PUMP_DIV while changing PUMP_ENABLE. To change the settings, do the following: 1) If the pump clock is enabled, write PUMP_ENABLE=0 without changing PUMP_SEL and PUMP_DIV. 2) Change PUMP_SEL and PUMP_DIV to desired settings with PUMP_ENABLE=0. 3) Write PUMP_ENABLE=1 without changing PUMP_SEL and PUMP_DIV."]
pub type PUMP_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_SELECT_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - Select source for LFCLK. Note that not all products support all clock sources. Selecting a clock source that is not supported will result in undefined behavior. Writes to this field are ignored unless the WDT is unlocked using WDT_LOCK register."]
    #[inline(always)]
    pub fn lfclk_sel(&self) -> LFCLK_SEL_R {
        LFCLK_SEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:11 - Selects clock PATH<k>, where k=PUMP_SEL. The output of this mux goes to the PUMP_DIV to make PUMPCLK Each product has a specific number of available clock paths. Selecting a path that is not implemented on a product will result in undefined behavior. Note that this is not a glitch free mux."]
    #[inline(always)]
    pub fn pump_sel(&self) -> PUMP_SEL_R {
        PUMP_SEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - Division ratio for PUMPCLK. Uses selected PUMP_SEL clock as the source."]
    #[inline(always)]
    pub fn pump_div(&self) -> PUMP_DIV_R {
        PUMP_DIV_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Enable the pump clock. PUMP_ENABLE and the PUMP_SEL mux are not glitch-free to minimize side-effects, avoid changing the PUMP_SEL and PUMP_DIV while changing PUMP_ENABLE. To change the settings, do the following: 1) If the pump clock is enabled, write PUMP_ENABLE=0 without changing PUMP_SEL and PUMP_DIV. 2) Change PUMP_SEL and PUMP_DIV to desired settings with PUMP_ENABLE=0. 3) Write PUMP_ENABLE=1 without changing PUMP_SEL and PUMP_DIV."]
    #[inline(always)]
    pub fn pump_enable(&self) -> PUMP_ENABLE_R {
        PUMP_ENABLE_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Select source for LFCLK. Note that not all products support all clock sources. Selecting a clock source that is not supported will result in undefined behavior. Writes to this field are ignored unless the WDT is unlocked using WDT_LOCK register."]
    #[inline(always)]
    pub fn lfclk_sel(&mut self) -> LFCLK_SEL_W<0> {
        LFCLK_SEL_W::new(self)
    }
    #[doc = "Bits 8:11 - Selects clock PATH<k>, where k=PUMP_SEL. The output of this mux goes to the PUMP_DIV to make PUMPCLK Each product has a specific number of available clock paths. Selecting a path that is not implemented on a product will result in undefined behavior. Note that this is not a glitch free mux."]
    #[inline(always)]
    pub fn pump_sel(&mut self) -> PUMP_SEL_W<8> {
        PUMP_SEL_W::new(self)
    }
    #[doc = "Bits 12:14 - Division ratio for PUMPCLK. Uses selected PUMP_SEL clock as the source."]
    #[inline(always)]
    pub fn pump_div(&mut self) -> PUMP_DIV_W<12> {
        PUMP_DIV_W::new(self)
    }
    #[doc = "Bit 15 - Enable the pump clock. PUMP_ENABLE and the PUMP_SEL mux are not glitch-free to minimize side-effects, avoid changing the PUMP_SEL and PUMP_DIV while changing PUMP_ENABLE. To change the settings, do the following: 1) If the pump clock is enabled, write PUMP_ENABLE=0 without changing PUMP_SEL and PUMP_DIV. 2) Change PUMP_SEL and PUMP_DIV to desired settings with PUMP_ENABLE=0. 3) Write PUMP_ENABLE=1 without changing PUMP_SEL and PUMP_DIV."]
    #[inline(always)]
    pub fn pump_enable(&mut self) -> PUMP_ENABLE_W<15> {
        PUMP_ENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_select](index.html) module"]
pub struct CLK_SELECT_SPEC;
impl crate::RegisterSpec for CLK_SELECT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_select::R](R) reader structure"]
impl crate::Readable for CLK_SELECT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_select::W](W) writer structure"]
impl crate::Writable for CLK_SELECT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_SELECT to value 0"]
impl crate::Resettable for CLK_SELECT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
