#[doc = "Register `CONN_EXT_INTR` reader"]
pub struct R(crate::R<CONN_EXT_INTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONN_EXT_INTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONN_EXT_INTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONN_EXT_INTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONN_EXT_INTR` writer"]
pub struct W(crate::W<CONN_EXT_INTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONN_EXT_INTR_SPEC>;
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
impl From<crate::W<CONN_EXT_INTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONN_EXT_INTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATARATE_UPDATE` reader - If this bit is set it indicates that the data rate is updated If this bit is written with 1, it clears the interrupt status bit"]
pub type DATARATE_UPDATE_R = crate::BitReader<bool>;
#[doc = "Field `DATARATE_UPDATE` writer - If this bit is set it indicates that the data rate is updated If this bit is written with 1, it clears the interrupt status bit"]
pub type DATARATE_UPDATE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CONN_EXT_INTR_SPEC, bool, O>;
#[doc = "Field `EARLY_INTR` reader - For master this bit is set on start_ce For Slave this bit is set on slave_timer_adj"]
pub type EARLY_INTR_R = crate::BitReader<bool>;
#[doc = "Field `EARLY_INTR` writer - For master this bit is set on start_ce For Slave this bit is set on slave_timer_adj"]
pub type EARLY_INTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONN_EXT_INTR_SPEC, bool, O>;
#[doc = "Field `GEN_TIMER_INTR` reader - If this bit is set it indicates that the generic timer (PDU response timer reconfigured in MMMS mode) has expired If this bit is written with 1, it clears the interrupt status bit"]
pub type GEN_TIMER_INTR_R = crate::BitReader<bool>;
#[doc = "Field `GEN_TIMER_INTR` writer - If this bit is set it indicates that the generic timer (PDU response timer reconfigured in MMMS mode) has expired If this bit is written with 1, it clears the interrupt status bit"]
pub type GEN_TIMER_INTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONN_EXT_INTR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - If this bit is set it indicates that the data rate is updated If this bit is written with 1, it clears the interrupt status bit"]
    #[inline(always)]
    pub fn datarate_update(&self) -> DATARATE_UPDATE_R {
        DATARATE_UPDATE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - For master this bit is set on start_ce For Slave this bit is set on slave_timer_adj"]
    #[inline(always)]
    pub fn early_intr(&self) -> EARLY_INTR_R {
        EARLY_INTR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - If this bit is set it indicates that the generic timer (PDU response timer reconfigured in MMMS mode) has expired If this bit is written with 1, it clears the interrupt status bit"]
    #[inline(always)]
    pub fn gen_timer_intr(&self) -> GEN_TIMER_INTR_R {
        GEN_TIMER_INTR_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - If this bit is set it indicates that the data rate is updated If this bit is written with 1, it clears the interrupt status bit"]
    #[inline(always)]
    pub fn datarate_update(&mut self) -> DATARATE_UPDATE_W<0> {
        DATARATE_UPDATE_W::new(self)
    }
    #[doc = "Bit 1 - For master this bit is set on start_ce For Slave this bit is set on slave_timer_adj"]
    #[inline(always)]
    pub fn early_intr(&mut self) -> EARLY_INTR_W<1> {
        EARLY_INTR_W::new(self)
    }
    #[doc = "Bit 2 - If this bit is set it indicates that the generic timer (PDU response timer reconfigured in MMMS mode) has expired If this bit is written with 1, it clears the interrupt status bit"]
    #[inline(always)]
    pub fn gen_timer_intr(&mut self) -> GEN_TIMER_INTR_W<2> {
        GEN_TIMER_INTR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Connection extended interrupt status and Clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conn_ext_intr](index.html) module"]
pub struct CONN_EXT_INTR_SPEC;
impl crate::RegisterSpec for CONN_EXT_INTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [conn_ext_intr::R](R) reader structure"]
impl crate::Readable for CONN_EXT_INTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [conn_ext_intr::W](W) writer structure"]
impl crate::Writable for CONN_EXT_INTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONN_EXT_INTR to value 0"]
impl crate::Resettable for CONN_EXT_INTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
