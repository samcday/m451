#[doc = "Register `PC_DATMSK` reader"]
pub struct R(crate::R<PC_DATMSK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PC_DATMSK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PC_DATMSK_SPEC>> for R {
    fn from(reader: crate::R<PC_DATMSK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PC_DATMSK` writer"]
pub struct W(crate::W<PC_DATMSK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PC_DATMSK_SPEC>;
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
impl core::convert::From<crate::W<PC_DATMSK_SPEC>> for W {
    fn from(writer: crate::W<PC_DATMSK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Port A-f Pin\\[N\\]
Data Output Write Mask\\nThese bits are used to protect the corresponding DOUT (Px_DOUT\\[n\\]) bit. When the DATMSK (Px_DATMSK\\[n\\]) bit is set to 1, the corresponding DOUT (Px_DOUT\\[n\\]) bit is protected. If the write signal is masked, writing data to the protect bit is ignored.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATMSK0_A {
    #[doc = "0: Corresponding DOUT (Px_DOUT\\[n\\]) bit can be updated"]
    _0 = 0,
    #[doc = "1: Corresponding DOUT (Px_DOUT\\[n\\]) bit protected"]
    _1 = 1,
}
impl From<DATMSK0_A> for bool {
    #[inline(always)]
    fn from(variant: DATMSK0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATMSK0` reader - Port A-f Pin\\[N\\]
Data Output Write Mask\\nThese bits are used to protect the corresponding DOUT (Px_DOUT\\[n\\]) bit. When the DATMSK (Px_DATMSK\\[n\\]) bit is set to 1, the corresponding DOUT (Px_DOUT\\[n\\]) bit is protected. If the write signal is masked, writing data to the protect bit is ignored.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DATMSK0_R(crate::FieldReader<bool, DATMSK0_A>);
impl DATMSK0_R {
    pub(crate) fn new(bits: bool) -> Self {
        DATMSK0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATMSK0_A {
        match self.bits {
            false => DATMSK0_A::_0,
            true => DATMSK0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DATMSK0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DATMSK0_A::_1
    }
}
impl core::ops::Deref for DATMSK0_R {
    type Target = crate::FieldReader<bool, DATMSK0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATMSK0` writer - Port A-f Pin\\[N\\]
Data Output Write Mask\\nThese bits are used to protect the corresponding DOUT (Px_DOUT\\[n\\]) bit. When the DATMSK (Px_DATMSK\\[n\\]) bit is set to 1, the corresponding DOUT (Px_DOUT\\[n\\]) bit is protected. If the write signal is masked, writing data to the protect bit is ignored.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DATMSK0_W<'a> {
    w: &'a mut W,
}
impl<'a> DATMSK0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATMSK0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Corresponding DOUT (Px_DOUT\\[n\\]) bit can be updated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DATMSK0_A::_0)
    }
    #[doc = "Corresponding DOUT (Px_DOUT\\[n\\]) bit protected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DATMSK0_A::_1)
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
#[doc = "Port A-f Pin\\[N\\]
Data Output Write Mask\\nThese bits are used to protect the corresponding DOUT (Px_DOUT\\[n\\]) bit. When the DATMSK (Px_DATMSK\\[n\\]) bit is set to 1, the corresponding DOUT (Px_DOUT\\[n\\]) bit is protected. If the write signal is masked, writing data to the protect bit is ignored.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATMSK1_A {
    #[doc = "0: Corresponding DOUT (Px_DOUT\\[n\\]) bit can be updated"]
    _0 = 0,
    #[doc = "1: Corresponding DOUT (Px_DOUT\\[n\\]) bit protected"]
    _1 = 1,
}
impl From<DATMSK1_A> for bool {
    #[inline(always)]
    fn from(variant: DATMSK1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATMSK1` reader - Port A-f Pin\\[N\\]
Data Output Write Mask\\nThese bits are used to protect the corresponding DOUT (Px_DOUT\\[n\\]) bit. When the DATMSK (Px_DATMSK\\[n\\]) bit is set to 1, the corresponding DOUT (Px_DOUT\\[n\\]) bit is protected. If the write signal is masked, writing data to the protect bit is ignored.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DATMSK1_R(crate::FieldReader<bool, DATMSK1_A>);
impl DATMSK1_R {
    pub(crate) fn new(bits: bool) -> Self {
        DATMSK1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATMSK1_A {
        match self.bits {
            false => DATMSK1_A::_0,
            true => DATMSK1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DATMSK1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DATMSK1_A::_1
    }
}
impl core::ops::Deref for DATMSK1_R {
    type Target = crate::FieldReader<bool, DATMSK1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATMSK1` writer - Port A-f Pin\\[N\\]
Data Output Write Mask\\nThese bits are used to protect the corresponding DOUT (Px_DOUT\\[n\\]) bit. When the DATMSK (Px_DATMSK\\[n\\]) bit is set to 1, the corresponding DOUT (Px_DOUT\\[n\\]) bit is protected. If the write signal is masked, writing data to the protect bit is ignored.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DATMSK1_W<'a> {
    w: &'a mut W,
}
impl<'a> DATMSK1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATMSK1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Corresponding DOUT (Px_DOUT\\[n\\]) bit can be updated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DATMSK1_A::_0)
    }
    #[doc = "Corresponding DOUT (Px_DOUT\\[n\\]) bit protected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DATMSK1_A::_1)
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
#[doc = "Port A-f Pin\\[N\\]
Data Output Write Mask\\nThese bits are used to protect the corresponding DOUT (Px_DOUT\\[n\\]) bit. When the DATMSK (Px_DATMSK\\[n\\]) bit is set to 1, the corresponding DOUT (Px_DOUT\\[n\\]) bit is protected. If the write signal is masked, writing data to the protect bit is ignored.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATMSK2_A {
    #[doc = "0: Corresponding DOUT (Px_DOUT\\[n\\]) bit can be updated"]
    _0 = 0,
    #[doc = "1: Corresponding DOUT (Px_DOUT\\[n\\]) bit protected"]
    _1 = 1,
}
impl From<DATMSK2_A> for bool {
    #[inline(always)]
    fn from(variant: DATMSK2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATMSK2` reader - Port A-f Pin\\[N\\]
Data Output Write Mask\\nThese bits are used to protect the corresponding DOUT (Px_DOUT\\[n\\]) bit. When the DATMSK (Px_DATMSK\\[n\\]) bit is set to 1, the corresponding DOUT (Px_DOUT\\[n\\]) bit is protected. If the write signal is masked, writing data to the protect bit is ignored.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DATMSK2_R(crate::FieldReader<bool, DATMSK2_A>);
impl DATMSK2_R {
    pub(crate) fn new(bits: bool) -> Self {
        DATMSK2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATMSK2_A {
        match self.bits {
            false => DATMSK2_A::_0,
            true => DATMSK2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DATMSK2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DATMSK2_A::_1
    }
}
impl core::ops::Deref for DATMSK2_R {
    type Target = crate::FieldReader<bool, DATMSK2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATMSK2` writer - Port A-f Pin\\[N\\]
Data Output Write Mask\\nThese bits are used to protect the corresponding DOUT (Px_DOUT\\[n\\]) bit. When the DATMSK (Px_DATMSK\\[n\\]) bit is set to 1, the corresponding DOUT (Px_DOUT\\[n\\]) bit is protected. If the write signal is masked, writing data to the protect bit is ignored.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DATMSK2_W<'a> {
    w: &'a mut W,
}
impl<'a> DATMSK2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATMSK2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Corresponding DOUT (Px_DOUT\\[n\\]) bit can be updated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DATMSK2_A::_0)
    }
    #[doc = "Corresponding DOUT (Px_DOUT\\[n\\]) bit protected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DATMSK2_A::_1)
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
#[doc = "Port A-f Pin\\[N\\]
Data Output Write Mask\\nThese bits are used to protect the corresponding DOUT (Px_DOUT\\[n\\]) bit. When the DATMSK (Px_DATMSK\\[n\\]) bit is set to 1, the corresponding DOUT (Px_DOUT\\[n\\]) bit is protected. If the write signal is masked, writing data to the protect bit is ignored.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATMSK3_A {
    #[doc = "0: Corresponding DOUT (Px_DOUT\\[n\\]) bit can be updated"]
    _0 = 0,
    #[doc = "1: Corresponding DOUT (Px_DOUT\\[n\\]) bit protected"]
    _1 = 1,
}
impl From<DATMSK3_A> for bool {
    #[inline(always)]
    fn from(variant: DATMSK3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATMSK3` reader - Port A-f Pin\\[N\\]
Data Output Write Mask\\nThese bits are used to protect the corresponding DOUT (Px_DOUT\\[n\\]) bit. When the DATMSK (Px_DATMSK\\[n\\]) bit is set to 1, the corresponding DOUT (Px_DOUT\\[n\\]) bit is protected. If the write signal is masked, writing data to the protect bit is ignored.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DATMSK3_R(crate::FieldReader<bool, DATMSK3_A>);
impl DATMSK3_R {
    pub(crate) fn new(bits: bool) -> Self {
        DATMSK3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATMSK3_A {
        match self.bits {
            false => DATMSK3_A::_0,
            true => DATMSK3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DATMSK3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DATMSK3_A::_1
    }
}
impl core::ops::Deref for DATMSK3_R {
    type Target = crate::FieldReader<bool, DATMSK3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATMSK3` writer - Port A-f Pin\\[N\\]
Data Output Write Mask\\nThese bits are used to protect the corresponding DOUT (Px_DOUT\\[n\\]) bit. When the DATMSK (Px_DATMSK\\[n\\]) bit is set to 1, the corresponding DOUT (Px_DOUT\\[n\\]) bit is protected. If the write signal is masked, writing data to the protect bit is ignored.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DATMSK3_W<'a> {
    w: &'a mut W,
}
impl<'a> DATMSK3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATMSK3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Corresponding DOUT (Px_DOUT\\[n\\]) bit can be updated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DATMSK3_A::_0)
    }
    #[doc = "Corresponding DOUT (Px_DOUT\\[n\\]) bit protected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DATMSK3_A::_1)
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
#[doc = "Port A-f Pin\\[N\\]
Data Output Write Mask\\nThese bits are used to protect the corresponding DOUT (Px_DOUT\\[n\\]) bit. When the DATMSK (Px_DATMSK\\[n\\]) bit is set to 1, the corresponding DOUT (Px_DOUT\\[n\\]) bit is protected. If the write signal is masked, writing data to the protect bit is ignored.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATMSK4_A {
    #[doc = "0: Corresponding DOUT (Px_DOUT\\[n\\]) bit can be updated"]
    _0 = 0,
    #[doc = "1: Corresponding DOUT (Px_DOUT\\[n\\]) bit protected"]
    _1 = 1,
}
impl From<DATMSK4_A> for bool {
    #[inline(always)]
    fn from(variant: DATMSK4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATMSK4` reader - Port A-f Pin\\[N\\]
Data Output Write Mask\\nThese bits are used to protect the corresponding DOUT (Px_DOUT\\[n\\]) bit. When the DATMSK (Px_DATMSK\\[n\\]) bit is set to 1, the corresponding DOUT (Px_DOUT\\[n\\]) bit is protected. If the write signal is masked, writing data to the protect bit is ignored.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DATMSK4_R(crate::FieldReader<bool, DATMSK4_A>);
impl DATMSK4_R {
    pub(crate) fn new(bits: bool) -> Self {
        DATMSK4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATMSK4_A {
        match self.bits {
            false => DATMSK4_A::_0,
            true => DATMSK4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DATMSK4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DATMSK4_A::_1
    }
}
impl core::ops::Deref for DATMSK4_R {
    type Target = crate::FieldReader<bool, DATMSK4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATMSK4` writer - Port A-f Pin\\[N\\]
Data Output Write Mask\\nThese bits are used to protect the corresponding DOUT (Px_DOUT\\[n\\]) bit. When the DATMSK (Px_DATMSK\\[n\\]) bit is set to 1, the corresponding DOUT (Px_DOUT\\[n\\]) bit is protected. If the write signal is masked, writing data to the protect bit is ignored.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DATMSK4_W<'a> {
    w: &'a mut W,
}
impl<'a> DATMSK4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATMSK4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Corresponding DOUT (Px_DOUT\\[n\\]) bit can be updated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DATMSK4_A::_0)
    }
    #[doc = "Corresponding DOUT (Px_DOUT\\[n\\]) bit protected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DATMSK4_A::_1)
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
#[doc = "Port A-f Pin\\[N\\]
Data Output Write Mask\\nThese bits are used to protect the corresponding DOUT (Px_DOUT\\[n\\]) bit. When the DATMSK (Px_DATMSK\\[n\\]) bit is set to 1, the corresponding DOUT (Px_DOUT\\[n\\]) bit is protected. If the write signal is masked, writing data to the protect bit is ignored.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATMSK5_A {
    #[doc = "0: Corresponding DOUT (Px_DOUT\\[n\\]) bit can be updated"]
    _0 = 0,
    #[doc = "1: Corresponding DOUT (Px_DOUT\\[n\\]) bit protected"]
    _1 = 1,
}
impl From<DATMSK5_A> for bool {
    #[inline(always)]
    fn from(variant: DATMSK5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATMSK5` reader - Port A-f Pin\\[N\\]
Data Output Write Mask\\nThese bits are used to protect the corresponding DOUT (Px_DOUT\\[n\\]) bit. When the DATMSK (Px_DATMSK\\[n\\]) bit is set to 1, the corresponding DOUT (Px_DOUT\\[n\\]) bit is protected. If the write signal is masked, writing data to the protect bit is ignored.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DATMSK5_R(crate::FieldReader<bool, DATMSK5_A>);
impl DATMSK5_R {
    pub(crate) fn new(bits: bool) -> Self {
        DATMSK5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATMSK5_A {
        match self.bits {
            false => DATMSK5_A::_0,
            true => DATMSK5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DATMSK5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DATMSK5_A::_1
    }
}
impl core::ops::Deref for DATMSK5_R {
    type Target = crate::FieldReader<bool, DATMSK5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATMSK5` writer - Port A-f Pin\\[N\\]
Data Output Write Mask\\nThese bits are used to protect the corresponding DOUT (Px_DOUT\\[n\\]) bit. When the DATMSK (Px_DATMSK\\[n\\]) bit is set to 1, the corresponding DOUT (Px_DOUT\\[n\\]) bit is protected. If the write signal is masked, writing data to the protect bit is ignored.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DATMSK5_W<'a> {
    w: &'a mut W,
}
impl<'a> DATMSK5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATMSK5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Corresponding DOUT (Px_DOUT\\[n\\]) bit can be updated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DATMSK5_A::_0)
    }
    #[doc = "Corresponding DOUT (Px_DOUT\\[n\\]) bit protected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DATMSK5_A::_1)
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
#[doc = "Port A-f Pin\\[N\\]
Data Output Write Mask\\nThese bits are used to protect the corresponding DOUT (Px_DOUT\\[n\\]) bit. When the DATMSK (Px_DATMSK\\[n\\]) bit is set to 1, the corresponding DOUT (Px_DOUT\\[n\\]) bit is protected. If the write signal is masked, writing data to the protect bit is ignored.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATMSK6_A {
    #[doc = "0: Corresponding DOUT (Px_DOUT\\[n\\]) bit can be updated"]
    _0 = 0,
    #[doc = "1: Corresponding DOUT (Px_DOUT\\[n\\]) bit protected"]
    _1 = 1,
}
impl From<DATMSK6_A> for bool {
    #[inline(always)]
    fn from(variant: DATMSK6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATMSK6` reader - Port A-f Pin\\[N\\]
Data Output Write Mask\\nThese bits are used to protect the corresponding DOUT (Px_DOUT\\[n\\]) bit. When the DATMSK (Px_DATMSK\\[n\\]) bit is set to 1, the corresponding DOUT (Px_DOUT\\[n\\]) bit is protected. If the write signal is masked, writing data to the protect bit is ignored.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DATMSK6_R(crate::FieldReader<bool, DATMSK6_A>);
impl DATMSK6_R {
    pub(crate) fn new(bits: bool) -> Self {
        DATMSK6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATMSK6_A {
        match self.bits {
            false => DATMSK6_A::_0,
            true => DATMSK6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DATMSK6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DATMSK6_A::_1
    }
}
impl core::ops::Deref for DATMSK6_R {
    type Target = crate::FieldReader<bool, DATMSK6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATMSK6` writer - Port A-f Pin\\[N\\]
Data Output Write Mask\\nThese bits are used to protect the corresponding DOUT (Px_DOUT\\[n\\]) bit. When the DATMSK (Px_DATMSK\\[n\\]) bit is set to 1, the corresponding DOUT (Px_DOUT\\[n\\]) bit is protected. If the write signal is masked, writing data to the protect bit is ignored.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DATMSK6_W<'a> {
    w: &'a mut W,
}
impl<'a> DATMSK6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATMSK6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Corresponding DOUT (Px_DOUT\\[n\\]) bit can be updated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DATMSK6_A::_0)
    }
    #[doc = "Corresponding DOUT (Px_DOUT\\[n\\]) bit protected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DATMSK6_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Port A-f Pin\\[N\\]
Data Output Write Mask\\nThese bits are used to protect the corresponding DOUT (Px_DOUT\\[n\\]) bit. When the DATMSK (Px_DATMSK\\[n\\]) bit is set to 1, the corresponding DOUT (Px_DOUT\\[n\\]) bit is protected. If the write signal is masked, writing data to the protect bit is ignored.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATMSK7_A {
    #[doc = "0: Corresponding DOUT (Px_DOUT\\[n\\]) bit can be updated"]
    _0 = 0,
    #[doc = "1: Corresponding DOUT (Px_DOUT\\[n\\]) bit protected"]
    _1 = 1,
}
impl From<DATMSK7_A> for bool {
    #[inline(always)]
    fn from(variant: DATMSK7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATMSK7` reader - Port A-f Pin\\[N\\]
Data Output Write Mask\\nThese bits are used to protect the corresponding DOUT (Px_DOUT\\[n\\]) bit. When the DATMSK (Px_DATMSK\\[n\\]) bit is set to 1, the corresponding DOUT (Px_DOUT\\[n\\]) bit is protected. If the write signal is masked, writing data to the protect bit is ignored.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DATMSK7_R(crate::FieldReader<bool, DATMSK7_A>);
impl DATMSK7_R {
    pub(crate) fn new(bits: bool) -> Self {
        DATMSK7_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATMSK7_A {
        match self.bits {
            false => DATMSK7_A::_0,
            true => DATMSK7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DATMSK7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DATMSK7_A::_1
    }
}
impl core::ops::Deref for DATMSK7_R {
    type Target = crate::FieldReader<bool, DATMSK7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATMSK7` writer - Port A-f Pin\\[N\\]
Data Output Write Mask\\nThese bits are used to protect the corresponding DOUT (Px_DOUT\\[n\\]) bit. When the DATMSK (Px_DATMSK\\[n\\]) bit is set to 1, the corresponding DOUT (Px_DOUT\\[n\\]) bit is protected. If the write signal is masked, writing data to the protect bit is ignored.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DATMSK7_W<'a> {
    w: &'a mut W,
}
impl<'a> DATMSK7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATMSK7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Corresponding DOUT (Px_DOUT\\[n\\]) bit can be updated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DATMSK7_A::_0)
    }
    #[doc = "Corresponding DOUT (Px_DOUT\\[n\\]) bit protected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DATMSK7_A::_1)
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
#[doc = "Port A-f Pin\\[N\\]
Data Output Write Mask\\nThese bits are used to protect the corresponding DOUT (Px_DOUT\\[n\\]) bit. When the DATMSK (Px_DATMSK\\[n\\]) bit is set to 1, the corresponding DOUT (Px_DOUT\\[n\\]) bit is protected. If the write signal is masked, writing data to the protect bit is ignored.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATMSK8_A {
    #[doc = "0: Corresponding DOUT (Px_DOUT\\[n\\]) bit can be updated"]
    _0 = 0,
    #[doc = "1: Corresponding DOUT (Px_DOUT\\[n\\]) bit protected"]
    _1 = 1,
}
impl From<DATMSK8_A> for bool {
    #[inline(always)]
    fn from(variant: DATMSK8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATMSK8` reader - Port A-f Pin\\[N\\]
Data Output Write Mask\\nThese bits are used to protect the corresponding DOUT (Px_DOUT\\[n\\]) bit. When the DATMSK (Px_DATMSK\\[n\\]) bit is set to 1, the corresponding DOUT (Px_DOUT\\[n\\]) bit is protected. If the write signal is masked, writing data to the protect bit is ignored.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DATMSK8_R(crate::FieldReader<bool, DATMSK8_A>);
impl DATMSK8_R {
    pub(crate) fn new(bits: bool) -> Self {
        DATMSK8_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATMSK8_A {
        match self.bits {
            false => DATMSK8_A::_0,
            true => DATMSK8_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DATMSK8_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DATMSK8_A::_1
    }
}
impl core::ops::Deref for DATMSK8_R {
    type Target = crate::FieldReader<bool, DATMSK8_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATMSK8` writer - Port A-f Pin\\[N\\]
Data Output Write Mask\\nThese bits are used to protect the corresponding DOUT (Px_DOUT\\[n\\]) bit. When the DATMSK (Px_DATMSK\\[n\\]) bit is set to 1, the corresponding DOUT (Px_DOUT\\[n\\]) bit is protected. If the write signal is masked, writing data to the protect bit is ignored.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DATMSK8_W<'a> {
    w: &'a mut W,
}
impl<'a> DATMSK8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATMSK8_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Corresponding DOUT (Px_DOUT\\[n\\]) bit can be updated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DATMSK8_A::_0)
    }
    #[doc = "Corresponding DOUT (Px_DOUT\\[n\\]) bit protected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DATMSK8_A::_1)
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
#[doc = "Port A-f Pin\\[N\\]
Data Output Write Mask\\nThese bits are used to protect the corresponding DOUT (Px_DOUT\\[n\\]) bit. When the DATMSK (Px_DATMSK\\[n\\]) bit is set to 1, the corresponding DOUT (Px_DOUT\\[n\\]) bit is protected. If the write signal is masked, writing data to the protect bit is ignored.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATMSK9_A {
    #[doc = "0: Corresponding DOUT (Px_DOUT\\[n\\]) bit can be updated"]
    _0 = 0,
    #[doc = "1: Corresponding DOUT (Px_DOUT\\[n\\]) bit protected"]
    _1 = 1,
}
impl From<DATMSK9_A> for bool {
    #[inline(always)]
    fn from(variant: DATMSK9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATMSK9` reader - Port A-f Pin\\[N\\]
Data Output Write Mask\\nThese bits are used to protect the corresponding DOUT (Px_DOUT\\[n\\]) bit. When the DATMSK (Px_DATMSK\\[n\\]) bit is set to 1, the corresponding DOUT (Px_DOUT\\[n\\]) bit is protected. If the write signal is masked, writing data to the protect bit is ignored.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DATMSK9_R(crate::FieldReader<bool, DATMSK9_A>);
impl DATMSK9_R {
    pub(crate) fn new(bits: bool) -> Self {
        DATMSK9_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATMSK9_A {
        match self.bits {
            false => DATMSK9_A::_0,
            true => DATMSK9_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DATMSK9_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DATMSK9_A::_1
    }
}
impl core::ops::Deref for DATMSK9_R {
    type Target = crate::FieldReader<bool, DATMSK9_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATMSK9` writer - Port A-f Pin\\[N\\]
Data Output Write Mask\\nThese bits are used to protect the corresponding DOUT (Px_DOUT\\[n\\]) bit. When the DATMSK (Px_DATMSK\\[n\\]) bit is set to 1, the corresponding DOUT (Px_DOUT\\[n\\]) bit is protected. If the write signal is masked, writing data to the protect bit is ignored.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DATMSK9_W<'a> {
    w: &'a mut W,
}
impl<'a> DATMSK9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATMSK9_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Corresponding DOUT (Px_DOUT\\[n\\]) bit can be updated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DATMSK9_A::_0)
    }
    #[doc = "Corresponding DOUT (Px_DOUT\\[n\\]) bit protected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DATMSK9_A::_1)
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
#[doc = "Port A-f Pin\\[N\\]
Data Output Write Mask\\nThese bits are used to protect the corresponding DOUT (Px_DOUT\\[n\\]) bit. When the DATMSK (Px_DATMSK\\[n\\]) bit is set to 1, the corresponding DOUT (Px_DOUT\\[n\\]) bit is protected. If the write signal is masked, writing data to the protect bit is ignored.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATMSK10_A {
    #[doc = "0: Corresponding DOUT (Px_DOUT\\[n\\]) bit can be updated"]
    _0 = 0,
    #[doc = "1: Corresponding DOUT (Px_DOUT\\[n\\]) bit protected"]
    _1 = 1,
}
impl From<DATMSK10_A> for bool {
    #[inline(always)]
    fn from(variant: DATMSK10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATMSK10` reader - Port A-f Pin\\[N\\]
Data Output Write Mask\\nThese bits are used to protect the corresponding DOUT (Px_DOUT\\[n\\]) bit. When the DATMSK (Px_DATMSK\\[n\\]) bit is set to 1, the corresponding DOUT (Px_DOUT\\[n\\]) bit is protected. If the write signal is masked, writing data to the protect bit is ignored.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DATMSK10_R(crate::FieldReader<bool, DATMSK10_A>);
impl DATMSK10_R {
    pub(crate) fn new(bits: bool) -> Self {
        DATMSK10_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATMSK10_A {
        match self.bits {
            false => DATMSK10_A::_0,
            true => DATMSK10_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DATMSK10_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DATMSK10_A::_1
    }
}
impl core::ops::Deref for DATMSK10_R {
    type Target = crate::FieldReader<bool, DATMSK10_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATMSK10` writer - Port A-f Pin\\[N\\]
Data Output Write Mask\\nThese bits are used to protect the corresponding DOUT (Px_DOUT\\[n\\]) bit. When the DATMSK (Px_DATMSK\\[n\\]) bit is set to 1, the corresponding DOUT (Px_DOUT\\[n\\]) bit is protected. If the write signal is masked, writing data to the protect bit is ignored.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DATMSK10_W<'a> {
    w: &'a mut W,
}
impl<'a> DATMSK10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATMSK10_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Corresponding DOUT (Px_DOUT\\[n\\]) bit can be updated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DATMSK10_A::_0)
    }
    #[doc = "Corresponding DOUT (Px_DOUT\\[n\\]) bit protected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DATMSK10_A::_1)
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
#[doc = "Port A-f Pin\\[N\\]
Data Output Write Mask\\nThese bits are used to protect the corresponding DOUT (Px_DOUT\\[n\\]) bit. When the DATMSK (Px_DATMSK\\[n\\]) bit is set to 1, the corresponding DOUT (Px_DOUT\\[n\\]) bit is protected. If the write signal is masked, writing data to the protect bit is ignored.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATMSK11_A {
    #[doc = "0: Corresponding DOUT (Px_DOUT\\[n\\]) bit can be updated"]
    _0 = 0,
    #[doc = "1: Corresponding DOUT (Px_DOUT\\[n\\]) bit protected"]
    _1 = 1,
}
impl From<DATMSK11_A> for bool {
    #[inline(always)]
    fn from(variant: DATMSK11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATMSK11` reader - Port A-f Pin\\[N\\]
Data Output Write Mask\\nThese bits are used to protect the corresponding DOUT (Px_DOUT\\[n\\]) bit. When the DATMSK (Px_DATMSK\\[n\\]) bit is set to 1, the corresponding DOUT (Px_DOUT\\[n\\]) bit is protected. If the write signal is masked, writing data to the protect bit is ignored.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DATMSK11_R(crate::FieldReader<bool, DATMSK11_A>);
impl DATMSK11_R {
    pub(crate) fn new(bits: bool) -> Self {
        DATMSK11_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATMSK11_A {
        match self.bits {
            false => DATMSK11_A::_0,
            true => DATMSK11_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DATMSK11_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DATMSK11_A::_1
    }
}
impl core::ops::Deref for DATMSK11_R {
    type Target = crate::FieldReader<bool, DATMSK11_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATMSK11` writer - Port A-f Pin\\[N\\]
Data Output Write Mask\\nThese bits are used to protect the corresponding DOUT (Px_DOUT\\[n\\]) bit. When the DATMSK (Px_DATMSK\\[n\\]) bit is set to 1, the corresponding DOUT (Px_DOUT\\[n\\]) bit is protected. If the write signal is masked, writing data to the protect bit is ignored.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DATMSK11_W<'a> {
    w: &'a mut W,
}
impl<'a> DATMSK11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATMSK11_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Corresponding DOUT (Px_DOUT\\[n\\]) bit can be updated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DATMSK11_A::_0)
    }
    #[doc = "Corresponding DOUT (Px_DOUT\\[n\\]) bit protected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DATMSK11_A::_1)
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
#[doc = "Port A-f Pin\\[N\\]
Data Output Write Mask\\nThese bits are used to protect the corresponding DOUT (Px_DOUT\\[n\\]) bit. When the DATMSK (Px_DATMSK\\[n\\]) bit is set to 1, the corresponding DOUT (Px_DOUT\\[n\\]) bit is protected. If the write signal is masked, writing data to the protect bit is ignored.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATMSK12_A {
    #[doc = "0: Corresponding DOUT (Px_DOUT\\[n\\]) bit can be updated"]
    _0 = 0,
    #[doc = "1: Corresponding DOUT (Px_DOUT\\[n\\]) bit protected"]
    _1 = 1,
}
impl From<DATMSK12_A> for bool {
    #[inline(always)]
    fn from(variant: DATMSK12_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATMSK12` reader - Port A-f Pin\\[N\\]
Data Output Write Mask\\nThese bits are used to protect the corresponding DOUT (Px_DOUT\\[n\\]) bit. When the DATMSK (Px_DATMSK\\[n\\]) bit is set to 1, the corresponding DOUT (Px_DOUT\\[n\\]) bit is protected. If the write signal is masked, writing data to the protect bit is ignored.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DATMSK12_R(crate::FieldReader<bool, DATMSK12_A>);
impl DATMSK12_R {
    pub(crate) fn new(bits: bool) -> Self {
        DATMSK12_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATMSK12_A {
        match self.bits {
            false => DATMSK12_A::_0,
            true => DATMSK12_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DATMSK12_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DATMSK12_A::_1
    }
}
impl core::ops::Deref for DATMSK12_R {
    type Target = crate::FieldReader<bool, DATMSK12_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATMSK12` writer - Port A-f Pin\\[N\\]
Data Output Write Mask\\nThese bits are used to protect the corresponding DOUT (Px_DOUT\\[n\\]) bit. When the DATMSK (Px_DATMSK\\[n\\]) bit is set to 1, the corresponding DOUT (Px_DOUT\\[n\\]) bit is protected. If the write signal is masked, writing data to the protect bit is ignored.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DATMSK12_W<'a> {
    w: &'a mut W,
}
impl<'a> DATMSK12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATMSK12_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Corresponding DOUT (Px_DOUT\\[n\\]) bit can be updated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DATMSK12_A::_0)
    }
    #[doc = "Corresponding DOUT (Px_DOUT\\[n\\]) bit protected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DATMSK12_A::_1)
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
#[doc = "Port A-f Pin\\[N\\]
Data Output Write Mask\\nThese bits are used to protect the corresponding DOUT (Px_DOUT\\[n\\]) bit. When the DATMSK (Px_DATMSK\\[n\\]) bit is set to 1, the corresponding DOUT (Px_DOUT\\[n\\]) bit is protected. If the write signal is masked, writing data to the protect bit is ignored.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATMSK13_A {
    #[doc = "0: Corresponding DOUT (Px_DOUT\\[n\\]) bit can be updated"]
    _0 = 0,
    #[doc = "1: Corresponding DOUT (Px_DOUT\\[n\\]) bit protected"]
    _1 = 1,
}
impl From<DATMSK13_A> for bool {
    #[inline(always)]
    fn from(variant: DATMSK13_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATMSK13` reader - Port A-f Pin\\[N\\]
Data Output Write Mask\\nThese bits are used to protect the corresponding DOUT (Px_DOUT\\[n\\]) bit. When the DATMSK (Px_DATMSK\\[n\\]) bit is set to 1, the corresponding DOUT (Px_DOUT\\[n\\]) bit is protected. If the write signal is masked, writing data to the protect bit is ignored.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DATMSK13_R(crate::FieldReader<bool, DATMSK13_A>);
impl DATMSK13_R {
    pub(crate) fn new(bits: bool) -> Self {
        DATMSK13_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATMSK13_A {
        match self.bits {
            false => DATMSK13_A::_0,
            true => DATMSK13_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DATMSK13_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DATMSK13_A::_1
    }
}
impl core::ops::Deref for DATMSK13_R {
    type Target = crate::FieldReader<bool, DATMSK13_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATMSK13` writer - Port A-f Pin\\[N\\]
Data Output Write Mask\\nThese bits are used to protect the corresponding DOUT (Px_DOUT\\[n\\]) bit. When the DATMSK (Px_DATMSK\\[n\\]) bit is set to 1, the corresponding DOUT (Px_DOUT\\[n\\]) bit is protected. If the write signal is masked, writing data to the protect bit is ignored.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DATMSK13_W<'a> {
    w: &'a mut W,
}
impl<'a> DATMSK13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATMSK13_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Corresponding DOUT (Px_DOUT\\[n\\]) bit can be updated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DATMSK13_A::_0)
    }
    #[doc = "Corresponding DOUT (Px_DOUT\\[n\\]) bit protected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DATMSK13_A::_1)
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
#[doc = "Port A-f Pin\\[N\\]
Data Output Write Mask\\nThese bits are used to protect the corresponding DOUT (Px_DOUT\\[n\\]) bit. When the DATMSK (Px_DATMSK\\[n\\]) bit is set to 1, the corresponding DOUT (Px_DOUT\\[n\\]) bit is protected. If the write signal is masked, writing data to the protect bit is ignored.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATMSK14_A {
    #[doc = "0: Corresponding DOUT (Px_DOUT\\[n\\]) bit can be updated"]
    _0 = 0,
    #[doc = "1: Corresponding DOUT (Px_DOUT\\[n\\]) bit protected"]
    _1 = 1,
}
impl From<DATMSK14_A> for bool {
    #[inline(always)]
    fn from(variant: DATMSK14_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATMSK14` reader - Port A-f Pin\\[N\\]
Data Output Write Mask\\nThese bits are used to protect the corresponding DOUT (Px_DOUT\\[n\\]) bit. When the DATMSK (Px_DATMSK\\[n\\]) bit is set to 1, the corresponding DOUT (Px_DOUT\\[n\\]) bit is protected. If the write signal is masked, writing data to the protect bit is ignored.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DATMSK14_R(crate::FieldReader<bool, DATMSK14_A>);
impl DATMSK14_R {
    pub(crate) fn new(bits: bool) -> Self {
        DATMSK14_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATMSK14_A {
        match self.bits {
            false => DATMSK14_A::_0,
            true => DATMSK14_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DATMSK14_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DATMSK14_A::_1
    }
}
impl core::ops::Deref for DATMSK14_R {
    type Target = crate::FieldReader<bool, DATMSK14_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATMSK14` writer - Port A-f Pin\\[N\\]
Data Output Write Mask\\nThese bits are used to protect the corresponding DOUT (Px_DOUT\\[n\\]) bit. When the DATMSK (Px_DATMSK\\[n\\]) bit is set to 1, the corresponding DOUT (Px_DOUT\\[n\\]) bit is protected. If the write signal is masked, writing data to the protect bit is ignored.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DATMSK14_W<'a> {
    w: &'a mut W,
}
impl<'a> DATMSK14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATMSK14_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Corresponding DOUT (Px_DOUT\\[n\\]) bit can be updated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DATMSK14_A::_0)
    }
    #[doc = "Corresponding DOUT (Px_DOUT\\[n\\]) bit protected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DATMSK14_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Port A-f Pin\\[N\\]
Data Output Write Mask\\nThese bits are used to protect the corresponding DOUT (Px_DOUT\\[n\\]) bit. When the DATMSK (Px_DATMSK\\[n\\]) bit is set to 1, the corresponding DOUT (Px_DOUT\\[n\\]) bit is protected. If the write signal is masked, writing data to the protect bit is ignored.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATMSK15_A {
    #[doc = "0: Corresponding DOUT (Px_DOUT\\[n\\]) bit can be updated"]
    _0 = 0,
    #[doc = "1: Corresponding DOUT (Px_DOUT\\[n\\]) bit protected"]
    _1 = 1,
}
impl From<DATMSK15_A> for bool {
    #[inline(always)]
    fn from(variant: DATMSK15_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATMSK15` reader - Port A-f Pin\\[N\\]
Data Output Write Mask\\nThese bits are used to protect the corresponding DOUT (Px_DOUT\\[n\\]) bit. When the DATMSK (Px_DATMSK\\[n\\]) bit is set to 1, the corresponding DOUT (Px_DOUT\\[n\\]) bit is protected. If the write signal is masked, writing data to the protect bit is ignored.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DATMSK15_R(crate::FieldReader<bool, DATMSK15_A>);
impl DATMSK15_R {
    pub(crate) fn new(bits: bool) -> Self {
        DATMSK15_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATMSK15_A {
        match self.bits {
            false => DATMSK15_A::_0,
            true => DATMSK15_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DATMSK15_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DATMSK15_A::_1
    }
}
impl core::ops::Deref for DATMSK15_R {
    type Target = crate::FieldReader<bool, DATMSK15_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATMSK15` writer - Port A-f Pin\\[N\\]
Data Output Write Mask\\nThese bits are used to protect the corresponding DOUT (Px_DOUT\\[n\\]) bit. When the DATMSK (Px_DATMSK\\[n\\]) bit is set to 1, the corresponding DOUT (Px_DOUT\\[n\\]) bit is protected. If the write signal is masked, writing data to the protect bit is ignored.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
pub struct DATMSK15_W<'a> {
    w: &'a mut W,
}
impl<'a> DATMSK15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATMSK15_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Corresponding DOUT (Px_DOUT\\[n\\]) bit can be updated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DATMSK15_A::_0)
    }
    #[doc = "Corresponding DOUT (Px_DOUT\\[n\\]) bit protected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DATMSK15_A::_1)
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
impl R {
    #[doc = "Bit 0 - Port A-f Pin\\[N\\]
Data Output Write Mask\\nThese bits are used to protect the corresponding DOUT (Px_DOUT\\[n\\]) bit. When the DATMSK (Px_DATMSK\\[n\\]) bit is set to 1, the corresponding DOUT (Px_DOUT\\[n\\]) bit is protected. If the write signal is masked, writing data to the protect bit is ignored.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn datmsk0(&self) -> DATMSK0_R {
        DATMSK0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Port A-f Pin\\[N\\]
Data Output Write Mask\\nThese bits are used to protect the corresponding DOUT (Px_DOUT\\[n\\]) bit. When the DATMSK (Px_DATMSK\\[n\\]) bit is set to 1, the corresponding DOUT (Px_DOUT\\[n\\]) bit is protected. If the write signal is masked, writing data to the protect bit is ignored.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn datmsk1(&self) -> DATMSK1_R {
        DATMSK1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Port A-f Pin\\[N\\]
Data Output Write Mask\\nThese bits are used to protect the corresponding DOUT (Px_DOUT\\[n\\]) bit. When the DATMSK (Px_DATMSK\\[n\\]) bit is set to 1, the corresponding DOUT (Px_DOUT\\[n\\]) bit is protected. If the write signal is masked, writing data to the protect bit is ignored.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn datmsk2(&self) -> DATMSK2_R {
        DATMSK2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Port A-f Pin\\[N\\]
Data Output Write Mask\\nThese bits are used to protect the corresponding DOUT (Px_DOUT\\[n\\]) bit. When the DATMSK (Px_DATMSK\\[n\\]) bit is set to 1, the corresponding DOUT (Px_DOUT\\[n\\]) bit is protected. If the write signal is masked, writing data to the protect bit is ignored.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn datmsk3(&self) -> DATMSK3_R {
        DATMSK3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Port A-f Pin\\[N\\]
Data Output Write Mask\\nThese bits are used to protect the corresponding DOUT (Px_DOUT\\[n\\]) bit. When the DATMSK (Px_DATMSK\\[n\\]) bit is set to 1, the corresponding DOUT (Px_DOUT\\[n\\]) bit is protected. If the write signal is masked, writing data to the protect bit is ignored.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn datmsk4(&self) -> DATMSK4_R {
        DATMSK4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Port A-f Pin\\[N\\]
Data Output Write Mask\\nThese bits are used to protect the corresponding DOUT (Px_DOUT\\[n\\]) bit. When the DATMSK (Px_DATMSK\\[n\\]) bit is set to 1, the corresponding DOUT (Px_DOUT\\[n\\]) bit is protected. If the write signal is masked, writing data to the protect bit is ignored.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn datmsk5(&self) -> DATMSK5_R {
        DATMSK5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Port A-f Pin\\[N\\]
Data Output Write Mask\\nThese bits are used to protect the corresponding DOUT (Px_DOUT\\[n\\]) bit. When the DATMSK (Px_DATMSK\\[n\\]) bit is set to 1, the corresponding DOUT (Px_DOUT\\[n\\]) bit is protected. If the write signal is masked, writing data to the protect bit is ignored.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn datmsk6(&self) -> DATMSK6_R {
        DATMSK6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Port A-f Pin\\[N\\]
Data Output Write Mask\\nThese bits are used to protect the corresponding DOUT (Px_DOUT\\[n\\]) bit. When the DATMSK (Px_DATMSK\\[n\\]) bit is set to 1, the corresponding DOUT (Px_DOUT\\[n\\]) bit is protected. If the write signal is masked, writing data to the protect bit is ignored.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn datmsk7(&self) -> DATMSK7_R {
        DATMSK7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Port A-f Pin\\[N\\]
Data Output Write Mask\\nThese bits are used to protect the corresponding DOUT (Px_DOUT\\[n\\]) bit. When the DATMSK (Px_DATMSK\\[n\\]) bit is set to 1, the corresponding DOUT (Px_DOUT\\[n\\]) bit is protected. If the write signal is masked, writing data to the protect bit is ignored.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn datmsk8(&self) -> DATMSK8_R {
        DATMSK8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Port A-f Pin\\[N\\]
Data Output Write Mask\\nThese bits are used to protect the corresponding DOUT (Px_DOUT\\[n\\]) bit. When the DATMSK (Px_DATMSK\\[n\\]) bit is set to 1, the corresponding DOUT (Px_DOUT\\[n\\]) bit is protected. If the write signal is masked, writing data to the protect bit is ignored.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn datmsk9(&self) -> DATMSK9_R {
        DATMSK9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Port A-f Pin\\[N\\]
Data Output Write Mask\\nThese bits are used to protect the corresponding DOUT (Px_DOUT\\[n\\]) bit. When the DATMSK (Px_DATMSK\\[n\\]) bit is set to 1, the corresponding DOUT (Px_DOUT\\[n\\]) bit is protected. If the write signal is masked, writing data to the protect bit is ignored.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn datmsk10(&self) -> DATMSK10_R {
        DATMSK10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Port A-f Pin\\[N\\]
Data Output Write Mask\\nThese bits are used to protect the corresponding DOUT (Px_DOUT\\[n\\]) bit. When the DATMSK (Px_DATMSK\\[n\\]) bit is set to 1, the corresponding DOUT (Px_DOUT\\[n\\]) bit is protected. If the write signal is masked, writing data to the protect bit is ignored.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn datmsk11(&self) -> DATMSK11_R {
        DATMSK11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Port A-f Pin\\[N\\]
Data Output Write Mask\\nThese bits are used to protect the corresponding DOUT (Px_DOUT\\[n\\]) bit. When the DATMSK (Px_DATMSK\\[n\\]) bit is set to 1, the corresponding DOUT (Px_DOUT\\[n\\]) bit is protected. If the write signal is masked, writing data to the protect bit is ignored.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn datmsk12(&self) -> DATMSK12_R {
        DATMSK12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Port A-f Pin\\[N\\]
Data Output Write Mask\\nThese bits are used to protect the corresponding DOUT (Px_DOUT\\[n\\]) bit. When the DATMSK (Px_DATMSK\\[n\\]) bit is set to 1, the corresponding DOUT (Px_DOUT\\[n\\]) bit is protected. If the write signal is masked, writing data to the protect bit is ignored.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn datmsk13(&self) -> DATMSK13_R {
        DATMSK13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Port A-f Pin\\[N\\]
Data Output Write Mask\\nThese bits are used to protect the corresponding DOUT (Px_DOUT\\[n\\]) bit. When the DATMSK (Px_DATMSK\\[n\\]) bit is set to 1, the corresponding DOUT (Px_DOUT\\[n\\]) bit is protected. If the write signal is masked, writing data to the protect bit is ignored.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn datmsk14(&self) -> DATMSK14_R {
        DATMSK14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Port A-f Pin\\[N\\]
Data Output Write Mask\\nThese bits are used to protect the corresponding DOUT (Px_DOUT\\[n\\]) bit. When the DATMSK (Px_DATMSK\\[n\\]) bit is set to 1, the corresponding DOUT (Px_DOUT\\[n\\]) bit is protected. If the write signal is masked, writing data to the protect bit is ignored.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn datmsk15(&self) -> DATMSK15_R {
        DATMSK15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port A-f Pin\\[N\\]
Data Output Write Mask\\nThese bits are used to protect the corresponding DOUT (Px_DOUT\\[n\\]) bit. When the DATMSK (Px_DATMSK\\[n\\]) bit is set to 1, the corresponding DOUT (Px_DOUT\\[n\\]) bit is protected. If the write signal is masked, writing data to the protect bit is ignored.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn datmsk0(&mut self) -> DATMSK0_W {
        DATMSK0_W { w: self }
    }
    #[doc = "Bit 1 - Port A-f Pin\\[N\\]
Data Output Write Mask\\nThese bits are used to protect the corresponding DOUT (Px_DOUT\\[n\\]) bit. When the DATMSK (Px_DATMSK\\[n\\]) bit is set to 1, the corresponding DOUT (Px_DOUT\\[n\\]) bit is protected. If the write signal is masked, writing data to the protect bit is ignored.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn datmsk1(&mut self) -> DATMSK1_W {
        DATMSK1_W { w: self }
    }
    #[doc = "Bit 2 - Port A-f Pin\\[N\\]
Data Output Write Mask\\nThese bits are used to protect the corresponding DOUT (Px_DOUT\\[n\\]) bit. When the DATMSK (Px_DATMSK\\[n\\]) bit is set to 1, the corresponding DOUT (Px_DOUT\\[n\\]) bit is protected. If the write signal is masked, writing data to the protect bit is ignored.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn datmsk2(&mut self) -> DATMSK2_W {
        DATMSK2_W { w: self }
    }
    #[doc = "Bit 3 - Port A-f Pin\\[N\\]
Data Output Write Mask\\nThese bits are used to protect the corresponding DOUT (Px_DOUT\\[n\\]) bit. When the DATMSK (Px_DATMSK\\[n\\]) bit is set to 1, the corresponding DOUT (Px_DOUT\\[n\\]) bit is protected. If the write signal is masked, writing data to the protect bit is ignored.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn datmsk3(&mut self) -> DATMSK3_W {
        DATMSK3_W { w: self }
    }
    #[doc = "Bit 4 - Port A-f Pin\\[N\\]
Data Output Write Mask\\nThese bits are used to protect the corresponding DOUT (Px_DOUT\\[n\\]) bit. When the DATMSK (Px_DATMSK\\[n\\]) bit is set to 1, the corresponding DOUT (Px_DOUT\\[n\\]) bit is protected. If the write signal is masked, writing data to the protect bit is ignored.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn datmsk4(&mut self) -> DATMSK4_W {
        DATMSK4_W { w: self }
    }
    #[doc = "Bit 5 - Port A-f Pin\\[N\\]
Data Output Write Mask\\nThese bits are used to protect the corresponding DOUT (Px_DOUT\\[n\\]) bit. When the DATMSK (Px_DATMSK\\[n\\]) bit is set to 1, the corresponding DOUT (Px_DOUT\\[n\\]) bit is protected. If the write signal is masked, writing data to the protect bit is ignored.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn datmsk5(&mut self) -> DATMSK5_W {
        DATMSK5_W { w: self }
    }
    #[doc = "Bit 6 - Port A-f Pin\\[N\\]
Data Output Write Mask\\nThese bits are used to protect the corresponding DOUT (Px_DOUT\\[n\\]) bit. When the DATMSK (Px_DATMSK\\[n\\]) bit is set to 1, the corresponding DOUT (Px_DOUT\\[n\\]) bit is protected. If the write signal is masked, writing data to the protect bit is ignored.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn datmsk6(&mut self) -> DATMSK6_W {
        DATMSK6_W { w: self }
    }
    #[doc = "Bit 7 - Port A-f Pin\\[N\\]
Data Output Write Mask\\nThese bits are used to protect the corresponding DOUT (Px_DOUT\\[n\\]) bit. When the DATMSK (Px_DATMSK\\[n\\]) bit is set to 1, the corresponding DOUT (Px_DOUT\\[n\\]) bit is protected. If the write signal is masked, writing data to the protect bit is ignored.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn datmsk7(&mut self) -> DATMSK7_W {
        DATMSK7_W { w: self }
    }
    #[doc = "Bit 8 - Port A-f Pin\\[N\\]
Data Output Write Mask\\nThese bits are used to protect the corresponding DOUT (Px_DOUT\\[n\\]) bit. When the DATMSK (Px_DATMSK\\[n\\]) bit is set to 1, the corresponding DOUT (Px_DOUT\\[n\\]) bit is protected. If the write signal is masked, writing data to the protect bit is ignored.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn datmsk8(&mut self) -> DATMSK8_W {
        DATMSK8_W { w: self }
    }
    #[doc = "Bit 9 - Port A-f Pin\\[N\\]
Data Output Write Mask\\nThese bits are used to protect the corresponding DOUT (Px_DOUT\\[n\\]) bit. When the DATMSK (Px_DATMSK\\[n\\]) bit is set to 1, the corresponding DOUT (Px_DOUT\\[n\\]) bit is protected. If the write signal is masked, writing data to the protect bit is ignored.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn datmsk9(&mut self) -> DATMSK9_W {
        DATMSK9_W { w: self }
    }
    #[doc = "Bit 10 - Port A-f Pin\\[N\\]
Data Output Write Mask\\nThese bits are used to protect the corresponding DOUT (Px_DOUT\\[n\\]) bit. When the DATMSK (Px_DATMSK\\[n\\]) bit is set to 1, the corresponding DOUT (Px_DOUT\\[n\\]) bit is protected. If the write signal is masked, writing data to the protect bit is ignored.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn datmsk10(&mut self) -> DATMSK10_W {
        DATMSK10_W { w: self }
    }
    #[doc = "Bit 11 - Port A-f Pin\\[N\\]
Data Output Write Mask\\nThese bits are used to protect the corresponding DOUT (Px_DOUT\\[n\\]) bit. When the DATMSK (Px_DATMSK\\[n\\]) bit is set to 1, the corresponding DOUT (Px_DOUT\\[n\\]) bit is protected. If the write signal is masked, writing data to the protect bit is ignored.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn datmsk11(&mut self) -> DATMSK11_W {
        DATMSK11_W { w: self }
    }
    #[doc = "Bit 12 - Port A-f Pin\\[N\\]
Data Output Write Mask\\nThese bits are used to protect the corresponding DOUT (Px_DOUT\\[n\\]) bit. When the DATMSK (Px_DATMSK\\[n\\]) bit is set to 1, the corresponding DOUT (Px_DOUT\\[n\\]) bit is protected. If the write signal is masked, writing data to the protect bit is ignored.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn datmsk12(&mut self) -> DATMSK12_W {
        DATMSK12_W { w: self }
    }
    #[doc = "Bit 13 - Port A-f Pin\\[N\\]
Data Output Write Mask\\nThese bits are used to protect the corresponding DOUT (Px_DOUT\\[n\\]) bit. When the DATMSK (Px_DATMSK\\[n\\]) bit is set to 1, the corresponding DOUT (Px_DOUT\\[n\\]) bit is protected. If the write signal is masked, writing data to the protect bit is ignored.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn datmsk13(&mut self) -> DATMSK13_W {
        DATMSK13_W { w: self }
    }
    #[doc = "Bit 14 - Port A-f Pin\\[N\\]
Data Output Write Mask\\nThese bits are used to protect the corresponding DOUT (Px_DOUT\\[n\\]) bit. When the DATMSK (Px_DATMSK\\[n\\]) bit is set to 1, the corresponding DOUT (Px_DOUT\\[n\\]) bit is protected. If the write signal is masked, writing data to the protect bit is ignored.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn datmsk14(&mut self) -> DATMSK14_W {
        DATMSK14_W { w: self }
    }
    #[doc = "Bit 15 - Port A-f Pin\\[N\\]
Data Output Write Mask\\nThese bits are used to protect the corresponding DOUT (Px_DOUT\\[n\\]) bit. When the DATMSK (Px_DATMSK\\[n\\]) bit is set to 1, the corresponding DOUT (Px_DOUT\\[n\\]) bit is protected. If the write signal is masked, writing data to the protect bit is ignored.\\nNote3: The PB.9/PB.10/PC.9/PC.14/PC.15/PD.10/PD.11/PE.2/PE.6/PE.7/PE.14 pin is ignored for M45xD/M45xC."]
    #[inline(always)]
    pub fn datmsk15(&mut self) -> DATMSK15_W {
        DATMSK15_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PC Data Output Write Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pc_datmsk](index.html) module"]
pub struct PC_DATMSK_SPEC;
impl crate::RegisterSpec for PC_DATMSK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pc_datmsk::R](R) reader structure"]
impl crate::Readable for PC_DATMSK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pc_datmsk::W](W) writer structure"]
impl crate::Writable for PC_DATMSK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PC_DATMSK to value 0"]
impl crate::Resettable for PC_DATMSK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
