#[doc = "Reader of register FLOW_CTL"]
pub type R = crate::R<u32, super::FLOW_CTL>;
#[doc = "Writer for register FLOW_CTL"]
pub type W = crate::W<u32, super::FLOW_CTL>;
#[doc = "Register FLOW_CTL `reset()`'s with value 0"]
impl crate::ResetValue for super::FLOW_CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EP1_ERR_RESP`"]
pub type EP1_ERR_RESP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP1_ERR_RESP`"]
pub struct EP1_ERR_RESP_W<'a> {
    w: &'a mut W,
}
impl<'a> EP1_ERR_RESP_W<'a> {
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
#[doc = "Reader of field `EP2_ERR_RESP`"]
pub type EP2_ERR_RESP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP2_ERR_RESP`"]
pub struct EP2_ERR_RESP_W<'a> {
    w: &'a mut W,
}
impl<'a> EP2_ERR_RESP_W<'a> {
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
#[doc = "Reader of field `EP3_ERR_RESP`"]
pub type EP3_ERR_RESP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP3_ERR_RESP`"]
pub struct EP3_ERR_RESP_W<'a> {
    w: &'a mut W,
}
impl<'a> EP3_ERR_RESP_W<'a> {
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
#[doc = "Reader of field `EP4_ERR_RESP`"]
pub type EP4_ERR_RESP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP4_ERR_RESP`"]
pub struct EP4_ERR_RESP_W<'a> {
    w: &'a mut W,
}
impl<'a> EP4_ERR_RESP_W<'a> {
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
#[doc = "Reader of field `EP5_ERR_RESP`"]
pub type EP5_ERR_RESP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP5_ERR_RESP`"]
pub struct EP5_ERR_RESP_W<'a> {
    w: &'a mut W,
}
impl<'a> EP5_ERR_RESP_W<'a> {
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
#[doc = "Reader of field `EP6_ERR_RESP`"]
pub type EP6_ERR_RESP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP6_ERR_RESP`"]
pub struct EP6_ERR_RESP_W<'a> {
    w: &'a mut W,
}
impl<'a> EP6_ERR_RESP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `EP7_ERR_RESP`"]
pub type EP7_ERR_RESP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP7_ERR_RESP`"]
pub struct EP7_ERR_RESP_W<'a> {
    w: &'a mut W,
}
impl<'a> EP7_ERR_RESP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `EP8_ERR_RESP`"]
pub type EP8_ERR_RESP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EP8_ERR_RESP`"]
pub struct EP8_ERR_RESP_W<'a> {
    w: &'a mut W,
}
impl<'a> EP8_ERR_RESP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - End Point 1 error response 0: do nothing (backward compatibility mode) 1: if this is an IN EP and an underflow occurs then cause a CRC error, if this is an OUT EP and an overflow occurs then send a NAK"]
    #[inline(always)]
    pub fn ep1_err_resp(&self) -> EP1_ERR_RESP_R {
        EP1_ERR_RESP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - End Point 2 error response"]
    #[inline(always)]
    pub fn ep2_err_resp(&self) -> EP2_ERR_RESP_R {
        EP2_ERR_RESP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - End Point 3 error response"]
    #[inline(always)]
    pub fn ep3_err_resp(&self) -> EP3_ERR_RESP_R {
        EP3_ERR_RESP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - End Point 4 error response"]
    #[inline(always)]
    pub fn ep4_err_resp(&self) -> EP4_ERR_RESP_R {
        EP4_ERR_RESP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - End Point 5 error response"]
    #[inline(always)]
    pub fn ep5_err_resp(&self) -> EP5_ERR_RESP_R {
        EP5_ERR_RESP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - End Point 6 error response"]
    #[inline(always)]
    pub fn ep6_err_resp(&self) -> EP6_ERR_RESP_R {
        EP6_ERR_RESP_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - End Point 7 error response"]
    #[inline(always)]
    pub fn ep7_err_resp(&self) -> EP7_ERR_RESP_R {
        EP7_ERR_RESP_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - End Point 8 error response"]
    #[inline(always)]
    pub fn ep8_err_resp(&self) -> EP8_ERR_RESP_R {
        EP8_ERR_RESP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - End Point 1 error response 0: do nothing (backward compatibility mode) 1: if this is an IN EP and an underflow occurs then cause a CRC error, if this is an OUT EP and an overflow occurs then send a NAK"]
    #[inline(always)]
    pub fn ep1_err_resp(&mut self) -> EP1_ERR_RESP_W {
        EP1_ERR_RESP_W { w: self }
    }
    #[doc = "Bit 1 - End Point 2 error response"]
    #[inline(always)]
    pub fn ep2_err_resp(&mut self) -> EP2_ERR_RESP_W {
        EP2_ERR_RESP_W { w: self }
    }
    #[doc = "Bit 2 - End Point 3 error response"]
    #[inline(always)]
    pub fn ep3_err_resp(&mut self) -> EP3_ERR_RESP_W {
        EP3_ERR_RESP_W { w: self }
    }
    #[doc = "Bit 3 - End Point 4 error response"]
    #[inline(always)]
    pub fn ep4_err_resp(&mut self) -> EP4_ERR_RESP_W {
        EP4_ERR_RESP_W { w: self }
    }
    #[doc = "Bit 4 - End Point 5 error response"]
    #[inline(always)]
    pub fn ep5_err_resp(&mut self) -> EP5_ERR_RESP_W {
        EP5_ERR_RESP_W { w: self }
    }
    #[doc = "Bit 5 - End Point 6 error response"]
    #[inline(always)]
    pub fn ep6_err_resp(&mut self) -> EP6_ERR_RESP_W {
        EP6_ERR_RESP_W { w: self }
    }
    #[doc = "Bit 6 - End Point 7 error response"]
    #[inline(always)]
    pub fn ep7_err_resp(&mut self) -> EP7_ERR_RESP_W {
        EP7_ERR_RESP_W { w: self }
    }
    #[doc = "Bit 7 - End Point 8 error response"]
    #[inline(always)]
    pub fn ep8_err_resp(&mut self) -> EP8_ERR_RESP_W {
        EP8_ERR_RESP_W { w: self }
    }
}
