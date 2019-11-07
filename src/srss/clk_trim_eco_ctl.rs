#[doc = "Reader of register CLK_TRIM_ECO_CTL"]
pub type R = crate::R<u32, super::CLK_TRIM_ECO_CTL>;
#[doc = "Writer for register CLK_TRIM_ECO_CTL"]
pub type W = crate::W<u32, super::CLK_TRIM_ECO_CTL>;
#[doc = "Register CLK_TRIM_ECO_CTL `reset()`'s with value 0x001f_0003"]
impl crate::ResetValue for super::CLK_TRIM_ECO_CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x001f_0003
    }
}
#[doc = "Reader of field `WDTRIM`"]
pub type WDTRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WDTRIM`"]
pub struct WDTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `ATRIM`"]
pub type ATRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ATRIM`"]
pub struct ATRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> ATRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `FTRIM`"]
pub type FTRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FTRIM`"]
pub struct FTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> FTRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `RTRIM`"]
pub type RTRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTRIM`"]
pub struct RTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RTRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Reader of field `GTRIM`"]
pub type GTRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `GTRIM`"]
pub struct GTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> GTRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `ITRIM`"]
pub type ITRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ITRIM`"]
pub struct ITRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> ITRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | (((value as u32) & 0x3f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Watch Dog Trim - Delta voltage below stead state level 0x0 - 50mV 0x1 - 75mV 0x2 - 100mV 0x3 - 125mV 0x4 - 150mV 0x5 - 175mV 0x6 - 200mV 0x7 - 225mV"]
    #[inline(always)]
    pub fn wdtrim(&self) -> WDTRIM_R {
        WDTRIM_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:7 - Amplitude trim to set the crystal drive level when ECO_CONFIG.AGC_EN=1. WARNING: use care when setting this field because driving a crystal beyond its rated limit can permanently damage the crystal. 0x0 - 150mV 0x1 - 175mV 0x2 - 200mV 0x3 - 225mV 0x4 - 250mV 0x5 - 275mV 0x6 - 300mV 0x7 - 325mV 0x8 - 350mV 0x9 - 375mV 0xA - 400mV 0xB - 425mV 0xC - 450mV 0xD - 475mV 0xE - 500mV 0xF - 525mV"]
    #[inline(always)]
    pub fn atrim(&self) -> ATRIM_R {
        ATRIM_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - Filter Trim - 3rd harmonic oscillation"]
    #[inline(always)]
    pub fn ftrim(&self) -> FTRIM_R {
        FTRIM_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Feedback resistor Trim"]
    #[inline(always)]
    pub fn rtrim(&self) -> RTRIM_R {
        RTRIM_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Gain Trim - Startup time"]
    #[inline(always)]
    pub fn gtrim(&self) -> GTRIM_R {
        GTRIM_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 16:21 - Current Trim"]
    #[inline(always)]
    pub fn itrim(&self) -> ITRIM_R {
        ITRIM_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Watch Dog Trim - Delta voltage below stead state level 0x0 - 50mV 0x1 - 75mV 0x2 - 100mV 0x3 - 125mV 0x4 - 150mV 0x5 - 175mV 0x6 - 200mV 0x7 - 225mV"]
    #[inline(always)]
    pub fn wdtrim(&mut self) -> WDTRIM_W {
        WDTRIM_W { w: self }
    }
    #[doc = "Bits 4:7 - Amplitude trim to set the crystal drive level when ECO_CONFIG.AGC_EN=1. WARNING: use care when setting this field because driving a crystal beyond its rated limit can permanently damage the crystal. 0x0 - 150mV 0x1 - 175mV 0x2 - 200mV 0x3 - 225mV 0x4 - 250mV 0x5 - 275mV 0x6 - 300mV 0x7 - 325mV 0x8 - 350mV 0x9 - 375mV 0xA - 400mV 0xB - 425mV 0xC - 450mV 0xD - 475mV 0xE - 500mV 0xF - 525mV"]
    #[inline(always)]
    pub fn atrim(&mut self) -> ATRIM_W {
        ATRIM_W { w: self }
    }
    #[doc = "Bits 8:9 - Filter Trim - 3rd harmonic oscillation"]
    #[inline(always)]
    pub fn ftrim(&mut self) -> FTRIM_W {
        FTRIM_W { w: self }
    }
    #[doc = "Bits 10:11 - Feedback resistor Trim"]
    #[inline(always)]
    pub fn rtrim(&mut self) -> RTRIM_W {
        RTRIM_W { w: self }
    }
    #[doc = "Bits 12:13 - Gain Trim - Startup time"]
    #[inline(always)]
    pub fn gtrim(&mut self) -> GTRIM_W {
        GTRIM_W { w: self }
    }
    #[doc = "Bits 16:21 - Current Trim"]
    #[inline(always)]
    pub fn itrim(&mut self) -> ITRIM_W {
        ITRIM_W { w: self }
    }
}
