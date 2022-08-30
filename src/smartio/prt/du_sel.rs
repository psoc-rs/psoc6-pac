#[doc = "Register `DU_SEL` reader"]
pub struct R(crate::R<DU_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DU_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DU_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DU_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DU_SEL` writer"]
pub struct W(crate::W<DU_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DU_SEL_SPEC>;
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
impl From<crate::W<DU_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DU_SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DU_TR0_SEL` reader - Data unit input signal 'tr0_in' source selection: '0': Constant '0'. '1': Constant '1'. '2': Data unit output. '10-3': LUT 7 - 0 outputs. Otherwise: Undefined."]
pub type DU_TR0_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DU_TR0_SEL` writer - Data unit input signal 'tr0_in' source selection: '0': Constant '0'. '1': Constant '1'. '2': Data unit output. '10-3': LUT 7 - 0 outputs. Otherwise: Undefined."]
pub type DU_TR0_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DU_SEL_SPEC, u8, u8, 4, O>;
#[doc = "Field `DU_TR1_SEL` reader - Data unit input signal 'tr1_in' source selection. Encoding is the same as for DU_TR0_SEL."]
pub type DU_TR1_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DU_TR1_SEL` writer - Data unit input signal 'tr1_in' source selection. Encoding is the same as for DU_TR0_SEL."]
pub type DU_TR1_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DU_SEL_SPEC, u8, u8, 4, O>;
#[doc = "Field `DU_TR2_SEL` reader - Data unit input signal 'tr2_in' source selection. Encoding is the same as for DU_TR0_SEL."]
pub type DU_TR2_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DU_TR2_SEL` writer - Data unit input signal 'tr2_in' source selection. Encoding is the same as for DU_TR0_SEL."]
pub type DU_TR2_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DU_SEL_SPEC, u8, u8, 4, O>;
#[doc = "Field `DU_DATA0_SEL` reader - Data unit input data 'data0_in' source selection: '0': Constant '0'. '1': chip_data\\[7:0\\]. '2': io_data_in\\[7:0\\]. '3': DATA.DATA MMIO register field."]
pub type DU_DATA0_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DU_DATA0_SEL` writer - Data unit input data 'data0_in' source selection: '0': Constant '0'. '1': chip_data\\[7:0\\]. '2': io_data_in\\[7:0\\]. '3': DATA.DATA MMIO register field."]
pub type DU_DATA0_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DU_SEL_SPEC, u8, u8, 2, O>;
#[doc = "Field `DU_DATA1_SEL` reader - Data unit input data 'data1_in' source selection. Encoding is the same as for DU_DATA0_SEL."]
pub type DU_DATA1_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DU_DATA1_SEL` writer - Data unit input data 'data1_in' source selection. Encoding is the same as for DU_DATA0_SEL."]
pub type DU_DATA1_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DU_SEL_SPEC, u8, u8, 2, O>;
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
        DU_DATA0_SEL_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Data unit input data 'data1_in' source selection. Encoding is the same as for DU_DATA0_SEL."]
    #[inline(always)]
    pub fn du_data1_sel(&self) -> DU_DATA1_SEL_R {
        DU_DATA1_SEL_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Data unit input signal 'tr0_in' source selection: '0': Constant '0'. '1': Constant '1'. '2': Data unit output. '10-3': LUT 7 - 0 outputs. Otherwise: Undefined."]
    #[inline(always)]
    pub fn du_tr0_sel(&mut self) -> DU_TR0_SEL_W<0> {
        DU_TR0_SEL_W::new(self)
    }
    #[doc = "Bits 8:11 - Data unit input signal 'tr1_in' source selection. Encoding is the same as for DU_TR0_SEL."]
    #[inline(always)]
    pub fn du_tr1_sel(&mut self) -> DU_TR1_SEL_W<8> {
        DU_TR1_SEL_W::new(self)
    }
    #[doc = "Bits 16:19 - Data unit input signal 'tr2_in' source selection. Encoding is the same as for DU_TR0_SEL."]
    #[inline(always)]
    pub fn du_tr2_sel(&mut self) -> DU_TR2_SEL_W<16> {
        DU_TR2_SEL_W::new(self)
    }
    #[doc = "Bits 24:25 - Data unit input data 'data0_in' source selection: '0': Constant '0'. '1': chip_data\\[7:0\\]. '2': io_data_in\\[7:0\\]. '3': DATA.DATA MMIO register field."]
    #[inline(always)]
    pub fn du_data0_sel(&mut self) -> DU_DATA0_SEL_W<24> {
        DU_DATA0_SEL_W::new(self)
    }
    #[doc = "Bits 28:29 - Data unit input data 'data1_in' source selection. Encoding is the same as for DU_DATA0_SEL."]
    #[inline(always)]
    pub fn du_data1_sel(&mut self) -> DU_DATA1_SEL_W<28> {
        DU_DATA1_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data unit component input selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [du_sel](index.html) module"]
pub struct DU_SEL_SPEC;
impl crate::RegisterSpec for DU_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [du_sel::R](R) reader structure"]
impl crate::Readable for DU_SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [du_sel::W](W) writer structure"]
impl crate::Writable for DU_SEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DU_SEL to value 0"]
impl crate::Resettable for DU_SEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
