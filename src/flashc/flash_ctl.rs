#[doc = "Register `FLASH_CTL` reader"]
pub struct R(crate::R<FLASH_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLASH_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLASH_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLASH_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLASH_CTL` writer"]
pub struct W(crate::W<FLASH_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLASH_CTL_SPEC>;
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
impl From<crate::W<FLASH_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLASH_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MAIN_WS` reader - FLASH macro main interface wait states: 0: 0 wait states. ... 15: 15 wait states"]
pub type MAIN_WS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAIN_WS` writer - FLASH macro main interface wait states: 0: 0 wait states. ... 15: 15 wait states"]
pub type MAIN_WS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FLASH_CTL_SPEC, u8, u8, 4, O>;
#[doc = "Field `REMAP` reader - Specifies remapping of FLASH macro main region. 0: No remapping. 1: Remapping. The highest address bit of the FLASH main region is inverted. This effectively remaps the location of FLASH main region physical sectors in the logical address space. In other words, the higher half physical sectors are swapped with the lower half physical sectors. Note: remapping only affects reading of the FLASH main region (over the R interface). It does NOT affect programming/erasing of the FLASH memory region (over the C interface). E.g., for a 512 KB / 4 Mb main region, the logical address space ranges from \\[0x1000:0000, 0x1007:ffff\\]
(the highest bit if the FLASH main region is bit 18). The memory has four physical sectors: sectors 0, 1, 2 and 3. If REMAP is '0', the physical regions logical addresses are as follows: - The physical region 0: \\[0x1000:0000, 0x1001:ffff\\]. - The physical region 1: \\[0x1002:0000, 0x1003:ffff\\]. - The physical region 2: \\[0x1004:0000, 0x1005:ffff\\]. - The physical region 3: \\[0x1006:0000, 0x1007:ffff\\]. If REMAP is '1', the physical regions logical addresses are as follows: - The physical region 0: \\[0x1004:0000, 0x1005:ffff\\]. - The physical region 1: \\[0x1006:0000, 0x1007:ffff\\]. - The physical region 2: \\[0x1000:0000, 0x1001:ffff\\]. - The physical region 3: \\[0x1002:0000, 0x1003:ffff\\]. Note: when the REMAP is changed, SW should invalidate the caches and buffers."]
pub type REMAP_R = crate::BitReader<bool>;
#[doc = "Field `REMAP` writer - Specifies remapping of FLASH macro main region. 0: No remapping. 1: Remapping. The highest address bit of the FLASH main region is inverted. This effectively remaps the location of FLASH main region physical sectors in the logical address space. In other words, the higher half physical sectors are swapped with the lower half physical sectors. Note: remapping only affects reading of the FLASH main region (over the R interface). It does NOT affect programming/erasing of the FLASH memory region (over the C interface). E.g., for a 512 KB / 4 Mb main region, the logical address space ranges from \\[0x1000:0000, 0x1007:ffff\\]
(the highest bit if the FLASH main region is bit 18). The memory has four physical sectors: sectors 0, 1, 2 and 3. If REMAP is '0', the physical regions logical addresses are as follows: - The physical region 0: \\[0x1000:0000, 0x1001:ffff\\]. - The physical region 1: \\[0x1002:0000, 0x1003:ffff\\]. - The physical region 2: \\[0x1004:0000, 0x1005:ffff\\]. - The physical region 3: \\[0x1006:0000, 0x1007:ffff\\]. If REMAP is '1', the physical regions logical addresses are as follows: - The physical region 0: \\[0x1004:0000, 0x1005:ffff\\]. - The physical region 1: \\[0x1006:0000, 0x1007:ffff\\]. - The physical region 2: \\[0x1000:0000, 0x1001:ffff\\]. - The physical region 3: \\[0x1002:0000, 0x1003:ffff\\]. Note: when the REMAP is changed, SW should invalidate the caches and buffers."]
pub type REMAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_CTL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - FLASH macro main interface wait states: 0: 0 wait states. ... 15: 15 wait states"]
    #[inline(always)]
    pub fn main_ws(&self) -> MAIN_WS_R {
        MAIN_WS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Specifies remapping of FLASH macro main region. 0: No remapping. 1: Remapping. The highest address bit of the FLASH main region is inverted. This effectively remaps the location of FLASH main region physical sectors in the logical address space. In other words, the higher half physical sectors are swapped with the lower half physical sectors. Note: remapping only affects reading of the FLASH main region (over the R interface). It does NOT affect programming/erasing of the FLASH memory region (over the C interface). E.g., for a 512 KB / 4 Mb main region, the logical address space ranges from \\[0x1000:0000, 0x1007:ffff\\]
(the highest bit if the FLASH main region is bit 18). The memory has four physical sectors: sectors 0, 1, 2 and 3. If REMAP is '0', the physical regions logical addresses are as follows: - The physical region 0: \\[0x1000:0000, 0x1001:ffff\\]. - The physical region 1: \\[0x1002:0000, 0x1003:ffff\\]. - The physical region 2: \\[0x1004:0000, 0x1005:ffff\\]. - The physical region 3: \\[0x1006:0000, 0x1007:ffff\\]. If REMAP is '1', the physical regions logical addresses are as follows: - The physical region 0: \\[0x1004:0000, 0x1005:ffff\\]. - The physical region 1: \\[0x1006:0000, 0x1007:ffff\\]. - The physical region 2: \\[0x1000:0000, 0x1001:ffff\\]. - The physical region 3: \\[0x1002:0000, 0x1003:ffff\\]. Note: when the REMAP is changed, SW should invalidate the caches and buffers."]
    #[inline(always)]
    pub fn remap(&self) -> REMAP_R {
        REMAP_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - FLASH macro main interface wait states: 0: 0 wait states. ... 15: 15 wait states"]
    #[inline(always)]
    pub fn main_ws(&mut self) -> MAIN_WS_W<0> {
        MAIN_WS_W::new(self)
    }
    #[doc = "Bit 8 - Specifies remapping of FLASH macro main region. 0: No remapping. 1: Remapping. The highest address bit of the FLASH main region is inverted. This effectively remaps the location of FLASH main region physical sectors in the logical address space. In other words, the higher half physical sectors are swapped with the lower half physical sectors. Note: remapping only affects reading of the FLASH main region (over the R interface). It does NOT affect programming/erasing of the FLASH memory region (over the C interface). E.g., for a 512 KB / 4 Mb main region, the logical address space ranges from \\[0x1000:0000, 0x1007:ffff\\]
(the highest bit if the FLASH main region is bit 18). The memory has four physical sectors: sectors 0, 1, 2 and 3. If REMAP is '0', the physical regions logical addresses are as follows: - The physical region 0: \\[0x1000:0000, 0x1001:ffff\\]. - The physical region 1: \\[0x1002:0000, 0x1003:ffff\\]. - The physical region 2: \\[0x1004:0000, 0x1005:ffff\\]. - The physical region 3: \\[0x1006:0000, 0x1007:ffff\\]. If REMAP is '1', the physical regions logical addresses are as follows: - The physical region 0: \\[0x1004:0000, 0x1005:ffff\\]. - The physical region 1: \\[0x1006:0000, 0x1007:ffff\\]. - The physical region 2: \\[0x1000:0000, 0x1001:ffff\\]. - The physical region 3: \\[0x1002:0000, 0x1003:ffff\\]. Note: when the REMAP is changed, SW should invalidate the caches and buffers."]
    #[inline(always)]
    pub fn remap(&mut self) -> REMAP_W<8> {
        REMAP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_ctl](index.html) module"]
pub struct FLASH_CTL_SPEC;
impl crate::RegisterSpec for FLASH_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flash_ctl::R](R) reader structure"]
impl crate::Readable for FLASH_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flash_ctl::W](W) writer structure"]
impl crate::Writable for FLASH_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLASH_CTL to value 0"]
impl crate::Resettable for FLASH_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
