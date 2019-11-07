#[doc = "Reader of register I2C_CFG"]
pub type R = crate::R<u32, super::I2C_CFG>;
#[doc = "Writer for register I2C_CFG"]
pub type W = crate::W<u32, super::I2C_CFG>;
#[doc = "Register I2C_CFG `reset()`'s with value 0x002a_1013"]
impl crate::ResetValue for super::I2C_CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x002a_1013
    }
}
#[doc = "Reader of field `SDA_IN_FILT_TRIM`"]
pub type SDA_IN_FILT_TRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SDA_IN_FILT_TRIM`"]
pub struct SDA_IN_FILT_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> SDA_IN_FILT_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `SDA_IN_FILT_SEL`"]
pub type SDA_IN_FILT_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SDA_IN_FILT_SEL`"]
pub struct SDA_IN_FILT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SDA_IN_FILT_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `SCL_IN_FILT_TRIM`"]
pub type SCL_IN_FILT_TRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SCL_IN_FILT_TRIM`"]
pub struct SCL_IN_FILT_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> SCL_IN_FILT_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `SCL_IN_FILT_SEL`"]
pub type SCL_IN_FILT_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCL_IN_FILT_SEL`"]
pub struct SCL_IN_FILT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SCL_IN_FILT_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `SDA_OUT_FILT0_TRIM`"]
pub type SDA_OUT_FILT0_TRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SDA_OUT_FILT0_TRIM`"]
pub struct SDA_OUT_FILT0_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> SDA_OUT_FILT0_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `SDA_OUT_FILT1_TRIM`"]
pub type SDA_OUT_FILT1_TRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SDA_OUT_FILT1_TRIM`"]
pub struct SDA_OUT_FILT1_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> SDA_OUT_FILT1_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "Reader of field `SDA_OUT_FILT2_TRIM`"]
pub type SDA_OUT_FILT2_TRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SDA_OUT_FILT2_TRIM`"]
pub struct SDA_OUT_FILT2_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> SDA_OUT_FILT2_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Reader of field `SDA_OUT_FILT_SEL`"]
pub type SDA_OUT_FILT_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SDA_OUT_FILT_SEL`"]
pub struct SDA_OUT_FILT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SDA_OUT_FILT_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Trim settings for the 50ns glitch filter on the SDA input. Default setting meets the I2C glitch rejections specs. Programmability available if required"]
    #[inline(always)]
    pub fn sda_in_filt_trim(&self) -> SDA_IN_FILT_TRIM_R {
        SDA_IN_FILT_TRIM_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 4 - Enable for 50ns glitch filter on SDA input '0': 0 ns. '1: 50 ns (filter enabled)."]
    #[inline(always)]
    pub fn sda_in_filt_sel(&self) -> SDA_IN_FILT_SEL_R {
        SDA_IN_FILT_SEL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Trim settings for the 50ns glitch filter on the SDA input. Default setting meets the I2C glitch rejections specs. Programmability available if required"]
    #[inline(always)]
    pub fn scl_in_filt_trim(&self) -> SCL_IN_FILT_TRIM_R {
        SCL_IN_FILT_TRIM_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 12 - Enable for 50ns glitch filter on SCL input '0': 0 ns. '1: 50 ns (filter enabled)."]
    #[inline(always)]
    pub fn scl_in_filt_sel(&self) -> SCL_IN_FILT_SEL_R {
        SCL_IN_FILT_SEL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - Trim settings for the 50ns delay filter on SDA output used to gurantee tHD_DAT I2C parameter. Default setting meets the I2C spec. Programmability available if required"]
    #[inline(always)]
    pub fn sda_out_filt0_trim(&self) -> SDA_OUT_FILT0_TRIM_R {
        SDA_OUT_FILT0_TRIM_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - Trim settings for the 50ns delay filter on SDA output used to gurantee tHD_DAT I2C parameter. Default setting meets the I2C spec. Programmability available if required"]
    #[inline(always)]
    pub fn sda_out_filt1_trim(&self) -> SDA_OUT_FILT1_TRIM_R {
        SDA_OUT_FILT1_TRIM_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - Trim settings for the 50ns delay filter on SDA output used to gurantee tHD_DAT I2C parameter. Default setting meets the I2C spec. Programmability available if required"]
    #[inline(always)]
    pub fn sda_out_filt2_trim(&self) -> SDA_OUT_FILT2_TRIM_R {
        SDA_OUT_FILT2_TRIM_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - Selection of cumulative filter delay on SDA output to meet tHD_DAT parameter '0': 0 ns. '1': 50 ns (filter 0 enabled). '2': 100 ns (filters 0 and 1 enabled). '3': 150 ns (filters 0, 1 and 2 enabled)."]
    #[inline(always)]
    pub fn sda_out_filt_sel(&self) -> SDA_OUT_FILT_SEL_R {
        SDA_OUT_FILT_SEL_R::new(((self.bits >> 28) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Trim settings for the 50ns glitch filter on the SDA input. Default setting meets the I2C glitch rejections specs. Programmability available if required"]
    #[inline(always)]
    pub fn sda_in_filt_trim(&mut self) -> SDA_IN_FILT_TRIM_W {
        SDA_IN_FILT_TRIM_W { w: self }
    }
    #[doc = "Bit 4 - Enable for 50ns glitch filter on SDA input '0': 0 ns. '1: 50 ns (filter enabled)."]
    #[inline(always)]
    pub fn sda_in_filt_sel(&mut self) -> SDA_IN_FILT_SEL_W {
        SDA_IN_FILT_SEL_W { w: self }
    }
    #[doc = "Bits 8:9 - Trim settings for the 50ns glitch filter on the SDA input. Default setting meets the I2C glitch rejections specs. Programmability available if required"]
    #[inline(always)]
    pub fn scl_in_filt_trim(&mut self) -> SCL_IN_FILT_TRIM_W {
        SCL_IN_FILT_TRIM_W { w: self }
    }
    #[doc = "Bit 12 - Enable for 50ns glitch filter on SCL input '0': 0 ns. '1: 50 ns (filter enabled)."]
    #[inline(always)]
    pub fn scl_in_filt_sel(&mut self) -> SCL_IN_FILT_SEL_W {
        SCL_IN_FILT_SEL_W { w: self }
    }
    #[doc = "Bits 16:17 - Trim settings for the 50ns delay filter on SDA output used to gurantee tHD_DAT I2C parameter. Default setting meets the I2C spec. Programmability available if required"]
    #[inline(always)]
    pub fn sda_out_filt0_trim(&mut self) -> SDA_OUT_FILT0_TRIM_W {
        SDA_OUT_FILT0_TRIM_W { w: self }
    }
    #[doc = "Bits 18:19 - Trim settings for the 50ns delay filter on SDA output used to gurantee tHD_DAT I2C parameter. Default setting meets the I2C spec. Programmability available if required"]
    #[inline(always)]
    pub fn sda_out_filt1_trim(&mut self) -> SDA_OUT_FILT1_TRIM_W {
        SDA_OUT_FILT1_TRIM_W { w: self }
    }
    #[doc = "Bits 20:21 - Trim settings for the 50ns delay filter on SDA output used to gurantee tHD_DAT I2C parameter. Default setting meets the I2C spec. Programmability available if required"]
    #[inline(always)]
    pub fn sda_out_filt2_trim(&mut self) -> SDA_OUT_FILT2_TRIM_W {
        SDA_OUT_FILT2_TRIM_W { w: self }
    }
    #[doc = "Bits 28:29 - Selection of cumulative filter delay on SDA output to meet tHD_DAT parameter '0': 0 ns. '1': 50 ns (filter 0 enabled). '2': 100 ns (filters 0 and 1 enabled). '3': 150 ns (filters 0, 1 and 2 enabled)."]
    #[inline(always)]
    pub fn sda_out_filt_sel(&mut self) -> SDA_OUT_FILT_SEL_W {
        SDA_OUT_FILT_SEL_W { w: self }
    }
}
