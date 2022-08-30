#[doc = "Register `INJ_CHAN_CONFIG` reader"]
pub struct R(crate::R<INJ_CHAN_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INJ_CHAN_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INJ_CHAN_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INJ_CHAN_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INJ_CHAN_CONFIG` writer"]
pub struct W(crate::W<INJ_CHAN_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INJ_CHAN_CONFIG_SPEC>;
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
impl From<crate::W<INJ_CHAN_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INJ_CHAN_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INJ_PIN_ADDR` reader - Address of the pin to be sampled by this injection channel. If differential is enabled then INJ_PIN_ADDR\\[0\\]
is ignored and considered to be 0, i.e. INJ_PIN_ADDR points to the even pin of a pin pair."]
pub type INJ_PIN_ADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INJ_PIN_ADDR` writer - Address of the pin to be sampled by this injection channel. If differential is enabled then INJ_PIN_ADDR\\[0\\]
is ignored and considered to be 0, i.e. INJ_PIN_ADDR points to the even pin of a pin pair."]
pub type INJ_PIN_ADDR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, INJ_CHAN_CONFIG_SPEC, u8, u8, 3, O>;
#[doc = "Address of the port that contains the pin to be sampled by this channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum INJ_PORT_ADDR_A {
    #[doc = "0: SARMUX pins."]
    SARMUX = 0,
    #[doc = "1: CTB0"]
    CTB0 = 1,
    #[doc = "2: CTB1"]
    CTB1 = 2,
    #[doc = "3: CTB2"]
    CTB2 = 3,
    #[doc = "4: CTB3"]
    CTB3 = 4,
    #[doc = "6: AROUTE virtual port"]
    AROUTE_VIRT = 6,
    #[doc = "7: SARMUX virtual port"]
    SARMUX_VIRT = 7,
}
impl From<INJ_PORT_ADDR_A> for u8 {
    #[inline(always)]
    fn from(variant: INJ_PORT_ADDR_A) -> Self {
        variant as _
    }
}
#[doc = "Field `INJ_PORT_ADDR` reader - Address of the port that contains the pin to be sampled by this channel."]
pub type INJ_PORT_ADDR_R = crate::FieldReader<u8, INJ_PORT_ADDR_A>;
impl INJ_PORT_ADDR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<INJ_PORT_ADDR_A> {
        match self.bits {
            0 => Some(INJ_PORT_ADDR_A::SARMUX),
            1 => Some(INJ_PORT_ADDR_A::CTB0),
            2 => Some(INJ_PORT_ADDR_A::CTB1),
            3 => Some(INJ_PORT_ADDR_A::CTB2),
            4 => Some(INJ_PORT_ADDR_A::CTB3),
            6 => Some(INJ_PORT_ADDR_A::AROUTE_VIRT),
            7 => Some(INJ_PORT_ADDR_A::SARMUX_VIRT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SARMUX`"]
    #[inline(always)]
    pub fn is_sarmux(&self) -> bool {
        *self == INJ_PORT_ADDR_A::SARMUX
    }
    #[doc = "Checks if the value of the field is `CTB0`"]
    #[inline(always)]
    pub fn is_ctb0(&self) -> bool {
        *self == INJ_PORT_ADDR_A::CTB0
    }
    #[doc = "Checks if the value of the field is `CTB1`"]
    #[inline(always)]
    pub fn is_ctb1(&self) -> bool {
        *self == INJ_PORT_ADDR_A::CTB1
    }
    #[doc = "Checks if the value of the field is `CTB2`"]
    #[inline(always)]
    pub fn is_ctb2(&self) -> bool {
        *self == INJ_PORT_ADDR_A::CTB2
    }
    #[doc = "Checks if the value of the field is `CTB3`"]
    #[inline(always)]
    pub fn is_ctb3(&self) -> bool {
        *self == INJ_PORT_ADDR_A::CTB3
    }
    #[doc = "Checks if the value of the field is `AROUTE_VIRT`"]
    #[inline(always)]
    pub fn is_aroute_virt(&self) -> bool {
        *self == INJ_PORT_ADDR_A::AROUTE_VIRT
    }
    #[doc = "Checks if the value of the field is `SARMUX_VIRT`"]
    #[inline(always)]
    pub fn is_sarmux_virt(&self) -> bool {
        *self == INJ_PORT_ADDR_A::SARMUX_VIRT
    }
}
#[doc = "Field `INJ_PORT_ADDR` writer - Address of the port that contains the pin to be sampled by this channel."]
pub type INJ_PORT_ADDR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, INJ_CHAN_CONFIG_SPEC, u8, INJ_PORT_ADDR_A, 3, O>;
impl<'a, const O: u8> INJ_PORT_ADDR_W<'a, O> {
    #[doc = "SARMUX pins."]
    #[inline(always)]
    pub fn sarmux(self) -> &'a mut W {
        self.variant(INJ_PORT_ADDR_A::SARMUX)
    }
    #[doc = "CTB0"]
    #[inline(always)]
    pub fn ctb0(self) -> &'a mut W {
        self.variant(INJ_PORT_ADDR_A::CTB0)
    }
    #[doc = "CTB1"]
    #[inline(always)]
    pub fn ctb1(self) -> &'a mut W {
        self.variant(INJ_PORT_ADDR_A::CTB1)
    }
    #[doc = "CTB2"]
    #[inline(always)]
    pub fn ctb2(self) -> &'a mut W {
        self.variant(INJ_PORT_ADDR_A::CTB2)
    }
    #[doc = "CTB3"]
    #[inline(always)]
    pub fn ctb3(self) -> &'a mut W {
        self.variant(INJ_PORT_ADDR_A::CTB3)
    }
    #[doc = "AROUTE virtual port"]
    #[inline(always)]
    pub fn aroute_virt(self) -> &'a mut W {
        self.variant(INJ_PORT_ADDR_A::AROUTE_VIRT)
    }
    #[doc = "SARMUX virtual port"]
    #[inline(always)]
    pub fn sarmux_virt(self) -> &'a mut W {
        self.variant(INJ_PORT_ADDR_A::SARMUX_VIRT)
    }
}
#[doc = "Field `INJ_DIFFERENTIAL_EN` reader - Differential enable for this channel. - 0: The voltage on the addressed pin is measured (Single-ended) and the resulting value is stored in the corresponding data register. - 1: The differential voltage on the addressed pin pair is measured and the resulting value is stored in the corresponding data register. (INJ_PIN_ADDR\\[0\\]
is ignored)."]
pub type INJ_DIFFERENTIAL_EN_R = crate::BitReader<bool>;
#[doc = "Field `INJ_DIFFERENTIAL_EN` writer - Differential enable for this channel. - 0: The voltage on the addressed pin is measured (Single-ended) and the resulting value is stored in the corresponding data register. - 1: The differential voltage on the addressed pin pair is measured and the resulting value is stored in the corresponding data register. (INJ_PIN_ADDR\\[0\\]
is ignored)."]
pub type INJ_DIFFERENTIAL_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INJ_CHAN_CONFIG_SPEC, bool, O>;
#[doc = "Field `INJ_AVG_EN` reader - Averaging enable for this channel. If set the AVG_CNT and AVG_SHIFT settings are used for sampling the addressed pin(s)"]
pub type INJ_AVG_EN_R = crate::BitReader<bool>;
#[doc = "Field `INJ_AVG_EN` writer - Averaging enable for this channel. If set the AVG_CNT and AVG_SHIFT settings are used for sampling the addressed pin(s)"]
pub type INJ_AVG_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INJ_CHAN_CONFIG_SPEC, bool, O>;
#[doc = "Field `INJ_SAMPLE_TIME_SEL` reader - Injection sample time select: select which of the 4 global sample times to use for this channel"]
pub type INJ_SAMPLE_TIME_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INJ_SAMPLE_TIME_SEL` writer - Injection sample time select: select which of the 4 global sample times to use for this channel"]
pub type INJ_SAMPLE_TIME_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, INJ_CHAN_CONFIG_SPEC, u8, u8, 2, O>;
#[doc = "Field `INJ_TAILGATING` reader - Injection channel tailgating. - 0: no tailgating for this channel, SAR is immediately triggered when the INJ_START_EN bit is set. - 1: injection channel tailgating. The addressed pin is sampled after the next trigger and after all enabled channels have been scanned."]
pub type INJ_TAILGATING_R = crate::BitReader<bool>;
#[doc = "Field `INJ_TAILGATING` writer - Injection channel tailgating. - 0: no tailgating for this channel, SAR is immediately triggered when the INJ_START_EN bit is set. - 1: injection channel tailgating. The addressed pin is sampled after the next trigger and after all enabled channels have been scanned."]
pub type INJ_TAILGATING_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INJ_CHAN_CONFIG_SPEC, bool, O>;
#[doc = "Field `INJ_START_EN` reader - Set by firmware to enable the injection channel. If INJ_TAILGATING is not set this bit also functions as trigger for this channel. Cleared by hardware after this channel has been sampled (i.e. this channel is always one shot even if CONTINUOUS is set). Also cleared if the SAR is disabled."]
pub type INJ_START_EN_R = crate::BitReader<bool>;
#[doc = "Field `INJ_START_EN` writer - Set by firmware to enable the injection channel. If INJ_TAILGATING is not set this bit also functions as trigger for this channel. Cleared by hardware after this channel has been sampled (i.e. this channel is always one shot even if CONTINUOUS is set). Also cleared if the SAR is disabled."]
pub type INJ_START_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INJ_CHAN_CONFIG_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - Address of the pin to be sampled by this injection channel. If differential is enabled then INJ_PIN_ADDR\\[0\\]
is ignored and considered to be 0, i.e. INJ_PIN_ADDR points to the even pin of a pin pair."]
    #[inline(always)]
    pub fn inj_pin_addr(&self) -> INJ_PIN_ADDR_R {
        INJ_PIN_ADDR_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Address of the port that contains the pin to be sampled by this channel."]
    #[inline(always)]
    pub fn inj_port_addr(&self) -> INJ_PORT_ADDR_R {
        INJ_PORT_ADDR_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 8 - Differential enable for this channel. - 0: The voltage on the addressed pin is measured (Single-ended) and the resulting value is stored in the corresponding data register. - 1: The differential voltage on the addressed pin pair is measured and the resulting value is stored in the corresponding data register. (INJ_PIN_ADDR\\[0\\]
is ignored)."]
    #[inline(always)]
    pub fn inj_differential_en(&self) -> INJ_DIFFERENTIAL_EN_R {
        INJ_DIFFERENTIAL_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Averaging enable for this channel. If set the AVG_CNT and AVG_SHIFT settings are used for sampling the addressed pin(s)"]
    #[inline(always)]
    pub fn inj_avg_en(&self) -> INJ_AVG_EN_R {
        INJ_AVG_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Injection sample time select: select which of the 4 global sample times to use for this channel"]
    #[inline(always)]
    pub fn inj_sample_time_sel(&self) -> INJ_SAMPLE_TIME_SEL_R {
        INJ_SAMPLE_TIME_SEL_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 30 - Injection channel tailgating. - 0: no tailgating for this channel, SAR is immediately triggered when the INJ_START_EN bit is set. - 1: injection channel tailgating. The addressed pin is sampled after the next trigger and after all enabled channels have been scanned."]
    #[inline(always)]
    pub fn inj_tailgating(&self) -> INJ_TAILGATING_R {
        INJ_TAILGATING_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Set by firmware to enable the injection channel. If INJ_TAILGATING is not set this bit also functions as trigger for this channel. Cleared by hardware after this channel has been sampled (i.e. this channel is always one shot even if CONTINUOUS is set). Also cleared if the SAR is disabled."]
    #[inline(always)]
    pub fn inj_start_en(&self) -> INJ_START_EN_R {
        INJ_START_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Address of the pin to be sampled by this injection channel. If differential is enabled then INJ_PIN_ADDR\\[0\\]
is ignored and considered to be 0, i.e. INJ_PIN_ADDR points to the even pin of a pin pair."]
    #[inline(always)]
    pub fn inj_pin_addr(&mut self) -> INJ_PIN_ADDR_W<0> {
        INJ_PIN_ADDR_W::new(self)
    }
    #[doc = "Bits 4:6 - Address of the port that contains the pin to be sampled by this channel."]
    #[inline(always)]
    pub fn inj_port_addr(&mut self) -> INJ_PORT_ADDR_W<4> {
        INJ_PORT_ADDR_W::new(self)
    }
    #[doc = "Bit 8 - Differential enable for this channel. - 0: The voltage on the addressed pin is measured (Single-ended) and the resulting value is stored in the corresponding data register. - 1: The differential voltage on the addressed pin pair is measured and the resulting value is stored in the corresponding data register. (INJ_PIN_ADDR\\[0\\]
is ignored)."]
    #[inline(always)]
    pub fn inj_differential_en(&mut self) -> INJ_DIFFERENTIAL_EN_W<8> {
        INJ_DIFFERENTIAL_EN_W::new(self)
    }
    #[doc = "Bit 10 - Averaging enable for this channel. If set the AVG_CNT and AVG_SHIFT settings are used for sampling the addressed pin(s)"]
    #[inline(always)]
    pub fn inj_avg_en(&mut self) -> INJ_AVG_EN_W<10> {
        INJ_AVG_EN_W::new(self)
    }
    #[doc = "Bits 12:13 - Injection sample time select: select which of the 4 global sample times to use for this channel"]
    #[inline(always)]
    pub fn inj_sample_time_sel(&mut self) -> INJ_SAMPLE_TIME_SEL_W<12> {
        INJ_SAMPLE_TIME_SEL_W::new(self)
    }
    #[doc = "Bit 30 - Injection channel tailgating. - 0: no tailgating for this channel, SAR is immediately triggered when the INJ_START_EN bit is set. - 1: injection channel tailgating. The addressed pin is sampled after the next trigger and after all enabled channels have been scanned."]
    #[inline(always)]
    pub fn inj_tailgating(&mut self) -> INJ_TAILGATING_W<30> {
        INJ_TAILGATING_W::new(self)
    }
    #[doc = "Bit 31 - Set by firmware to enable the injection channel. If INJ_TAILGATING is not set this bit also functions as trigger for this channel. Cleared by hardware after this channel has been sampled (i.e. this channel is always one shot even if CONTINUOUS is set). Also cleared if the SAR is disabled."]
    #[inline(always)]
    pub fn inj_start_en(&mut self) -> INJ_START_EN_W<31> {
        INJ_START_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Injection channel configuration register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inj_chan_config](index.html) module"]
pub struct INJ_CHAN_CONFIG_SPEC;
impl crate::RegisterSpec for INJ_CHAN_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inj_chan_config::R](R) reader structure"]
impl crate::Readable for INJ_CHAN_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [inj_chan_config::W](W) writer structure"]
impl crate::Writable for INJ_CHAN_CONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INJ_CHAN_CONFIG to value 0"]
impl crate::Resettable for INJ_CHAN_CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
