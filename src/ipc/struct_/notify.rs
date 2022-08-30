#[doc = "Register `NOTIFY` writer"]
pub struct W(crate::W<NOTIFY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NOTIFY_SPEC>;
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
impl From<crate::W<NOTIFY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NOTIFY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTR_NOTIFY` writer - This field allows for the generation of notification events to the IPC interrupt structures. The IPC notification cause fields associated with this IPC structure are set to '1', but only for those IPC interrupt structures for which the corresponding bit field in INTR_NOTIFY\\[\\]
is set to '1'. SW writes a '1' to the bit fields to generate a notify event. Due to the transient nature of this event, SW always reads a '0' from this field."]
pub type INTR_NOTIFY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NOTIFY_SPEC, u16, u16, 16, O>;
impl W {
    #[doc = "Bits 0:15 - This field allows for the generation of notification events to the IPC interrupt structures. The IPC notification cause fields associated with this IPC structure are set to '1', but only for those IPC interrupt structures for which the corresponding bit field in INTR_NOTIFY\\[\\]
is set to '1'. SW writes a '1' to the bit fields to generate a notify event. Due to the transient nature of this event, SW always reads a '0' from this field."]
    #[inline(always)]
    pub fn intr_notify(&mut self) -> INTR_NOTIFY_W<0> {
        INTR_NOTIFY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IPC notification\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [notify](index.html) module"]
pub struct NOTIFY_SPEC;
impl crate::RegisterSpec for NOTIFY_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [notify::W](W) writer structure"]
impl crate::Writable for NOTIFY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NOTIFY to value 0"]
impl crate::Resettable for NOTIFY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
