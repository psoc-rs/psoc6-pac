#[doc = "Reader of register ENC_INTR"]
pub type R = crate::R<u32, super::ENC_INTR>;
#[doc = "Writer for register ENC_INTR"]
pub type W = crate::W<u32, super::ENC_INTR>;
#[doc = "Register ENC_INTR `reset()`'s with value 0"]
impl crate::ResetValue for super::ENC_INTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `AUTH_PASS_INTR`"]
pub type AUTH_PASS_INTR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUTH_PASS_INTR`"]
pub struct AUTH_PASS_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTH_PASS_INTR_W<'a> {
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
#[doc = "Reader of field `ECB_PROC_INTR`"]
pub type ECB_PROC_INTR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ECB_PROC_INTR`"]
pub struct ECB_PROC_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> ECB_PROC_INTR_W<'a> {
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
#[doc = "Reader of field `CCM_PROC_INTR`"]
pub type CCM_PROC_INTR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CCM_PROC_INTR`"]
pub struct CCM_PROC_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> CCM_PROC_INTR_W<'a> {
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
#[doc = "Reader of field `IN_DATA_CLEAR`"]
pub type IN_DATA_CLEAR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IN_DATA_CLEAR`"]
pub struct IN_DATA_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> IN_DATA_CLEAR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Authentication interrupt. 0x1- indicates MIC matched 0x0 -indicated MIC mismatched Writing 1 to this register clears the interrupt."]
    #[inline(always)]
    pub fn auth_pass_intr(&self) -> AUTH_PASS_INTR_R {
        AUTH_PASS_INTR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ECB processed interrupt. Writing 1 to this register clears the interrupt."]
    #[inline(always)]
    pub fn ecb_proc_intr(&self) -> ECB_PROC_INTR_R {
        ECB_PROC_INTR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - CCM processed interrupt. Writing 1 to this register clears the interrupt"]
    #[inline(always)]
    pub fn ccm_proc_intr(&self) -> CCM_PROC_INTR_R {
        CCM_PROC_INTR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Clears the input data. Used for Zero padding of encryption for less than block sized data."]
    #[inline(always)]
    pub fn in_data_clear(&self) -> IN_DATA_CLEAR_R {
        IN_DATA_CLEAR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Authentication interrupt. 0x1- indicates MIC matched 0x0 -indicated MIC mismatched Writing 1 to this register clears the interrupt."]
    #[inline(always)]
    pub fn auth_pass_intr(&mut self) -> AUTH_PASS_INTR_W {
        AUTH_PASS_INTR_W { w: self }
    }
    #[doc = "Bit 1 - ECB processed interrupt. Writing 1 to this register clears the interrupt."]
    #[inline(always)]
    pub fn ecb_proc_intr(&mut self) -> ECB_PROC_INTR_W {
        ECB_PROC_INTR_W { w: self }
    }
    #[doc = "Bit 2 - CCM processed interrupt. Writing 1 to this register clears the interrupt"]
    #[inline(always)]
    pub fn ccm_proc_intr(&mut self) -> CCM_PROC_INTR_W {
        CCM_PROC_INTR_W { w: self }
    }
    #[doc = "Bit 3 - Clears the input data. Used for Zero padding of encryption for less than block sized data."]
    #[inline(always)]
    pub fn in_data_clear(&mut self) -> IN_DATA_CLEAR_W {
        IN_DATA_CLEAR_W { w: self }
    }
}
