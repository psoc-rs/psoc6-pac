#[doc = "Reader of register PWR_HIB_DATA[%s]"]
pub type R = crate::R<u32, super::PWR_HIB_DATA>;
#[doc = "Writer for register PWR_HIB_DATA[%s]"]
pub type W = crate::W<u32, super::PWR_HIB_DATA>;
#[doc = "Register PWR_HIB_DATA[%s] `reset()`'s with value 0"]
impl crate::ResetValue for super::PWR_HIB_DATA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HIB_DATA`"]
pub type HIB_DATA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `HIB_DATA`"]
pub struct HIB_DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> HIB_DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Additional data that is retained through a HIBERNATE/WAKEUP sequence that can be used by firmware for any application-specific purpose. Note that waking up from HIBERNATE using XRES will reset this register."]
    #[inline(always)]
    pub fn hib_data(&self) -> HIB_DATA_R {
        HIB_DATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Additional data that is retained through a HIBERNATE/WAKEUP sequence that can be used by firmware for any application-specific purpose. Note that waking up from HIBERNATE using XRES will reset this register."]
    #[inline(always)]
    pub fn hib_data(&mut self) -> HIB_DATA_W {
        HIB_DATA_W { w: self }
    }
}
