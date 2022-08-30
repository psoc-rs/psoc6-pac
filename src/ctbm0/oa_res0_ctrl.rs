#[doc = "Register `OA_RES0_CTRL` reader"]
pub struct R(crate::R<OA_RES0_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OA_RES0_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OA_RES0_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OA_RES0_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OA_RES0_CTRL` writer"]
pub struct W(crate::W<OA_RES0_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OA_RES0_CTRL_SPEC>;
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
impl From<crate::W<OA_RES0_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OA_RES0_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Opamp0 power level, assumes Cload=15pF for the (internal only) 1x driver or 50pF for the (external) 10x driver\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OA0_PWR_MODE_A {
    #[doc = "0: Off"]
    OFF = 0,
    #[doc = "1: Low power mode (IDD: 350uA, GBW: 1MHz for both 1x/10x)"]
    LOW = 1,
    #[doc = "2: Medium power mode (IDD: 600uA, GBW: 3MHz for 1x & 2.5MHz for 10x)"]
    MEDIUM = 2,
    #[doc = "3: High power mode for highest GBW (IDD: 1500uA, GBW: 8MHz for 1x & 6MHz for 10x)"]
    HIGH = 3,
    #[doc = "4: N/A"]
    RSVD = 4,
    #[doc = "5: Power Saver Low power mode (IDD: ~20uA with 1uA bias from AREF, GBW: ~100kHz for 1x/10x, offset correcting IDAC is disabled)"]
    PS_LOW = 5,
    #[doc = "6: Power Saver Medium power mode (IDD: ~40uA with 1uA bias from AREF, GBW: ~100kHz for 1x/10x, offset correcting IDAC is enabled)"]
    PS_MEDIUM = 6,
    #[doc = "7: Power Saver Medium power mode (IDD: ~60uA with 1uA bias from AREF, GBW: ~200kHz for 1x/10x, offset correcting IDAC is enabled)"]
    PS_HIGH = 7,
}
impl From<OA0_PWR_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: OA0_PWR_MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `OA0_PWR_MODE` reader - Opamp0 power level, assumes Cload=15pF for the (internal only) 1x driver or 50pF for the (external) 10x driver"]
pub type OA0_PWR_MODE_R = crate::FieldReader<u8, OA0_PWR_MODE_A>;
impl OA0_PWR_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OA0_PWR_MODE_A {
        match self.bits {
            0 => OA0_PWR_MODE_A::OFF,
            1 => OA0_PWR_MODE_A::LOW,
            2 => OA0_PWR_MODE_A::MEDIUM,
            3 => OA0_PWR_MODE_A::HIGH,
            4 => OA0_PWR_MODE_A::RSVD,
            5 => OA0_PWR_MODE_A::PS_LOW,
            6 => OA0_PWR_MODE_A::PS_MEDIUM,
            7 => OA0_PWR_MODE_A::PS_HIGH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == OA0_PWR_MODE_A::OFF
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == OA0_PWR_MODE_A::LOW
    }
    #[doc = "Checks if the value of the field is `MEDIUM`"]
    #[inline(always)]
    pub fn is_medium(&self) -> bool {
        *self == OA0_PWR_MODE_A::MEDIUM
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == OA0_PWR_MODE_A::HIGH
    }
    #[doc = "Checks if the value of the field is `RSVD`"]
    #[inline(always)]
    pub fn is_rsvd(&self) -> bool {
        *self == OA0_PWR_MODE_A::RSVD
    }
    #[doc = "Checks if the value of the field is `PS_LOW`"]
    #[inline(always)]
    pub fn is_ps_low(&self) -> bool {
        *self == OA0_PWR_MODE_A::PS_LOW
    }
    #[doc = "Checks if the value of the field is `PS_MEDIUM`"]
    #[inline(always)]
    pub fn is_ps_medium(&self) -> bool {
        *self == OA0_PWR_MODE_A::PS_MEDIUM
    }
    #[doc = "Checks if the value of the field is `PS_HIGH`"]
    #[inline(always)]
    pub fn is_ps_high(&self) -> bool {
        *self == OA0_PWR_MODE_A::PS_HIGH
    }
}
#[doc = "Field `OA0_PWR_MODE` writer - Opamp0 power level, assumes Cload=15pF for the (internal only) 1x driver or 50pF for the (external) 10x driver"]
pub type OA0_PWR_MODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, OA_RES0_CTRL_SPEC, u8, OA0_PWR_MODE_A, 3, O>;
impl<'a, const O: u8> OA0_PWR_MODE_W<'a, O> {
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(OA0_PWR_MODE_A::OFF)
    }
    #[doc = "Low power mode (IDD: 350uA, GBW: 1MHz for both 1x/10x)"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(OA0_PWR_MODE_A::LOW)
    }
    #[doc = "Medium power mode (IDD: 600uA, GBW: 3MHz for 1x & 2.5MHz for 10x)"]
    #[inline(always)]
    pub fn medium(self) -> &'a mut W {
        self.variant(OA0_PWR_MODE_A::MEDIUM)
    }
    #[doc = "High power mode for highest GBW (IDD: 1500uA, GBW: 8MHz for 1x & 6MHz for 10x)"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(OA0_PWR_MODE_A::HIGH)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn rsvd(self) -> &'a mut W {
        self.variant(OA0_PWR_MODE_A::RSVD)
    }
    #[doc = "Power Saver Low power mode (IDD: ~20uA with 1uA bias from AREF, GBW: ~100kHz for 1x/10x, offset correcting IDAC is disabled)"]
    #[inline(always)]
    pub fn ps_low(self) -> &'a mut W {
        self.variant(OA0_PWR_MODE_A::PS_LOW)
    }
    #[doc = "Power Saver Medium power mode (IDD: ~40uA with 1uA bias from AREF, GBW: ~100kHz for 1x/10x, offset correcting IDAC is enabled)"]
    #[inline(always)]
    pub fn ps_medium(self) -> &'a mut W {
        self.variant(OA0_PWR_MODE_A::PS_MEDIUM)
    }
    #[doc = "Power Saver Medium power mode (IDD: ~60uA with 1uA bias from AREF, GBW: ~200kHz for 1x/10x, offset correcting IDAC is enabled)"]
    #[inline(always)]
    pub fn ps_high(self) -> &'a mut W {
        self.variant(OA0_PWR_MODE_A::PS_HIGH)
    }
}
#[doc = "Field `OA0_DRIVE_STR_SEL` reader - Opamp0 output strength select 0=1x, 1=10x This setting sets specific requirements for OA0_BOOST_EN and OA0_COMP_TRIM"]
pub type OA0_DRIVE_STR_SEL_R = crate::BitReader<bool>;
#[doc = "Field `OA0_DRIVE_STR_SEL` writer - Opamp0 output strength select 0=1x, 1=10x This setting sets specific requirements for OA0_BOOST_EN and OA0_COMP_TRIM"]
pub type OA0_DRIVE_STR_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, OA_RES0_CTRL_SPEC, bool, O>;
#[doc = "Field `OA0_COMP_EN` reader - Opamp0 comparator enable"]
pub type OA0_COMP_EN_R = crate::BitReader<bool>;
#[doc = "Field `OA0_COMP_EN` writer - Opamp0 comparator enable"]
pub type OA0_COMP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OA_RES0_CTRL_SPEC, bool, O>;
#[doc = "Field `OA0_HYST_EN` reader - Opamp0 hysteresis enable (10mV)"]
pub type OA0_HYST_EN_R = crate::BitReader<bool>;
#[doc = "Field `OA0_HYST_EN` writer - Opamp0 hysteresis enable (10mV)"]
pub type OA0_HYST_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OA_RES0_CTRL_SPEC, bool, O>;
#[doc = "Field `OA0_BYPASS_DSI_SYNC` reader - Opamp0 bypass comparator output synchronization for DSI (trigger) output: 0=synchronize (level or pulse), 1=bypass (output async)"]
pub type OA0_BYPASS_DSI_SYNC_R = crate::BitReader<bool>;
#[doc = "Field `OA0_BYPASS_DSI_SYNC` writer - Opamp0 bypass comparator output synchronization for DSI (trigger) output: 0=synchronize (level or pulse), 1=bypass (output async)"]
pub type OA0_BYPASS_DSI_SYNC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, OA_RES0_CTRL_SPEC, bool, O>;
#[doc = "Field `OA0_DSI_LEVEL` reader - Opamp0 comparator DSI (trigger) out level : 0=pulse, each time an edge is detected (see OA0_COMPINT) a pulse is sent out on DSI 1=level, DSI output is a synchronized version of the comparator output"]
pub type OA0_DSI_LEVEL_R = crate::BitReader<bool>;
#[doc = "Field `OA0_DSI_LEVEL` writer - Opamp0 comparator DSI (trigger) out level : 0=pulse, each time an edge is detected (see OA0_COMPINT) a pulse is sent out on DSI 1=level, DSI output is a synchronized version of the comparator output"]
pub type OA0_DSI_LEVEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, OA_RES0_CTRL_SPEC, bool, O>;
#[doc = "Opamp0 comparator edge detect for interrupt and pulse mode of DSI (trigger)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OA0_COMPINT_A {
    #[doc = "0: Disabled, no interrupts will be detected"]
    DISABLE = 0,
    #[doc = "1: Rising edge"]
    RISING = 1,
    #[doc = "2: Falling edge"]
    FALLING = 2,
    #[doc = "3: Both rising and falling edges"]
    BOTH = 3,
}
impl From<OA0_COMPINT_A> for u8 {
    #[inline(always)]
    fn from(variant: OA0_COMPINT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `OA0_COMPINT` reader - Opamp0 comparator edge detect for interrupt and pulse mode of DSI (trigger)"]
pub type OA0_COMPINT_R = crate::FieldReader<u8, OA0_COMPINT_A>;
impl OA0_COMPINT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OA0_COMPINT_A {
        match self.bits {
            0 => OA0_COMPINT_A::DISABLE,
            1 => OA0_COMPINT_A::RISING,
            2 => OA0_COMPINT_A::FALLING,
            3 => OA0_COMPINT_A::BOTH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == OA0_COMPINT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == OA0_COMPINT_A::RISING
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == OA0_COMPINT_A::FALLING
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == OA0_COMPINT_A::BOTH
    }
}
#[doc = "Field `OA0_COMPINT` writer - Opamp0 comparator edge detect for interrupt and pulse mode of DSI (trigger)"]
pub type OA0_COMPINT_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, OA_RES0_CTRL_SPEC, u8, OA0_COMPINT_A, 2, O>;
impl<'a, const O: u8> OA0_COMPINT_W<'a, O> {
    #[doc = "Disabled, no interrupts will be detected"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(OA0_COMPINT_A::DISABLE)
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(OA0_COMPINT_A::RISING)
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(OA0_COMPINT_A::FALLING)
    }
    #[doc = "Both rising and falling edges"]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(OA0_COMPINT_A::BOTH)
    }
}
#[doc = "Field `OA0_PUMP_EN` reader - Opamp0 pump enable"]
pub type OA0_PUMP_EN_R = crate::BitReader<bool>;
#[doc = "Field `OA0_PUMP_EN` writer - Opamp0 pump enable"]
pub type OA0_PUMP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OA_RES0_CTRL_SPEC, bool, O>;
#[doc = "Field `OA0_BOOST_EN` reader - Opamp0 gain booster enable for class A output, for risk mitigation only, not user selectable. Value depends on the drive strength setting - 1x mode: set to 1; 10x mode: set to 0"]
pub type OA0_BOOST_EN_R = crate::BitReader<bool>;
#[doc = "Field `OA0_BOOST_EN` writer - Opamp0 gain booster enable for class A output, for risk mitigation only, not user selectable. Value depends on the drive strength setting - 1x mode: set to 1; 10x mode: set to 0"]
pub type OA0_BOOST_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OA_RES0_CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - Opamp0 power level, assumes Cload=15pF for the (internal only) 1x driver or 50pF for the (external) 10x driver"]
    #[inline(always)]
    pub fn oa0_pwr_mode(&self) -> OA0_PWR_MODE_R {
        OA0_PWR_MODE_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Opamp0 output strength select 0=1x, 1=10x This setting sets specific requirements for OA0_BOOST_EN and OA0_COMP_TRIM"]
    #[inline(always)]
    pub fn oa0_drive_str_sel(&self) -> OA0_DRIVE_STR_SEL_R {
        OA0_DRIVE_STR_SEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Opamp0 comparator enable"]
    #[inline(always)]
    pub fn oa0_comp_en(&self) -> OA0_COMP_EN_R {
        OA0_COMP_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Opamp0 hysteresis enable (10mV)"]
    #[inline(always)]
    pub fn oa0_hyst_en(&self) -> OA0_HYST_EN_R {
        OA0_HYST_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Opamp0 bypass comparator output synchronization for DSI (trigger) output: 0=synchronize (level or pulse), 1=bypass (output async)"]
    #[inline(always)]
    pub fn oa0_bypass_dsi_sync(&self) -> OA0_BYPASS_DSI_SYNC_R {
        OA0_BYPASS_DSI_SYNC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Opamp0 comparator DSI (trigger) out level : 0=pulse, each time an edge is detected (see OA0_COMPINT) a pulse is sent out on DSI 1=level, DSI output is a synchronized version of the comparator output"]
    #[inline(always)]
    pub fn oa0_dsi_level(&self) -> OA0_DSI_LEVEL_R {
        OA0_DSI_LEVEL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Opamp0 comparator edge detect for interrupt and pulse mode of DSI (trigger)"]
    #[inline(always)]
    pub fn oa0_compint(&self) -> OA0_COMPINT_R {
        OA0_COMPINT_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 11 - Opamp0 pump enable"]
    #[inline(always)]
    pub fn oa0_pump_en(&self) -> OA0_PUMP_EN_R {
        OA0_PUMP_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Opamp0 gain booster enable for class A output, for risk mitigation only, not user selectable. Value depends on the drive strength setting - 1x mode: set to 1; 10x mode: set to 0"]
    #[inline(always)]
    pub fn oa0_boost_en(&self) -> OA0_BOOST_EN_R {
        OA0_BOOST_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Opamp0 power level, assumes Cload=15pF for the (internal only) 1x driver or 50pF for the (external) 10x driver"]
    #[inline(always)]
    pub fn oa0_pwr_mode(&mut self) -> OA0_PWR_MODE_W<0> {
        OA0_PWR_MODE_W::new(self)
    }
    #[doc = "Bit 3 - Opamp0 output strength select 0=1x, 1=10x This setting sets specific requirements for OA0_BOOST_EN and OA0_COMP_TRIM"]
    #[inline(always)]
    pub fn oa0_drive_str_sel(&mut self) -> OA0_DRIVE_STR_SEL_W<3> {
        OA0_DRIVE_STR_SEL_W::new(self)
    }
    #[doc = "Bit 4 - Opamp0 comparator enable"]
    #[inline(always)]
    pub fn oa0_comp_en(&mut self) -> OA0_COMP_EN_W<4> {
        OA0_COMP_EN_W::new(self)
    }
    #[doc = "Bit 5 - Opamp0 hysteresis enable (10mV)"]
    #[inline(always)]
    pub fn oa0_hyst_en(&mut self) -> OA0_HYST_EN_W<5> {
        OA0_HYST_EN_W::new(self)
    }
    #[doc = "Bit 6 - Opamp0 bypass comparator output synchronization for DSI (trigger) output: 0=synchronize (level or pulse), 1=bypass (output async)"]
    #[inline(always)]
    pub fn oa0_bypass_dsi_sync(&mut self) -> OA0_BYPASS_DSI_SYNC_W<6> {
        OA0_BYPASS_DSI_SYNC_W::new(self)
    }
    #[doc = "Bit 7 - Opamp0 comparator DSI (trigger) out level : 0=pulse, each time an edge is detected (see OA0_COMPINT) a pulse is sent out on DSI 1=level, DSI output is a synchronized version of the comparator output"]
    #[inline(always)]
    pub fn oa0_dsi_level(&mut self) -> OA0_DSI_LEVEL_W<7> {
        OA0_DSI_LEVEL_W::new(self)
    }
    #[doc = "Bits 8:9 - Opamp0 comparator edge detect for interrupt and pulse mode of DSI (trigger)"]
    #[inline(always)]
    pub fn oa0_compint(&mut self) -> OA0_COMPINT_W<8> {
        OA0_COMPINT_W::new(self)
    }
    #[doc = "Bit 11 - Opamp0 pump enable"]
    #[inline(always)]
    pub fn oa0_pump_en(&mut self) -> OA0_PUMP_EN_W<11> {
        OA0_PUMP_EN_W::new(self)
    }
    #[doc = "Bit 12 - Opamp0 gain booster enable for class A output, for risk mitigation only, not user selectable. Value depends on the drive strength setting - 1x mode: set to 1; 10x mode: set to 0"]
    #[inline(always)]
    pub fn oa0_boost_en(&mut self) -> OA0_BOOST_EN_W<12> {
        OA0_BOOST_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Opamp0 and resistor0 control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oa_res0_ctrl](index.html) module"]
pub struct OA_RES0_CTRL_SPEC;
impl crate::RegisterSpec for OA_RES0_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [oa_res0_ctrl::R](R) reader structure"]
impl crate::Readable for OA_RES0_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [oa_res0_ctrl::W](W) writer structure"]
impl crate::Writable for OA_RES0_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OA_RES0_CTRL to value 0"]
impl crate::Resettable for OA_RES0_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
