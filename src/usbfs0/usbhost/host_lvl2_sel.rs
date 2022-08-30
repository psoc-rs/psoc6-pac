#[doc = "Register `HOST_LVL2_SEL` reader"]
pub struct R(crate::R<HOST_LVL2_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HOST_LVL2_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HOST_LVL2_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HOST_LVL2_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HOST_LVL2_SEL` writer"]
pub struct W(crate::W<HOST_LVL2_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HOST_LVL2_SEL_SPEC>;
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
impl From<crate::W<HOST_LVL2_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HOST_LVL2_SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "These bits assign EP1_DRQ interrupt flag to any interrupt signals.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EP1_DRQ_SEL_A {
    #[doc = "0: High priority interrupt"]
    HI = 0,
    #[doc = "1: Medium priority interrupt"]
    MED = 1,
    #[doc = "2: Low priority interrupt"]
    LO = 2,
    #[doc = "3: illegal"]
    RSVD = 3,
}
impl From<EP1_DRQ_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: EP1_DRQ_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EP1_DRQ_SEL` reader - These bits assign EP1_DRQ interrupt flag to any interrupt signals."]
pub type EP1_DRQ_SEL_R = crate::FieldReader<u8, EP1_DRQ_SEL_A>;
impl EP1_DRQ_SEL_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `EP1_DRQ_SEL` writer - These bits assign EP1_DRQ interrupt flag to any interrupt signals."]
pub type EP1_DRQ_SEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, HOST_LVL2_SEL_SPEC, u8, EP1_DRQ_SEL_A, 2, O>;
impl<'a, const O: u8> EP1_DRQ_SEL_W<'a, O> {
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
}
#[doc = "Field `EP1_SPK_SEL` reader - These bits assign EP1_SPK interrupt flag to any interrupt signals."]
pub type EP1_SPK_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EP1_SPK_SEL` writer - These bits assign EP1_SPK interrupt flag to any interrupt signals."]
pub type EP1_SPK_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HOST_LVL2_SEL_SPEC, u8, u8, 2, O>;
#[doc = "Field `EP2_DRQ_SEL` reader - These bits assign EP2_DRQ interrupt flag to any interrupt signals."]
pub type EP2_DRQ_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EP2_DRQ_SEL` writer - These bits assign EP2_DRQ interrupt flag to any interrupt signals."]
pub type EP2_DRQ_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HOST_LVL2_SEL_SPEC, u8, u8, 2, O>;
#[doc = "Field `EP2_SPK_SEL` reader - These bits assign EP2_SPK interrupt flag to any interrupt signals."]
pub type EP2_SPK_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EP2_SPK_SEL` writer - These bits assign EP2_SPK interrupt flag to any interrupt signals."]
pub type EP2_SPK_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HOST_LVL2_SEL_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 4:5 - These bits assign EP1_DRQ interrupt flag to any interrupt signals."]
    #[inline(always)]
    pub fn ep1_drq_sel(&self) -> EP1_DRQ_SEL_R {
        EP1_DRQ_SEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - These bits assign EP1_SPK interrupt flag to any interrupt signals."]
    #[inline(always)]
    pub fn ep1_spk_sel(&self) -> EP1_SPK_SEL_R {
        EP1_SPK_SEL_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - These bits assign EP2_DRQ interrupt flag to any interrupt signals."]
    #[inline(always)]
    pub fn ep2_drq_sel(&self) -> EP2_DRQ_SEL_R {
        EP2_DRQ_SEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - These bits assign EP2_SPK interrupt flag to any interrupt signals."]
    #[inline(always)]
    pub fn ep2_spk_sel(&self) -> EP2_SPK_SEL_R {
        EP2_SPK_SEL_R::new(((self.bits >> 10) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 4:5 - These bits assign EP1_DRQ interrupt flag to any interrupt signals."]
    #[inline(always)]
    pub fn ep1_drq_sel(&mut self) -> EP1_DRQ_SEL_W<4> {
        EP1_DRQ_SEL_W::new(self)
    }
    #[doc = "Bits 6:7 - These bits assign EP1_SPK interrupt flag to any interrupt signals."]
    #[inline(always)]
    pub fn ep1_spk_sel(&mut self) -> EP1_SPK_SEL_W<6> {
        EP1_SPK_SEL_W::new(self)
    }
    #[doc = "Bits 8:9 - These bits assign EP2_DRQ interrupt flag to any interrupt signals."]
    #[inline(always)]
    pub fn ep2_drq_sel(&mut self) -> EP2_DRQ_SEL_W<8> {
        EP2_DRQ_SEL_W::new(self)
    }
    #[doc = "Bits 10:11 - These bits assign EP2_SPK interrupt flag to any interrupt signals."]
    #[inline(always)]
    pub fn ep2_spk_sel(&mut self) -> EP2_SPK_SEL_W<10> {
        EP2_SPK_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Host Interrupt Level 2 Selection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_lvl2_sel](index.html) module"]
pub struct HOST_LVL2_SEL_SPEC;
impl crate::RegisterSpec for HOST_LVL2_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [host_lvl2_sel::R](R) reader structure"]
impl crate::Readable for HOST_LVL2_SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [host_lvl2_sel::W](W) writer structure"]
impl crate::Writable for HOST_LVL2_SEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HOST_LVL2_SEL to value 0"]
impl crate::Resettable for HOST_LVL2_SEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
