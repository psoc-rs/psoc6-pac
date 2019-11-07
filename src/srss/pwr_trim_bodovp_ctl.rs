#[doc = "Reader of register PWR_TRIM_BODOVP_CTL"]
pub type R = crate::R<u32, super::PWR_TRIM_BODOVP_CTL>;
#[doc = "Writer for register PWR_TRIM_BODOVP_CTL"]
pub type W = crate::W<u32, super::PWR_TRIM_BODOVP_CTL>;
#[doc = "Register PWR_TRIM_BODOVP_CTL `reset()`'s with value 0x0004_0d04"]
impl crate::ResetValue for super::PWR_TRIM_BODOVP_CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0004_0d04
    }
}
#[doc = "Reader of field `HVPORBOD_TRIPSEL`"]
pub type HVPORBOD_TRIPSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HVPORBOD_TRIPSEL`"]
pub struct HVPORBOD_TRIPSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> HVPORBOD_TRIPSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `HVPORBOD_OFSTRIM`"]
pub type HVPORBOD_OFSTRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HVPORBOD_OFSTRIM`"]
pub struct HVPORBOD_OFSTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> HVPORBOD_OFSTRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `HVPORBOD_ITRIM`"]
pub type HVPORBOD_ITRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HVPORBOD_ITRIM`"]
pub struct HVPORBOD_ITRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> HVPORBOD_ITRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 7)) | (((value as u32) & 0x07) << 7);
        self.w
    }
}
#[doc = "Reader of field `LVPORBOD_TRIPSEL`"]
pub type LVPORBOD_TRIPSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LVPORBOD_TRIPSEL`"]
pub struct LVPORBOD_TRIPSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> LVPORBOD_TRIPSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 10)) | (((value as u32) & 0x07) << 10);
        self.w
    }
}
#[doc = "Reader of field `LVPORBOD_OFSTRIM`"]
pub type LVPORBOD_OFSTRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LVPORBOD_OFSTRIM`"]
pub struct LVPORBOD_OFSTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> LVPORBOD_OFSTRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 14)) | (((value as u32) & 0x07) << 14);
        self.w
    }
}
#[doc = "Reader of field `LVPORBOD_ITRIM`"]
pub type LVPORBOD_ITRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LVPORBOD_ITRIM`"]
pub struct LVPORBOD_ITRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> LVPORBOD_ITRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 17)) | (((value as u32) & 0x07) << 17);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - HVPORBOD trip point selection. Monitors vddd. This register is only reset by XRES/POR/BOD/OVP/HIBERNATE."]
    #[inline(always)]
    pub fn hvporbod_tripsel(&self) -> HVPORBOD_TRIPSEL_R {
        HVPORBOD_TRIPSEL_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:6 - HVPORBOD offset trim. This register is only reset by XRES/POR/BOD/OVP/HIBERNATE."]
    #[inline(always)]
    pub fn hvporbod_ofstrim(&self) -> HVPORBOD_OFSTRIM_R {
        HVPORBOD_OFSTRIM_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 7:9 - HVPORBOD current trim. This register is only reset by XRES/POR/BOD/OVP/HIBERNATE."]
    #[inline(always)]
    pub fn hvporbod_itrim(&self) -> HVPORBOD_ITRIM_R {
        HVPORBOD_ITRIM_R::new(((self.bits >> 7) & 0x07) as u8)
    }
    #[doc = "Bits 10:12 - LVPORBOD trip point selection. Monitors vccd. This register is only reset by XRES/POR/BOD/OVP/HIBERNATE."]
    #[inline(always)]
    pub fn lvporbod_tripsel(&self) -> LVPORBOD_TRIPSEL_R {
        LVPORBOD_TRIPSEL_R::new(((self.bits >> 10) & 0x07) as u8)
    }
    #[doc = "Bits 14:16 - LVPORBOD offset trim. This register is only reset by XRES/POR/BOD/OVP/HIBERNATE."]
    #[inline(always)]
    pub fn lvporbod_ofstrim(&self) -> LVPORBOD_OFSTRIM_R {
        LVPORBOD_OFSTRIM_R::new(((self.bits >> 14) & 0x07) as u8)
    }
    #[doc = "Bits 17:19 - LVPORBOD current trim. This register is only reset by XRES/POR/BOD/OVP/HIBERNATE."]
    #[inline(always)]
    pub fn lvporbod_itrim(&self) -> LVPORBOD_ITRIM_R {
        LVPORBOD_ITRIM_R::new(((self.bits >> 17) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - HVPORBOD trip point selection. Monitors vddd. This register is only reset by XRES/POR/BOD/OVP/HIBERNATE."]
    #[inline(always)]
    pub fn hvporbod_tripsel(&mut self) -> HVPORBOD_TRIPSEL_W {
        HVPORBOD_TRIPSEL_W { w: self }
    }
    #[doc = "Bits 4:6 - HVPORBOD offset trim. This register is only reset by XRES/POR/BOD/OVP/HIBERNATE."]
    #[inline(always)]
    pub fn hvporbod_ofstrim(&mut self) -> HVPORBOD_OFSTRIM_W {
        HVPORBOD_OFSTRIM_W { w: self }
    }
    #[doc = "Bits 7:9 - HVPORBOD current trim. This register is only reset by XRES/POR/BOD/OVP/HIBERNATE."]
    #[inline(always)]
    pub fn hvporbod_itrim(&mut self) -> HVPORBOD_ITRIM_W {
        HVPORBOD_ITRIM_W { w: self }
    }
    #[doc = "Bits 10:12 - LVPORBOD trip point selection. Monitors vccd. This register is only reset by XRES/POR/BOD/OVP/HIBERNATE."]
    #[inline(always)]
    pub fn lvporbod_tripsel(&mut self) -> LVPORBOD_TRIPSEL_W {
        LVPORBOD_TRIPSEL_W { w: self }
    }
    #[doc = "Bits 14:16 - LVPORBOD offset trim. This register is only reset by XRES/POR/BOD/OVP/HIBERNATE."]
    #[inline(always)]
    pub fn lvporbod_ofstrim(&mut self) -> LVPORBOD_OFSTRIM_W {
        LVPORBOD_OFSTRIM_W { w: self }
    }
    #[doc = "Bits 17:19 - LVPORBOD current trim. This register is only reset by XRES/POR/BOD/OVP/HIBERNATE."]
    #[inline(always)]
    pub fn lvporbod_itrim(&mut self) -> LVPORBOD_ITRIM_W {
        LVPORBOD_ITRIM_W { w: self }
    }
}
