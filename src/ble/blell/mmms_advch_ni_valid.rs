#[doc = "Reader of register MMMS_ADVCH_NI_VALID"]
pub type R = crate::R<u32, super::MMMS_ADVCH_NI_VALID>;
#[doc = "Writer for register MMMS_ADVCH_NI_VALID"]
pub type W = crate::W<u32, super::MMMS_ADVCH_NI_VALID>;
#[doc = "Register MMMS_ADVCH_NI_VALID `reset()`'s with value 0"]
impl crate::ResetValue for super::MMMS_ADVCH_NI_VALID {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADV_NI_VALID`"]
pub type ADV_NI_VALID_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADV_NI_VALID`"]
pub struct ADV_NI_VALID_W<'a> {
    w: &'a mut W,
}
impl<'a> ADV_NI_VALID_W<'a> {
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
#[doc = "Reader of field `SCAN_NI_VALID`"]
pub type SCAN_NI_VALID_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCAN_NI_VALID`"]
pub struct SCAN_NI_VALID_W<'a> {
    w: &'a mut W,
}
impl<'a> SCAN_NI_VALID_W<'a> {
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
#[doc = "Reader of field `INIT_NI_VALID`"]
pub type INIT_NI_VALID_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INIT_NI_VALID`"]
pub struct INIT_NI_VALID_W<'a> {
    w: &'a mut W,
}
impl<'a> INIT_NI_VALID_W<'a> {
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
    #[doc = "Bit 0 - This bit indicates if the programmed advertisement NI_TIMER is valid. FW sets this bit to indicate that the NI_TIMER is programmed. HW clears this bit on servicing the advertisment event 0 - ADV_NI timer is not valid 1 - ADV_NI timer is valid"]
    #[inline(always)]
    pub fn adv_ni_valid(&self) -> ADV_NI_VALID_R {
        ADV_NI_VALID_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - This bit indicates if the programmed scan NI_TIMER is valid. FW sets this bit to indicate that the NI_TIMER is programmed. HW clears this bit on servicing the scanner event 0 - SCAN_NI timer is not valid 1 - SCAN_NI timer is valid"]
    #[inline(always)]
    pub fn scan_ni_valid(&self) -> SCAN_NI_VALID_R {
        SCAN_NI_VALID_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - This bit indicates if the programmed initiator NI_TIMER is valid. FW sets this bit to indicate that the NI_TIMER is programmed. HW clears this bit on servicing the initiator event 0 - INIT_NI timer is not valid 1 - INIT_NI timer is valid"]
    #[inline(always)]
    pub fn init_ni_valid(&self) -> INIT_NI_VALID_R {
        INIT_NI_VALID_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit indicates if the programmed advertisement NI_TIMER is valid. FW sets this bit to indicate that the NI_TIMER is programmed. HW clears this bit on servicing the advertisment event 0 - ADV_NI timer is not valid 1 - ADV_NI timer is valid"]
    #[inline(always)]
    pub fn adv_ni_valid(&mut self) -> ADV_NI_VALID_W {
        ADV_NI_VALID_W { w: self }
    }
    #[doc = "Bit 1 - This bit indicates if the programmed scan NI_TIMER is valid. FW sets this bit to indicate that the NI_TIMER is programmed. HW clears this bit on servicing the scanner event 0 - SCAN_NI timer is not valid 1 - SCAN_NI timer is valid"]
    #[inline(always)]
    pub fn scan_ni_valid(&mut self) -> SCAN_NI_VALID_W {
        SCAN_NI_VALID_W { w: self }
    }
    #[doc = "Bit 2 - This bit indicates if the programmed initiator NI_TIMER is valid. FW sets this bit to indicate that the NI_TIMER is programmed. HW clears this bit on servicing the initiator event 0 - INIT_NI timer is not valid 1 - INIT_NI timer is valid"]
    #[inline(always)]
    pub fn init_ni_valid(&mut self) -> INIT_NI_VALID_W {
        INIT_NI_VALID_W { w: self }
    }
}
