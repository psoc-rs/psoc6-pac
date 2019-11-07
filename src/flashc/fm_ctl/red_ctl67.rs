#[doc = "Reader of register RED_CTL67"]
pub type R = crate::R<u32, super::RED_CTL67>;
#[doc = "Writer for register RED_CTL67"]
pub type W = crate::W<u32, super::RED_CTL67>;
#[doc = "Register RED_CTL67 `reset()`'s with value 0"]
impl crate::ResetValue for super::RED_CTL67 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `VLIM_TRIM_HV_1`"]
pub type VLIM_TRIM_HV_1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VLIM_TRIM_HV_1`"]
pub struct VLIM_TRIM_HV_1_W<'a> {
    w: &'a mut W,
}
impl<'a> VLIM_TRIM_HV_1_W<'a> {
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
#[doc = "Reader of field `DNU_67_1`"]
pub type DNU_67_1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DNU_67_1`"]
pub struct DNU_67_1_W<'a> {
    w: &'a mut W,
}
impl<'a> DNU_67_1_W<'a> {
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
#[doc = "Reader of field `VPROT_ACT_HV`"]
pub type VPROT_ACT_HV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VPROT_ACT_HV`"]
pub struct VPROT_ACT_HV_W<'a> {
    w: &'a mut W,
}
impl<'a> VPROT_ACT_HV_W<'a> {
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
#[doc = "Reader of field `DNU_67_3`"]
pub type DNU_67_3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DNU_67_3`"]
pub struct DNU_67_3_W<'a> {
    w: &'a mut W,
}
impl<'a> DNU_67_3_W<'a> {
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
#[doc = "Reader of field `IPREF_TC_HV`"]
pub type IPREF_TC_HV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IPREF_TC_HV`"]
pub struct IPREF_TC_HV_W<'a> {
    w: &'a mut W,
}
impl<'a> IPREF_TC_HV_W<'a> {
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
#[doc = "Reader of field `DNU_67_5`"]
pub type DNU_67_5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DNU_67_5`"]
pub struct DNU_67_5_W<'a> {
    w: &'a mut W,
}
impl<'a> DNU_67_5_W<'a> {
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
#[doc = "Reader of field `IPREF_TRIMA_HI_HV`"]
pub type IPREF_TRIMA_HI_HV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IPREF_TRIMA_HI_HV`"]
pub struct IPREF_TRIMA_HI_HV_W<'a> {
    w: &'a mut W,
}
impl<'a> IPREF_TRIMA_HI_HV_W<'a> {
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
#[doc = "Reader of field `DNU_67_7`"]
pub type DNU_67_7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DNU_67_7`"]
pub struct DNU_67_7_W<'a> {
    w: &'a mut W,
}
impl<'a> DNU_67_7_W<'a> {
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
#[doc = "Reader of field `IPREF_TRIMA_LO_HV`"]
pub type IPREF_TRIMA_LO_HV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IPREF_TRIMA_LO_HV`"]
pub struct IPREF_TRIMA_LO_HV_W<'a> {
    w: &'a mut W,
}
impl<'a> IPREF_TRIMA_LO_HV_W<'a> {
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
#[doc = "Reader of field `DNU_67_23_16`"]
pub type DNU_67_23_16_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DNU_67_23_16`"]
pub struct DNU_67_23_16_W<'a> {
    w: &'a mut W,
}
impl<'a> DNU_67_23_16_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - '2b00' V2 = 650mV see vlim_trim_hv<0> value as well '2b01' V2 = 600mV '2b10' V2 = 750mV '2b11' V2 = 700mV"]
    #[inline(always)]
    pub fn vlim_trim_hv_1(&self) -> VLIM_TRIM_HV_1_R {
        VLIM_TRIM_HV_1_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Not Used"]
    #[inline(always)]
    pub fn dnu_67_1(&self) -> DNU_67_1_R {
        DNU_67_1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Forces VPROT in active mode all the time"]
    #[inline(always)]
    pub fn vprot_act_hv(&self) -> VPROT_ACT_HV_R {
        VPROT_ACT_HV_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Not Used"]
    #[inline(always)]
    pub fn dnu_67_3(&self) -> DNU_67_3_R {
        DNU_67_3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Reduces the IPREF Tempco by not subtracting ICREF form IPREF - IPREF will be 1uA"]
    #[inline(always)]
    pub fn ipref_tc_hv(&self) -> IPREF_TC_HV_R {
        IPREF_TC_HV_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Not Used"]
    #[inline(always)]
    pub fn dnu_67_5(&self) -> DNU_67_5_R {
        DNU_67_5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Adds 200-300nA boost on IPREF_HI"]
    #[inline(always)]
    pub fn ipref_trima_hi_hv(&self) -> IPREF_TRIMA_HI_HV_R {
        IPREF_TRIMA_HI_HV_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Not Used"]
    #[inline(always)]
    pub fn dnu_67_7(&self) -> DNU_67_7_R {
        DNU_67_7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Adds 200-300nA boost on IPREF_LO"]
    #[inline(always)]
    pub fn ipref_trima_lo_hv(&self) -> IPREF_TRIMA_LO_HV_R {
        IPREF_TRIMA_LO_HV_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 16:23 - Not Used"]
    #[inline(always)]
    pub fn dnu_67_23_16(&self) -> DNU_67_23_16_R {
        DNU_67_23_16_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - '2b00' V2 = 650mV see vlim_trim_hv<0> value as well '2b01' V2 = 600mV '2b10' V2 = 750mV '2b11' V2 = 700mV"]
    #[inline(always)]
    pub fn vlim_trim_hv_1(&mut self) -> VLIM_TRIM_HV_1_W {
        VLIM_TRIM_HV_1_W { w: self }
    }
    #[doc = "Bit 1 - Not Used"]
    #[inline(always)]
    pub fn dnu_67_1(&mut self) -> DNU_67_1_W {
        DNU_67_1_W { w: self }
    }
    #[doc = "Bit 2 - Forces VPROT in active mode all the time"]
    #[inline(always)]
    pub fn vprot_act_hv(&mut self) -> VPROT_ACT_HV_W {
        VPROT_ACT_HV_W { w: self }
    }
    #[doc = "Bit 3 - Not Used"]
    #[inline(always)]
    pub fn dnu_67_3(&mut self) -> DNU_67_3_W {
        DNU_67_3_W { w: self }
    }
    #[doc = "Bit 4 - Reduces the IPREF Tempco by not subtracting ICREF form IPREF - IPREF will be 1uA"]
    #[inline(always)]
    pub fn ipref_tc_hv(&mut self) -> IPREF_TC_HV_W {
        IPREF_TC_HV_W { w: self }
    }
    #[doc = "Bit 5 - Not Used"]
    #[inline(always)]
    pub fn dnu_67_5(&mut self) -> DNU_67_5_W {
        DNU_67_5_W { w: self }
    }
    #[doc = "Bit 6 - Adds 200-300nA boost on IPREF_HI"]
    #[inline(always)]
    pub fn ipref_trima_hi_hv(&mut self) -> IPREF_TRIMA_HI_HV_W {
        IPREF_TRIMA_HI_HV_W { w: self }
    }
    #[doc = "Bit 7 - Not Used"]
    #[inline(always)]
    pub fn dnu_67_7(&mut self) -> DNU_67_7_W {
        DNU_67_7_W { w: self }
    }
    #[doc = "Bit 8 - Adds 200-300nA boost on IPREF_LO"]
    #[inline(always)]
    pub fn ipref_trima_lo_hv(&mut self) -> IPREF_TRIMA_LO_HV_W {
        IPREF_TRIMA_LO_HV_W { w: self }
    }
    #[doc = "Bits 16:23 - Not Used"]
    #[inline(always)]
    pub fn dnu_67_23_16(&mut self) -> DNU_67_23_16_W {
        DNU_67_23_16_W { w: self }
    }
}
