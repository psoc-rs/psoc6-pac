#[doc = "Reader of register FLASH_CTL"]
pub type R = crate::R<u32, super::FLASH_CTL>;
#[doc = "Writer for register FLASH_CTL"]
pub type W = crate::W<u32, super::FLASH_CTL>;
#[doc = "Register FLASH_CTL `reset()`'s with value 0"]
impl crate::ResetValue for super::FLASH_CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MAIN_WS`"]
pub type MAIN_WS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MAIN_WS`"]
pub struct MAIN_WS_W<'a> {
    w: &'a mut W,
}
impl<'a> MAIN_WS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `REMAP`"]
pub type REMAP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `REMAP`"]
pub struct REMAP_W<'a> {
    w: &'a mut W,
}
impl<'a> REMAP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - FLASH macro main interface wait states: '0': 0 wait states. ... '15': 15 wait states"]
    #[inline(always)]
    pub fn main_ws(&self) -> MAIN_WS_R {
        MAIN_WS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Specifies remapping of FLASH macro main region. '0': No remapping. '1': Remapping. The highest address bit of the FLASH main region is inverted. This effectively remaps the location of FLASH main region physical sectors in the logical address space. In other words, the higher half physical sectors are swapped with the lower half physical sectors. Note: remapping only affects reading of the FLASH main region (over the R interface). It does NOT affect programming/erasing of the FLASH memory region (over the C interface). E.g., for a 512 KB / 4 Mb main region, the logical address space ranges from \\[0x1000:0000, 0x1007:ffff\\] (the highest bit if the FLASH main region is bit 18). The memory has four physical sectors: sectors 0, 1, 2 and 3. If REMAP is '0', the physical regions logical addresses are as follows: - The physical region 0: \\[0x1000:0000, 0x1001:ffff\\]. - The physical region 1: \\[0x1002:0000, 0x1003:ffff\\]. - The physical region 2: \\[0x1004:0000, 0x1005:ffff\\]. - The physical region 3: \\[0x1006:0000, 0x1007:ffff\\]. If REMAP is '1', the physical regions logical addresses are as follows: - The physical region 0: \\[0x1004:0000, 0x1005:ffff\\]. - The physical region 1: \\[0x1006:0000, 0x1007:ffff\\]. - The physical region 2: \\[0x1000:0000, 0x1001:ffff\\]. - The physical region 3: \\[0x1002:0000, 0x1003:ffff\\]. Note: when the REMAP is changed, SW should invalidate the caches and buffers."]
    #[inline(always)]
    pub fn remap(&self) -> REMAP_R {
        REMAP_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - FLASH macro main interface wait states: '0': 0 wait states. ... '15': 15 wait states"]
    #[inline(always)]
    pub fn main_ws(&mut self) -> MAIN_WS_W {
        MAIN_WS_W { w: self }
    }
    #[doc = "Bit 8 - Specifies remapping of FLASH macro main region. '0': No remapping. '1': Remapping. The highest address bit of the FLASH main region is inverted. This effectively remaps the location of FLASH main region physical sectors in the logical address space. In other words, the higher half physical sectors are swapped with the lower half physical sectors. Note: remapping only affects reading of the FLASH main region (over the R interface). It does NOT affect programming/erasing of the FLASH memory region (over the C interface). E.g., for a 512 KB / 4 Mb main region, the logical address space ranges from \\[0x1000:0000, 0x1007:ffff\\] (the highest bit if the FLASH main region is bit 18). The memory has four physical sectors: sectors 0, 1, 2 and 3. If REMAP is '0', the physical regions logical addresses are as follows: - The physical region 0: \\[0x1000:0000, 0x1001:ffff\\]. - The physical region 1: \\[0x1002:0000, 0x1003:ffff\\]. - The physical region 2: \\[0x1004:0000, 0x1005:ffff\\]. - The physical region 3: \\[0x1006:0000, 0x1007:ffff\\]. If REMAP is '1', the physical regions logical addresses are as follows: - The physical region 0: \\[0x1004:0000, 0x1005:ffff\\]. - The physical region 1: \\[0x1006:0000, 0x1007:ffff\\]. - The physical region 2: \\[0x1000:0000, 0x1001:ffff\\]. - The physical region 3: \\[0x1002:0000, 0x1003:ffff\\]. Note: when the REMAP is changed, SW should invalidate the caches and buffers."]
    #[inline(always)]
    pub fn remap(&mut self) -> REMAP_W {
        REMAP_W { w: self }
    }
}
