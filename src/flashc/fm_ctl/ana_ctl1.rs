#[doc = "Reader of register ANA_CTL1"]
pub type R = crate::R<u32, super::ANA_CTL1>;
#[doc = "Writer for register ANA_CTL1"]
pub type W = crate::W<u32, super::ANA_CTL1>;
#[doc = "Register ANA_CTL1 `reset()`'s with value 0x0606_0000"]
impl crate::ResetValue for super::ANA_CTL1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0606_0000
    }
}
#[doc = "Reader of field `MDAC`"]
pub type MDAC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MDAC`"]
pub struct MDAC_W<'a> {
    w: &'a mut W,
}
impl<'a> MDAC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `PDAC`"]
pub type PDAC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PDAC`"]
pub struct PDAC_W<'a> {
    w: &'a mut W,
}
impl<'a> PDAC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `NDAC`"]
pub type NDAC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NDAC`"]
pub struct NDAC_W<'a> {
    w: &'a mut W,
}
impl<'a> NDAC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Reader of field `VPROT_OVERRIDE`"]
pub type VPROT_OVERRIDE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VPROT_OVERRIDE`"]
pub struct VPROT_OVERRIDE_W<'a> {
    w: &'a mut W,
}
impl<'a> VPROT_OVERRIDE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `R_GRANT_CTL`"]
pub type R_GRANT_CTL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `R_GRANT_CTL`"]
pub struct R_GRANT_CTL_W<'a> {
    w: &'a mut W,
}
impl<'a> R_GRANT_CTL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `RST_SFT_HVPL`"]
pub type RST_SFT_HVPL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RST_SFT_HVPL`"]
pub struct RST_SFT_HVPL_W<'a> {
    w: &'a mut W,
}
impl<'a> RST_SFT_HVPL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Trimming of the output margin Voltage as a function of Vpos and Vneg."]
    #[inline(always)]
    pub fn mdac(&self) -> MDAC_R {
        MDAC_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - Trimming of positive pump output Voltage:"]
    #[inline(always)]
    pub fn pdac(&self) -> PDAC_R {
        PDAC_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Trimming of negative pump output Voltage:"]
    #[inline(always)]
    pub fn ndac(&self) -> NDAC_R {
        NDAC_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - '0': vprot = BG.vprot. '1': vprot = vcc"]
    #[inline(always)]
    pub fn vprot_override(&self) -> VPROT_OVERRIDE_R {
        VPROT_OVERRIDE_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - r_grant control: '0': r_grant normal functionality '1': forces r_grant LO synchronized on clk_r"]
    #[inline(always)]
    pub fn r_grant_ctl(&self) -> R_GRANT_CTL_R {
        R_GRANT_CTL_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - '1': Page Latches Soft Reset"]
    #[inline(always)]
    pub fn rst_sft_hvpl(&self) -> RST_SFT_HVPL_R {
        RST_SFT_HVPL_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Trimming of the output margin Voltage as a function of Vpos and Vneg."]
    #[inline(always)]
    pub fn mdac(&mut self) -> MDAC_W {
        MDAC_W { w: self }
    }
    #[doc = "Bits 16:19 - Trimming of positive pump output Voltage:"]
    #[inline(always)]
    pub fn pdac(&mut self) -> PDAC_W {
        PDAC_W { w: self }
    }
    #[doc = "Bits 24:27 - Trimming of negative pump output Voltage:"]
    #[inline(always)]
    pub fn ndac(&mut self) -> NDAC_W {
        NDAC_W { w: self }
    }
    #[doc = "Bit 28 - '0': vprot = BG.vprot. '1': vprot = vcc"]
    #[inline(always)]
    pub fn vprot_override(&mut self) -> VPROT_OVERRIDE_W {
        VPROT_OVERRIDE_W { w: self }
    }
    #[doc = "Bit 29 - r_grant control: '0': r_grant normal functionality '1': forces r_grant LO synchronized on clk_r"]
    #[inline(always)]
    pub fn r_grant_ctl(&mut self) -> R_GRANT_CTL_W {
        R_GRANT_CTL_W { w: self }
    }
    #[doc = "Bit 30 - '1': Page Latches Soft Reset"]
    #[inline(always)]
    pub fn rst_sft_hvpl(&mut self) -> RST_SFT_HVPL_W {
        RST_SFT_HVPL_W { w: self }
    }
}
