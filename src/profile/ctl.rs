#[doc = "Reader of register CTL"]
pub type R = crate::R<u32, super::CTL>;
#[doc = "Writer for register CTL"]
pub type W = crate::W<u32, super::CTL>;
#[doc = "Register CTL `reset()`'s with value 0"]
impl crate::ResetValue for super::CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WIN_MODE`"]
pub type WIN_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WIN_MODE`"]
pub struct WIN_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> WIN_MODE_W<'a> {
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
#[doc = "Reader of field `ENABLED`"]
pub type ENABLED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLED`"]
pub struct ENABLED_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLED_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Specifies the profiling time window mode: '0': Start / stop mode. The profiling time window is started when a rising edge of the start trigger signal occurs and stopped when a rising edge of the stop trigger signal occurs. In case both rising edges (of start and stop trigger signals) occur in the same cycle, the profiling time window is stopped. '1': Enable mode. The profiling time window is active as long as the start 'trigger' signal is active. The stop trigger signal has no effect."]
    #[inline(always)]
    pub fn win_mode(&self) -> WIN_MODE_R {
        WIN_MODE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 31 - Enables the profiling block: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn enabled(&self) -> ENABLED_R {
        ENABLED_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Specifies the profiling time window mode: '0': Start / stop mode. The profiling time window is started when a rising edge of the start trigger signal occurs and stopped when a rising edge of the stop trigger signal occurs. In case both rising edges (of start and stop trigger signals) occur in the same cycle, the profiling time window is stopped. '1': Enable mode. The profiling time window is active as long as the start 'trigger' signal is active. The stop trigger signal has no effect."]
    #[inline(always)]
    pub fn win_mode(&mut self) -> WIN_MODE_W {
        WIN_MODE_W { w: self }
    }
    #[doc = "Bit 31 - Enables the profiling block: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn enabled(&mut self) -> ENABLED_W {
        ENABLED_W { w: self }
    }
}
