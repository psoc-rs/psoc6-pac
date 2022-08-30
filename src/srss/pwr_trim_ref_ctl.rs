#[doc = "Register `PWR_TRIM_REF_CTL` reader"]
pub struct R(crate::R<PWR_TRIM_REF_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWR_TRIM_REF_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWR_TRIM_REF_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWR_TRIM_REF_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWR_TRIM_REF_CTL` writer"]
pub struct W(crate::W<PWR_TRIM_REF_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWR_TRIM_REF_CTL_SPEC>;
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
impl From<crate::W<PWR_TRIM_REF_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWR_TRIM_REF_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ACT_REF_TCTRIM` reader - Active-Reference temperature trim. This register is only reset by XRES/POR/BOD/HIBERNATE. 0 -> default setting at POR; not for trimming use others -> normal trim range"]
pub type ACT_REF_TCTRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ACT_REF_TCTRIM` writer - Active-Reference temperature trim. This register is only reset by XRES/POR/BOD/HIBERNATE. 0 -> default setting at POR; not for trimming use others -> normal trim range"]
pub type ACT_REF_TCTRIM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PWR_TRIM_REF_CTL_SPEC, u8, u8, 4, O>;
#[doc = "Field `ACT_REF_ITRIM` reader - Active-Reference current trim. This register is only reset by XRES/POR/BOD/HIBERNATE. 0 -> default setting at POR; not for trimming use others -> normal trim range"]
pub type ACT_REF_ITRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ACT_REF_ITRIM` writer - Active-Reference current trim. This register is only reset by XRES/POR/BOD/HIBERNATE. 0 -> default setting at POR; not for trimming use others -> normal trim range"]
pub type ACT_REF_ITRIM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PWR_TRIM_REF_CTL_SPEC, u8, u8, 4, O>;
#[doc = "Field `ACT_REF_ABSTRIM` reader - Active-Reference absolute voltage trim. This register is only reset by XRES/POR/BOD/HIBERNATE. 0 -> default setting at POR; not for trimming use others -> normal trim range"]
pub type ACT_REF_ABSTRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ACT_REF_ABSTRIM` writer - Active-Reference absolute voltage trim. This register is only reset by XRES/POR/BOD/HIBERNATE. 0 -> default setting at POR; not for trimming use others -> normal trim range"]
pub type ACT_REF_ABSTRIM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PWR_TRIM_REF_CTL_SPEC, u8, u8, 5, O>;
#[doc = "Field `ACT_REF_IBOOST` reader - Active-Reference current boost. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: normal operation others: risk mitigation"]
pub type ACT_REF_IBOOST_R = crate::BitReader<bool>;
#[doc = "Field `ACT_REF_IBOOST` writer - Active-Reference current boost. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: normal operation others: risk mitigation"]
pub type ACT_REF_IBOOST_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PWR_TRIM_REF_CTL_SPEC, bool, O>;
#[doc = "Field `DPSLP_REF_TCTRIM` reader - DeepSleep-Reference temperature trim. This register is only reset by XRES/POR/BOD/HIBERNATE. 0 -> default setting at POR; not for trimming use others -> normal trim range"]
pub type DPSLP_REF_TCTRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DPSLP_REF_TCTRIM` writer - DeepSleep-Reference temperature trim. This register is only reset by XRES/POR/BOD/HIBERNATE. 0 -> default setting at POR; not for trimming use others -> normal trim range"]
pub type DPSLP_REF_TCTRIM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PWR_TRIM_REF_CTL_SPEC, u8, u8, 4, O>;
#[doc = "Field `DPSLP_REF_ABSTRIM` reader - DeepSleep-Reference absolute voltage trim. This register is only reset by XRES/POR/BOD/HIBERNATE."]
pub type DPSLP_REF_ABSTRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DPSLP_REF_ABSTRIM` writer - DeepSleep-Reference absolute voltage trim. This register is only reset by XRES/POR/BOD/HIBERNATE."]
pub type DPSLP_REF_ABSTRIM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PWR_TRIM_REF_CTL_SPEC, u8, u8, 5, O>;
#[doc = "Field `DPSLP_REF_ITRIM` reader - DeepSleep current reference trim. This register is only reset by XRES/POR/BOD/HIBERNATE."]
pub type DPSLP_REF_ITRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DPSLP_REF_ITRIM` writer - DeepSleep current reference trim. This register is only reset by XRES/POR/BOD/HIBERNATE."]
pub type DPSLP_REF_ITRIM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PWR_TRIM_REF_CTL_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Active-Reference temperature trim. This register is only reset by XRES/POR/BOD/HIBERNATE. 0 -> default setting at POR; not for trimming use others -> normal trim range"]
    #[inline(always)]
    pub fn act_ref_tctrim(&self) -> ACT_REF_TCTRIM_R {
        ACT_REF_TCTRIM_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Active-Reference current trim. This register is only reset by XRES/POR/BOD/HIBERNATE. 0 -> default setting at POR; not for trimming use others -> normal trim range"]
    #[inline(always)]
    pub fn act_ref_itrim(&self) -> ACT_REF_ITRIM_R {
        ACT_REF_ITRIM_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:12 - Active-Reference absolute voltage trim. This register is only reset by XRES/POR/BOD/HIBERNATE. 0 -> default setting at POR; not for trimming use others -> normal trim range"]
    #[inline(always)]
    pub fn act_ref_abstrim(&self) -> ACT_REF_ABSTRIM_R {
        ACT_REF_ABSTRIM_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 14 - Active-Reference current boost. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: normal operation others: risk mitigation"]
    #[inline(always)]
    pub fn act_ref_iboost(&self) -> ACT_REF_IBOOST_R {
        ACT_REF_IBOOST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:19 - DeepSleep-Reference temperature trim. This register is only reset by XRES/POR/BOD/HIBERNATE. 0 -> default setting at POR; not for trimming use others -> normal trim range"]
    #[inline(always)]
    pub fn dpslp_ref_tctrim(&self) -> DPSLP_REF_TCTRIM_R {
        DPSLP_REF_TCTRIM_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:24 - DeepSleep-Reference absolute voltage trim. This register is only reset by XRES/POR/BOD/HIBERNATE."]
    #[inline(always)]
    pub fn dpslp_ref_abstrim(&self) -> DPSLP_REF_ABSTRIM_R {
        DPSLP_REF_ABSTRIM_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 28:31 - DeepSleep current reference trim. This register is only reset by XRES/POR/BOD/HIBERNATE."]
    #[inline(always)]
    pub fn dpslp_ref_itrim(&self) -> DPSLP_REF_ITRIM_R {
        DPSLP_REF_ITRIM_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Active-Reference temperature trim. This register is only reset by XRES/POR/BOD/HIBERNATE. 0 -> default setting at POR; not for trimming use others -> normal trim range"]
    #[inline(always)]
    pub fn act_ref_tctrim(&mut self) -> ACT_REF_TCTRIM_W<0> {
        ACT_REF_TCTRIM_W::new(self)
    }
    #[doc = "Bits 4:7 - Active-Reference current trim. This register is only reset by XRES/POR/BOD/HIBERNATE. 0 -> default setting at POR; not for trimming use others -> normal trim range"]
    #[inline(always)]
    pub fn act_ref_itrim(&mut self) -> ACT_REF_ITRIM_W<4> {
        ACT_REF_ITRIM_W::new(self)
    }
    #[doc = "Bits 8:12 - Active-Reference absolute voltage trim. This register is only reset by XRES/POR/BOD/HIBERNATE. 0 -> default setting at POR; not for trimming use others -> normal trim range"]
    #[inline(always)]
    pub fn act_ref_abstrim(&mut self) -> ACT_REF_ABSTRIM_W<8> {
        ACT_REF_ABSTRIM_W::new(self)
    }
    #[doc = "Bit 14 - Active-Reference current boost. This register is only reset by XRES/POR/BOD/HIBERNATE. 0: normal operation others: risk mitigation"]
    #[inline(always)]
    pub fn act_ref_iboost(&mut self) -> ACT_REF_IBOOST_W<14> {
        ACT_REF_IBOOST_W::new(self)
    }
    #[doc = "Bits 16:19 - DeepSleep-Reference temperature trim. This register is only reset by XRES/POR/BOD/HIBERNATE. 0 -> default setting at POR; not for trimming use others -> normal trim range"]
    #[inline(always)]
    pub fn dpslp_ref_tctrim(&mut self) -> DPSLP_REF_TCTRIM_W<16> {
        DPSLP_REF_TCTRIM_W::new(self)
    }
    #[doc = "Bits 20:24 - DeepSleep-Reference absolute voltage trim. This register is only reset by XRES/POR/BOD/HIBERNATE."]
    #[inline(always)]
    pub fn dpslp_ref_abstrim(&mut self) -> DPSLP_REF_ABSTRIM_W<20> {
        DPSLP_REF_ABSTRIM_W::new(self)
    }
    #[doc = "Bits 28:31 - DeepSleep current reference trim. This register is only reset by XRES/POR/BOD/HIBERNATE."]
    #[inline(always)]
    pub fn dpslp_ref_itrim(&mut self) -> DPSLP_REF_ITRIM_W<28> {
        DPSLP_REF_ITRIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reference Trim Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr_trim_ref_ctl](index.html) module"]
pub struct PWR_TRIM_REF_CTL_SPEC;
impl crate::RegisterSpec for PWR_TRIM_REF_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwr_trim_ref_ctl::R](R) reader structure"]
impl crate::Readable for PWR_TRIM_REF_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwr_trim_ref_ctl::W](W) writer structure"]
impl crate::Writable for PWR_TRIM_REF_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWR_TRIM_REF_CTL to value 0x70f0_0000"]
impl crate::Resettable for PWR_TRIM_REF_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x70f0_0000
    }
}
