#[doc = "Register `INIT_INTERVAL` reader"]
pub struct R(crate::R<INIT_INTERVAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INIT_INTERVAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INIT_INTERVAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INIT_INTERVAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INIT_INTERVAL` writer"]
pub struct W(crate::W<INIT_INTERVAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INIT_INTERVAL_SPEC>;
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
impl From<crate::W<INIT_INTERVAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INIT_INTERVAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INIT_SCAN_INTERVAL` reader - Initiator interval register. Firmware sets the initiator's scanning interval value to this regis-ter before issuing create connection command. Interval between two consecutive scanning events. Range: 0x0004 to 0x4000 Time = N * 0.625 msec Time Range: 2.5 msec to 10.24 sec. In MMMS mode, this register is used as INIT_NI_TIMER when the INIT_NI_VALID is set by firmware"]
pub type INIT_SCAN_INTERVAL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `INIT_SCAN_INTERVAL` writer - Initiator interval register. Firmware sets the initiator's scanning interval value to this regis-ter before issuing create connection command. Interval between two consecutive scanning events. Range: 0x0004 to 0x4000 Time = N * 0.625 msec Time Range: 2.5 msec to 10.24 sec. In MMMS mode, this register is used as INIT_NI_TIMER when the INIT_NI_VALID is set by firmware"]
pub type INIT_SCAN_INTERVAL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, INIT_INTERVAL_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Initiator interval register. Firmware sets the initiator's scanning interval value to this regis-ter before issuing create connection command. Interval between two consecutive scanning events. Range: 0x0004 to 0x4000 Time = N * 0.625 msec Time Range: 2.5 msec to 10.24 sec. In MMMS mode, this register is used as INIT_NI_TIMER when the INIT_NI_VALID is set by firmware"]
    #[inline(always)]
    pub fn init_scan_interval(&self) -> INIT_SCAN_INTERVAL_R {
        INIT_SCAN_INTERVAL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Initiator interval register. Firmware sets the initiator's scanning interval value to this regis-ter before issuing create connection command. Interval between two consecutive scanning events. Range: 0x0004 to 0x4000 Time = N * 0.625 msec Time Range: 2.5 msec to 10.24 sec. In MMMS mode, this register is used as INIT_NI_TIMER when the INIT_NI_VALID is set by firmware"]
    #[inline(always)]
    pub fn init_scan_interval(&mut self) -> INIT_SCAN_INTERVAL_W<0> {
        INIT_SCAN_INTERVAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Initiator Interval Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [init_interval](index.html) module"]
pub struct INIT_INTERVAL_SPEC;
impl crate::RegisterSpec for INIT_INTERVAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [init_interval::R](R) reader structure"]
impl crate::Readable for INIT_INTERVAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [init_interval::W](W) writer structure"]
impl crate::Writable for INIT_INTERVAL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INIT_INTERVAL to value 0"]
impl crate::Resettable for INIT_INTERVAL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
