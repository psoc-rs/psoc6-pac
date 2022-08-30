#[doc = "Register `CONN_UPDATE_NEW_SUP_TO` reader"]
pub struct R(crate::R<CONN_UPDATE_NEW_SUP_TO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONN_UPDATE_NEW_SUP_TO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONN_UPDATE_NEW_SUP_TO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONN_UPDATE_NEW_SUP_TO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONN_UPDATE_NEW_SUP_TO` writer"]
pub struct W(crate::W<CONN_UPDATE_NEW_SUP_TO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONN_UPDATE_NEW_SUP_TO_SPEC>;
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
impl From<crate::W<CONN_UPDATE_NEW_SUP_TO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONN_UPDATE_NEW_SUP_TO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CONN_UPDT_SUP_TO` reader - This register will have the new supervision timeout that the hardware will use after the connection update instant. Before the instant, the connection interval in the register SUP_TIMEOUT will be used by hardware."]
pub type CONN_UPDT_SUP_TO_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CONN_UPDT_SUP_TO` writer - This register will have the new supervision timeout that the hardware will use after the connection update instant. Before the instant, the connection interval in the register SUP_TIMEOUT will be used by hardware."]
pub type CONN_UPDT_SUP_TO_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONN_UPDATE_NEW_SUP_TO_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - This register will have the new supervision timeout that the hardware will use after the connection update instant. Before the instant, the connection interval in the register SUP_TIMEOUT will be used by hardware."]
    #[inline(always)]
    pub fn conn_updt_sup_to(&self) -> CONN_UPDT_SUP_TO_R {
        CONN_UPDT_SUP_TO_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This register will have the new supervision timeout that the hardware will use after the connection update instant. Before the instant, the connection interval in the register SUP_TIMEOUT will be used by hardware."]
    #[inline(always)]
    pub fn conn_updt_sup_to(&mut self) -> CONN_UPDT_SUP_TO_W<0> {
        CONN_UPDT_SUP_TO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Connection update new supervision timeout\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conn_update_new_sup_to](index.html) module"]
pub struct CONN_UPDATE_NEW_SUP_TO_SPEC;
impl crate::RegisterSpec for CONN_UPDATE_NEW_SUP_TO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [conn_update_new_sup_to::R](R) reader structure"]
impl crate::Readable for CONN_UPDATE_NEW_SUP_TO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [conn_update_new_sup_to::W](W) writer structure"]
impl crate::Writable for CONN_UPDATE_NEW_SUP_TO_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONN_UPDATE_NEW_SUP_TO to value 0"]
impl crate::Resettable for CONN_UPDATE_NEW_SUP_TO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
