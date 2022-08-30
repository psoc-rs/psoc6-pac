#[doc = "Register `IDACA` reader"]
pub struct R(crate::R<IDACA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IDACA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IDACA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IDACA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IDACA` writer"]
pub struct W(crate::W<IDACA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IDACA_SPEC>;
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
impl From<crate::W<IDACA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IDACA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VAL` reader - N/A"]
pub type VAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VAL` writer - N/A"]
pub type VAL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IDACA_SPEC, u8, u8, 7, O>;
#[doc = "N/A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POL_DYN_A {
    #[doc = "0: N/A"]
    STATIC = 0,
    #[doc = "1: N/A"]
    DYNAMIC = 1,
}
impl From<POL_DYN_A> for bool {
    #[inline(always)]
    fn from(variant: POL_DYN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POL_DYN` reader - N/A"]
pub type POL_DYN_R = crate::BitReader<POL_DYN_A>;
impl POL_DYN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POL_DYN_A {
        match self.bits {
            false => POL_DYN_A::STATIC,
            true => POL_DYN_A::DYNAMIC,
        }
    }
    #[doc = "Checks if the value of the field is `STATIC`"]
    #[inline(always)]
    pub fn is_static(&self) -> bool {
        *self == POL_DYN_A::STATIC
    }
    #[doc = "Checks if the value of the field is `DYNAMIC`"]
    #[inline(always)]
    pub fn is_dynamic(&self) -> bool {
        *self == POL_DYN_A::DYNAMIC
    }
}
#[doc = "Field `POL_DYN` writer - N/A"]
pub type POL_DYN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDACA_SPEC, POL_DYN_A, O>;
impl<'a, const O: u8> POL_DYN_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn static_(self) -> &'a mut W {
        self.variant(POL_DYN_A::STATIC)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn dynamic(self) -> &'a mut W {
        self.variant(POL_DYN_A::DYNAMIC)
    }
}
#[doc = "N/A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum POLARITY_A {
    #[doc = "0: Normal: sensor switching between Vssio and Cmod. For non-CSD application, IDAC1 will source current."]
    VSSA_SRC = 0,
    #[doc = "1: Inverted: sensor switch between Vddio and Cmod. For non-CSD application, IDAC1 will sink current."]
    VDDA_SNK = 1,
    #[doc = "2: N/A"]
    SENSE = 2,
    #[doc = "3: N/A"]
    SENSE_INV = 3,
}
impl From<POLARITY_A> for u8 {
    #[inline(always)]
    fn from(variant: POLARITY_A) -> Self {
        variant as _
    }
}
#[doc = "Field `POLARITY` reader - N/A"]
pub type POLARITY_R = crate::FieldReader<u8, POLARITY_A>;
impl POLARITY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POLARITY_A {
        match self.bits {
            0 => POLARITY_A::VSSA_SRC,
            1 => POLARITY_A::VDDA_SNK,
            2 => POLARITY_A::SENSE,
            3 => POLARITY_A::SENSE_INV,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VSSA_SRC`"]
    #[inline(always)]
    pub fn is_vssa_src(&self) -> bool {
        *self == POLARITY_A::VSSA_SRC
    }
    #[doc = "Checks if the value of the field is `VDDA_SNK`"]
    #[inline(always)]
    pub fn is_vdda_snk(&self) -> bool {
        *self == POLARITY_A::VDDA_SNK
    }
    #[doc = "Checks if the value of the field is `SENSE`"]
    #[inline(always)]
    pub fn is_sense(&self) -> bool {
        *self == POLARITY_A::SENSE
    }
    #[doc = "Checks if the value of the field is `SENSE_INV`"]
    #[inline(always)]
    pub fn is_sense_inv(&self) -> bool {
        *self == POLARITY_A::SENSE_INV
    }
}
#[doc = "Field `POLARITY` writer - N/A"]
pub type POLARITY_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, IDACA_SPEC, u8, POLARITY_A, 2, O>;
impl<'a, const O: u8> POLARITY_W<'a, O> {
    #[doc = "Normal: sensor switching between Vssio and Cmod. For non-CSD application, IDAC1 will source current."]
    #[inline(always)]
    pub fn vssa_src(self) -> &'a mut W {
        self.variant(POLARITY_A::VSSA_SRC)
    }
    #[doc = "Inverted: sensor switch between Vddio and Cmod. For non-CSD application, IDAC1 will sink current."]
    #[inline(always)]
    pub fn vdda_snk(self) -> &'a mut W {
        self.variant(POLARITY_A::VDDA_SNK)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn sense(self) -> &'a mut W {
        self.variant(POLARITY_A::SENSE)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn sense_inv(self) -> &'a mut W {
        self.variant(POLARITY_A::SENSE_INV)
    }
}
#[doc = "N/A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BAL_MODE_A {
    #[doc = "0: N/A"]
    FULL = 0,
    #[doc = "1: N/A"]
    PHI1 = 1,
    #[doc = "2: N/A"]
    PHI2 = 2,
    #[doc = "3: N/A"]
    PHI1_2 = 3,
}
impl From<BAL_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: BAL_MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `BAL_MODE` reader - N/A"]
pub type BAL_MODE_R = crate::FieldReader<u8, BAL_MODE_A>;
impl BAL_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BAL_MODE_A {
        match self.bits {
            0 => BAL_MODE_A::FULL,
            1 => BAL_MODE_A::PHI1,
            2 => BAL_MODE_A::PHI2,
            3 => BAL_MODE_A::PHI1_2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FULL`"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == BAL_MODE_A::FULL
    }
    #[doc = "Checks if the value of the field is `PHI1`"]
    #[inline(always)]
    pub fn is_phi1(&self) -> bool {
        *self == BAL_MODE_A::PHI1
    }
    #[doc = "Checks if the value of the field is `PHI2`"]
    #[inline(always)]
    pub fn is_phi2(&self) -> bool {
        *self == BAL_MODE_A::PHI2
    }
    #[doc = "Checks if the value of the field is `PHI1_2`"]
    #[inline(always)]
    pub fn is_phi1_2(&self) -> bool {
        *self == BAL_MODE_A::PHI1_2
    }
}
#[doc = "Field `BAL_MODE` writer - N/A"]
pub type BAL_MODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, IDACA_SPEC, u8, BAL_MODE_A, 2, O>;
impl<'a, const O: u8> BAL_MODE_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn full(self) -> &'a mut W {
        self.variant(BAL_MODE_A::FULL)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn phi1(self) -> &'a mut W {
        self.variant(BAL_MODE_A::PHI1)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn phi2(self) -> &'a mut W {
        self.variant(BAL_MODE_A::PHI2)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn phi1_2(self) -> &'a mut W {
        self.variant(BAL_MODE_A::PHI1_2)
    }
}
#[doc = "N/A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LEG1_MODE_A {
    #[doc = "0: N/A"]
    GP_STATIC = 0,
    #[doc = "1: N/A"]
    GP = 1,
    #[doc = "2: N/A"]
    CSD_STATIC = 2,
    #[doc = "3: N/A"]
    CSD = 3,
}
impl From<LEG1_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: LEG1_MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `LEG1_MODE` reader - N/A"]
pub type LEG1_MODE_R = crate::FieldReader<u8, LEG1_MODE_A>;
impl LEG1_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LEG1_MODE_A {
        match self.bits {
            0 => LEG1_MODE_A::GP_STATIC,
            1 => LEG1_MODE_A::GP,
            2 => LEG1_MODE_A::CSD_STATIC,
            3 => LEG1_MODE_A::CSD,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GP_STATIC`"]
    #[inline(always)]
    pub fn is_gp_static(&self) -> bool {
        *self == LEG1_MODE_A::GP_STATIC
    }
    #[doc = "Checks if the value of the field is `GP`"]
    #[inline(always)]
    pub fn is_gp(&self) -> bool {
        *self == LEG1_MODE_A::GP
    }
    #[doc = "Checks if the value of the field is `CSD_STATIC`"]
    #[inline(always)]
    pub fn is_csd_static(&self) -> bool {
        *self == LEG1_MODE_A::CSD_STATIC
    }
    #[doc = "Checks if the value of the field is `CSD`"]
    #[inline(always)]
    pub fn is_csd(&self) -> bool {
        *self == LEG1_MODE_A::CSD
    }
}
#[doc = "Field `LEG1_MODE` writer - N/A"]
pub type LEG1_MODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, IDACA_SPEC, u8, LEG1_MODE_A, 2, O>;
impl<'a, const O: u8> LEG1_MODE_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn gp_static(self) -> &'a mut W {
        self.variant(LEG1_MODE_A::GP_STATIC)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn gp(self) -> &'a mut W {
        self.variant(LEG1_MODE_A::GP)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn csd_static(self) -> &'a mut W {
        self.variant(LEG1_MODE_A::CSD_STATIC)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn csd(self) -> &'a mut W {
        self.variant(LEG1_MODE_A::CSD)
    }
}
#[doc = "N/A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LEG2_MODE_A {
    #[doc = "0: N/A"]
    GP_STATIC = 0,
    #[doc = "1: N/A"]
    GP = 1,
    #[doc = "2: N/A"]
    CSD_STATIC = 2,
    #[doc = "3: N/A"]
    CSD = 3,
}
impl From<LEG2_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: LEG2_MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `LEG2_MODE` reader - N/A"]
pub type LEG2_MODE_R = crate::FieldReader<u8, LEG2_MODE_A>;
impl LEG2_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LEG2_MODE_A {
        match self.bits {
            0 => LEG2_MODE_A::GP_STATIC,
            1 => LEG2_MODE_A::GP,
            2 => LEG2_MODE_A::CSD_STATIC,
            3 => LEG2_MODE_A::CSD,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `GP_STATIC`"]
    #[inline(always)]
    pub fn is_gp_static(&self) -> bool {
        *self == LEG2_MODE_A::GP_STATIC
    }
    #[doc = "Checks if the value of the field is `GP`"]
    #[inline(always)]
    pub fn is_gp(&self) -> bool {
        *self == LEG2_MODE_A::GP
    }
    #[doc = "Checks if the value of the field is `CSD_STATIC`"]
    #[inline(always)]
    pub fn is_csd_static(&self) -> bool {
        *self == LEG2_MODE_A::CSD_STATIC
    }
    #[doc = "Checks if the value of the field is `CSD`"]
    #[inline(always)]
    pub fn is_csd(&self) -> bool {
        *self == LEG2_MODE_A::CSD
    }
}
#[doc = "Field `LEG2_MODE` writer - N/A"]
pub type LEG2_MODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, IDACA_SPEC, u8, LEG2_MODE_A, 2, O>;
impl<'a, const O: u8> LEG2_MODE_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn gp_static(self) -> &'a mut W {
        self.variant(LEG2_MODE_A::GP_STATIC)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn gp(self) -> &'a mut W {
        self.variant(LEG2_MODE_A::GP)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn csd_static(self) -> &'a mut W {
        self.variant(LEG2_MODE_A::CSD_STATIC)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn csd(self) -> &'a mut W {
        self.variant(LEG2_MODE_A::CSD)
    }
}
#[doc = "Field `DSI_CTRL_EN` reader - N/A"]
pub type DSI_CTRL_EN_R = crate::BitReader<bool>;
#[doc = "Field `DSI_CTRL_EN` writer - N/A"]
pub type DSI_CTRL_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDACA_SPEC, bool, O>;
#[doc = "N/A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RANGE_A {
    #[doc = "0: N/A"]
    IDAC_LO = 0,
    #[doc = "1: N/A"]
    IDAC_MED = 1,
    #[doc = "2: N/A"]
    IDAC_HI = 2,
}
impl From<RANGE_A> for u8 {
    #[inline(always)]
    fn from(variant: RANGE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RANGE` reader - N/A"]
pub type RANGE_R = crate::FieldReader<u8, RANGE_A>;
impl RANGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RANGE_A> {
        match self.bits {
            0 => Some(RANGE_A::IDAC_LO),
            1 => Some(RANGE_A::IDAC_MED),
            2 => Some(RANGE_A::IDAC_HI),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `IDAC_LO`"]
    #[inline(always)]
    pub fn is_idac_lo(&self) -> bool {
        *self == RANGE_A::IDAC_LO
    }
    #[doc = "Checks if the value of the field is `IDAC_MED`"]
    #[inline(always)]
    pub fn is_idac_med(&self) -> bool {
        *self == RANGE_A::IDAC_MED
    }
    #[doc = "Checks if the value of the field is `IDAC_HI`"]
    #[inline(always)]
    pub fn is_idac_hi(&self) -> bool {
        *self == RANGE_A::IDAC_HI
    }
}
#[doc = "Field `RANGE` writer - N/A"]
pub type RANGE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IDACA_SPEC, u8, RANGE_A, 2, O>;
impl<'a, const O: u8> RANGE_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn idac_lo(self) -> &'a mut W {
        self.variant(RANGE_A::IDAC_LO)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn idac_med(self) -> &'a mut W {
        self.variant(RANGE_A::IDAC_MED)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn idac_hi(self) -> &'a mut W {
        self.variant(RANGE_A::IDAC_HI)
    }
}
#[doc = "Field `LEG1_EN` reader - N/A"]
pub type LEG1_EN_R = crate::BitReader<bool>;
#[doc = "Field `LEG1_EN` writer - N/A"]
pub type LEG1_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDACA_SPEC, bool, O>;
#[doc = "Field `LEG2_EN` reader - N/A"]
pub type LEG2_EN_R = crate::BitReader<bool>;
#[doc = "Field `LEG2_EN` writer - N/A"]
pub type LEG2_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDACA_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:6 - N/A"]
    #[inline(always)]
    pub fn val(&self) -> VAL_R {
        VAL_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - N/A"]
    #[inline(always)]
    pub fn pol_dyn(&self) -> POL_DYN_R {
        POL_DYN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - N/A"]
    #[inline(always)]
    pub fn polarity(&self) -> POLARITY_R {
        POLARITY_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - N/A"]
    #[inline(always)]
    pub fn bal_mode(&self) -> BAL_MODE_R {
        BAL_MODE_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 16:17 - N/A"]
    #[inline(always)]
    pub fn leg1_mode(&self) -> LEG1_MODE_R {
        LEG1_MODE_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - N/A"]
    #[inline(always)]
    pub fn leg2_mode(&self) -> LEG2_MODE_R {
        LEG2_MODE_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 21 - N/A"]
    #[inline(always)]
    pub fn dsi_ctrl_en(&self) -> DSI_CTRL_EN_R {
        DSI_CTRL_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:23 - N/A"]
    #[inline(always)]
    pub fn range(&self) -> RANGE_R {
        RANGE_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bit 24 - N/A"]
    #[inline(always)]
    pub fn leg1_en(&self) -> LEG1_EN_R {
        LEG1_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - N/A"]
    #[inline(always)]
    pub fn leg2_en(&self) -> LEG2_EN_R {
        LEG2_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - N/A"]
    #[inline(always)]
    pub fn val(&mut self) -> VAL_W<0> {
        VAL_W::new(self)
    }
    #[doc = "Bit 7 - N/A"]
    #[inline(always)]
    pub fn pol_dyn(&mut self) -> POL_DYN_W<7> {
        POL_DYN_W::new(self)
    }
    #[doc = "Bits 8:9 - N/A"]
    #[inline(always)]
    pub fn polarity(&mut self) -> POLARITY_W<8> {
        POLARITY_W::new(self)
    }
    #[doc = "Bits 10:11 - N/A"]
    #[inline(always)]
    pub fn bal_mode(&mut self) -> BAL_MODE_W<10> {
        BAL_MODE_W::new(self)
    }
    #[doc = "Bits 16:17 - N/A"]
    #[inline(always)]
    pub fn leg1_mode(&mut self) -> LEG1_MODE_W<16> {
        LEG1_MODE_W::new(self)
    }
    #[doc = "Bits 18:19 - N/A"]
    #[inline(always)]
    pub fn leg2_mode(&mut self) -> LEG2_MODE_W<18> {
        LEG2_MODE_W::new(self)
    }
    #[doc = "Bit 21 - N/A"]
    #[inline(always)]
    pub fn dsi_ctrl_en(&mut self) -> DSI_CTRL_EN_W<21> {
        DSI_CTRL_EN_W::new(self)
    }
    #[doc = "Bits 22:23 - N/A"]
    #[inline(always)]
    pub fn range(&mut self) -> RANGE_W<22> {
        RANGE_W::new(self)
    }
    #[doc = "Bit 24 - N/A"]
    #[inline(always)]
    pub fn leg1_en(&mut self) -> LEG1_EN_W<24> {
        LEG1_EN_W::new(self)
    }
    #[doc = "Bit 25 - N/A"]
    #[inline(always)]
    pub fn leg2_en(&mut self) -> LEG2_EN_W<25> {
        LEG2_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IDACA Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idaca](index.html) module"]
pub struct IDACA_SPEC;
impl crate::RegisterSpec for IDACA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [idaca::R](R) reader structure"]
impl crate::Readable for IDACA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [idaca::W](W) writer structure"]
impl crate::Writable for IDACA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IDACA to value 0"]
impl crate::Resettable for IDACA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
