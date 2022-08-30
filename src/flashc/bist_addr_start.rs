#[doc = "Register `BIST_ADDR_START` reader"]
pub struct R(crate::R<BIST_ADDR_START_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BIST_ADDR_START_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BIST_ADDR_START_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BIST_ADDR_START_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BIST_ADDR_START` writer"]
pub struct W(crate::W<BIST_ADDR_START_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BIST_ADDR_START_SPEC>;
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
impl From<crate::W<BIST_ADDR_START_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BIST_ADDR_START_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COL_ADDR_START` reader - Column start address. Useful to apply BIST to a part of an Flash. The value of this field should be in a legal range (a value outside of the legal range has an undefined result, and may lock up the BIST state machine). This legal range is dependent on the number of columns of the SRAM the BIST is applied to (as specified by BIST_CTL.SRAMS_ENABLED). E.g. for a Flash with n columns, the legal range is \\[0, n-1\\]."]
pub type COL_ADDR_START_R = crate::FieldReader<u16, u16>;
#[doc = "Field `COL_ADDR_START` writer - Column start address. Useful to apply BIST to a part of an Flash. The value of this field should be in a legal range (a value outside of the legal range has an undefined result, and may lock up the BIST state machine). This legal range is dependent on the number of columns of the SRAM the BIST is applied to (as specified by BIST_CTL.SRAMS_ENABLED). E.g. for a Flash with n columns, the legal range is \\[0, n-1\\]."]
pub type COL_ADDR_START_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BIST_ADDR_START_SPEC, u16, u16, 16, O>;
#[doc = "Field `ROW_ADDR_START` reader - Row start address. Useful to apply BIST to a part of an Flash. The value of this field should be in a legal range (a value outside of the legal range has an undefined result, and may lock up the BIST state machine). This legal range is dependent on the number of rows of the SRAM the BIST is applied to (as specified by BIST_CTL.SRAMS_ENABLED). E.g. for a Flash with m columns, the legal range is \\[0, m-1\\]."]
pub type ROW_ADDR_START_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ROW_ADDR_START` writer - Row start address. Useful to apply BIST to a part of an Flash. The value of this field should be in a legal range (a value outside of the legal range has an undefined result, and may lock up the BIST state machine). This legal range is dependent on the number of rows of the SRAM the BIST is applied to (as specified by BIST_CTL.SRAMS_ENABLED). E.g. for a Flash with m columns, the legal range is \\[0, m-1\\]."]
pub type ROW_ADDR_START_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BIST_ADDR_START_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Column start address. Useful to apply BIST to a part of an Flash. The value of this field should be in a legal range (a value outside of the legal range has an undefined result, and may lock up the BIST state machine). This legal range is dependent on the number of columns of the SRAM the BIST is applied to (as specified by BIST_CTL.SRAMS_ENABLED). E.g. for a Flash with n columns, the legal range is \\[0, n-1\\]."]
    #[inline(always)]
    pub fn col_addr_start(&self) -> COL_ADDR_START_R {
        COL_ADDR_START_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Row start address. Useful to apply BIST to a part of an Flash. The value of this field should be in a legal range (a value outside of the legal range has an undefined result, and may lock up the BIST state machine). This legal range is dependent on the number of rows of the SRAM the BIST is applied to (as specified by BIST_CTL.SRAMS_ENABLED). E.g. for a Flash with m columns, the legal range is \\[0, m-1\\]."]
    #[inline(always)]
    pub fn row_addr_start(&self) -> ROW_ADDR_START_R {
        ROW_ADDR_START_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Column start address. Useful to apply BIST to a part of an Flash. The value of this field should be in a legal range (a value outside of the legal range has an undefined result, and may lock up the BIST state machine). This legal range is dependent on the number of columns of the SRAM the BIST is applied to (as specified by BIST_CTL.SRAMS_ENABLED). E.g. for a Flash with n columns, the legal range is \\[0, n-1\\]."]
    #[inline(always)]
    pub fn col_addr_start(&mut self) -> COL_ADDR_START_W<0> {
        COL_ADDR_START_W::new(self)
    }
    #[doc = "Bits 16:31 - Row start address. Useful to apply BIST to a part of an Flash. The value of this field should be in a legal range (a value outside of the legal range has an undefined result, and may lock up the BIST state machine). This legal range is dependent on the number of rows of the SRAM the BIST is applied to (as specified by BIST_CTL.SRAMS_ENABLED). E.g. for a Flash with m columns, the legal range is \\[0, m-1\\]."]
    #[inline(always)]
    pub fn row_addr_start(&mut self) -> ROW_ADDR_START_W<16> {
        ROW_ADDR_START_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BIST address start register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bist_addr_start](index.html) module"]
pub struct BIST_ADDR_START_SPEC;
impl crate::RegisterSpec for BIST_ADDR_START_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bist_addr_start::R](R) reader structure"]
impl crate::Readable for BIST_ADDR_START_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bist_addr_start::W](W) writer structure"]
impl crate::Writable for BIST_ADDR_START_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BIST_ADDR_START to value 0"]
impl crate::Resettable for BIST_ADDR_START_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
