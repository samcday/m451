#[doc = "Register `PF_DINOFF` reader"]
pub struct R(crate::R<PF_DINOFF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PF_DINOFF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PF_DINOFF_SPEC>> for R {
    fn from(reader: crate::R<PF_DINOFF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PF_DINOFF` writer"]
pub struct W(crate::W<PF_DINOFF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PF_DINOFF_SPEC>;
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
impl core::convert::From<crate::W<PF_DINOFF_SPEC>> for W {
    fn from(writer: crate::W<PF_DINOFF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Port A-f Pin\\[N\\]
Digital Input Path Disable Control\\nEach of these bits is used to control if the digital input path of corresponding Px.n pin is disabled. If input is analog signal, users can disable Px.n digital input path to avoid input current leakage.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DINOFF0_A {
    #[doc = "0: Px.n digital input path Enabled"]
    _0 = 0,
    #[doc = "1: Px.n digital input path Disabled (digital input tied to low)"]
    _1 = 1,
}
impl From<DINOFF0_A> for bool {
    #[inline(always)]
    fn from(variant: DINOFF0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DINOFF0` reader - Port A-f Pin\\[N\\]
Digital Input Path Disable Control\\nEach of these bits is used to control if the digital input path of corresponding Px.n pin is disabled. If input is analog signal, users can disable Px.n digital input path to avoid input current leakage.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DINOFF0_R(crate::FieldReader<bool, DINOFF0_A>);
impl DINOFF0_R {
    pub(crate) fn new(bits: bool) -> Self {
        DINOFF0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DINOFF0_A {
        match self.bits {
            false => DINOFF0_A::_0,
            true => DINOFF0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DINOFF0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DINOFF0_A::_1
    }
}
impl core::ops::Deref for DINOFF0_R {
    type Target = crate::FieldReader<bool, DINOFF0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DINOFF0` writer - Port A-f Pin\\[N\\]
Digital Input Path Disable Control\\nEach of these bits is used to control if the digital input path of corresponding Px.n pin is disabled. If input is analog signal, users can disable Px.n digital input path to avoid input current leakage.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DINOFF0_W<'a> {
    w: &'a mut W,
}
impl<'a> DINOFF0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DINOFF0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n digital input path Enabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DINOFF0_A::_0)
    }
    #[doc = "Px.n digital input path Disabled (digital input tied to low)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DINOFF0_A::_1)
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
#[doc = "Port A-f Pin\\[N\\]
Digital Input Path Disable Control\\nEach of these bits is used to control if the digital input path of corresponding Px.n pin is disabled. If input is analog signal, users can disable Px.n digital input path to avoid input current leakage.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DINOFF1_A {
    #[doc = "0: Px.n digital input path Enabled"]
    _0 = 0,
    #[doc = "1: Px.n digital input path Disabled (digital input tied to low)"]
    _1 = 1,
}
impl From<DINOFF1_A> for bool {
    #[inline(always)]
    fn from(variant: DINOFF1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DINOFF1` reader - Port A-f Pin\\[N\\]
Digital Input Path Disable Control\\nEach of these bits is used to control if the digital input path of corresponding Px.n pin is disabled. If input is analog signal, users can disable Px.n digital input path to avoid input current leakage.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DINOFF1_R(crate::FieldReader<bool, DINOFF1_A>);
impl DINOFF1_R {
    pub(crate) fn new(bits: bool) -> Self {
        DINOFF1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DINOFF1_A {
        match self.bits {
            false => DINOFF1_A::_0,
            true => DINOFF1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DINOFF1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DINOFF1_A::_1
    }
}
impl core::ops::Deref for DINOFF1_R {
    type Target = crate::FieldReader<bool, DINOFF1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DINOFF1` writer - Port A-f Pin\\[N\\]
Digital Input Path Disable Control\\nEach of these bits is used to control if the digital input path of corresponding Px.n pin is disabled. If input is analog signal, users can disable Px.n digital input path to avoid input current leakage.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DINOFF1_W<'a> {
    w: &'a mut W,
}
impl<'a> DINOFF1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DINOFF1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n digital input path Enabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DINOFF1_A::_0)
    }
    #[doc = "Px.n digital input path Disabled (digital input tied to low)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DINOFF1_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Port A-f Pin\\[N\\]
Digital Input Path Disable Control\\nEach of these bits is used to control if the digital input path of corresponding Px.n pin is disabled. If input is analog signal, users can disable Px.n digital input path to avoid input current leakage.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DINOFF2_A {
    #[doc = "0: Px.n digital input path Enabled"]
    _0 = 0,
    #[doc = "1: Px.n digital input path Disabled (digital input tied to low)"]
    _1 = 1,
}
impl From<DINOFF2_A> for bool {
    #[inline(always)]
    fn from(variant: DINOFF2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DINOFF2` reader - Port A-f Pin\\[N\\]
Digital Input Path Disable Control\\nEach of these bits is used to control if the digital input path of corresponding Px.n pin is disabled. If input is analog signal, users can disable Px.n digital input path to avoid input current leakage.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DINOFF2_R(crate::FieldReader<bool, DINOFF2_A>);
impl DINOFF2_R {
    pub(crate) fn new(bits: bool) -> Self {
        DINOFF2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DINOFF2_A {
        match self.bits {
            false => DINOFF2_A::_0,
            true => DINOFF2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DINOFF2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DINOFF2_A::_1
    }
}
impl core::ops::Deref for DINOFF2_R {
    type Target = crate::FieldReader<bool, DINOFF2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DINOFF2` writer - Port A-f Pin\\[N\\]
Digital Input Path Disable Control\\nEach of these bits is used to control if the digital input path of corresponding Px.n pin is disabled. If input is analog signal, users can disable Px.n digital input path to avoid input current leakage.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DINOFF2_W<'a> {
    w: &'a mut W,
}
impl<'a> DINOFF2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DINOFF2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n digital input path Enabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DINOFF2_A::_0)
    }
    #[doc = "Px.n digital input path Disabled (digital input tied to low)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DINOFF2_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Port A-f Pin\\[N\\]
Digital Input Path Disable Control\\nEach of these bits is used to control if the digital input path of corresponding Px.n pin is disabled. If input is analog signal, users can disable Px.n digital input path to avoid input current leakage.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DINOFF3_A {
    #[doc = "0: Px.n digital input path Enabled"]
    _0 = 0,
    #[doc = "1: Px.n digital input path Disabled (digital input tied to low)"]
    _1 = 1,
}
impl From<DINOFF3_A> for bool {
    #[inline(always)]
    fn from(variant: DINOFF3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DINOFF3` reader - Port A-f Pin\\[N\\]
Digital Input Path Disable Control\\nEach of these bits is used to control if the digital input path of corresponding Px.n pin is disabled. If input is analog signal, users can disable Px.n digital input path to avoid input current leakage.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DINOFF3_R(crate::FieldReader<bool, DINOFF3_A>);
impl DINOFF3_R {
    pub(crate) fn new(bits: bool) -> Self {
        DINOFF3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DINOFF3_A {
        match self.bits {
            false => DINOFF3_A::_0,
            true => DINOFF3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DINOFF3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DINOFF3_A::_1
    }
}
impl core::ops::Deref for DINOFF3_R {
    type Target = crate::FieldReader<bool, DINOFF3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DINOFF3` writer - Port A-f Pin\\[N\\]
Digital Input Path Disable Control\\nEach of these bits is used to control if the digital input path of corresponding Px.n pin is disabled. If input is analog signal, users can disable Px.n digital input path to avoid input current leakage.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DINOFF3_W<'a> {
    w: &'a mut W,
}
impl<'a> DINOFF3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DINOFF3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n digital input path Enabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DINOFF3_A::_0)
    }
    #[doc = "Px.n digital input path Disabled (digital input tied to low)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DINOFF3_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Port A-f Pin\\[N\\]
Digital Input Path Disable Control\\nEach of these bits is used to control if the digital input path of corresponding Px.n pin is disabled. If input is analog signal, users can disable Px.n digital input path to avoid input current leakage.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DINOFF4_A {
    #[doc = "0: Px.n digital input path Enabled"]
    _0 = 0,
    #[doc = "1: Px.n digital input path Disabled (digital input tied to low)"]
    _1 = 1,
}
impl From<DINOFF4_A> for bool {
    #[inline(always)]
    fn from(variant: DINOFF4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DINOFF4` reader - Port A-f Pin\\[N\\]
Digital Input Path Disable Control\\nEach of these bits is used to control if the digital input path of corresponding Px.n pin is disabled. If input is analog signal, users can disable Px.n digital input path to avoid input current leakage.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DINOFF4_R(crate::FieldReader<bool, DINOFF4_A>);
impl DINOFF4_R {
    pub(crate) fn new(bits: bool) -> Self {
        DINOFF4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DINOFF4_A {
        match self.bits {
            false => DINOFF4_A::_0,
            true => DINOFF4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DINOFF4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DINOFF4_A::_1
    }
}
impl core::ops::Deref for DINOFF4_R {
    type Target = crate::FieldReader<bool, DINOFF4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DINOFF4` writer - Port A-f Pin\\[N\\]
Digital Input Path Disable Control\\nEach of these bits is used to control if the digital input path of corresponding Px.n pin is disabled. If input is analog signal, users can disable Px.n digital input path to avoid input current leakage.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DINOFF4_W<'a> {
    w: &'a mut W,
}
impl<'a> DINOFF4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DINOFF4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n digital input path Enabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DINOFF4_A::_0)
    }
    #[doc = "Px.n digital input path Disabled (digital input tied to low)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DINOFF4_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Port A-f Pin\\[N\\]
Digital Input Path Disable Control\\nEach of these bits is used to control if the digital input path of corresponding Px.n pin is disabled. If input is analog signal, users can disable Px.n digital input path to avoid input current leakage.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DINOFF5_A {
    #[doc = "0: Px.n digital input path Enabled"]
    _0 = 0,
    #[doc = "1: Px.n digital input path Disabled (digital input tied to low)"]
    _1 = 1,
}
impl From<DINOFF5_A> for bool {
    #[inline(always)]
    fn from(variant: DINOFF5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DINOFF5` reader - Port A-f Pin\\[N\\]
Digital Input Path Disable Control\\nEach of these bits is used to control if the digital input path of corresponding Px.n pin is disabled. If input is analog signal, users can disable Px.n digital input path to avoid input current leakage.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DINOFF5_R(crate::FieldReader<bool, DINOFF5_A>);
impl DINOFF5_R {
    pub(crate) fn new(bits: bool) -> Self {
        DINOFF5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DINOFF5_A {
        match self.bits {
            false => DINOFF5_A::_0,
            true => DINOFF5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DINOFF5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DINOFF5_A::_1
    }
}
impl core::ops::Deref for DINOFF5_R {
    type Target = crate::FieldReader<bool, DINOFF5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DINOFF5` writer - Port A-f Pin\\[N\\]
Digital Input Path Disable Control\\nEach of these bits is used to control if the digital input path of corresponding Px.n pin is disabled. If input is analog signal, users can disable Px.n digital input path to avoid input current leakage.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DINOFF5_W<'a> {
    w: &'a mut W,
}
impl<'a> DINOFF5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DINOFF5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n digital input path Enabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DINOFF5_A::_0)
    }
    #[doc = "Px.n digital input path Disabled (digital input tied to low)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DINOFF5_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Port A-f Pin\\[N\\]
Digital Input Path Disable Control\\nEach of these bits is used to control if the digital input path of corresponding Px.n pin is disabled. If input is analog signal, users can disable Px.n digital input path to avoid input current leakage.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DINOFF6_A {
    #[doc = "0: Px.n digital input path Enabled"]
    _0 = 0,
    #[doc = "1: Px.n digital input path Disabled (digital input tied to low)"]
    _1 = 1,
}
impl From<DINOFF6_A> for bool {
    #[inline(always)]
    fn from(variant: DINOFF6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DINOFF6` reader - Port A-f Pin\\[N\\]
Digital Input Path Disable Control\\nEach of these bits is used to control if the digital input path of corresponding Px.n pin is disabled. If input is analog signal, users can disable Px.n digital input path to avoid input current leakage.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DINOFF6_R(crate::FieldReader<bool, DINOFF6_A>);
impl DINOFF6_R {
    pub(crate) fn new(bits: bool) -> Self {
        DINOFF6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DINOFF6_A {
        match self.bits {
            false => DINOFF6_A::_0,
            true => DINOFF6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DINOFF6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DINOFF6_A::_1
    }
}
impl core::ops::Deref for DINOFF6_R {
    type Target = crate::FieldReader<bool, DINOFF6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DINOFF6` writer - Port A-f Pin\\[N\\]
Digital Input Path Disable Control\\nEach of these bits is used to control if the digital input path of corresponding Px.n pin is disabled. If input is analog signal, users can disable Px.n digital input path to avoid input current leakage.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DINOFF6_W<'a> {
    w: &'a mut W,
}
impl<'a> DINOFF6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DINOFF6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n digital input path Enabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DINOFF6_A::_0)
    }
    #[doc = "Px.n digital input path Disabled (digital input tied to low)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DINOFF6_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Port A-f Pin\\[N\\]
Digital Input Path Disable Control\\nEach of these bits is used to control if the digital input path of corresponding Px.n pin is disabled. If input is analog signal, users can disable Px.n digital input path to avoid input current leakage.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DINOFF7_A {
    #[doc = "0: Px.n digital input path Enabled"]
    _0 = 0,
    #[doc = "1: Px.n digital input path Disabled (digital input tied to low)"]
    _1 = 1,
}
impl From<DINOFF7_A> for bool {
    #[inline(always)]
    fn from(variant: DINOFF7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DINOFF7` reader - Port A-f Pin\\[N\\]
Digital Input Path Disable Control\\nEach of these bits is used to control if the digital input path of corresponding Px.n pin is disabled. If input is analog signal, users can disable Px.n digital input path to avoid input current leakage.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DINOFF7_R(crate::FieldReader<bool, DINOFF7_A>);
impl DINOFF7_R {
    pub(crate) fn new(bits: bool) -> Self {
        DINOFF7_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DINOFF7_A {
        match self.bits {
            false => DINOFF7_A::_0,
            true => DINOFF7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DINOFF7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DINOFF7_A::_1
    }
}
impl core::ops::Deref for DINOFF7_R {
    type Target = crate::FieldReader<bool, DINOFF7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DINOFF7` writer - Port A-f Pin\\[N\\]
Digital Input Path Disable Control\\nEach of these bits is used to control if the digital input path of corresponding Px.n pin is disabled. If input is analog signal, users can disable Px.n digital input path to avoid input current leakage.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DINOFF7_W<'a> {
    w: &'a mut W,
}
impl<'a> DINOFF7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DINOFF7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n digital input path Enabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DINOFF7_A::_0)
    }
    #[doc = "Px.n digital input path Disabled (digital input tied to low)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DINOFF7_A::_1)
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
#[doc = "Port A-f Pin\\[N\\]
Digital Input Path Disable Control\\nEach of these bits is used to control if the digital input path of corresponding Px.n pin is disabled. If input is analog signal, users can disable Px.n digital input path to avoid input current leakage.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DINOFF8_A {
    #[doc = "0: Px.n digital input path Enabled"]
    _0 = 0,
    #[doc = "1: Px.n digital input path Disabled (digital input tied to low)"]
    _1 = 1,
}
impl From<DINOFF8_A> for bool {
    #[inline(always)]
    fn from(variant: DINOFF8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DINOFF8` reader - Port A-f Pin\\[N\\]
Digital Input Path Disable Control\\nEach of these bits is used to control if the digital input path of corresponding Px.n pin is disabled. If input is analog signal, users can disable Px.n digital input path to avoid input current leakage.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DINOFF8_R(crate::FieldReader<bool, DINOFF8_A>);
impl DINOFF8_R {
    pub(crate) fn new(bits: bool) -> Self {
        DINOFF8_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DINOFF8_A {
        match self.bits {
            false => DINOFF8_A::_0,
            true => DINOFF8_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DINOFF8_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DINOFF8_A::_1
    }
}
impl core::ops::Deref for DINOFF8_R {
    type Target = crate::FieldReader<bool, DINOFF8_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DINOFF8` writer - Port A-f Pin\\[N\\]
Digital Input Path Disable Control\\nEach of these bits is used to control if the digital input path of corresponding Px.n pin is disabled. If input is analog signal, users can disable Px.n digital input path to avoid input current leakage.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DINOFF8_W<'a> {
    w: &'a mut W,
}
impl<'a> DINOFF8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DINOFF8_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n digital input path Enabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DINOFF8_A::_0)
    }
    #[doc = "Px.n digital input path Disabled (digital input tied to low)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DINOFF8_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Port A-f Pin\\[N\\]
Digital Input Path Disable Control\\nEach of these bits is used to control if the digital input path of corresponding Px.n pin is disabled. If input is analog signal, users can disable Px.n digital input path to avoid input current leakage.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DINOFF9_A {
    #[doc = "0: Px.n digital input path Enabled"]
    _0 = 0,
    #[doc = "1: Px.n digital input path Disabled (digital input tied to low)"]
    _1 = 1,
}
impl From<DINOFF9_A> for bool {
    #[inline(always)]
    fn from(variant: DINOFF9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DINOFF9` reader - Port A-f Pin\\[N\\]
Digital Input Path Disable Control\\nEach of these bits is used to control if the digital input path of corresponding Px.n pin is disabled. If input is analog signal, users can disable Px.n digital input path to avoid input current leakage.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DINOFF9_R(crate::FieldReader<bool, DINOFF9_A>);
impl DINOFF9_R {
    pub(crate) fn new(bits: bool) -> Self {
        DINOFF9_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DINOFF9_A {
        match self.bits {
            false => DINOFF9_A::_0,
            true => DINOFF9_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DINOFF9_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DINOFF9_A::_1
    }
}
impl core::ops::Deref for DINOFF9_R {
    type Target = crate::FieldReader<bool, DINOFF9_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DINOFF9` writer - Port A-f Pin\\[N\\]
Digital Input Path Disable Control\\nEach of these bits is used to control if the digital input path of corresponding Px.n pin is disabled. If input is analog signal, users can disable Px.n digital input path to avoid input current leakage.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DINOFF9_W<'a> {
    w: &'a mut W,
}
impl<'a> DINOFF9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DINOFF9_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n digital input path Enabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DINOFF9_A::_0)
    }
    #[doc = "Px.n digital input path Disabled (digital input tied to low)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DINOFF9_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Port A-f Pin\\[N\\]
Digital Input Path Disable Control\\nEach of these bits is used to control if the digital input path of corresponding Px.n pin is disabled. If input is analog signal, users can disable Px.n digital input path to avoid input current leakage.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DINOFF10_A {
    #[doc = "0: Px.n digital input path Enabled"]
    _0 = 0,
    #[doc = "1: Px.n digital input path Disabled (digital input tied to low)"]
    _1 = 1,
}
impl From<DINOFF10_A> for bool {
    #[inline(always)]
    fn from(variant: DINOFF10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DINOFF10` reader - Port A-f Pin\\[N\\]
Digital Input Path Disable Control\\nEach of these bits is used to control if the digital input path of corresponding Px.n pin is disabled. If input is analog signal, users can disable Px.n digital input path to avoid input current leakage.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DINOFF10_R(crate::FieldReader<bool, DINOFF10_A>);
impl DINOFF10_R {
    pub(crate) fn new(bits: bool) -> Self {
        DINOFF10_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DINOFF10_A {
        match self.bits {
            false => DINOFF10_A::_0,
            true => DINOFF10_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DINOFF10_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DINOFF10_A::_1
    }
}
impl core::ops::Deref for DINOFF10_R {
    type Target = crate::FieldReader<bool, DINOFF10_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DINOFF10` writer - Port A-f Pin\\[N\\]
Digital Input Path Disable Control\\nEach of these bits is used to control if the digital input path of corresponding Px.n pin is disabled. If input is analog signal, users can disable Px.n digital input path to avoid input current leakage.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DINOFF10_W<'a> {
    w: &'a mut W,
}
impl<'a> DINOFF10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DINOFF10_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n digital input path Enabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DINOFF10_A::_0)
    }
    #[doc = "Px.n digital input path Disabled (digital input tied to low)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DINOFF10_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Port A-f Pin\\[N\\]
Digital Input Path Disable Control\\nEach of these bits is used to control if the digital input path of corresponding Px.n pin is disabled. If input is analog signal, users can disable Px.n digital input path to avoid input current leakage.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DINOFF11_A {
    #[doc = "0: Px.n digital input path Enabled"]
    _0 = 0,
    #[doc = "1: Px.n digital input path Disabled (digital input tied to low)"]
    _1 = 1,
}
impl From<DINOFF11_A> for bool {
    #[inline(always)]
    fn from(variant: DINOFF11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DINOFF11` reader - Port A-f Pin\\[N\\]
Digital Input Path Disable Control\\nEach of these bits is used to control if the digital input path of corresponding Px.n pin is disabled. If input is analog signal, users can disable Px.n digital input path to avoid input current leakage.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DINOFF11_R(crate::FieldReader<bool, DINOFF11_A>);
impl DINOFF11_R {
    pub(crate) fn new(bits: bool) -> Self {
        DINOFF11_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DINOFF11_A {
        match self.bits {
            false => DINOFF11_A::_0,
            true => DINOFF11_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DINOFF11_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DINOFF11_A::_1
    }
}
impl core::ops::Deref for DINOFF11_R {
    type Target = crate::FieldReader<bool, DINOFF11_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DINOFF11` writer - Port A-f Pin\\[N\\]
Digital Input Path Disable Control\\nEach of these bits is used to control if the digital input path of corresponding Px.n pin is disabled. If input is analog signal, users can disable Px.n digital input path to avoid input current leakage.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DINOFF11_W<'a> {
    w: &'a mut W,
}
impl<'a> DINOFF11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DINOFF11_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n digital input path Enabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DINOFF11_A::_0)
    }
    #[doc = "Px.n digital input path Disabled (digital input tied to low)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DINOFF11_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Port A-f Pin\\[N\\]
Digital Input Path Disable Control\\nEach of these bits is used to control if the digital input path of corresponding Px.n pin is disabled. If input is analog signal, users can disable Px.n digital input path to avoid input current leakage.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DINOFF12_A {
    #[doc = "0: Px.n digital input path Enabled"]
    _0 = 0,
    #[doc = "1: Px.n digital input path Disabled (digital input tied to low)"]
    _1 = 1,
}
impl From<DINOFF12_A> for bool {
    #[inline(always)]
    fn from(variant: DINOFF12_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DINOFF12` reader - Port A-f Pin\\[N\\]
Digital Input Path Disable Control\\nEach of these bits is used to control if the digital input path of corresponding Px.n pin is disabled. If input is analog signal, users can disable Px.n digital input path to avoid input current leakage.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DINOFF12_R(crate::FieldReader<bool, DINOFF12_A>);
impl DINOFF12_R {
    pub(crate) fn new(bits: bool) -> Self {
        DINOFF12_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DINOFF12_A {
        match self.bits {
            false => DINOFF12_A::_0,
            true => DINOFF12_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DINOFF12_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DINOFF12_A::_1
    }
}
impl core::ops::Deref for DINOFF12_R {
    type Target = crate::FieldReader<bool, DINOFF12_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DINOFF12` writer - Port A-f Pin\\[N\\]
Digital Input Path Disable Control\\nEach of these bits is used to control if the digital input path of corresponding Px.n pin is disabled. If input is analog signal, users can disable Px.n digital input path to avoid input current leakage.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DINOFF12_W<'a> {
    w: &'a mut W,
}
impl<'a> DINOFF12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DINOFF12_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n digital input path Enabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DINOFF12_A::_0)
    }
    #[doc = "Px.n digital input path Disabled (digital input tied to low)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DINOFF12_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Port A-f Pin\\[N\\]
Digital Input Path Disable Control\\nEach of these bits is used to control if the digital input path of corresponding Px.n pin is disabled. If input is analog signal, users can disable Px.n digital input path to avoid input current leakage.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DINOFF13_A {
    #[doc = "0: Px.n digital input path Enabled"]
    _0 = 0,
    #[doc = "1: Px.n digital input path Disabled (digital input tied to low)"]
    _1 = 1,
}
impl From<DINOFF13_A> for bool {
    #[inline(always)]
    fn from(variant: DINOFF13_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DINOFF13` reader - Port A-f Pin\\[N\\]
Digital Input Path Disable Control\\nEach of these bits is used to control if the digital input path of corresponding Px.n pin is disabled. If input is analog signal, users can disable Px.n digital input path to avoid input current leakage.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DINOFF13_R(crate::FieldReader<bool, DINOFF13_A>);
impl DINOFF13_R {
    pub(crate) fn new(bits: bool) -> Self {
        DINOFF13_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DINOFF13_A {
        match self.bits {
            false => DINOFF13_A::_0,
            true => DINOFF13_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DINOFF13_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DINOFF13_A::_1
    }
}
impl core::ops::Deref for DINOFF13_R {
    type Target = crate::FieldReader<bool, DINOFF13_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DINOFF13` writer - Port A-f Pin\\[N\\]
Digital Input Path Disable Control\\nEach of these bits is used to control if the digital input path of corresponding Px.n pin is disabled. If input is analog signal, users can disable Px.n digital input path to avoid input current leakage.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DINOFF13_W<'a> {
    w: &'a mut W,
}
impl<'a> DINOFF13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DINOFF13_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n digital input path Enabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DINOFF13_A::_0)
    }
    #[doc = "Px.n digital input path Disabled (digital input tied to low)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DINOFF13_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Port A-f Pin\\[N\\]
Digital Input Path Disable Control\\nEach of these bits is used to control if the digital input path of corresponding Px.n pin is disabled. If input is analog signal, users can disable Px.n digital input path to avoid input current leakage.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DINOFF14_A {
    #[doc = "0: Px.n digital input path Enabled"]
    _0 = 0,
    #[doc = "1: Px.n digital input path Disabled (digital input tied to low)"]
    _1 = 1,
}
impl From<DINOFF14_A> for bool {
    #[inline(always)]
    fn from(variant: DINOFF14_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DINOFF14` reader - Port A-f Pin\\[N\\]
Digital Input Path Disable Control\\nEach of these bits is used to control if the digital input path of corresponding Px.n pin is disabled. If input is analog signal, users can disable Px.n digital input path to avoid input current leakage.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DINOFF14_R(crate::FieldReader<bool, DINOFF14_A>);
impl DINOFF14_R {
    pub(crate) fn new(bits: bool) -> Self {
        DINOFF14_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DINOFF14_A {
        match self.bits {
            false => DINOFF14_A::_0,
            true => DINOFF14_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DINOFF14_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DINOFF14_A::_1
    }
}
impl core::ops::Deref for DINOFF14_R {
    type Target = crate::FieldReader<bool, DINOFF14_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DINOFF14` writer - Port A-f Pin\\[N\\]
Digital Input Path Disable Control\\nEach of these bits is used to control if the digital input path of corresponding Px.n pin is disabled. If input is analog signal, users can disable Px.n digital input path to avoid input current leakage.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DINOFF14_W<'a> {
    w: &'a mut W,
}
impl<'a> DINOFF14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DINOFF14_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n digital input path Enabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DINOFF14_A::_0)
    }
    #[doc = "Px.n digital input path Disabled (digital input tied to low)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DINOFF14_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Port A-f Pin\\[N\\]
Digital Input Path Disable Control\\nEach of these bits is used to control if the digital input path of corresponding Px.n pin is disabled. If input is analog signal, users can disable Px.n digital input path to avoid input current leakage.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DINOFF15_A {
    #[doc = "0: Px.n digital input path Enabled"]
    _0 = 0,
    #[doc = "1: Px.n digital input path Disabled (digital input tied to low)"]
    _1 = 1,
}
impl From<DINOFF15_A> for bool {
    #[inline(always)]
    fn from(variant: DINOFF15_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DINOFF15` reader - Port A-f Pin\\[N\\]
Digital Input Path Disable Control\\nEach of these bits is used to control if the digital input path of corresponding Px.n pin is disabled. If input is analog signal, users can disable Px.n digital input path to avoid input current leakage.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DINOFF15_R(crate::FieldReader<bool, DINOFF15_A>);
impl DINOFF15_R {
    pub(crate) fn new(bits: bool) -> Self {
        DINOFF15_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DINOFF15_A {
        match self.bits {
            false => DINOFF15_A::_0,
            true => DINOFF15_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DINOFF15_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DINOFF15_A::_1
    }
}
impl core::ops::Deref for DINOFF15_R {
    type Target = crate::FieldReader<bool, DINOFF15_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DINOFF15` writer - Port A-f Pin\\[N\\]
Digital Input Path Disable Control\\nEach of these bits is used to control if the digital input path of corresponding Px.n pin is disabled. If input is analog signal, users can disable Px.n digital input path to avoid input current leakage.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DINOFF15_W<'a> {
    w: &'a mut W,
}
impl<'a> DINOFF15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DINOFF15_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Px.n digital input path Enabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DINOFF15_A::_0)
    }
    #[doc = "Px.n digital input path Disabled (digital input tied to low)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DINOFF15_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 16 - Port A-f Pin\\[N\\]
Digital Input Path Disable Control\\nEach of these bits is used to control if the digital input path of corresponding Px.n pin is disabled. If input is analog signal, users can disable Px.n digital input path to avoid input current leakage.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn dinoff0(&self) -> DINOFF0_R {
        DINOFF0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Port A-f Pin\\[N\\]
Digital Input Path Disable Control\\nEach of these bits is used to control if the digital input path of corresponding Px.n pin is disabled. If input is analog signal, users can disable Px.n digital input path to avoid input current leakage.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn dinoff1(&self) -> DINOFF1_R {
        DINOFF1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Port A-f Pin\\[N\\]
Digital Input Path Disable Control\\nEach of these bits is used to control if the digital input path of corresponding Px.n pin is disabled. If input is analog signal, users can disable Px.n digital input path to avoid input current leakage.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn dinoff2(&self) -> DINOFF2_R {
        DINOFF2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Port A-f Pin\\[N\\]
Digital Input Path Disable Control\\nEach of these bits is used to control if the digital input path of corresponding Px.n pin is disabled. If input is analog signal, users can disable Px.n digital input path to avoid input current leakage.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn dinoff3(&self) -> DINOFF3_R {
        DINOFF3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Port A-f Pin\\[N\\]
Digital Input Path Disable Control\\nEach of these bits is used to control if the digital input path of corresponding Px.n pin is disabled. If input is analog signal, users can disable Px.n digital input path to avoid input current leakage.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn dinoff4(&self) -> DINOFF4_R {
        DINOFF4_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Port A-f Pin\\[N\\]
Digital Input Path Disable Control\\nEach of these bits is used to control if the digital input path of corresponding Px.n pin is disabled. If input is analog signal, users can disable Px.n digital input path to avoid input current leakage.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn dinoff5(&self) -> DINOFF5_R {
        DINOFF5_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Port A-f Pin\\[N\\]
Digital Input Path Disable Control\\nEach of these bits is used to control if the digital input path of corresponding Px.n pin is disabled. If input is analog signal, users can disable Px.n digital input path to avoid input current leakage.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn dinoff6(&self) -> DINOFF6_R {
        DINOFF6_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Port A-f Pin\\[N\\]
Digital Input Path Disable Control\\nEach of these bits is used to control if the digital input path of corresponding Px.n pin is disabled. If input is analog signal, users can disable Px.n digital input path to avoid input current leakage.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn dinoff7(&self) -> DINOFF7_R {
        DINOFF7_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Port A-f Pin\\[N\\]
Digital Input Path Disable Control\\nEach of these bits is used to control if the digital input path of corresponding Px.n pin is disabled. If input is analog signal, users can disable Px.n digital input path to avoid input current leakage.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn dinoff8(&self) -> DINOFF8_R {
        DINOFF8_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Port A-f Pin\\[N\\]
Digital Input Path Disable Control\\nEach of these bits is used to control if the digital input path of corresponding Px.n pin is disabled. If input is analog signal, users can disable Px.n digital input path to avoid input current leakage.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn dinoff9(&self) -> DINOFF9_R {
        DINOFF9_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Port A-f Pin\\[N\\]
Digital Input Path Disable Control\\nEach of these bits is used to control if the digital input path of corresponding Px.n pin is disabled. If input is analog signal, users can disable Px.n digital input path to avoid input current leakage.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn dinoff10(&self) -> DINOFF10_R {
        DINOFF10_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Port A-f Pin\\[N\\]
Digital Input Path Disable Control\\nEach of these bits is used to control if the digital input path of corresponding Px.n pin is disabled. If input is analog signal, users can disable Px.n digital input path to avoid input current leakage.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn dinoff11(&self) -> DINOFF11_R {
        DINOFF11_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Port A-f Pin\\[N\\]
Digital Input Path Disable Control\\nEach of these bits is used to control if the digital input path of corresponding Px.n pin is disabled. If input is analog signal, users can disable Px.n digital input path to avoid input current leakage.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn dinoff12(&self) -> DINOFF12_R {
        DINOFF12_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Port A-f Pin\\[N\\]
Digital Input Path Disable Control\\nEach of these bits is used to control if the digital input path of corresponding Px.n pin is disabled. If input is analog signal, users can disable Px.n digital input path to avoid input current leakage.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn dinoff13(&self) -> DINOFF13_R {
        DINOFF13_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Port A-f Pin\\[N\\]
Digital Input Path Disable Control\\nEach of these bits is used to control if the digital input path of corresponding Px.n pin is disabled. If input is analog signal, users can disable Px.n digital input path to avoid input current leakage.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn dinoff14(&self) -> DINOFF14_R {
        DINOFF14_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Port A-f Pin\\[N\\]
Digital Input Path Disable Control\\nEach of these bits is used to control if the digital input path of corresponding Px.n pin is disabled. If input is analog signal, users can disable Px.n digital input path to avoid input current leakage.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn dinoff15(&self) -> DINOFF15_R {
        DINOFF15_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - Port A-f Pin\\[N\\]
Digital Input Path Disable Control\\nEach of these bits is used to control if the digital input path of corresponding Px.n pin is disabled. If input is analog signal, users can disable Px.n digital input path to avoid input current leakage.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn dinoff0(&mut self) -> DINOFF0_W {
        DINOFF0_W { w: self }
    }
    #[doc = "Bit 17 - Port A-f Pin\\[N\\]
Digital Input Path Disable Control\\nEach of these bits is used to control if the digital input path of corresponding Px.n pin is disabled. If input is analog signal, users can disable Px.n digital input path to avoid input current leakage.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn dinoff1(&mut self) -> DINOFF1_W {
        DINOFF1_W { w: self }
    }
    #[doc = "Bit 18 - Port A-f Pin\\[N\\]
Digital Input Path Disable Control\\nEach of these bits is used to control if the digital input path of corresponding Px.n pin is disabled. If input is analog signal, users can disable Px.n digital input path to avoid input current leakage.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn dinoff2(&mut self) -> DINOFF2_W {
        DINOFF2_W { w: self }
    }
    #[doc = "Bit 19 - Port A-f Pin\\[N\\]
Digital Input Path Disable Control\\nEach of these bits is used to control if the digital input path of corresponding Px.n pin is disabled. If input is analog signal, users can disable Px.n digital input path to avoid input current leakage.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn dinoff3(&mut self) -> DINOFF3_W {
        DINOFF3_W { w: self }
    }
    #[doc = "Bit 20 - Port A-f Pin\\[N\\]
Digital Input Path Disable Control\\nEach of these bits is used to control if the digital input path of corresponding Px.n pin is disabled. If input is analog signal, users can disable Px.n digital input path to avoid input current leakage.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn dinoff4(&mut self) -> DINOFF4_W {
        DINOFF4_W { w: self }
    }
    #[doc = "Bit 21 - Port A-f Pin\\[N\\]
Digital Input Path Disable Control\\nEach of these bits is used to control if the digital input path of corresponding Px.n pin is disabled. If input is analog signal, users can disable Px.n digital input path to avoid input current leakage.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn dinoff5(&mut self) -> DINOFF5_W {
        DINOFF5_W { w: self }
    }
    #[doc = "Bit 22 - Port A-f Pin\\[N\\]
Digital Input Path Disable Control\\nEach of these bits is used to control if the digital input path of corresponding Px.n pin is disabled. If input is analog signal, users can disable Px.n digital input path to avoid input current leakage.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn dinoff6(&mut self) -> DINOFF6_W {
        DINOFF6_W { w: self }
    }
    #[doc = "Bit 23 - Port A-f Pin\\[N\\]
Digital Input Path Disable Control\\nEach of these bits is used to control if the digital input path of corresponding Px.n pin is disabled. If input is analog signal, users can disable Px.n digital input path to avoid input current leakage.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn dinoff7(&mut self) -> DINOFF7_W {
        DINOFF7_W { w: self }
    }
    #[doc = "Bit 24 - Port A-f Pin\\[N\\]
Digital Input Path Disable Control\\nEach of these bits is used to control if the digital input path of corresponding Px.n pin is disabled. If input is analog signal, users can disable Px.n digital input path to avoid input current leakage.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn dinoff8(&mut self) -> DINOFF8_W {
        DINOFF8_W { w: self }
    }
    #[doc = "Bit 25 - Port A-f Pin\\[N\\]
Digital Input Path Disable Control\\nEach of these bits is used to control if the digital input path of corresponding Px.n pin is disabled. If input is analog signal, users can disable Px.n digital input path to avoid input current leakage.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn dinoff9(&mut self) -> DINOFF9_W {
        DINOFF9_W { w: self }
    }
    #[doc = "Bit 26 - Port A-f Pin\\[N\\]
Digital Input Path Disable Control\\nEach of these bits is used to control if the digital input path of corresponding Px.n pin is disabled. If input is analog signal, users can disable Px.n digital input path to avoid input current leakage.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn dinoff10(&mut self) -> DINOFF10_W {
        DINOFF10_W { w: self }
    }
    #[doc = "Bit 27 - Port A-f Pin\\[N\\]
Digital Input Path Disable Control\\nEach of these bits is used to control if the digital input path of corresponding Px.n pin is disabled. If input is analog signal, users can disable Px.n digital input path to avoid input current leakage.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn dinoff11(&mut self) -> DINOFF11_W {
        DINOFF11_W { w: self }
    }
    #[doc = "Bit 28 - Port A-f Pin\\[N\\]
Digital Input Path Disable Control\\nEach of these bits is used to control if the digital input path of corresponding Px.n pin is disabled. If input is analog signal, users can disable Px.n digital input path to avoid input current leakage.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn dinoff12(&mut self) -> DINOFF12_W {
        DINOFF12_W { w: self }
    }
    #[doc = "Bit 29 - Port A-f Pin\\[N\\]
Digital Input Path Disable Control\\nEach of these bits is used to control if the digital input path of corresponding Px.n pin is disabled. If input is analog signal, users can disable Px.n digital input path to avoid input current leakage.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn dinoff13(&mut self) -> DINOFF13_W {
        DINOFF13_W { w: self }
    }
    #[doc = "Bit 30 - Port A-f Pin\\[N\\]
Digital Input Path Disable Control\\nEach of these bits is used to control if the digital input path of corresponding Px.n pin is disabled. If input is analog signal, users can disable Px.n digital input path to avoid input current leakage.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn dinoff14(&mut self) -> DINOFF14_W {
        DINOFF14_W { w: self }
    }
    #[doc = "Bit 31 - Port A-f Pin\\[N\\]
Digital Input Path Disable Control\\nEach of these bits is used to control if the digital input path of corresponding Px.n pin is disabled. If input is analog signal, users can disable Px.n digital input path to avoid input current leakage.\\nNote2: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn dinoff15(&mut self) -> DINOFF15_W {
        DINOFF15_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PF Digital Input Path Disable Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pf_dinoff](index.html) module"]
pub struct PF_DINOFF_SPEC;
impl crate::RegisterSpec for PF_DINOFF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pf_dinoff::R](R) reader structure"]
impl crate::Readable for PF_DINOFF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pf_dinoff::W](W) writer structure"]
impl crate::Writable for PF_DINOFF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PF_DINOFF to value 0"]
impl crate::Resettable for PF_DINOFF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
