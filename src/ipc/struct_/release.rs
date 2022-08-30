#[doc = "Register `RELEASE` writer"]
pub struct W(crate::W<RELEASE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RELEASE_SPEC>;
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
impl From<crate::W<RELEASE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RELEASE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTR_RELEASE` writer - This field allows for the generation of release events to the IPC interrupt structures, but only when the lock is acquired (LOCK_STATUS.ACQUIRED is '1'). The IPC release cause fields associated with this IPC structure are set to '1', but only for those IPC interrupt structures for which the corresponding bit field in INTR_RELEASE\\[\\]
is set to '1'. SW writes a '1' to the bit fields to generate a release event. Due to the transient nature of this event, SW always reads a '0' from this field. As a side effect, a write to this register will always set LOCK_STATUS.ACQUIRED to '0' (even when no release event is generated; i.e. the written data is '0')."]
pub type INTR_RELEASE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RELEASE_SPEC, u16, u16, 16, O>;
impl W {
    #[doc = "Bits 0:15 - This field allows for the generation of release events to the IPC interrupt structures, but only when the lock is acquired (LOCK_STATUS.ACQUIRED is '1'). The IPC release cause fields associated with this IPC structure are set to '1', but only for those IPC interrupt structures for which the corresponding bit field in INTR_RELEASE\\[\\]
is set to '1'. SW writes a '1' to the bit fields to generate a release event. Due to the transient nature of this event, SW always reads a '0' from this field. As a side effect, a write to this register will always set LOCK_STATUS.ACQUIRED to '0' (even when no release event is generated; i.e. the written data is '0')."]
    #[inline(always)]
    pub fn intr_release(&mut self) -> INTR_RELEASE_W<0> {
        INTR_RELEASE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IPC release\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [release](index.html) module"]
pub struct RELEASE_SPEC;
impl crate::RegisterSpec for RELEASE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [release::W](W) writer structure"]
impl crate::Writable for RELEASE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RELEASE to value 0"]
impl crate::Resettable for RELEASE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
