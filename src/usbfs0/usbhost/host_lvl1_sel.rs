#[doc = "Register `HOST_LVL1_SEL` reader"]
pub struct R(crate::R<HOST_LVL1_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HOST_LVL1_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HOST_LVL1_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HOST_LVL1_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HOST_LVL1_SEL` writer"]
pub struct W(crate::W<HOST_LVL1_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HOST_LVL1_SEL_SPEC>;
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
impl From<crate::W<HOST_LVL1_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HOST_LVL1_SEL_SPEC>) -> Self {
        W(writer)
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
#[doc = "Field `SOFIRQ_SEL` reader - These bits assign SOFIRQ interrupt flag to any interrupt signals."]
pub type SOFIRQ_SEL_R = crate::FieldReader<u8, SOFIRQ_SEL_A>;
impl SOFIRQ_SEL_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `SOFIRQ_SEL` writer - These bits assign SOFIRQ interrupt flag to any interrupt signals."]
pub type SOFIRQ_SEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, HOST_LVL1_SEL_SPEC, u8, SOFIRQ_SEL_A, 2, O>;
impl<'a, const O: u8> SOFIRQ_SEL_W<'a, O> {
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
}
#[doc = "Field `DIRQ_SEL` reader - These bits assign DIRQ interrupt flag to any interrupt signals."]
pub type DIRQ_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DIRQ_SEL` writer - These bits assign DIRQ interrupt flag to any interrupt signals."]
pub type DIRQ_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HOST_LVL1_SEL_SPEC, u8, u8, 2, O>;
#[doc = "Field `CNNIRQ_SEL` reader - These bits assign CNNIRQ interrupt flag to any interrupt signals."]
pub type CNNIRQ_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CNNIRQ_SEL` writer - These bits assign CNNIRQ interrupt flag to any interrupt signals."]
pub type CNNIRQ_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HOST_LVL1_SEL_SPEC, u8, u8, 2, O>;
#[doc = "Field `CMPIRQ_SEL` reader - These bits assign URIRQ interrupt flag to any interrupt signals."]
pub type CMPIRQ_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CMPIRQ_SEL` writer - These bits assign URIRQ interrupt flag to any interrupt signals."]
pub type CMPIRQ_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HOST_LVL1_SEL_SPEC, u8, u8, 2, O>;
#[doc = "Field `URIRQ_SEL` reader - These bits assign URIRQ interrupt flag to any interrupt signals."]
pub type URIRQ_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `URIRQ_SEL` writer - These bits assign URIRQ interrupt flag to any interrupt signals."]
pub type URIRQ_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HOST_LVL1_SEL_SPEC, u8, u8, 2, O>;
#[doc = "Field `RWKIRQ_SEL` reader - These bits assign RWKIRQ interrupt flag to any interrupt signals."]
pub type RWKIRQ_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RWKIRQ_SEL` writer - These bits assign RWKIRQ interrupt flag to any interrupt signals."]
pub type RWKIRQ_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HOST_LVL1_SEL_SPEC, u8, u8, 2, O>;
#[doc = "Field `RSVD_13_12` reader - N/A"]
pub type RSVD_13_12_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RSVD_13_12` writer - N/A"]
pub type RSVD_13_12_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HOST_LVL1_SEL_SPEC, u8, u8, 2, O>;
#[doc = "Field `TCAN_SEL` reader - These bits assign TCAN interrupt flag to any interrupt signals."]
pub type TCAN_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TCAN_SEL` writer - These bits assign TCAN interrupt flag to any interrupt signals."]
pub type TCAN_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HOST_LVL1_SEL_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - These bits assign SOFIRQ interrupt flag to any interrupt signals."]
    #[inline(always)]
    pub fn sofirq_sel(&self) -> SOFIRQ_SEL_R {
        SOFIRQ_SEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - These bits assign DIRQ interrupt flag to any interrupt signals."]
    #[inline(always)]
    pub fn dirq_sel(&self) -> DIRQ_SEL_R {
        DIRQ_SEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - These bits assign CNNIRQ interrupt flag to any interrupt signals."]
    #[inline(always)]
    pub fn cnnirq_sel(&self) -> CNNIRQ_SEL_R {
        CNNIRQ_SEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - These bits assign URIRQ interrupt flag to any interrupt signals."]
    #[inline(always)]
    pub fn cmpirq_sel(&self) -> CMPIRQ_SEL_R {
        CMPIRQ_SEL_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - These bits assign URIRQ interrupt flag to any interrupt signals."]
    #[inline(always)]
    pub fn urirq_sel(&self) -> URIRQ_SEL_R {
        URIRQ_SEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - These bits assign RWKIRQ interrupt flag to any interrupt signals."]
    #[inline(always)]
    pub fn rwkirq_sel(&self) -> RWKIRQ_SEL_R {
        RWKIRQ_SEL_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - N/A"]
    #[inline(always)]
    pub fn rsvd_13_12(&self) -> RSVD_13_12_R {
        RSVD_13_12_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - These bits assign TCAN interrupt flag to any interrupt signals."]
    #[inline(always)]
    pub fn tcan_sel(&self) -> TCAN_SEL_R {
        TCAN_SEL_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - These bits assign SOFIRQ interrupt flag to any interrupt signals."]
    #[inline(always)]
    pub fn sofirq_sel(&mut self) -> SOFIRQ_SEL_W<0> {
        SOFIRQ_SEL_W::new(self)
    }
    #[doc = "Bits 2:3 - These bits assign DIRQ interrupt flag to any interrupt signals."]
    #[inline(always)]
    pub fn dirq_sel(&mut self) -> DIRQ_SEL_W<2> {
        DIRQ_SEL_W::new(self)
    }
    #[doc = "Bits 4:5 - These bits assign CNNIRQ interrupt flag to any interrupt signals."]
    #[inline(always)]
    pub fn cnnirq_sel(&mut self) -> CNNIRQ_SEL_W<4> {
        CNNIRQ_SEL_W::new(self)
    }
    #[doc = "Bits 6:7 - These bits assign URIRQ interrupt flag to any interrupt signals."]
    #[inline(always)]
    pub fn cmpirq_sel(&mut self) -> CMPIRQ_SEL_W<6> {
        CMPIRQ_SEL_W::new(self)
    }
    #[doc = "Bits 8:9 - These bits assign URIRQ interrupt flag to any interrupt signals."]
    #[inline(always)]
    pub fn urirq_sel(&mut self) -> URIRQ_SEL_W<8> {
        URIRQ_SEL_W::new(self)
    }
    #[doc = "Bits 10:11 - These bits assign RWKIRQ interrupt flag to any interrupt signals."]
    #[inline(always)]
    pub fn rwkirq_sel(&mut self) -> RWKIRQ_SEL_W<10> {
        RWKIRQ_SEL_W::new(self)
    }
    #[doc = "Bits 12:13 - N/A"]
    #[inline(always)]
    pub fn rsvd_13_12(&mut self) -> RSVD_13_12_W<12> {
        RSVD_13_12_W::new(self)
    }
    #[doc = "Bits 14:15 - These bits assign TCAN interrupt flag to any interrupt signals."]
    #[inline(always)]
    pub fn tcan_sel(&mut self) -> TCAN_SEL_W<14> {
        TCAN_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Host Interrupt Level 1 Selection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_lvl1_sel](index.html) module"]
pub struct HOST_LVL1_SEL_SPEC;
impl crate::RegisterSpec for HOST_LVL1_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [host_lvl1_sel::R](R) reader structure"]
impl crate::Readable for HOST_LVL1_SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [host_lvl1_sel::W](W) writer structure"]
impl crate::Writable for HOST_LVL1_SEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HOST_LVL1_SEL to value 0"]
impl crate::Resettable for HOST_LVL1_SEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
