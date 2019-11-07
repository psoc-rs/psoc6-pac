#[doc = "Reader of register ADV_INTERVAL_TIMEOUT"]
pub type R = crate::R<u32, super::ADV_INTERVAL_TIMEOUT>;
#[doc = "Writer for register ADV_INTERVAL_TIMEOUT"]
pub type W = crate::W<u32, super::ADV_INTERVAL_TIMEOUT>;
#[doc = "Register ADV_INTERVAL_TIMEOUT `reset()`'s with value 0x20"]
impl crate::ResetValue for super::ADV_INTERVAL_TIMEOUT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x20
    }
}
#[doc = "Reader of field `ADV_INTERVAL`"]
pub type ADV_INTERVAL_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ADV_INTERVAL`"]
pub struct ADV_INTERVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADV_INTERVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7fff) | ((value as u32) & 0x7fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:14 - Range: 0x0020 to 0x4000 (For ADV_IND) 0x00A0 to 0x4000 (For ADV_SCAN_IND and NONCONN_IND) Invalid for ADV_DIRECT_IND Time = N * 0.625 msec Time Range: 20 ms to 10.24 sec. For directed advertising, firmware programs the default value of 1.28 seconds. In MMMS mode, this register is used as ADV_NI_TIMER when the ADV_NI_VALID is set by firmware"]
    #[inline(always)]
    pub fn adv_interval(&self) -> ADV_INTERVAL_R {
        ADV_INTERVAL_R::new((self.bits & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - Range: 0x0020 to 0x4000 (For ADV_IND) 0x00A0 to 0x4000 (For ADV_SCAN_IND and NONCONN_IND) Invalid for ADV_DIRECT_IND Time = N * 0.625 msec Time Range: 20 ms to 10.24 sec. For directed advertising, firmware programs the default value of 1.28 seconds. In MMMS mode, this register is used as ADV_NI_TIMER when the ADV_NI_VALID is set by firmware"]
    #[inline(always)]
    pub fn adv_interval(&mut self) -> ADV_INTERVAL_W {
        ADV_INTERVAL_W { w: self }
    }
}
