#[doc = "Register `CHAN_EN` reader"]
pub struct R(crate::R<CHAN_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHAN_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHAN_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHAN_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHAN_EN` writer"]
pub struct W(crate::W<CHAN_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHAN_EN_SPEC>;
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
impl From<crate::W<CHAN_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHAN_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHAN_EN` reader - Channel enable. - 0: the corresponding channel is disabled. - 1: the corresponding channel is enabled, it will be included in the next scan."]
pub type CHAN_EN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CHAN_EN` writer - Channel enable. - 0: the corresponding channel is disabled. - 1: the corresponding channel is enabled, it will be included in the next scan."]
pub type CHAN_EN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CHAN_EN_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Channel enable. - 0: the corresponding channel is disabled. - 1: the corresponding channel is enabled, it will be included in the next scan."]
    #[inline(always)]
    pub fn chan_en(&self) -> CHAN_EN_R {
        CHAN_EN_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Channel enable. - 0: the corresponding channel is disabled. - 1: the corresponding channel is enabled, it will be included in the next scan."]
    #[inline(always)]
    pub fn chan_en(&mut self) -> CHAN_EN_W<0> {
        CHAN_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Enable bits for the channels\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chan_en](index.html) module"]
pub struct CHAN_EN_SPEC;
impl crate::RegisterSpec for CHAN_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chan_en::R](R) reader structure"]
impl crate::Readable for CHAN_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chan_en::W](W) writer structure"]
impl crate::Writable for CHAN_EN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CHAN_EN to value 0"]
impl crate::Resettable for CHAN_EN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
