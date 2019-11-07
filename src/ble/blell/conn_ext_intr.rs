#[doc = "Reader of register CONN_EXT_INTR"]
pub type R = crate::R<u32, super::CONN_EXT_INTR>;
#[doc = "Writer for register CONN_EXT_INTR"]
pub type W = crate::W<u32, super::CONN_EXT_INTR>;
#[doc = "Register CONN_EXT_INTR `reset()`'s with value 0"]
impl crate::ResetValue for super::CONN_EXT_INTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DATARATE_UPDATE`"]
pub type DATARATE_UPDATE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DATARATE_UPDATE`"]
pub struct DATARATE_UPDATE_W<'a> {
    w: &'a mut W,
}
impl<'a> DATARATE_UPDATE_W<'a> {
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
#[doc = "Reader of field `EARLY_INTR`"]
pub type EARLY_INTR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EARLY_INTR`"]
pub struct EARLY_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> EARLY_INTR_W<'a> {
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
#[doc = "Reader of field `GEN_TIMER_INTR`"]
pub type GEN_TIMER_INTR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GEN_TIMER_INTR`"]
pub struct GEN_TIMER_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> GEN_TIMER_INTR_W<'a> {
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
    #[doc = "Bit 0 - If this bit is set it indicates that the data rate is updated If this bit is written with 1, it clears the interrupt status bit"]
    #[inline(always)]
    pub fn datarate_update(&self) -> DATARATE_UPDATE_R {
        DATARATE_UPDATE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - For master this bit is set on start_ce For Slave this bit is set on slave_timer_adj"]
    #[inline(always)]
    pub fn early_intr(&self) -> EARLY_INTR_R {
        EARLY_INTR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - If this bit is set it indicates that the generic timer (PDU response timer reconfigured in MMMS mode) has expired If this bit is written with 1, it clears the interrupt status bit"]
    #[inline(always)]
    pub fn gen_timer_intr(&self) -> GEN_TIMER_INTR_R {
        GEN_TIMER_INTR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - If this bit is set it indicates that the data rate is updated If this bit is written with 1, it clears the interrupt status bit"]
    #[inline(always)]
    pub fn datarate_update(&mut self) -> DATARATE_UPDATE_W {
        DATARATE_UPDATE_W { w: self }
    }
    #[doc = "Bit 1 - For master this bit is set on start_ce For Slave this bit is set on slave_timer_adj"]
    #[inline(always)]
    pub fn early_intr(&mut self) -> EARLY_INTR_W {
        EARLY_INTR_W { w: self }
    }
    #[doc = "Bit 2 - If this bit is set it indicates that the generic timer (PDU response timer reconfigured in MMMS mode) has expired If this bit is written with 1, it clears the interrupt status bit"]
    #[inline(always)]
    pub fn gen_timer_intr(&mut self) -> GEN_TIMER_INTR_W {
        GEN_TIMER_INTR_W { w: self }
    }
}
