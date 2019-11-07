#[doc = "Reader of register ENC_INTR_EN"]
pub type R = crate::R<u32, super::ENC_INTR_EN>;
#[doc = "Writer for register ENC_INTR_EN"]
pub type W = crate::W<u32, super::ENC_INTR_EN>;
#[doc = "Register ENC_INTR_EN `reset()`'s with value 0"]
impl crate::ResetValue for super::ENC_INTR_EN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `AUTH_PASS_INTR_EN`"]
pub type AUTH_PASS_INTR_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUTH_PASS_INTR_EN`"]
pub struct AUTH_PASS_INTR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTH_PASS_INTR_EN_W<'a> {
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
#[doc = "Reader of field `ECB_PROC_INTR_EN`"]
pub type ECB_PROC_INTR_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ECB_PROC_INTR_EN`"]
pub struct ECB_PROC_INTR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ECB_PROC_INTR_EN_W<'a> {
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
#[doc = "Reader of field `CCM_PROC_INTR_EN`"]
pub type CCM_PROC_INTR_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CCM_PROC_INTR_EN`"]
pub struct CCM_PROC_INTR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CCM_PROC_INTR_EN_W<'a> {
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
    #[doc = "Bit 0 - Authentication interrupt enable 0 - Disable 1 - Enable"]
    #[inline(always)]
    pub fn auth_pass_intr_en(&self) -> AUTH_PASS_INTR_EN_R {
        AUTH_PASS_INTR_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ECB processed interrupt enable 0 - Disable 1 - Enable"]
    #[inline(always)]
    pub fn ecb_proc_intr_en(&self) -> ECB_PROC_INTR_EN_R {
        ECB_PROC_INTR_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - CCM processed interupt enable 0 - Disable 1 - Enable"]
    #[inline(always)]
    pub fn ccm_proc_intr_en(&self) -> CCM_PROC_INTR_EN_R {
        CCM_PROC_INTR_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Authentication interrupt enable 0 - Disable 1 - Enable"]
    #[inline(always)]
    pub fn auth_pass_intr_en(&mut self) -> AUTH_PASS_INTR_EN_W {
        AUTH_PASS_INTR_EN_W { w: self }
    }
    #[doc = "Bit 1 - ECB processed interrupt enable 0 - Disable 1 - Enable"]
    #[inline(always)]
    pub fn ecb_proc_intr_en(&mut self) -> ECB_PROC_INTR_EN_W {
        ECB_PROC_INTR_EN_W { w: self }
    }
    #[doc = "Bit 2 - CCM processed interupt enable 0 - Disable 1 - Enable"]
    #[inline(always)]
    pub fn ccm_proc_intr_en(&mut self) -> CCM_PROC_INTR_EN_W {
        CCM_PROC_INTR_EN_W { w: self }
    }
}
