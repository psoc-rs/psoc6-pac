#[doc = "Reader of register RTC_RW"]
pub type R = crate::R<u32, super::RTC_RW>;
#[doc = "Writer for register RTC_RW"]
pub type W = crate::W<u32, super::RTC_RW>;
#[doc = "Register RTC_RW `reset()`'s with value 0"]
impl crate::ResetValue for super::RTC_RW {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `READ`"]
pub type READ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `READ`"]
pub struct READ_W<'a> {
    w: &'a mut W,
}
impl<'a> READ_W<'a> {
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
#[doc = "Reader of field `WRITE`"]
pub type WRITE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRITE`"]
pub struct WRITE_W<'a> {
    w: &'a mut W,
}
impl<'a> WRITE_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Read bit When this bit is set the RTC registers will be copied to user registers and frozen so that a coherent RTC value can safely be read. The RTC will keep on running. Do not set the read bit if the RTC is still busy with a previous update (see RTC_BUSY bit) or if the Write bit is set. Do not set the Read bit at the same time that the Write bit is cleared."]
    #[inline(always)]
    pub fn read(&self) -> READ_R {
        READ_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Write bit Only when this bit is set can the RTC registers be written to (otherwise writes are ignored). This bit cannot be set if the RTC is still busy with a previous update (see RTC_BUSY bit) or if the Read bit is set or getting set. The user writes to the RTC user registers, when the Write bit is cleared by the user then the user registers content is copied to the actual RTC registers. Only user RTC registers that were written to will get copied, others will not be affected. When the SECONDS field is updated then TICKS will also be reset (WDT is not affected). When the Write bit is cleared by a reset (brown out/DeepSleep) then the RTC update will be ignored/lost. Do not set the Write bit if the RTC if the RTC is still busy with a previous update (see RTC_BUSY). Do not set the Write bit at the same time that the Read bit is cleared."]
    #[inline(always)]
    pub fn write(&self) -> WRITE_R {
        WRITE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Read bit When this bit is set the RTC registers will be copied to user registers and frozen so that a coherent RTC value can safely be read. The RTC will keep on running. Do not set the read bit if the RTC is still busy with a previous update (see RTC_BUSY bit) or if the Write bit is set. Do not set the Read bit at the same time that the Write bit is cleared."]
    #[inline(always)]
    pub fn read(&mut self) -> READ_W {
        READ_W { w: self }
    }
    #[doc = "Bit 1 - Write bit Only when this bit is set can the RTC registers be written to (otherwise writes are ignored). This bit cannot be set if the RTC is still busy with a previous update (see RTC_BUSY bit) or if the Read bit is set or getting set. The user writes to the RTC user registers, when the Write bit is cleared by the user then the user registers content is copied to the actual RTC registers. Only user RTC registers that were written to will get copied, others will not be affected. When the SECONDS field is updated then TICKS will also be reset (WDT is not affected). When the Write bit is cleared by a reset (brown out/DeepSleep) then the RTC update will be ignored/lost. Do not set the Write bit if the RTC if the RTC is still busy with a previous update (see RTC_BUSY). Do not set the Write bit at the same time that the Read bit is cleared."]
    #[inline(always)]
    pub fn write(&mut self) -> WRITE_W {
        WRITE_W { w: self }
    }
}
