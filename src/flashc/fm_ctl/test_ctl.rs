#[doc = "Register `TEST_CTL` reader"]
pub struct R(crate::R<TEST_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TEST_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TEST_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TEST_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TEST_CTL` writer"]
pub struct W(crate::W<TEST_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TEST_CTL_SPEC>;
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
impl From<crate::W<TEST_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TEST_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TEST_MODE` reader - Test mode control: '0'-'31': TBD"]
pub type TEST_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TEST_MODE` writer - Test mode control: '0'-'31': TBD"]
pub type TEST_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TEST_CTL_SPEC, u8, u8, 5, O>;
#[doc = "Field `PN_CTL` reader - Positive/negative margin mode control: '0': negative margin control '1': positive margin control"]
pub type PN_CTL_R = crate::BitReader<bool>;
#[doc = "Field `PN_CTL` writer - Positive/negative margin mode control: '0': negative margin control '1': positive margin control"]
pub type PN_CTL_W<'a, const O: u8> = crate::BitWriter<'a, u32, TEST_CTL_SPEC, bool, O>;
#[doc = "Field `TM_PE` reader - PUMP_EN override: Pump Enable =PUMP_EN | PE_TM"]
pub type TM_PE_R = crate::BitReader<bool>;
#[doc = "Field `TM_PE` writer - PUMP_EN override: Pump Enable =PUMP_EN | PE_TM"]
pub type TM_PE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TEST_CTL_SPEC, bool, O>;
#[doc = "Field `TM_DISPOS` reader - Test mode positive pump disable"]
pub type TM_DISPOS_R = crate::BitReader<bool>;
#[doc = "Field `TM_DISPOS` writer - Test mode positive pump disable"]
pub type TM_DISPOS_W<'a, const O: u8> = crate::BitWriter<'a, u32, TEST_CTL_SPEC, bool, O>;
#[doc = "Field `TM_DISNEG` reader - Test mode negative pump disable"]
pub type TM_DISNEG_R = crate::BitReader<bool>;
#[doc = "Field `TM_DISNEG` writer - Test mode negative pump disable"]
pub type TM_DISNEG_W<'a, const O: u8> = crate::BitWriter<'a, u32, TEST_CTL_SPEC, bool, O>;
#[doc = "Field `EN_CLK_MON` reader - 1: enables the oscillator output monitor"]
pub type EN_CLK_MON_R = crate::BitReader<bool>;
#[doc = "Field `EN_CLK_MON` writer - 1: enables the oscillator output monitor"]
pub type EN_CLK_MON_W<'a, const O: u8> = crate::BitWriter<'a, u32, TEST_CTL_SPEC, bool, O>;
#[doc = "Field `CSL_DEBUG` reader - Engineering Debug Register"]
pub type CSL_DEBUG_R = crate::BitReader<bool>;
#[doc = "Field `CSL_DEBUG` writer - Engineering Debug Register"]
pub type CSL_DEBUG_W<'a, const O: u8> = crate::BitWriter<'a, u32, TEST_CTL_SPEC, bool, O>;
#[doc = "Field `ENABLE_OSC` reader - 0': the oscillator enable logic has control over the internal oscillator '1': forces oscillator enable HI"]
pub type ENABLE_OSC_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE_OSC` writer - 0': the oscillator enable logic has control over the internal oscillator '1': forces oscillator enable HI"]
pub type ENABLE_OSC_W<'a, const O: u8> = crate::BitWriter<'a, u32, TEST_CTL_SPEC, bool, O>;
#[doc = "Field `UNSCRAMBLE_WA` reader - See BSN-242 memo '0': normal '1': disables the Word Address scrambling"]
pub type UNSCRAMBLE_WA_R = crate::BitReader<bool>;
#[doc = "Field `UNSCRAMBLE_WA` writer - See BSN-242 memo '0': normal '1': disables the Word Address scrambling"]
pub type UNSCRAMBLE_WA_W<'a, const O: u8> = crate::BitWriter<'a, u32, TEST_CTL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:4 - Test mode control: '0'-'31': TBD"]
    #[inline(always)]
    pub fn test_mode(&self) -> TEST_MODE_R {
        TEST_MODE_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 8 - Positive/negative margin mode control: '0': negative margin control '1': positive margin control"]
    #[inline(always)]
    pub fn pn_ctl(&self) -> PN_CTL_R {
        PN_CTL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PUMP_EN override: Pump Enable =PUMP_EN | PE_TM"]
    #[inline(always)]
    pub fn tm_pe(&self) -> TM_PE_R {
        TM_PE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Test mode positive pump disable"]
    #[inline(always)]
    pub fn tm_dispos(&self) -> TM_DISPOS_R {
        TM_DISPOS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Test mode negative pump disable"]
    #[inline(always)]
    pub fn tm_disneg(&self) -> TM_DISNEG_R {
        TM_DISNEG_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - 1: enables the oscillator output monitor"]
    #[inline(always)]
    pub fn en_clk_mon(&self) -> EN_CLK_MON_R {
        EN_CLK_MON_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Engineering Debug Register"]
    #[inline(always)]
    pub fn csl_debug(&self) -> CSL_DEBUG_R {
        CSL_DEBUG_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 0': the oscillator enable logic has control over the internal oscillator '1': forces oscillator enable HI"]
    #[inline(always)]
    pub fn enable_osc(&self) -> ENABLE_OSC_R {
        ENABLE_OSC_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 31 - See BSN-242 memo '0': normal '1': disables the Word Address scrambling"]
    #[inline(always)]
    pub fn unscramble_wa(&self) -> UNSCRAMBLE_WA_R {
        UNSCRAMBLE_WA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Test mode control: '0'-'31': TBD"]
    #[inline(always)]
    pub fn test_mode(&mut self) -> TEST_MODE_W<0> {
        TEST_MODE_W::new(self)
    }
    #[doc = "Bit 8 - Positive/negative margin mode control: '0': negative margin control '1': positive margin control"]
    #[inline(always)]
    pub fn pn_ctl(&mut self) -> PN_CTL_W<8> {
        PN_CTL_W::new(self)
    }
    #[doc = "Bit 9 - PUMP_EN override: Pump Enable =PUMP_EN | PE_TM"]
    #[inline(always)]
    pub fn tm_pe(&mut self) -> TM_PE_W<9> {
        TM_PE_W::new(self)
    }
    #[doc = "Bit 10 - Test mode positive pump disable"]
    #[inline(always)]
    pub fn tm_dispos(&mut self) -> TM_DISPOS_W<10> {
        TM_DISPOS_W::new(self)
    }
    #[doc = "Bit 11 - Test mode negative pump disable"]
    #[inline(always)]
    pub fn tm_disneg(&mut self) -> TM_DISNEG_W<11> {
        TM_DISNEG_W::new(self)
    }
    #[doc = "Bit 16 - 1: enables the oscillator output monitor"]
    #[inline(always)]
    pub fn en_clk_mon(&mut self) -> EN_CLK_MON_W<16> {
        EN_CLK_MON_W::new(self)
    }
    #[doc = "Bit 17 - Engineering Debug Register"]
    #[inline(always)]
    pub fn csl_debug(&mut self) -> CSL_DEBUG_W<17> {
        CSL_DEBUG_W::new(self)
    }
    #[doc = "Bit 18 - 0': the oscillator enable logic has control over the internal oscillator '1': forces oscillator enable HI"]
    #[inline(always)]
    pub fn enable_osc(&mut self) -> ENABLE_OSC_W<18> {
        ENABLE_OSC_W::new(self)
    }
    #[doc = "Bit 31 - See BSN-242 memo '0': normal '1': disables the Word Address scrambling"]
    #[inline(always)]
    pub fn unscramble_wa(&mut self) -> UNSCRAMBLE_WA_W<31> {
        UNSCRAMBLE_WA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Test mode control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [test_ctl](index.html) module"]
pub struct TEST_CTL_SPEC;
impl crate::RegisterSpec for TEST_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [test_ctl::R](R) reader structure"]
impl crate::Readable for TEST_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [test_ctl::W](W) writer structure"]
impl crate::Writable for TEST_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TEST_CTL to value 0"]
impl crate::Resettable for TEST_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
