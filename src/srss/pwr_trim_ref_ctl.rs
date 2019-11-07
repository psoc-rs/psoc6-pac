#[doc = "Reader of register PWR_TRIM_REF_CTL"]
pub type R = crate::R<u32, super::PWR_TRIM_REF_CTL>;
#[doc = "Writer for register PWR_TRIM_REF_CTL"]
pub type W = crate::W<u32, super::PWR_TRIM_REF_CTL>;
#[doc = "Register PWR_TRIM_REF_CTL `reset()`'s with value 0x70f0_0000"]
impl crate::ResetValue for super::PWR_TRIM_REF_CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x70f0_0000
    }
}
#[doc = "Reader of field `ACT_REF_TCTRIM`"]
pub type ACT_REF_TCTRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ACT_REF_TCTRIM`"]
pub struct ACT_REF_TCTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> ACT_REF_TCTRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `ACT_REF_ITRIM`"]
pub type ACT_REF_ITRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ACT_REF_ITRIM`"]
pub struct ACT_REF_ITRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> ACT_REF_ITRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `ACT_REF_ABSTRIM`"]
pub type ACT_REF_ABSTRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ACT_REF_ABSTRIM`"]
pub struct ACT_REF_ABSTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> ACT_REF_ABSTRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "Reader of field `ACT_REF_IBOOST`"]
pub type ACT_REF_IBOOST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACT_REF_IBOOST`"]
pub struct ACT_REF_IBOOST_W<'a> {
    w: &'a mut W,
}
impl<'a> ACT_REF_IBOOST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `DPSLP_REF_TCTRIM`"]
pub type DPSLP_REF_TCTRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DPSLP_REF_TCTRIM`"]
pub struct DPSLP_REF_TCTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> DPSLP_REF_TCTRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `DPSLP_REF_ABSTRIM`"]
pub type DPSLP_REF_ABSTRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DPSLP_REF_ABSTRIM`"]
pub struct DPSLP_REF_ABSTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> DPSLP_REF_ABSTRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 20)) | (((value as u32) & 0x1f) << 20);
        self.w
    }
}
#[doc = "Reader of field `DPSLP_REF_ITRIM`"]
pub type DPSLP_REF_ITRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DPSLP_REF_ITRIM`"]
pub struct DPSLP_REF_ITRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> DPSLP_REF_ITRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Active-Reference temperature trim. This register is only reset by XRES/POR/BOD/OVP/HIBERNATE. 0 -> default setting at POR; not for trimming use others -> normal trim range"]
    #[inline(always)]
    pub fn act_ref_tctrim(&self) -> ACT_REF_TCTRIM_R {
        ACT_REF_TCTRIM_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Active-Reference current trim. This register is only reset by XRES/POR/BOD/OVP/HIBERNATE. 0 -> default setting at POR; not for trimming use others -> normal trim range"]
    #[inline(always)]
    pub fn act_ref_itrim(&self) -> ACT_REF_ITRIM_R {
        ACT_REF_ITRIM_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:12 - Active-Reference absolute voltage trim. This register is only reset by XRES/POR/BOD/OVP/HIBERNATE. 0 -> default setting at POR; not for trimming use others -> normal trim range"]
    #[inline(always)]
    pub fn act_ref_abstrim(&self) -> ACT_REF_ABSTRIM_R {
        ACT_REF_ABSTRIM_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 14 - Active-Reference current boost. This register is only reset by XRES/POR/BOD/OVP/HIBERNATE. 0: normal operation others: risk mitigation"]
    #[inline(always)]
    pub fn act_ref_iboost(&self) -> ACT_REF_IBOOST_R {
        ACT_REF_IBOOST_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 16:19 - DeepSleep-Reference temperature trim. This register is only reset by XRES/POR/BOD/OVP/HIBERNATE. 0 -> default setting at POR; not for trimming use others -> normal trim range"]
    #[inline(always)]
    pub fn dpslp_ref_tctrim(&self) -> DPSLP_REF_TCTRIM_R {
        DPSLP_REF_TCTRIM_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:24 - DeepSleep-Reference absolute voltage trim. This register is only reset by XRES/POR/BOD/OVP/HIBERNATE."]
    #[inline(always)]
    pub fn dpslp_ref_abstrim(&self) -> DPSLP_REF_ABSTRIM_R {
        DPSLP_REF_ABSTRIM_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 28:31 - DeepSleep current reference trim. This register is only reset by XRES/POR/BOD/OVP/HIBERNATE."]
    #[inline(always)]
    pub fn dpslp_ref_itrim(&self) -> DPSLP_REF_ITRIM_R {
        DPSLP_REF_ITRIM_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Active-Reference temperature trim. This register is only reset by XRES/POR/BOD/OVP/HIBERNATE. 0 -> default setting at POR; not for trimming use others -> normal trim range"]
    #[inline(always)]
    pub fn act_ref_tctrim(&mut self) -> ACT_REF_TCTRIM_W {
        ACT_REF_TCTRIM_W { w: self }
    }
    #[doc = "Bits 4:7 - Active-Reference current trim. This register is only reset by XRES/POR/BOD/OVP/HIBERNATE. 0 -> default setting at POR; not for trimming use others -> normal trim range"]
    #[inline(always)]
    pub fn act_ref_itrim(&mut self) -> ACT_REF_ITRIM_W {
        ACT_REF_ITRIM_W { w: self }
    }
    #[doc = "Bits 8:12 - Active-Reference absolute voltage trim. This register is only reset by XRES/POR/BOD/OVP/HIBERNATE. 0 -> default setting at POR; not for trimming use others -> normal trim range"]
    #[inline(always)]
    pub fn act_ref_abstrim(&mut self) -> ACT_REF_ABSTRIM_W {
        ACT_REF_ABSTRIM_W { w: self }
    }
    #[doc = "Bit 14 - Active-Reference current boost. This register is only reset by XRES/POR/BOD/OVP/HIBERNATE. 0: normal operation others: risk mitigation"]
    #[inline(always)]
    pub fn act_ref_iboost(&mut self) -> ACT_REF_IBOOST_W {
        ACT_REF_IBOOST_W { w: self }
    }
    #[doc = "Bits 16:19 - DeepSleep-Reference temperature trim. This register is only reset by XRES/POR/BOD/OVP/HIBERNATE. 0 -> default setting at POR; not for trimming use others -> normal trim range"]
    #[inline(always)]
    pub fn dpslp_ref_tctrim(&mut self) -> DPSLP_REF_TCTRIM_W {
        DPSLP_REF_TCTRIM_W { w: self }
    }
    #[doc = "Bits 20:24 - DeepSleep-Reference absolute voltage trim. This register is only reset by XRES/POR/BOD/OVP/HIBERNATE."]
    #[inline(always)]
    pub fn dpslp_ref_abstrim(&mut self) -> DPSLP_REF_ABSTRIM_W {
        DPSLP_REF_ABSTRIM_W { w: self }
    }
    #[doc = "Bits 28:31 - DeepSleep current reference trim. This register is only reset by XRES/POR/BOD/OVP/HIBERNATE."]
    #[inline(always)]
    pub fn dpslp_ref_itrim(&mut self) -> DPSLP_REF_ITRIM_W {
        DPSLP_REF_ITRIM_W { w: self }
    }
}
