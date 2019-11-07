#[doc = "Reader of register START_CTRL"]
pub type R = crate::R<u32, super::START_CTRL>;
#[doc = "Writer for register START_CTRL"]
pub type W = crate::W<u32, super::START_CTRL>;
#[doc = "Register START_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::START_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FW_TRIGGER`"]
pub type FW_TRIGGER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FW_TRIGGER`"]
pub struct FW_TRIGGER_W<'a> {
    w: &'a mut W,
}
impl<'a> FW_TRIGGER_W<'a> {
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
    #[doc = "Bit 0 - When firmware writes a 1 here it will trigger the next scan of enabled channels, hardware clears this bit when the scan started with this trigger is completed. If scanning continuously the trigger is ignored and hardware clears this bit after the next scan is done. This bit is also cleared when the SAR is disabled."]
    #[inline(always)]
    pub fn fw_trigger(&self) -> FW_TRIGGER_R {
        FW_TRIGGER_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When firmware writes a 1 here it will trigger the next scan of enabled channels, hardware clears this bit when the scan started with this trigger is completed. If scanning continuously the trigger is ignored and hardware clears this bit after the next scan is done. This bit is also cleared when the SAR is disabled."]
    #[inline(always)]
    pub fn fw_trigger(&mut self) -> FW_TRIGGER_W {
        FW_TRIGGER_W { w: self }
    }
}
