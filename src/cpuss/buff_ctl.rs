#[doc = "Reader of register BUFF_CTL"]
pub type R = crate::R<u32, super::BUFF_CTL>;
#[doc = "Writer for register BUFF_CTL"]
pub type W = crate::W<u32, super::BUFF_CTL>;
#[doc = "Register BUFF_CTL `reset()`'s with value 0x01"]
impl crate::ResetValue for super::BUFF_CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Reader of field `WRITE_BUFF`"]
pub type WRITE_BUFF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRITE_BUFF`"]
pub struct WRITE_BUFF_W<'a> {
    w: &'a mut W,
}
impl<'a> WRITE_BUFF_W<'a> {
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
    #[doc = "Bit 0 - Specifies if write transfer can be buffered in the bus infrastructure bridges: '0': Write transfers are not buffered, independent of the transfer's bufferable attribute. '1': Write transfers can be buffered, if the transfer's bufferable attribute indicates that the transfer is a bufferable/posted write."]
    #[inline(always)]
    pub fn write_buff(&self) -> WRITE_BUFF_R {
        WRITE_BUFF_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Specifies if write transfer can be buffered in the bus infrastructure bridges: '0': Write transfers are not buffered, independent of the transfer's bufferable attribute. '1': Write transfers can be buffered, if the transfer's bufferable attribute indicates that the transfer is a bufferable/posted write."]
    #[inline(always)]
    pub fn write_buff(&mut self) -> WRITE_BUFF_W {
        WRITE_BUFF_W { w: self }
    }
}
