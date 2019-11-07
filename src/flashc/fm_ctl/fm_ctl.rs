#[doc = "Reader of register FM_CTL"]
pub type R = crate::R<u32, super::FM_CTL>;
#[doc = "Writer for register FM_CTL"]
pub type W = crate::W<u32, super::FM_CTL>;
#[doc = "Register FM_CTL `reset()`'s with value 0"]
impl crate::ResetValue for super::FM_CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FM_MODE`"]
pub type FM_MODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FM_MODE`"]
pub struct FM_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> FM_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `FM_SEQ`"]
pub type FM_SEQ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FM_SEQ`"]
pub struct FM_SEQ_W<'a> {
    w: &'a mut W,
}
impl<'a> FM_SEQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `DAA_MUX_SEL`"]
pub type DAA_MUX_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DAA_MUX_SEL`"]
pub struct DAA_MUX_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DAA_MUX_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | (((value as u32) & 0x7f) << 16);
        self.w
    }
}
#[doc = "Reader of field `IF_SEL`"]
pub type IF_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IF_SEL`"]
pub struct IF_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> IF_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `WR_EN`"]
pub type WR_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WR_EN`"]
pub struct WR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> WR_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Flash macro mode selection: '0': Normal functional mode. '1': Sets 'pre-program control bit' for soft pre-program operation of all selected SONOS cells. the control bit is cleared by the HW after any program operation. '2': Sets ... '15': TBD"]
    #[inline(always)]
    pub fn fm_mode(&self) -> FM_MODE_R {
        FM_MODE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - Flash macro sequence select: '0': TBD '1': TBD '2': TBD '3': TBD"]
    #[inline(always)]
    pub fn fm_seq(&self) -> FM_SEQ_R {
        FM_SEQ_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 16:22 - Direct memory cell access address."]
    #[inline(always)]
    pub fn daa_mux_sel(&self) -> DAA_MUX_SEL_R {
        DAA_MUX_SEL_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 24 - Interface selection. Specifies the interface that is used for flash memory read operations: '0': R interface is used (default value). In this case, the flash memory address is provided as part of the R signal interface. '1': C interface is used. In this case, the flash memory address is provided by FM_MEM_ADDR (the page address) and by the C interface access offset in the FM_MEM_DATA structure."]
    #[inline(always)]
    pub fn if_sel(&self) -> IF_SEL_R {
        IF_SEL_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - '0': normal mode '1': Fm Write Enable"]
    #[inline(always)]
    pub fn wr_en(&self) -> WR_EN_R {
        WR_EN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Flash macro mode selection: '0': Normal functional mode. '1': Sets 'pre-program control bit' for soft pre-program operation of all selected SONOS cells. the control bit is cleared by the HW after any program operation. '2': Sets ... '15': TBD"]
    #[inline(always)]
    pub fn fm_mode(&mut self) -> FM_MODE_W {
        FM_MODE_W { w: self }
    }
    #[doc = "Bits 8:9 - Flash macro sequence select: '0': TBD '1': TBD '2': TBD '3': TBD"]
    #[inline(always)]
    pub fn fm_seq(&mut self) -> FM_SEQ_W {
        FM_SEQ_W { w: self }
    }
    #[doc = "Bits 16:22 - Direct memory cell access address."]
    #[inline(always)]
    pub fn daa_mux_sel(&mut self) -> DAA_MUX_SEL_W {
        DAA_MUX_SEL_W { w: self }
    }
    #[doc = "Bit 24 - Interface selection. Specifies the interface that is used for flash memory read operations: '0': R interface is used (default value). In this case, the flash memory address is provided as part of the R signal interface. '1': C interface is used. In this case, the flash memory address is provided by FM_MEM_ADDR (the page address) and by the C interface access offset in the FM_MEM_DATA structure."]
    #[inline(always)]
    pub fn if_sel(&mut self) -> IF_SEL_W {
        IF_SEL_W { w: self }
    }
    #[doc = "Bit 25 - '0': normal mode '1': Fm Write Enable"]
    #[inline(always)]
    pub fn wr_en(&mut self) -> WR_EN_W {
        WR_EN_W { w: self }
    }
}
