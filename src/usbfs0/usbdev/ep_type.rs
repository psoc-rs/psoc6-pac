#[doc = "Reader of register EP_TYPE"]
pub type R = crate::R<u32, super::EP_TYPE>;
#[doc = "Writer for register EP_TYPE"]
pub type W = crate::W<u32, super::EP_TYPE>;
#[doc = "Register EP_TYPE `reset()`'s with value 0"]
impl crate::ResetValue for super::EP_TYPE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Endpoint Type Indication.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EP1_TYP_A {
    #[doc = "0: IN outpoint"]
    EP_IN = 0,
    #[doc = "1: OUT outpoint"]
    EP_OUT = 1,
}
impl From<EP1_TYP_A> for bool {
    #[inline(always)]
    fn from(variant: EP1_TYP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EP1_TYP`"]
pub type EP1_TYP_R = crate::R<bool, EP1_TYP_A>;
impl EP1_TYP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EP1_TYP_A {
        match self.bits {
            false => EP1_TYP_A::EP_IN,
            true => EP1_TYP_A::EP_OUT,
        }
    }
    #[doc = "Checks if the value of the field is `EP_IN`"]
    #[inline(always)]
    pub fn is_ep_in(&self) -> bool {
        *self == EP1_TYP_A::EP_IN
    }
    #[doc = "Checks if the value of the field is `EP_OUT`"]
    #[inline(always)]
    pub fn is_ep_out(&self) -> bool {
        *self == EP1_TYP_A::EP_OUT
    }
}
#[doc = "Write proxy for field `EP1_TYP`"]
pub struct EP1_TYP_W<'a> {
    w: &'a mut W,
}
impl<'a> EP1_TYP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EP1_TYP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "IN outpoint"]
    #[inline(always)]
    pub fn ep_in(self) -> &'a mut W {
        self.variant(EP1_TYP_A::EP_IN)
    }
    #[doc = "OUT outpoint"]
    #[inline(always)]
    pub fn ep_out(self) -> &'a mut W {
        self.variant(EP1_TYP_A::EP_OUT)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Endpoint Type Indication.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EP2_TYP_A {
    #[doc = "0: IN outpoint"]
    EP_IN = 0,
    #[doc = "1: OUT outpoint"]
    EP_OUT = 1,
}
impl From<EP2_TYP_A> for bool {
    #[inline(always)]
    fn from(variant: EP2_TYP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EP2_TYP`"]
pub type EP2_TYP_R = crate::R<bool, EP2_TYP_A>;
impl EP2_TYP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EP2_TYP_A {
        match self.bits {
            false => EP2_TYP_A::EP_IN,
            true => EP2_TYP_A::EP_OUT,
        }
    }
    #[doc = "Checks if the value of the field is `EP_IN`"]
    #[inline(always)]
    pub fn is_ep_in(&self) -> bool {
        *self == EP2_TYP_A::EP_IN
    }
    #[doc = "Checks if the value of the field is `EP_OUT`"]
    #[inline(always)]
    pub fn is_ep_out(&self) -> bool {
        *self == EP2_TYP_A::EP_OUT
    }
}
#[doc = "Write proxy for field `EP2_TYP`"]
pub struct EP2_TYP_W<'a> {
    w: &'a mut W,
}
impl<'a> EP2_TYP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EP2_TYP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "IN outpoint"]
    #[inline(always)]
    pub fn ep_in(self) -> &'a mut W {
        self.variant(EP2_TYP_A::EP_IN)
    }
    #[doc = "OUT outpoint"]
    #[inline(always)]
    pub fn ep_out(self) -> &'a mut W {
        self.variant(EP2_TYP_A::EP_OUT)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Endpoint Type Indication.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EP3_TYP_A {
    #[doc = "0: IN outpoint"]
    EP_IN = 0,
    #[doc = "1: OUT outpoint"]
    EP_OUT = 1,
}
impl From<EP3_TYP_A> for bool {
    #[inline(always)]
    fn from(variant: EP3_TYP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EP3_TYP`"]
pub type EP3_TYP_R = crate::R<bool, EP3_TYP_A>;
impl EP3_TYP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EP3_TYP_A {
        match self.bits {
            false => EP3_TYP_A::EP_IN,
            true => EP3_TYP_A::EP_OUT,
        }
    }
    #[doc = "Checks if the value of the field is `EP_IN`"]
    #[inline(always)]
    pub fn is_ep_in(&self) -> bool {
        *self == EP3_TYP_A::EP_IN
    }
    #[doc = "Checks if the value of the field is `EP_OUT`"]
    #[inline(always)]
    pub fn is_ep_out(&self) -> bool {
        *self == EP3_TYP_A::EP_OUT
    }
}
#[doc = "Write proxy for field `EP3_TYP`"]
pub struct EP3_TYP_W<'a> {
    w: &'a mut W,
}
impl<'a> EP3_TYP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EP3_TYP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "IN outpoint"]
    #[inline(always)]
    pub fn ep_in(self) -> &'a mut W {
        self.variant(EP3_TYP_A::EP_IN)
    }
    #[doc = "OUT outpoint"]
    #[inline(always)]
    pub fn ep_out(self) -> &'a mut W {
        self.variant(EP3_TYP_A::EP_OUT)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Endpoint Type Indication.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EP4_TYP_A {
    #[doc = "0: IN outpoint"]
    EP_IN = 0,
    #[doc = "1: OUT outpoint"]
    EP_OUT = 1,
}
impl From<EP4_TYP_A> for bool {
    #[inline(always)]
    fn from(variant: EP4_TYP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EP4_TYP`"]
pub type EP4_TYP_R = crate::R<bool, EP4_TYP_A>;
impl EP4_TYP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EP4_TYP_A {
        match self.bits {
            false => EP4_TYP_A::EP_IN,
            true => EP4_TYP_A::EP_OUT,
        }
    }
    #[doc = "Checks if the value of the field is `EP_IN`"]
    #[inline(always)]
    pub fn is_ep_in(&self) -> bool {
        *self == EP4_TYP_A::EP_IN
    }
    #[doc = "Checks if the value of the field is `EP_OUT`"]
    #[inline(always)]
    pub fn is_ep_out(&self) -> bool {
        *self == EP4_TYP_A::EP_OUT
    }
}
#[doc = "Write proxy for field `EP4_TYP`"]
pub struct EP4_TYP_W<'a> {
    w: &'a mut W,
}
impl<'a> EP4_TYP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EP4_TYP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "IN outpoint"]
    #[inline(always)]
    pub fn ep_in(self) -> &'a mut W {
        self.variant(EP4_TYP_A::EP_IN)
    }
    #[doc = "OUT outpoint"]
    #[inline(always)]
    pub fn ep_out(self) -> &'a mut W {
        self.variant(EP4_TYP_A::EP_OUT)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Endpoint Type Indication.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EP5_TYP_A {
    #[doc = "0: IN outpoint"]
    EP_IN = 0,
    #[doc = "1: OUT outpoint"]
    EP_OUT = 1,
}
impl From<EP5_TYP_A> for bool {
    #[inline(always)]
    fn from(variant: EP5_TYP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EP5_TYP`"]
pub type EP5_TYP_R = crate::R<bool, EP5_TYP_A>;
impl EP5_TYP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EP5_TYP_A {
        match self.bits {
            false => EP5_TYP_A::EP_IN,
            true => EP5_TYP_A::EP_OUT,
        }
    }
    #[doc = "Checks if the value of the field is `EP_IN`"]
    #[inline(always)]
    pub fn is_ep_in(&self) -> bool {
        *self == EP5_TYP_A::EP_IN
    }
    #[doc = "Checks if the value of the field is `EP_OUT`"]
    #[inline(always)]
    pub fn is_ep_out(&self) -> bool {
        *self == EP5_TYP_A::EP_OUT
    }
}
#[doc = "Write proxy for field `EP5_TYP`"]
pub struct EP5_TYP_W<'a> {
    w: &'a mut W,
}
impl<'a> EP5_TYP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EP5_TYP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "IN outpoint"]
    #[inline(always)]
    pub fn ep_in(self) -> &'a mut W {
        self.variant(EP5_TYP_A::EP_IN)
    }
    #[doc = "OUT outpoint"]
    #[inline(always)]
    pub fn ep_out(self) -> &'a mut W {
        self.variant(EP5_TYP_A::EP_OUT)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Endpoint Type Indication.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EP6_TYP_A {
    #[doc = "0: IN outpoint"]
    EP_IN = 0,
    #[doc = "1: OUT outpoint"]
    EP_OUT = 1,
}
impl From<EP6_TYP_A> for bool {
    #[inline(always)]
    fn from(variant: EP6_TYP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EP6_TYP`"]
pub type EP6_TYP_R = crate::R<bool, EP6_TYP_A>;
impl EP6_TYP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EP6_TYP_A {
        match self.bits {
            false => EP6_TYP_A::EP_IN,
            true => EP6_TYP_A::EP_OUT,
        }
    }
    #[doc = "Checks if the value of the field is `EP_IN`"]
    #[inline(always)]
    pub fn is_ep_in(&self) -> bool {
        *self == EP6_TYP_A::EP_IN
    }
    #[doc = "Checks if the value of the field is `EP_OUT`"]
    #[inline(always)]
    pub fn is_ep_out(&self) -> bool {
        *self == EP6_TYP_A::EP_OUT
    }
}
#[doc = "Write proxy for field `EP6_TYP`"]
pub struct EP6_TYP_W<'a> {
    w: &'a mut W,
}
impl<'a> EP6_TYP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EP6_TYP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "IN outpoint"]
    #[inline(always)]
    pub fn ep_in(self) -> &'a mut W {
        self.variant(EP6_TYP_A::EP_IN)
    }
    #[doc = "OUT outpoint"]
    #[inline(always)]
    pub fn ep_out(self) -> &'a mut W {
        self.variant(EP6_TYP_A::EP_OUT)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Endpoint Type Indication.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EP7_TYP_A {
    #[doc = "0: IN outpoint"]
    EP_IN = 0,
    #[doc = "1: OUT outpoint"]
    EP_OUT = 1,
}
impl From<EP7_TYP_A> for bool {
    #[inline(always)]
    fn from(variant: EP7_TYP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EP7_TYP`"]
pub type EP7_TYP_R = crate::R<bool, EP7_TYP_A>;
impl EP7_TYP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EP7_TYP_A {
        match self.bits {
            false => EP7_TYP_A::EP_IN,
            true => EP7_TYP_A::EP_OUT,
        }
    }
    #[doc = "Checks if the value of the field is `EP_IN`"]
    #[inline(always)]
    pub fn is_ep_in(&self) -> bool {
        *self == EP7_TYP_A::EP_IN
    }
    #[doc = "Checks if the value of the field is `EP_OUT`"]
    #[inline(always)]
    pub fn is_ep_out(&self) -> bool {
        *self == EP7_TYP_A::EP_OUT
    }
}
#[doc = "Write proxy for field `EP7_TYP`"]
pub struct EP7_TYP_W<'a> {
    w: &'a mut W,
}
impl<'a> EP7_TYP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EP7_TYP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "IN outpoint"]
    #[inline(always)]
    pub fn ep_in(self) -> &'a mut W {
        self.variant(EP7_TYP_A::EP_IN)
    }
    #[doc = "OUT outpoint"]
    #[inline(always)]
    pub fn ep_out(self) -> &'a mut W {
        self.variant(EP7_TYP_A::EP_OUT)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Endpoint Type Indication.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EP8_TYP_A {
    #[doc = "0: IN outpoint"]
    EP_IN = 0,
    #[doc = "1: OUT outpoint"]
    EP_OUT = 1,
}
impl From<EP8_TYP_A> for bool {
    #[inline(always)]
    fn from(variant: EP8_TYP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EP8_TYP`"]
pub type EP8_TYP_R = crate::R<bool, EP8_TYP_A>;
impl EP8_TYP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EP8_TYP_A {
        match self.bits {
            false => EP8_TYP_A::EP_IN,
            true => EP8_TYP_A::EP_OUT,
        }
    }
    #[doc = "Checks if the value of the field is `EP_IN`"]
    #[inline(always)]
    pub fn is_ep_in(&self) -> bool {
        *self == EP8_TYP_A::EP_IN
    }
    #[doc = "Checks if the value of the field is `EP_OUT`"]
    #[inline(always)]
    pub fn is_ep_out(&self) -> bool {
        *self == EP8_TYP_A::EP_OUT
    }
}
#[doc = "Write proxy for field `EP8_TYP`"]
pub struct EP8_TYP_W<'a> {
    w: &'a mut W,
}
impl<'a> EP8_TYP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EP8_TYP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "IN outpoint"]
    #[inline(always)]
    pub fn ep_in(self) -> &'a mut W {
        self.variant(EP8_TYP_A::EP_IN)
    }
    #[doc = "OUT outpoint"]
    #[inline(always)]
    pub fn ep_out(self) -> &'a mut W {
        self.variant(EP8_TYP_A::EP_OUT)
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
impl R {
    #[doc = "Bit 0 - Endpoint Type Indication."]
    #[inline(always)]
    pub fn ep1_typ(&self) -> EP1_TYP_R {
        EP1_TYP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Endpoint Type Indication."]
    #[inline(always)]
    pub fn ep2_typ(&self) -> EP2_TYP_R {
        EP2_TYP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Endpoint Type Indication."]
    #[inline(always)]
    pub fn ep3_typ(&self) -> EP3_TYP_R {
        EP3_TYP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Endpoint Type Indication."]
    #[inline(always)]
    pub fn ep4_typ(&self) -> EP4_TYP_R {
        EP4_TYP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Endpoint Type Indication."]
    #[inline(always)]
    pub fn ep5_typ(&self) -> EP5_TYP_R {
        EP5_TYP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Endpoint Type Indication."]
    #[inline(always)]
    pub fn ep6_typ(&self) -> EP6_TYP_R {
        EP6_TYP_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Endpoint Type Indication."]
    #[inline(always)]
    pub fn ep7_typ(&self) -> EP7_TYP_R {
        EP7_TYP_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Endpoint Type Indication."]
    #[inline(always)]
    pub fn ep8_typ(&self) -> EP8_TYP_R {
        EP8_TYP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Endpoint Type Indication."]
    #[inline(always)]
    pub fn ep1_typ(&mut self) -> EP1_TYP_W {
        EP1_TYP_W { w: self }
    }
    #[doc = "Bit 1 - Endpoint Type Indication."]
    #[inline(always)]
    pub fn ep2_typ(&mut self) -> EP2_TYP_W {
        EP2_TYP_W { w: self }
    }
    #[doc = "Bit 2 - Endpoint Type Indication."]
    #[inline(always)]
    pub fn ep3_typ(&mut self) -> EP3_TYP_W {
        EP3_TYP_W { w: self }
    }
    #[doc = "Bit 3 - Endpoint Type Indication."]
    #[inline(always)]
    pub fn ep4_typ(&mut self) -> EP4_TYP_W {
        EP4_TYP_W { w: self }
    }
    #[doc = "Bit 4 - Endpoint Type Indication."]
    #[inline(always)]
    pub fn ep5_typ(&mut self) -> EP5_TYP_W {
        EP5_TYP_W { w: self }
    }
    #[doc = "Bit 5 - Endpoint Type Indication."]
    #[inline(always)]
    pub fn ep6_typ(&mut self) -> EP6_TYP_W {
        EP6_TYP_W { w: self }
    }
    #[doc = "Bit 6 - Endpoint Type Indication."]
    #[inline(always)]
    pub fn ep7_typ(&mut self) -> EP7_TYP_W {
        EP7_TYP_W { w: self }
    }
    #[doc = "Bit 7 - Endpoint Type Indication."]
    #[inline(always)]
    pub fn ep8_typ(&mut self) -> EP8_TYP_W {
        EP8_TYP_W { w: self }
    }
}
