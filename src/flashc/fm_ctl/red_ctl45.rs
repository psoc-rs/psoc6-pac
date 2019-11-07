#[doc = "Reader of register RED_CTL45"]
pub type R = crate::R<u32, super::RED_CTL45>;
#[doc = "Writer for register RED_CTL45"]
pub type W = crate::W<u32, super::RED_CTL45>;
#[doc = "Register RED_CTL45 `reset()`'s with value 0"]
impl crate::ResetValue for super::RED_CTL45 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DNU_45_1`"]
pub type DNU_45_1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DNU_45_1`"]
pub struct DNU_45_1_W<'a> {
    w: &'a mut W,
}
impl<'a> DNU_45_1_W<'a> {
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
#[doc = "Reader of field `REG_ACT_HV`"]
pub type REG_ACT_HV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `REG_ACT_HV`"]
pub struct REG_ACT_HV_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_ACT_HV_W<'a> {
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
#[doc = "Reader of field `DNU_45_3`"]
pub type DNU_45_3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DNU_45_3`"]
pub struct DNU_45_3_W<'a> {
    w: &'a mut W,
}
impl<'a> DNU_45_3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `FDIV_TRIM_HV_0`"]
pub type FDIV_TRIM_HV_0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FDIV_TRIM_HV_0`"]
pub struct FDIV_TRIM_HV_0_W<'a> {
    w: &'a mut W,
}
impl<'a> FDIV_TRIM_HV_0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `DNU_45_5`"]
pub type DNU_45_5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DNU_45_5`"]
pub struct DNU_45_5_W<'a> {
    w: &'a mut W,
}
impl<'a> DNU_45_5_W<'a> {
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
#[doc = "Reader of field `FDIV_TRIM_HV_1`"]
pub type FDIV_TRIM_HV_1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FDIV_TRIM_HV_1`"]
pub struct FDIV_TRIM_HV_1_W<'a> {
    w: &'a mut W,
}
impl<'a> FDIV_TRIM_HV_1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `DNU_45_6`"]
pub type DNU_45_6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DNU_45_6`"]
pub struct DNU_45_6_W<'a> {
    w: &'a mut W,
}
impl<'a> DNU_45_6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `VLIM_TRIM_HV_0`"]
pub type VLIM_TRIM_HV_0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VLIM_TRIM_HV_0`"]
pub struct VLIM_TRIM_HV_0_W<'a> {
    w: &'a mut W,
}
impl<'a> VLIM_TRIM_HV_0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `DNU_45_8`"]
pub type DNU_45_8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DNU_45_8`"]
pub struct DNU_45_8_W<'a> {
    w: &'a mut W,
}
impl<'a> DNU_45_8_W<'a> {
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
#[doc = "Reader of field `DNU_45_23_16`"]
pub type DNU_45_23_16_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DNU_45_23_16`"]
pub struct DNU_45_23_16_W<'a> {
    w: &'a mut W,
}
impl<'a> DNU_45_23_16_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Not Used"]
    #[inline(always)]
    pub fn dnu_45_1(&self) -> DNU_45_1_R {
        DNU_45_1_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Forces the VBST regulator in active mode all the time"]
    #[inline(always)]
    pub fn reg_act_hv(&self) -> REG_ACT_HV_R {
        REG_ACT_HV_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Not Used"]
    #[inline(always)]
    pub fn dnu_45_3(&self) -> DNU_45_3_R {
        DNU_45_3_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - '2b00' F = 1MHz see fdiv_trim_hv<1> value as well '2b01' F = 0.5MHz '2b10' F = 2MHz '2b11' F = 4Mhz"]
    #[inline(always)]
    pub fn fdiv_trim_hv_0(&self) -> FDIV_TRIM_HV_0_R {
        FDIV_TRIM_HV_0_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Not Used"]
    #[inline(always)]
    pub fn dnu_45_5(&self) -> DNU_45_5_R {
        DNU_45_5_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - '2b00' F = 1MHz see fdiv_trim_hv<0> value as well '2b01' F = 0.5MHz '2b10' F = 2MHz '2b11' F = 4Mhz"]
    #[inline(always)]
    pub fn fdiv_trim_hv_1(&self) -> FDIV_TRIM_HV_1_R {
        FDIV_TRIM_HV_1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Not Used"]
    #[inline(always)]
    pub fn dnu_45_6(&self) -> DNU_45_6_R {
        DNU_45_6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - '2b00' V2 = 650mV see vlim_trim_hv<1> value as well '2b01' V2 = 600mV '2b10' V2 = 750mV '2b11' V2 = 700mV"]
    #[inline(always)]
    pub fn vlim_trim_hv_0(&self) -> VLIM_TRIM_HV_0_R {
        VLIM_TRIM_HV_0_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Not Used"]
    #[inline(always)]
    pub fn dnu_45_8(&self) -> DNU_45_8_R {
        DNU_45_8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 16:23 - Not Used"]
    #[inline(always)]
    pub fn dnu_45_23_16(&self) -> DNU_45_23_16_R {
        DNU_45_23_16_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Not Used"]
    #[inline(always)]
    pub fn dnu_45_1(&mut self) -> DNU_45_1_W {
        DNU_45_1_W { w: self }
    }
    #[doc = "Bit 1 - Forces the VBST regulator in active mode all the time"]
    #[inline(always)]
    pub fn reg_act_hv(&mut self) -> REG_ACT_HV_W {
        REG_ACT_HV_W { w: self }
    }
    #[doc = "Bit 2 - Not Used"]
    #[inline(always)]
    pub fn dnu_45_3(&mut self) -> DNU_45_3_W {
        DNU_45_3_W { w: self }
    }
    #[doc = "Bit 3 - '2b00' F = 1MHz see fdiv_trim_hv<1> value as well '2b01' F = 0.5MHz '2b10' F = 2MHz '2b11' F = 4Mhz"]
    #[inline(always)]
    pub fn fdiv_trim_hv_0(&mut self) -> FDIV_TRIM_HV_0_W {
        FDIV_TRIM_HV_0_W { w: self }
    }
    #[doc = "Bit 4 - Not Used"]
    #[inline(always)]
    pub fn dnu_45_5(&mut self) -> DNU_45_5_W {
        DNU_45_5_W { w: self }
    }
    #[doc = "Bit 5 - '2b00' F = 1MHz see fdiv_trim_hv<0> value as well '2b01' F = 0.5MHz '2b10' F = 2MHz '2b11' F = 4Mhz"]
    #[inline(always)]
    pub fn fdiv_trim_hv_1(&mut self) -> FDIV_TRIM_HV_1_W {
        FDIV_TRIM_HV_1_W { w: self }
    }
    #[doc = "Bit 6 - Not Used"]
    #[inline(always)]
    pub fn dnu_45_6(&mut self) -> DNU_45_6_W {
        DNU_45_6_W { w: self }
    }
    #[doc = "Bit 7 - '2b00' V2 = 650mV see vlim_trim_hv<1> value as well '2b01' V2 = 600mV '2b10' V2 = 750mV '2b11' V2 = 700mV"]
    #[inline(always)]
    pub fn vlim_trim_hv_0(&mut self) -> VLIM_TRIM_HV_0_W {
        VLIM_TRIM_HV_0_W { w: self }
    }
    #[doc = "Bit 8 - Not Used"]
    #[inline(always)]
    pub fn dnu_45_8(&mut self) -> DNU_45_8_W {
        DNU_45_8_W { w: self }
    }
    #[doc = "Bits 16:23 - Not Used"]
    #[inline(always)]
    pub fn dnu_45_23_16(&mut self) -> DNU_45_23_16_W {
        DNU_45_23_16_W { w: self }
    }
}
