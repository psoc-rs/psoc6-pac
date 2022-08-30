#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_CLK_EDGE` reader - N/A"]
pub type TX_CLK_EDGE_R = crate::BitReader<bool>;
#[doc = "Field `TX_CLK_EDGE` writer - N/A"]
pub type TX_CLK_EDGE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `RX_CLK_EDGE` reader - N/A"]
pub type RX_CLK_EDGE_R = crate::BitReader<bool>;
#[doc = "Field `RX_CLK_EDGE` writer - N/A"]
pub type RX_CLK_EDGE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `RX_CLK_SRC` reader - N/A"]
pub type RX_CLK_SRC_R = crate::BitReader<bool>;
#[doc = "Field `RX_CLK_SRC` writer - N/A"]
pub type RX_CLK_SRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `SCLK_CONTINUOUS` reader - N/A"]
pub type SCLK_CONTINUOUS_R = crate::BitReader<bool>;
#[doc = "Field `SCLK_CONTINUOUS` writer - N/A"]
pub type SCLK_CONTINUOUS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `SSEL_POLARITY` reader - N/A"]
pub type SSEL_POLARITY_R = crate::BitReader<bool>;
#[doc = "Field `SSEL_POLARITY` writer - N/A"]
pub type SSEL_POLARITY_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `LEAD` reader - N/A"]
pub type LEAD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LEAD` writer - N/A"]
pub type LEAD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `LAG` reader - N/A"]
pub type LAG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LAG` writer - N/A"]
pub type LAG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `DIV_ENABLED` reader - N/A"]
pub type DIV_ENABLED_R = crate::BitReader<bool>;
#[doc = "Field `DIV_ENABLED` writer - N/A"]
pub type DIV_ENABLED_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `DIV` reader - N/A"]
pub type DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DIV` writer - N/A"]
pub type DIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 6, O>;
#[doc = "Field `ADDR_WIDTH` reader - N/A"]
pub type ADDR_WIDTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADDR_WIDTH` writer - N/A"]
pub type ADDR_WIDTH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `DATA_WIDTH` reader - N/A"]
pub type DATA_WIDTH_R = crate::BitReader<bool>;
#[doc = "Field `DATA_WIDTH` writer - N/A"]
pub type DATA_WIDTH_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `ENABLED` reader - N/A"]
pub type ENABLED_R = crate::BitReader<bool>;
#[doc = "Field `ENABLED` writer - N/A"]
pub type ENABLED_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - N/A"]
    #[inline(always)]
    pub fn tx_clk_edge(&self) -> TX_CLK_EDGE_R {
        TX_CLK_EDGE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - N/A"]
    #[inline(always)]
    pub fn rx_clk_edge(&self) -> RX_CLK_EDGE_R {
        RX_CLK_EDGE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - N/A"]
    #[inline(always)]
    pub fn rx_clk_src(&self) -> RX_CLK_SRC_R {
        RX_CLK_SRC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - N/A"]
    #[inline(always)]
    pub fn sclk_continuous(&self) -> SCLK_CONTINUOUS_R {
        SCLK_CONTINUOUS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - N/A"]
    #[inline(always)]
    pub fn ssel_polarity(&self) -> SSEL_POLARITY_R {
        SSEL_POLARITY_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:9 - N/A"]
    #[inline(always)]
    pub fn lead(&self) -> LEAD_R {
        LEAD_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - N/A"]
    #[inline(always)]
    pub fn lag(&self) -> LAG_R {
        LAG_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - N/A"]
    #[inline(always)]
    pub fn div_enabled(&self) -> DIV_ENABLED_R {
        DIV_ENABLED_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:18 - N/A"]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new(((self.bits >> 13) & 0x3f) as u8)
    }
    #[doc = "Bits 19:22 - N/A"]
    #[inline(always)]
    pub fn addr_width(&self) -> ADDR_WIDTH_R {
        ADDR_WIDTH_R::new(((self.bits >> 19) & 0x0f) as u8)
    }
    #[doc = "Bit 23 - N/A"]
    #[inline(always)]
    pub fn data_width(&self) -> DATA_WIDTH_R {
        DATA_WIDTH_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 31 - N/A"]
    #[inline(always)]
    pub fn enabled(&self) -> ENABLED_R {
        ENABLED_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - N/A"]
    #[inline(always)]
    pub fn tx_clk_edge(&mut self) -> TX_CLK_EDGE_W<1> {
        TX_CLK_EDGE_W::new(self)
    }
    #[doc = "Bit 2 - N/A"]
    #[inline(always)]
    pub fn rx_clk_edge(&mut self) -> RX_CLK_EDGE_W<2> {
        RX_CLK_EDGE_W::new(self)
    }
    #[doc = "Bit 3 - N/A"]
    #[inline(always)]
    pub fn rx_clk_src(&mut self) -> RX_CLK_SRC_W<3> {
        RX_CLK_SRC_W::new(self)
    }
    #[doc = "Bit 4 - N/A"]
    #[inline(always)]
    pub fn sclk_continuous(&mut self) -> SCLK_CONTINUOUS_W<4> {
        SCLK_CONTINUOUS_W::new(self)
    }
    #[doc = "Bit 5 - N/A"]
    #[inline(always)]
    pub fn ssel_polarity(&mut self) -> SSEL_POLARITY_W<5> {
        SSEL_POLARITY_W::new(self)
    }
    #[doc = "Bits 8:9 - N/A"]
    #[inline(always)]
    pub fn lead(&mut self) -> LEAD_W<8> {
        LEAD_W::new(self)
    }
    #[doc = "Bits 10:11 - N/A"]
    #[inline(always)]
    pub fn lag(&mut self) -> LAG_W<10> {
        LAG_W::new(self)
    }
    #[doc = "Bit 12 - N/A"]
    #[inline(always)]
    pub fn div_enabled(&mut self) -> DIV_ENABLED_W<12> {
        DIV_ENABLED_W::new(self)
    }
    #[doc = "Bits 13:18 - N/A"]
    #[inline(always)]
    pub fn div(&mut self) -> DIV_W<13> {
        DIV_W::new(self)
    }
    #[doc = "Bits 19:22 - N/A"]
    #[inline(always)]
    pub fn addr_width(&mut self) -> ADDR_WIDTH_W<19> {
        ADDR_WIDTH_W::new(self)
    }
    #[doc = "Bit 23 - N/A"]
    #[inline(always)]
    pub fn data_width(&mut self) -> DATA_WIDTH_W<23> {
        DATA_WIDTH_W::new(self)
    }
    #[doc = "Bit 31 - N/A"]
    #[inline(always)]
    pub fn enabled(&mut self) -> ENABLED_W<31> {
        ENABLED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCB control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL to value 0x00f8_0000"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x00f8_0000
    }
}
