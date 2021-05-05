#[doc = "Register `CAN_CON` reader"]
pub struct R(crate::R<CAN_CON_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAN_CON_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CAN_CON_SPEC>> for R {
    fn from(reader: crate::R<CAN_CON_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAN_CON` writer"]
pub struct W(crate::W<CAN_CON_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAN_CON_SPEC>;
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
impl core::convert::From<crate::W<CAN_CON_SPEC>> for W {
    fn from(writer: crate::W<CAN_CON_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Init Initialization\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INIT_A {
    #[doc = "0: Normal Operation"]
    _0 = 0,
    #[doc = "1: Initialization is started"]
    _1 = 1,
}
impl From<INIT_A> for bool {
    #[inline(always)]
    fn from(variant: INIT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Init` reader - Init Initialization"]
pub struct INIT_R(crate::FieldReader<bool, INIT_A>);
impl INIT_R {
    pub(crate) fn new(bits: bool) -> Self {
        INIT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INIT_A {
        match self.bits {
            false => INIT_A::_0,
            true => INIT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == INIT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == INIT_A::_1
    }
}
impl core::ops::Deref for INIT_R {
    type Target = crate::FieldReader<bool, INIT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Init` writer - Init Initialization"]
pub struct INIT_W<'a> {
    w: &'a mut W,
}
impl<'a> INIT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INIT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Normal Operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INIT_A::_0)
    }
    #[doc = "Initialization is started"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INIT_A::_1)
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
#[doc = "Module Interrupt Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IE_A {
    #[doc = "0: Module Interrupt Disabled"]
    _0 = 0,
    #[doc = "1: Module Interrupt Enabled"]
    _1 = 1,
}
impl From<IE_A> for bool {
    #[inline(always)]
    fn from(variant: IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IE` reader - Module Interrupt Enable Bit"]
pub struct IE_R(crate::FieldReader<bool, IE_A>);
impl IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        IE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IE_A {
        match self.bits {
            false => IE_A::_0,
            true => IE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == IE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == IE_A::_1
    }
}
impl core::ops::Deref for IE_R {
    type Target = crate::FieldReader<bool, IE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IE` writer - Module Interrupt Enable Bit"]
pub struct IE_W<'a> {
    w: &'a mut W,
}
impl<'a> IE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Module Interrupt Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IE_A::_0)
    }
    #[doc = "Module Interrupt Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IE_A::_1)
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
#[doc = "Status Change Interrupt Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SIE_A {
    #[doc = "0: Disabled - No Status Change Interrupt will be generated"]
    _0 = 0,
    #[doc = "1: Enabled - An interrupt will be generated when a message transfer is successfully completed or a CAN bus error is detected"]
    _1 = 1,
}
impl From<SIE_A> for bool {
    #[inline(always)]
    fn from(variant: SIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SIE` reader - Status Change Interrupt Enable Bit"]
pub struct SIE_R(crate::FieldReader<bool, SIE_A>);
impl SIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SIE_A {
        match self.bits {
            false => SIE_A::_0,
            true => SIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == SIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == SIE_A::_1
    }
}
impl core::ops::Deref for SIE_R {
    type Target = crate::FieldReader<bool, SIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SIE` writer - Status Change Interrupt Enable Bit"]
pub struct SIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled - No Status Change Interrupt will be generated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SIE_A::_0)
    }
    #[doc = "Enabled - An interrupt will be generated when a message transfer is successfully completed or a CAN bus error is detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SIE_A::_1)
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
#[doc = "Error Interrupt Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EIE_A {
    #[doc = "0: Disabled - No Error Status Interrupt will be generated"]
    _0 = 0,
    #[doc = "1: Enabled - A change in the bits BOff (CAN_STATUS\\[7\\]) or EWarn (CAN_STATUS\\[6\\]) in the Status Register will generate an interrupt"]
    _1 = 1,
}
impl From<EIE_A> for bool {
    #[inline(always)]
    fn from(variant: EIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EIE` reader - Error Interrupt Enable Bit"]
pub struct EIE_R(crate::FieldReader<bool, EIE_A>);
impl EIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        EIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EIE_A {
        match self.bits {
            false => EIE_A::_0,
            true => EIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == EIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == EIE_A::_1
    }
}
impl core::ops::Deref for EIE_R {
    type Target = crate::FieldReader<bool, EIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EIE` writer - Error Interrupt Enable Bit"]
pub struct EIE_W<'a> {
    w: &'a mut W,
}
impl<'a> EIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled - No Error Status Interrupt will be generated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EIE_A::_0)
    }
    #[doc = "Enabled - A change in the bits BOff (CAN_STATUS\\[7\\]) or EWarn (CAN_STATUS\\[6\\]) in the Status Register will generate an interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EIE_A::_1)
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
#[doc = "Automatic Re-transmission Disable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DAR_A {
    #[doc = "0: Automatic Retransmission of disturbed messages Enabled"]
    _0 = 0,
    #[doc = "1: Automatic Retransmission Disabled"]
    _1 = 1,
}
impl From<DAR_A> for bool {
    #[inline(always)]
    fn from(variant: DAR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DAR` reader - Automatic Re-transmission Disable Bit"]
pub struct DAR_R(crate::FieldReader<bool, DAR_A>);
impl DAR_R {
    pub(crate) fn new(bits: bool) -> Self {
        DAR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DAR_A {
        match self.bits {
            false => DAR_A::_0,
            true => DAR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DAR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DAR_A::_1
    }
}
impl core::ops::Deref for DAR_R {
    type Target = crate::FieldReader<bool, DAR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAR` writer - Automatic Re-transmission Disable Bit"]
pub struct DAR_W<'a> {
    w: &'a mut W,
}
impl<'a> DAR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DAR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Automatic Retransmission of disturbed messages Enabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DAR_A::_0)
    }
    #[doc = "Automatic Retransmission Disabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DAR_A::_1)
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
#[doc = "Configuration Change Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCE_A {
    #[doc = "0: No write access to the Bit Timing Register"]
    _0 = 0,
    #[doc = "1: Write access to the Bit Timing Register (CAN_BTIME) allowed. (while Init bit (CAN_CON\\[0\\]) = 1)"]
    _1 = 1,
}
impl From<CCE_A> for bool {
    #[inline(always)]
    fn from(variant: CCE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCE` reader - Configuration Change Enable Bit"]
pub struct CCE_R(crate::FieldReader<bool, CCE_A>);
impl CCE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CCE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCE_A {
        match self.bits {
            false => CCE_A::_0,
            true => CCE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CCE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CCE_A::_1
    }
}
impl core::ops::Deref for CCE_R {
    type Target = crate::FieldReader<bool, CCE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCE` writer - Configuration Change Enable Bit"]
pub struct CCE_W<'a> {
    w: &'a mut W,
}
impl<'a> CCE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No write access to the Bit Timing Register"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CCE_A::_0)
    }
    #[doc = "Write access to the Bit Timing Register (CAN_BTIME) allowed. (while Init bit (CAN_CON\\[0\\]) = 1)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CCE_A::_1)
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
#[doc = "Test Mode Enable Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEST_A {
    #[doc = "0: Normal Operation"]
    _0 = 0,
    #[doc = "1: Test Mode"]
    _1 = 1,
}
impl From<TEST_A> for bool {
    #[inline(always)]
    fn from(variant: TEST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Test` reader - Test Mode Enable Bit"]
pub struct TEST_R(crate::FieldReader<bool, TEST_A>);
impl TEST_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEST_A {
        match self.bits {
            false => TEST_A::_0,
            true => TEST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == TEST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TEST_A::_1
    }
}
impl core::ops::Deref for TEST_R {
    type Target = crate::FieldReader<bool, TEST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Test` writer - Test Mode Enable Bit"]
pub struct TEST_W<'a> {
    w: &'a mut W,
}
impl<'a> TEST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TEST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Normal Operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TEST_A::_0)
    }
    #[doc = "Test Mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TEST_A::_1)
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
impl R {
    #[doc = "Bit 0 - Init Initialization"]
    #[inline(always)]
    pub fn init(&self) -> INIT_R {
        INIT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Module Interrupt Enable Bit"]
    #[inline(always)]
    pub fn ie(&self) -> IE_R {
        IE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Status Change Interrupt Enable Bit"]
    #[inline(always)]
    pub fn sie(&self) -> SIE_R {
        SIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Error Interrupt Enable Bit"]
    #[inline(always)]
    pub fn eie(&self) -> EIE_R {
        EIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Automatic Re-transmission Disable Bit"]
    #[inline(always)]
    pub fn dar(&self) -> DAR_R {
        DAR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Configuration Change Enable Bit"]
    #[inline(always)]
    pub fn cce(&self) -> CCE_R {
        CCE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Test Mode Enable Bit"]
    #[inline(always)]
    pub fn test(&self) -> TEST_R {
        TEST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Init Initialization"]
    #[inline(always)]
    pub fn init(&mut self) -> INIT_W {
        INIT_W { w: self }
    }
    #[doc = "Bit 1 - Module Interrupt Enable Bit"]
    #[inline(always)]
    pub fn ie(&mut self) -> IE_W {
        IE_W { w: self }
    }
    #[doc = "Bit 2 - Status Change Interrupt Enable Bit"]
    #[inline(always)]
    pub fn sie(&mut self) -> SIE_W {
        SIE_W { w: self }
    }
    #[doc = "Bit 3 - Error Interrupt Enable Bit"]
    #[inline(always)]
    pub fn eie(&mut self) -> EIE_W {
        EIE_W { w: self }
    }
    #[doc = "Bit 5 - Automatic Re-transmission Disable Bit"]
    #[inline(always)]
    pub fn dar(&mut self) -> DAR_W {
        DAR_W { w: self }
    }
    #[doc = "Bit 6 - Configuration Change Enable Bit"]
    #[inline(always)]
    pub fn cce(&mut self) -> CCE_W {
        CCE_W { w: self }
    }
    #[doc = "Bit 7 - Test Mode Enable Bit"]
    #[inline(always)]
    pub fn test(&mut self) -> TEST_W {
        TEST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_con](index.html) module"]
pub struct CAN_CON_SPEC;
impl crate::RegisterSpec for CAN_CON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [can_con::R](R) reader structure"]
impl crate::Readable for CAN_CON_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [can_con::W](W) writer structure"]
impl crate::Writable for CAN_CON_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CAN_CON to value 0x01"]
impl crate::Resettable for CAN_CON_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
