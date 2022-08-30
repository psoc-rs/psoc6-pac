#[doc = "Register `LF_CLK_CTRL` reader"]
pub struct R(crate::R<LF_CLK_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LF_CLK_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LF_CLK_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LF_CLK_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LF_CLK_CTRL` writer"]
pub struct W(crate::W<LF_CLK_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LF_CLK_CTRL_SPEC>;
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
impl From<crate::W<LF_CLK_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LF_CLK_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DISABLE_LF_CLK` reader - When set to 1, gates the LF clock input to the Link Layer. Ths is done for extended DSM mode where the DSM state machine needs to be forzen to prevent a default auto exit."]
pub type DISABLE_LF_CLK_R = crate::BitReader<bool>;
#[doc = "Field `DISABLE_LF_CLK` writer - When set to 1, gates the LF clock input to the Link Layer. Ths is done for extended DSM mode where the DSM state machine needs to be forzen to prevent a default auto exit."]
pub type DISABLE_LF_CLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, LF_CLK_CTRL_SPEC, bool, O>;
#[doc = "Field `ENABLE_ENC_CLK` reader - This bit is used to enable the clock to the encryption engine 0 - Disable the clock to ENC engine 1 - Enable the clock to ENC engine"]
pub type ENABLE_ENC_CLK_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE_ENC_CLK` writer - This bit is used to enable the clock to the encryption engine 0 - Disable the clock to ENC engine 1 - Enable the clock to ENC engine"]
pub type ENABLE_ENC_CLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, LF_CLK_CTRL_SPEC, bool, O>;
#[doc = "Field `M0S8BLESS_REV_ID` reader - Indicates the m0s8bless IP revision."]
pub type M0S8BLESS_REV_ID_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - When set to 1, gates the LF clock input to the Link Layer. Ths is done for extended DSM mode where the DSM state machine needs to be forzen to prevent a default auto exit."]
    #[inline(always)]
    pub fn disable_lf_clk(&self) -> DISABLE_LF_CLK_R {
        DISABLE_LF_CLK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit is used to enable the clock to the encryption engine 0 - Disable the clock to ENC engine 1 - Enable the clock to ENC engine"]
    #[inline(always)]
    pub fn enable_enc_clk(&self) -> ENABLE_ENC_CLK_R {
        ENABLE_ENC_CLK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 29:31 - Indicates the m0s8bless IP revision."]
    #[inline(always)]
    pub fn m0s8bless_rev_id(&self) -> M0S8BLESS_REV_ID_R {
        M0S8BLESS_REV_ID_R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - When set to 1, gates the LF clock input to the Link Layer. Ths is done for extended DSM mode where the DSM state machine needs to be forzen to prevent a default auto exit."]
    #[inline(always)]
    pub fn disable_lf_clk(&mut self) -> DISABLE_LF_CLK_W<0> {
        DISABLE_LF_CLK_W::new(self)
    }
    #[doc = "Bit 1 - This bit is used to enable the clock to the encryption engine 0 - Disable the clock to ENC engine 1 - Enable the clock to ENC engine"]
    #[inline(always)]
    pub fn enable_enc_clk(&mut self) -> ENABLE_ENC_CLK_W<1> {
        ENABLE_ENC_CLK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BLESS LF clock control and BLESS revision ID indicator\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lf_clk_ctrl](index.html) module"]
pub struct LF_CLK_CTRL_SPEC;
impl crate::RegisterSpec for LF_CLK_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lf_clk_ctrl::R](R) reader structure"]
impl crate::Readable for LF_CLK_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lf_clk_ctrl::W](W) writer structure"]
impl crate::Writable for LF_CLK_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LF_CLK_CTRL to value 0x4000_0000"]
impl crate::Resettable for LF_CLK_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x4000_0000
    }
}
