#[doc = "Reader of register NEXT_CONN"]
pub type R = crate::R<u32, super::NEXT_CONN>;
#[doc = "Writer for register NEXT_CONN"]
pub type W = crate::W<u32, super::NEXT_CONN>;
#[doc = "Register NEXT_CONN `reset()`'s with value 0"]
impl crate::ResetValue for super::NEXT_CONN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `NEXT_CONN_INDEX`"]
pub type NEXT_CONN_INDEX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NEXT_CONN_INDEX`"]
pub struct NEXT_CONN_INDEX_W<'a> {
    w: &'a mut W,
}
impl<'a> NEXT_CONN_INDEX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Reader of field `NEXT_CONN_TYPE`"]
pub type NEXT_CONN_TYPE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NEXT_CONN_TYPE`"]
pub struct NEXT_CONN_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> NEXT_CONN_TYPE_W<'a> {
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
#[doc = "Reader of field `NI_VALID`"]
pub type NI_VALID_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NI_VALID`"]
pub struct NI_VALID_W<'a> {
    w: &'a mut W,
}
impl<'a> NI_VALID_W<'a> {
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
impl R {
    #[doc = "Bits 0:4 - Connection Index to be serviced. Allowed values are 0,1,2,3."]
    #[inline(always)]
    pub fn next_conn_index(&self) -> NEXT_CONN_INDEX_R {
        NEXT_CONN_INDEX_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - Connection type 1 - Master Connection 0 - Slave Connection"]
    #[inline(always)]
    pub fn next_conn_type(&self) -> NEXT_CONN_TYPE_R {
        NEXT_CONN_TYPE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Flag indication if programmed NI_TIMER is valid. FW sets this bit to indicate that the NI_TIMER is programmed. HW clears this bit on servicing the connection of if NI_TIMER is pointing to past value"]
    #[inline(always)]
    pub fn ni_valid(&self) -> NI_VALID_R {
        NI_VALID_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Connection Index to be serviced. Allowed values are 0,1,2,3."]
    #[inline(always)]
    pub fn next_conn_index(&mut self) -> NEXT_CONN_INDEX_W {
        NEXT_CONN_INDEX_W { w: self }
    }
    #[doc = "Bit 5 - Connection type 1 - Master Connection 0 - Slave Connection"]
    #[inline(always)]
    pub fn next_conn_type(&mut self) -> NEXT_CONN_TYPE_W {
        NEXT_CONN_TYPE_W { w: self }
    }
    #[doc = "Bit 6 - Flag indication if programmed NI_TIMER is valid. FW sets this bit to indicate that the NI_TIMER is programmed. HW clears this bit on servicing the connection of if NI_TIMER is pointing to past value"]
    #[inline(always)]
    pub fn ni_valid(&mut self) -> NI_VALID_W {
        NI_VALID_W { w: self }
    }
}
