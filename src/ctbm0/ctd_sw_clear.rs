#[doc = "Register `CTD_SW_CLEAR` reader"]
pub struct R(crate::R<CTD_SW_CLEAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTD_SW_CLEAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTD_SW_CLEAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTD_SW_CLEAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTD_SW_CLEAR` writer"]
pub struct W(crate::W<CTD_SW_CLEAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTD_SW_CLEAR_SPEC>;
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
impl From<crate::W<CTD_SW_CLEAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTD_SW_CLEAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTDD_CRD` reader - see corresponding bit in CTD_SW"]
pub type CTDD_CRD_R = crate::BitReader<bool>;
#[doc = "Field `CTDD_CRD` writer - see corresponding bit in CTD_SW"]
pub type CTDD_CRD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTD_SW_CLEAR_SPEC, bool, O>;
#[doc = "Field `CTDS_CRS` reader - see corresponding bit in CTD_SW"]
pub type CTDS_CRS_R = crate::BitReader<bool>;
#[doc = "Field `CTDS_CRS` writer - see corresponding bit in CTD_SW"]
pub type CTDS_CRS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTD_SW_CLEAR_SPEC, bool, O>;
#[doc = "Field `CTDS_COR` reader - see corresponding bit in CTD_SW"]
pub type CTDS_COR_R = crate::BitReader<bool>;
#[doc = "Field `CTDS_COR` writer - see corresponding bit in CTD_SW"]
pub type CTDS_COR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTD_SW_CLEAR_SPEC, bool, O>;
#[doc = "Field `CTDO_C6H` reader - see corresponding bit in CTD_SW"]
pub type CTDO_C6H_R = crate::BitReader<bool>;
#[doc = "Field `CTDO_C6H` writer - see corresponding bit in CTD_SW"]
pub type CTDO_C6H_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTD_SW_CLEAR_SPEC, bool, O>;
#[doc = "Field `CTDO_COS` reader - see corresponding bit in CTD_SW"]
pub type CTDO_COS_R = crate::BitReader<bool>;
#[doc = "Field `CTDO_COS` writer - see corresponding bit in CTD_SW"]
pub type CTDO_COS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTD_SW_CLEAR_SPEC, bool, O>;
#[doc = "Field `CTDH_COB` reader - see corresponding bit in CTD_SW"]
pub type CTDH_COB_R = crate::BitReader<bool>;
#[doc = "Field `CTDH_COB` writer - see corresponding bit in CTD_SW"]
pub type CTDH_COB_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTD_SW_CLEAR_SPEC, bool, O>;
#[doc = "Field `CTDH_CHD` reader - see corresponding bit in CTD_SW"]
pub type CTDH_CHD_R = crate::BitReader<bool>;
#[doc = "Field `CTDH_CHD` writer - see corresponding bit in CTD_SW"]
pub type CTDH_CHD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTD_SW_CLEAR_SPEC, bool, O>;
#[doc = "Field `CTDH_CA0` reader - see corresponding bit in CTD_SW"]
pub type CTDH_CA0_R = crate::BitReader<bool>;
#[doc = "Field `CTDH_CA0` writer - see corresponding bit in CTD_SW"]
pub type CTDH_CA0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTD_SW_CLEAR_SPEC, bool, O>;
#[doc = "Field `CTDH_CIS` reader - see corresponding bit in CTD_SW"]
pub type CTDH_CIS_R = crate::BitReader<bool>;
#[doc = "Field `CTDH_CIS` writer - see corresponding bit in CTD_SW"]
pub type CTDH_CIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTD_SW_CLEAR_SPEC, bool, O>;
#[doc = "Field `CTDH_ILR` reader - see corresponding bit in CTD_SW"]
pub type CTDH_ILR_R = crate::BitReader<bool>;
#[doc = "Field `CTDH_ILR` writer - see corresponding bit in CTD_SW"]
pub type CTDH_ILR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTD_SW_CLEAR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - see corresponding bit in CTD_SW"]
    #[inline(always)]
    pub fn ctdd_crd(&self) -> CTDD_CRD_R {
        CTDD_CRD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - see corresponding bit in CTD_SW"]
    #[inline(always)]
    pub fn ctds_crs(&self) -> CTDS_CRS_R {
        CTDS_CRS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - see corresponding bit in CTD_SW"]
    #[inline(always)]
    pub fn ctds_cor(&self) -> CTDS_COR_R {
        CTDS_COR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - see corresponding bit in CTD_SW"]
    #[inline(always)]
    pub fn ctdo_c6h(&self) -> CTDO_C6H_R {
        CTDO_C6H_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - see corresponding bit in CTD_SW"]
    #[inline(always)]
    pub fn ctdo_cos(&self) -> CTDO_COS_R {
        CTDO_COS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - see corresponding bit in CTD_SW"]
    #[inline(always)]
    pub fn ctdh_cob(&self) -> CTDH_COB_R {
        CTDH_COB_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - see corresponding bit in CTD_SW"]
    #[inline(always)]
    pub fn ctdh_chd(&self) -> CTDH_CHD_R {
        CTDH_CHD_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - see corresponding bit in CTD_SW"]
    #[inline(always)]
    pub fn ctdh_ca0(&self) -> CTDH_CA0_R {
        CTDH_CA0_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - see corresponding bit in CTD_SW"]
    #[inline(always)]
    pub fn ctdh_cis(&self) -> CTDH_CIS_R {
        CTDH_CIS_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - see corresponding bit in CTD_SW"]
    #[inline(always)]
    pub fn ctdh_ilr(&self) -> CTDH_ILR_R {
        CTDH_ILR_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - see corresponding bit in CTD_SW"]
    #[inline(always)]
    pub fn ctdd_crd(&mut self) -> CTDD_CRD_W<1> {
        CTDD_CRD_W::new(self)
    }
    #[doc = "Bit 4 - see corresponding bit in CTD_SW"]
    #[inline(always)]
    pub fn ctds_crs(&mut self) -> CTDS_CRS_W<4> {
        CTDS_CRS_W::new(self)
    }
    #[doc = "Bit 5 - see corresponding bit in CTD_SW"]
    #[inline(always)]
    pub fn ctds_cor(&mut self) -> CTDS_COR_W<5> {
        CTDS_COR_W::new(self)
    }
    #[doc = "Bit 8 - see corresponding bit in CTD_SW"]
    #[inline(always)]
    pub fn ctdo_c6h(&mut self) -> CTDO_C6H_W<8> {
        CTDO_C6H_W::new(self)
    }
    #[doc = "Bit 9 - see corresponding bit in CTD_SW"]
    #[inline(always)]
    pub fn ctdo_cos(&mut self) -> CTDO_COS_W<9> {
        CTDO_COS_W::new(self)
    }
    #[doc = "Bit 10 - see corresponding bit in CTD_SW"]
    #[inline(always)]
    pub fn ctdh_cob(&mut self) -> CTDH_COB_W<10> {
        CTDH_COB_W::new(self)
    }
    #[doc = "Bit 12 - see corresponding bit in CTD_SW"]
    #[inline(always)]
    pub fn ctdh_chd(&mut self) -> CTDH_CHD_W<12> {
        CTDH_CHD_W::new(self)
    }
    #[doc = "Bit 13 - see corresponding bit in CTD_SW"]
    #[inline(always)]
    pub fn ctdh_ca0(&mut self) -> CTDH_CA0_W<13> {
        CTDH_CA0_W::new(self)
    }
    #[doc = "Bit 14 - see corresponding bit in CTD_SW"]
    #[inline(always)]
    pub fn ctdh_cis(&mut self) -> CTDH_CIS_W<14> {
        CTDH_CIS_W::new(self)
    }
    #[doc = "Bit 15 - see corresponding bit in CTD_SW"]
    #[inline(always)]
    pub fn ctdh_ilr(&mut self) -> CTDH_ILR_W<15> {
        CTDH_ILR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CTDAC connection switch control clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctd_sw_clear](index.html) module"]
pub struct CTD_SW_CLEAR_SPEC;
impl crate::RegisterSpec for CTD_SW_CLEAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctd_sw_clear::R](R) reader structure"]
impl crate::Readable for CTD_SW_CLEAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctd_sw_clear::W](W) writer structure"]
impl crate::Writable for CTD_SW_CLEAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTD_SW_CLEAR to value 0"]
impl crate::Resettable for CTD_SW_CLEAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
