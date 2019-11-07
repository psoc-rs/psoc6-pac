#[doc = "Reader of register CONN_PARAM1"]
pub type R = crate::R<u32, super::CONN_PARAM1>;
#[doc = "Writer for register CONN_PARAM1"]
pub type W = crate::W<u32, super::CONN_PARAM1>;
#[doc = "Register CONN_PARAM1 `reset()`'s with value 0"]
impl crate::ResetValue for super::CONN_PARAM1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SCA_PARAM`"]
pub type SCA_PARAM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SCA_PARAM`"]
pub struct SCA_PARAM_W<'a> {
    w: &'a mut W,
}
impl<'a> SCA_PARAM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `HOP_INCREMENT_PARAM`"]
pub type HOP_INCREMENT_PARAM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HOP_INCREMENT_PARAM`"]
pub struct HOP_INCREMENT_PARAM_W<'a> {
    w: &'a mut W,
}
impl<'a> HOP_INCREMENT_PARAM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 3)) | (((value as u32) & 0x1f) << 3);
        self.w
    }
}
#[doc = "Reader of field `CRC_INIT_L`"]
pub type CRC_INIT_L_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CRC_INIT_L`"]
pub struct CRC_INIT_L_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC_INIT_L_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Sleep Clock Accuracy"]
    #[inline(always)]
    pub fn sca_param(&self) -> SCA_PARAM_R {
        SCA_PARAM_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 3:7 - Hop increment for connection channel."]
    #[inline(always)]
    pub fn hop_increment_param(&self) -> HOP_INCREMENT_PARAM_R {
        HOP_INCREMENT_PARAM_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 8:15 - This field defines the lower byte (7:0) of the CRC initialization vector."]
    #[inline(always)]
    pub fn crc_init_l(&self) -> CRC_INIT_L_R {
        CRC_INIT_L_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Sleep Clock Accuracy"]
    #[inline(always)]
    pub fn sca_param(&mut self) -> SCA_PARAM_W {
        SCA_PARAM_W { w: self }
    }
    #[doc = "Bits 3:7 - Hop increment for connection channel."]
    #[inline(always)]
    pub fn hop_increment_param(&mut self) -> HOP_INCREMENT_PARAM_W {
        HOP_INCREMENT_PARAM_W { w: self }
    }
    #[doc = "Bits 8:15 - This field defines the lower byte (7:0) of the CRC initialization vector."]
    #[inline(always)]
    pub fn crc_init_l(&mut self) -> CRC_INIT_L_W {
        CRC_INIT_L_W { w: self }
    }
}
