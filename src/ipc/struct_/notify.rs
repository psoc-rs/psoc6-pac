#[doc = "Writer for register NOTIFY"]
pub type W = crate::W<u32, super::NOTIFY>;
#[doc = "Register NOTIFY `reset()`'s with value 0"]
impl crate::ResetValue for super::NOTIFY {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `INTR_NOTIFY`"]
pub struct INTR_NOTIFY_W<'a> {
    w: &'a mut W,
}
impl<'a> INTR_NOTIFY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:15 - This field allows for the generation of notification events to the IPC interrupt structures. The IPC notification cause fields associated with this IPC structure are set to '1', but only for those IPC interrupt structures for which the corresponding bit field in INTR_NOTIFY\\[\\] is set to '1'. SW writes a '1' to the bit fields to generate a notify event. Due to the transient nature of this event, SW always reads a '0' from this field."]
    #[inline(always)]
    pub fn intr_notify(&mut self) -> INTR_NOTIFY_W {
        INTR_NOTIFY_W { w: self }
    }
}
