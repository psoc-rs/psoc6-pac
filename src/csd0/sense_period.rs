#[doc = "Register `SENSE_PERIOD` reader"]
pub struct R(crate::R<SENSE_PERIOD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SENSE_PERIOD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SENSE_PERIOD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SENSE_PERIOD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SENSE_PERIOD` writer"]
pub struct W(crate::W<SENSE_PERIOD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SENSE_PERIOD_SPEC>;
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
impl From<crate::W<SENSE_PERIOD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SENSE_PERIOD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SENSE_DIV` reader - The length-1 of the Sense modulation 'clock' period in clk_csd cycles. For regular CSD one sense clock cycle = one conversion (=phi1+phi2) . Note this is the base divider, clock dithering may change the actual period length. Note that SENSE_DIV must be at least 1 and additionally also allow for one clk_hf of non overlap (if OVERLAP_HI1/2 is set) on both phases, i.e. if clk_csd=clk_hf then SENSE_DIV must be >=3. In addition the FILTER_DELAY needs to be added to the minimum allowed SENSE_DIV value."]
pub type SENSE_DIV_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SENSE_DIV` writer - The length-1 of the Sense modulation 'clock' period in clk_csd cycles. For regular CSD one sense clock cycle = one conversion (=phi1+phi2) . Note this is the base divider, clock dithering may change the actual period length. Note that SENSE_DIV must be at least 1 and additionally also allow for one clk_hf of non overlap (if OVERLAP_HI1/2 is set) on both phases, i.e. if clk_csd=clk_hf then SENSE_DIV must be >=3. In addition the FILTER_DELAY needs to be added to the minimum allowed SENSE_DIV value."]
pub type SENSE_DIV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SENSE_PERIOD_SPEC, u16, u16, 12, O>;
#[doc = "Selects the length of the LFSR which determines the LFSR repeat period. LFSR_BITS LSB of the LFSR are used for the clock dithering variation on the base period (was PRS in CSDv1). Whenever the LFSR is used (non zero value in this field) the LFSR_CLEAR bit should also be set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LFSR_SIZE_A {
    #[doc = "0: Don't use clock dithering (=spreadspectrum) (LFSR output value is zero)"]
    OFF = 0,
    #[doc = "1: 6-bit LFSR (G(x)=X^6 +X^4+X^3+ X+1, period= 63)"]
    _6B = 1,
    #[doc = "2: 7-bit LFSR (G(x)=X^7 +X^4+X^3+X^2+1, period= 127)"]
    _7B = 2,
    #[doc = "3: 9-bit LFSR (G(x)=X^9 +X^4+X^3+ X+1, period= 511)"]
    _9B = 3,
    #[doc = "4: 10-bit LFSR (G(x)=X^10+X^4+X^3+ X+1, period= 1023)"]
    _10B = 4,
    #[doc = "5: 8-bit LFSR (G(x)=X^8+X^4+X^3+X^2+1, period= 255)"]
    _8B = 5,
    #[doc = "6: 12-bit LFSR (G(x)=X^12+X^7+X^4+X^3+1, period= 4095)"]
    _12B = 6,
}
impl From<LFSR_SIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: LFSR_SIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `LFSR_SIZE` reader - Selects the length of the LFSR which determines the LFSR repeat period. LFSR_BITS LSB of the LFSR are used for the clock dithering variation on the base period (was PRS in CSDv1). Whenever the LFSR is used (non zero value in this field) the LFSR_CLEAR bit should also be set."]
pub type LFSR_SIZE_R = crate::FieldReader<u8, LFSR_SIZE_A>;
impl LFSR_SIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LFSR_SIZE_A> {
        match self.bits {
            0 => Some(LFSR_SIZE_A::OFF),
            1 => Some(LFSR_SIZE_A::_6B),
            2 => Some(LFSR_SIZE_A::_7B),
            3 => Some(LFSR_SIZE_A::_9B),
            4 => Some(LFSR_SIZE_A::_10B),
            5 => Some(LFSR_SIZE_A::_8B),
            6 => Some(LFSR_SIZE_A::_12B),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == LFSR_SIZE_A::OFF
    }
    #[doc = "Checks if the value of the field is `_6B`"]
    #[inline(always)]
    pub fn is_6b(&self) -> bool {
        *self == LFSR_SIZE_A::_6B
    }
    #[doc = "Checks if the value of the field is `_7B`"]
    #[inline(always)]
    pub fn is_7b(&self) -> bool {
        *self == LFSR_SIZE_A::_7B
    }
    #[doc = "Checks if the value of the field is `_9B`"]
    #[inline(always)]
    pub fn is_9b(&self) -> bool {
        *self == LFSR_SIZE_A::_9B
    }
    #[doc = "Checks if the value of the field is `_10B`"]
    #[inline(always)]
    pub fn is_10b(&self) -> bool {
        *self == LFSR_SIZE_A::_10B
    }
    #[doc = "Checks if the value of the field is `_8B`"]
    #[inline(always)]
    pub fn is_8b(&self) -> bool {
        *self == LFSR_SIZE_A::_8B
    }
    #[doc = "Checks if the value of the field is `_12B`"]
    #[inline(always)]
    pub fn is_12b(&self) -> bool {
        *self == LFSR_SIZE_A::_12B
    }
}
#[doc = "Field `LFSR_SIZE` writer - Selects the length of the LFSR which determines the LFSR repeat period. LFSR_BITS LSB of the LFSR are used for the clock dithering variation on the base period (was PRS in CSDv1). Whenever the LFSR is used (non zero value in this field) the LFSR_CLEAR bit should also be set."]
pub type LFSR_SIZE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SENSE_PERIOD_SPEC, u8, LFSR_SIZE_A, 3, O>;
impl<'a, const O: u8> LFSR_SIZE_W<'a, O> {
    #[doc = "Don't use clock dithering (=spreadspectrum) (LFSR output value is zero)"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(LFSR_SIZE_A::OFF)
    }
    #[doc = "6-bit LFSR (G(x)=X^6 +X^4+X^3+ X+1, period= 63)"]
    #[inline(always)]
    pub fn _6b(self) -> &'a mut W {
        self.variant(LFSR_SIZE_A::_6B)
    }
    #[doc = "7-bit LFSR (G(x)=X^7 +X^4+X^3+X^2+1, period= 127)"]
    #[inline(always)]
    pub fn _7b(self) -> &'a mut W {
        self.variant(LFSR_SIZE_A::_7B)
    }
    #[doc = "9-bit LFSR (G(x)=X^9 +X^4+X^3+ X+1, period= 511)"]
    #[inline(always)]
    pub fn _9b(self) -> &'a mut W {
        self.variant(LFSR_SIZE_A::_9B)
    }
    #[doc = "10-bit LFSR (G(x)=X^10+X^4+X^3+ X+1, period= 1023)"]
    #[inline(always)]
    pub fn _10b(self) -> &'a mut W {
        self.variant(LFSR_SIZE_A::_10B)
    }
    #[doc = "8-bit LFSR (G(x)=X^8+X^4+X^3+X^2+1, period= 255)"]
    #[inline(always)]
    pub fn _8b(self) -> &'a mut W {
        self.variant(LFSR_SIZE_A::_8B)
    }
    #[doc = "12-bit LFSR (G(x)=X^12+X^7+X^4+X^3+1, period= 4095)"]
    #[inline(always)]
    pub fn _12b(self) -> &'a mut W {
        self.variant(LFSR_SIZE_A::_12B)
    }
}
#[doc = "Field `LFSR_SCALE` reader - Shift the LFSR output left by LSFR_SCALE bits before adding to SENSE_DIV. This dithering is disabled when SEL_LSFR_MSB is set. The clock divider to be used = (SENSE_DIV+1) + (SEL_LSFR_MSB ? 0 : (LFSR_OUT<<LFSR_SCALE)). Note that the clock divider including the dithering term must fit in 12 bits, otherwise the result is undefined."]
pub type LFSR_SCALE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LFSR_SCALE` writer - Shift the LFSR output left by LSFR_SCALE bits before adding to SENSE_DIV. This dithering is disabled when SEL_LSFR_MSB is set. The clock divider to be used = (SENSE_DIV+1) + (SEL_LSFR_MSB ? 0 : (LFSR_OUT<<LFSR_SCALE)). Note that the clock divider including the dithering term must fit in 12 bits, otherwise the result is undefined."]
pub type LFSR_SCALE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SENSE_PERIOD_SPEC, u8, u8, 4, O>;
#[doc = "Field `LFSR_CLEAR` reader - When set, forces the LFSR to it's initial state (all ones). This bit is automatically cleared by hardware after the LFSR is cleared, which is at the next clk_csd positive edge. This bit should be set whenever this register is written and the LFSR is used. Note that the LFSR will also get reset to all ones during the AutoZero_1/2 states."]
pub type LFSR_CLEAR_R = crate::BitReader<bool>;
#[doc = "Field `LFSR_CLEAR` writer - When set, forces the LFSR to it's initial state (all ones). This bit is automatically cleared by hardware after the LFSR is cleared, which is at the next clk_csd positive edge. This bit should be set whenever this register is written and the LFSR is used. Note that the LFSR will also get reset to all ones during the AutoZero_1/2 states."]
pub type LFSR_CLEAR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SENSE_PERIOD_SPEC, bool, O>;
#[doc = "Field `SEL_LFSR_MSB` reader - Use the MSB of configured LSFR size as csd_sense signal. Intended to be used only with bit 8 or 12-bit LFSR size for CSDv1 backward compatibility (PRS). When this bit is set then clock divider dithering is disabled and SENSE_WIDTH is disabled."]
pub type SEL_LFSR_MSB_R = crate::BitReader<bool>;
#[doc = "Field `SEL_LFSR_MSB` writer - Use the MSB of configured LSFR size as csd_sense signal. Intended to be used only with bit 8 or 12-bit LFSR size for CSDv1 backward compatibility (PRS). When this bit is set then clock divider dithering is disabled and SENSE_WIDTH is disabled."]
pub type SEL_LFSR_MSB_W<'a, const O: u8> = crate::BitWriter<'a, u32, SENSE_PERIOD_SPEC, bool, O>;
#[doc = "Selects the number of LSB bits to use from the LSFR to provide the clock dithering variation on the base period. Caveat make sure that SENSE_DIV > the maximum absolute range (e.g. for 4B SENSE_DIV > 8), otherwise results are undefined.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LFSR_BITS_A {
    #[doc = "0: use 2 bits: range = \\[-2,1\\]"]
    _2B = 0,
    #[doc = "1: use 3 bits: range = \\[-4,3\\]"]
    _3B = 1,
    #[doc = "2: use 4 bits: range = \\[-8,7\\]"]
    _4B = 2,
    #[doc = "3: use 5 bits: range = \\[-16,15\\]
(default)"]
    _5B = 3,
}
impl From<LFSR_BITS_A> for u8 {
    #[inline(always)]
    fn from(variant: LFSR_BITS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `LFSR_BITS` reader - Selects the number of LSB bits to use from the LSFR to provide the clock dithering variation on the base period. Caveat make sure that SENSE_DIV > the maximum absolute range (e.g. for 4B SENSE_DIV > 8), otherwise results are undefined."]
pub type LFSR_BITS_R = crate::FieldReader<u8, LFSR_BITS_A>;
impl LFSR_BITS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LFSR_BITS_A {
        match self.bits {
            0 => LFSR_BITS_A::_2B,
            1 => LFSR_BITS_A::_3B,
            2 => LFSR_BITS_A::_4B,
            3 => LFSR_BITS_A::_5B,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_2B`"]
    #[inline(always)]
    pub fn is_2b(&self) -> bool {
        *self == LFSR_BITS_A::_2B
    }
    #[doc = "Checks if the value of the field is `_3B`"]
    #[inline(always)]
    pub fn is_3b(&self) -> bool {
        *self == LFSR_BITS_A::_3B
    }
    #[doc = "Checks if the value of the field is `_4B`"]
    #[inline(always)]
    pub fn is_4b(&self) -> bool {
        *self == LFSR_BITS_A::_4B
    }
    #[doc = "Checks if the value of the field is `_5B`"]
    #[inline(always)]
    pub fn is_5b(&self) -> bool {
        *self == LFSR_BITS_A::_5B
    }
}
#[doc = "Field `LFSR_BITS` writer - Selects the number of LSB bits to use from the LSFR to provide the clock dithering variation on the base period. Caveat make sure that SENSE_DIV > the maximum absolute range (e.g. for 4B SENSE_DIV > 8), otherwise results are undefined."]
pub type LFSR_BITS_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, SENSE_PERIOD_SPEC, u8, LFSR_BITS_A, 2, O>;
impl<'a, const O: u8> LFSR_BITS_W<'a, O> {
    #[doc = "use 2 bits: range = \\[-2,1\\]"]
    #[inline(always)]
    pub fn _2b(self) -> &'a mut W {
        self.variant(LFSR_BITS_A::_2B)
    }
    #[doc = "use 3 bits: range = \\[-4,3\\]"]
    #[inline(always)]
    pub fn _3b(self) -> &'a mut W {
        self.variant(LFSR_BITS_A::_3B)
    }
    #[doc = "use 4 bits: range = \\[-8,7\\]"]
    #[inline(always)]
    pub fn _4b(self) -> &'a mut W {
        self.variant(LFSR_BITS_A::_4B)
    }
    #[doc = "use 5 bits: range = \\[-16,15\\]
(default)"]
    #[inline(always)]
    pub fn _5b(self) -> &'a mut W {
        self.variant(LFSR_BITS_A::_5B)
    }
}
impl R {
    #[doc = "Bits 0:11 - The length-1 of the Sense modulation 'clock' period in clk_csd cycles. For regular CSD one sense clock cycle = one conversion (=phi1+phi2) . Note this is the base divider, clock dithering may change the actual period length. Note that SENSE_DIV must be at least 1 and additionally also allow for one clk_hf of non overlap (if OVERLAP_HI1/2 is set) on both phases, i.e. if clk_csd=clk_hf then SENSE_DIV must be >=3. In addition the FILTER_DELAY needs to be added to the minimum allowed SENSE_DIV value."]
    #[inline(always)]
    pub fn sense_div(&self) -> SENSE_DIV_R {
        SENSE_DIV_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:18 - Selects the length of the LFSR which determines the LFSR repeat period. LFSR_BITS LSB of the LFSR are used for the clock dithering variation on the base period (was PRS in CSDv1). Whenever the LFSR is used (non zero value in this field) the LFSR_CLEAR bit should also be set."]
    #[inline(always)]
    pub fn lfsr_size(&self) -> LFSR_SIZE_R {
        LFSR_SIZE_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:23 - Shift the LFSR output left by LSFR_SCALE bits before adding to SENSE_DIV. This dithering is disabled when SEL_LSFR_MSB is set. The clock divider to be used = (SENSE_DIV+1) + (SEL_LSFR_MSB ? 0 : (LFSR_OUT<<LFSR_SCALE)). Note that the clock divider including the dithering term must fit in 12 bits, otherwise the result is undefined."]
    #[inline(always)]
    pub fn lfsr_scale(&self) -> LFSR_SCALE_R {
        LFSR_SCALE_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - When set, forces the LFSR to it's initial state (all ones). This bit is automatically cleared by hardware after the LFSR is cleared, which is at the next clk_csd positive edge. This bit should be set whenever this register is written and the LFSR is used. Note that the LFSR will also get reset to all ones during the AutoZero_1/2 states."]
    #[inline(always)]
    pub fn lfsr_clear(&self) -> LFSR_CLEAR_R {
        LFSR_CLEAR_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Use the MSB of configured LSFR size as csd_sense signal. Intended to be used only with bit 8 or 12-bit LFSR size for CSDv1 backward compatibility (PRS). When this bit is set then clock divider dithering is disabled and SENSE_WIDTH is disabled."]
    #[inline(always)]
    pub fn sel_lfsr_msb(&self) -> SEL_LFSR_MSB_R {
        SEL_LFSR_MSB_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:27 - Selects the number of LSB bits to use from the LSFR to provide the clock dithering variation on the base period. Caveat make sure that SENSE_DIV > the maximum absolute range (e.g. for 4B SENSE_DIV > 8), otherwise results are undefined."]
    #[inline(always)]
    pub fn lfsr_bits(&self) -> LFSR_BITS_R {
        LFSR_BITS_R::new(((self.bits >> 26) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - The length-1 of the Sense modulation 'clock' period in clk_csd cycles. For regular CSD one sense clock cycle = one conversion (=phi1+phi2) . Note this is the base divider, clock dithering may change the actual period length. Note that SENSE_DIV must be at least 1 and additionally also allow for one clk_hf of non overlap (if OVERLAP_HI1/2 is set) on both phases, i.e. if clk_csd=clk_hf then SENSE_DIV must be >=3. In addition the FILTER_DELAY needs to be added to the minimum allowed SENSE_DIV value."]
    #[inline(always)]
    pub fn sense_div(&mut self) -> SENSE_DIV_W<0> {
        SENSE_DIV_W::new(self)
    }
    #[doc = "Bits 16:18 - Selects the length of the LFSR which determines the LFSR repeat period. LFSR_BITS LSB of the LFSR are used for the clock dithering variation on the base period (was PRS in CSDv1). Whenever the LFSR is used (non zero value in this field) the LFSR_CLEAR bit should also be set."]
    #[inline(always)]
    pub fn lfsr_size(&mut self) -> LFSR_SIZE_W<16> {
        LFSR_SIZE_W::new(self)
    }
    #[doc = "Bits 20:23 - Shift the LFSR output left by LSFR_SCALE bits before adding to SENSE_DIV. This dithering is disabled when SEL_LSFR_MSB is set. The clock divider to be used = (SENSE_DIV+1) + (SEL_LSFR_MSB ? 0 : (LFSR_OUT<<LFSR_SCALE)). Note that the clock divider including the dithering term must fit in 12 bits, otherwise the result is undefined."]
    #[inline(always)]
    pub fn lfsr_scale(&mut self) -> LFSR_SCALE_W<20> {
        LFSR_SCALE_W::new(self)
    }
    #[doc = "Bit 24 - When set, forces the LFSR to it's initial state (all ones). This bit is automatically cleared by hardware after the LFSR is cleared, which is at the next clk_csd positive edge. This bit should be set whenever this register is written and the LFSR is used. Note that the LFSR will also get reset to all ones during the AutoZero_1/2 states."]
    #[inline(always)]
    pub fn lfsr_clear(&mut self) -> LFSR_CLEAR_W<24> {
        LFSR_CLEAR_W::new(self)
    }
    #[doc = "Bit 25 - Use the MSB of configured LSFR size as csd_sense signal. Intended to be used only with bit 8 or 12-bit LFSR size for CSDv1 backward compatibility (PRS). When this bit is set then clock divider dithering is disabled and SENSE_WIDTH is disabled."]
    #[inline(always)]
    pub fn sel_lfsr_msb(&mut self) -> SEL_LFSR_MSB_W<25> {
        SEL_LFSR_MSB_W::new(self)
    }
    #[doc = "Bits 26:27 - Selects the number of LSB bits to use from the LSFR to provide the clock dithering variation on the base period. Caveat make sure that SENSE_DIV > the maximum absolute range (e.g. for 4B SENSE_DIV > 8), otherwise results are undefined."]
    #[inline(always)]
    pub fn lfsr_bits(&mut self) -> LFSR_BITS_W<26> {
        LFSR_BITS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sense clock period\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sense_period](index.html) module"]
pub struct SENSE_PERIOD_SPEC;
impl crate::RegisterSpec for SENSE_PERIOD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sense_period::R](R) reader structure"]
impl crate::Readable for SENSE_PERIOD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sense_period::W](W) writer structure"]
impl crate::Writable for SENSE_PERIOD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SENSE_PERIOD to value 0x0c00_0000"]
impl crate::Resettable for SENSE_PERIOD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0c00_0000
    }
}
