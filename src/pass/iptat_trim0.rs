#[doc = "Reader of register IPTAT_TRIM0"]
pub type R = crate::R<u32, super::IPTAT_TRIM0>;
#[doc = "Writer for register IPTAT_TRIM0"]
pub type W = crate::W<u32, super::IPTAT_TRIM0>;
#[doc = "Register IPTAT_TRIM0 `reset()`'s with value 0"]
impl crate::ResetValue for super::IPTAT_TRIM0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IPTAT_CORE_TRIM`"]
pub type IPTAT_CORE_TRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IPTAT_CORE_TRIM`"]
pub struct IPTAT_CORE_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> IPTAT_CORE_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `IPTAT_CTBM_TRIM`"]
pub type IPTAT_CTBM_TRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IPTAT_CTBM_TRIM`"]
pub struct IPTAT_CTBM_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> IPTAT_CTBM_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - IPTAT trim 0x0 : Minimum IPTAT current (~150nA at room) 0xF : Maximum IPTAT current (~350nA at room)"]
    #[inline(always)]
    pub fn iptat_core_trim(&self) -> IPTAT_CORE_TRIM_R {
        IPTAT_CORE_TRIM_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - CTMB PTAT Current Trim 0x0 : Minimum CTMB IPTAT Current (~875nA) 0xF : Maximum CTMB IPTAT Current (~1.1uA)"]
    #[inline(always)]
    pub fn iptat_ctbm_trim(&self) -> IPTAT_CTBM_TRIM_R {
        IPTAT_CTBM_TRIM_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - IPTAT trim 0x0 : Minimum IPTAT current (~150nA at room) 0xF : Maximum IPTAT current (~350nA at room)"]
    #[inline(always)]
    pub fn iptat_core_trim(&mut self) -> IPTAT_CORE_TRIM_W {
        IPTAT_CORE_TRIM_W { w: self }
    }
    #[doc = "Bits 4:7 - CTMB PTAT Current Trim 0x0 : Minimum CTMB IPTAT Current (~875nA) 0xF : Maximum CTMB IPTAT Current (~1.1uA)"]
    #[inline(always)]
    pub fn iptat_ctbm_trim(&mut self) -> IPTAT_CTBM_TRIM_W {
        IPTAT_CTBM_TRIM_W { w: self }
    }
}
