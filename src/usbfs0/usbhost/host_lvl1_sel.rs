#[doc = "Reader of register HOST_LVL1_SEL"]
pub type R = crate::R<u32, super::HOST_LVL1_SEL>;
#[doc = "Writer for register HOST_LVL1_SEL"]
pub type W = crate::W<u32, super::HOST_LVL1_SEL>;
#[doc = "Register HOST_LVL1_SEL `reset()`'s with value 0"]
impl crate::ResetValue for super::HOST_LVL1_SEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "These bits assign SOFIRQ interrupt flag to any interrupt signals.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SOFIRQ_SEL_A {
    #[doc = "0: High priority interrupt"]
    HI = 0,
    #[doc = "1: Medium priority interrupt"]
    MED = 1,
    #[doc = "2: Low priority interrupt"]
    LO = 2,
    #[doc = "3: illegal"]
    RSVD = 3,
}
impl From<SOFIRQ_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SOFIRQ_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SOFIRQ_SEL`"]
pub type SOFIRQ_SEL_R = crate::R<u8, SOFIRQ_SEL_A>;
impl SOFIRQ_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOFIRQ_SEL_A {
        match self.bits {
            0 => SOFIRQ_SEL_A::HI,
            1 => SOFIRQ_SEL_A::MED,
            2 => SOFIRQ_SEL_A::LO,
            3 => SOFIRQ_SEL_A::RSVD,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `HI`"]
    #[inline(always)]
    pub fn is_hi(&self) -> bool {
        *self == SOFIRQ_SEL_A::HI
    }
    #[doc = "Checks if the value of the field is `MED`"]
    #[inline(always)]
    pub fn is_med(&self) -> bool {
        *self == SOFIRQ_SEL_A::MED
    }
    #[doc = "Checks if the value of the field is `LO`"]
    #[inline(always)]
    pub fn is_lo(&self) -> bool {
        *self == SOFIRQ_SEL_A::LO
    }
    #[doc = "Checks if the value of the field is `RSVD`"]
    #[inline(always)]
    pub fn is_rsvd(&self) -> bool {
        *self == SOFIRQ_SEL_A::RSVD
    }
}
#[doc = "Write proxy for field `SOFIRQ_SEL`"]
pub struct SOFIRQ_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFIRQ_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SOFIRQ_SEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "High priority interrupt"]
    #[inline(always)]
    pub fn hi(self) -> &'a mut W {
        self.variant(SOFIRQ_SEL_A::HI)
    }
    #[doc = "Medium priority interrupt"]
    #[inline(always)]
    pub fn med(self) -> &'a mut W {
        self.variant(SOFIRQ_SEL_A::MED)
    }
    #[doc = "Low priority interrupt"]
    #[inline(always)]
    pub fn lo(self) -> &'a mut W {
        self.variant(SOFIRQ_SEL_A::LO)
    }
    #[doc = "illegal"]
    #[inline(always)]
    pub fn rsvd(self) -> &'a mut W {
        self.variant(SOFIRQ_SEL_A::RSVD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `DIRQ_SEL`"]
pub type DIRQ_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DIRQ_SEL`"]
pub struct DIRQ_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRQ_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `CNNIRQ_SEL`"]
pub type CNNIRQ_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CNNIRQ_SEL`"]
pub struct CNNIRQ_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CNNIRQ_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `CMPIRQ_SEL`"]
pub type CMPIRQ_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CMPIRQ_SEL`"]
pub struct CMPIRQ_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPIRQ_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `URIRQ_SEL`"]
pub type URIRQ_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `URIRQ_SEL`"]
pub struct URIRQ_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> URIRQ_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `RWKIRQ_SEL`"]
pub type RWKIRQ_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RWKIRQ_SEL`"]
pub struct RWKIRQ_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RWKIRQ_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Reader of field `RSVD_13_12`"]
pub type RSVD_13_12_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RSVD_13_12`"]
pub struct RSVD_13_12_W<'a> {
    w: &'a mut W,
}
impl<'a> RSVD_13_12_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `TCAN_SEL`"]
pub type TCAN_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TCAN_SEL`"]
pub struct TCAN_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TCAN_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - These bits assign SOFIRQ interrupt flag to any interrupt signals."]
    #[inline(always)]
    pub fn sofirq_sel(&self) -> SOFIRQ_SEL_R {
        SOFIRQ_SEL_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - These bits assign DIRQ interrupt flag to any interrupt signals."]
    #[inline(always)]
    pub fn dirq_sel(&self) -> DIRQ_SEL_R {
        DIRQ_SEL_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - These bits assign CNNIRQ interrupt flag to any interrupt signals."]
    #[inline(always)]
    pub fn cnnirq_sel(&self) -> CNNIRQ_SEL_R {
        CNNIRQ_SEL_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - These bits assign URIRQ interrupt flag to any interrupt signals."]
    #[inline(always)]
    pub fn cmpirq_sel(&self) -> CMPIRQ_SEL_R {
        CMPIRQ_SEL_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - These bits assign URIRQ interrupt flag to any interrupt signals."]
    #[inline(always)]
    pub fn urirq_sel(&self) -> URIRQ_SEL_R {
        URIRQ_SEL_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - These bits assign RWKIRQ interrupt flag to any interrupt signals."]
    #[inline(always)]
    pub fn rwkirq_sel(&self) -> RWKIRQ_SEL_R {
        RWKIRQ_SEL_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - N/A"]
    #[inline(always)]
    pub fn rsvd_13_12(&self) -> RSVD_13_12_R {
        RSVD_13_12_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - These bits assign TCAN interrupt flag to any interrupt signals."]
    #[inline(always)]
    pub fn tcan_sel(&self) -> TCAN_SEL_R {
        TCAN_SEL_R::new(((self.bits >> 14) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - These bits assign SOFIRQ interrupt flag to any interrupt signals."]
    #[inline(always)]
    pub fn sofirq_sel(&mut self) -> SOFIRQ_SEL_W {
        SOFIRQ_SEL_W { w: self }
    }
    #[doc = "Bits 2:3 - These bits assign DIRQ interrupt flag to any interrupt signals."]
    #[inline(always)]
    pub fn dirq_sel(&mut self) -> DIRQ_SEL_W {
        DIRQ_SEL_W { w: self }
    }
    #[doc = "Bits 4:5 - These bits assign CNNIRQ interrupt flag to any interrupt signals."]
    #[inline(always)]
    pub fn cnnirq_sel(&mut self) -> CNNIRQ_SEL_W {
        CNNIRQ_SEL_W { w: self }
    }
    #[doc = "Bits 6:7 - These bits assign URIRQ interrupt flag to any interrupt signals."]
    #[inline(always)]
    pub fn cmpirq_sel(&mut self) -> CMPIRQ_SEL_W {
        CMPIRQ_SEL_W { w: self }
    }
    #[doc = "Bits 8:9 - These bits assign URIRQ interrupt flag to any interrupt signals."]
    #[inline(always)]
    pub fn urirq_sel(&mut self) -> URIRQ_SEL_W {
        URIRQ_SEL_W { w: self }
    }
    #[doc = "Bits 10:11 - These bits assign RWKIRQ interrupt flag to any interrupt signals."]
    #[inline(always)]
    pub fn rwkirq_sel(&mut self) -> RWKIRQ_SEL_W {
        RWKIRQ_SEL_W { w: self }
    }
    #[doc = "Bits 12:13 - N/A"]
    #[inline(always)]
    pub fn rsvd_13_12(&mut self) -> RSVD_13_12_W {
        RSVD_13_12_W { w: self }
    }
    #[doc = "Bits 14:15 - These bits assign TCAN interrupt flag to any interrupt signals."]
    #[inline(always)]
    pub fn tcan_sel(&mut self) -> TCAN_SEL_W {
        TCAN_SEL_W { w: self }
    }
}
