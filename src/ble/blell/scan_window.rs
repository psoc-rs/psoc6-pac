#[doc = "Register `SCAN_WINDOW` reader"]
pub struct R(crate::R<SCAN_WINDOW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCAN_WINDOW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCAN_WINDOW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCAN_WINDOW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCAN_WINDOW` writer"]
pub struct W(crate::W<SCAN_WINDOW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCAN_WINDOW_SPEC>;
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
impl From<crate::W<SCAN_WINDOW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCAN_WINDOW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCAN_WINDOW` reader - Duration of scan in a scanning event, which should be less than or equal to scan interval value. Firmware sets the scan window value to this register before issuing start scan command. Range: 0x0004 to 0x4000 Default: 0x0010 (10 ms) Time = N * 0.625 msec Time Range: 2.5 msec to 10.24 sec. (To prevent ADV RX - SCAN REQ TX - SCAN RSP RX spilling over across the scan window, when not in continuous scan, the scan window must be 2 slots less that the scan interval."]
pub type SCAN_WINDOW_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SCAN_WINDOW` writer - Duration of scan in a scanning event, which should be less than or equal to scan interval value. Firmware sets the scan window value to this register before issuing start scan command. Range: 0x0004 to 0x4000 Default: 0x0010 (10 ms) Time = N * 0.625 msec Time Range: 2.5 msec to 10.24 sec. (To prevent ADV RX - SCAN REQ TX - SCAN RSP RX spilling over across the scan window, when not in continuous scan, the scan window must be 2 slots less that the scan interval."]
pub type SCAN_WINDOW_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SCAN_WINDOW_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Duration of scan in a scanning event, which should be less than or equal to scan interval value. Firmware sets the scan window value to this register before issuing start scan command. Range: 0x0004 to 0x4000 Default: 0x0010 (10 ms) Time = N * 0.625 msec Time Range: 2.5 msec to 10.24 sec. (To prevent ADV RX - SCAN REQ TX - SCAN RSP RX spilling over across the scan window, when not in continuous scan, the scan window must be 2 slots less that the scan interval."]
    #[inline(always)]
    pub fn scan_window(&self) -> SCAN_WINDOW_R {
        SCAN_WINDOW_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Duration of scan in a scanning event, which should be less than or equal to scan interval value. Firmware sets the scan window value to this register before issuing start scan command. Range: 0x0004 to 0x4000 Default: 0x0010 (10 ms) Time = N * 0.625 msec Time Range: 2.5 msec to 10.24 sec. (To prevent ADV RX - SCAN REQ TX - SCAN RSP RX spilling over across the scan window, when not in continuous scan, the scan window must be 2 slots less that the scan interval."]
    #[inline(always)]
    pub fn scan_window(&mut self) -> SCAN_WINDOW_W<0> {
        SCAN_WINDOW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Scan window Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scan_window](index.html) module"]
pub struct SCAN_WINDOW_SPEC;
impl crate::RegisterSpec for SCAN_WINDOW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scan_window::R](R) reader structure"]
impl crate::Readable for SCAN_WINDOW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scan_window::W](W) writer structure"]
impl crate::Writable for SCAN_WINDOW_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCAN_WINDOW to value 0x10"]
impl crate::Resettable for SCAN_WINDOW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x10
    }
}
