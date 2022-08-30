#[doc = "Register `ANA_CTL1` reader"]
pub struct R(crate::R<ANA_CTL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ANA_CTL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ANA_CTL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ANA_CTL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ANA_CTL1` writer"]
pub struct W(crate::W<ANA_CTL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ANA_CTL1_SPEC>;
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
impl From<crate::W<ANA_CTL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ANA_CTL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MDAC` reader - Trimming of the output margin Voltage as a function of Vpos and Vneg."]
pub type MDAC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MDAC` writer - Trimming of the output margin Voltage as a function of Vpos and Vneg."]
pub type MDAC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ANA_CTL1_SPEC, u8, u8, 8, O>;
#[doc = "Field `PDAC` reader - Trimming of positive pump output Voltage:"]
pub type PDAC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PDAC` writer - Trimming of positive pump output Voltage:"]
pub type PDAC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ANA_CTL1_SPEC, u8, u8, 4, O>;
#[doc = "Field `NDAC` reader - Trimming of negative pump output Voltage:"]
pub type NDAC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NDAC` writer - Trimming of negative pump output Voltage:"]
pub type NDAC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ANA_CTL1_SPEC, u8, u8, 4, O>;
#[doc = "Field `VPROT_OVERRIDE` reader - '0': vprot = BG.vprot. '1': vprot = vcc"]
pub type VPROT_OVERRIDE_R = crate::BitReader<bool>;
#[doc = "Field `VPROT_OVERRIDE` writer - '0': vprot = BG.vprot. '1': vprot = vcc"]
pub type VPROT_OVERRIDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ANA_CTL1_SPEC, bool, O>;
#[doc = "Field `R_GRANT_CTL` reader - r_grant control: '0': r_grant normal functionality '1': forces r_grant LO synchronized on clk_r"]
pub type R_GRANT_CTL_R = crate::BitReader<bool>;
#[doc = "Field `R_GRANT_CTL` writer - r_grant control: '0': r_grant normal functionality '1': forces r_grant LO synchronized on clk_r"]
pub type R_GRANT_CTL_W<'a, const O: u8> = crate::BitWriter<'a, u32, ANA_CTL1_SPEC, bool, O>;
#[doc = "Field `RST_SFT_HVPL` reader - '1': Page Latches Soft Reset"]
pub type RST_SFT_HVPL_R = crate::BitReader<bool>;
#[doc = "Field `RST_SFT_HVPL` writer - '1': Page Latches Soft Reset"]
pub type RST_SFT_HVPL_W<'a, const O: u8> = crate::BitWriter<'a, u32, ANA_CTL1_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7 - Trimming of the output margin Voltage as a function of Vpos and Vneg."]
    #[inline(always)]
    pub fn mdac(&self) -> MDAC_R {
        MDAC_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - Trimming of positive pump output Voltage:"]
    #[inline(always)]
    pub fn pdac(&self) -> PDAC_R {
        PDAC_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Trimming of negative pump output Voltage:"]
    #[inline(always)]
    pub fn ndac(&self) -> NDAC_R {
        NDAC_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - '0': vprot = BG.vprot. '1': vprot = vcc"]
    #[inline(always)]
    pub fn vprot_override(&self) -> VPROT_OVERRIDE_R {
        VPROT_OVERRIDE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - r_grant control: '0': r_grant normal functionality '1': forces r_grant LO synchronized on clk_r"]
    #[inline(always)]
    pub fn r_grant_ctl(&self) -> R_GRANT_CTL_R {
        R_GRANT_CTL_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - '1': Page Latches Soft Reset"]
    #[inline(always)]
    pub fn rst_sft_hvpl(&self) -> RST_SFT_HVPL_R {
        RST_SFT_HVPL_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Trimming of the output margin Voltage as a function of Vpos and Vneg."]
    #[inline(always)]
    pub fn mdac(&mut self) -> MDAC_W<0> {
        MDAC_W::new(self)
    }
    #[doc = "Bits 16:19 - Trimming of positive pump output Voltage:"]
    #[inline(always)]
    pub fn pdac(&mut self) -> PDAC_W<16> {
        PDAC_W::new(self)
    }
    #[doc = "Bits 24:27 - Trimming of negative pump output Voltage:"]
    #[inline(always)]
    pub fn ndac(&mut self) -> NDAC_W<24> {
        NDAC_W::new(self)
    }
    #[doc = "Bit 28 - '0': vprot = BG.vprot. '1': vprot = vcc"]
    #[inline(always)]
    pub fn vprot_override(&mut self) -> VPROT_OVERRIDE_W<28> {
        VPROT_OVERRIDE_W::new(self)
    }
    #[doc = "Bit 29 - r_grant control: '0': r_grant normal functionality '1': forces r_grant LO synchronized on clk_r"]
    #[inline(always)]
    pub fn r_grant_ctl(&mut self) -> R_GRANT_CTL_W<29> {
        R_GRANT_CTL_W::new(self)
    }
    #[doc = "Bit 30 - '1': Page Latches Soft Reset"]
    #[inline(always)]
    pub fn rst_sft_hvpl(&mut self) -> RST_SFT_HVPL_W<30> {
        RST_SFT_HVPL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Analog control 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ana_ctl1](index.html) module"]
pub struct ANA_CTL1_SPEC;
impl crate::RegisterSpec for ANA_CTL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ana_ctl1::R](R) reader structure"]
impl crate::Readable for ANA_CTL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ana_ctl1::W](W) writer structure"]
impl crate::Writable for ANA_CTL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ANA_CTL1 to value 0x0606_0000"]
impl crate::Resettable for ANA_CTL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0606_0000
    }
}
