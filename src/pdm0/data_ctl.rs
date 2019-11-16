#[doc = "Reader of register DATA_CTL"]
pub type R = crate::R<u32, super::DATA_CTL>;
#[doc = "Writer for register DATA_CTL"]
pub type W = crate::W<u32, super::DATA_CTL>;
#[doc = "Register DATA_CTL `reset()`'s with value 0"]
impl crate::ResetValue for super::DATA_CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "PCM Word Length in number of bits: (Note: These bits are connected to AR36U12.PDM_CORE2_CFG.PCM_IWL)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WORD_LEN_A {
    #[doc = "0: 16-bit"]
    BIT_LEN16 = 0,
    #[doc = "1: 18-bit"]
    BIT_LEN18 = 1,
    #[doc = "2: 20-bit"]
    BIT_LEN20 = 2,
    #[doc = "3: 24-bit"]
    BIT_LEN24 = 3,
}
impl From<WORD_LEN_A> for u8 {
    #[inline(always)]
    fn from(variant: WORD_LEN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `WORD_LEN`"]
pub type WORD_LEN_R = crate::R<u8, WORD_LEN_A>;
impl WORD_LEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WORD_LEN_A {
        match self.bits {
            0 => WORD_LEN_A::BIT_LEN16,
            1 => WORD_LEN_A::BIT_LEN18,
            2 => WORD_LEN_A::BIT_LEN20,
            3 => WORD_LEN_A::BIT_LEN24,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BIT_LEN16`"]
    #[inline(always)]
    pub fn is_bit_len16(&self) -> bool {
        *self == WORD_LEN_A::BIT_LEN16
    }
    #[doc = "Checks if the value of the field is `BIT_LEN18`"]
    #[inline(always)]
    pub fn is_bit_len18(&self) -> bool {
        *self == WORD_LEN_A::BIT_LEN18
    }
    #[doc = "Checks if the value of the field is `BIT_LEN20`"]
    #[inline(always)]
    pub fn is_bit_len20(&self) -> bool {
        *self == WORD_LEN_A::BIT_LEN20
    }
    #[doc = "Checks if the value of the field is `BIT_LEN24`"]
    #[inline(always)]
    pub fn is_bit_len24(&self) -> bool {
        *self == WORD_LEN_A::BIT_LEN24
    }
}
#[doc = "Write proxy for field `WORD_LEN`"]
pub struct WORD_LEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WORD_LEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WORD_LEN_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "16-bit"]
    #[inline(always)]
    pub fn bit_len16(self) -> &'a mut W {
        self.variant(WORD_LEN_A::BIT_LEN16)
    }
    #[doc = "18-bit"]
    #[inline(always)]
    pub fn bit_len18(self) -> &'a mut W {
        self.variant(WORD_LEN_A::BIT_LEN18)
    }
    #[doc = "20-bit"]
    #[inline(always)]
    pub fn bit_len20(self) -> &'a mut W {
        self.variant(WORD_LEN_A::BIT_LEN20)
    }
    #[doc = "24-bit"]
    #[inline(always)]
    pub fn bit_len24(self) -> &'a mut W {
        self.variant(WORD_LEN_A::BIT_LEN24)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `BIT_EXTENSION`"]
pub type BIT_EXTENSION_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BIT_EXTENSION`"]
pub struct BIT_EXTENSION_W<'a> {
    w: &'a mut W,
}
impl<'a> BIT_EXTENSION_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - PCM Word Length in number of bits: (Note: These bits are connected to AR36U12.PDM_CORE2_CFG.PCM_IWL)"]
    #[inline(always)]
    pub fn word_len(&self) -> WORD_LEN_R {
        WORD_LEN_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 8 - When reception word length is shorter than the word length of RX_FIFO_RD, extension mode of upper bit should be set. '0': Extended by '0' '1': Extended by sign bit (if MSB word is '1', then it is extended by '1', if MSB is '0' then it is extended by '0')"]
    #[inline(always)]
    pub fn bit_extension(&self) -> BIT_EXTENSION_R {
        BIT_EXTENSION_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - PCM Word Length in number of bits: (Note: These bits are connected to AR36U12.PDM_CORE2_CFG.PCM_IWL)"]
    #[inline(always)]
    pub fn word_len(&mut self) -> WORD_LEN_W {
        WORD_LEN_W { w: self }
    }
    #[doc = "Bit 8 - When reception word length is shorter than the word length of RX_FIFO_RD, extension mode of upper bit should be set. '0': Extended by '0' '1': Extended by sign bit (if MSB word is '1', then it is extended by '1', if MSB is '0' then it is extended by '0')"]
    #[inline(always)]
    pub fn bit_extension(&mut self) -> BIT_EXTENSION_W {
        BIT_EXTENSION_W { w: self }
    }
}
