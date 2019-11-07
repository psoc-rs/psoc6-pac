#[doc = "Writer for register RELEASE"]
pub type W = crate::W<u32, super::RELEASE>;
#[doc = "Register RELEASE `reset()`'s with value 0"]
impl crate::ResetValue for super::RELEASE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `INTR_RELEASE`"]
pub struct INTR_RELEASE_W<'a> {
    w: &'a mut W,
}
impl<'a> INTR_RELEASE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:15 - This field allows for the generation of release events to the IPC interrupt structures, but only when the lock is acquired (LOCK_STATUS.ACQUIRED is '1'). The IPC release cause fields associated with this IPC structure are set to '1', but only for those IPC interrupt structures for which the corresponding bit field in INTR_RELEASE\\[\\] is set to '1'. SW writes a '1' to the bit fields to generate a release event. Due to the transient nature of this event, SW always reads a '0' from this field."]
    #[inline(always)]
    pub fn intr_release(&mut self) -> INTR_RELEASE_W {
        INTR_RELEASE_W { w: self }
    }
}
