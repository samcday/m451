#[doc = "Register `PWM_SYNC` reader"]
pub struct R(crate::R<PWM_SYNC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM_SYNC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PWM_SYNC_SPEC>> for R {
    fn from(reader: crate::R<PWM_SYNC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWM_SYNC` writer"]
pub struct W(crate::W<PWM_SYNC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWM_SYNC_SPEC>;
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
impl core::convert::From<crate::W<PWM_SYNC_SPEC>> for W {
    fn from(writer: crate::W<PWM_SYNC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "SYNC Phase Enable Bit\\nEach bit n controls corresponding PWM channel n.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PHSENN_A {
    #[doc = "0: PWM counter load PHS value Disabled"]
    _0 = 0,
    #[doc = "1: PWM counter load PHS value Enabled"]
    _1 = 1,
}
impl From<PHSENN_A> for u8 {
    #[inline(always)]
    fn from(variant: PHSENN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PHSENn` reader - SYNC Phase Enable Bit\\nEach bit n controls corresponding PWM channel n."]
pub struct PHSENN_R(crate::FieldReader<u8, PHSENN_A>);
impl PHSENN_R {
    pub(crate) fn new(bits: u8) -> Self {
        PHSENN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PHSENN_A> {
        match self.bits {
            0 => Some(PHSENN_A::_0),
            1 => Some(PHSENN_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PHSENN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PHSENN_A::_1
    }
}
impl core::ops::Deref for PHSENN_R {
    type Target = crate::FieldReader<u8, PHSENN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PHSENn` writer - SYNC Phase Enable Bit\\nEach bit n controls corresponding PWM channel n."]
pub struct PHSENN_W<'a> {
    w: &'a mut W,
}
impl<'a> PHSENN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PHSENN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "PWM counter load PHS value Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PHSENN_A::_0)
    }
    #[doc = "PWM counter load PHS value Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PHSENN_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "PWM0_SYNC_IN Source Selection\\nEach bit n controls corresponding PWM channel n.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SINSRCN_A {
    #[doc = "0: Synchronize source from SYNC_IN or SWSYNC"]
    _0 = 0,
    #[doc = "1: Counter equal to 0"]
    _1 = 1,
    #[doc = "10: Counter equal to PWM_CMPDATm, m denotes 1, 3, 5"]
    _10 = 10,
    #[doc = "11: SYNC_OUT will not be generated"]
    _11 = 11,
}
impl From<SINSRCN_A> for u8 {
    #[inline(always)]
    fn from(variant: SINSRCN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SINSRCn` reader - PWM0_SYNC_IN Source Selection\\nEach bit n controls corresponding PWM channel n."]
pub struct SINSRCN_R(crate::FieldReader<u8, SINSRCN_A>);
impl SINSRCN_R {
    pub(crate) fn new(bits: u8) -> Self {
        SINSRCN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SINSRCN_A> {
        match self.bits {
            0 => Some(SINSRCN_A::_0),
            1 => Some(SINSRCN_A::_1),
            10 => Some(SINSRCN_A::_10),
            11 => Some(SINSRCN_A::_11),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SINSRCN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SINSRCN_A::_1
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        **self == SINSRCN_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == SINSRCN_A::_11
    }
}
impl core::ops::Deref for SINSRCN_R {
    type Target = crate::FieldReader<u8, SINSRCN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SINSRCn` writer - PWM0_SYNC_IN Source Selection\\nEach bit n controls corresponding PWM channel n."]
pub struct SINSRCN_W<'a> {
    w: &'a mut W,
}
impl<'a> SINSRCN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SINSRCN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Synchronize source from SYNC_IN or SWSYNC"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SINSRCN_A::_0)
    }
    #[doc = "Counter equal to 0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SINSRCN_A::_1)
    }
    #[doc = "Counter equal to PWM_CMPDATm, m denotes 1, 3, 5"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(SINSRCN_A::_10)
    }
    #[doc = "SYNC_OUT will not be generated"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(SINSRCN_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | ((value as u32 & 0x3f) << 8);
        self.w
    }
}
#[doc = "PWM0_SYNC_IN Noise Filter Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SNFLTEN_A {
    #[doc = "0: Noise filter of input pin PWM0_SYNC_IN Disabled"]
    _0 = 0,
    #[doc = "1: Noise filter of input pin PWM0_SYNC_IN Enabled"]
    _1 = 1,
}
impl From<SNFLTEN_A> for bool {
    #[inline(always)]
    fn from(variant: SNFLTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SNFLTEN` reader - PWM0_SYNC_IN Noise Filter Enable Bit"]
pub struct SNFLTEN_R(crate::FieldReader<bool, SNFLTEN_A>);
impl SNFLTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SNFLTEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SNFLTEN_A {
        match self.bits {
            false => SNFLTEN_A::_0,
            true => SNFLTEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SNFLTEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SNFLTEN_A::_1
    }
}
impl core::ops::Deref for SNFLTEN_R {
    type Target = crate::FieldReader<bool, SNFLTEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SNFLTEN` writer - PWM0_SYNC_IN Noise Filter Enable Bit"]
pub struct SNFLTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SNFLTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SNFLTEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Noise filter of input pin PWM0_SYNC_IN Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SNFLTEN_A::_0)
    }
    #[doc = "Noise filter of input pin PWM0_SYNC_IN Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SNFLTEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "SYNC Edge Detector Filter Clock Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SFLTCSEL_A {
    #[doc = "0: Filter clock = HCLK"]
    _0 = 0,
    #[doc = "1: Filter clock = HCLK/2"]
    _1 = 1,
    #[doc = "2: Filter clock = HCLK/4"]
    _2 = 2,
    #[doc = "3: Filter clock = HCLK/8"]
    _3 = 3,
    #[doc = "4: Filter clock = HCLK/16"]
    _4 = 4,
    #[doc = "5: Filter clock = HCLK/32"]
    _5 = 5,
    #[doc = "6: Filter clock = HCLK/64"]
    _6 = 6,
    #[doc = "7: Filter clock = HCLK/128"]
    _7 = 7,
}
impl From<SFLTCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SFLTCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SFLTCSEL` reader - SYNC Edge Detector Filter Clock Selection"]
pub struct SFLTCSEL_R(crate::FieldReader<u8, SFLTCSEL_A>);
impl SFLTCSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        SFLTCSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SFLTCSEL_A {
        match self.bits {
            0 => SFLTCSEL_A::_0,
            1 => SFLTCSEL_A::_1,
            2 => SFLTCSEL_A::_2,
            3 => SFLTCSEL_A::_3,
            4 => SFLTCSEL_A::_4,
            5 => SFLTCSEL_A::_5,
            6 => SFLTCSEL_A::_6,
            7 => SFLTCSEL_A::_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SFLTCSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SFLTCSEL_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == SFLTCSEL_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == SFLTCSEL_A::_3
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        **self == SFLTCSEL_A::_4
    }
    #[doc = "Checks if the value of the field is `_5`"]
    #[inline(always)]
    pub fn is_5(&self) -> bool {
        **self == SFLTCSEL_A::_5
    }
    #[doc = "Checks if the value of the field is `_6`"]
    #[inline(always)]
    pub fn is_6(&self) -> bool {
        **self == SFLTCSEL_A::_6
    }
    #[doc = "Checks if the value of the field is `_7`"]
    #[inline(always)]
    pub fn is_7(&self) -> bool {
        **self == SFLTCSEL_A::_7
    }
}
impl core::ops::Deref for SFLTCSEL_R {
    type Target = crate::FieldReader<u8, SFLTCSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SFLTCSEL` writer - SYNC Edge Detector Filter Clock Selection"]
pub struct SFLTCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SFLTCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SFLTCSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Filter clock = HCLK"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SFLTCSEL_A::_0)
    }
    #[doc = "Filter clock = HCLK/2"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SFLTCSEL_A::_1)
    }
    #[doc = "Filter clock = HCLK/4"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(SFLTCSEL_A::_2)
    }
    #[doc = "Filter clock = HCLK/8"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(SFLTCSEL_A::_3)
    }
    #[doc = "Filter clock = HCLK/16"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(SFLTCSEL_A::_4)
    }
    #[doc = "Filter clock = HCLK/32"]
    #[inline(always)]
    pub fn _5(self) -> &'a mut W {
        self.variant(SFLTCSEL_A::_5)
    }
    #[doc = "Filter clock = HCLK/64"]
    #[inline(always)]
    pub fn _6(self) -> &'a mut W {
        self.variant(SFLTCSEL_A::_6)
    }
    #[doc = "Filter clock = HCLK/128"]
    #[inline(always)]
    pub fn _7(self) -> &'a mut W {
        self.variant(SFLTCSEL_A::_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 17)) | ((value as u32 & 0x07) << 17);
        self.w
    }
}
#[doc = "Field `SFLTCNT` reader - SYNC Edge Detector Filter Count\\nThe register bits control the counter number of edge detector."]
pub struct SFLTCNT_R(crate::FieldReader<u8, u8>);
impl SFLTCNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        SFLTCNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SFLTCNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SFLTCNT` writer - SYNC Edge Detector Filter Count\\nThe register bits control the counter number of edge detector."]
pub struct SFLTCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> SFLTCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | ((value as u32 & 0x07) << 20);
        self.w
    }
}
#[doc = "SYNC Input Pin Inverse\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SINPINV_A {
    #[doc = "0: The state of pin SYNC is passed to the positive edge detector"]
    _0 = 0,
    #[doc = "1: The inversed state of pin SYNC is passed to the positive edge detector"]
    _1 = 1,
}
impl From<SINPINV_A> for bool {
    #[inline(always)]
    fn from(variant: SINPINV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SINPINV` reader - SYNC Input Pin Inverse"]
pub struct SINPINV_R(crate::FieldReader<bool, SINPINV_A>);
impl SINPINV_R {
    pub(crate) fn new(bits: bool) -> Self {
        SINPINV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SINPINV_A {
        match self.bits {
            false => SINPINV_A::_0,
            true => SINPINV_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SINPINV_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SINPINV_A::_1
    }
}
impl core::ops::Deref for SINPINV_R {
    type Target = crate::FieldReader<bool, SINPINV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SINPINV` writer - SYNC Input Pin Inverse"]
pub struct SINPINV_W<'a> {
    w: &'a mut W,
}
impl<'a> SINPINV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SINPINV_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The state of pin SYNC is passed to the positive edge detector"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SINPINV_A::_0)
    }
    #[doc = "The inversed state of pin SYNC is passed to the positive edge detector"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SINPINV_A::_1)
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
#[doc = "PWM Phase Direction Control\\nEach bit n controls corresponding PWM channel n.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PHSDIRN_A {
    #[doc = "0: Control PWM counter count decrement after synchronizing"]
    _0 = 0,
    #[doc = "1: Control PWM counter count increment after synchronizing"]
    _1 = 1,
}
impl From<PHSDIRN_A> for u8 {
    #[inline(always)]
    fn from(variant: PHSDIRN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PHSDIRn` reader - PWM Phase Direction Control\\nEach bit n controls corresponding PWM channel n."]
pub struct PHSDIRN_R(crate::FieldReader<u8, PHSDIRN_A>);
impl PHSDIRN_R {
    pub(crate) fn new(bits: u8) -> Self {
        PHSDIRN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PHSDIRN_A> {
        match self.bits {
            0 => Some(PHSDIRN_A::_0),
            1 => Some(PHSDIRN_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PHSDIRN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PHSDIRN_A::_1
    }
}
impl core::ops::Deref for PHSDIRN_R {
    type Target = crate::FieldReader<u8, PHSDIRN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PHSDIRn` writer - PWM Phase Direction Control\\nEach bit n controls corresponding PWM channel n."]
pub struct PHSDIRN_W<'a> {
    w: &'a mut W,
}
impl<'a> PHSDIRN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PHSDIRN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Control PWM counter count decrement after synchronizing"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PHSDIRN_A::_0)
    }
    #[doc = "Control PWM counter count increment after synchronizing"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PHSDIRN_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | ((value as u32 & 0x07) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - SYNC Phase Enable Bit\\nEach bit n controls corresponding PWM channel n."]
    #[inline(always)]
    pub fn phsenn(&self) -> PHSENN_R {
        PHSENN_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 8:13 - PWM0_SYNC_IN Source Selection\\nEach bit n controls corresponding PWM channel n."]
    #[inline(always)]
    pub fn sinsrcn(&self) -> SINSRCN_R {
        SINSRCN_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - PWM0_SYNC_IN Noise Filter Enable Bit"]
    #[inline(always)]
    pub fn snflten(&self) -> SNFLTEN_R {
        SNFLTEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 17:19 - SYNC Edge Detector Filter Clock Selection"]
    #[inline(always)]
    pub fn sfltcsel(&self) -> SFLTCSEL_R {
        SFLTCSEL_R::new(((self.bits >> 17) & 0x07) as u8)
    }
    #[doc = "Bits 20:22 - SYNC Edge Detector Filter Count\\nThe register bits control the counter number of edge detector."]
    #[inline(always)]
    pub fn sfltcnt(&self) -> SFLTCNT_R {
        SFLTCNT_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bit 23 - SYNC Input Pin Inverse"]
    #[inline(always)]
    pub fn sinpinv(&self) -> SINPINV_R {
        SINPINV_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:26 - PWM Phase Direction Control\\nEach bit n controls corresponding PWM channel n."]
    #[inline(always)]
    pub fn phsdirn(&self) -> PHSDIRN_R {
        PHSDIRN_R::new(((self.bits >> 24) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - SYNC Phase Enable Bit\\nEach bit n controls corresponding PWM channel n."]
    #[inline(always)]
    pub fn phsenn(&mut self) -> PHSENN_W {
        PHSENN_W { w: self }
    }
    #[doc = "Bits 8:13 - PWM0_SYNC_IN Source Selection\\nEach bit n controls corresponding PWM channel n."]
    #[inline(always)]
    pub fn sinsrcn(&mut self) -> SINSRCN_W {
        SINSRCN_W { w: self }
    }
    #[doc = "Bit 16 - PWM0_SYNC_IN Noise Filter Enable Bit"]
    #[inline(always)]
    pub fn snflten(&mut self) -> SNFLTEN_W {
        SNFLTEN_W { w: self }
    }
    #[doc = "Bits 17:19 - SYNC Edge Detector Filter Clock Selection"]
    #[inline(always)]
    pub fn sfltcsel(&mut self) -> SFLTCSEL_W {
        SFLTCSEL_W { w: self }
    }
    #[doc = "Bits 20:22 - SYNC Edge Detector Filter Count\\nThe register bits control the counter number of edge detector."]
    #[inline(always)]
    pub fn sfltcnt(&mut self) -> SFLTCNT_W {
        SFLTCNT_W { w: self }
    }
    #[doc = "Bit 23 - SYNC Input Pin Inverse"]
    #[inline(always)]
    pub fn sinpinv(&mut self) -> SINPINV_W {
        SINPINV_W { w: self }
    }
    #[doc = "Bits 24:26 - PWM Phase Direction Control\\nEach bit n controls corresponding PWM channel n."]
    #[inline(always)]
    pub fn phsdirn(&mut self) -> PHSDIRN_W {
        PHSDIRN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Synchronization Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_sync](index.html) module"]
pub struct PWM_SYNC_SPEC;
impl crate::RegisterSpec for PWM_SYNC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwm_sync::R](R) reader structure"]
impl crate::Readable for PWM_SYNC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwm_sync::W](W) writer structure"]
impl crate::Writable for PWM_SYNC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWM_SYNC to value 0"]
impl crate::Resettable for PWM_SYNC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
