#[doc = "Register `RED_CTL45` reader"]
pub struct R(crate::R<RED_CTL45_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RED_CTL45_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RED_CTL45_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RED_CTL45_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RED_CTL45` writer"]
pub struct W(crate::W<RED_CTL45_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RED_CTL45_SPEC>;
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
impl From<crate::W<RED_CTL45_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RED_CTL45_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DNU_45_1` reader - Not Used"]
pub type DNU_45_1_R = crate::BitReader<bool>;
#[doc = "Field `DNU_45_1` writer - Not Used"]
pub type DNU_45_1_W<'a, const O: u8> = crate::BitWriter<'a, u32, RED_CTL45_SPEC, bool, O>;
#[doc = "Field `REG_ACT_HV` reader - Forces the VBST regulator in active mode all the time"]
pub type REG_ACT_HV_R = crate::BitReader<bool>;
#[doc = "Field `REG_ACT_HV` writer - Forces the VBST regulator in active mode all the time"]
pub type REG_ACT_HV_W<'a, const O: u8> = crate::BitWriter<'a, u32, RED_CTL45_SPEC, bool, O>;
#[doc = "Field `DNU_45_3` reader - Not Used"]
pub type DNU_45_3_R = crate::BitReader<bool>;
#[doc = "Field `DNU_45_3` writer - Not Used"]
pub type DNU_45_3_W<'a, const O: u8> = crate::BitWriter<'a, u32, RED_CTL45_SPEC, bool, O>;
#[doc = "Field `FDIV_TRIM_HV_0` reader - '2b00' F = 1MHz see fdiv_trim_hv<1> value as well '2b01' F = 0.5MHz '2b10' F = 2MHz '2b11' F = 4Mhz"]
pub type FDIV_TRIM_HV_0_R = crate::BitReader<bool>;
#[doc = "Field `FDIV_TRIM_HV_0` writer - '2b00' F = 1MHz see fdiv_trim_hv<1> value as well '2b01' F = 0.5MHz '2b10' F = 2MHz '2b11' F = 4Mhz"]
pub type FDIV_TRIM_HV_0_W<'a, const O: u8> = crate::BitWriter<'a, u32, RED_CTL45_SPEC, bool, O>;
#[doc = "Field `DNU_45_5` reader - Not Used"]
pub type DNU_45_5_R = crate::BitReader<bool>;
#[doc = "Field `DNU_45_5` writer - Not Used"]
pub type DNU_45_5_W<'a, const O: u8> = crate::BitWriter<'a, u32, RED_CTL45_SPEC, bool, O>;
#[doc = "Field `FDIV_TRIM_HV_1` reader - '2b00' F = 1MHz see fdiv_trim_hv<0> value as well '2b01' F = 0.5MHz '2b10' F = 2MHz '2b11' F = 4Mhz"]
pub type FDIV_TRIM_HV_1_R = crate::BitReader<bool>;
#[doc = "Field `FDIV_TRIM_HV_1` writer - '2b00' F = 1MHz see fdiv_trim_hv<0> value as well '2b01' F = 0.5MHz '2b10' F = 2MHz '2b11' F = 4Mhz"]
pub type FDIV_TRIM_HV_1_W<'a, const O: u8> = crate::BitWriter<'a, u32, RED_CTL45_SPEC, bool, O>;
#[doc = "Field `DNU_45_6` reader - Not Used"]
pub type DNU_45_6_R = crate::BitReader<bool>;
#[doc = "Field `DNU_45_6` writer - Not Used"]
pub type DNU_45_6_W<'a, const O: u8> = crate::BitWriter<'a, u32, RED_CTL45_SPEC, bool, O>;
#[doc = "Field `VLIM_TRIM_HV_0` reader - '2b00' V2 = 650mV see vlim_trim_hv<1> value as well '2b01' V2 = 600mV '2b10' V2 = 750mV '2b11' V2 = 700mV"]
pub type VLIM_TRIM_HV_0_R = crate::BitReader<bool>;
#[doc = "Field `VLIM_TRIM_HV_0` writer - '2b00' V2 = 650mV see vlim_trim_hv<1> value as well '2b01' V2 = 600mV '2b10' V2 = 750mV '2b11' V2 = 700mV"]
pub type VLIM_TRIM_HV_0_W<'a, const O: u8> = crate::BitWriter<'a, u32, RED_CTL45_SPEC, bool, O>;
#[doc = "Field `DNU_45_8` reader - Not Used"]
pub type DNU_45_8_R = crate::BitReader<bool>;
#[doc = "Field `DNU_45_8` writer - Not Used"]
pub type DNU_45_8_W<'a, const O: u8> = crate::BitWriter<'a, u32, RED_CTL45_SPEC, bool, O>;
#[doc = "Field `DNU_45_23_16` reader - Not Used"]
pub type DNU_45_23_16_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DNU_45_23_16` writer - Not Used"]
pub type DNU_45_23_16_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RED_CTL45_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0 - Not Used"]
    #[inline(always)]
    pub fn dnu_45_1(&self) -> DNU_45_1_R {
        DNU_45_1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Forces the VBST regulator in active mode all the time"]
    #[inline(always)]
    pub fn reg_act_hv(&self) -> REG_ACT_HV_R {
        REG_ACT_HV_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Not Used"]
    #[inline(always)]
    pub fn dnu_45_3(&self) -> DNU_45_3_R {
        DNU_45_3_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - '2b00' F = 1MHz see fdiv_trim_hv<1> value as well '2b01' F = 0.5MHz '2b10' F = 2MHz '2b11' F = 4Mhz"]
    #[inline(always)]
    pub fn fdiv_trim_hv_0(&self) -> FDIV_TRIM_HV_0_R {
        FDIV_TRIM_HV_0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Not Used"]
    #[inline(always)]
    pub fn dnu_45_5(&self) -> DNU_45_5_R {
        DNU_45_5_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - '2b00' F = 1MHz see fdiv_trim_hv<0> value as well '2b01' F = 0.5MHz '2b10' F = 2MHz '2b11' F = 4Mhz"]
    #[inline(always)]
    pub fn fdiv_trim_hv_1(&self) -> FDIV_TRIM_HV_1_R {
        FDIV_TRIM_HV_1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Not Used"]
    #[inline(always)]
    pub fn dnu_45_6(&self) -> DNU_45_6_R {
        DNU_45_6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - '2b00' V2 = 650mV see vlim_trim_hv<1> value as well '2b01' V2 = 600mV '2b10' V2 = 750mV '2b11' V2 = 700mV"]
    #[inline(always)]
    pub fn vlim_trim_hv_0(&self) -> VLIM_TRIM_HV_0_R {
        VLIM_TRIM_HV_0_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Not Used"]
    #[inline(always)]
    pub fn dnu_45_8(&self) -> DNU_45_8_R {
        DNU_45_8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Not Used"]
    #[inline(always)]
    pub fn dnu_45_23_16(&self) -> DNU_45_23_16_R {
        DNU_45_23_16_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Not Used"]
    #[inline(always)]
    pub fn dnu_45_1(&mut self) -> DNU_45_1_W<0> {
        DNU_45_1_W::new(self)
    }
    #[doc = "Bit 1 - Forces the VBST regulator in active mode all the time"]
    #[inline(always)]
    pub fn reg_act_hv(&mut self) -> REG_ACT_HV_W<1> {
        REG_ACT_HV_W::new(self)
    }
    #[doc = "Bit 2 - Not Used"]
    #[inline(always)]
    pub fn dnu_45_3(&mut self) -> DNU_45_3_W<2> {
        DNU_45_3_W::new(self)
    }
    #[doc = "Bit 3 - '2b00' F = 1MHz see fdiv_trim_hv<1> value as well '2b01' F = 0.5MHz '2b10' F = 2MHz '2b11' F = 4Mhz"]
    #[inline(always)]
    pub fn fdiv_trim_hv_0(&mut self) -> FDIV_TRIM_HV_0_W<3> {
        FDIV_TRIM_HV_0_W::new(self)
    }
    #[doc = "Bit 4 - Not Used"]
    #[inline(always)]
    pub fn dnu_45_5(&mut self) -> DNU_45_5_W<4> {
        DNU_45_5_W::new(self)
    }
    #[doc = "Bit 5 - '2b00' F = 1MHz see fdiv_trim_hv<0> value as well '2b01' F = 0.5MHz '2b10' F = 2MHz '2b11' F = 4Mhz"]
    #[inline(always)]
    pub fn fdiv_trim_hv_1(&mut self) -> FDIV_TRIM_HV_1_W<5> {
        FDIV_TRIM_HV_1_W::new(self)
    }
    #[doc = "Bit 6 - Not Used"]
    #[inline(always)]
    pub fn dnu_45_6(&mut self) -> DNU_45_6_W<6> {
        DNU_45_6_W::new(self)
    }
    #[doc = "Bit 7 - '2b00' V2 = 650mV see vlim_trim_hv<1> value as well '2b01' V2 = 600mV '2b10' V2 = 750mV '2b11' V2 = 700mV"]
    #[inline(always)]
    pub fn vlim_trim_hv_0(&mut self) -> VLIM_TRIM_HV_0_W<7> {
        VLIM_TRIM_HV_0_W::new(self)
    }
    #[doc = "Bit 8 - Not Used"]
    #[inline(always)]
    pub fn dnu_45_8(&mut self) -> DNU_45_8_W<8> {
        DNU_45_8_W::new(self)
    }
    #[doc = "Bits 16:23 - Not Used"]
    #[inline(always)]
    pub fn dnu_45_23_16(&mut self) -> DNU_45_23_16_W<16> {
        DNU_45_23_16_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Redundancy Controll normal sectors 4,5\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [red_ctl45](index.html) module"]
pub struct RED_CTL45_SPEC;
impl crate::RegisterSpec for RED_CTL45_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [red_ctl45::R](R) reader structure"]
impl crate::Readable for RED_CTL45_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [red_ctl45::W](W) writer structure"]
impl crate::Writable for RED_CTL45_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RED_CTL45 to value 0"]
impl crate::Resettable for RED_CTL45_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
