#[doc = "Register `RED_CTL_SM01` reader"]
pub struct R(crate::R<RED_CTL_SM01_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RED_CTL_SM01_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RED_CTL_SM01_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RED_CTL_SM01_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RED_CTL_SM01` writer"]
pub struct W(crate::W<RED_CTL_SM01_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RED_CTL_SM01_SPEC>;
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
impl From<crate::W<RED_CTL_SM01_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RED_CTL_SM01_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RED_ADDR_SM0` reader - Bad Row Pair Address for Special Sector 0"]
pub type RED_ADDR_SM0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RED_ADDR_SM0` writer - Bad Row Pair Address for Special Sector 0"]
pub type RED_ADDR_SM0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RED_CTL_SM01_SPEC, u8, u8, 8, O>;
#[doc = "Field `RED_EN_SM0` reader - Redundancy Enable for Special Sector 0"]
pub type RED_EN_SM0_R = crate::BitReader<bool>;
#[doc = "Field `RED_EN_SM0` writer - Redundancy Enable for Special Sector 0"]
pub type RED_EN_SM0_W<'a, const O: u8> = crate::BitWriter<'a, u32, RED_CTL_SM01_SPEC, bool, O>;
#[doc = "Field `RED_ADDR_SM1` reader - Bad Row Pair Address for Special Sector 1"]
pub type RED_ADDR_SM1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RED_ADDR_SM1` writer - Bad Row Pair Address for Special Sector 1"]
pub type RED_ADDR_SM1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RED_CTL_SM01_SPEC, u8, u8, 8, O>;
#[doc = "Field `RED_EN_SM1` reader - Redundancy Enable for Special Sector 1"]
pub type RED_EN_SM1_R = crate::BitReader<bool>;
#[doc = "Field `RED_EN_SM1` writer - Redundancy Enable for Special Sector 1"]
pub type RED_EN_SM1_W<'a, const O: u8> = crate::BitWriter<'a, u32, RED_CTL_SM01_SPEC, bool, O>;
#[doc = "Field `TRKD` reader - Sense Amp Control tracking delay"]
pub type TRKD_R = crate::BitReader<bool>;
#[doc = "Field `TRKD` writer - Sense Amp Control tracking delay"]
pub type TRKD_W<'a, const O: u8> = crate::BitWriter<'a, u32, RED_CTL_SM01_SPEC, bool, O>;
#[doc = "Field `R_GRANT_EN` reader - '0': r_grant handshake disabled, r_grant always 1. '1': r_grand handshake enabled"]
pub type R_GRANT_EN_R = crate::BitReader<bool>;
#[doc = "Field `R_GRANT_EN` writer - '0': r_grant handshake disabled, r_grant always 1. '1': r_grand handshake enabled"]
pub type R_GRANT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RED_CTL_SM01_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7 - Bad Row Pair Address for Special Sector 0"]
    #[inline(always)]
    pub fn red_addr_sm0(&self) -> RED_ADDR_SM0_R {
        RED_ADDR_SM0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Redundancy Enable for Special Sector 0"]
    #[inline(always)]
    pub fn red_en_sm0(&self) -> RED_EN_SM0_R {
        RED_EN_SM0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Bad Row Pair Address for Special Sector 1"]
    #[inline(always)]
    pub fn red_addr_sm1(&self) -> RED_ADDR_SM1_R {
        RED_ADDR_SM1_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Redundancy Enable for Special Sector 1"]
    #[inline(always)]
    pub fn red_en_sm1(&self) -> RED_EN_SM1_R {
        RED_EN_SM1_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 30 - Sense Amp Control tracking delay"]
    #[inline(always)]
    pub fn trkd(&self) -> TRKD_R {
        TRKD_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - '0': r_grant handshake disabled, r_grant always 1. '1': r_grand handshake enabled"]
    #[inline(always)]
    pub fn r_grant_en(&self) -> R_GRANT_EN_R {
        R_GRANT_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Bad Row Pair Address for Special Sector 0"]
    #[inline(always)]
    pub fn red_addr_sm0(&mut self) -> RED_ADDR_SM0_W<0> {
        RED_ADDR_SM0_W::new(self)
    }
    #[doc = "Bit 8 - Redundancy Enable for Special Sector 0"]
    #[inline(always)]
    pub fn red_en_sm0(&mut self) -> RED_EN_SM0_W<8> {
        RED_EN_SM0_W::new(self)
    }
    #[doc = "Bits 16:23 - Bad Row Pair Address for Special Sector 1"]
    #[inline(always)]
    pub fn red_addr_sm1(&mut self) -> RED_ADDR_SM1_W<16> {
        RED_ADDR_SM1_W::new(self)
    }
    #[doc = "Bit 24 - Redundancy Enable for Special Sector 1"]
    #[inline(always)]
    pub fn red_en_sm1(&mut self) -> RED_EN_SM1_W<24> {
        RED_EN_SM1_W::new(self)
    }
    #[doc = "Bit 30 - Sense Amp Control tracking delay"]
    #[inline(always)]
    pub fn trkd(&mut self) -> TRKD_W<30> {
        TRKD_W::new(self)
    }
    #[doc = "Bit 31 - '0': r_grant handshake disabled, r_grant always 1. '1': r_grand handshake enabled"]
    #[inline(always)]
    pub fn r_grant_en(&mut self) -> R_GRANT_EN_W<31> {
        R_GRANT_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Redundancy Controll special sectors 0,1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [red_ctl_sm01](index.html) module"]
pub struct RED_CTL_SM01_SPEC;
impl crate::RegisterSpec for RED_CTL_SM01_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [red_ctl_sm01::R](R) reader structure"]
impl crate::Readable for RED_CTL_SM01_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [red_ctl_sm01::W](W) writer structure"]
impl crate::Writable for RED_CTL_SM01_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RED_CTL_SM01 to value 0"]
impl crate::Resettable for RED_CTL_SM01_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
