#[doc = "Reader of register SW_RES"]
pub type R = crate::R<u32, super::SW_RES>;
#[doc = "Writer for register SW_RES"]
pub type W = crate::W<u32, super::SW_RES>;
#[doc = "Register SW_RES `reset()`'s with value 0"]
impl crate::ResetValue for super::SW_RES {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Select resistance or low EMI (slow ramp) for the HCAV switch\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RES_HCAV_A {
    #[doc = "0: Low"]
    LOW = 0,
    #[doc = "1: Medium"]
    MED = 1,
    #[doc = "2: High"]
    HIGH = 2,
    #[doc = "3: Low EMI (slow ramp: 3 switches closed by fixed delay line)"]
    LOWEMI = 3,
}
impl From<RES_HCAV_A> for u8 {
    #[inline(always)]
    fn from(variant: RES_HCAV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RES_HCAV`"]
pub type RES_HCAV_R = crate::R<u8, RES_HCAV_A>;
impl RES_HCAV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RES_HCAV_A {
        match self.bits {
            0 => RES_HCAV_A::LOW,
            1 => RES_HCAV_A::MED,
            2 => RES_HCAV_A::HIGH,
            3 => RES_HCAV_A::LOWEMI,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == RES_HCAV_A::LOW
    }
    #[doc = "Checks if the value of the field is `MED`"]
    #[inline(always)]
    pub fn is_med(&self) -> bool {
        *self == RES_HCAV_A::MED
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == RES_HCAV_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOWEMI`"]
    #[inline(always)]
    pub fn is_lowemi(&self) -> bool {
        *self == RES_HCAV_A::LOWEMI
    }
}
#[doc = "Write proxy for field `RES_HCAV`"]
pub struct RES_HCAV_W<'a> {
    w: &'a mut W,
}
impl<'a> RES_HCAV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RES_HCAV_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(RES_HCAV_A::LOW)
    }
    #[doc = "Medium"]
    #[inline(always)]
    pub fn med(self) -> &'a mut W {
        self.variant(RES_HCAV_A::MED)
    }
    #[doc = "High"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(RES_HCAV_A::HIGH)
    }
    #[doc = "Low EMI (slow ramp: 3 switches closed by fixed delay line)"]
    #[inline(always)]
    pub fn lowemi(self) -> &'a mut W {
        self.variant(RES_HCAV_A::LOWEMI)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `RES_HCAG`"]
pub type RES_HCAG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RES_HCAG`"]
pub struct RES_HCAG_W<'a> {
    w: &'a mut W,
}
impl<'a> RES_HCAG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `RES_HCBV`"]
pub type RES_HCBV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RES_HCBV`"]
pub struct RES_HCBV_W<'a> {
    w: &'a mut W,
}
impl<'a> RES_HCBV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `RES_HCBG`"]
pub type RES_HCBG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RES_HCBG`"]
pub struct RES_HCBG_W<'a> {
    w: &'a mut W,
}
impl<'a> RES_HCBG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Select resistance for the corresponding switch\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RES_F1PM_A {
    #[doc = "0: Low"]
    LOW = 0,
    #[doc = "1: Medium"]
    MED = 1,
    #[doc = "2: High"]
    HIGH = 2,
    #[doc = "3: N/A"]
    RSVD = 3,
}
impl From<RES_F1PM_A> for u8 {
    #[inline(always)]
    fn from(variant: RES_F1PM_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RES_F1PM`"]
pub type RES_F1PM_R = crate::R<u8, RES_F1PM_A>;
impl RES_F1PM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RES_F1PM_A {
        match self.bits {
            0 => RES_F1PM_A::LOW,
            1 => RES_F1PM_A::MED,
            2 => RES_F1PM_A::HIGH,
            3 => RES_F1PM_A::RSVD,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == RES_F1PM_A::LOW
    }
    #[doc = "Checks if the value of the field is `MED`"]
    #[inline(always)]
    pub fn is_med(&self) -> bool {
        *self == RES_F1PM_A::MED
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == RES_F1PM_A::HIGH
    }
    #[doc = "Checks if the value of the field is `RSVD`"]
    #[inline(always)]
    pub fn is_rsvd(&self) -> bool {
        *self == RES_F1PM_A::RSVD
    }
}
#[doc = "Write proxy for field `RES_F1PM`"]
pub struct RES_F1PM_W<'a> {
    w: &'a mut W,
}
impl<'a> RES_F1PM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RES_F1PM_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(RES_F1PM_A::LOW)
    }
    #[doc = "Medium"]
    #[inline(always)]
    pub fn med(self) -> &'a mut W {
        self.variant(RES_F1PM_A::MED)
    }
    #[doc = "High"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(RES_F1PM_A::HIGH)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn rsvd(self) -> &'a mut W {
        self.variant(RES_F1PM_A::RSVD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `RES_F2PT`"]
pub type RES_F2PT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RES_F2PT`"]
pub struct RES_F2PT_W<'a> {
    w: &'a mut W,
}
impl<'a> RES_F2PT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Select resistance or low EMI (slow ramp) for the HCAV switch"]
    #[inline(always)]
    pub fn res_hcav(&self) -> RES_HCAV_R {
        RES_HCAV_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Select resistance or low EMI for the corresponding switch"]
    #[inline(always)]
    pub fn res_hcag(&self) -> RES_HCAG_R {
        RES_HCAG_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Select resistance or low EMI for the corresponding switch"]
    #[inline(always)]
    pub fn res_hcbv(&self) -> RES_HCBV_R {
        RES_HCBV_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Select resistance or low EMI for the corresponding switch"]
    #[inline(always)]
    pub fn res_hcbg(&self) -> RES_HCBG_R {
        RES_HCBG_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - Select resistance for the corresponding switch"]
    #[inline(always)]
    pub fn res_f1pm(&self) -> RES_F1PM_R {
        RES_F1PM_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - Select resistance for the corresponding switch"]
    #[inline(always)]
    pub fn res_f2pt(&self) -> RES_F2PT_R {
        RES_F2PT_R::new(((self.bits >> 18) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Select resistance or low EMI (slow ramp) for the HCAV switch"]
    #[inline(always)]
    pub fn res_hcav(&mut self) -> RES_HCAV_W {
        RES_HCAV_W { w: self }
    }
    #[doc = "Bits 2:3 - Select resistance or low EMI for the corresponding switch"]
    #[inline(always)]
    pub fn res_hcag(&mut self) -> RES_HCAG_W {
        RES_HCAG_W { w: self }
    }
    #[doc = "Bits 4:5 - Select resistance or low EMI for the corresponding switch"]
    #[inline(always)]
    pub fn res_hcbv(&mut self) -> RES_HCBV_W {
        RES_HCBV_W { w: self }
    }
    #[doc = "Bits 6:7 - Select resistance or low EMI for the corresponding switch"]
    #[inline(always)]
    pub fn res_hcbg(&mut self) -> RES_HCBG_W {
        RES_HCBG_W { w: self }
    }
    #[doc = "Bits 16:17 - Select resistance for the corresponding switch"]
    #[inline(always)]
    pub fn res_f1pm(&mut self) -> RES_F1PM_W {
        RES_F1PM_W { w: self }
    }
    #[doc = "Bits 18:19 - Select resistance for the corresponding switch"]
    #[inline(always)]
    pub fn res_f2pt(&mut self) -> RES_F2PT_W {
        RES_F2PT_W { w: self }
    }
}
