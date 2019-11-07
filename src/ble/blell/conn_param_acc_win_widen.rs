#[doc = "Reader of register CONN_PARAM_ACC_WIN_WIDEN"]
pub type R = crate::R<u32, super::CONN_PARAM_ACC_WIN_WIDEN>;
#[doc = "Writer for register CONN_PARAM_ACC_WIN_WIDEN"]
pub type W = crate::W<u32, super::CONN_PARAM_ACC_WIN_WIDEN>;
#[doc = "Register CONN_PARAM_ACC_WIN_WIDEN `reset()`'s with value 0"]
impl crate::ResetValue for super::CONN_PARAM_ACC_WIN_WIDEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ACC_WINDOW_WIDEN`"]
pub type ACC_WINDOW_WIDEN_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ACC_WINDOW_WIDEN`"]
pub struct ACC_WINDOW_WIDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ACC_WINDOW_WIDEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - HW uses this register to load the accumulated window windeing value from the connection memory. This can be used by firmware as a failsafe option when the hardware load is disabled. In all other conditions, this register should not be updated by firmware."]
    #[inline(always)]
    pub fn acc_window_widen(&self) -> ACC_WINDOW_WIDEN_R {
        ACC_WINDOW_WIDEN_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - HW uses this register to load the accumulated window windeing value from the connection memory. This can be used by firmware as a failsafe option when the hardware load is disabled. In all other conditions, this register should not be updated by firmware."]
    #[inline(always)]
    pub fn acc_window_widen(&mut self) -> ACC_WINDOW_WIDEN_W {
        ACC_WINDOW_WIDEN_W { w: self }
    }
}
