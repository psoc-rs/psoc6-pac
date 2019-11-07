#[doc = "Reader of register DU_SEL"]
pub type R = crate::R<u32, super::DU_SEL>;
#[doc = "Writer for register DU_SEL"]
pub type W = crate::W<u32, super::DU_SEL>;
#[doc = "Register DU_SEL `reset()`'s with value 0"]
impl crate::ResetValue for super::DU_SEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DU_TR0_SEL`"]
pub type DU_TR0_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DU_TR0_SEL`"]
pub struct DU_TR0_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DU_TR0_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `DU_TR1_SEL`"]
pub type DU_TR1_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DU_TR1_SEL`"]
pub struct DU_TR1_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DU_TR1_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `DU_TR2_SEL`"]
pub type DU_TR2_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DU_TR2_SEL`"]
pub struct DU_TR2_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DU_TR2_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `DU_DATA0_SEL`"]
pub type DU_DATA0_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DU_DATA0_SEL`"]
pub struct DU_DATA0_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DU_DATA0_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Reader of field `DU_DATA1_SEL`"]
pub type DU_DATA1_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DU_DATA1_SEL`"]
pub struct DU_DATA1_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DU_DATA1_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Data unit input signal 'tr0_in' source selection: '0': Constant '0'. '1': Constant '1'. '2': Data unit output. '10-3': LUT 7 - 0 outputs. Otherwise: Undefined."]
    #[inline(always)]
    pub fn du_tr0_sel(&self) -> DU_TR0_SEL_R {
        DU_TR0_SEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Data unit input signal 'tr1_in' source selection. Encoding is the same as for DU_TR0_SEL."]
    #[inline(always)]
    pub fn du_tr1_sel(&self) -> DU_TR1_SEL_R {
        DU_TR1_SEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Data unit input signal 'tr2_in' source selection. Encoding is the same as for DU_TR0_SEL."]
    #[inline(always)]
    pub fn du_tr2_sel(&self) -> DU_TR2_SEL_R {
        DU_TR2_SEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:25 - Data unit input data 'data0_in' source selection: '0': Constant '0'. '1': chip_data\\[7:0\\]. '2': io_data_in\\[7:0\\]. '3': DATA.DATA MMIO register field."]
    #[inline(always)]
    pub fn du_data0_sel(&self) -> DU_DATA0_SEL_R {
        DU_DATA0_SEL_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - Data unit input data 'data1_in' source selection. Encoding is the same as for DU_DATA0_SEL."]
    #[inline(always)]
    pub fn du_data1_sel(&self) -> DU_DATA1_SEL_R {
        DU_DATA1_SEL_R::new(((self.bits >> 28) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Data unit input signal 'tr0_in' source selection: '0': Constant '0'. '1': Constant '1'. '2': Data unit output. '10-3': LUT 7 - 0 outputs. Otherwise: Undefined."]
    #[inline(always)]
    pub fn du_tr0_sel(&mut self) -> DU_TR0_SEL_W {
        DU_TR0_SEL_W { w: self }
    }
    #[doc = "Bits 8:11 - Data unit input signal 'tr1_in' source selection. Encoding is the same as for DU_TR0_SEL."]
    #[inline(always)]
    pub fn du_tr1_sel(&mut self) -> DU_TR1_SEL_W {
        DU_TR1_SEL_W { w: self }
    }
    #[doc = "Bits 16:19 - Data unit input signal 'tr2_in' source selection. Encoding is the same as for DU_TR0_SEL."]
    #[inline(always)]
    pub fn du_tr2_sel(&mut self) -> DU_TR2_SEL_W {
        DU_TR2_SEL_W { w: self }
    }
    #[doc = "Bits 24:25 - Data unit input data 'data0_in' source selection: '0': Constant '0'. '1': chip_data\\[7:0\\]. '2': io_data_in\\[7:0\\]. '3': DATA.DATA MMIO register field."]
    #[inline(always)]
    pub fn du_data0_sel(&mut self) -> DU_DATA0_SEL_W {
        DU_DATA0_SEL_W { w: self }
    }
    #[doc = "Bits 28:29 - Data unit input data 'data1_in' source selection. Encoding is the same as for DU_DATA0_SEL."]
    #[inline(always)]
    pub fn du_data1_sel(&mut self) -> DU_DATA1_SEL_W {
        DU_DATA1_SEL_W { w: self }
    }
}
