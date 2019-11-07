#[doc = "Reader of register TRIM_RAM_CTL"]
pub type R = crate::R<u32, super::TRIM_RAM_CTL>;
#[doc = "Writer for register TRIM_RAM_CTL"]
pub type W = crate::W<u32, super::TRIM_RAM_CTL>;
#[doc = "Register TRIM_RAM_CTL `reset()`'s with value 0x6002"]
impl crate::ResetValue for super::TRIM_RAM_CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x6002
    }
}
#[doc = "Reader of field `RM`"]
pub type RM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RM`"]
pub struct RM_W<'a> {
    w: &'a mut W,
}
impl<'a> RM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `RME`"]
pub type RME_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RME`"]
pub struct RME_W<'a> {
    w: &'a mut W,
}
impl<'a> RME_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `WPULSE`"]
pub type WPULSE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WPULSE`"]
pub struct WPULSE_W<'a> {
    w: &'a mut W,
}
impl<'a> WPULSE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | (((value as u32) & 0x07) << 5);
        self.w
    }
}
#[doc = "Reader of field `RA`"]
pub type RA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RA`"]
pub struct RA_W<'a> {
    w: &'a mut W,
}
impl<'a> RA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `WA`"]
pub type WA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WA`"]
pub struct WA_W<'a> {
    w: &'a mut W,
}
impl<'a> WA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - N/A"]
    #[inline(always)]
    pub fn rm(&self) -> RM_R {
        RM_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Read-Write margin enable control. This selects between the default Read-Write margin setting, and the external RM\\[3:0\\] Read-Write margin settting."]
    #[inline(always)]
    pub fn rme(&self) -> RME_R {
        RME_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:7 - Write Assist Pulse to control pulse width of negative voltage on SRAM bitline."]
    #[inline(always)]
    pub fn wpulse(&self) -> WPULSE_R {
        WPULSE_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bits 8:9 - Read Assist control for WL under-drive."]
    #[inline(always)]
    pub fn ra(&self) -> RA_R {
        RA_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 12:14 - Write assist enable control (Active High). - WA\\[1:0\\] Write Assist pins to control negative voltage on SRAM bitline."]
    #[inline(always)]
    pub fn wa(&self) -> WA_R {
        WA_R::new(((self.bits >> 12) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - N/A"]
    #[inline(always)]
    pub fn rm(&mut self) -> RM_W {
        RM_W { w: self }
    }
    #[doc = "Bit 4 - Read-Write margin enable control. This selects between the default Read-Write margin setting, and the external RM\\[3:0\\] Read-Write margin settting."]
    #[inline(always)]
    pub fn rme(&mut self) -> RME_W {
        RME_W { w: self }
    }
    #[doc = "Bits 5:7 - Write Assist Pulse to control pulse width of negative voltage on SRAM bitline."]
    #[inline(always)]
    pub fn wpulse(&mut self) -> WPULSE_W {
        WPULSE_W { w: self }
    }
    #[doc = "Bits 8:9 - Read Assist control for WL under-drive."]
    #[inline(always)]
    pub fn ra(&mut self) -> RA_W {
        RA_W { w: self }
    }
    #[doc = "Bits 12:14 - Write assist enable control (Active High). - WA\\[1:0\\] Write Assist pins to control negative voltage on SRAM bitline."]
    #[inline(always)]
    pub fn wa(&mut self) -> WA_W {
        WA_W { w: self }
    }
}
