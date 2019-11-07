#[doc = "Reader of register LUT_SEL[%s]"]
pub type R = crate::R<u32, super::LUT_SEL>;
#[doc = "Writer for register LUT_SEL[%s]"]
pub type W = crate::W<u32, super::LUT_SEL>;
#[doc = "Register LUT_SEL[%s] `reset()`'s with value 0"]
impl crate::ResetValue for super::LUT_SEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LUT_TR0_SEL`"]
pub type LUT_TR0_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LUT_TR0_SEL`"]
pub struct LUT_TR0_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> LUT_TR0_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `LUT_TR1_SEL`"]
pub type LUT_TR1_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LUT_TR1_SEL`"]
pub struct LUT_TR1_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> LUT_TR1_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `LUT_TR2_SEL`"]
pub type LUT_TR2_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LUT_TR2_SEL`"]
pub struct LUT_TR2_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> LUT_TR2_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - LUT input signal 'tr0_in' source selection: '0': Data unit output. '1': LUT 1 output. '2': LUT 2 output. '3': LUT 3 output. '4': LUT 4 output. '5': LUT 5 output. '6': LUT 6 output. '7': LUT 7 output. '8': chip_data\\[0\\] (for LUTs 0, 1, 2, 3); chip_data\\[4\\] (for LUTs 4, 5, 6, 7). '9': chip_data\\[1\\] (for LUTs 0, 1, 2, 3); chip_data\\[5\\] (for LUTs 4, 5, 6, 7). '10': chip_data\\[2\\] (for LUTs 0, 1, 2, 3); chip_data\\[6\\] (for LUTs 4, 5, 6, 7). '11': chip_data\\[3\\] (for LUTs 0, 1, 2, 3); chip_data\\[7\\] (for LUTs 4, 5, 6, 7). '12': io_data_in\\[0\\] (for LUTs 0, 1, 2, 3); io_data_in\\[4\\] (for LUTs 4, 5, 6, 7). '13': io_data_in\\[1\\] (for LUTs 0, 1, 2, 3); io_data_in\\[5\\] (for LUTs 4, 5, 6, 7). '14': io_data_in\\[2\\] (for LUTs 0, 1, 2, 3); io_data_in\\[6\\] (for LUTs 4, 5, 6, 7). '15': io_data_in\\[3\\] (for LUTs 0, 1, 2, 3); io_data_in\\[7\\] (for LUTs 4, 5, 6, 7)."]
    #[inline(always)]
    pub fn lut_tr0_sel(&self) -> LUT_TR0_SEL_R {
        LUT_TR0_SEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - LUT input signal 'tr1_in' source selection: '0': LUT 0 output. '1': LUT 1 output. '2': LUT 2 output. '3': LUT 3 output. '4': LUT 4 output. '5': LUT 5 output. '6': LUT 6 output. '7': LUT 7 output. '8': chip_data\\[0\\] (for LUTs 0, 1, 2, 3); chip_data\\[4\\] (for LUTs 4, 5, 6, 7). '9': chip_data\\[1\\] (for LUTs 0, 1, 2, 3); chip_data\\[5\\] (for LUTs 4, 5, 6, 7). '10': chip_data\\[2\\] (for LUTs 0, 1, 2, 3); chip_data\\[6\\] (for LUTs 4, 5, 6, 7). '11': chip_data\\[3\\] (for LUTs 0, 1, 2, 3); chip_data\\[7\\] (for LUTs 4, 5, 6, 7). '12': io_data_in\\[0\\] (for LUTs 0, 1, 2, 3); io_data_in\\[4\\] (for LUTs 4, 5, 6, 7). '13': io_data_in\\[1\\] (for LUTs 0, 1, 2, 3); io_data_in\\[5\\] (for LUTs 4, 5, 6, 7). '14': io_data_in\\[2\\] (for LUTs 0, 1, 2, 3); io_data_in\\[6\\] (for LUTs 4, 5, 6, 7). '15': io_data_in\\[3\\] (for LUTs 0, 1, 2, 3); io_data_in\\[7\\] (for LUTs 4, 5, 6, 7)."]
    #[inline(always)]
    pub fn lut_tr1_sel(&self) -> LUT_TR1_SEL_R {
        LUT_TR1_SEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - LUT input signal 'tr2_in' source selection. Encoding is the same as for LUT_TR1_SEL."]
    #[inline(always)]
    pub fn lut_tr2_sel(&self) -> LUT_TR2_SEL_R {
        LUT_TR2_SEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - LUT input signal 'tr0_in' source selection: '0': Data unit output. '1': LUT 1 output. '2': LUT 2 output. '3': LUT 3 output. '4': LUT 4 output. '5': LUT 5 output. '6': LUT 6 output. '7': LUT 7 output. '8': chip_data\\[0\\] (for LUTs 0, 1, 2, 3); chip_data\\[4\\] (for LUTs 4, 5, 6, 7). '9': chip_data\\[1\\] (for LUTs 0, 1, 2, 3); chip_data\\[5\\] (for LUTs 4, 5, 6, 7). '10': chip_data\\[2\\] (for LUTs 0, 1, 2, 3); chip_data\\[6\\] (for LUTs 4, 5, 6, 7). '11': chip_data\\[3\\] (for LUTs 0, 1, 2, 3); chip_data\\[7\\] (for LUTs 4, 5, 6, 7). '12': io_data_in\\[0\\] (for LUTs 0, 1, 2, 3); io_data_in\\[4\\] (for LUTs 4, 5, 6, 7). '13': io_data_in\\[1\\] (for LUTs 0, 1, 2, 3); io_data_in\\[5\\] (for LUTs 4, 5, 6, 7). '14': io_data_in\\[2\\] (for LUTs 0, 1, 2, 3); io_data_in\\[6\\] (for LUTs 4, 5, 6, 7). '15': io_data_in\\[3\\] (for LUTs 0, 1, 2, 3); io_data_in\\[7\\] (for LUTs 4, 5, 6, 7)."]
    #[inline(always)]
    pub fn lut_tr0_sel(&mut self) -> LUT_TR0_SEL_W {
        LUT_TR0_SEL_W { w: self }
    }
    #[doc = "Bits 8:11 - LUT input signal 'tr1_in' source selection: '0': LUT 0 output. '1': LUT 1 output. '2': LUT 2 output. '3': LUT 3 output. '4': LUT 4 output. '5': LUT 5 output. '6': LUT 6 output. '7': LUT 7 output. '8': chip_data\\[0\\] (for LUTs 0, 1, 2, 3); chip_data\\[4\\] (for LUTs 4, 5, 6, 7). '9': chip_data\\[1\\] (for LUTs 0, 1, 2, 3); chip_data\\[5\\] (for LUTs 4, 5, 6, 7). '10': chip_data\\[2\\] (for LUTs 0, 1, 2, 3); chip_data\\[6\\] (for LUTs 4, 5, 6, 7). '11': chip_data\\[3\\] (for LUTs 0, 1, 2, 3); chip_data\\[7\\] (for LUTs 4, 5, 6, 7). '12': io_data_in\\[0\\] (for LUTs 0, 1, 2, 3); io_data_in\\[4\\] (for LUTs 4, 5, 6, 7). '13': io_data_in\\[1\\] (for LUTs 0, 1, 2, 3); io_data_in\\[5\\] (for LUTs 4, 5, 6, 7). '14': io_data_in\\[2\\] (for LUTs 0, 1, 2, 3); io_data_in\\[6\\] (for LUTs 4, 5, 6, 7). '15': io_data_in\\[3\\] (for LUTs 0, 1, 2, 3); io_data_in\\[7\\] (for LUTs 4, 5, 6, 7)."]
    #[inline(always)]
    pub fn lut_tr1_sel(&mut self) -> LUT_TR1_SEL_W {
        LUT_TR1_SEL_W { w: self }
    }
    #[doc = "Bits 16:19 - LUT input signal 'tr2_in' source selection. Encoding is the same as for LUT_TR1_SEL."]
    #[inline(always)]
    pub fn lut_tr2_sel(&mut self) -> LUT_TR2_SEL_W {
        LUT_TR2_SEL_W { w: self }
    }
}
