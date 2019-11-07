#[doc = "Reader of register INTR_MASK"]
pub type R = crate::R<u32, super::INTR_MASK>;
#[doc = "Writer for register INTR_MASK"]
pub type W = crate::W<u32, super::INTR_MASK>;
#[doc = "Register INTR_MASK `reset()`'s with value 0"]
impl crate::ResetValue for super::INTR_MASK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RELEASE`"]
pub type RELEASE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RELEASE`"]
pub struct RELEASE_W<'a> {
    w: &'a mut W,
}
impl<'a> RELEASE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `NOTIFY`"]
pub type NOTIFY_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `NOTIFY`"]
pub struct NOTIFY_W<'a> {
    w: &'a mut W,
}
impl<'a> NOTIFY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Mask bit for corresponding field in the INTR register."]
    #[inline(always)]
    pub fn release(&self) -> RELEASE_R {
        RELEASE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Mask bit for corresponding field in the INTR register."]
    #[inline(always)]
    pub fn notify(&self) -> NOTIFY_R {
        NOTIFY_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Mask bit for corresponding field in the INTR register."]
    #[inline(always)]
    pub fn release(&mut self) -> RELEASE_W {
        RELEASE_W { w: self }
    }
    #[doc = "Bits 16:31 - Mask bit for corresponding field in the INTR register."]
    #[inline(always)]
    pub fn notify(&mut self) -> NOTIFY_W {
        NOTIFY_W { w: self }
    }
}
