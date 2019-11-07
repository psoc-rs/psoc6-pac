#[doc = "Reader of register ALM2_DATE"]
pub type R = crate::R<u32, super::ALM2_DATE>;
#[doc = "Writer for register ALM2_DATE"]
pub type W = crate::W<u32, super::ALM2_DATE>;
#[doc = "Register ALM2_DATE `reset()`'s with value 0x0101"]
impl crate::ResetValue for super::ALM2_DATE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0101
    }
}
#[doc = "Reader of field `ALM_DATE`"]
pub type ALM_DATE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ALM_DATE`"]
pub struct ALM_DATE_W<'a> {
    w: &'a mut W,
}
impl<'a> ALM_DATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Reader of field `ALM_DATE_EN`"]
pub type ALM_DATE_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ALM_DATE_EN`"]
pub struct ALM_DATE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ALM_DATE_EN_W<'a> {
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
#[doc = "Reader of field `ALM_MON`"]
pub type ALM_MON_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ALM_MON`"]
pub struct ALM_MON_W<'a> {
    w: &'a mut W,
}
impl<'a> ALM_MON_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "Reader of field `ALM_MON_EN`"]
pub type ALM_MON_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ALM_MON_EN`"]
pub struct ALM_MON_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ALM_MON_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `ALM_EN`"]
pub type ALM_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ALM_EN`"]
pub struct ALM_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ALM_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Alarm Day of the Month in BCD, 1-31 Leap Year corrected"]
    #[inline(always)]
    pub fn alm_date(&self) -> ALM_DATE_R {
        ALM_DATE_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 7 - Alarm Day of the Month enable: 0=ignore, 1=match"]
    #[inline(always)]
    pub fn alm_date_en(&self) -> ALM_DATE_EN_R {
        ALM_DATE_EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:12 - Alarm Month in BCD, 1-12"]
    #[inline(always)]
    pub fn alm_mon(&self) -> ALM_MON_R {
        ALM_MON_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 15 - Alarm Month enable: 0=ignore, 1=match"]
    #[inline(always)]
    pub fn alm_mon_en(&self) -> ALM_MON_EN_R {
        ALM_MON_EN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Master enable for alarm 2. 0: Alarm 2 is disabled. Fields for date and time are ignored. 1: Alarm 2 is enabled. If none of the date and time fields are enabled, then this alarm triggers once every second."]
    #[inline(always)]
    pub fn alm_en(&self) -> ALM_EN_R {
        ALM_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Alarm Day of the Month in BCD, 1-31 Leap Year corrected"]
    #[inline(always)]
    pub fn alm_date(&mut self) -> ALM_DATE_W {
        ALM_DATE_W { w: self }
    }
    #[doc = "Bit 7 - Alarm Day of the Month enable: 0=ignore, 1=match"]
    #[inline(always)]
    pub fn alm_date_en(&mut self) -> ALM_DATE_EN_W {
        ALM_DATE_EN_W { w: self }
    }
    #[doc = "Bits 8:12 - Alarm Month in BCD, 1-12"]
    #[inline(always)]
    pub fn alm_mon(&mut self) -> ALM_MON_W {
        ALM_MON_W { w: self }
    }
    #[doc = "Bit 15 - Alarm Month enable: 0=ignore, 1=match"]
    #[inline(always)]
    pub fn alm_mon_en(&mut self) -> ALM_MON_EN_W {
        ALM_MON_EN_W { w: self }
    }
    #[doc = "Bit 31 - Master enable for alarm 2. 0: Alarm 2 is disabled. Fields for date and time are ignored. 1: Alarm 2 is enabled. If none of the date and time fields are enabled, then this alarm triggers once every second."]
    #[inline(always)]
    pub fn alm_en(&mut self) -> ALM_EN_W {
        ALM_EN_W { w: self }
    }
}
