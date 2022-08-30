#[doc = "Register `WAIT_CTL` reader"]
pub struct R(crate::R<WAIT_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WAIT_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WAIT_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WAIT_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WAIT_CTL` writer"]
pub struct W(crate::W<WAIT_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WAIT_CTL_SPEC>;
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
impl From<crate::W<WAIT_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WAIT_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WAIT_FM_MEM_RD` reader - Number of C interface wait cycles (on 'clk_c') for a read from the memory"]
pub type WAIT_FM_MEM_RD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WAIT_FM_MEM_RD` writer - Number of C interface wait cycles (on 'clk_c') for a read from the memory"]
pub type WAIT_FM_MEM_RD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, WAIT_CTL_SPEC, u8, u8, 4, O>;
#[doc = "Field `WAIT_FM_HV_RD` reader - Number of C interface wait cycles (on 'clk_c') for a read from the high Voltage page latches. Common for reading HV Page Latches and the DATA_COMP_RESULT bit"]
pub type WAIT_FM_HV_RD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WAIT_FM_HV_RD` writer - Number of C interface wait cycles (on 'clk_c') for a read from the high Voltage page latches. Common for reading HV Page Latches and the DATA_COMP_RESULT bit"]
pub type WAIT_FM_HV_RD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, WAIT_CTL_SPEC, u8, u8, 4, O>;
#[doc = "Field `WAIT_FM_HV_WR` reader - Number of C interface wait cycles (on 'clk_c') for a write to the high Voltage page latches."]
pub type WAIT_FM_HV_WR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WAIT_FM_HV_WR` writer - Number of C interface wait cycles (on 'clk_c') for a write to the high Voltage page latches."]
pub type WAIT_FM_HV_WR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, WAIT_CTL_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:3 - Number of C interface wait cycles (on 'clk_c') for a read from the memory"]
    #[inline(always)]
    pub fn wait_fm_mem_rd(&self) -> WAIT_FM_MEM_RD_R {
        WAIT_FM_MEM_RD_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Number of C interface wait cycles (on 'clk_c') for a read from the high Voltage page latches. Common for reading HV Page Latches and the DATA_COMP_RESULT bit"]
    #[inline(always)]
    pub fn wait_fm_hv_rd(&self) -> WAIT_FM_HV_RD_R {
        WAIT_FM_HV_RD_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:18 - Number of C interface wait cycles (on 'clk_c') for a write to the high Voltage page latches."]
    #[inline(always)]
    pub fn wait_fm_hv_wr(&self) -> WAIT_FM_HV_WR_R {
        WAIT_FM_HV_WR_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Number of C interface wait cycles (on 'clk_c') for a read from the memory"]
    #[inline(always)]
    pub fn wait_fm_mem_rd(&mut self) -> WAIT_FM_MEM_RD_W<0> {
        WAIT_FM_MEM_RD_W::new(self)
    }
    #[doc = "Bits 8:11 - Number of C interface wait cycles (on 'clk_c') for a read from the high Voltage page latches. Common for reading HV Page Latches and the DATA_COMP_RESULT bit"]
    #[inline(always)]
    pub fn wait_fm_hv_rd(&mut self) -> WAIT_FM_HV_RD_W<8> {
        WAIT_FM_HV_RD_W::new(self)
    }
    #[doc = "Bits 16:18 - Number of C interface wait cycles (on 'clk_c') for a write to the high Voltage page latches."]
    #[inline(always)]
    pub fn wait_fm_hv_wr(&mut self) -> WAIT_FM_HV_WR_W<16> {
        WAIT_FM_HV_WR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Wiat State control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wait_ctl](index.html) module"]
pub struct WAIT_CTL_SPEC;
impl crate::RegisterSpec for WAIT_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wait_ctl::R](R) reader structure"]
impl crate::Readable for WAIT_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wait_ctl::W](W) writer structure"]
impl crate::Writable for WAIT_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WAIT_CTL to value 0x0003_0b09"]
impl crate::Resettable for WAIT_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0003_0b09
    }
}
