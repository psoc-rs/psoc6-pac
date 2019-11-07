#[doc = "Reader of register HOST_FCOMP"]
pub type R = crate::R<u32, super::HOST_FCOMP>;
#[doc = "Writer for register HOST_FCOMP"]
pub type W = crate::W<u32, super::HOST_FCOMP>;
#[doc = "Register HOST_FCOMP `reset()`'s with value 0"]
impl crate::ResetValue for super::HOST_FCOMP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FRAMECOMP`"]
pub type FRAMECOMP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FRAMECOMP`"]
pub struct FRAMECOMP_W<'a> {
    w: &'a mut W,
}
impl<'a> FRAMECOMP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - These bits are used to specify the data to be compared with the low-order eight bits of a frame number when sending a SOF token. If the SOFSTEP bit of Host Control 2 Register (HOST_CTL2) is '0', the frame number of SOF is compared with the value of this register when sending a SOF token. If they match, the SOFIRQ bit of the Interrupt USB Host Register (INTR_USBHOST) is set to '1'. The setting of this register is invalid when the SOFSTEP bit of Host Control 2 Register (HOST_CTL2) is '1'. Note: - This bit is not initialized even if the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn framecomp(&self) -> FRAMECOMP_R {
        FRAMECOMP_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - These bits are used to specify the data to be compared with the low-order eight bits of a frame number when sending a SOF token. If the SOFSTEP bit of Host Control 2 Register (HOST_CTL2) is '0', the frame number of SOF is compared with the value of this register when sending a SOF token. If they match, the SOFIRQ bit of the Interrupt USB Host Register (INTR_USBHOST) is set to '1'. The setting of this register is invalid when the SOFSTEP bit of Host Control 2 Register (HOST_CTL2) is '1'. Note: - This bit is not initialized even if the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn framecomp(&mut self) -> FRAMECOMP_W {
        FRAMECOMP_W { w: self }
    }
}
