#[doc = "Reader of register WINDOW_WIDEN_WINOFF"]
pub type R = crate::R<u32, super::WINDOW_WIDEN_WINOFF>;
#[doc = "Writer for register WINDOW_WIDEN_WINOFF"]
pub type W = crate::W<u32, super::WINDOW_WIDEN_WINOFF>;
#[doc = "Register WINDOW_WIDEN_WINOFF `reset()`'s with value 0x0a"]
impl crate::ResetValue for super::WINDOW_WIDEN_WINOFF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0a
    }
}
#[doc = "Reader of field `WINDOW_WIDEN_WINOFF`"]
pub type WINDOW_WIDEN_WINOFF_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `WINDOW_WIDEN_WINOFF`"]
pub struct WINDOW_WIDEN_WINOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> WINDOW_WIDEN_WINOFF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - This field stores the additional number of microseconds the slave must extend its listening window to listen for a master packet. This value is calculated based on the window offset value. This is used at connection setup directly. During connection setup, this value is added with window_widen_intvl register value to calculate the win-dow widening size."]
    #[inline(always)]
    pub fn window_widen_winoff(&self) -> WINDOW_WIDEN_WINOFF_R {
        WINDOW_WIDEN_WINOFF_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - This field stores the additional number of microseconds the slave must extend its listening window to listen for a master packet. This value is calculated based on the window offset value. This is used at connection setup directly. During connection setup, this value is added with window_widen_intvl register value to calculate the win-dow widening size."]
    #[inline(always)]
    pub fn window_widen_winoff(&mut self) -> WINDOW_WIDEN_WINOFF_W {
        WINDOW_WIDEN_WINOFF_W { w: self }
    }
}
