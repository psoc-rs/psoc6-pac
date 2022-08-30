#[doc = "Register `CLK_TRIM_CCO_CTL` reader"]
pub struct R(crate::R<CLK_TRIM_CCO_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_TRIM_CCO_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_TRIM_CCO_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_TRIM_CCO_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_TRIM_CCO_CTL` writer"]
pub struct W(crate::W<CLK_TRIM_CCO_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_TRIM_CCO_CTL_SPEC>;
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
impl From<crate::W<CLK_TRIM_CCO_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_TRIM_CCO_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CCO_RCSTRIM` reader - CCO reference current source trim."]
pub type CCO_RCSTRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CCO_RCSTRIM` writer - CCO reference current source trim."]
pub type CCO_RCSTRIM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLK_TRIM_CCO_CTL_SPEC, u8, u8, 6, O>;
#[doc = "Field `CCO_STABLE_CNT` reader - Terminal count for the stabilization counter from CCO_ENABLE until stable."]
pub type CCO_STABLE_CNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CCO_STABLE_CNT` writer - Terminal count for the stabilization counter from CCO_ENABLE until stable."]
pub type CCO_STABLE_CNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLK_TRIM_CCO_CTL_SPEC, u8, u8, 6, O>;
#[doc = "Field `ENABLE_CNT` reader - Enables the automatic stabilization counter."]
pub type ENABLE_CNT_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE_CNT` writer - Enables the automatic stabilization counter."]
pub type ENABLE_CNT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_TRIM_CCO_CTL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:5 - CCO reference current source trim."]
    #[inline(always)]
    pub fn cco_rcstrim(&self) -> CCO_RCSTRIM_R {
        CCO_RCSTRIM_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - Terminal count for the stabilization counter from CCO_ENABLE until stable."]
    #[inline(always)]
    pub fn cco_stable_cnt(&self) -> CCO_STABLE_CNT_R {
        CCO_STABLE_CNT_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 31 - Enables the automatic stabilization counter."]
    #[inline(always)]
    pub fn enable_cnt(&self) -> ENABLE_CNT_R {
        ENABLE_CNT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - CCO reference current source trim."]
    #[inline(always)]
    pub fn cco_rcstrim(&mut self) -> CCO_RCSTRIM_W<0> {
        CCO_RCSTRIM_W::new(self)
    }
    #[doc = "Bits 24:29 - Terminal count for the stabilization counter from CCO_ENABLE until stable."]
    #[inline(always)]
    pub fn cco_stable_cnt(&mut self) -> CCO_STABLE_CNT_W<24> {
        CCO_STABLE_CNT_W::new(self)
    }
    #[doc = "Bit 31 - Enables the automatic stabilization counter."]
    #[inline(always)]
    pub fn enable_cnt(&mut self) -> ENABLE_CNT_W<31> {
        ENABLE_CNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CCO Trim Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_trim_cco_ctl](index.html) module"]
pub struct CLK_TRIM_CCO_CTL_SPEC;
impl crate::RegisterSpec for CLK_TRIM_CCO_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_trim_cco_ctl::R](R) reader structure"]
impl crate::Readable for CLK_TRIM_CCO_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_trim_cco_ctl::W](W) writer structure"]
impl crate::Writable for CLK_TRIM_CCO_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_TRIM_CCO_CTL to value 0xa700_0020"]
impl crate::Resettable for CLK_TRIM_CCO_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xa700_0020
    }
}
