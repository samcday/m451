#[doc = "Register `PWM_INTEN0` reader"]
pub struct R(crate::R<PWM_INTEN0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM_INTEN0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PWM_INTEN0_SPEC>> for R {
    fn from(reader: crate::R<PWM_INTEN0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWM_INTEN0` writer"]
pub struct W(crate::W<PWM_INTEN0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWM_INTEN0_SPEC>;
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
impl core::convert::From<crate::W<PWM_INTEN0_SPEC>> for W {
    fn from(writer: crate::W<PWM_INTEN0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "PWM Zero Point Interrupt Enable Bits\\nEach bit n controls the corresponding PWM channel n.\\nNote: Odd channels will read always 0 at complementary mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ZIENN_A {
    #[doc = "0: Zero point interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Zero point interrupt Enabled"]
    _1 = 1,
}
impl From<ZIENN_A> for u8 {
    #[inline(always)]
    fn from(variant: ZIENN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ZIENn` reader - PWM Zero Point Interrupt Enable Bits\\nEach bit n controls the corresponding PWM channel n.\\nNote: Odd channels will read always 0 at complementary mode."]
pub struct ZIENN_R(crate::FieldReader<u8, ZIENN_A>);
impl ZIENN_R {
    pub(crate) fn new(bits: u8) -> Self {
        ZIENN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ZIENN_A> {
        match self.bits {
            0 => Some(ZIENN_A::_0),
            1 => Some(ZIENN_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ZIENN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ZIENN_A::_1
    }
}
impl core::ops::Deref for ZIENN_R {
    type Target = crate::FieldReader<u8, ZIENN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ZIENn` writer - PWM Zero Point Interrupt Enable Bits\\nEach bit n controls the corresponding PWM channel n.\\nNote: Odd channels will read always 0 at complementary mode."]
pub struct ZIENN_W<'a> {
    w: &'a mut W,
}
impl<'a> ZIENN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ZIENN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Zero point interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ZIENN_A::_0)
    }
    #[doc = "Zero point interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ZIENN_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
#[doc = "PWM_CH0/1 Interrupt Flag Accumulator Interrupt Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IFAIEN0_1_A {
    #[doc = "0: Interrupt Flag accumulator interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Interrupt Flag accumulator interrupt Enabled"]
    _1 = 1,
}
impl From<IFAIEN0_1_A> for bool {
    #[inline(always)]
    fn from(variant: IFAIEN0_1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IFAIEN0_1` reader - PWM_CH0/1 Interrupt Flag Accumulator Interrupt Enable Bit"]
pub struct IFAIEN0_1_R(crate::FieldReader<bool, IFAIEN0_1_A>);
impl IFAIEN0_1_R {
    pub(crate) fn new(bits: bool) -> Self {
        IFAIEN0_1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IFAIEN0_1_A {
        match self.bits {
            false => IFAIEN0_1_A::_0,
            true => IFAIEN0_1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == IFAIEN0_1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == IFAIEN0_1_A::_1
    }
}
impl core::ops::Deref for IFAIEN0_1_R {
    type Target = crate::FieldReader<bool, IFAIEN0_1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IFAIEN0_1` writer - PWM_CH0/1 Interrupt Flag Accumulator Interrupt Enable Bit"]
pub struct IFAIEN0_1_W<'a> {
    w: &'a mut W,
}
impl<'a> IFAIEN0_1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IFAIEN0_1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt Flag accumulator interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IFAIEN0_1_A::_0)
    }
    #[doc = "Interrupt Flag accumulator interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IFAIEN0_1_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "PWM Period Point Interrupt Enable Bits\\nEach bit n controls the corresponding PWM channel n.\\nNote1: When up-down counter type period point means center point.\\nNote2: Odd channels will read always 0 at complementary mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PIENN_A {
    #[doc = "0: Period point interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Period point interrupt Enabled"]
    _1 = 1,
}
impl From<PIENN_A> for u8 {
    #[inline(always)]
    fn from(variant: PIENN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PIENn` reader - PWM Period Point Interrupt Enable Bits\\nEach bit n controls the corresponding PWM channel n.\\nNote1: When up-down counter type period point means center point.\\nNote2: Odd channels will read always 0 at complementary mode."]
pub struct PIENN_R(crate::FieldReader<u8, PIENN_A>);
impl PIENN_R {
    pub(crate) fn new(bits: u8) -> Self {
        PIENN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PIENN_A> {
        match self.bits {
            0 => Some(PIENN_A::_0),
            1 => Some(PIENN_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PIENN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PIENN_A::_1
    }
}
impl core::ops::Deref for PIENN_R {
    type Target = crate::FieldReader<u8, PIENN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIENn` writer - PWM Period Point Interrupt Enable Bits\\nEach bit n controls the corresponding PWM channel n.\\nNote1: When up-down counter type period point means center point.\\nNote2: Odd channels will read always 0 at complementary mode."]
pub struct PIENN_W<'a> {
    w: &'a mut W,
}
impl<'a> PIENN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIENN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Period point interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PIENN_A::_0)
    }
    #[doc = "Period point interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PIENN_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | ((value as u32 & 0x3f) << 8);
        self.w
    }
}
#[doc = "PWM_CH2/3 Interrupt Flag Accumulator Interrupt Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IFAIEN2_3_A {
    #[doc = "0: Interrupt Flag accumulator interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Interrupt Flag accumulator interrupt Enabled"]
    _1 = 1,
}
impl From<IFAIEN2_3_A> for bool {
    #[inline(always)]
    fn from(variant: IFAIEN2_3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IFAIEN2_3` reader - PWM_CH2/3 Interrupt Flag Accumulator Interrupt Enable Bit"]
pub struct IFAIEN2_3_R(crate::FieldReader<bool, IFAIEN2_3_A>);
impl IFAIEN2_3_R {
    pub(crate) fn new(bits: bool) -> Self {
        IFAIEN2_3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IFAIEN2_3_A {
        match self.bits {
            false => IFAIEN2_3_A::_0,
            true => IFAIEN2_3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == IFAIEN2_3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == IFAIEN2_3_A::_1
    }
}
impl core::ops::Deref for IFAIEN2_3_R {
    type Target = crate::FieldReader<bool, IFAIEN2_3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IFAIEN2_3` writer - PWM_CH2/3 Interrupt Flag Accumulator Interrupt Enable Bit"]
pub struct IFAIEN2_3_W<'a> {
    w: &'a mut W,
}
impl<'a> IFAIEN2_3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IFAIEN2_3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt Flag accumulator interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IFAIEN2_3_A::_0)
    }
    #[doc = "Interrupt Flag accumulator interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IFAIEN2_3_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "PWM Compare Up Count Interrupt Enable Bits\\nEach bit n controls the corresponding PWM channel n.\\nNote: In complementary mode, CMPUIEN1, 3, 5 use as another CMPUIEN for channel 0, 2, 4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CMPUIENN_A {
    #[doc = "0: Compare up count interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Compare up count interrupt Enabled"]
    _1 = 1,
}
impl From<CMPUIENN_A> for u8 {
    #[inline(always)]
    fn from(variant: CMPUIENN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CMPUIENn` reader - PWM Compare Up Count Interrupt Enable Bits\\nEach bit n controls the corresponding PWM channel n.\\nNote: In complementary mode, CMPUIEN1, 3, 5 use as another CMPUIEN for channel 0, 2, 4."]
pub struct CMPUIENN_R(crate::FieldReader<u8, CMPUIENN_A>);
impl CMPUIENN_R {
    pub(crate) fn new(bits: u8) -> Self {
        CMPUIENN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CMPUIENN_A> {
        match self.bits {
            0 => Some(CMPUIENN_A::_0),
            1 => Some(CMPUIENN_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CMPUIENN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CMPUIENN_A::_1
    }
}
impl core::ops::Deref for CMPUIENN_R {
    type Target = crate::FieldReader<u8, CMPUIENN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPUIENn` writer - PWM Compare Up Count Interrupt Enable Bits\\nEach bit n controls the corresponding PWM channel n.\\nNote: In complementary mode, CMPUIEN1, 3, 5 use as another CMPUIEN for channel 0, 2, 4."]
pub struct CMPUIENN_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPUIENN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMPUIENN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Compare up count interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPUIENN_A::_0)
    }
    #[doc = "Compare up count interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPUIENN_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | ((value as u32 & 0x3f) << 16);
        self.w
    }
}
#[doc = "PWM_CH4/5 Interrupt Flag Accumulator Interrupt Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IFAIEN4_5_A {
    #[doc = "0: Interrupt Flag accumulator interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Interrupt Flag accumulator interrupt Enabled"]
    _1 = 1,
}
impl From<IFAIEN4_5_A> for bool {
    #[inline(always)]
    fn from(variant: IFAIEN4_5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IFAIEN4_5` reader - PWM_CH4/5 Interrupt Flag Accumulator Interrupt Enable Bit"]
pub struct IFAIEN4_5_R(crate::FieldReader<bool, IFAIEN4_5_A>);
impl IFAIEN4_5_R {
    pub(crate) fn new(bits: bool) -> Self {
        IFAIEN4_5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IFAIEN4_5_A {
        match self.bits {
            false => IFAIEN4_5_A::_0,
            true => IFAIEN4_5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == IFAIEN4_5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == IFAIEN4_5_A::_1
    }
}
impl core::ops::Deref for IFAIEN4_5_R {
    type Target = crate::FieldReader<bool, IFAIEN4_5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IFAIEN4_5` writer - PWM_CH4/5 Interrupt Flag Accumulator Interrupt Enable Bit"]
pub struct IFAIEN4_5_W<'a> {
    w: &'a mut W,
}
impl<'a> IFAIEN4_5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IFAIEN4_5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt Flag accumulator interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IFAIEN4_5_A::_0)
    }
    #[doc = "Interrupt Flag accumulator interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IFAIEN4_5_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "PWM Compare Down Count Interrupt Enable Bits\\nEach bit n controls the corresponding PWM channel n.\\nNote: In complementary mode, CMPDIEN1, 3, 5 use as another CMPDIEN for channel 0, 2, 4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CMPDIENN_A {
    #[doc = "0: Compare down count interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Compare down count interrupt Enabled"]
    _1 = 1,
}
impl From<CMPDIENN_A> for u8 {
    #[inline(always)]
    fn from(variant: CMPDIENN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CMPDIENn` reader - PWM Compare Down Count Interrupt Enable Bits\\nEach bit n controls the corresponding PWM channel n.\\nNote: In complementary mode, CMPDIEN1, 3, 5 use as another CMPDIEN for channel 0, 2, 4."]
pub struct CMPDIENN_R(crate::FieldReader<u8, CMPDIENN_A>);
impl CMPDIENN_R {
    pub(crate) fn new(bits: u8) -> Self {
        CMPDIENN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CMPDIENN_A> {
        match self.bits {
            0 => Some(CMPDIENN_A::_0),
            1 => Some(CMPDIENN_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CMPDIENN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CMPDIENN_A::_1
    }
}
impl core::ops::Deref for CMPDIENN_R {
    type Target = crate::FieldReader<u8, CMPDIENN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPDIENn` writer - PWM Compare Down Count Interrupt Enable Bits\\nEach bit n controls the corresponding PWM channel n.\\nNote: In complementary mode, CMPDIEN1, 3, 5 use as another CMPDIEN for channel 0, 2, 4."]
pub struct CMPDIENN_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPDIENN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMPDIENN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Compare down count interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMPDIENN_A::_0)
    }
    #[doc = "Compare down count interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMPDIENN_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | ((value as u32 & 0x3f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - PWM Zero Point Interrupt Enable Bits\\nEach bit n controls the corresponding PWM channel n.\\nNote: Odd channels will read always 0 at complementary mode."]
    #[inline(always)]
    pub fn zienn(&self) -> ZIENN_R {
        ZIENN_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 7 - PWM_CH0/1 Interrupt Flag Accumulator Interrupt Enable Bit"]
    #[inline(always)]
    pub fn ifaien0_1(&self) -> IFAIEN0_1_R {
        IFAIEN0_1_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:13 - PWM Period Point Interrupt Enable Bits\\nEach bit n controls the corresponding PWM channel n.\\nNote1: When up-down counter type period point means center point.\\nNote2: Odd channels will read always 0 at complementary mode."]
    #[inline(always)]
    pub fn pienn(&self) -> PIENN_R {
        PIENN_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 15 - PWM_CH2/3 Interrupt Flag Accumulator Interrupt Enable Bit"]
    #[inline(always)]
    pub fn ifaien2_3(&self) -> IFAIEN2_3_R {
        IFAIEN2_3_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:21 - PWM Compare Up Count Interrupt Enable Bits\\nEach bit n controls the corresponding PWM channel n.\\nNote: In complementary mode, CMPUIEN1, 3, 5 use as another CMPUIEN for channel 0, 2, 4."]
    #[inline(always)]
    pub fn cmpuienn(&self) -> CMPUIENN_R {
        CMPUIENN_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 23 - PWM_CH4/5 Interrupt Flag Accumulator Interrupt Enable Bit"]
    #[inline(always)]
    pub fn ifaien4_5(&self) -> IFAIEN4_5_R {
        IFAIEN4_5_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:29 - PWM Compare Down Count Interrupt Enable Bits\\nEach bit n controls the corresponding PWM channel n.\\nNote: In complementary mode, CMPDIEN1, 3, 5 use as another CMPDIEN for channel 0, 2, 4."]
    #[inline(always)]
    pub fn cmpdienn(&self) -> CMPDIENN_R {
        CMPDIENN_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - PWM Zero Point Interrupt Enable Bits\\nEach bit n controls the corresponding PWM channel n.\\nNote: Odd channels will read always 0 at complementary mode."]
    #[inline(always)]
    pub fn zienn(&mut self) -> ZIENN_W {
        ZIENN_W { w: self }
    }
    #[doc = "Bit 7 - PWM_CH0/1 Interrupt Flag Accumulator Interrupt Enable Bit"]
    #[inline(always)]
    pub fn ifaien0_1(&mut self) -> IFAIEN0_1_W {
        IFAIEN0_1_W { w: self }
    }
    #[doc = "Bits 8:13 - PWM Period Point Interrupt Enable Bits\\nEach bit n controls the corresponding PWM channel n.\\nNote1: When up-down counter type period point means center point.\\nNote2: Odd channels will read always 0 at complementary mode."]
    #[inline(always)]
    pub fn pienn(&mut self) -> PIENN_W {
        PIENN_W { w: self }
    }
    #[doc = "Bit 15 - PWM_CH2/3 Interrupt Flag Accumulator Interrupt Enable Bit"]
    #[inline(always)]
    pub fn ifaien2_3(&mut self) -> IFAIEN2_3_W {
        IFAIEN2_3_W { w: self }
    }
    #[doc = "Bits 16:21 - PWM Compare Up Count Interrupt Enable Bits\\nEach bit n controls the corresponding PWM channel n.\\nNote: In complementary mode, CMPUIEN1, 3, 5 use as another CMPUIEN for channel 0, 2, 4."]
    #[inline(always)]
    pub fn cmpuienn(&mut self) -> CMPUIENN_W {
        CMPUIENN_W { w: self }
    }
    #[doc = "Bit 23 - PWM_CH4/5 Interrupt Flag Accumulator Interrupt Enable Bit"]
    #[inline(always)]
    pub fn ifaien4_5(&mut self) -> IFAIEN4_5_W {
        IFAIEN4_5_W { w: self }
    }
    #[doc = "Bits 24:29 - PWM Compare Down Count Interrupt Enable Bits\\nEach bit n controls the corresponding PWM channel n.\\nNote: In complementary mode, CMPDIEN1, 3, 5 use as another CMPDIEN for channel 0, 2, 4."]
    #[inline(always)]
    pub fn cmpdienn(&mut self) -> CMPDIENN_W {
        CMPDIENN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Interrupt Enable Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_inten0](index.html) module"]
pub struct PWM_INTEN0_SPEC;
impl crate::RegisterSpec for PWM_INTEN0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwm_inten0::R](R) reader structure"]
impl crate::Readable for PWM_INTEN0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwm_inten0::W](W) writer structure"]
impl crate::Writable for PWM_INTEN0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWM_INTEN0 to value 0"]
impl crate::Resettable for PWM_INTEN0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
