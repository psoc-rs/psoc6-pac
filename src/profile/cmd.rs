#[doc = "Reader of register CMD"]
pub type R = crate::R<u32, super::CMD>;
#[doc = "Writer for register CMD"]
pub type W = crate::W<u32, super::CMD>;
#[doc = "Register CMD `reset()`'s with value 0"]
impl crate::ResetValue for super::CMD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `START_TR`"]
pub type START_TR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `START_TR`"]
pub struct START_TR_W<'a> {
    w: &'a mut W,
}
impl<'a> START_TR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `STOP_TR`"]
pub type STOP_TR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STOP_TR`"]
pub struct STOP_TR_W<'a> {
    w: &'a mut W,
}
impl<'a> STOP_TR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `CLR_ALL_CNT`"]
pub type CLR_ALL_CNT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLR_ALL_CNT`"]
pub struct CLR_ALL_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR_ALL_CNT_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Software start trigger for the profiling time window. When written with '1', the profiling time window is started. Can only be used in start / stop mode (PROFILE_WIN_MODE=0). Has no effect in enable mode (PROFILE_WIN_MODE=1)."]
    #[inline(always)]
    pub fn start_tr(&self) -> START_TR_R {
        START_TR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Software stop trigger for the profiling time window. When written with '1', the profiling time window is stopped. Can only be used in start / stop mode (PROFILE_WIN_MODE=0). Has no effect in enable mode (PROFILE_WIN_MODE=1)."]
    #[inline(always)]
    pub fn stop_tr(&self) -> STOP_TR_R {
        STOP_TR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Counter clear. When written with '1', all profiling counter registers are cleared to 0x00."]
    #[inline(always)]
    pub fn clr_all_cnt(&self) -> CLR_ALL_CNT_R {
        CLR_ALL_CNT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software start trigger for the profiling time window. When written with '1', the profiling time window is started. Can only be used in start / stop mode (PROFILE_WIN_MODE=0). Has no effect in enable mode (PROFILE_WIN_MODE=1)."]
    #[inline(always)]
    pub fn start_tr(&mut self) -> START_TR_W {
        START_TR_W { w: self }
    }
    #[doc = "Bit 1 - Software stop trigger for the profiling time window. When written with '1', the profiling time window is stopped. Can only be used in start / stop mode (PROFILE_WIN_MODE=0). Has no effect in enable mode (PROFILE_WIN_MODE=1)."]
    #[inline(always)]
    pub fn stop_tr(&mut self) -> STOP_TR_W {
        STOP_TR_W { w: self }
    }
    #[doc = "Bit 8 - Counter clear. When written with '1', all profiling counter registers are cleared to 0x00."]
    #[inline(always)]
    pub fn clr_all_cnt(&mut self) -> CLR_ALL_CNT_W {
        CLR_ALL_CNT_W { w: self }
    }
}
