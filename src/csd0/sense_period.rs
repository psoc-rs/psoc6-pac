#[doc = "Reader of register SENSE_PERIOD"]
pub type R = crate::R<u32, super::SENSE_PERIOD>;
#[doc = "Writer for register SENSE_PERIOD"]
pub type W = crate::W<u32, super::SENSE_PERIOD>;
#[doc = "Register SENSE_PERIOD `reset()`'s with value 0x0c00_0000"]
impl crate::ResetValue for super::SENSE_PERIOD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0c00_0000
    }
}
#[doc = "Reader of field `SENSE_DIV`"]
pub type SENSE_DIV_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SENSE_DIV`"]
pub struct SENSE_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSE_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
#[doc = "Selects the length of the LFSR which determines the LFSR repeat period. LFSR_BITS LSB of the LFSR are used for the clock dithering variation on the base period (was PRS in CSDv1). Whenever the LFSR is used (non zero value in this field) the LFSR_CLEAR bit should also be set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LFSR_SIZE_A {
    #[doc = "0: Don't use clock dithering (=spreadspectrum) (LFSR output value is zero)"]
    OFF,
    #[doc = "1: 6-bit LFSR (G(x)=X^6  +X^4+X^3+    X+1, period= 63)"]
    _6B,
    #[doc = "2: 7-bit LFSR (G(x)=X^7  +X^4+X^3+X^2+1, period= 127)"]
    _7B,
    #[doc = "3: 9-bit LFSR (G(x)=X^9  +X^4+X^3+    X+1, period= 511)"]
    _9B,
    #[doc = "4: 10-bit LFSR (G(x)=X^10+X^4+X^3+    X+1, period= 1023)"]
    _10B,
    #[doc = "5: 8-bit LFSR (G(x)=X^8+X^4+X^3+X^2+1, period= 255)"]
    _8B,
    #[doc = "6: 12-bit LFSR (G(x)=X^12+X^7+X^4+X^3+1, period= 4095)"]
    _12B,
}
impl From<LFSR_SIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: LFSR_SIZE_A) -> Self {
        match variant {
            LFSR_SIZE_A::OFF => 0,
            LFSR_SIZE_A::_6B => 1,
            LFSR_SIZE_A::_7B => 2,
            LFSR_SIZE_A::_9B => 3,
            LFSR_SIZE_A::_10B => 4,
            LFSR_SIZE_A::_8B => 5,
            LFSR_SIZE_A::_12B => 6,
        }
    }
}
#[doc = "Reader of field `LFSR_SIZE`"]
pub type LFSR_SIZE_R = crate::R<u8, LFSR_SIZE_A>;
impl LFSR_SIZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, LFSR_SIZE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(LFSR_SIZE_A::OFF),
            1 => Val(LFSR_SIZE_A::_6B),
            2 => Val(LFSR_SIZE_A::_7B),
            3 => Val(LFSR_SIZE_A::_9B),
            4 => Val(LFSR_SIZE_A::_10B),
            5 => Val(LFSR_SIZE_A::_8B),
            6 => Val(LFSR_SIZE_A::_12B),
            i => Res(i),
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
#[doc = "Write proxy for field `LFSR_SIZE`"]
pub struct LFSR_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> LFSR_SIZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LFSR_SIZE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
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
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Reader of field `LFSR_SCALE`"]
pub type LFSR_SCALE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LFSR_SCALE`"]
pub struct LFSR_SCALE_W<'a> {
    w: &'a mut W,
}
impl<'a> LFSR_SCALE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Reader of field `LFSR_CLEAR`"]
pub type LFSR_CLEAR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LFSR_CLEAR`"]
pub struct LFSR_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> LFSR_CLEAR_W<'a> {
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
#[doc = "Reader of field `SEL_LFSR_MSB`"]
pub type SEL_LFSR_MSB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SEL_LFSR_MSB`"]
pub struct SEL_LFSR_MSB_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL_LFSR_MSB_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Selects the number of LSB bits to use from the LSFR to provide the clock dithering variation on the base period. Caveat make sure that SENSE_DIV > the maximum absolute range (e.g. for 4B SENSE_DIV > 8), otherwise results are undefined.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LFSR_BITS_A {
    #[doc = "0: use 2 bits: range = \\[-2,1\\]"]
    _2B,
    #[doc = "1: use 3 bits: range = \\[-4,3\\]"]
    _3B,
    #[doc = "2: use 4 bits: range = \\[-8,7\\]"]
    _4B,
    #[doc = "3: use 5 bits: range = \\[-16,15\\] (default)"]
    _5B,
}
impl From<LFSR_BITS_A> for u8 {
    #[inline(always)]
    fn from(variant: LFSR_BITS_A) -> Self {
        match variant {
            LFSR_BITS_A::_2B => 0,
            LFSR_BITS_A::_3B => 1,
            LFSR_BITS_A::_4B => 2,
            LFSR_BITS_A::_5B => 3,
        }
    }
}
#[doc = "Reader of field `LFSR_BITS`"]
pub type LFSR_BITS_R = crate::R<u8, LFSR_BITS_A>;
impl LFSR_BITS_R {
    #[doc = r"Get enumerated values variant"]
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
#[doc = "Write proxy for field `LFSR_BITS`"]
pub struct LFSR_BITS_W<'a> {
    w: &'a mut W,
}
impl<'a> LFSR_BITS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LFSR_BITS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
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
    #[doc = "use 5 bits: range = \\[-16,15\\] (default)"]
    #[inline(always)]
    pub fn _5b(self) -> &'a mut W {
        self.variant(LFSR_BITS_A::_5B)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
        self.w
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
        LFSR_SIZE_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 20:23 - Shift the LFSR output left by LSFR_SCALE bits before adding to SENSE_DIV. This dithering is disabled when SEL_LSFR_MSB is set. The clock divider to be used = (SENSE_DIV+1) + (SEL_LSFR_MSB ? 0 : (LFSR_OUT<<LFSR_SCALE)). Note that the clock divider including the dithering term must fit in 12 bits, otherwise the result is undefined."]
    #[inline(always)]
    pub fn lfsr_scale(&self) -> LFSR_SCALE_R {
        LFSR_SCALE_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - When set, forces the LFSR to it's initial state (all ones). This bit is automatically cleared by hardware after the LFSR is cleared, which is at the next clk_csd positive edge. This bit should be set whenever this register is written and the LFSR is used. Note that the LFSR will also get reset to all ones during the AutoZero_1/2 states."]
    #[inline(always)]
    pub fn lfsr_clear(&self) -> LFSR_CLEAR_R {
        LFSR_CLEAR_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Use the MSB of configured LSFR size as csd_sense signal. Intended to be used only with bit 8 or 12-bit LFSR size for CSDv1 backward compatibility (PRS). When this bit is set then clock divider dithering is disabled and SENSE_WIDTH is disabled."]
    #[inline(always)]
    pub fn sel_lfsr_msb(&self) -> SEL_LFSR_MSB_R {
        SEL_LFSR_MSB_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bits 26:27 - Selects the number of LSB bits to use from the LSFR to provide the clock dithering variation on the base period. Caveat make sure that SENSE_DIV > the maximum absolute range (e.g. for 4B SENSE_DIV > 8), otherwise results are undefined."]
    #[inline(always)]
    pub fn lfsr_bits(&self) -> LFSR_BITS_R {
        LFSR_BITS_R::new(((self.bits >> 26) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - The length-1 of the Sense modulation 'clock' period in clk_csd cycles. For regular CSD one sense clock cycle = one conversion (=phi1+phi2) . Note this is the base divider, clock dithering may change the actual period length. Note that SENSE_DIV must be at least 1 and additionally also allow for one clk_hf of non overlap (if OVERLAP_HI1/2 is set) on both phases, i.e. if clk_csd=clk_hf then SENSE_DIV must be >=3. In addition the FILTER_DELAY needs to be added to the minimum allowed SENSE_DIV value."]
    #[inline(always)]
    pub fn sense_div(&mut self) -> SENSE_DIV_W {
        SENSE_DIV_W { w: self }
    }
    #[doc = "Bits 16:18 - Selects the length of the LFSR which determines the LFSR repeat period. LFSR_BITS LSB of the LFSR are used for the clock dithering variation on the base period (was PRS in CSDv1). Whenever the LFSR is used (non zero value in this field) the LFSR_CLEAR bit should also be set."]
    #[inline(always)]
    pub fn lfsr_size(&mut self) -> LFSR_SIZE_W {
        LFSR_SIZE_W { w: self }
    }
    #[doc = "Bits 20:23 - Shift the LFSR output left by LSFR_SCALE bits before adding to SENSE_DIV. This dithering is disabled when SEL_LSFR_MSB is set. The clock divider to be used = (SENSE_DIV+1) + (SEL_LSFR_MSB ? 0 : (LFSR_OUT<<LFSR_SCALE)). Note that the clock divider including the dithering term must fit in 12 bits, otherwise the result is undefined."]
    #[inline(always)]
    pub fn lfsr_scale(&mut self) -> LFSR_SCALE_W {
        LFSR_SCALE_W { w: self }
    }
    #[doc = "Bit 24 - When set, forces the LFSR to it's initial state (all ones). This bit is automatically cleared by hardware after the LFSR is cleared, which is at the next clk_csd positive edge. This bit should be set whenever this register is written and the LFSR is used. Note that the LFSR will also get reset to all ones during the AutoZero_1/2 states."]
    #[inline(always)]
    pub fn lfsr_clear(&mut self) -> LFSR_CLEAR_W {
        LFSR_CLEAR_W { w: self }
    }
    #[doc = "Bit 25 - Use the MSB of configured LSFR size as csd_sense signal. Intended to be used only with bit 8 or 12-bit LFSR size for CSDv1 backward compatibility (PRS). When this bit is set then clock divider dithering is disabled and SENSE_WIDTH is disabled."]
    #[inline(always)]
    pub fn sel_lfsr_msb(&mut self) -> SEL_LFSR_MSB_W {
        SEL_LFSR_MSB_W { w: self }
    }
    #[doc = "Bits 26:27 - Selects the number of LSB bits to use from the LSFR to provide the clock dithering variation on the base period. Caveat make sure that SENSE_DIV > the maximum absolute range (e.g. for 4B SENSE_DIV > 8), otherwise results are undefined."]
    #[inline(always)]
    pub fn lfsr_bits(&mut self) -> LFSR_BITS_W {
        LFSR_BITS_W { w: self }
    }
}
