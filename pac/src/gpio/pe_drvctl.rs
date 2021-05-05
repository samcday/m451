#[doc = "Register `PE_DRVCTL` reader"]
pub struct R(crate::R<PE_DRVCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PE_DRVCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PE_DRVCTL_SPEC>> for R {
    fn from(reader: crate::R<PE_DRVCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PE_DRVCTL` writer"]
pub struct W(crate::W<PE_DRVCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PE_DRVCTL_SPEC>;
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
impl core::convert::From<crate::W<PE_DRVCTL_SPEC>> for W {
    fn from(writer: crate::W<PE_DRVCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Port E Pin\\[N\\]
Driving Strength Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HDRVEN8_A {
    #[doc = "0: Px.n output with basic driving strength"]
    _0 = 0,
    #[doc = "1: Px.n output with high driving strength"]
    _1 = 1,
}
impl From<HDRVEN8_A> for bool {
    #[inline(always)]
    fn from(variant: HDRVEN8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HDRVEN8` reader - Port E Pin\\[N\\]
Driving Strength Control"]
pub struct HDRVEN8_R(crate::FieldReader<bool, HDRVEN8_A>);
impl HDRVEN8_R {
    pub(crate) fn new(bits: bool) -> Self {
        HDRVEN8_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HDRVEN8_A {
        match self.bits {
            false => HDRVEN8_A::_0,
            true => HDRVEN8_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == HDRVEN8_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == HDRVEN8_A::_1
    }
}
impl core::ops::Deref for HDRVEN8_R {
    type Target = crate::FieldReader<bool, HDRVEN8_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HDRVEN8` writer - Port E Pin\\[N\\]
Driving Strength Control"]
pub struct HDRVEN8_W<'a> {
    w: &'a mut W,
}
impl<'a> HDRVEN8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HDRVEN8_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n output with basic driving strength"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HDRVEN8_A::_0)
    }
    #[doc = "Px.n output with high driving strength"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HDRVEN8_A::_1)
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
#[doc = "Port E Pin\\[N\\]
Driving Strength Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HDRVEN9_A {
    #[doc = "0: Px.n output with basic driving strength"]
    _0 = 0,
    #[doc = "1: Px.n output with high driving strength"]
    _1 = 1,
}
impl From<HDRVEN9_A> for bool {
    #[inline(always)]
    fn from(variant: HDRVEN9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HDRVEN9` reader - Port E Pin\\[N\\]
Driving Strength Control"]
pub struct HDRVEN9_R(crate::FieldReader<bool, HDRVEN9_A>);
impl HDRVEN9_R {
    pub(crate) fn new(bits: bool) -> Self {
        HDRVEN9_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HDRVEN9_A {
        match self.bits {
            false => HDRVEN9_A::_0,
            true => HDRVEN9_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == HDRVEN9_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == HDRVEN9_A::_1
    }
}
impl core::ops::Deref for HDRVEN9_R {
    type Target = crate::FieldReader<bool, HDRVEN9_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HDRVEN9` writer - Port E Pin\\[N\\]
Driving Strength Control"]
pub struct HDRVEN9_W<'a> {
    w: &'a mut W,
}
impl<'a> HDRVEN9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HDRVEN9_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n output with basic driving strength"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HDRVEN9_A::_0)
    }
    #[doc = "Px.n output with high driving strength"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HDRVEN9_A::_1)
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
#[doc = "Port E Pin\\[N\\]
Driving Strength Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HDRVEN10_A {
    #[doc = "0: Px.n output with basic driving strength"]
    _0 = 0,
    #[doc = "1: Px.n output with high driving strength"]
    _1 = 1,
}
impl From<HDRVEN10_A> for bool {
    #[inline(always)]
    fn from(variant: HDRVEN10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HDRVEN10` reader - Port E Pin\\[N\\]
Driving Strength Control"]
pub struct HDRVEN10_R(crate::FieldReader<bool, HDRVEN10_A>);
impl HDRVEN10_R {
    pub(crate) fn new(bits: bool) -> Self {
        HDRVEN10_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HDRVEN10_A {
        match self.bits {
            false => HDRVEN10_A::_0,
            true => HDRVEN10_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == HDRVEN10_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == HDRVEN10_A::_1
    }
}
impl core::ops::Deref for HDRVEN10_R {
    type Target = crate::FieldReader<bool, HDRVEN10_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HDRVEN10` writer - Port E Pin\\[N\\]
Driving Strength Control"]
pub struct HDRVEN10_W<'a> {
    w: &'a mut W,
}
impl<'a> HDRVEN10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HDRVEN10_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n output with basic driving strength"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HDRVEN10_A::_0)
    }
    #[doc = "Px.n output with high driving strength"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HDRVEN10_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Port E Pin\\[N\\]
Driving Strength Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HDRVEN11_A {
    #[doc = "0: Px.n output with basic driving strength"]
    _0 = 0,
    #[doc = "1: Px.n output with high driving strength"]
    _1 = 1,
}
impl From<HDRVEN11_A> for bool {
    #[inline(always)]
    fn from(variant: HDRVEN11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HDRVEN11` reader - Port E Pin\\[N\\]
Driving Strength Control"]
pub struct HDRVEN11_R(crate::FieldReader<bool, HDRVEN11_A>);
impl HDRVEN11_R {
    pub(crate) fn new(bits: bool) -> Self {
        HDRVEN11_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HDRVEN11_A {
        match self.bits {
            false => HDRVEN11_A::_0,
            true => HDRVEN11_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == HDRVEN11_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == HDRVEN11_A::_1
    }
}
impl core::ops::Deref for HDRVEN11_R {
    type Target = crate::FieldReader<bool, HDRVEN11_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HDRVEN11` writer - Port E Pin\\[N\\]
Driving Strength Control"]
pub struct HDRVEN11_W<'a> {
    w: &'a mut W,
}
impl<'a> HDRVEN11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HDRVEN11_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n output with basic driving strength"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HDRVEN11_A::_0)
    }
    #[doc = "Px.n output with high driving strength"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HDRVEN11_A::_1)
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
#[doc = "Port E Pin\\[N\\]
Driving Strength Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HDRVEN12_A {
    #[doc = "0: Px.n output with basic driving strength"]
    _0 = 0,
    #[doc = "1: Px.n output with high driving strength"]
    _1 = 1,
}
impl From<HDRVEN12_A> for bool {
    #[inline(always)]
    fn from(variant: HDRVEN12_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HDRVEN12` reader - Port E Pin\\[N\\]
Driving Strength Control"]
pub struct HDRVEN12_R(crate::FieldReader<bool, HDRVEN12_A>);
impl HDRVEN12_R {
    pub(crate) fn new(bits: bool) -> Self {
        HDRVEN12_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HDRVEN12_A {
        match self.bits {
            false => HDRVEN12_A::_0,
            true => HDRVEN12_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == HDRVEN12_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == HDRVEN12_A::_1
    }
}
impl core::ops::Deref for HDRVEN12_R {
    type Target = crate::FieldReader<bool, HDRVEN12_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HDRVEN12` writer - Port E Pin\\[N\\]
Driving Strength Control"]
pub struct HDRVEN12_W<'a> {
    w: &'a mut W,
}
impl<'a> HDRVEN12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HDRVEN12_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n output with basic driving strength"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HDRVEN12_A::_0)
    }
    #[doc = "Px.n output with high driving strength"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HDRVEN12_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Port E Pin\\[N\\]
Driving Strength Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HDRVEN13_A {
    #[doc = "0: Px.n output with basic driving strength"]
    _0 = 0,
    #[doc = "1: Px.n output with high driving strength"]
    _1 = 1,
}
impl From<HDRVEN13_A> for bool {
    #[inline(always)]
    fn from(variant: HDRVEN13_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HDRVEN13` reader - Port E Pin\\[N\\]
Driving Strength Control"]
pub struct HDRVEN13_R(crate::FieldReader<bool, HDRVEN13_A>);
impl HDRVEN13_R {
    pub(crate) fn new(bits: bool) -> Self {
        HDRVEN13_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HDRVEN13_A {
        match self.bits {
            false => HDRVEN13_A::_0,
            true => HDRVEN13_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == HDRVEN13_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == HDRVEN13_A::_1
    }
}
impl core::ops::Deref for HDRVEN13_R {
    type Target = crate::FieldReader<bool, HDRVEN13_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HDRVEN13` writer - Port E Pin\\[N\\]
Driving Strength Control"]
pub struct HDRVEN13_W<'a> {
    w: &'a mut W,
}
impl<'a> HDRVEN13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HDRVEN13_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n output with basic driving strength"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HDRVEN13_A::_0)
    }
    #[doc = "Px.n output with high driving strength"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HDRVEN13_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
impl R {
    #[doc = "Bit 8 - Port E Pin\\[N\\]
Driving Strength Control"]
    #[inline(always)]
    pub fn hdrven8(&self) -> HDRVEN8_R {
        HDRVEN8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Port E Pin\\[N\\]
Driving Strength Control"]
    #[inline(always)]
    pub fn hdrven9(&self) -> HDRVEN9_R {
        HDRVEN9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Port E Pin\\[N\\]
Driving Strength Control"]
    #[inline(always)]
    pub fn hdrven10(&self) -> HDRVEN10_R {
        HDRVEN10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Port E Pin\\[N\\]
Driving Strength Control"]
    #[inline(always)]
    pub fn hdrven11(&self) -> HDRVEN11_R {
        HDRVEN11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Port E Pin\\[N\\]
Driving Strength Control"]
    #[inline(always)]
    pub fn hdrven12(&self) -> HDRVEN12_R {
        HDRVEN12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Port E Pin\\[N\\]
Driving Strength Control"]
    #[inline(always)]
    pub fn hdrven13(&self) -> HDRVEN13_R {
        HDRVEN13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Port E Pin\\[N\\]
Driving Strength Control"]
    #[inline(always)]
    pub fn hdrven8(&mut self) -> HDRVEN8_W {
        HDRVEN8_W { w: self }
    }
    #[doc = "Bit 9 - Port E Pin\\[N\\]
Driving Strength Control"]
    #[inline(always)]
    pub fn hdrven9(&mut self) -> HDRVEN9_W {
        HDRVEN9_W { w: self }
    }
    #[doc = "Bit 10 - Port E Pin\\[N\\]
Driving Strength Control"]
    #[inline(always)]
    pub fn hdrven10(&mut self) -> HDRVEN10_W {
        HDRVEN10_W { w: self }
    }
    #[doc = "Bit 11 - Port E Pin\\[N\\]
Driving Strength Control"]
    #[inline(always)]
    pub fn hdrven11(&mut self) -> HDRVEN11_W {
        HDRVEN11_W { w: self }
    }
    #[doc = "Bit 12 - Port E Pin\\[N\\]
Driving Strength Control"]
    #[inline(always)]
    pub fn hdrven12(&mut self) -> HDRVEN12_W {
        HDRVEN12_W { w: self }
    }
    #[doc = "Bit 13 - Port E Pin\\[N\\]
Driving Strength Control"]
    #[inline(always)]
    pub fn hdrven13(&mut self) -> HDRVEN13_W {
        HDRVEN13_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PE High Drive Strength Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pe_drvctl](index.html) module"]
pub struct PE_DRVCTL_SPEC;
impl crate::RegisterSpec for PE_DRVCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pe_drvctl::R](R) reader structure"]
impl crate::Readable for PE_DRVCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pe_drvctl::W](W) writer structure"]
impl crate::Writable for PE_DRVCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PE_DRVCTL to value 0"]
impl crate::Resettable for PE_DRVCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
