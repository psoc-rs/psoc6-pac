#[doc = "Register `INTR_LVL_SEL` reader"]
pub struct R(crate::R<INTR_LVL_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_LVL_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_LVL_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_LVL_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTR_LVL_SEL` writer"]
pub struct W(crate::W<INTR_LVL_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTR_LVL_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<INTR_LVL_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTR_LVL_SEL_SPEC>) -> Self {
        W(writer)
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
#[doc = "Field `SOF_LVL_SEL` reader - USB SOF Interrupt level select"]
pub type SOF_LVL_SEL_R = crate::FieldReader<u8, SOF_LVL_SEL_A>;
impl SOF_LVL_SEL_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `SOF_LVL_SEL` writer - USB SOF Interrupt level select"]
pub type SOF_LVL_SEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, INTR_LVL_SEL_SPEC, u8, SOF_LVL_SEL_A, 2, O>;
impl<'a, const O: u8> SOF_LVL_SEL_W<'a, O> {
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
}
#[doc = "Field `BUS_RESET_LVL_SEL` reader - BUS RESET Interrupt level select"]
pub type BUS_RESET_LVL_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BUS_RESET_LVL_SEL` writer - BUS RESET Interrupt level select"]
pub type BUS_RESET_LVL_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, INTR_LVL_SEL_SPEC, u8, u8, 2, O>;
#[doc = "Field `EP0_LVL_SEL` reader - EP0 Interrupt level select"]
pub type EP0_LVL_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EP0_LVL_SEL` writer - EP0 Interrupt level select"]
pub type EP0_LVL_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, INTR_LVL_SEL_SPEC, u8, u8, 2, O>;
#[doc = "Field `LPM_LVL_SEL` reader - LPM Interrupt level select"]
pub type LPM_LVL_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LPM_LVL_SEL` writer - LPM Interrupt level select"]
pub type LPM_LVL_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, INTR_LVL_SEL_SPEC, u8, u8, 2, O>;
#[doc = "Field `RESUME_LVL_SEL` reader - Resume Interrupt level select"]
pub type RESUME_LVL_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESUME_LVL_SEL` writer - Resume Interrupt level select"]
pub type RESUME_LVL_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, INTR_LVL_SEL_SPEC, u8, u8, 2, O>;
#[doc = "Field `ARB_EP_LVL_SEL` reader - Arbiter Endpoint Interrupt level select"]
pub type ARB_EP_LVL_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ARB_EP_LVL_SEL` writer - Arbiter Endpoint Interrupt level select"]
pub type ARB_EP_LVL_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, INTR_LVL_SEL_SPEC, u8, u8, 2, O>;
#[doc = "Field `EP1_LVL_SEL` reader - EP1 Interrupt level select"]
pub type EP1_LVL_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EP1_LVL_SEL` writer - EP1 Interrupt level select"]
pub type EP1_LVL_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, INTR_LVL_SEL_SPEC, u8, u8, 2, O>;
#[doc = "Field `EP2_LVL_SEL` reader - EP2 Interrupt level select"]
pub type EP2_LVL_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EP2_LVL_SEL` writer - EP2 Interrupt level select"]
pub type EP2_LVL_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, INTR_LVL_SEL_SPEC, u8, u8, 2, O>;
#[doc = "Field `EP3_LVL_SEL` reader - EP3 Interrupt level select"]
pub type EP3_LVL_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EP3_LVL_SEL` writer - EP3 Interrupt level select"]
pub type EP3_LVL_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, INTR_LVL_SEL_SPEC, u8, u8, 2, O>;
#[doc = "Field `EP4_LVL_SEL` reader - EP4 Interrupt level select"]
pub type EP4_LVL_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EP4_LVL_SEL` writer - EP4 Interrupt level select"]
pub type EP4_LVL_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, INTR_LVL_SEL_SPEC, u8, u8, 2, O>;
#[doc = "Field `EP5_LVL_SEL` reader - EP5 Interrupt level select"]
pub type EP5_LVL_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EP5_LVL_SEL` writer - EP5 Interrupt level select"]
pub type EP5_LVL_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, INTR_LVL_SEL_SPEC, u8, u8, 2, O>;
#[doc = "Field `EP6_LVL_SEL` reader - EP6 Interrupt level select"]
pub type EP6_LVL_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EP6_LVL_SEL` writer - EP6 Interrupt level select"]
pub type EP6_LVL_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, INTR_LVL_SEL_SPEC, u8, u8, 2, O>;
#[doc = "Field `EP7_LVL_SEL` reader - EP7 Interrupt level select"]
pub type EP7_LVL_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EP7_LVL_SEL` writer - EP7 Interrupt level select"]
pub type EP7_LVL_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, INTR_LVL_SEL_SPEC, u8, u8, 2, O>;
#[doc = "Field `EP8_LVL_SEL` reader - EP8 Interrupt level select"]
pub type EP8_LVL_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EP8_LVL_SEL` writer - EP8 Interrupt level select"]
pub type EP8_LVL_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, INTR_LVL_SEL_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - USB SOF Interrupt level select"]
    #[inline(always)]
    pub fn sof_lvl_sel(&self) -> SOF_LVL_SEL_R {
        SOF_LVL_SEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - BUS RESET Interrupt level select"]
    #[inline(always)]
    pub fn bus_reset_lvl_sel(&self) -> BUS_RESET_LVL_SEL_R {
        BUS_RESET_LVL_SEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - EP0 Interrupt level select"]
    #[inline(always)]
    pub fn ep0_lvl_sel(&self) -> EP0_LVL_SEL_R {
        EP0_LVL_SEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - LPM Interrupt level select"]
    #[inline(always)]
    pub fn lpm_lvl_sel(&self) -> LPM_LVL_SEL_R {
        LPM_LVL_SEL_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Resume Interrupt level select"]
    #[inline(always)]
    pub fn resume_lvl_sel(&self) -> RESUME_LVL_SEL_R {
        RESUME_LVL_SEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Arbiter Endpoint Interrupt level select"]
    #[inline(always)]
    pub fn arb_ep_lvl_sel(&self) -> ARB_EP_LVL_SEL_R {
        ARB_EP_LVL_SEL_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - EP1 Interrupt level select"]
    #[inline(always)]
    pub fn ep1_lvl_sel(&self) -> EP1_LVL_SEL_R {
        EP1_LVL_SEL_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - EP2 Interrupt level select"]
    #[inline(always)]
    pub fn ep2_lvl_sel(&self) -> EP2_LVL_SEL_R {
        EP2_LVL_SEL_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - EP3 Interrupt level select"]
    #[inline(always)]
    pub fn ep3_lvl_sel(&self) -> EP3_LVL_SEL_R {
        EP3_LVL_SEL_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - EP4 Interrupt level select"]
    #[inline(always)]
    pub fn ep4_lvl_sel(&self) -> EP4_LVL_SEL_R {
        EP4_LVL_SEL_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - EP5 Interrupt level select"]
    #[inline(always)]
    pub fn ep5_lvl_sel(&self) -> EP5_LVL_SEL_R {
        EP5_LVL_SEL_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - EP6 Interrupt level select"]
    #[inline(always)]
    pub fn ep6_lvl_sel(&self) -> EP6_LVL_SEL_R {
        EP6_LVL_SEL_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - EP7 Interrupt level select"]
    #[inline(always)]
    pub fn ep7_lvl_sel(&self) -> EP7_LVL_SEL_R {
        EP7_LVL_SEL_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - EP8 Interrupt level select"]
    #[inline(always)]
    pub fn ep8_lvl_sel(&self) -> EP8_LVL_SEL_R {
        EP8_LVL_SEL_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - USB SOF Interrupt level select"]
    #[inline(always)]
    pub fn sof_lvl_sel(&mut self) -> SOF_LVL_SEL_W<0> {
        SOF_LVL_SEL_W::new(self)
    }
    #[doc = "Bits 2:3 - BUS RESET Interrupt level select"]
    #[inline(always)]
    pub fn bus_reset_lvl_sel(&mut self) -> BUS_RESET_LVL_SEL_W<2> {
        BUS_RESET_LVL_SEL_W::new(self)
    }
    #[doc = "Bits 4:5 - EP0 Interrupt level select"]
    #[inline(always)]
    pub fn ep0_lvl_sel(&mut self) -> EP0_LVL_SEL_W<4> {
        EP0_LVL_SEL_W::new(self)
    }
    #[doc = "Bits 6:7 - LPM Interrupt level select"]
    #[inline(always)]
    pub fn lpm_lvl_sel(&mut self) -> LPM_LVL_SEL_W<6> {
        LPM_LVL_SEL_W::new(self)
    }
    #[doc = "Bits 8:9 - Resume Interrupt level select"]
    #[inline(always)]
    pub fn resume_lvl_sel(&mut self) -> RESUME_LVL_SEL_W<8> {
        RESUME_LVL_SEL_W::new(self)
    }
    #[doc = "Bits 14:15 - Arbiter Endpoint Interrupt level select"]
    #[inline(always)]
    pub fn arb_ep_lvl_sel(&mut self) -> ARB_EP_LVL_SEL_W<14> {
        ARB_EP_LVL_SEL_W::new(self)
    }
    #[doc = "Bits 16:17 - EP1 Interrupt level select"]
    #[inline(always)]
    pub fn ep1_lvl_sel(&mut self) -> EP1_LVL_SEL_W<16> {
        EP1_LVL_SEL_W::new(self)
    }
    #[doc = "Bits 18:19 - EP2 Interrupt level select"]
    #[inline(always)]
    pub fn ep2_lvl_sel(&mut self) -> EP2_LVL_SEL_W<18> {
        EP2_LVL_SEL_W::new(self)
    }
    #[doc = "Bits 20:21 - EP3 Interrupt level select"]
    #[inline(always)]
    pub fn ep3_lvl_sel(&mut self) -> EP3_LVL_SEL_W<20> {
        EP3_LVL_SEL_W::new(self)
    }
    #[doc = "Bits 22:23 - EP4 Interrupt level select"]
    #[inline(always)]
    pub fn ep4_lvl_sel(&mut self) -> EP4_LVL_SEL_W<22> {
        EP4_LVL_SEL_W::new(self)
    }
    #[doc = "Bits 24:25 - EP5 Interrupt level select"]
    #[inline(always)]
    pub fn ep5_lvl_sel(&mut self) -> EP5_LVL_SEL_W<24> {
        EP5_LVL_SEL_W::new(self)
    }
    #[doc = "Bits 26:27 - EP6 Interrupt level select"]
    #[inline(always)]
    pub fn ep6_lvl_sel(&mut self) -> EP6_LVL_SEL_W<26> {
        EP6_LVL_SEL_W::new(self)
    }
    #[doc = "Bits 28:29 - EP7 Interrupt level select"]
    #[inline(always)]
    pub fn ep7_lvl_sel(&mut self) -> EP7_LVL_SEL_W<28> {
        EP7_LVL_SEL_W::new(self)
    }
    #[doc = "Bits 30:31 - EP8 Interrupt level select"]
    #[inline(always)]
    pub fn ep8_lvl_sel(&mut self) -> EP8_LVL_SEL_W<30> {
        EP8_LVL_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Select interrupt level for each interrupt source\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_lvl_sel](index.html) module"]
pub struct INTR_LVL_SEL_SPEC;
impl crate::RegisterSpec for INTR_LVL_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_lvl_sel::R](R) reader structure"]
impl crate::Readable for INTR_LVL_SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intr_lvl_sel::W](W) writer structure"]
impl crate::Writable for INTR_LVL_SEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTR_LVL_SEL to value 0"]
impl crate::Resettable for INTR_LVL_SEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
