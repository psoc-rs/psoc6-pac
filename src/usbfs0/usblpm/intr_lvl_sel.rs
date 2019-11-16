#[doc = "Reader of register INTR_LVL_SEL"]
pub type R = crate::R<u32, super::INTR_LVL_SEL>;
#[doc = "Writer for register INTR_LVL_SEL"]
pub type W = crate::W<u32, super::INTR_LVL_SEL>;
#[doc = "Register INTR_LVL_SEL `reset()`'s with value 0"]
impl crate::ResetValue for super::INTR_LVL_SEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "USB SOF Interrupt level select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SOF_LVL_SEL_A {
    #[doc = "0: High priority interrupt"]
    HI = 0,
    #[doc = "1: Medium priority interrupt"]
    MED = 1,
    #[doc = "2: Low priority interrupt"]
    LO = 2,
    #[doc = "3: illegal"]
    RSVD = 3,
}
impl From<SOF_LVL_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SOF_LVL_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SOF_LVL_SEL`"]
pub type SOF_LVL_SEL_R = crate::R<u8, SOF_LVL_SEL_A>;
impl SOF_LVL_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOF_LVL_SEL_A {
        match self.bits {
            0 => SOF_LVL_SEL_A::HI,
            1 => SOF_LVL_SEL_A::MED,
            2 => SOF_LVL_SEL_A::LO,
            3 => SOF_LVL_SEL_A::RSVD,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `HI`"]
    #[inline(always)]
    pub fn is_hi(&self) -> bool {
        *self == SOF_LVL_SEL_A::HI
    }
    #[doc = "Checks if the value of the field is `MED`"]
    #[inline(always)]
    pub fn is_med(&self) -> bool {
        *self == SOF_LVL_SEL_A::MED
    }
    #[doc = "Checks if the value of the field is `LO`"]
    #[inline(always)]
    pub fn is_lo(&self) -> bool {
        *self == SOF_LVL_SEL_A::LO
    }
    #[doc = "Checks if the value of the field is `RSVD`"]
    #[inline(always)]
    pub fn is_rsvd(&self) -> bool {
        *self == SOF_LVL_SEL_A::RSVD
    }
}
#[doc = "Write proxy for field `SOF_LVL_SEL`"]
pub struct SOF_LVL_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SOF_LVL_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SOF_LVL_SEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "High priority interrupt"]
    #[inline(always)]
    pub fn hi(self) -> &'a mut W {
        self.variant(SOF_LVL_SEL_A::HI)
    }
    #[doc = "Medium priority interrupt"]
    #[inline(always)]
    pub fn med(self) -> &'a mut W {
        self.variant(SOF_LVL_SEL_A::MED)
    }
    #[doc = "Low priority interrupt"]
    #[inline(always)]
    pub fn lo(self) -> &'a mut W {
        self.variant(SOF_LVL_SEL_A::LO)
    }
    #[doc = "illegal"]
    #[inline(always)]
    pub fn rsvd(self) -> &'a mut W {
        self.variant(SOF_LVL_SEL_A::RSVD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `BUS_RESET_LVL_SEL`"]
pub type BUS_RESET_LVL_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BUS_RESET_LVL_SEL`"]
pub struct BUS_RESET_LVL_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> BUS_RESET_LVL_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `EP0_LVL_SEL`"]
pub type EP0_LVL_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EP0_LVL_SEL`"]
pub struct EP0_LVL_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> EP0_LVL_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `LPM_LVL_SEL`"]
pub type LPM_LVL_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LPM_LVL_SEL`"]
pub struct LPM_LVL_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> LPM_LVL_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `RESUME_LVL_SEL`"]
pub type RESUME_LVL_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESUME_LVL_SEL`"]
pub struct RESUME_LVL_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RESUME_LVL_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `ARB_EP_LVL_SEL`"]
pub type ARB_EP_LVL_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ARB_EP_LVL_SEL`"]
pub struct ARB_EP_LVL_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ARB_EP_LVL_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Reader of field `EP1_LVL_SEL`"]
pub type EP1_LVL_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EP1_LVL_SEL`"]
pub struct EP1_LVL_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> EP1_LVL_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `EP2_LVL_SEL`"]
pub type EP2_LVL_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EP2_LVL_SEL`"]
pub struct EP2_LVL_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> EP2_LVL_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "Reader of field `EP3_LVL_SEL`"]
pub type EP3_LVL_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EP3_LVL_SEL`"]
pub struct EP3_LVL_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> EP3_LVL_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Reader of field `EP4_LVL_SEL`"]
pub type EP4_LVL_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EP4_LVL_SEL`"]
pub struct EP4_LVL_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> EP4_LVL_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Reader of field `EP5_LVL_SEL`"]
pub type EP5_LVL_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EP5_LVL_SEL`"]
pub struct EP5_LVL_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> EP5_LVL_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Reader of field `EP6_LVL_SEL`"]
pub type EP6_LVL_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EP6_LVL_SEL`"]
pub struct EP6_LVL_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> EP6_LVL_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
        self.w
    }
}
#[doc = "Reader of field `EP7_LVL_SEL`"]
pub type EP7_LVL_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EP7_LVL_SEL`"]
pub struct EP7_LVL_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> EP7_LVL_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
#[doc = "Reader of field `EP8_LVL_SEL`"]
pub type EP8_LVL_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EP8_LVL_SEL`"]
pub struct EP8_LVL_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> EP8_LVL_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - USB SOF Interrupt level select"]
    #[inline(always)]
    pub fn sof_lvl_sel(&self) -> SOF_LVL_SEL_R {
        SOF_LVL_SEL_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - BUS RESET Interrupt level select"]
    #[inline(always)]
    pub fn bus_reset_lvl_sel(&self) -> BUS_RESET_LVL_SEL_R {
        BUS_RESET_LVL_SEL_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - EP0 Interrupt level select"]
    #[inline(always)]
    pub fn ep0_lvl_sel(&self) -> EP0_LVL_SEL_R {
        EP0_LVL_SEL_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - LPM Interrupt level select"]
    #[inline(always)]
    pub fn lpm_lvl_sel(&self) -> LPM_LVL_SEL_R {
        LPM_LVL_SEL_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Resume Interrupt level select"]
    #[inline(always)]
    pub fn resume_lvl_sel(&self) -> RESUME_LVL_SEL_R {
        RESUME_LVL_SEL_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Arbiter Endpoint Interrupt level select"]
    #[inline(always)]
    pub fn arb_ep_lvl_sel(&self) -> ARB_EP_LVL_SEL_R {
        ARB_EP_LVL_SEL_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - EP1 Interrupt level select"]
    #[inline(always)]
    pub fn ep1_lvl_sel(&self) -> EP1_LVL_SEL_R {
        EP1_LVL_SEL_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - EP2 Interrupt level select"]
    #[inline(always)]
    pub fn ep2_lvl_sel(&self) -> EP2_LVL_SEL_R {
        EP2_LVL_SEL_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - EP3 Interrupt level select"]
    #[inline(always)]
    pub fn ep3_lvl_sel(&self) -> EP3_LVL_SEL_R {
        EP3_LVL_SEL_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - EP4 Interrupt level select"]
    #[inline(always)]
    pub fn ep4_lvl_sel(&self) -> EP4_LVL_SEL_R {
        EP4_LVL_SEL_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - EP5 Interrupt level select"]
    #[inline(always)]
    pub fn ep5_lvl_sel(&self) -> EP5_LVL_SEL_R {
        EP5_LVL_SEL_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - EP6 Interrupt level select"]
    #[inline(always)]
    pub fn ep6_lvl_sel(&self) -> EP6_LVL_SEL_R {
        EP6_LVL_SEL_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - EP7 Interrupt level select"]
    #[inline(always)]
    pub fn ep7_lvl_sel(&self) -> EP7_LVL_SEL_R {
        EP7_LVL_SEL_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 30:31 - EP8 Interrupt level select"]
    #[inline(always)]
    pub fn ep8_lvl_sel(&self) -> EP8_LVL_SEL_R {
        EP8_LVL_SEL_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - USB SOF Interrupt level select"]
    #[inline(always)]
    pub fn sof_lvl_sel(&mut self) -> SOF_LVL_SEL_W {
        SOF_LVL_SEL_W { w: self }
    }
    #[doc = "Bits 2:3 - BUS RESET Interrupt level select"]
    #[inline(always)]
    pub fn bus_reset_lvl_sel(&mut self) -> BUS_RESET_LVL_SEL_W {
        BUS_RESET_LVL_SEL_W { w: self }
    }
    #[doc = "Bits 4:5 - EP0 Interrupt level select"]
    #[inline(always)]
    pub fn ep0_lvl_sel(&mut self) -> EP0_LVL_SEL_W {
        EP0_LVL_SEL_W { w: self }
    }
    #[doc = "Bits 6:7 - LPM Interrupt level select"]
    #[inline(always)]
    pub fn lpm_lvl_sel(&mut self) -> LPM_LVL_SEL_W {
        LPM_LVL_SEL_W { w: self }
    }
    #[doc = "Bits 8:9 - Resume Interrupt level select"]
    #[inline(always)]
    pub fn resume_lvl_sel(&mut self) -> RESUME_LVL_SEL_W {
        RESUME_LVL_SEL_W { w: self }
    }
    #[doc = "Bits 14:15 - Arbiter Endpoint Interrupt level select"]
    #[inline(always)]
    pub fn arb_ep_lvl_sel(&mut self) -> ARB_EP_LVL_SEL_W {
        ARB_EP_LVL_SEL_W { w: self }
    }
    #[doc = "Bits 16:17 - EP1 Interrupt level select"]
    #[inline(always)]
    pub fn ep1_lvl_sel(&mut self) -> EP1_LVL_SEL_W {
        EP1_LVL_SEL_W { w: self }
    }
    #[doc = "Bits 18:19 - EP2 Interrupt level select"]
    #[inline(always)]
    pub fn ep2_lvl_sel(&mut self) -> EP2_LVL_SEL_W {
        EP2_LVL_SEL_W { w: self }
    }
    #[doc = "Bits 20:21 - EP3 Interrupt level select"]
    #[inline(always)]
    pub fn ep3_lvl_sel(&mut self) -> EP3_LVL_SEL_W {
        EP3_LVL_SEL_W { w: self }
    }
    #[doc = "Bits 22:23 - EP4 Interrupt level select"]
    #[inline(always)]
    pub fn ep4_lvl_sel(&mut self) -> EP4_LVL_SEL_W {
        EP4_LVL_SEL_W { w: self }
    }
    #[doc = "Bits 24:25 - EP5 Interrupt level select"]
    #[inline(always)]
    pub fn ep5_lvl_sel(&mut self) -> EP5_LVL_SEL_W {
        EP5_LVL_SEL_W { w: self }
    }
    #[doc = "Bits 26:27 - EP6 Interrupt level select"]
    #[inline(always)]
    pub fn ep6_lvl_sel(&mut self) -> EP6_LVL_SEL_W {
        EP6_LVL_SEL_W { w: self }
    }
    #[doc = "Bits 28:29 - EP7 Interrupt level select"]
    #[inline(always)]
    pub fn ep7_lvl_sel(&mut self) -> EP7_LVL_SEL_W {
        EP7_LVL_SEL_W { w: self }
    }
    #[doc = "Bits 30:31 - EP8 Interrupt level select"]
    #[inline(always)]
    pub fn ep8_lvl_sel(&mut self) -> EP8_LVL_SEL_W {
        EP8_LVL_SEL_W { w: self }
    }
}
