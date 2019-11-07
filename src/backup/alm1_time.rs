#[doc = "Reader of register ALM1_TIME"]
pub type R = crate::R<u32, super::ALM1_TIME>;
#[doc = "Writer for register ALM1_TIME"]
pub type W = crate::W<u32, super::ALM1_TIME>;
#[doc = "Register ALM1_TIME `reset()`'s with value 0x0100_0000"]
impl crate::ResetValue for super::ALM1_TIME {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0100_0000
    }
}
#[doc = "Reader of field `ALM_SEC`"]
pub type ALM_SEC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ALM_SEC`"]
pub struct ALM_SEC_W<'a> {
    w: &'a mut W,
}
impl<'a> ALM_SEC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `ALM_SEC_EN`"]
pub type ALM_SEC_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ALM_SEC_EN`"]
pub struct ALM_SEC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ALM_SEC_EN_W<'a> {
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
#[doc = "Reader of field `ALM_MIN`"]
pub type ALM_MIN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ALM_MIN`"]
pub struct ALM_MIN_W<'a> {
    w: &'a mut W,
}
impl<'a> ALM_MIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u32) & 0x7f) << 8);
        self.w
    }
}
#[doc = "Reader of field `ALM_MIN_EN`"]
pub type ALM_MIN_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ALM_MIN_EN`"]
pub struct ALM_MIN_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ALM_MIN_EN_W<'a> {
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
#[doc = "Reader of field `ALM_HOUR`"]
pub type ALM_HOUR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ALM_HOUR`"]
pub struct ALM_HOUR_W<'a> {
    w: &'a mut W,
}
impl<'a> ALM_HOUR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | (((value as u32) & 0x3f) << 16);
        self.w
    }
}
#[doc = "Reader of field `ALM_HOUR_EN`"]
pub type ALM_HOUR_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ALM_HOUR_EN`"]
pub struct ALM_HOUR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ALM_HOUR_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `ALM_DAY`"]
pub type ALM_DAY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ALM_DAY`"]
pub struct ALM_DAY_W<'a> {
    w: &'a mut W,
}
impl<'a> ALM_DAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
#[doc = "Reader of field `ALM_DAY_EN`"]
pub type ALM_DAY_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ALM_DAY_EN`"]
pub struct ALM_DAY_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ALM_DAY_EN_W<'a> {
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
    #[doc = "Bits 0:6 - Alarm seconds in BCD, 0-59"]
    #[inline(always)]
    pub fn alm_sec(&self) -> ALM_SEC_R {
        ALM_SEC_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Alarm second enable: 0=ignore, 1=match"]
    #[inline(always)]
    pub fn alm_sec_en(&self) -> ALM_SEC_EN_R {
        ALM_SEC_EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:14 - Alarm minutes in BCD, 0-59"]
    #[inline(always)]
    pub fn alm_min(&self) -> ALM_MIN_R {
        ALM_MIN_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - Alarm minutes enable: 0=ignore, 1=match"]
    #[inline(always)]
    pub fn alm_min_en(&self) -> ALM_MIN_EN_R {
        ALM_MIN_EN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:21 - Alarm hours in BCD, value depending on 12/24HR mode 12HR: \\[5\\]:0=AM, 1=PM, \\[4:0\\]=1-12 24HR: \\[5:0\\]=0-23"]
    #[inline(always)]
    pub fn alm_hour(&self) -> ALM_HOUR_R {
        ALM_HOUR_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 23 - Alarm hour enable: 0=ignore, 1=match"]
    #[inline(always)]
    pub fn alm_hour_en(&self) -> ALM_HOUR_EN_R {
        ALM_HOUR_EN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:26 - Alarm Day of the week in BCD, 1-7 It is up to the user to define the meaning of the values, but 1=Monday is recommended"]
    #[inline(always)]
    pub fn alm_day(&self) -> ALM_DAY_R {
        ALM_DAY_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bit 31 - Alarm Day of the Week enable: 0=ignore, 1=match"]
    #[inline(always)]
    pub fn alm_day_en(&self) -> ALM_DAY_EN_R {
        ALM_DAY_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Alarm seconds in BCD, 0-59"]
    #[inline(always)]
    pub fn alm_sec(&mut self) -> ALM_SEC_W {
        ALM_SEC_W { w: self }
    }
    #[doc = "Bit 7 - Alarm second enable: 0=ignore, 1=match"]
    #[inline(always)]
    pub fn alm_sec_en(&mut self) -> ALM_SEC_EN_W {
        ALM_SEC_EN_W { w: self }
    }
    #[doc = "Bits 8:14 - Alarm minutes in BCD, 0-59"]
    #[inline(always)]
    pub fn alm_min(&mut self) -> ALM_MIN_W {
        ALM_MIN_W { w: self }
    }
    #[doc = "Bit 15 - Alarm minutes enable: 0=ignore, 1=match"]
    #[inline(always)]
    pub fn alm_min_en(&mut self) -> ALM_MIN_EN_W {
        ALM_MIN_EN_W { w: self }
    }
    #[doc = "Bits 16:21 - Alarm hours in BCD, value depending on 12/24HR mode 12HR: \\[5\\]:0=AM, 1=PM, \\[4:0\\]=1-12 24HR: \\[5:0\\]=0-23"]
    #[inline(always)]
    pub fn alm_hour(&mut self) -> ALM_HOUR_W {
        ALM_HOUR_W { w: self }
    }
    #[doc = "Bit 23 - Alarm hour enable: 0=ignore, 1=match"]
    #[inline(always)]
    pub fn alm_hour_en(&mut self) -> ALM_HOUR_EN_W {
        ALM_HOUR_EN_W { w: self }
    }
    #[doc = "Bits 24:26 - Alarm Day of the week in BCD, 1-7 It is up to the user to define the meaning of the values, but 1=Monday is recommended"]
    #[inline(always)]
    pub fn alm_day(&mut self) -> ALM_DAY_W {
        ALM_DAY_W { w: self }
    }
    #[doc = "Bit 31 - Alarm Day of the Week enable: 0=ignore, 1=match"]
    #[inline(always)]
    pub fn alm_day_en(&mut self) -> ALM_DAY_EN_W {
        ALM_DAY_EN_W { w: self }
    }
}
