#[doc = "Register `CLOCK_CTL` reader"]
pub struct R(crate::R<CLOCK_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLOCK_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLOCK_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLOCK_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLOCK_CTL` writer"]
pub struct W(crate::W<CLOCK_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLOCK_CTL_SPEC>;
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
impl From<crate::W<CLOCK_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLOCK_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "PDM CLK (FPDM_CLK) (1st divider): This configures a frequency of PDM CLK. The configured frequency is used to operate PDM core. I.e. the frequency is input to MCLKQ_CLOCK_DIV register. Note: configure a frequency of PDM CLK as lower than or equal 50MHz with this divider.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CLK_CLOCK_DIV_A {
    #[doc = "0: Divide by 1"]
    DIVBY1 = 0,
    #[doc = "1: Divide by 2 (no 50 percent duty cycle)"]
    DIVBY2 = 1,
    #[doc = "2: Divide by 3 (no 50 percent duty cycle)"]
    DIVBY3 = 2,
    #[doc = "3: Divide by 4 (no 50 percent duty cycle)"]
    DIVBY4 = 3,
}
impl From<CLK_CLOCK_DIV_A> for u8 {
    #[inline(always)]
    fn from(variant: CLK_CLOCK_DIV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CLK_CLOCK_DIV` reader - PDM CLK (FPDM_CLK) (1st divider): This configures a frequency of PDM CLK. The configured frequency is used to operate PDM core. I.e. the frequency is input to MCLKQ_CLOCK_DIV register. Note: configure a frequency of PDM CLK as lower than or equal 50MHz with this divider."]
pub type CLK_CLOCK_DIV_R = crate::FieldReader<u8, CLK_CLOCK_DIV_A>;
impl CLK_CLOCK_DIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLK_CLOCK_DIV_A {
        match self.bits {
            0 => CLK_CLOCK_DIV_A::DIVBY1,
            1 => CLK_CLOCK_DIV_A::DIVBY2,
            2 => CLK_CLOCK_DIV_A::DIVBY3,
            3 => CLK_CLOCK_DIV_A::DIVBY4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIVBY1`"]
    #[inline(always)]
    pub fn is_divby1(&self) -> bool {
        *self == CLK_CLOCK_DIV_A::DIVBY1
    }
    #[doc = "Checks if the value of the field is `DIVBY2`"]
    #[inline(always)]
    pub fn is_divby2(&self) -> bool {
        *self == CLK_CLOCK_DIV_A::DIVBY2
    }
    #[doc = "Checks if the value of the field is `DIVBY3`"]
    #[inline(always)]
    pub fn is_divby3(&self) -> bool {
        *self == CLK_CLOCK_DIV_A::DIVBY3
    }
    #[doc = "Checks if the value of the field is `DIVBY4`"]
    #[inline(always)]
    pub fn is_divby4(&self) -> bool {
        *self == CLK_CLOCK_DIV_A::DIVBY4
    }
}
#[doc = "Field `CLK_CLOCK_DIV` writer - PDM CLK (FPDM_CLK) (1st divider): This configures a frequency of PDM CLK. The configured frequency is used to operate PDM core. I.e. the frequency is input to MCLKQ_CLOCK_DIV register. Note: configure a frequency of PDM CLK as lower than or equal 50MHz with this divider."]
pub type CLK_CLOCK_DIV_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CLOCK_CTL_SPEC, u8, CLK_CLOCK_DIV_A, 2, O>;
impl<'a, const O: u8> CLK_CLOCK_DIV_W<'a, O> {
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn divby1(self) -> &'a mut W {
        self.variant(CLK_CLOCK_DIV_A::DIVBY1)
    }
    #[doc = "Divide by 2 (no 50 percent duty cycle)"]
    #[inline(always)]
    pub fn divby2(self) -> &'a mut W {
        self.variant(CLK_CLOCK_DIV_A::DIVBY2)
    }
    #[doc = "Divide by 3 (no 50 percent duty cycle)"]
    #[inline(always)]
    pub fn divby3(self) -> &'a mut W {
        self.variant(CLK_CLOCK_DIV_A::DIVBY3)
    }
    #[doc = "Divide by 4 (no 50 percent duty cycle)"]
    #[inline(always)]
    pub fn divby4(self) -> &'a mut W {
        self.variant(CLK_CLOCK_DIV_A::DIVBY4)
    }
}
#[doc = "MCLKQ divider (2nd divider) (Note: These bits are connected to AR36U12.PDM_CORE2_CFG.DIV_MCLKQ)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MCLKQ_CLOCK_DIV_A {
    #[doc = "0: Divide by 1"]
    DIVBY1 = 0,
    #[doc = "1: Divide by 2 (no 50 percent duty cycle)"]
    DIVBY2 = 1,
    #[doc = "2: Divide by 3 (no 50 percent duty cycle)"]
    DIVBY3 = 2,
    #[doc = "3: Divide by 4 (no 50 percent duty cycle)"]
    DIVBY4 = 3,
}
impl From<MCLKQ_CLOCK_DIV_A> for u8 {
    #[inline(always)]
    fn from(variant: MCLKQ_CLOCK_DIV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MCLKQ_CLOCK_DIV` reader - MCLKQ divider (2nd divider) (Note: These bits are connected to AR36U12.PDM_CORE2_CFG.DIV_MCLKQ)"]
pub type MCLKQ_CLOCK_DIV_R = crate::FieldReader<u8, MCLKQ_CLOCK_DIV_A>;
impl MCLKQ_CLOCK_DIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MCLKQ_CLOCK_DIV_A {
        match self.bits {
            0 => MCLKQ_CLOCK_DIV_A::DIVBY1,
            1 => MCLKQ_CLOCK_DIV_A::DIVBY2,
            2 => MCLKQ_CLOCK_DIV_A::DIVBY3,
            3 => MCLKQ_CLOCK_DIV_A::DIVBY4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIVBY1`"]
    #[inline(always)]
    pub fn is_divby1(&self) -> bool {
        *self == MCLKQ_CLOCK_DIV_A::DIVBY1
    }
    #[doc = "Checks if the value of the field is `DIVBY2`"]
    #[inline(always)]
    pub fn is_divby2(&self) -> bool {
        *self == MCLKQ_CLOCK_DIV_A::DIVBY2
    }
    #[doc = "Checks if the value of the field is `DIVBY3`"]
    #[inline(always)]
    pub fn is_divby3(&self) -> bool {
        *self == MCLKQ_CLOCK_DIV_A::DIVBY3
    }
    #[doc = "Checks if the value of the field is `DIVBY4`"]
    #[inline(always)]
    pub fn is_divby4(&self) -> bool {
        *self == MCLKQ_CLOCK_DIV_A::DIVBY4
    }
}
#[doc = "Field `MCLKQ_CLOCK_DIV` writer - MCLKQ divider (2nd divider) (Note: These bits are connected to AR36U12.PDM_CORE2_CFG.DIV_MCLKQ)"]
pub type MCLKQ_CLOCK_DIV_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CLOCK_CTL_SPEC, u8, MCLKQ_CLOCK_DIV_A, 2, O>;
impl<'a, const O: u8> MCLKQ_CLOCK_DIV_W<'a, O> {
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn divby1(self) -> &'a mut W {
        self.variant(MCLKQ_CLOCK_DIV_A::DIVBY1)
    }
    #[doc = "Divide by 2 (no 50 percent duty cycle)"]
    #[inline(always)]
    pub fn divby2(self) -> &'a mut W {
        self.variant(MCLKQ_CLOCK_DIV_A::DIVBY2)
    }
    #[doc = "Divide by 3 (no 50 percent duty cycle)"]
    #[inline(always)]
    pub fn divby3(self) -> &'a mut W {
        self.variant(MCLKQ_CLOCK_DIV_A::DIVBY3)
    }
    #[doc = "Divide by 4 (no 50 percent duty cycle)"]
    #[inline(always)]
    pub fn divby4(self) -> &'a mut W {
        self.variant(MCLKQ_CLOCK_DIV_A::DIVBY4)
    }
}
#[doc = "Field `CKO_CLOCK_DIV` reader - PDM CKO (FPDM_CKO) clock divider (3rd divider): FPDM_CKO = MCLKQ / (CKO_CLOCK_DIV + 1) Note: To configure '0' to this field is prohibited. (Note: PDM_CKO is configured by MCLKQ_CLOCK_DIV, CLK_CLOCK_DIV and CKO_CLOCK_DIV. ) (Note: These bits are connected to AR36U12.PDM_CORE_CFG.MCLKDIV)"]
pub type CKO_CLOCK_DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CKO_CLOCK_DIV` writer - PDM CKO (FPDM_CKO) clock divider (3rd divider): FPDM_CKO = MCLKQ / (CKO_CLOCK_DIV + 1) Note: To configure '0' to this field is prohibited. (Note: PDM_CKO is configured by MCLKQ_CLOCK_DIV, CLK_CLOCK_DIV and CKO_CLOCK_DIV. ) (Note: These bits are connected to AR36U12.PDM_CORE_CFG.MCLKDIV)"]
pub type CKO_CLOCK_DIV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLOCK_CTL_SPEC, u8, u8, 4, O>;
#[doc = "Field `SINC_RATE` reader - SINC Decimation Rate. For details, see the data sheet provided by Archband. Oversampling Ratio = Decimation Rate = 2 X SINC_RATE (Note: These bits are connected to AR36U12.PDM_CORE_CFG.SINC_RATE)"]
pub type SINC_RATE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SINC_RATE` writer - SINC Decimation Rate. For details, see the data sheet provided by Archband. Oversampling Ratio = Decimation Rate = 2 X SINC_RATE (Note: These bits are connected to AR36U12.PDM_CORE_CFG.SINC_RATE)"]
pub type SINC_RATE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLOCK_CTL_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:1 - PDM CLK (FPDM_CLK) (1st divider): This configures a frequency of PDM CLK. The configured frequency is used to operate PDM core. I.e. the frequency is input to MCLKQ_CLOCK_DIV register. Note: configure a frequency of PDM CLK as lower than or equal 50MHz with this divider."]
    #[inline(always)]
    pub fn clk_clock_div(&self) -> CLK_CLOCK_DIV_R {
        CLK_CLOCK_DIV_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - MCLKQ divider (2nd divider) (Note: These bits are connected to AR36U12.PDM_CORE2_CFG.DIV_MCLKQ)"]
    #[inline(always)]
    pub fn mclkq_clock_div(&self) -> MCLKQ_CLOCK_DIV_R {
        MCLKQ_CLOCK_DIV_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:11 - PDM CKO (FPDM_CKO) clock divider (3rd divider): FPDM_CKO = MCLKQ / (CKO_CLOCK_DIV + 1) Note: To configure '0' to this field is prohibited. (Note: PDM_CKO is configured by MCLKQ_CLOCK_DIV, CLK_CLOCK_DIV and CKO_CLOCK_DIV. ) (Note: These bits are connected to AR36U12.PDM_CORE_CFG.MCLKDIV)"]
    #[inline(always)]
    pub fn cko_clock_div(&self) -> CKO_CLOCK_DIV_R {
        CKO_CLOCK_DIV_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:22 - SINC Decimation Rate. For details, see the data sheet provided by Archband. Oversampling Ratio = Decimation Rate = 2 X SINC_RATE (Note: These bits are connected to AR36U12.PDM_CORE_CFG.SINC_RATE)"]
    #[inline(always)]
    pub fn sinc_rate(&self) -> SINC_RATE_R {
        SINC_RATE_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - PDM CLK (FPDM_CLK) (1st divider): This configures a frequency of PDM CLK. The configured frequency is used to operate PDM core. I.e. the frequency is input to MCLKQ_CLOCK_DIV register. Note: configure a frequency of PDM CLK as lower than or equal 50MHz with this divider."]
    #[inline(always)]
    pub fn clk_clock_div(&mut self) -> CLK_CLOCK_DIV_W<0> {
        CLK_CLOCK_DIV_W::new(self)
    }
    #[doc = "Bits 4:5 - MCLKQ divider (2nd divider) (Note: These bits are connected to AR36U12.PDM_CORE2_CFG.DIV_MCLKQ)"]
    #[inline(always)]
    pub fn mclkq_clock_div(&mut self) -> MCLKQ_CLOCK_DIV_W<4> {
        MCLKQ_CLOCK_DIV_W::new(self)
    }
    #[doc = "Bits 8:11 - PDM CKO (FPDM_CKO) clock divider (3rd divider): FPDM_CKO = MCLKQ / (CKO_CLOCK_DIV + 1) Note: To configure '0' to this field is prohibited. (Note: PDM_CKO is configured by MCLKQ_CLOCK_DIV, CLK_CLOCK_DIV and CKO_CLOCK_DIV. ) (Note: These bits are connected to AR36U12.PDM_CORE_CFG.MCLKDIV)"]
    #[inline(always)]
    pub fn cko_clock_div(&mut self) -> CKO_CLOCK_DIV_W<8> {
        CKO_CLOCK_DIV_W::new(self)
    }
    #[doc = "Bits 16:22 - SINC Decimation Rate. For details, see the data sheet provided by Archband. Oversampling Ratio = Decimation Rate = 2 X SINC_RATE (Note: These bits are connected to AR36U12.PDM_CORE_CFG.SINC_RATE)"]
    #[inline(always)]
    pub fn sinc_rate(&mut self) -> SINC_RATE_W<16> {
        SINC_RATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clock_ctl](index.html) module"]
pub struct CLOCK_CTL_SPEC;
impl crate::RegisterSpec for CLOCK_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clock_ctl::R](R) reader structure"]
impl crate::Readable for CLOCK_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clock_ctl::W](W) writer structure"]
impl crate::Writable for CLOCK_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLOCK_CTL to value 0x0020_0310"]
impl crate::Resettable for CLOCK_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0020_0310
    }
}
