#[doc = "Register `PROTECTION` reader"]
pub struct R(crate::R<PROTECTION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PROTECTION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PROTECTION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PROTECTION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PROTECTION` writer"]
pub struct W(crate::W<PROTECTION_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PROTECTION_SPEC>;
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
impl From<crate::W<PROTECTION_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PROTECTION_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STATE` reader - Protection state: '0': UNKNOWN. '1': VIRGIN. '2': NORMAL. '3': SECURE. '4': DEAD. The following state transitions are allowed (and enforced by HW): - UNKNOWN => VIRGIN/NORMAL/SECURE/DEAD - NORMAL => DEAD - SECURE => DEAD An attempt to make a NOT allowed state transition will NOT affect this register field."]
pub type STATE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STATE` writer - Protection state: '0': UNKNOWN. '1': VIRGIN. '2': NORMAL. '3': SECURE. '4': DEAD. The following state transitions are allowed (and enforced by HW): - UNKNOWN => VIRGIN/NORMAL/SECURE/DEAD - NORMAL => DEAD - SECURE => DEAD An attempt to make a NOT allowed state transition will NOT affect this register field."]
pub type STATE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PROTECTION_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:2 - Protection state: '0': UNKNOWN. '1': VIRGIN. '2': NORMAL. '3': SECURE. '4': DEAD. The following state transitions are allowed (and enforced by HW): - UNKNOWN => VIRGIN/NORMAL/SECURE/DEAD - NORMAL => DEAD - SECURE => DEAD An attempt to make a NOT allowed state transition will NOT affect this register field."]
    #[inline(always)]
    pub fn state(&self) -> STATE_R {
        STATE_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Protection state: '0': UNKNOWN. '1': VIRGIN. '2': NORMAL. '3': SECURE. '4': DEAD. The following state transitions are allowed (and enforced by HW): - UNKNOWN => VIRGIN/NORMAL/SECURE/DEAD - NORMAL => DEAD - SECURE => DEAD An attempt to make a NOT allowed state transition will NOT affect this register field."]
    #[inline(always)]
    pub fn state(&mut self) -> STATE_W<0> {
        STATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Protection status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [protection](index.html) module"]
pub struct PROTECTION_SPEC;
impl crate::RegisterSpec for PROTECTION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [protection::R](R) reader structure"]
impl crate::Readable for PROTECTION_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [protection::W](W) writer structure"]
impl crate::Writable for PROTECTION_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PROTECTION to value 0"]
impl crate::Resettable for PROTECTION_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
