#[doc = "Reader of register INIT_WINDOW_TIMER_CTRL"]
pub type R = crate::R<u32, super::INIT_WINDOW_TIMER_CTRL>;
#[doc = "Writer for register INIT_WINDOW_TIMER_CTRL"]
pub type W = crate::W<u32, super::INIT_WINDOW_TIMER_CTRL>;
#[doc = "Register INIT_WINDOW_TIMER_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::INIT_WINDOW_TIMER_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INIT_WINDOW_OFFSET_SEL`"]
pub type INIT_WINDOW_OFFSET_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INIT_WINDOW_OFFSET_SEL`"]
pub struct INIT_WINDOW_OFFSET_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> INIT_WINDOW_OFFSET_SEL_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Controls the INIT Window offset source 1 - Pick INIT Window Offset from HW calculated INIT_WINDOW_OFFSET 0 - Pick INIT Window Offset from FW loaded register"]
    #[inline(always)]
    pub fn init_window_offset_sel(&self) -> INIT_WINDOW_OFFSET_SEL_R {
        INIT_WINDOW_OFFSET_SEL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Controls the INIT Window offset source 1 - Pick INIT Window Offset from HW calculated INIT_WINDOW_OFFSET 0 - Pick INIT Window Offset from FW loaded register"]
    #[inline(always)]
    pub fn init_window_offset_sel(&mut self) -> INIT_WINDOW_OFFSET_SEL_W {
        INIT_WINDOW_OFFSET_SEL_W { w: self }
    }
}
