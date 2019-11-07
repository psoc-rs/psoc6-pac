#[doc = "Reader of register CLK_ILO_CONFIG"]
pub type R = crate::R<u32, super::CLK_ILO_CONFIG>;
#[doc = "Writer for register CLK_ILO_CONFIG"]
pub type W = crate::W<u32, super::CLK_ILO_CONFIG>;
#[doc = "Register CLK_ILO_CONFIG `reset()`'s with value 0x8000_0000"]
impl crate::ResetValue for super::CLK_ILO_CONFIG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x8000_0000
    }
}
#[doc = "Reader of field `ILO_BACKUP`"]
pub type ILO_BACKUP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ILO_BACKUP`"]
pub struct ILO_BACKUP_W<'a> {
    w: &'a mut W,
}
impl<'a> ILO_BACKUP_W<'a> {
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
#[doc = "Reader of field `ENABLE`"]
pub type ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE`"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
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
    #[doc = "Bit 0 - If backup domain is present, this register indicates that ILO should stay enabled for use by backup domain during XRES, HIBERNATE mode, and through power-related resets like BOD. Writes to this field are ignored unless the WDT is unlocked using WDT_LOCK register. 0: ILO turns off at XRES/BOD event or HIBERNATE entry. 1: ILO remains on if backup domain is present and powered even for XRES/BOD or HIBERNATE entry."]
    #[inline(always)]
    pub fn ilo_backup(&self) -> ILO_BACKUP_R {
        ILO_BACKUP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 31 - Master enable for ILO. Writes to this field are ignored unless the WDT is unlocked using WDT_LOCK register."]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - If backup domain is present, this register indicates that ILO should stay enabled for use by backup domain during XRES, HIBERNATE mode, and through power-related resets like BOD. Writes to this field are ignored unless the WDT is unlocked using WDT_LOCK register. 0: ILO turns off at XRES/BOD event or HIBERNATE entry. 1: ILO remains on if backup domain is present and powered even for XRES/BOD or HIBERNATE entry."]
    #[inline(always)]
    pub fn ilo_backup(&mut self) -> ILO_BACKUP_W {
        ILO_BACKUP_W { w: self }
    }
    #[doc = "Bit 31 - Master enable for ILO. Writes to this field are ignored unless the WDT is unlocked using WDT_LOCK register."]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
}
