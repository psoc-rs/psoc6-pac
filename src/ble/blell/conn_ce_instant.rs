#[doc = "Register `CONN_CE_INSTANT` reader"]
pub struct R(crate::R<CONN_CE_INSTANT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONN_CE_INSTANT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONN_CE_INSTANT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONN_CE_INSTANT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONN_CE_INSTANT` writer"]
pub struct W(crate::W<CONN_CE_INSTANT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONN_CE_INSTANT_SPEC>;
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
impl From<crate::W<CONN_CE_INSTANT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONN_CE_INSTANT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CE_INSTANT` reader - This is the value of the free running Connection Event counter when the new parameters of 'connection update' and/or 'Channel map update' will be effective. Range : 0x0000 to 0xFFFF"]
pub type CE_INSTANT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CE_INSTANT` writer - This is the value of the free running Connection Event counter when the new parameters of 'connection update' and/or 'Channel map update' will be effective. Range : 0x0000 to 0xFFFF"]
pub type CE_INSTANT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONN_CE_INSTANT_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - This is the value of the free running Connection Event counter when the new parameters of 'connection update' and/or 'Channel map update' will be effective. Range : 0x0000 to 0xFFFF"]
    #[inline(always)]
    pub fn ce_instant(&self) -> CE_INSTANT_R {
        CE_INSTANT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This is the value of the free running Connection Event counter when the new parameters of 'connection update' and/or 'Channel map update' will be effective. Range : 0x0000 to 0xFFFF"]
    #[inline(always)]
    pub fn ce_instant(&mut self) -> CE_INSTANT_W<0> {
        CE_INSTANT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Connection event instant\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conn_ce_instant](index.html) module"]
pub struct CONN_CE_INSTANT_SPEC;
impl crate::RegisterSpec for CONN_CE_INSTANT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [conn_ce_instant::R](R) reader structure"]
impl crate::Readable for CONN_CE_INSTANT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [conn_ce_instant::W](W) writer structure"]
impl crate::Writable for CONN_CE_INSTANT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONN_CE_INSTANT to value 0"]
impl crate::Resettable for CONN_CE_INSTANT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
