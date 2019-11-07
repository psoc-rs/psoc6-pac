#[doc = "Reader of register CTB_CTRL"]
pub type R = crate::R<u32, super::CTB_CTRL>;
#[doc = "Writer for register CTB_CTRL"]
pub type W = crate::W<u32, super::CTB_CTRL>;
#[doc = "Register CTB_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CTB_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DEEPSLEEP_ON`"]
pub type DEEPSLEEP_ON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DEEPSLEEP_ON`"]
pub struct DEEPSLEEP_ON_W<'a> {
    w: &'a mut W,
}
impl<'a> DEEPSLEEP_ON_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
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
    #[doc = "Bit 30 - - 0: CTB IP disabled off during DeepSleep power mode - 1: CTB IP remains enabled during DeepSleep power mode (if ENABLED=1)"]
    #[inline(always)]
    pub fn deepsleep_on(&self) -> DEEPSLEEP_ON_R {
        DEEPSLEEP_ON_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - - 0: CTB IP disabled (put analog in power down, open all switches) - 1: CTB IP enabled"]
    #[inline(always)]
    pub fn enabled(&self) -> ENABLED_R {
        ENABLED_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 30 - - 0: CTB IP disabled off during DeepSleep power mode - 1: CTB IP remains enabled during DeepSleep power mode (if ENABLED=1)"]
    #[inline(always)]
    pub fn deepsleep_on(&mut self) -> DEEPSLEEP_ON_W {
        DEEPSLEEP_ON_W { w: self }
    }
    #[doc = "Bit 31 - - 0: CTB IP disabled (put analog in power down, open all switches) - 1: CTB IP enabled"]
    #[inline(always)]
    pub fn enabled(&mut self) -> ENABLED_W {
        ENABLED_W { w: self }
    }
}
