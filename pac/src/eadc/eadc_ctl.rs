#[doc = "Register `EADC_CTL` reader"]
pub struct R(crate::R<EADC_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EADC_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<EADC_CTL_SPEC>> for R {
    fn from(reader: crate::R<EADC_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EADC_CTL` writer"]
pub struct W(crate::W<EADC_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EADC_CTL_SPEC>;
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
impl core::convert::From<crate::W<EADC_CTL_SPEC>> for W {
    fn from(writer: crate::W<EADC_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "A/D Converter Enable Bit\\nNote: Before starting A/D conversion function, this bit should be set to 1. Clear it to 0 to disable A/D converter analog circuit power consumption.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCEN_A {
    #[doc = "0: ADC Disabled"]
    _0 = 0,
    #[doc = "1: ADC Enabled"]
    _1 = 1,
}
impl From<ADCEN_A> for bool {
    #[inline(always)]
    fn from(variant: ADCEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCEN` reader - A/D Converter Enable Bit\\nNote: Before starting A/D conversion function, this bit should be set to 1. Clear it to 0 to disable A/D converter analog circuit power consumption."]
pub struct ADCEN_R(crate::FieldReader<bool, ADCEN_A>);
impl ADCEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADCEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCEN_A {
        match self.bits {
            false => ADCEN_A::_0,
            true => ADCEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ADCEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ADCEN_A::_1
    }
}
impl core::ops::Deref for ADCEN_R {
    type Target = crate::FieldReader<bool, ADCEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADCEN` writer - A/D Converter Enable Bit\\nNote: Before starting A/D conversion function, this bit should be set to 1. Clear it to 0 to disable A/D converter analog circuit power consumption."]
pub struct ADCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "ADC Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADCEN_A::_0)
    }
    #[doc = "ADC Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADCEN_A::_1)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "ADC A/D Converter Control Circuits Reset\\nNote: ADCRST bit remains 1 during ADC reset, when ADC reset end, the ADCRST bit is automatically cleared to 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCRST_A {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Cause ADC control circuits reset to initial state, but not change the ADC registers value"]
    _1 = 1,
}
impl From<ADCRST_A> for bool {
    #[inline(always)]
    fn from(variant: ADCRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCRST` reader - ADC A/D Converter Control Circuits Reset\\nNote: ADCRST bit remains 1 during ADC reset, when ADC reset end, the ADCRST bit is automatically cleared to 0."]
pub struct ADCRST_R(crate::FieldReader<bool, ADCRST_A>);
impl ADCRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADCRST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCRST_A {
        match self.bits {
            false => ADCRST_A::_0,
            true => ADCRST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ADCRST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ADCRST_A::_1
    }
}
impl core::ops::Deref for ADCRST_R {
    type Target = crate::FieldReader<bool, ADCRST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADCRST` writer - ADC A/D Converter Control Circuits Reset\\nNote: ADCRST bit remains 1 during ADC reset, when ADC reset end, the ADCRST bit is automatically cleared to 0."]
pub struct ADCRST_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCRST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADCRST_A::_0)
    }
    #[doc = "Cause ADC control circuits reset to initial state, but not change the ADC registers value"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADCRST_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Specific Sample Module A/D ADINT0 Interrupt Enable Bit\\nThe A/D converter generates a conversion end ADIF0 (EADC_STATUS2\\[0\\]) upon the end of specific sample module A/D conversion. If ADCIEN0 bit is set then conversion end interrupt request ADINT0 is generated.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCIEN0_A {
    #[doc = "0: Specific sample module A/D ADINT0 interrupt function Disabled"]
    _0 = 0,
    #[doc = "1: Specific sample module A/D ADINT0 interrupt function Enabled"]
    _1 = 1,
}
impl From<ADCIEN0_A> for bool {
    #[inline(always)]
    fn from(variant: ADCIEN0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCIEN0` reader - Specific Sample Module A/D ADINT0 Interrupt Enable Bit\\nThe A/D converter generates a conversion end ADIF0 (EADC_STATUS2\\[0\\]) upon the end of specific sample module A/D conversion. If ADCIEN0 bit is set then conversion end interrupt request ADINT0 is generated."]
pub struct ADCIEN0_R(crate::FieldReader<bool, ADCIEN0_A>);
impl ADCIEN0_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADCIEN0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCIEN0_A {
        match self.bits {
            false => ADCIEN0_A::_0,
            true => ADCIEN0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ADCIEN0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ADCIEN0_A::_1
    }
}
impl core::ops::Deref for ADCIEN0_R {
    type Target = crate::FieldReader<bool, ADCIEN0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADCIEN0` writer - Specific Sample Module A/D ADINT0 Interrupt Enable Bit\\nThe A/D converter generates a conversion end ADIF0 (EADC_STATUS2\\[0\\]) upon the end of specific sample module A/D conversion. If ADCIEN0 bit is set then conversion end interrupt request ADINT0 is generated."]
pub struct ADCIEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCIEN0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCIEN0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Specific sample module A/D ADINT0 interrupt function Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADCIEN0_A::_0)
    }
    #[doc = "Specific sample module A/D ADINT0 interrupt function Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADCIEN0_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Specific Sample Module A/D ADINT1 Interrupt Enable Bit\\nThe A/D converter generates a conversion end ADIF1 (EADC_STATUS2\\[1\\]) upon the end of specific sample module A/D conversion. If ADCIEN1 bit is set then conversion end interrupt request ADINT1 is generated.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCIEN1_A {
    #[doc = "0: Specific sample module A/D ADINT1 interrupt function Disabled"]
    _0 = 0,
    #[doc = "1: Specific sample module A/D ADINT1 interrupt function Enabled"]
    _1 = 1,
}
impl From<ADCIEN1_A> for bool {
    #[inline(always)]
    fn from(variant: ADCIEN1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCIEN1` reader - Specific Sample Module A/D ADINT1 Interrupt Enable Bit\\nThe A/D converter generates a conversion end ADIF1 (EADC_STATUS2\\[1\\]) upon the end of specific sample module A/D conversion. If ADCIEN1 bit is set then conversion end interrupt request ADINT1 is generated."]
pub struct ADCIEN1_R(crate::FieldReader<bool, ADCIEN1_A>);
impl ADCIEN1_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADCIEN1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCIEN1_A {
        match self.bits {
            false => ADCIEN1_A::_0,
            true => ADCIEN1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ADCIEN1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ADCIEN1_A::_1
    }
}
impl core::ops::Deref for ADCIEN1_R {
    type Target = crate::FieldReader<bool, ADCIEN1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADCIEN1` writer - Specific Sample Module A/D ADINT1 Interrupt Enable Bit\\nThe A/D converter generates a conversion end ADIF1 (EADC_STATUS2\\[1\\]) upon the end of specific sample module A/D conversion. If ADCIEN1 bit is set then conversion end interrupt request ADINT1 is generated."]
pub struct ADCIEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCIEN1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCIEN1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Specific sample module A/D ADINT1 interrupt function Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADCIEN1_A::_0)
    }
    #[doc = "Specific sample module A/D ADINT1 interrupt function Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADCIEN1_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Specific Sample Module A/D ADINT2 Interrupt Enable Bit\\nThe A/D converter generates a conversion end ADIF2 (EADC_STATUS2\\[2\\]) upon the end of specific sample module A/D conversion. If ADCIEN2 bit is set then conversion end interrupt request ADINT2 is generated.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCIEN2_A {
    #[doc = "0: Specific sample module A/D ADINT2 interrupt function Disabled"]
    _0 = 0,
    #[doc = "1: Specific sample module A/D ADINT2 interrupt function Enabled"]
    _1 = 1,
}
impl From<ADCIEN2_A> for bool {
    #[inline(always)]
    fn from(variant: ADCIEN2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCIEN2` reader - Specific Sample Module A/D ADINT2 Interrupt Enable Bit\\nThe A/D converter generates a conversion end ADIF2 (EADC_STATUS2\\[2\\]) upon the end of specific sample module A/D conversion. If ADCIEN2 bit is set then conversion end interrupt request ADINT2 is generated."]
pub struct ADCIEN2_R(crate::FieldReader<bool, ADCIEN2_A>);
impl ADCIEN2_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADCIEN2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCIEN2_A {
        match self.bits {
            false => ADCIEN2_A::_0,
            true => ADCIEN2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ADCIEN2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ADCIEN2_A::_1
    }
}
impl core::ops::Deref for ADCIEN2_R {
    type Target = crate::FieldReader<bool, ADCIEN2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADCIEN2` writer - Specific Sample Module A/D ADINT2 Interrupt Enable Bit\\nThe A/D converter generates a conversion end ADIF2 (EADC_STATUS2\\[2\\]) upon the end of specific sample module A/D conversion. If ADCIEN2 bit is set then conversion end interrupt request ADINT2 is generated."]
pub struct ADCIEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCIEN2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCIEN2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Specific sample module A/D ADINT2 interrupt function Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADCIEN2_A::_0)
    }
    #[doc = "Specific sample module A/D ADINT2 interrupt function Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADCIEN2_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Specific Sample Module A/D ADINT3 Interrupt Enable Bit\\nThe A/D converter generates a conversion end ADIF3 (EADC_STATUS2\\[3\\]) upon the end of specific sample module A/D conversion. If ADCIEN3 bit is set then conversion end interrupt request ADINT3 is generated.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCIEN3_A {
    #[doc = "0: Specific sample module A/D ADINT3 interrupt function Disabled"]
    _0 = 0,
    #[doc = "1: Specific sample module A/D ADINT3 interrupt function Enabled"]
    _1 = 1,
}
impl From<ADCIEN3_A> for bool {
    #[inline(always)]
    fn from(variant: ADCIEN3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCIEN3` reader - Specific Sample Module A/D ADINT3 Interrupt Enable Bit\\nThe A/D converter generates a conversion end ADIF3 (EADC_STATUS2\\[3\\]) upon the end of specific sample module A/D conversion. If ADCIEN3 bit is set then conversion end interrupt request ADINT3 is generated."]
pub struct ADCIEN3_R(crate::FieldReader<bool, ADCIEN3_A>);
impl ADCIEN3_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADCIEN3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCIEN3_A {
        match self.bits {
            false => ADCIEN3_A::_0,
            true => ADCIEN3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ADCIEN3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ADCIEN3_A::_1
    }
}
impl core::ops::Deref for ADCIEN3_R {
    type Target = crate::FieldReader<bool, ADCIEN3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADCIEN3` writer - Specific Sample Module A/D ADINT3 Interrupt Enable Bit\\nThe A/D converter generates a conversion end ADIF3 (EADC_STATUS2\\[3\\]) upon the end of specific sample module A/D conversion. If ADCIEN3 bit is set then conversion end interrupt request ADINT3 is generated."]
pub struct ADCIEN3_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCIEN3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCIEN3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Specific sample module A/D ADINT3 interrupt function Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADCIEN3_A::_0)
    }
    #[doc = "Specific sample module A/D ADINT3 interrupt function Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADCIEN3_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Differential Analog Input Mode Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIFFEN_A {
    #[doc = "0: Single-end analog input mode"]
    _0 = 0,
    #[doc = "1: Differential analog input mode"]
    _1 = 1,
}
impl From<DIFFEN_A> for bool {
    #[inline(always)]
    fn from(variant: DIFFEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIFFEN` reader - Differential Analog Input Mode Enable Bit"]
pub struct DIFFEN_R(crate::FieldReader<bool, DIFFEN_A>);
impl DIFFEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIFFEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIFFEN_A {
        match self.bits {
            false => DIFFEN_A::_0,
            true => DIFFEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DIFFEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DIFFEN_A::_1
    }
}
impl core::ops::Deref for DIFFEN_R {
    type Target = crate::FieldReader<bool, DIFFEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIFFEN` writer - Differential Analog Input Mode Enable Bit"]
pub struct DIFFEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DIFFEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIFFEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Single-end analog input mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DIFFEN_A::_0)
    }
    #[doc = "Differential analog input mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DIFFEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "ADC Differential Input Mode Output Format\\nNote: This bit must be set to 0 in single-end analog input mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMOF_A {
    #[doc = "0: A/D conversion result will be filled in RESULT (EADC_DATn\\[15:0\\]
, n= 0 ~18) with unsigned format"]
    _0 = 0,
    #[doc = "1: A/D conversion result will be filled in RESULT (EADC_DATn\\[15:0\\]
, n= 0 ~18) with 2'complement format"]
    _1 = 1,
}
impl From<DMOF_A> for bool {
    #[inline(always)]
    fn from(variant: DMOF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMOF` reader - ADC Differential Input Mode Output Format\\nNote: This bit must be set to 0 in single-end analog input mode."]
pub struct DMOF_R(crate::FieldReader<bool, DMOF_A>);
impl DMOF_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMOF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMOF_A {
        match self.bits {
            false => DMOF_A::_0,
            true => DMOF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DMOF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DMOF_A::_1
    }
}
impl core::ops::Deref for DMOF_R {
    type Target = crate::FieldReader<bool, DMOF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMOF` writer - ADC Differential Input Mode Output Format\\nNote: This bit must be set to 0 in single-end analog input mode."]
pub struct DMOF_W<'a> {
    w: &'a mut W,
}
impl<'a> DMOF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMOF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "A/D conversion result will be filled in RESULT (EADC_DATn\\[15:0\\]
, n= 0 ~18) with unsigned format"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DMOF_A::_0)
    }
    #[doc = "A/D conversion result will be filled in RESULT (EADC_DATn\\[15:0\\]
, n= 0 ~18) with 2'complement format"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DMOF_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "PDMA Transfer Enable Bit\\nWhen A/D conversion is completed, the converted data is loaded into EADC_DATn (n: 0 ~ 18) register, user can enable this bit to generate a PDMA data transfer request.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDMAEN_A {
    #[doc = "0: PDMA data transfer Disabled"]
    _0 = 0,
    #[doc = "1: PDMA data transfer Enabled"]
    _1 = 1,
}
impl From<PDMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: PDMAEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDMAEN` reader - PDMA Transfer Enable Bit\\nWhen A/D conversion is completed, the converted data is loaded into EADC_DATn (n: 0 ~ 18) register, user can enable this bit to generate a PDMA data transfer request."]
pub struct PDMAEN_R(crate::FieldReader<bool, PDMAEN_A>);
impl PDMAEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PDMAEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDMAEN_A {
        match self.bits {
            false => PDMAEN_A::_0,
            true => PDMAEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PDMAEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PDMAEN_A::_1
    }
}
impl core::ops::Deref for PDMAEN_R {
    type Target = crate::FieldReader<bool, PDMAEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDMAEN` writer - PDMA Transfer Enable Bit\\nWhen A/D conversion is completed, the converted data is loaded into EADC_DATn (n: 0 ~ 18) register, user can enable this bit to generate a PDMA data transfer request."]
pub struct PDMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PDMAEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDMAEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PDMA data transfer Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDMAEN_A::_0)
    }
    #[doc = "PDMA data transfer Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDMAEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "ADC Internal Sampling Time Selection\n\nValue on reset: 5"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SMPTSEL_A {
    #[doc = "0: 1 ADC clock sampling time"]
    _0 = 0,
    #[doc = "1: 2 ADC clock sampling time"]
    _1 = 1,
    #[doc = "2: 3 ADC clock sampling time"]
    _2 = 2,
    #[doc = "3: 4 ADC clock sampling time"]
    _3 = 3,
    #[doc = "4: 5 ADC clock sampling time"]
    _4 = 4,
    #[doc = "5: 6 ADC clock sampling time"]
    _5 = 5,
    #[doc = "6: 7 ADC clock sampling time"]
    _6 = 6,
    #[doc = "7: 8 ADC clock sampling time"]
    _7 = 7,
}
impl From<SMPTSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SMPTSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SMPTSEL` reader - ADC Internal Sampling Time Selection"]
pub struct SMPTSEL_R(crate::FieldReader<u8, SMPTSEL_A>);
impl SMPTSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        SMPTSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMPTSEL_A {
        match self.bits {
            0 => SMPTSEL_A::_0,
            1 => SMPTSEL_A::_1,
            2 => SMPTSEL_A::_2,
            3 => SMPTSEL_A::_3,
            4 => SMPTSEL_A::_4,
            5 => SMPTSEL_A::_5,
            6 => SMPTSEL_A::_6,
            7 => SMPTSEL_A::_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SMPTSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SMPTSEL_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == SMPTSEL_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == SMPTSEL_A::_3
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        **self == SMPTSEL_A::_4
    }
    #[doc = "Checks if the value of the field is `_5`"]
    #[inline(always)]
    pub fn is_5(&self) -> bool {
        **self == SMPTSEL_A::_5
    }
    #[doc = "Checks if the value of the field is `_6`"]
    #[inline(always)]
    pub fn is_6(&self) -> bool {
        **self == SMPTSEL_A::_6
    }
    #[doc = "Checks if the value of the field is `_7`"]
    #[inline(always)]
    pub fn is_7(&self) -> bool {
        **self == SMPTSEL_A::_7
    }
}
impl core::ops::Deref for SMPTSEL_R {
    type Target = crate::FieldReader<u8, SMPTSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMPTSEL` writer - ADC Internal Sampling Time Selection"]
pub struct SMPTSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SMPTSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMPTSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "1 ADC clock sampling time"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SMPTSEL_A::_0)
    }
    #[doc = "2 ADC clock sampling time"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SMPTSEL_A::_1)
    }
    #[doc = "3 ADC clock sampling time"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(SMPTSEL_A::_2)
    }
    #[doc = "4 ADC clock sampling time"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(SMPTSEL_A::_3)
    }
    #[doc = "5 ADC clock sampling time"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(SMPTSEL_A::_4)
    }
    #[doc = "6 ADC clock sampling time"]
    #[inline(always)]
    pub fn _5(self) -> &'a mut W {
        self.variant(SMPTSEL_A::_5)
    }
    #[doc = "7 ADC clock sampling time"]
    #[inline(always)]
    pub fn _6(self) -> &'a mut W {
        self.variant(SMPTSEL_A::_6)
    }
    #[doc = "8 ADC clock sampling time"]
    #[inline(always)]
    pub fn _7(self) -> &'a mut W {
        self.variant(SMPTSEL_A::_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | ((value as u32 & 0x07) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - A/D Converter Enable Bit\\nNote: Before starting A/D conversion function, this bit should be set to 1. Clear it to 0 to disable A/D converter analog circuit power consumption."]
    #[inline(always)]
    pub fn adcen(&self) -> ADCEN_R {
        ADCEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ADC A/D Converter Control Circuits Reset\\nNote: ADCRST bit remains 1 during ADC reset, when ADC reset end, the ADCRST bit is automatically cleared to 0."]
    #[inline(always)]
    pub fn adcrst(&self) -> ADCRST_R {
        ADCRST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Specific Sample Module A/D ADINT0 Interrupt Enable Bit\\nThe A/D converter generates a conversion end ADIF0 (EADC_STATUS2\\[0\\]) upon the end of specific sample module A/D conversion. If ADCIEN0 bit is set then conversion end interrupt request ADINT0 is generated."]
    #[inline(always)]
    pub fn adcien0(&self) -> ADCIEN0_R {
        ADCIEN0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Specific Sample Module A/D ADINT1 Interrupt Enable Bit\\nThe A/D converter generates a conversion end ADIF1 (EADC_STATUS2\\[1\\]) upon the end of specific sample module A/D conversion. If ADCIEN1 bit is set then conversion end interrupt request ADINT1 is generated."]
    #[inline(always)]
    pub fn adcien1(&self) -> ADCIEN1_R {
        ADCIEN1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Specific Sample Module A/D ADINT2 Interrupt Enable Bit\\nThe A/D converter generates a conversion end ADIF2 (EADC_STATUS2\\[2\\]) upon the end of specific sample module A/D conversion. If ADCIEN2 bit is set then conversion end interrupt request ADINT2 is generated."]
    #[inline(always)]
    pub fn adcien2(&self) -> ADCIEN2_R {
        ADCIEN2_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Specific Sample Module A/D ADINT3 Interrupt Enable Bit\\nThe A/D converter generates a conversion end ADIF3 (EADC_STATUS2\\[3\\]) upon the end of specific sample module A/D conversion. If ADCIEN3 bit is set then conversion end interrupt request ADINT3 is generated."]
    #[inline(always)]
    pub fn adcien3(&self) -> ADCIEN3_R {
        ADCIEN3_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Differential Analog Input Mode Enable Bit"]
    #[inline(always)]
    pub fn diffen(&self) -> DIFFEN_R {
        DIFFEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - ADC Differential Input Mode Output Format\\nNote: This bit must be set to 0 in single-end analog input mode."]
    #[inline(always)]
    pub fn dmof(&self) -> DMOF_R {
        DMOF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 11 - PDMA Transfer Enable Bit\\nWhen A/D conversion is completed, the converted data is loaded into EADC_DATn (n: 0 ~ 18) register, user can enable this bit to generate a PDMA data transfer request."]
    #[inline(always)]
    pub fn pdmaen(&self) -> PDMAEN_R {
        PDMAEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 16:18 - ADC Internal Sampling Time Selection"]
    #[inline(always)]
    pub fn smptsel(&self) -> SMPTSEL_R {
        SMPTSEL_R::new(((self.bits >> 16) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - A/D Converter Enable Bit\\nNote: Before starting A/D conversion function, this bit should be set to 1. Clear it to 0 to disable A/D converter analog circuit power consumption."]
    #[inline(always)]
    pub fn adcen(&mut self) -> ADCEN_W {
        ADCEN_W { w: self }
    }
    #[doc = "Bit 1 - ADC A/D Converter Control Circuits Reset\\nNote: ADCRST bit remains 1 during ADC reset, when ADC reset end, the ADCRST bit is automatically cleared to 0."]
    #[inline(always)]
    pub fn adcrst(&mut self) -> ADCRST_W {
        ADCRST_W { w: self }
    }
    #[doc = "Bit 2 - Specific Sample Module A/D ADINT0 Interrupt Enable Bit\\nThe A/D converter generates a conversion end ADIF0 (EADC_STATUS2\\[0\\]) upon the end of specific sample module A/D conversion. If ADCIEN0 bit is set then conversion end interrupt request ADINT0 is generated."]
    #[inline(always)]
    pub fn adcien0(&mut self) -> ADCIEN0_W {
        ADCIEN0_W { w: self }
    }
    #[doc = "Bit 3 - Specific Sample Module A/D ADINT1 Interrupt Enable Bit\\nThe A/D converter generates a conversion end ADIF1 (EADC_STATUS2\\[1\\]) upon the end of specific sample module A/D conversion. If ADCIEN1 bit is set then conversion end interrupt request ADINT1 is generated."]
    #[inline(always)]
    pub fn adcien1(&mut self) -> ADCIEN1_W {
        ADCIEN1_W { w: self }
    }
    #[doc = "Bit 4 - Specific Sample Module A/D ADINT2 Interrupt Enable Bit\\nThe A/D converter generates a conversion end ADIF2 (EADC_STATUS2\\[2\\]) upon the end of specific sample module A/D conversion. If ADCIEN2 bit is set then conversion end interrupt request ADINT2 is generated."]
    #[inline(always)]
    pub fn adcien2(&mut self) -> ADCIEN2_W {
        ADCIEN2_W { w: self }
    }
    #[doc = "Bit 5 - Specific Sample Module A/D ADINT3 Interrupt Enable Bit\\nThe A/D converter generates a conversion end ADIF3 (EADC_STATUS2\\[3\\]) upon the end of specific sample module A/D conversion. If ADCIEN3 bit is set then conversion end interrupt request ADINT3 is generated."]
    #[inline(always)]
    pub fn adcien3(&mut self) -> ADCIEN3_W {
        ADCIEN3_W { w: self }
    }
    #[doc = "Bit 8 - Differential Analog Input Mode Enable Bit"]
    #[inline(always)]
    pub fn diffen(&mut self) -> DIFFEN_W {
        DIFFEN_W { w: self }
    }
    #[doc = "Bit 9 - ADC Differential Input Mode Output Format\\nNote: This bit must be set to 0 in single-end analog input mode."]
    #[inline(always)]
    pub fn dmof(&mut self) -> DMOF_W {
        DMOF_W { w: self }
    }
    #[doc = "Bit 11 - PDMA Transfer Enable Bit\\nWhen A/D conversion is completed, the converted data is loaded into EADC_DATn (n: 0 ~ 18) register, user can enable this bit to generate a PDMA data transfer request."]
    #[inline(always)]
    pub fn pdmaen(&mut self) -> PDMAEN_W {
        PDMAEN_W { w: self }
    }
    #[doc = "Bits 16:18 - ADC Internal Sampling Time Selection"]
    #[inline(always)]
    pub fn smptsel(&mut self) -> SMPTSEL_W {
        SMPTSEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A/D Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eadc_ctl](index.html) module"]
pub struct EADC_CTL_SPEC;
impl crate::RegisterSpec for EADC_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eadc_ctl::R](R) reader structure"]
impl crate::Readable for EADC_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eadc_ctl::W](W) writer structure"]
impl crate::Writable for EADC_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EADC_CTL to value 0x0005_0000"]
impl crate::Resettable for EADC_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0005_0000
    }
}
