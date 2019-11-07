#[doc = "Reader of register SYNC_CTL"]
pub type R = crate::R<u32, super::SYNC_CTL>;
#[doc = "Writer for register SYNC_CTL"]
pub type W = crate::W<u32, super::SYNC_CTL>;
#[doc = "Register SYNC_CTL `reset()`'s with value 0"]
impl crate::ResetValue for super::SYNC_CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IO_SYNC_EN`"]
pub type IO_SYNC_EN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IO_SYNC_EN`"]
pub struct IO_SYNC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> IO_SYNC_EN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `CHIP_SYNC_EN`"]
pub type CHIP_SYNC_EN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CHIP_SYNC_EN`"]
pub struct CHIP_SYNC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CHIP_SYNC_EN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Synchronization of the IO pin input signals to 'clk_fabric', one bit for each IO pin: IO_SYNC_EN\\[i\\] is for IO pin i. '0': No synchronization. '1': Synchronization."]
    #[inline(always)]
    pub fn io_sync_en(&self) -> IO_SYNC_EN_R {
        IO_SYNC_EN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Synchronization of the chip input signals to 'clk_fabric', one bit for each input: CHIP_SYNC_EN\\[i\\] is for input i. '0': No synchronization. '1': Synchronization."]
    #[inline(always)]
    pub fn chip_sync_en(&self) -> CHIP_SYNC_EN_R {
        CHIP_SYNC_EN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Synchronization of the IO pin input signals to 'clk_fabric', one bit for each IO pin: IO_SYNC_EN\\[i\\] is for IO pin i. '0': No synchronization. '1': Synchronization."]
    #[inline(always)]
    pub fn io_sync_en(&mut self) -> IO_SYNC_EN_W {
        IO_SYNC_EN_W { w: self }
    }
    #[doc = "Bits 8:15 - Synchronization of the chip input signals to 'clk_fabric', one bit for each input: CHIP_SYNC_EN\\[i\\] is for input i. '0': No synchronization. '1': Synchronization."]
    #[inline(always)]
    pub fn chip_sync_en(&mut self) -> CHIP_SYNC_EN_W {
        CHIP_SYNC_EN_W { w: self }
    }
}
