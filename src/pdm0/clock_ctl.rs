#[doc = "Reader of register CLOCK_CTL"]
pub type R = crate::R<u32, super::CLOCK_CTL>;
#[doc = "Writer for register CLOCK_CTL"]
pub type W = crate::W<u32, super::CLOCK_CTL>;
#[doc = "Register CLOCK_CTL `reset()`'s with value 0x0020_0310"]
impl crate::ResetValue for super::CLOCK_CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0020_0310
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
#[doc = "Reader of field `CLK_CLOCK_DIV`"]
pub type CLK_CLOCK_DIV_R = crate::R<u8, CLK_CLOCK_DIV_A>;
impl CLK_CLOCK_DIV_R {
    #[doc = r"Get enumerated values variant"]
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
#[doc = "Write proxy for field `CLK_CLOCK_DIV`"]
pub struct CLK_CLOCK_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_CLOCK_DIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLK_CLOCK_DIV_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
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
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
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
#[doc = "Reader of field `MCLKQ_CLOCK_DIV`"]
pub type MCLKQ_CLOCK_DIV_R = crate::R<u8, MCLKQ_CLOCK_DIV_A>;
impl MCLKQ_CLOCK_DIV_R {
    #[doc = r"Get enumerated values variant"]
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
#[doc = "Write proxy for field `MCLKQ_CLOCK_DIV`"]
pub struct MCLKQ_CLOCK_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> MCLKQ_CLOCK_DIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MCLKQ_CLOCK_DIV_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
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
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `CKO_CLOCK_DIV`"]
pub type CKO_CLOCK_DIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CKO_CLOCK_DIV`"]
pub struct CKO_CLOCK_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> CKO_CLOCK_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `SINC_RATE`"]
pub type SINC_RATE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SINC_RATE`"]
pub struct SINC_RATE_W<'a> {
    w: &'a mut W,
}
impl<'a> SINC_RATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | (((value as u32) & 0x7f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - PDM CLK (FPDM_CLK) (1st divider): This configures a frequency of PDM CLK. The configured frequency is used to operate PDM core. I.e. the frequency is input to MCLKQ_CLOCK_DIV register. Note: configure a frequency of PDM CLK as lower than or equal 50MHz with this divider."]
    #[inline(always)]
    pub fn clk_clock_div(&self) -> CLK_CLOCK_DIV_R {
        CLK_CLOCK_DIV_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - MCLKQ divider (2nd divider) (Note: These bits are connected to AR36U12.PDM_CORE2_CFG.DIV_MCLKQ)"]
    #[inline(always)]
    pub fn mclkq_clock_div(&self) -> MCLKQ_CLOCK_DIV_R {
        MCLKQ_CLOCK_DIV_R::new(((self.bits >> 4) & 0x03) as u8)
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
    pub fn clk_clock_div(&mut self) -> CLK_CLOCK_DIV_W {
        CLK_CLOCK_DIV_W { w: self }
    }
    #[doc = "Bits 4:5 - MCLKQ divider (2nd divider) (Note: These bits are connected to AR36U12.PDM_CORE2_CFG.DIV_MCLKQ)"]
    #[inline(always)]
    pub fn mclkq_clock_div(&mut self) -> MCLKQ_CLOCK_DIV_W {
        MCLKQ_CLOCK_DIV_W { w: self }
    }
    #[doc = "Bits 8:11 - PDM CKO (FPDM_CKO) clock divider (3rd divider): FPDM_CKO = MCLKQ / (CKO_CLOCK_DIV + 1) Note: To configure '0' to this field is prohibited. (Note: PDM_CKO is configured by MCLKQ_CLOCK_DIV, CLK_CLOCK_DIV and CKO_CLOCK_DIV. ) (Note: These bits are connected to AR36U12.PDM_CORE_CFG.MCLKDIV)"]
    #[inline(always)]
    pub fn cko_clock_div(&mut self) -> CKO_CLOCK_DIV_W {
        CKO_CLOCK_DIV_W { w: self }
    }
    #[doc = "Bits 16:22 - SINC Decimation Rate. For details, see the data sheet provided by Archband. Oversampling Ratio = Decimation Rate = 2 X SINC_RATE (Note: These bits are connected to AR36U12.PDM_CORE_CFG.SINC_RATE)"]
    #[inline(always)]
    pub fn sinc_rate(&mut self) -> SINC_RATE_W {
        SINC_RATE_W { w: self }
    }
}
