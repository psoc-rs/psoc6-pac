#[doc = "Reader of register LF_CLK_CTRL"]
pub type R = crate::R<u32, super::LF_CLK_CTRL>;
#[doc = "Writer for register LF_CLK_CTRL"]
pub type W = crate::W<u32, super::LF_CLK_CTRL>;
#[doc = "Register LF_CLK_CTRL `reset()`'s with value 0x4000_0000"]
impl crate::ResetValue for super::LF_CLK_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x4000_0000
    }
}
#[doc = "Reader of field `DISABLE_LF_CLK`"]
pub type DISABLE_LF_CLK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DISABLE_LF_CLK`"]
pub struct DISABLE_LF_CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> DISABLE_LF_CLK_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `ENABLE_ENC_CLK`"]
pub type ENABLE_ENC_CLK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE_ENC_CLK`"]
pub struct ENABLE_ENC_CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_ENC_CLK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `M0S8BLESS_REV_ID`"]
pub type M0S8BLESS_REV_ID_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 0 - When set to 1, gates the LF clock input to the Link Layer. Ths is done for extended DSM mode where the DSM state machine needs to be forzen to prevent a default auto exit."]
    #[inline(always)]
    pub fn disable_lf_clk(&self) -> DISABLE_LF_CLK_R {
        DISABLE_LF_CLK_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - This bit is used to enable the clock to the encryption engine 0 - Disable the clock to ENC engine 1 - Enable the clock to ENC engine"]
    #[inline(always)]
    pub fn enable_enc_clk(&self) -> ENABLE_ENC_CLK_R {
        ENABLE_ENC_CLK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 29:31 - Indicates the m0s8bless IP revision."]
    #[inline(always)]
    pub fn m0s8bless_rev_id(&self) -> M0S8BLESS_REV_ID_R {
        M0S8BLESS_REV_ID_R::new(((self.bits >> 29) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - When set to 1, gates the LF clock input to the Link Layer. Ths is done for extended DSM mode where the DSM state machine needs to be forzen to prevent a default auto exit."]
    #[inline(always)]
    pub fn disable_lf_clk(&mut self) -> DISABLE_LF_CLK_W {
        DISABLE_LF_CLK_W { w: self }
    }
    #[doc = "Bit 1 - This bit is used to enable the clock to the encryption engine 0 - Disable the clock to ENC engine 1 - Enable the clock to ENC engine"]
    #[inline(always)]
    pub fn enable_enc_clk(&mut self) -> ENABLE_ENC_CLK_W {
        ENABLE_ENC_CLK_W { w: self }
    }
}
