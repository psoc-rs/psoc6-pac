#[doc = "Reader of register WINDOW_WIDEN_INTVL"]
pub type R = crate::R<u32, super::WINDOW_WIDEN_INTVL>;
#[doc = "Writer for register WINDOW_WIDEN_INTVL"]
pub type W = crate::W<u32, super::WINDOW_WIDEN_INTVL>;
#[doc = "Register WINDOW_WIDEN_INTVL `reset()`'s with value 0x0a"]
impl crate::ResetValue for super::WINDOW_WIDEN_INTVL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0a
    }
}
#[doc = "Reader of field `WINDOW_WIDEN_INTVL`"]
pub type WINDOW_WIDEN_INTVL_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `WINDOW_WIDEN_INTVL`"]
pub struct WINDOW_WIDEN_INTVL_W<'a> {
    w: &'a mut W,
}
impl<'a> WINDOW_WIDEN_INTVL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - This value defines the increased listening time for the slave. The window widening shall be smaller than ((connInterval/2)-T_IFS us) This value is calculated by firmware based on the drift, the connec-tion interval value. The value is the unit widening value for one con-nection interval duration. In case of slave latency, this value is accu-mulated till the next anchor point at which the slave will listen."]
    #[inline(always)]
    pub fn window_widen_intvl(&self) -> WINDOW_WIDEN_INTVL_R {
        WINDOW_WIDEN_INTVL_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - This value defines the increased listening time for the slave. The window widening shall be smaller than ((connInterval/2)-T_IFS us) This value is calculated by firmware based on the drift, the connec-tion interval value. The value is the unit widening value for one con-nection interval duration. In case of slave latency, this value is accu-mulated till the next anchor point at which the slave will listen."]
    #[inline(always)]
    pub fn window_widen_intvl(&mut self) -> WINDOW_WIDEN_INTVL_W {
        WINDOW_WIDEN_INTVL_W { w: self }
    }
}
