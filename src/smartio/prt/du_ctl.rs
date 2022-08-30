#[doc = "Register `DU_CTL` reader"]
pub struct R(crate::R<DU_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DU_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DU_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DU_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DU_CTL` writer"]
pub struct W(crate::W<DU_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DU_CTL_SPEC>;
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
impl From<crate::W<DU_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DU_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DU_SIZE` reader - Size/width of the data unit data operands (in bits) is DU_SIZE+1. E.g., if DU_SIZE is 7, the width is 8 bits."]
pub type DU_SIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DU_SIZE` writer - Size/width of the data unit data operands (in bits) is DU_SIZE+1. E.g., if DU_SIZE is 7, the width is 8 bits."]
pub type DU_SIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DU_CTL_SPEC, u8, u8, 3, O>;
#[doc = "Field `DU_OPC` reader - Data unit opcode specifies the data unit operation: '1': INCR '2': DECR '3': INCR_WRAP '4': DECR_WRAP '5': INCR_DECR '6': INCR_DECR_WRAP '7': ROR '8': SHR '9': AND_OR '10': SHR_MAJ3 '11': SHR_EQL. Otherwise: Undefined."]
pub type DU_OPC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DU_OPC` writer - Data unit opcode specifies the data unit operation: '1': INCR '2': DECR '3': INCR_WRAP '4': DECR_WRAP '5': INCR_DECR '6': INCR_DECR_WRAP '7': ROR '8': SHR '9': AND_OR '10': SHR_MAJ3 '11': SHR_EQL. Otherwise: Undefined."]
pub type DU_OPC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DU_CTL_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:2 - Size/width of the data unit data operands (in bits) is DU_SIZE+1. E.g., if DU_SIZE is 7, the width is 8 bits."]
    #[inline(always)]
    pub fn du_size(&self) -> DU_SIZE_R {
        DU_SIZE_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:11 - Data unit opcode specifies the data unit operation: '1': INCR '2': DECR '3': INCR_WRAP '4': DECR_WRAP '5': INCR_DECR '6': INCR_DECR_WRAP '7': ROR '8': SHR '9': AND_OR '10': SHR_MAJ3 '11': SHR_EQL. Otherwise: Undefined."]
    #[inline(always)]
    pub fn du_opc(&self) -> DU_OPC_R {
        DU_OPC_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Size/width of the data unit data operands (in bits) is DU_SIZE+1. E.g., if DU_SIZE is 7, the width is 8 bits."]
    #[inline(always)]
    pub fn du_size(&mut self) -> DU_SIZE_W<0> {
        DU_SIZE_W::new(self)
    }
    #[doc = "Bits 8:11 - Data unit opcode specifies the data unit operation: '1': INCR '2': DECR '3': INCR_WRAP '4': DECR_WRAP '5': INCR_DECR '6': INCR_DECR_WRAP '7': ROR '8': SHR '9': AND_OR '10': SHR_MAJ3 '11': SHR_EQL. Otherwise: Undefined."]
    #[inline(always)]
    pub fn du_opc(&mut self) -> DU_OPC_W<8> {
        DU_OPC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data unit component control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [du_ctl](index.html) module"]
pub struct DU_CTL_SPEC;
impl crate::RegisterSpec for DU_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [du_ctl::R](R) reader structure"]
impl crate::Readable for DU_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [du_ctl::W](W) writer structure"]
impl crate::Writable for DU_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DU_CTL to value 0"]
impl crate::Resettable for DU_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
