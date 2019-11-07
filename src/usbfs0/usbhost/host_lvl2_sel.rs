#[doc = "Reader of register HOST_LVL2_SEL"]
pub type R = crate::R<u32, super::HOST_LVL2_SEL>;
#[doc = "Writer for register HOST_LVL2_SEL"]
pub type W = crate::W<u32, super::HOST_LVL2_SEL>;
#[doc = "Register HOST_LVL2_SEL `reset()`'s with value 0"]
impl crate::ResetValue for super::HOST_LVL2_SEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "These bits assign EP1_DRQ interrupt flag to any interrupt signals.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EP1_DRQ_SEL_A {
    #[doc = "0: High priority interrupt"]
    HI,
    #[doc = "1: Medium priority interrupt"]
    MED,
    #[doc = "2: Low priority interrupt"]
    LO,
    #[doc = "3: illegal"]
    RSVD,
}
impl From<EP1_DRQ_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: EP1_DRQ_SEL_A) -> Self {
        match variant {
            EP1_DRQ_SEL_A::HI => 0,
            EP1_DRQ_SEL_A::MED => 1,
            EP1_DRQ_SEL_A::LO => 2,
            EP1_DRQ_SEL_A::RSVD => 3,
        }
    }
}
#[doc = "Reader of field `EP1_DRQ_SEL`"]
pub type EP1_DRQ_SEL_R = crate::R<u8, EP1_DRQ_SEL_A>;
impl EP1_DRQ_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EP1_DRQ_SEL_A {
        match self.bits {
            0 => EP1_DRQ_SEL_A::HI,
            1 => EP1_DRQ_SEL_A::MED,
            2 => EP1_DRQ_SEL_A::LO,
            3 => EP1_DRQ_SEL_A::RSVD,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `HI`"]
    #[inline(always)]
    pub fn is_hi(&self) -> bool {
        *self == EP1_DRQ_SEL_A::HI
    }
    #[doc = "Checks if the value of the field is `MED`"]
    #[inline(always)]
    pub fn is_med(&self) -> bool {
        *self == EP1_DRQ_SEL_A::MED
    }
    #[doc = "Checks if the value of the field is `LO`"]
    #[inline(always)]
    pub fn is_lo(&self) -> bool {
        *self == EP1_DRQ_SEL_A::LO
    }
    #[doc = "Checks if the value of the field is `RSVD`"]
    #[inline(always)]
    pub fn is_rsvd(&self) -> bool {
        *self == EP1_DRQ_SEL_A::RSVD
    }
}
#[doc = "Write proxy for field `EP1_DRQ_SEL`"]
pub struct EP1_DRQ_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> EP1_DRQ_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EP1_DRQ_SEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "High priority interrupt"]
    #[inline(always)]
    pub fn hi(self) -> &'a mut W {
        self.variant(EP1_DRQ_SEL_A::HI)
    }
    #[doc = "Medium priority interrupt"]
    #[inline(always)]
    pub fn med(self) -> &'a mut W {
        self.variant(EP1_DRQ_SEL_A::MED)
    }
    #[doc = "Low priority interrupt"]
    #[inline(always)]
    pub fn lo(self) -> &'a mut W {
        self.variant(EP1_DRQ_SEL_A::LO)
    }
    #[doc = "illegal"]
    #[inline(always)]
    pub fn rsvd(self) -> &'a mut W {
        self.variant(EP1_DRQ_SEL_A::RSVD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `EP1_SPK_SEL`"]
pub type EP1_SPK_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EP1_SPK_SEL`"]
pub struct EP1_SPK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> EP1_SPK_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `EP2_DRQ_SEL`"]
pub type EP2_DRQ_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EP2_DRQ_SEL`"]
pub struct EP2_DRQ_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> EP2_DRQ_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `EP2_SPK_SEL`"]
pub type EP2_SPK_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EP2_SPK_SEL`"]
pub struct EP2_SPK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> EP2_SPK_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bits 4:5 - These bits assign EP1_DRQ interrupt flag to any interrupt signals."]
    #[inline(always)]
    pub fn ep1_drq_sel(&self) -> EP1_DRQ_SEL_R {
        EP1_DRQ_SEL_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - These bits assign EP1_SPK interrupt flag to any interrupt signals."]
    #[inline(always)]
    pub fn ep1_spk_sel(&self) -> EP1_SPK_SEL_R {
        EP1_SPK_SEL_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - These bits assign EP2_DRQ interrupt flag to any interrupt signals."]
    #[inline(always)]
    pub fn ep2_drq_sel(&self) -> EP2_DRQ_SEL_R {
        EP2_DRQ_SEL_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - These bits assign EP2_SPK interrupt flag to any interrupt signals."]
    #[inline(always)]
    pub fn ep2_spk_sel(&self) -> EP2_SPK_SEL_R {
        EP2_SPK_SEL_R::new(((self.bits >> 10) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 4:5 - These bits assign EP1_DRQ interrupt flag to any interrupt signals."]
    #[inline(always)]
    pub fn ep1_drq_sel(&mut self) -> EP1_DRQ_SEL_W {
        EP1_DRQ_SEL_W { w: self }
    }
    #[doc = "Bits 6:7 - These bits assign EP1_SPK interrupt flag to any interrupt signals."]
    #[inline(always)]
    pub fn ep1_spk_sel(&mut self) -> EP1_SPK_SEL_W {
        EP1_SPK_SEL_W { w: self }
    }
    #[doc = "Bits 8:9 - These bits assign EP2_DRQ interrupt flag to any interrupt signals."]
    #[inline(always)]
    pub fn ep2_drq_sel(&mut self) -> EP2_DRQ_SEL_W {
        EP2_DRQ_SEL_W { w: self }
    }
    #[doc = "Bits 10:11 - These bits assign EP2_SPK interrupt flag to any interrupt signals."]
    #[inline(always)]
    pub fn ep2_spk_sel(&mut self) -> EP2_SPK_SEL_W {
        EP2_SPK_SEL_W { w: self }
    }
}
