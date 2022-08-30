#[doc = "Register `SYNC_CTL` reader"]
pub struct R(crate::R<SYNC_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYNC_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYNC_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYNC_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYNC_CTL` writer"]
pub struct W(crate::W<SYNC_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYNC_CTL_SPEC>;
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
impl From<crate::W<SYNC_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYNC_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IO_SYNC_EN` reader - Synchronization of the IO pin input signals to 'clk_fabric', one bit for each IO pin: IO_SYNC_EN\\[i\\]
is for IO pin i. '0': No synchronization. '1': Synchronization."]
pub type IO_SYNC_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IO_SYNC_EN` writer - Synchronization of the IO pin input signals to 'clk_fabric', one bit for each IO pin: IO_SYNC_EN\\[i\\]
is for IO pin i. '0': No synchronization. '1': Synchronization."]
pub type IO_SYNC_EN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SYNC_CTL_SPEC, u8, u8, 8, O>;
#[doc = "Field `CHIP_SYNC_EN` reader - Synchronization of the chip input signals to 'clk_fabric', one bit for each input: CHIP_SYNC_EN\\[i\\]
is for input i. '0': No synchronization. '1': Synchronization."]
pub type CHIP_SYNC_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CHIP_SYNC_EN` writer - Synchronization of the chip input signals to 'clk_fabric', one bit for each input: CHIP_SYNC_EN\\[i\\]
is for input i. '0': No synchronization. '1': Synchronization."]
pub type CHIP_SYNC_EN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SYNC_CTL_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Synchronization of the IO pin input signals to 'clk_fabric', one bit for each IO pin: IO_SYNC_EN\\[i\\]
is for IO pin i. '0': No synchronization. '1': Synchronization."]
    #[inline(always)]
    pub fn io_sync_en(&self) -> IO_SYNC_EN_R {
        IO_SYNC_EN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Synchronization of the chip input signals to 'clk_fabric', one bit for each input: CHIP_SYNC_EN\\[i\\]
is for input i. '0': No synchronization. '1': Synchronization."]
    #[inline(always)]
    pub fn chip_sync_en(&self) -> CHIP_SYNC_EN_R {
        CHIP_SYNC_EN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Synchronization of the IO pin input signals to 'clk_fabric', one bit for each IO pin: IO_SYNC_EN\\[i\\]
is for IO pin i. '0': No synchronization. '1': Synchronization."]
    #[inline(always)]
    pub fn io_sync_en(&mut self) -> IO_SYNC_EN_W<0> {
        IO_SYNC_EN_W::new(self)
    }
    #[doc = "Bits 8:15 - Synchronization of the chip input signals to 'clk_fabric', one bit for each input: CHIP_SYNC_EN\\[i\\]
is for input i. '0': No synchronization. '1': Synchronization."]
    #[inline(always)]
    pub fn chip_sync_en(&mut self) -> CHIP_SYNC_EN_W<8> {
        CHIP_SYNC_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Synchronization control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sync_ctl](index.html) module"]
pub struct SYNC_CTL_SPEC;
impl crate::RegisterSpec for SYNC_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sync_ctl::R](R) reader structure"]
impl crate::Readable for SYNC_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sync_ctl::W](W) writer structure"]
impl crate::Writable for SYNC_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYNC_CTL to value 0"]
impl crate::Resettable for SYNC_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
