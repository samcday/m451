#[doc = "Register `HCCOMMANDSTATUS` reader"]
pub struct R(crate::R<HCCOMMANDSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCCOMMANDSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<HCCOMMANDSTATUS_SPEC>> for R {
    fn from(reader: crate::R<HCCOMMANDSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HCCOMMANDSTATUS` writer"]
pub struct W(crate::W<HCCOMMANDSTATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCCOMMANDSTATUS_SPEC>;
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
impl core::convert::From<crate::W<HCCOMMANDSTATUS_SPEC>> for W {
    fn from(writer: crate::W<HCCOMMANDSTATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Host Controller Reset\\nThis bit is set to initiate the software reset of Host Controller. This bit is cleared by the Host Controller, upon completed of the reset operation.\\nThis bit, when set, didn't reset the Root Hub and no subsequent reset signaling be asserted to its downstream ports.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HCR_A {
    #[doc = "0: Host Controller is not in software reset state"]
    _0 = 0,
    #[doc = "1: Host Controller is in software reset state"]
    _1 = 1,
}
impl From<HCR_A> for bool {
    #[inline(always)]
    fn from(variant: HCR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HCR` reader - Host Controller Reset\\nThis bit is set to initiate the software reset of Host Controller. This bit is cleared by the Host Controller, upon completed of the reset operation.\\nThis bit, when set, didn't reset the Root Hub and no subsequent reset signaling be asserted to its downstream ports."]
pub struct HCR_R(crate::FieldReader<bool, HCR_A>);
impl HCR_R {
    pub(crate) fn new(bits: bool) -> Self {
        HCR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HCR_A {
        match self.bits {
            false => HCR_A::_0,
            true => HCR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == HCR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == HCR_A::_1
    }
}
impl core::ops::Deref for HCR_R {
    type Target = crate::FieldReader<bool, HCR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HCR` writer - Host Controller Reset\\nThis bit is set to initiate the software reset of Host Controller. This bit is cleared by the Host Controller, upon completed of the reset operation.\\nThis bit, when set, didn't reset the Root Hub and no subsequent reset signaling be asserted to its downstream ports."]
pub struct HCR_W<'a> {
    w: &'a mut W,
}
impl<'a> HCR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HCR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Host Controller is not in software reset state"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HCR_A::_0)
    }
    #[doc = "Host Controller is in software reset state"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HCR_A::_1)
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
#[doc = "Control List Filled\\nSet high to indicate there is an active TD on the Control List. It may be set by either software or the Host Controller and cleared by the Host Controller each time it begins processing the head of the Control List.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLF_A {
    #[doc = "0: No active TD found or Host Controller begins to process the head of the Control list"]
    _0 = 0,
    #[doc = "1: An active TD added or found on the Control list"]
    _1 = 1,
}
impl From<CLF_A> for bool {
    #[inline(always)]
    fn from(variant: CLF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLF` reader - Control List Filled\\nSet high to indicate there is an active TD on the Control List. It may be set by either software or the Host Controller and cleared by the Host Controller each time it begins processing the head of the Control List."]
pub struct CLF_R(crate::FieldReader<bool, CLF_A>);
impl CLF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLF_A {
        match self.bits {
            false => CLF_A::_0,
            true => CLF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CLF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CLF_A::_1
    }
}
impl core::ops::Deref for CLF_R {
    type Target = crate::FieldReader<bool, CLF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLF` writer - Control List Filled\\nSet high to indicate there is an active TD on the Control List. It may be set by either software or the Host Controller and cleared by the Host Controller each time it begins processing the head of the Control List."]
pub struct CLF_W<'a> {
    w: &'a mut W,
}
impl<'a> CLF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No active TD found or Host Controller begins to process the head of the Control list"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CLF_A::_0)
    }
    #[doc = "An active TD added or found on the Control list"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CLF_A::_1)
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
#[doc = "Bulk List Filled\\nSet high to indicate there is an active TD on the Bulk list. This bit may be set by either software or the Host Controller and cleared by the Host Controller each time it begins processing the head of the Bulk list.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BLF_A {
    #[doc = "0: No active TD found or Host Controller begins to process the head of the Bulk list"]
    _0 = 0,
    #[doc = "1: An active TD added or found on the Bulk list"]
    _1 = 1,
}
impl From<BLF_A> for bool {
    #[inline(always)]
    fn from(variant: BLF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BLF` reader - Bulk List Filled\\nSet high to indicate there is an active TD on the Bulk list. This bit may be set by either software or the Host Controller and cleared by the Host Controller each time it begins processing the head of the Bulk list."]
pub struct BLF_R(crate::FieldReader<bool, BLF_A>);
impl BLF_R {
    pub(crate) fn new(bits: bool) -> Self {
        BLF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLF_A {
        match self.bits {
            false => BLF_A::_0,
            true => BLF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BLF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BLF_A::_1
    }
}
impl core::ops::Deref for BLF_R {
    type Target = crate::FieldReader<bool, BLF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLF` writer - Bulk List Filled\\nSet high to indicate there is an active TD on the Bulk list. This bit may be set by either software or the Host Controller and cleared by the Host Controller each time it begins processing the head of the Bulk list."]
pub struct BLF_W<'a> {
    w: &'a mut W,
}
impl<'a> BLF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BLF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No active TD found or Host Controller begins to process the head of the Bulk list"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BLF_A::_0)
    }
    #[doc = "An active TD added or found on the Bulk list"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BLF_A::_1)
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
#[doc = "Field `SOC` reader - Schedule Overrun Count\\nThese bits are incremented on each scheduling overrun error. It is initialized to 00b and wraps around at 11b. This will be incremented when a scheduling overrun is detected even if SO (HcInterruptStatus\\[0\\]) has already been set."]
pub struct SOC_R(crate::FieldReader<u8, u8>);
impl SOC_R {
    pub(crate) fn new(bits: u8) -> Self {
        SOC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOC` writer - Schedule Overrun Count\\nThese bits are incremented on each scheduling overrun error. It is initialized to 00b and wraps around at 11b. This will be incremented when a scheduling overrun is detected even if SO (HcInterruptStatus\\[0\\]) has already been set."]
pub struct SOC_W<'a> {
    w: &'a mut W,
}
impl<'a> SOC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Host Controller Reset\\nThis bit is set to initiate the software reset of Host Controller. This bit is cleared by the Host Controller, upon completed of the reset operation.\\nThis bit, when set, didn't reset the Root Hub and no subsequent reset signaling be asserted to its downstream ports."]
    #[inline(always)]
    pub fn hcr(&self) -> HCR_R {
        HCR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Control List Filled\\nSet high to indicate there is an active TD on the Control List. It may be set by either software or the Host Controller and cleared by the Host Controller each time it begins processing the head of the Control List."]
    #[inline(always)]
    pub fn clf(&self) -> CLF_R {
        CLF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Bulk List Filled\\nSet high to indicate there is an active TD on the Bulk list. This bit may be set by either software or the Host Controller and cleared by the Host Controller each time it begins processing the head of the Bulk list."]
    #[inline(always)]
    pub fn blf(&self) -> BLF_R {
        BLF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - Schedule Overrun Count\\nThese bits are incremented on each scheduling overrun error. It is initialized to 00b and wraps around at 11b. This will be incremented when a scheduling overrun is detected even if SO (HcInterruptStatus\\[0\\]) has already been set."]
    #[inline(always)]
    pub fn soc(&self) -> SOC_R {
        SOC_R::new(((self.bits >> 16) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Host Controller Reset\\nThis bit is set to initiate the software reset of Host Controller. This bit is cleared by the Host Controller, upon completed of the reset operation.\\nThis bit, when set, didn't reset the Root Hub and no subsequent reset signaling be asserted to its downstream ports."]
    #[inline(always)]
    pub fn hcr(&mut self) -> HCR_W {
        HCR_W { w: self }
    }
    #[doc = "Bit 1 - Control List Filled\\nSet high to indicate there is an active TD on the Control List. It may be set by either software or the Host Controller and cleared by the Host Controller each time it begins processing the head of the Control List."]
    #[inline(always)]
    pub fn clf(&mut self) -> CLF_W {
        CLF_W { w: self }
    }
    #[doc = "Bit 2 - Bulk List Filled\\nSet high to indicate there is an active TD on the Bulk list. This bit may be set by either software or the Host Controller and cleared by the Host Controller each time it begins processing the head of the Bulk list."]
    #[inline(always)]
    pub fn blf(&mut self) -> BLF_W {
        BLF_W { w: self }
    }
    #[doc = "Bits 16:17 - Schedule Overrun Count\\nThese bits are incremented on each scheduling overrun error. It is initialized to 00b and wraps around at 11b. This will be incremented when a scheduling overrun is detected even if SO (HcInterruptStatus\\[0\\]) has already been set."]
    #[inline(always)]
    pub fn soc(&mut self) -> SOC_W {
        SOC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Host Controller CMD Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hccommandstatus](index.html) module"]
pub struct HCCOMMANDSTATUS_SPEC;
impl crate::RegisterSpec for HCCOMMANDSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hccommandstatus::R](R) reader structure"]
impl crate::Readable for HCCOMMANDSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hccommandstatus::W](W) writer structure"]
impl crate::Writable for HCCOMMANDSTATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HCCOMMANDSTATUS to value 0"]
impl crate::Resettable for HCCOMMANDSTATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
