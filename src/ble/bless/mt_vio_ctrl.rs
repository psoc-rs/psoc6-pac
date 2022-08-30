#[doc = "Register `MT_VIO_CTRL` reader"]
pub struct R(crate::R<MT_VIO_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MT_VIO_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MT_VIO_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MT_VIO_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MT_VIO_CTRL` writer"]
pub struct W(crate::W<MT_VIO_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MT_VIO_CTRL_SPEC>;
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
impl From<crate::W<MT_VIO_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MT_VIO_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SRSS_SWITCH_EN` reader - Enable to turn on HVLDO (One leg) 1'b0 - Switch is turned off 1'b1 - Switch is turned on"]
pub type SRSS_SWITCH_EN_R = crate::BitReader<bool>;
#[doc = "Field `SRSS_SWITCH_EN` writer - Enable to turn on HVLDO (One leg) 1'b0 - Switch is turned off 1'b1 - Switch is turned on"]
pub type SRSS_SWITCH_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MT_VIO_CTRL_SPEC, bool, O>;
#[doc = "Field `SRSS_SWITCH_EN_DLY` reader - Enable to turn on HVLDO (All legs). This must be enabled 64us after enabling SRSS_SWITCH_EN 1'b0 - Switch is turned off 1'b1 - Switch is turned on"]
pub type SRSS_SWITCH_EN_DLY_R = crate::BitReader<bool>;
#[doc = "Field `SRSS_SWITCH_EN_DLY` writer - Enable to turn on HVLDO (All legs). This must be enabled 64us after enabling SRSS_SWITCH_EN 1'b0 - Switch is turned off 1'b1 - Switch is turned on"]
pub type SRSS_SWITCH_EN_DLY_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MT_VIO_CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Enable to turn on HVLDO (One leg) 1'b0 - Switch is turned off 1'b1 - Switch is turned on"]
    #[inline(always)]
    pub fn srss_switch_en(&self) -> SRSS_SWITCH_EN_R {
        SRSS_SWITCH_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable to turn on HVLDO (All legs). This must be enabled 64us after enabling SRSS_SWITCH_EN 1'b0 - Switch is turned off 1'b1 - Switch is turned on"]
    #[inline(always)]
    pub fn srss_switch_en_dly(&self) -> SRSS_SWITCH_EN_DLY_R {
        SRSS_SWITCH_EN_DLY_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable to turn on HVLDO (One leg) 1'b0 - Switch is turned off 1'b1 - Switch is turned on"]
    #[inline(always)]
    pub fn srss_switch_en(&mut self) -> SRSS_SWITCH_EN_W<0> {
        SRSS_SWITCH_EN_W::new(self)
    }
    #[doc = "Bit 1 - Enable to turn on HVLDO (All legs). This must be enabled 64us after enabling SRSS_SWITCH_EN 1'b0 - Switch is turned off 1'b1 - Switch is turned on"]
    #[inline(always)]
    pub fn srss_switch_en_dly(&mut self) -> SRSS_SWITCH_EN_DLY_W<1> {
        SRSS_SWITCH_EN_DLY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MT Configuration Register to control VIO switches\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mt_vio_ctrl](index.html) module"]
pub struct MT_VIO_CTRL_SPEC;
impl crate::RegisterSpec for MT_VIO_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mt_vio_ctrl::R](R) reader structure"]
impl crate::Readable for MT_VIO_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mt_vio_ctrl::W](W) writer structure"]
impl crate::Writable for MT_VIO_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MT_VIO_CTRL to value 0"]
impl crate::Resettable for MT_VIO_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
