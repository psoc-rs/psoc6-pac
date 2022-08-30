#[doc = "Register `CONTROL` reader"]
pub struct R(crate::R<CONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONTROL` writer"]
pub struct W(crate::W<CONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONTROL_SPEC>;
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
impl From<crate::W<CONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LS_EN` reader - Low speed (LS) generator enable 1: enable 0: disable"]
pub type LS_EN_R = crate::BitReader<bool>;
#[doc = "Field `LS_EN` writer - Low speed (LS) generator enable 1: enable 0: disable"]
pub type LS_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL_SPEC, bool, O>;
#[doc = "Field `HS_EN` reader - High speed (HS) generator enable 1: enable 0: disable"]
pub type HS_EN_R = crate::BitReader<bool>;
#[doc = "Field `HS_EN` writer - High speed (HS) generator enable 1: enable 0: disable"]
pub type HS_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL_SPEC, bool, O>;
#[doc = "HS/LS Mode selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LCD_MODE_A {
    #[doc = "0: Select Low Speed (32kHz) Generator (Works in Active, Sleep and DeepSleep power modes)."]
    LS = 0,
    #[doc = "1: Select High Speed (system clock) Generator (Works in Active and Sleep power modes only)."]
    HS = 1,
}
impl From<LCD_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: LCD_MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCD_MODE` reader - HS/LS Mode selection"]
pub type LCD_MODE_R = crate::BitReader<LCD_MODE_A>;
impl LCD_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LCD_MODE_A {
        match self.bits {
            false => LCD_MODE_A::LS,
            true => LCD_MODE_A::HS,
        }
    }
    #[doc = "Checks if the value of the field is `LS`"]
    #[inline(always)]
    pub fn is_ls(&self) -> bool {
        *self == LCD_MODE_A::LS
    }
    #[doc = "Checks if the value of the field is `HS`"]
    #[inline(always)]
    pub fn is_hs(&self) -> bool {
        *self == LCD_MODE_A::HS
    }
}
#[doc = "Field `LCD_MODE` writer - HS/LS Mode selection"]
pub type LCD_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL_SPEC, LCD_MODE_A, O>;
impl<'a, const O: u8> LCD_MODE_W<'a, O> {
    #[doc = "Select Low Speed (32kHz) Generator (Works in Active, Sleep and DeepSleep power modes)."]
    #[inline(always)]
    pub fn ls(self) -> &'a mut W {
        self.variant(LCD_MODE_A::LS)
    }
    #[doc = "Select High Speed (system clock) Generator (Works in Active and Sleep power modes only)."]
    #[inline(always)]
    pub fn hs(self) -> &'a mut W {
        self.variant(LCD_MODE_A::HS)
    }
}
#[doc = "LCD driving waveform type configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TYPE_A {
    #[doc = "0: Type A - Each frame addresses each COM pin only once with a balanced (DC=0) waveform."]
    TYPE_A = 0,
    #[doc = "1: Type B - Each frame addresses each COM pin twice in sequence with a positive and negative waveform that together are balanced (DC=0)."]
    TYPE_B = 1,
}
impl From<TYPE_A> for bool {
    #[inline(always)]
    fn from(variant: TYPE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TYPE` reader - LCD driving waveform type configuration."]
pub type TYPE_R = crate::BitReader<TYPE_A>;
impl TYPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TYPE_A {
        match self.bits {
            false => TYPE_A::TYPE_A,
            true => TYPE_A::TYPE_B,
        }
    }
    #[doc = "Checks if the value of the field is `TYPE_A`"]
    #[inline(always)]
    pub fn is_type_a(&self) -> bool {
        *self == TYPE_A::TYPE_A
    }
    #[doc = "Checks if the value of the field is `TYPE_B`"]
    #[inline(always)]
    pub fn is_type_b(&self) -> bool {
        *self == TYPE_A::TYPE_B
    }
}
#[doc = "Field `TYPE` writer - LCD driving waveform type configuration."]
pub type TYPE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL_SPEC, TYPE_A, O>;
impl<'a, const O: u8> TYPE_W<'a, O> {
    #[doc = "Type A - Each frame addresses each COM pin only once with a balanced (DC=0) waveform."]
    #[inline(always)]
    pub fn type_a(self) -> &'a mut W {
        self.variant(TYPE_A::TYPE_A)
    }
    #[doc = "Type B - Each frame addresses each COM pin twice in sequence with a positive and negative waveform that together are balanced (DC=0)."]
    #[inline(always)]
    pub fn type_b(self) -> &'a mut W {
        self.variant(TYPE_A::TYPE_B)
    }
}
#[doc = "Driving mode configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OP_MODE_A {
    #[doc = "0: PWM Mode"]
    PWM = 0,
    #[doc = "1: Digital Correlation Mode"]
    CORRELATION = 1,
}
impl From<OP_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: OP_MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OP_MODE` reader - Driving mode configuration"]
pub type OP_MODE_R = crate::BitReader<OP_MODE_A>;
impl OP_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OP_MODE_A {
        match self.bits {
            false => OP_MODE_A::PWM,
            true => OP_MODE_A::CORRELATION,
        }
    }
    #[doc = "Checks if the value of the field is `PWM`"]
    #[inline(always)]
    pub fn is_pwm(&self) -> bool {
        *self == OP_MODE_A::PWM
    }
    #[doc = "Checks if the value of the field is `CORRELATION`"]
    #[inline(always)]
    pub fn is_correlation(&self) -> bool {
        *self == OP_MODE_A::CORRELATION
    }
}
#[doc = "Field `OP_MODE` writer - Driving mode configuration"]
pub type OP_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL_SPEC, OP_MODE_A, O>;
impl<'a, const O: u8> OP_MODE_W<'a, O> {
    #[doc = "PWM Mode"]
    #[inline(always)]
    pub fn pwm(self) -> &'a mut W {
        self.variant(OP_MODE_A::PWM)
    }
    #[doc = "Digital Correlation Mode"]
    #[inline(always)]
    pub fn correlation(self) -> &'a mut W {
        self.variant(OP_MODE_A::CORRELATION)
    }
}
#[doc = "PWM bias selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BIAS_A {
    #[doc = "0: 1/2 Bias"]
    HALF = 0,
    #[doc = "1: 1/3 Bias"]
    THIRD = 1,
    #[doc = "2: 1/4 Bias (not supported by LS generator)"]
    FOURTH = 2,
    #[doc = "3: 1/5 Bias (not supported by LS generator)"]
    FIFTH = 3,
}
impl From<BIAS_A> for u8 {
    #[inline(always)]
    fn from(variant: BIAS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `BIAS` reader - PWM bias selection"]
pub type BIAS_R = crate::FieldReader<u8, BIAS_A>;
impl BIAS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BIAS_A {
        match self.bits {
            0 => BIAS_A::HALF,
            1 => BIAS_A::THIRD,
            2 => BIAS_A::FOURTH,
            3 => BIAS_A::FIFTH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `HALF`"]
    #[inline(always)]
    pub fn is_half(&self) -> bool {
        *self == BIAS_A::HALF
    }
    #[doc = "Checks if the value of the field is `THIRD`"]
    #[inline(always)]
    pub fn is_third(&self) -> bool {
        *self == BIAS_A::THIRD
    }
    #[doc = "Checks if the value of the field is `FOURTH`"]
    #[inline(always)]
    pub fn is_fourth(&self) -> bool {
        *self == BIAS_A::FOURTH
    }
    #[doc = "Checks if the value of the field is `FIFTH`"]
    #[inline(always)]
    pub fn is_fifth(&self) -> bool {
        *self == BIAS_A::FIFTH
    }
}
#[doc = "Field `BIAS` writer - PWM bias selection"]
pub type BIAS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CONTROL_SPEC, u8, BIAS_A, 2, O>;
impl<'a, const O: u8> BIAS_W<'a, O> {
    #[doc = "1/2 Bias"]
    #[inline(always)]
    pub fn half(self) -> &'a mut W {
        self.variant(BIAS_A::HALF)
    }
    #[doc = "1/3 Bias"]
    #[inline(always)]
    pub fn third(self) -> &'a mut W {
        self.variant(BIAS_A::THIRD)
    }
    #[doc = "1/4 Bias (not supported by LS generator)"]
    #[inline(always)]
    pub fn fourth(self) -> &'a mut W {
        self.variant(BIAS_A::FOURTH)
    }
    #[doc = "1/5 Bias (not supported by LS generator)"]
    #[inline(always)]
    pub fn fifth(self) -> &'a mut W {
        self.variant(BIAS_A::FIFTH)
    }
}
#[doc = "Field `COM_NUM` reader - The number of COM connections minus 2. So: 0: 2 COM's 1: 3 COM's ... 13: 15 COM's 14: 16 COM's 15: undefined"]
pub type COM_NUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COM_NUM` writer - The number of COM connections minus 2. So: 0: 2 COM's 1: 3 COM's ... 13: 15 COM's 14: 16 COM's 15: undefined"]
pub type COM_NUM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CONTROL_SPEC, u8, u8, 4, O>;
#[doc = "Field `LS_EN_STAT` reader - LS enable status bit. This bit is a copy of LS_EN that is synchronized to the low speed clock domain and back to the system clock domain. Firmware can use this bit to observe whether LS_EN has taken effect in the low speed clock domain. Firmware should never change the configuration for the LS generator without ensuring this bit is 0. The following procedure should be followed to disable the LS generator: 1. If LS_EN=0 we are done. Exit the procedure. 2. Check that LS_EN_STAT=1. If not, wait until it is. This will catch the case of a recent enable (LS_EN=1) that has not taken effect yet. 3. Set LS_EN=0. 4. Wait until LS_EN_STAT=0."]
pub type LS_EN_STAT_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Low speed (LS) generator enable 1: enable 0: disable"]
    #[inline(always)]
    pub fn ls_en(&self) -> LS_EN_R {
        LS_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - High speed (HS) generator enable 1: enable 0: disable"]
    #[inline(always)]
    pub fn hs_en(&self) -> HS_EN_R {
        HS_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HS/LS Mode selection"]
    #[inline(always)]
    pub fn lcd_mode(&self) -> LCD_MODE_R {
        LCD_MODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LCD driving waveform type configuration."]
    #[inline(always)]
    pub fn type_(&self) -> TYPE_R {
        TYPE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Driving mode configuration"]
    #[inline(always)]
    pub fn op_mode(&self) -> OP_MODE_R {
        OP_MODE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - PWM bias selection"]
    #[inline(always)]
    pub fn bias(&self) -> BIAS_R {
        BIAS_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bits 8:11 - The number of COM connections minus 2. So: 0: 2 COM's 1: 3 COM's ... 13: 15 COM's 14: 16 COM's 15: undefined"]
    #[inline(always)]
    pub fn com_num(&self) -> COM_NUM_R {
        COM_NUM_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - LS enable status bit. This bit is a copy of LS_EN that is synchronized to the low speed clock domain and back to the system clock domain. Firmware can use this bit to observe whether LS_EN has taken effect in the low speed clock domain. Firmware should never change the configuration for the LS generator without ensuring this bit is 0. The following procedure should be followed to disable the LS generator: 1. If LS_EN=0 we are done. Exit the procedure. 2. Check that LS_EN_STAT=1. If not, wait until it is. This will catch the case of a recent enable (LS_EN=1) that has not taken effect yet. 3. Set LS_EN=0. 4. Wait until LS_EN_STAT=0."]
    #[inline(always)]
    pub fn ls_en_stat(&self) -> LS_EN_STAT_R {
        LS_EN_STAT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Low speed (LS) generator enable 1: enable 0: disable"]
    #[inline(always)]
    pub fn ls_en(&mut self) -> LS_EN_W<0> {
        LS_EN_W::new(self)
    }
    #[doc = "Bit 1 - High speed (HS) generator enable 1: enable 0: disable"]
    #[inline(always)]
    pub fn hs_en(&mut self) -> HS_EN_W<1> {
        HS_EN_W::new(self)
    }
    #[doc = "Bit 2 - HS/LS Mode selection"]
    #[inline(always)]
    pub fn lcd_mode(&mut self) -> LCD_MODE_W<2> {
        LCD_MODE_W::new(self)
    }
    #[doc = "Bit 3 - LCD driving waveform type configuration."]
    #[inline(always)]
    pub fn type_(&mut self) -> TYPE_W<3> {
        TYPE_W::new(self)
    }
    #[doc = "Bit 4 - Driving mode configuration"]
    #[inline(always)]
    pub fn op_mode(&mut self) -> OP_MODE_W<4> {
        OP_MODE_W::new(self)
    }
    #[doc = "Bits 5:6 - PWM bias selection"]
    #[inline(always)]
    pub fn bias(&mut self) -> BIAS_W<5> {
        BIAS_W::new(self)
    }
    #[doc = "Bits 8:11 - The number of COM connections minus 2. So: 0: 2 COM's 1: 3 COM's ... 13: 15 COM's 14: 16 COM's 15: undefined"]
    #[inline(always)]
    pub fn com_num(&mut self) -> COM_NUM_W<8> {
        COM_NUM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LCD Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [control](index.html) module"]
pub struct CONTROL_SPEC;
impl crate::RegisterSpec for CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [control::R](R) reader structure"]
impl crate::Readable for CONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [control::W](W) writer structure"]
impl crate::Writable for CONTROL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONTROL to value 0"]
impl crate::Resettable for CONTROL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
