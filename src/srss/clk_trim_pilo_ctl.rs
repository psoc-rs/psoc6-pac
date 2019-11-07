#[doc = "Reader of register CLK_TRIM_PILO_CTL"]
pub type R = crate::R<u32, super::CLK_TRIM_PILO_CTL>;
#[doc = "Writer for register CLK_TRIM_PILO_CTL"]
pub type W = crate::W<u32, super::CLK_TRIM_PILO_CTL>;
#[doc = "Register CLK_TRIM_PILO_CTL `reset()`'s with value 0x0108_500f"]
impl crate::ResetValue for super::CLK_TRIM_PILO_CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0108_500f
    }
}
#[doc = "Reader of field `PILO_CFREQ`"]
pub type PILO_CFREQ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PILO_CFREQ`"]
pub struct PILO_CFREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> PILO_CFREQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Reader of field `PILO_OSC_TRIM`"]
pub type PILO_OSC_TRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PILO_OSC_TRIM`"]
pub struct PILO_OSC_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> PILO_OSC_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
#[doc = "Reader of field `PILO_COMP_TRIM`"]
pub type PILO_COMP_TRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PILO_COMP_TRIM`"]
pub struct PILO_COMP_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> PILO_COMP_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `PILO_NBIAS_TRIM`"]
pub type PILO_NBIAS_TRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PILO_NBIAS_TRIM`"]
pub struct PILO_NBIAS_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> PILO_NBIAS_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "Reader of field `PILO_RES_TRIM`"]
pub type PILO_RES_TRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PILO_RES_TRIM`"]
pub struct PILO_RES_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> PILO_RES_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 20)) | (((value as u32) & 0x1f) << 20);
        self.w
    }
}
#[doc = "Reader of field `PILO_ISLOPE_TRIM`"]
pub type PILO_ISLOPE_TRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PILO_ISLOPE_TRIM`"]
pub struct PILO_ISLOPE_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> PILO_ISLOPE_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
        self.w
    }
}
#[doc = "Reader of field `PILO_VTDIFF_TRIM`"]
pub type PILO_VTDIFF_TRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PILO_VTDIFF_TRIM`"]
pub struct PILO_VTDIFF_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> PILO_VTDIFF_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 28)) | (((value as u32) & 0x07) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Coarse frequency trim to meet 32.768kHz +/-2 percent across PVT without calibration. The nominal step size of the LSB is 1kHz."]
    #[inline(always)]
    pub fn pilo_cfreq(&self) -> PILO_CFREQ_R {
        PILO_CFREQ_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 12:14 - Trim for current in oscillator block."]
    #[inline(always)]
    pub fn pilo_osc_trim(&self) -> PILO_OSC_TRIM_R {
        PILO_OSC_TRIM_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 16:17 - Trim for comparator bias current."]
    #[inline(always)]
    pub fn pilo_comp_trim(&self) -> PILO_COMP_TRIM_R {
        PILO_COMP_TRIM_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - Trim for biasn by trimming sub-Vth NMOS width in beta-multiplier"]
    #[inline(always)]
    pub fn pilo_nbias_trim(&self) -> PILO_NBIAS_TRIM_R {
        PILO_NBIAS_TRIM_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:24 - Trim for beta-multiplier branch current"]
    #[inline(always)]
    pub fn pilo_res_trim(&self) -> PILO_RES_TRIM_R {
        PILO_RES_TRIM_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 26:27 - Trim for beta-multiplier current slope"]
    #[inline(always)]
    pub fn pilo_islope_trim(&self) -> PILO_ISLOPE_TRIM_R {
        PILO_ISLOPE_TRIM_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 28:30 - Trim for VT-DIFF output (internal power supply)"]
    #[inline(always)]
    pub fn pilo_vtdiff_trim(&self) -> PILO_VTDIFF_TRIM_R {
        PILO_VTDIFF_TRIM_R::new(((self.bits >> 28) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Coarse frequency trim to meet 32.768kHz +/-2 percent across PVT without calibration. The nominal step size of the LSB is 1kHz."]
    #[inline(always)]
    pub fn pilo_cfreq(&mut self) -> PILO_CFREQ_W {
        PILO_CFREQ_W { w: self }
    }
    #[doc = "Bits 12:14 - Trim for current in oscillator block."]
    #[inline(always)]
    pub fn pilo_osc_trim(&mut self) -> PILO_OSC_TRIM_W {
        PILO_OSC_TRIM_W { w: self }
    }
    #[doc = "Bits 16:17 - Trim for comparator bias current."]
    #[inline(always)]
    pub fn pilo_comp_trim(&mut self) -> PILO_COMP_TRIM_W {
        PILO_COMP_TRIM_W { w: self }
    }
    #[doc = "Bits 18:19 - Trim for biasn by trimming sub-Vth NMOS width in beta-multiplier"]
    #[inline(always)]
    pub fn pilo_nbias_trim(&mut self) -> PILO_NBIAS_TRIM_W {
        PILO_NBIAS_TRIM_W { w: self }
    }
    #[doc = "Bits 20:24 - Trim for beta-multiplier branch current"]
    #[inline(always)]
    pub fn pilo_res_trim(&mut self) -> PILO_RES_TRIM_W {
        PILO_RES_TRIM_W { w: self }
    }
    #[doc = "Bits 26:27 - Trim for beta-multiplier current slope"]
    #[inline(always)]
    pub fn pilo_islope_trim(&mut self) -> PILO_ISLOPE_TRIM_W {
        PILO_ISLOPE_TRIM_W { w: self }
    }
    #[doc = "Bits 28:30 - Trim for VT-DIFF output (internal power supply)"]
    #[inline(always)]
    pub fn pilo_vtdiff_trim(&mut self) -> PILO_VTDIFF_TRIM_W {
        PILO_VTDIFF_TRIM_W { w: self }
    }
}
