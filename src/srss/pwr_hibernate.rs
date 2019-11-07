#[doc = "Reader of register PWR_HIBERNATE"]
pub type R = crate::R<u32, super::PWR_HIBERNATE>;
#[doc = "Writer for register PWR_HIBERNATE"]
pub type W = crate::W<u32, super::PWR_HIBERNATE>;
#[doc = "Register PWR_HIBERNATE `reset()`'s with value 0"]
impl crate::ResetValue for super::PWR_HIBERNATE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TOKEN`"]
pub type TOKEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TOKEN`"]
pub struct TOKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TOKEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `UNLOCK`"]
pub type UNLOCK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `UNLOCK`"]
pub struct UNLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> UNLOCK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `FREEZE`"]
pub type FREEZE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FREEZE`"]
pub struct FREEZE_W<'a> {
    w: &'a mut W,
}
impl<'a> FREEZE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `MASK_HIBALARM`"]
pub type MASK_HIBALARM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MASK_HIBALARM`"]
pub struct MASK_HIBALARM_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK_HIBALARM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `MASK_HIBWDT`"]
pub type MASK_HIBWDT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MASK_HIBWDT`"]
pub struct MASK_HIBWDT_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK_HIBWDT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `POLARITY_HIBPIN`"]
pub type POLARITY_HIBPIN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `POLARITY_HIBPIN`"]
pub struct POLARITY_HIBPIN_W<'a> {
    w: &'a mut W,
}
impl<'a> POLARITY_HIBPIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Reader of field `MASK_HIBPIN`"]
pub type MASK_HIBPIN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MASK_HIBPIN`"]
pub struct MASK_HIBPIN_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK_HIBPIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Reader of field `HIBERNATE_DISABLE`"]
pub type HIBERNATE_DISABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HIBERNATE_DISABLE`"]
pub struct HIBERNATE_DISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> HIBERNATE_DISABLE_W<'a> {
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
#[doc = "Reader of field `HIBERNATE`"]
pub type HIBERNATE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HIBERNATE`"]
pub struct HIBERNATE_W<'a> {
    w: &'a mut W,
}
impl<'a> HIBERNATE_W<'a> {
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
    #[doc = "Bits 0:7 - Contains a 8-bit token that is retained through a HIBERNATE/WAKEUP sequence that can be used by firmware to differentiate WAKEUP from a general RESET event. Note that waking up from HIBERNATE using XRES will reset this register."]
    #[inline(always)]
    pub fn token(&self) -> TOKEN_R {
        TOKEN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - This byte must be set to 0x3A for FREEZE or HIBERNATE fields to operate. Any other value in this register will cause FREEZE/HIBERNATE to have no effect, except as noted in the FREEZE description."]
    #[inline(always)]
    pub fn unlock(&self) -> UNLOCK_R {
        UNLOCK_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 17 - Firmware sets this bit to freeze the configuration, mode and state of all GPIOs and SIOs in the system. When entering HIBERNATE mode, the first write instructs DEEPSLEEP peripherals that they cannot ignore the upcoming freeze command. This occurs even in the illegal condition where UNLOCK is not set. If UNLOCK and HIBERNATE are properly set, the IOs actually freeze on the second write."]
    #[inline(always)]
    pub fn freeze(&self) -> FREEZE_R {
        FREEZE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - When set, HIBERNATE will wakeup for a RTC interrupt"]
    #[inline(always)]
    pub fn mask_hibalarm(&self) -> MASK_HIBALARM_R {
        MASK_HIBALARM_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - When set, HIBERNATE will wakeup if WDT matches"]
    #[inline(always)]
    pub fn mask_hibwdt(&self) -> MASK_HIBWDT_R {
        MASK_HIBWDT_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bits 20:23 - Each bit sets the active polarity of the corresponding wakeup pin. 0: Pin input of 0 will wakeup the part from HIBERNATE 1: Pin input of 1 will wakeup the part from HIBERNATE"]
    #[inline(always)]
    pub fn polarity_hibpin(&self) -> POLARITY_HIBPIN_R {
        POLARITY_HIBPIN_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - When set, HIBERNATE will wakeup if the corresponding pin input matches the POLARITY_HIBPIN setting. Each bit corresponds to one of the wakeup pins."]
    #[inline(always)]
    pub fn mask_hibpin(&self) -> MASK_HIBPIN_R {
        MASK_HIBPIN_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 30 - Hibernate disable bit. 0: Normal operation, HIBERNATE works as described 1: Further writes to this register are ignored Note: This bit is a write-once bit until the next reset. Avoid changing any other bits in this register while disabling HIBERNATE mode. Also, it is recommended to clear the UNLOCK code, if it was previously written.."]
    #[inline(always)]
    pub fn hibernate_disable(&self) -> HIBERNATE_DISABLE_R {
        HIBERNATE_DISABLE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Firmware sets this bit to enter HIBERNATE mode. The system will enter HIBERNATE mode immediately after writing to this bit and will wakeup only in response to XRES or WAKEUP event. Both UNLOCK and FREEZE must have been set correctly in a previous write operations. Otherwise, it will not enter HIBERNATE. External supplies must have been stable for 250us before entering HIBERNATE mode."]
    #[inline(always)]
    pub fn hibernate(&self) -> HIBERNATE_R {
        HIBERNATE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Contains a 8-bit token that is retained through a HIBERNATE/WAKEUP sequence that can be used by firmware to differentiate WAKEUP from a general RESET event. Note that waking up from HIBERNATE using XRES will reset this register."]
    #[inline(always)]
    pub fn token(&mut self) -> TOKEN_W {
        TOKEN_W { w: self }
    }
    #[doc = "Bits 8:15 - This byte must be set to 0x3A for FREEZE or HIBERNATE fields to operate. Any other value in this register will cause FREEZE/HIBERNATE to have no effect, except as noted in the FREEZE description."]
    #[inline(always)]
    pub fn unlock(&mut self) -> UNLOCK_W {
        UNLOCK_W { w: self }
    }
    #[doc = "Bit 17 - Firmware sets this bit to freeze the configuration, mode and state of all GPIOs and SIOs in the system. When entering HIBERNATE mode, the first write instructs DEEPSLEEP peripherals that they cannot ignore the upcoming freeze command. This occurs even in the illegal condition where UNLOCK is not set. If UNLOCK and HIBERNATE are properly set, the IOs actually freeze on the second write."]
    #[inline(always)]
    pub fn freeze(&mut self) -> FREEZE_W {
        FREEZE_W { w: self }
    }
    #[doc = "Bit 18 - When set, HIBERNATE will wakeup for a RTC interrupt"]
    #[inline(always)]
    pub fn mask_hibalarm(&mut self) -> MASK_HIBALARM_W {
        MASK_HIBALARM_W { w: self }
    }
    #[doc = "Bit 19 - When set, HIBERNATE will wakeup if WDT matches"]
    #[inline(always)]
    pub fn mask_hibwdt(&mut self) -> MASK_HIBWDT_W {
        MASK_HIBWDT_W { w: self }
    }
    #[doc = "Bits 20:23 - Each bit sets the active polarity of the corresponding wakeup pin. 0: Pin input of 0 will wakeup the part from HIBERNATE 1: Pin input of 1 will wakeup the part from HIBERNATE"]
    #[inline(always)]
    pub fn polarity_hibpin(&mut self) -> POLARITY_HIBPIN_W {
        POLARITY_HIBPIN_W { w: self }
    }
    #[doc = "Bits 24:27 - When set, HIBERNATE will wakeup if the corresponding pin input matches the POLARITY_HIBPIN setting. Each bit corresponds to one of the wakeup pins."]
    #[inline(always)]
    pub fn mask_hibpin(&mut self) -> MASK_HIBPIN_W {
        MASK_HIBPIN_W { w: self }
    }
    #[doc = "Bit 30 - Hibernate disable bit. 0: Normal operation, HIBERNATE works as described 1: Further writes to this register are ignored Note: This bit is a write-once bit until the next reset. Avoid changing any other bits in this register while disabling HIBERNATE mode. Also, it is recommended to clear the UNLOCK code, if it was previously written.."]
    #[inline(always)]
    pub fn hibernate_disable(&mut self) -> HIBERNATE_DISABLE_W {
        HIBERNATE_DISABLE_W { w: self }
    }
    #[doc = "Bit 31 - Firmware sets this bit to enter HIBERNATE mode. The system will enter HIBERNATE mode immediately after writing to this bit and will wakeup only in response to XRES or WAKEUP event. Both UNLOCK and FREEZE must have been set correctly in a previous write operations. Otherwise, it will not enter HIBERNATE. External supplies must have been stable for 250us before entering HIBERNATE mode."]
    #[inline(always)]
    pub fn hibernate(&mut self) -> HIBERNATE_W {
        HIBERNATE_W { w: self }
    }
}
