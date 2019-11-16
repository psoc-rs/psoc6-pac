#[doc = "Reader of register IDACA"]
pub type R = crate::R<u32, super::IDACA>;
#[doc = "Writer for register IDACA"]
pub type W = crate::W<u32, super::IDACA>;
#[doc = "Register IDACA `reset()`'s with value 0"]
impl crate::ResetValue for super::IDACA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `VAL`"]
pub type VAL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VAL`"]
pub struct VAL_W<'a> {
    w: &'a mut W,
}
impl<'a> VAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
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
#[doc = "Reader of field `POL_DYN`"]
pub type POL_DYN_R = crate::R<bool, POL_DYN_A>;
impl POL_DYN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POL_DYN_A {
        match self.bits {
            false => POL_DYN_A::STATIC,
            true => POL_DYN_A::DYNAMIC,
        }
    }
    #[doc = "Checks if the value of the field is `STATIC`"]
    #[inline(always)]
    pub fn is_static_(&self) -> bool {
        *self == POL_DYN_A::STATIC
    }
    #[doc = "Checks if the value of the field is `DYNAMIC`"]
    #[inline(always)]
    pub fn is_dynamic(&self) -> bool {
        *self == POL_DYN_A::DYNAMIC
    }
}
#[doc = "Write proxy for field `POL_DYN`"]
pub struct POL_DYN_W<'a> {
    w: &'a mut W,
}
impl<'a> POL_DYN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POL_DYN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "N/A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum POLARITY_A {
    #[doc = "0: Normal: sensor switching between Vssio and Cmod.   For non-CSD application, IDAC1 will source current."]
    VSSA_SRC = 0,
    #[doc = "1: Inverted: sensor switch between Vddio and Cmod.  For non-CSD application, IDAC1 will sink current."]
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
#[doc = "Reader of field `POLARITY`"]
pub type POLARITY_R = crate::R<u8, POLARITY_A>;
impl POLARITY_R {
    #[doc = r"Get enumerated values variant"]
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
#[doc = "Write proxy for field `POLARITY`"]
pub struct POLARITY_W<'a> {
    w: &'a mut W,
}
impl<'a> POLARITY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POLARITY_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
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
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
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
#[doc = "Reader of field `BAL_MODE`"]
pub type BAL_MODE_R = crate::R<u8, BAL_MODE_A>;
impl BAL_MODE_R {
    #[doc = r"Get enumerated values variant"]
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
#[doc = "Write proxy for field `BAL_MODE`"]
pub struct BAL_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> BAL_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BAL_MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
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
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
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
#[doc = "Reader of field `LEG1_MODE`"]
pub type LEG1_MODE_R = crate::R<u8, LEG1_MODE_A>;
impl LEG1_MODE_R {
    #[doc = r"Get enumerated values variant"]
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
#[doc = "Write proxy for field `LEG1_MODE`"]
pub struct LEG1_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> LEG1_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LEG1_MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
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
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
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
#[doc = "Reader of field `LEG2_MODE`"]
pub type LEG2_MODE_R = crate::R<u8, LEG2_MODE_A>;
impl LEG2_MODE_R {
    #[doc = r"Get enumerated values variant"]
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
#[doc = "Write proxy for field `LEG2_MODE`"]
pub struct LEG2_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> LEG2_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LEG2_MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
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
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "Reader of field `DSI_CTRL_EN`"]
pub type DSI_CTRL_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DSI_CTRL_EN`"]
pub struct DSI_CTRL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DSI_CTRL_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
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
#[doc = "Reader of field `RANGE`"]
pub type RANGE_R = crate::R<u8, RANGE_A>;
impl RANGE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RANGE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RANGE_A::IDAC_LO),
            1 => Val(RANGE_A::IDAC_MED),
            2 => Val(RANGE_A::IDAC_HI),
            i => Res(i),
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
#[doc = "Write proxy for field `RANGE`"]
pub struct RANGE_W<'a> {
    w: &'a mut W,
}
impl<'a> RANGE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RANGE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
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
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Reader of field `LEG1_EN`"]
pub type LEG1_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LEG1_EN`"]
pub struct LEG1_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LEG1_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `LEG2_EN`"]
pub type LEG2_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LEG2_EN`"]
pub struct LEG2_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LEG2_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - N/A"]
    #[inline(always)]
    pub fn val(&self) -> VAL_R {
        VAL_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - N/A"]
    #[inline(always)]
    pub fn pol_dyn(&self) -> POL_DYN_R {
        POL_DYN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - N/A"]
    #[inline(always)]
    pub fn polarity(&self) -> POLARITY_R {
        POLARITY_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - N/A"]
    #[inline(always)]
    pub fn bal_mode(&self) -> BAL_MODE_R {
        BAL_MODE_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - N/A"]
    #[inline(always)]
    pub fn leg1_mode(&self) -> LEG1_MODE_R {
        LEG1_MODE_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - N/A"]
    #[inline(always)]
    pub fn leg2_mode(&self) -> LEG2_MODE_R {
        LEG2_MODE_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bit 21 - N/A"]
    #[inline(always)]
    pub fn dsi_ctrl_en(&self) -> DSI_CTRL_EN_R {
        DSI_CTRL_EN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bits 22:23 - N/A"]
    #[inline(always)]
    pub fn range(&self) -> RANGE_R {
        RANGE_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bit 24 - N/A"]
    #[inline(always)]
    pub fn leg1_en(&self) -> LEG1_EN_R {
        LEG1_EN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - N/A"]
    #[inline(always)]
    pub fn leg2_en(&self) -> LEG2_EN_R {
        LEG2_EN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - N/A"]
    #[inline(always)]
    pub fn val(&mut self) -> VAL_W {
        VAL_W { w: self }
    }
    #[doc = "Bit 7 - N/A"]
    #[inline(always)]
    pub fn pol_dyn(&mut self) -> POL_DYN_W {
        POL_DYN_W { w: self }
    }
    #[doc = "Bits 8:9 - N/A"]
    #[inline(always)]
    pub fn polarity(&mut self) -> POLARITY_W {
        POLARITY_W { w: self }
    }
    #[doc = "Bits 10:11 - N/A"]
    #[inline(always)]
    pub fn bal_mode(&mut self) -> BAL_MODE_W {
        BAL_MODE_W { w: self }
    }
    #[doc = "Bits 16:17 - N/A"]
    #[inline(always)]
    pub fn leg1_mode(&mut self) -> LEG1_MODE_W {
        LEG1_MODE_W { w: self }
    }
    #[doc = "Bits 18:19 - N/A"]
    #[inline(always)]
    pub fn leg2_mode(&mut self) -> LEG2_MODE_W {
        LEG2_MODE_W { w: self }
    }
    #[doc = "Bit 21 - N/A"]
    #[inline(always)]
    pub fn dsi_ctrl_en(&mut self) -> DSI_CTRL_EN_W {
        DSI_CTRL_EN_W { w: self }
    }
    #[doc = "Bits 22:23 - N/A"]
    #[inline(always)]
    pub fn range(&mut self) -> RANGE_W {
        RANGE_W { w: self }
    }
    #[doc = "Bit 24 - N/A"]
    #[inline(always)]
    pub fn leg1_en(&mut self) -> LEG1_EN_W {
        LEG1_EN_W { w: self }
    }
    #[doc = "Bit 25 - N/A"]
    #[inline(always)]
    pub fn leg2_en(&mut self) -> LEG2_EN_W {
        LEG2_EN_W { w: self }
    }
}
