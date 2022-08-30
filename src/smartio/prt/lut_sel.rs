#[doc = "Register `LUT_SEL[%s]` reader"]
pub struct R(crate::R<LUT_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LUT_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LUT_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LUT_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LUT_SEL[%s]` writer"]
pub struct W(crate::W<LUT_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LUT_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<LUT_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LUT_SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LUT_TR0_SEL` reader - LUT input signal 'tr0_in' source selection: '0': Data unit output. '1': LUT 1 output. '2': LUT 2 output. '3': LUT 3 output. '4': LUT 4 output. '5': LUT 5 output. '6': LUT 6 output. '7': LUT 7 output. '8': chip_data\\[0\\]
(for LUTs 0, 1, 2, 3); chip_data\\[4\\]
(for LUTs 4, 5, 6, 7). '9': chip_data\\[1\\]
(for LUTs 0, 1, 2, 3); chip_data\\[5\\]
(for LUTs 4, 5, 6, 7). '10': chip_data\\[2\\]
(for LUTs 0, 1, 2, 3); chip_data\\[6\\]
(for LUTs 4, 5, 6, 7). '11': chip_data\\[3\\]
(for LUTs 0, 1, 2, 3); chip_data\\[7\\]
(for LUTs 4, 5, 6, 7). '12': io_data_in\\[0\\]
(for LUTs 0, 1, 2, 3); io_data_in\\[4\\]
(for LUTs 4, 5, 6, 7). '13': io_data_in\\[1\\]
(for LUTs 0, 1, 2, 3); io_data_in\\[5\\]
(for LUTs 4, 5, 6, 7). '14': io_data_in\\[2\\]
(for LUTs 0, 1, 2, 3); io_data_in\\[6\\]
(for LUTs 4, 5, 6, 7). '15': io_data_in\\[3\\]
(for LUTs 0, 1, 2, 3); io_data_in\\[7\\]
(for LUTs 4, 5, 6, 7)."]
pub type LUT_TR0_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LUT_TR0_SEL` writer - LUT input signal 'tr0_in' source selection: '0': Data unit output. '1': LUT 1 output. '2': LUT 2 output. '3': LUT 3 output. '4': LUT 4 output. '5': LUT 5 output. '6': LUT 6 output. '7': LUT 7 output. '8': chip_data\\[0\\]
(for LUTs 0, 1, 2, 3); chip_data\\[4\\]
(for LUTs 4, 5, 6, 7). '9': chip_data\\[1\\]
(for LUTs 0, 1, 2, 3); chip_data\\[5\\]
(for LUTs 4, 5, 6, 7). '10': chip_data\\[2\\]
(for LUTs 0, 1, 2, 3); chip_data\\[6\\]
(for LUTs 4, 5, 6, 7). '11': chip_data\\[3\\]
(for LUTs 0, 1, 2, 3); chip_data\\[7\\]
(for LUTs 4, 5, 6, 7). '12': io_data_in\\[0\\]
(for LUTs 0, 1, 2, 3); io_data_in\\[4\\]
(for LUTs 4, 5, 6, 7). '13': io_data_in\\[1\\]
(for LUTs 0, 1, 2, 3); io_data_in\\[5\\]
(for LUTs 4, 5, 6, 7). '14': io_data_in\\[2\\]
(for LUTs 0, 1, 2, 3); io_data_in\\[6\\]
(for LUTs 4, 5, 6, 7). '15': io_data_in\\[3\\]
(for LUTs 0, 1, 2, 3); io_data_in\\[7\\]
(for LUTs 4, 5, 6, 7)."]
pub type LUT_TR0_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LUT_SEL_SPEC, u8, u8, 4, O>;
#[doc = "Field `LUT_TR1_SEL` reader - LUT input signal 'tr1_in' source selection: '0': LUT 0 output. '1': LUT 1 output. '2': LUT 2 output. '3': LUT 3 output. '4': LUT 4 output. '5': LUT 5 output. '6': LUT 6 output. '7': LUT 7 output. '8': chip_data\\[0\\]
(for LUTs 0, 1, 2, 3); chip_data\\[4\\]
(for LUTs 4, 5, 6, 7). '9': chip_data\\[1\\]
(for LUTs 0, 1, 2, 3); chip_data\\[5\\]
(for LUTs 4, 5, 6, 7). '10': chip_data\\[2\\]
(for LUTs 0, 1, 2, 3); chip_data\\[6\\]
(for LUTs 4, 5, 6, 7). '11': chip_data\\[3\\]
(for LUTs 0, 1, 2, 3); chip_data\\[7\\]
(for LUTs 4, 5, 6, 7). '12': io_data_in\\[0\\]
(for LUTs 0, 1, 2, 3); io_data_in\\[4\\]
(for LUTs 4, 5, 6, 7). '13': io_data_in\\[1\\]
(for LUTs 0, 1, 2, 3); io_data_in\\[5\\]
(for LUTs 4, 5, 6, 7). '14': io_data_in\\[2\\]
(for LUTs 0, 1, 2, 3); io_data_in\\[6\\]
(for LUTs 4, 5, 6, 7). '15': io_data_in\\[3\\]
(for LUTs 0, 1, 2, 3); io_data_in\\[7\\]
(for LUTs 4, 5, 6, 7)."]
pub type LUT_TR1_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LUT_TR1_SEL` writer - LUT input signal 'tr1_in' source selection: '0': LUT 0 output. '1': LUT 1 output. '2': LUT 2 output. '3': LUT 3 output. '4': LUT 4 output. '5': LUT 5 output. '6': LUT 6 output. '7': LUT 7 output. '8': chip_data\\[0\\]
(for LUTs 0, 1, 2, 3); chip_data\\[4\\]
(for LUTs 4, 5, 6, 7). '9': chip_data\\[1\\]
(for LUTs 0, 1, 2, 3); chip_data\\[5\\]
(for LUTs 4, 5, 6, 7). '10': chip_data\\[2\\]
(for LUTs 0, 1, 2, 3); chip_data\\[6\\]
(for LUTs 4, 5, 6, 7). '11': chip_data\\[3\\]
(for LUTs 0, 1, 2, 3); chip_data\\[7\\]
(for LUTs 4, 5, 6, 7). '12': io_data_in\\[0\\]
(for LUTs 0, 1, 2, 3); io_data_in\\[4\\]
(for LUTs 4, 5, 6, 7). '13': io_data_in\\[1\\]
(for LUTs 0, 1, 2, 3); io_data_in\\[5\\]
(for LUTs 4, 5, 6, 7). '14': io_data_in\\[2\\]
(for LUTs 0, 1, 2, 3); io_data_in\\[6\\]
(for LUTs 4, 5, 6, 7). '15': io_data_in\\[3\\]
(for LUTs 0, 1, 2, 3); io_data_in\\[7\\]
(for LUTs 4, 5, 6, 7)."]
pub type LUT_TR1_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LUT_SEL_SPEC, u8, u8, 4, O>;
#[doc = "Field `LUT_TR2_SEL` reader - LUT input signal 'tr2_in' source selection. Encoding is the same as for LUT_TR1_SEL."]
pub type LUT_TR2_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LUT_TR2_SEL` writer - LUT input signal 'tr2_in' source selection. Encoding is the same as for LUT_TR1_SEL."]
pub type LUT_TR2_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LUT_SEL_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - LUT input signal 'tr0_in' source selection: '0': Data unit output. '1': LUT 1 output. '2': LUT 2 output. '3': LUT 3 output. '4': LUT 4 output. '5': LUT 5 output. '6': LUT 6 output. '7': LUT 7 output. '8': chip_data\\[0\\]
(for LUTs 0, 1, 2, 3); chip_data\\[4\\]
(for LUTs 4, 5, 6, 7). '9': chip_data\\[1\\]
(for LUTs 0, 1, 2, 3); chip_data\\[5\\]
(for LUTs 4, 5, 6, 7). '10': chip_data\\[2\\]
(for LUTs 0, 1, 2, 3); chip_data\\[6\\]
(for LUTs 4, 5, 6, 7). '11': chip_data\\[3\\]
(for LUTs 0, 1, 2, 3); chip_data\\[7\\]
(for LUTs 4, 5, 6, 7). '12': io_data_in\\[0\\]
(for LUTs 0, 1, 2, 3); io_data_in\\[4\\]
(for LUTs 4, 5, 6, 7). '13': io_data_in\\[1\\]
(for LUTs 0, 1, 2, 3); io_data_in\\[5\\]
(for LUTs 4, 5, 6, 7). '14': io_data_in\\[2\\]
(for LUTs 0, 1, 2, 3); io_data_in\\[6\\]
(for LUTs 4, 5, 6, 7). '15': io_data_in\\[3\\]
(for LUTs 0, 1, 2, 3); io_data_in\\[7\\]
(for LUTs 4, 5, 6, 7)."]
    #[inline(always)]
    pub fn lut_tr0_sel(&self) -> LUT_TR0_SEL_R {
        LUT_TR0_SEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - LUT input signal 'tr1_in' source selection: '0': LUT 0 output. '1': LUT 1 output. '2': LUT 2 output. '3': LUT 3 output. '4': LUT 4 output. '5': LUT 5 output. '6': LUT 6 output. '7': LUT 7 output. '8': chip_data\\[0\\]
(for LUTs 0, 1, 2, 3); chip_data\\[4\\]
(for LUTs 4, 5, 6, 7). '9': chip_data\\[1\\]
(for LUTs 0, 1, 2, 3); chip_data\\[5\\]
(for LUTs 4, 5, 6, 7). '10': chip_data\\[2\\]
(for LUTs 0, 1, 2, 3); chip_data\\[6\\]
(for LUTs 4, 5, 6, 7). '11': chip_data\\[3\\]
(for LUTs 0, 1, 2, 3); chip_data\\[7\\]
(for LUTs 4, 5, 6, 7). '12': io_data_in\\[0\\]
(for LUTs 0, 1, 2, 3); io_data_in\\[4\\]
(for LUTs 4, 5, 6, 7). '13': io_data_in\\[1\\]
(for LUTs 0, 1, 2, 3); io_data_in\\[5\\]
(for LUTs 4, 5, 6, 7). '14': io_data_in\\[2\\]
(for LUTs 0, 1, 2, 3); io_data_in\\[6\\]
(for LUTs 4, 5, 6, 7). '15': io_data_in\\[3\\]
(for LUTs 0, 1, 2, 3); io_data_in\\[7\\]
(for LUTs 4, 5, 6, 7)."]
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
    #[doc = "Bits 0:3 - LUT input signal 'tr0_in' source selection: '0': Data unit output. '1': LUT 1 output. '2': LUT 2 output. '3': LUT 3 output. '4': LUT 4 output. '5': LUT 5 output. '6': LUT 6 output. '7': LUT 7 output. '8': chip_data\\[0\\]
(for LUTs 0, 1, 2, 3); chip_data\\[4\\]
(for LUTs 4, 5, 6, 7). '9': chip_data\\[1\\]
(for LUTs 0, 1, 2, 3); chip_data\\[5\\]
(for LUTs 4, 5, 6, 7). '10': chip_data\\[2\\]
(for LUTs 0, 1, 2, 3); chip_data\\[6\\]
(for LUTs 4, 5, 6, 7). '11': chip_data\\[3\\]
(for LUTs 0, 1, 2, 3); chip_data\\[7\\]
(for LUTs 4, 5, 6, 7). '12': io_data_in\\[0\\]
(for LUTs 0, 1, 2, 3); io_data_in\\[4\\]
(for LUTs 4, 5, 6, 7). '13': io_data_in\\[1\\]
(for LUTs 0, 1, 2, 3); io_data_in\\[5\\]
(for LUTs 4, 5, 6, 7). '14': io_data_in\\[2\\]
(for LUTs 0, 1, 2, 3); io_data_in\\[6\\]
(for LUTs 4, 5, 6, 7). '15': io_data_in\\[3\\]
(for LUTs 0, 1, 2, 3); io_data_in\\[7\\]
(for LUTs 4, 5, 6, 7)."]
    #[inline(always)]
    pub fn lut_tr0_sel(&mut self) -> LUT_TR0_SEL_W<0> {
        LUT_TR0_SEL_W::new(self)
    }
    #[doc = "Bits 8:11 - LUT input signal 'tr1_in' source selection: '0': LUT 0 output. '1': LUT 1 output. '2': LUT 2 output. '3': LUT 3 output. '4': LUT 4 output. '5': LUT 5 output. '6': LUT 6 output. '7': LUT 7 output. '8': chip_data\\[0\\]
(for LUTs 0, 1, 2, 3); chip_data\\[4\\]
(for LUTs 4, 5, 6, 7). '9': chip_data\\[1\\]
(for LUTs 0, 1, 2, 3); chip_data\\[5\\]
(for LUTs 4, 5, 6, 7). '10': chip_data\\[2\\]
(for LUTs 0, 1, 2, 3); chip_data\\[6\\]
(for LUTs 4, 5, 6, 7). '11': chip_data\\[3\\]
(for LUTs 0, 1, 2, 3); chip_data\\[7\\]
(for LUTs 4, 5, 6, 7). '12': io_data_in\\[0\\]
(for LUTs 0, 1, 2, 3); io_data_in\\[4\\]
(for LUTs 4, 5, 6, 7). '13': io_data_in\\[1\\]
(for LUTs 0, 1, 2, 3); io_data_in\\[5\\]
(for LUTs 4, 5, 6, 7). '14': io_data_in\\[2\\]
(for LUTs 0, 1, 2, 3); io_data_in\\[6\\]
(for LUTs 4, 5, 6, 7). '15': io_data_in\\[3\\]
(for LUTs 0, 1, 2, 3); io_data_in\\[7\\]
(for LUTs 4, 5, 6, 7)."]
    #[inline(always)]
    pub fn lut_tr1_sel(&mut self) -> LUT_TR1_SEL_W<8> {
        LUT_TR1_SEL_W::new(self)
    }
    #[doc = "Bits 16:19 - LUT input signal 'tr2_in' source selection. Encoding is the same as for LUT_TR1_SEL."]
    #[inline(always)]
    pub fn lut_tr2_sel(&mut self) -> LUT_TR2_SEL_W<16> {
        LUT_TR2_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LUT component input selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lut_sel](index.html) module"]
pub struct LUT_SEL_SPEC;
impl crate::RegisterSpec for LUT_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lut_sel::R](R) reader structure"]
impl crate::Readable for LUT_SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lut_sel::W](W) writer structure"]
impl crate::Writable for LUT_SEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LUT_SEL[%s]
to value 0"]
impl crate::Resettable for LUT_SEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
