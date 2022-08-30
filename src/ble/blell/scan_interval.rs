#[doc = "Register `SCAN_INTERVAL` reader"]
pub struct R(crate::R<SCAN_INTERVAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCAN_INTERVAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCAN_INTERVAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCAN_INTERVAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCAN_INTERVAL` writer"]
pub struct W(crate::W<SCAN_INTERVAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCAN_INTERVAL_SPEC>;
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
impl From<crate::W<SCAN_INTERVAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCAN_INTERVAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCAN_INTERVAL` reader - Scan interval register. Interval between two consecutive scanning events. Firmware sets the scanning interval value to this register before issuing start scan command. Range: 0x0004 to 0x4000 Default: 0x0010 (10 ms) Time = N * 0.625 msec Time Range: 2.5 msec to 10.24 sec. In MMMS mode, this register is used as SCAN_NI_TIMER when the SCAN_NI_VALID is set by firmware"]
pub type SCAN_INTERVAL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SCAN_INTERVAL` writer - Scan interval register. Interval between two consecutive scanning events. Firmware sets the scanning interval value to this register before issuing start scan command. Range: 0x0004 to 0x4000 Default: 0x0010 (10 ms) Time = N * 0.625 msec Time Range: 2.5 msec to 10.24 sec. In MMMS mode, this register is used as SCAN_NI_TIMER when the SCAN_NI_VALID is set by firmware"]
pub type SCAN_INTERVAL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SCAN_INTERVAL_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Scan interval register. Interval between two consecutive scanning events. Firmware sets the scanning interval value to this register before issuing start scan command. Range: 0x0004 to 0x4000 Default: 0x0010 (10 ms) Time = N * 0.625 msec Time Range: 2.5 msec to 10.24 sec. In MMMS mode, this register is used as SCAN_NI_TIMER when the SCAN_NI_VALID is set by firmware"]
    #[inline(always)]
    pub fn scan_interval(&self) -> SCAN_INTERVAL_R {
        SCAN_INTERVAL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Scan interval register. Interval between two consecutive scanning events. Firmware sets the scanning interval value to this register before issuing start scan command. Range: 0x0004 to 0x4000 Default: 0x0010 (10 ms) Time = N * 0.625 msec Time Range: 2.5 msec to 10.24 sec. In MMMS mode, this register is used as SCAN_NI_TIMER when the SCAN_NI_VALID is set by firmware"]
    #[inline(always)]
    pub fn scan_interval(&mut self) -> SCAN_INTERVAL_W<0> {
        SCAN_INTERVAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Scan Interval Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scan_interval](index.html) module"]
pub struct SCAN_INTERVAL_SPEC;
impl crate::RegisterSpec for SCAN_INTERVAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scan_interval::R](R) reader structure"]
impl crate::Readable for SCAN_INTERVAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scan_interval::W](W) writer structure"]
impl crate::Writable for SCAN_INTERVAL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCAN_INTERVAL to value 0x10"]
impl crate::Resettable for SCAN_INTERVAL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x10
    }
}
