#[doc = "Register `CFG_SIO` reader"]
pub struct R(crate::R<CFG_SIO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_SIO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_SIO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_SIO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG_SIO` writer"]
pub struct W(crate::W<CFG_SIO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_SIO_SPEC>;
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
impl From<crate::W<CFG_SIO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG_SIO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VREG_EN01` reader - The regulated output mode is selected ONLY if the CFG.DRIVE_MODE bits are set to the strong pull up (Z_1 = '5') mode If the CFG.DRIVE_MODE bits are set to any other mode the regulated output buffer will be disabled and the standard CMOS output buffer is used."]
pub type VREG_EN01_R = crate::BitReader<bool>;
#[doc = "Field `VREG_EN01` writer - The regulated output mode is selected ONLY if the CFG.DRIVE_MODE bits are set to the strong pull up (Z_1 = '5') mode If the CFG.DRIVE_MODE bits are set to any other mode the regulated output buffer will be disabled and the standard CMOS output buffer is used."]
pub type VREG_EN01_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SIO_SPEC, bool, O>;
#[doc = "Field `IBUF_SEL01` reader - N/A"]
pub type IBUF_SEL01_R = crate::BitReader<bool>;
#[doc = "Field `IBUF_SEL01` writer - N/A"]
pub type IBUF_SEL01_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SIO_SPEC, bool, O>;
#[doc = "Field `VTRIP_SEL01` reader - N/A"]
pub type VTRIP_SEL01_R = crate::BitReader<bool>;
#[doc = "Field `VTRIP_SEL01` writer - N/A"]
pub type VTRIP_SEL01_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SIO_SPEC, bool, O>;
#[doc = "Field `VREF_SEL01` reader - N/A"]
pub type VREF_SEL01_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VREF_SEL01` writer - N/A"]
pub type VREF_SEL01_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_SIO_SPEC, u8, u8, 2, O>;
#[doc = "Field `VOH_SEL01` reader - Selects trip-point of input buffer. In single ended input buffer mode (IBUF01_SEL = '0'): 0: input buffer functions as a CMOS input buffer. 1: input buffer functions as a LVTTL input buffer. In differential input buffer mode (IBUF01_SEL = '1'): VTRIP_SEL=0: a) VREF_SEL=00, VOH_SEL=X -> Trip point=50 percent of vddio b) VREF_SEL=01, VOH_SEL=000 -> Trip point=Vohref (buffered) c) VREF_SEL=01, VOH_SEL=\\[1-7\\]
-> Input buffer functions as CMOS input buffer. d) VREF_SEL=10/11, VOH_SEL=000 -> Trip point=Amuxbus_a/b (buffered) e) VREF_SEL=10/11, VOH_SEL=\\[1-7\\]
-> Input buffer functions as CMOS input buffer. VTRIP_SEL=1: a) VREF_SEL=00, VOH_SEL=X -> Trip point=40 percent of vddio b) VREF_SEL=01, VOH_SEL=000 -> Trip point=0.5*Vohref c) VREF_SEL=01, VOH_SEL=\\[1-7\\]
-> Input buffer functions as LVTTL input buffer. d) VREF_SEL=10/11, VOH_SEL=000 -> Trip point=0.5*Amuxbus_a/b (buffered) e) VREF_SEL=10/11, VOH_SEL=\\[1-7\\]
-> Input buffer functions as LVTTL input buffer."]
pub type VOH_SEL01_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VOH_SEL01` writer - Selects trip-point of input buffer. In single ended input buffer mode (IBUF01_SEL = '0'): 0: input buffer functions as a CMOS input buffer. 1: input buffer functions as a LVTTL input buffer. In differential input buffer mode (IBUF01_SEL = '1'): VTRIP_SEL=0: a) VREF_SEL=00, VOH_SEL=X -> Trip point=50 percent of vddio b) VREF_SEL=01, VOH_SEL=000 -> Trip point=Vohref (buffered) c) VREF_SEL=01, VOH_SEL=\\[1-7\\]
-> Input buffer functions as CMOS input buffer. d) VREF_SEL=10/11, VOH_SEL=000 -> Trip point=Amuxbus_a/b (buffered) e) VREF_SEL=10/11, VOH_SEL=\\[1-7\\]
-> Input buffer functions as CMOS input buffer. VTRIP_SEL=1: a) VREF_SEL=00, VOH_SEL=X -> Trip point=40 percent of vddio b) VREF_SEL=01, VOH_SEL=000 -> Trip point=0.5*Vohref c) VREF_SEL=01, VOH_SEL=\\[1-7\\]
-> Input buffer functions as LVTTL input buffer. d) VREF_SEL=10/11, VOH_SEL=000 -> Trip point=0.5*Amuxbus_a/b (buffered) e) VREF_SEL=10/11, VOH_SEL=\\[1-7\\]
-> Input buffer functions as LVTTL input buffer."]
pub type VOH_SEL01_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_SIO_SPEC, u8, u8, 3, O>;
#[doc = "Field `VREG_EN23` reader - N/A"]
pub type VREG_EN23_R = crate::BitReader<bool>;
#[doc = "Field `VREG_EN23` writer - N/A"]
pub type VREG_EN23_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SIO_SPEC, bool, O>;
#[doc = "Field `IBUF_SEL23` reader - N/A"]
pub type IBUF_SEL23_R = crate::BitReader<bool>;
#[doc = "Field `IBUF_SEL23` writer - N/A"]
pub type IBUF_SEL23_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SIO_SPEC, bool, O>;
#[doc = "Field `VTRIP_SEL23` reader - N/A"]
pub type VTRIP_SEL23_R = crate::BitReader<bool>;
#[doc = "Field `VTRIP_SEL23` writer - N/A"]
pub type VTRIP_SEL23_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SIO_SPEC, bool, O>;
#[doc = "Field `VREF_SEL23` reader - N/A"]
pub type VREF_SEL23_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VREF_SEL23` writer - N/A"]
pub type VREF_SEL23_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_SIO_SPEC, u8, u8, 2, O>;
#[doc = "Field `VOH_SEL23` reader - N/A"]
pub type VOH_SEL23_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VOH_SEL23` writer - N/A"]
pub type VOH_SEL23_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_SIO_SPEC, u8, u8, 3, O>;
#[doc = "Field `VREG_EN45` reader - N/A"]
pub type VREG_EN45_R = crate::BitReader<bool>;
#[doc = "Field `VREG_EN45` writer - N/A"]
pub type VREG_EN45_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SIO_SPEC, bool, O>;
#[doc = "Field `IBUF_SEL45` reader - N/A"]
pub type IBUF_SEL45_R = crate::BitReader<bool>;
#[doc = "Field `IBUF_SEL45` writer - N/A"]
pub type IBUF_SEL45_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SIO_SPEC, bool, O>;
#[doc = "Field `VTRIP_SEL45` reader - N/A"]
pub type VTRIP_SEL45_R = crate::BitReader<bool>;
#[doc = "Field `VTRIP_SEL45` writer - N/A"]
pub type VTRIP_SEL45_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SIO_SPEC, bool, O>;
#[doc = "Field `VREF_SEL45` reader - N/A"]
pub type VREF_SEL45_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VREF_SEL45` writer - N/A"]
pub type VREF_SEL45_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_SIO_SPEC, u8, u8, 2, O>;
#[doc = "Field `VOH_SEL45` reader - N/A"]
pub type VOH_SEL45_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VOH_SEL45` writer - N/A"]
pub type VOH_SEL45_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_SIO_SPEC, u8, u8, 3, O>;
#[doc = "Field `VREG_EN67` reader - N/A"]
pub type VREG_EN67_R = crate::BitReader<bool>;
#[doc = "Field `VREG_EN67` writer - N/A"]
pub type VREG_EN67_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SIO_SPEC, bool, O>;
#[doc = "Field `IBUF_SEL67` reader - N/A"]
pub type IBUF_SEL67_R = crate::BitReader<bool>;
#[doc = "Field `IBUF_SEL67` writer - N/A"]
pub type IBUF_SEL67_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SIO_SPEC, bool, O>;
#[doc = "Field `VTRIP_SEL67` reader - N/A"]
pub type VTRIP_SEL67_R = crate::BitReader<bool>;
#[doc = "Field `VTRIP_SEL67` writer - N/A"]
pub type VTRIP_SEL67_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SIO_SPEC, bool, O>;
#[doc = "Field `VREF_SEL67` reader - N/A"]
pub type VREF_SEL67_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VREF_SEL67` writer - N/A"]
pub type VREF_SEL67_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_SIO_SPEC, u8, u8, 2, O>;
#[doc = "Field `VOH_SEL67` reader - N/A"]
pub type VOH_SEL67_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VOH_SEL67` writer - N/A"]
pub type VOH_SEL67_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_SIO_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bit 0 - The regulated output mode is selected ONLY if the CFG.DRIVE_MODE bits are set to the strong pull up (Z_1 = '5') mode If the CFG.DRIVE_MODE bits are set to any other mode the regulated output buffer will be disabled and the standard CMOS output buffer is used."]
    #[inline(always)]
    pub fn vreg_en01(&self) -> VREG_EN01_R {
        VREG_EN01_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - N/A"]
    #[inline(always)]
    pub fn ibuf_sel01(&self) -> IBUF_SEL01_R {
        IBUF_SEL01_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - N/A"]
    #[inline(always)]
    pub fn vtrip_sel01(&self) -> VTRIP_SEL01_R {
        VTRIP_SEL01_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - N/A"]
    #[inline(always)]
    pub fn vref_sel01(&self) -> VREF_SEL01_R {
        VREF_SEL01_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:7 - Selects trip-point of input buffer. In single ended input buffer mode (IBUF01_SEL = '0'): 0: input buffer functions as a CMOS input buffer. 1: input buffer functions as a LVTTL input buffer. In differential input buffer mode (IBUF01_SEL = '1'): VTRIP_SEL=0: a) VREF_SEL=00, VOH_SEL=X -> Trip point=50 percent of vddio b) VREF_SEL=01, VOH_SEL=000 -> Trip point=Vohref (buffered) c) VREF_SEL=01, VOH_SEL=\\[1-7\\]
-> Input buffer functions as CMOS input buffer. d) VREF_SEL=10/11, VOH_SEL=000 -> Trip point=Amuxbus_a/b (buffered) e) VREF_SEL=10/11, VOH_SEL=\\[1-7\\]
-> Input buffer functions as CMOS input buffer. VTRIP_SEL=1: a) VREF_SEL=00, VOH_SEL=X -> Trip point=40 percent of vddio b) VREF_SEL=01, VOH_SEL=000 -> Trip point=0.5*Vohref c) VREF_SEL=01, VOH_SEL=\\[1-7\\]
-> Input buffer functions as LVTTL input buffer. d) VREF_SEL=10/11, VOH_SEL=000 -> Trip point=0.5*Amuxbus_a/b (buffered) e) VREF_SEL=10/11, VOH_SEL=\\[1-7\\]
-> Input buffer functions as LVTTL input buffer."]
    #[inline(always)]
    pub fn voh_sel01(&self) -> VOH_SEL01_R {
        VOH_SEL01_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - N/A"]
    #[inline(always)]
    pub fn vreg_en23(&self) -> VREG_EN23_R {
        VREG_EN23_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - N/A"]
    #[inline(always)]
    pub fn ibuf_sel23(&self) -> IBUF_SEL23_R {
        IBUF_SEL23_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - N/A"]
    #[inline(always)]
    pub fn vtrip_sel23(&self) -> VTRIP_SEL23_R {
        VTRIP_SEL23_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:12 - N/A"]
    #[inline(always)]
    pub fn vref_sel23(&self) -> VREF_SEL23_R {
        VREF_SEL23_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bits 13:15 - N/A"]
    #[inline(always)]
    pub fn voh_sel23(&self) -> VOH_SEL23_R {
        VOH_SEL23_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bit 16 - N/A"]
    #[inline(always)]
    pub fn vreg_en45(&self) -> VREG_EN45_R {
        VREG_EN45_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - N/A"]
    #[inline(always)]
    pub fn ibuf_sel45(&self) -> IBUF_SEL45_R {
        IBUF_SEL45_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - N/A"]
    #[inline(always)]
    pub fn vtrip_sel45(&self) -> VTRIP_SEL45_R {
        VTRIP_SEL45_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:20 - N/A"]
    #[inline(always)]
    pub fn vref_sel45(&self) -> VREF_SEL45_R {
        VREF_SEL45_R::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bits 21:23 - N/A"]
    #[inline(always)]
    pub fn voh_sel45(&self) -> VOH_SEL45_R {
        VOH_SEL45_R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bit 24 - N/A"]
    #[inline(always)]
    pub fn vreg_en67(&self) -> VREG_EN67_R {
        VREG_EN67_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - N/A"]
    #[inline(always)]
    pub fn ibuf_sel67(&self) -> IBUF_SEL67_R {
        IBUF_SEL67_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - N/A"]
    #[inline(always)]
    pub fn vtrip_sel67(&self) -> VTRIP_SEL67_R {
        VTRIP_SEL67_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:28 - N/A"]
    #[inline(always)]
    pub fn vref_sel67(&self) -> VREF_SEL67_R {
        VREF_SEL67_R::new(((self.bits >> 27) & 3) as u8)
    }
    #[doc = "Bits 29:31 - N/A"]
    #[inline(always)]
    pub fn voh_sel67(&self) -> VOH_SEL67_R {
        VOH_SEL67_R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - The regulated output mode is selected ONLY if the CFG.DRIVE_MODE bits are set to the strong pull up (Z_1 = '5') mode If the CFG.DRIVE_MODE bits are set to any other mode the regulated output buffer will be disabled and the standard CMOS output buffer is used."]
    #[inline(always)]
    pub fn vreg_en01(&mut self) -> VREG_EN01_W<0> {
        VREG_EN01_W::new(self)
    }
    #[doc = "Bit 1 - N/A"]
    #[inline(always)]
    pub fn ibuf_sel01(&mut self) -> IBUF_SEL01_W<1> {
        IBUF_SEL01_W::new(self)
    }
    #[doc = "Bit 2 - N/A"]
    #[inline(always)]
    pub fn vtrip_sel01(&mut self) -> VTRIP_SEL01_W<2> {
        VTRIP_SEL01_W::new(self)
    }
    #[doc = "Bits 3:4 - N/A"]
    #[inline(always)]
    pub fn vref_sel01(&mut self) -> VREF_SEL01_W<3> {
        VREF_SEL01_W::new(self)
    }
    #[doc = "Bits 5:7 - Selects trip-point of input buffer. In single ended input buffer mode (IBUF01_SEL = '0'): 0: input buffer functions as a CMOS input buffer. 1: input buffer functions as a LVTTL input buffer. In differential input buffer mode (IBUF01_SEL = '1'): VTRIP_SEL=0: a) VREF_SEL=00, VOH_SEL=X -> Trip point=50 percent of vddio b) VREF_SEL=01, VOH_SEL=000 -> Trip point=Vohref (buffered) c) VREF_SEL=01, VOH_SEL=\\[1-7\\]
-> Input buffer functions as CMOS input buffer. d) VREF_SEL=10/11, VOH_SEL=000 -> Trip point=Amuxbus_a/b (buffered) e) VREF_SEL=10/11, VOH_SEL=\\[1-7\\]
-> Input buffer functions as CMOS input buffer. VTRIP_SEL=1: a) VREF_SEL=00, VOH_SEL=X -> Trip point=40 percent of vddio b) VREF_SEL=01, VOH_SEL=000 -> Trip point=0.5*Vohref c) VREF_SEL=01, VOH_SEL=\\[1-7\\]
-> Input buffer functions as LVTTL input buffer. d) VREF_SEL=10/11, VOH_SEL=000 -> Trip point=0.5*Amuxbus_a/b (buffered) e) VREF_SEL=10/11, VOH_SEL=\\[1-7\\]
-> Input buffer functions as LVTTL input buffer."]
    #[inline(always)]
    pub fn voh_sel01(&mut self) -> VOH_SEL01_W<5> {
        VOH_SEL01_W::new(self)
    }
    #[doc = "Bit 8 - N/A"]
    #[inline(always)]
    pub fn vreg_en23(&mut self) -> VREG_EN23_W<8> {
        VREG_EN23_W::new(self)
    }
    #[doc = "Bit 9 - N/A"]
    #[inline(always)]
    pub fn ibuf_sel23(&mut self) -> IBUF_SEL23_W<9> {
        IBUF_SEL23_W::new(self)
    }
    #[doc = "Bit 10 - N/A"]
    #[inline(always)]
    pub fn vtrip_sel23(&mut self) -> VTRIP_SEL23_W<10> {
        VTRIP_SEL23_W::new(self)
    }
    #[doc = "Bits 11:12 - N/A"]
    #[inline(always)]
    pub fn vref_sel23(&mut self) -> VREF_SEL23_W<11> {
        VREF_SEL23_W::new(self)
    }
    #[doc = "Bits 13:15 - N/A"]
    #[inline(always)]
    pub fn voh_sel23(&mut self) -> VOH_SEL23_W<13> {
        VOH_SEL23_W::new(self)
    }
    #[doc = "Bit 16 - N/A"]
    #[inline(always)]
    pub fn vreg_en45(&mut self) -> VREG_EN45_W<16> {
        VREG_EN45_W::new(self)
    }
    #[doc = "Bit 17 - N/A"]
    #[inline(always)]
    pub fn ibuf_sel45(&mut self) -> IBUF_SEL45_W<17> {
        IBUF_SEL45_W::new(self)
    }
    #[doc = "Bit 18 - N/A"]
    #[inline(always)]
    pub fn vtrip_sel45(&mut self) -> VTRIP_SEL45_W<18> {
        VTRIP_SEL45_W::new(self)
    }
    #[doc = "Bits 19:20 - N/A"]
    #[inline(always)]
    pub fn vref_sel45(&mut self) -> VREF_SEL45_W<19> {
        VREF_SEL45_W::new(self)
    }
    #[doc = "Bits 21:23 - N/A"]
    #[inline(always)]
    pub fn voh_sel45(&mut self) -> VOH_SEL45_W<21> {
        VOH_SEL45_W::new(self)
    }
    #[doc = "Bit 24 - N/A"]
    #[inline(always)]
    pub fn vreg_en67(&mut self) -> VREG_EN67_W<24> {
        VREG_EN67_W::new(self)
    }
    #[doc = "Bit 25 - N/A"]
    #[inline(always)]
    pub fn ibuf_sel67(&mut self) -> IBUF_SEL67_W<25> {
        IBUF_SEL67_W::new(self)
    }
    #[doc = "Bit 26 - N/A"]
    #[inline(always)]
    pub fn vtrip_sel67(&mut self) -> VTRIP_SEL67_W<26> {
        VTRIP_SEL67_W::new(self)
    }
    #[doc = "Bits 27:28 - N/A"]
    #[inline(always)]
    pub fn vref_sel67(&mut self) -> VREF_SEL67_W<27> {
        VREF_SEL67_W::new(self)
    }
    #[doc = "Bits 29:31 - N/A"]
    #[inline(always)]
    pub fn voh_sel67(&mut self) -> VOH_SEL67_W<29> {
        VOH_SEL67_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port SIO configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg_sio](index.html) module"]
pub struct CFG_SIO_SPEC;
impl crate::RegisterSpec for CFG_SIO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg_sio::R](R) reader structure"]
impl crate::Readable for CFG_SIO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg_sio::W](W) writer structure"]
impl crate::Writable for CFG_SIO_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFG_SIO to value 0"]
impl crate::Resettable for CFG_SIO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
