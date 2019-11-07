#[doc = "Reader of register EFUSE_CONFIG"]
pub type R = crate::R<u32, super::EFUSE_CONFIG>;
#[doc = "Writer for register EFUSE_CONFIG"]
pub type W = crate::W<u32, super::EFUSE_CONFIG>;
#[doc = "Register EFUSE_CONFIG `reset()`'s with value 0"]
impl crate::ResetValue for super::EFUSE_CONFIG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EFUSE_MODE`"]
pub type EFUSE_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EFUSE_MODE`"]
pub struct EFUSE_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_MODE_W<'a> {
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
#[doc = "Reader of field `EFUSE_READ`"]
pub type EFUSE_READ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EFUSE_READ`"]
pub struct EFUSE_READ_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_READ_W<'a> {
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
#[doc = "Reader of field `EFUSE_WRITE`"]
pub type EFUSE_WRITE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EFUSE_WRITE`"]
pub struct EFUSE_WRITE_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_WRITE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - This register enables the efuse mode in m0s8bless_ver3"]
    #[inline(always)]
    pub fn efuse_mode(&self) -> EFUSE_MODE_R {
        EFUSE_MODE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - This bit when set by firmware enables the read from EFUSE macro. It is cleared when the efuse read is completed"]
    #[inline(always)]
    pub fn efuse_read(&self) -> EFUSE_READ_R {
        EFUSE_READ_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - This bit when set by firmware enables the write to EFUSE macro. It is cleared when the efuse write is completed"]
    #[inline(always)]
    pub fn efuse_write(&self) -> EFUSE_WRITE_R {
        EFUSE_WRITE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This register enables the efuse mode in m0s8bless_ver3"]
    #[inline(always)]
    pub fn efuse_mode(&mut self) -> EFUSE_MODE_W {
        EFUSE_MODE_W { w: self }
    }
    #[doc = "Bit 1 - This bit when set by firmware enables the read from EFUSE macro. It is cleared when the efuse read is completed"]
    #[inline(always)]
    pub fn efuse_read(&mut self) -> EFUSE_READ_W {
        EFUSE_READ_W { w: self }
    }
    #[doc = "Bit 2 - This bit when set by firmware enables the write to EFUSE macro. It is cleared when the efuse write is completed"]
    #[inline(always)]
    pub fn efuse_write(&mut self) -> EFUSE_WRITE_W {
        EFUSE_WRITE_W { w: self }
    }
}
