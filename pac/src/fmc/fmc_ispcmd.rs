#[doc = "Register `FMC_ISPCMD` reader"]
pub struct R(crate::R<FMC_ISPCMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMC_ISPCMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<FMC_ISPCMD_SPEC>> for R {
    fn from(reader: crate::R<FMC_ISPCMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FMC_ISPCMD` writer"]
pub struct W(crate::W<FMC_ISPCMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FMC_ISPCMD_SPEC>;
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
impl core::convert::From<crate::W<FMC_ISPCMD_SPEC>> for W {
    fn from(writer: crate::W<FMC_ISPCMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "ISP CMD\\nISP command table is shown below:\\nThe other commands are invalid.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CMD_A {
    #[doc = "0: FLASH 32-bit Read"]
    _0 = 0,
    #[doc = "4: Read Unique ID"]
    _4 = 4,
    #[doc = "11: Read Company ID"]
    _11 = 11,
    #[doc = "13: Read Checksum"]
    _13 = 13,
    #[doc = "33: FLASH 32-bit Program"]
    _33 = 33,
    #[doc = "34: FLASH Page Erase"]
    _34 = 34,
    #[doc = "39: FLASH Multi-Word Program"]
    _39 = 39,
    #[doc = "45: Run Checksum Calculation"]
    _45 = 45,
    #[doc = "46: Vector Remap"]
    _46 = 46,
    #[doc = "64: FLASH 64-bit Read"]
    _64 = 64,
    #[doc = "97: FLASH 64-bit Program"]
    _97 = 97,
}
impl From<CMD_A> for u8 {
    #[inline(always)]
    fn from(variant: CMD_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CMD` reader - ISP CMD\\nISP command table is shown below:\\nThe other commands are invalid."]
pub struct CMD_R(crate::FieldReader<u8, CMD_A>);
impl CMD_R {
    pub(crate) fn new(bits: u8) -> Self {
        CMD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CMD_A> {
        match self.bits {
            0 => Some(CMD_A::_0),
            4 => Some(CMD_A::_4),
            11 => Some(CMD_A::_11),
            13 => Some(CMD_A::_13),
            33 => Some(CMD_A::_33),
            34 => Some(CMD_A::_34),
            39 => Some(CMD_A::_39),
            45 => Some(CMD_A::_45),
            46 => Some(CMD_A::_46),
            64 => Some(CMD_A::_64),
            97 => Some(CMD_A::_97),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CMD_A::_0
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        **self == CMD_A::_4
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        **self == CMD_A::_11
    }
    #[doc = "Checks if the value of the field is `_13`"]
    #[inline(always)]
    pub fn is_13(&self) -> bool {
        **self == CMD_A::_13
    }
    #[doc = "Checks if the value of the field is `_33`"]
    #[inline(always)]
    pub fn is_33(&self) -> bool {
        **self == CMD_A::_33
    }
    #[doc = "Checks if the value of the field is `_34`"]
    #[inline(always)]
    pub fn is_34(&self) -> bool {
        **self == CMD_A::_34
    }
    #[doc = "Checks if the value of the field is `_39`"]
    #[inline(always)]
    pub fn is_39(&self) -> bool {
        **self == CMD_A::_39
    }
    #[doc = "Checks if the value of the field is `_45`"]
    #[inline(always)]
    pub fn is_45(&self) -> bool {
        **self == CMD_A::_45
    }
    #[doc = "Checks if the value of the field is `_46`"]
    #[inline(always)]
    pub fn is_46(&self) -> bool {
        **self == CMD_A::_46
    }
    #[doc = "Checks if the value of the field is `_64`"]
    #[inline(always)]
    pub fn is_64(&self) -> bool {
        **self == CMD_A::_64
    }
    #[doc = "Checks if the value of the field is `_97`"]
    #[inline(always)]
    pub fn is_97(&self) -> bool {
        **self == CMD_A::_97
    }
}
impl core::ops::Deref for CMD_R {
    type Target = crate::FieldReader<u8, CMD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMD` writer - ISP CMD\\nISP command table is shown below:\\nThe other commands are invalid."]
pub struct CMD_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMD_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "FLASH 32-bit Read"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMD_A::_0)
    }
    #[doc = "Read Unique ID"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(CMD_A::_4)
    }
    #[doc = "Read Company ID"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(CMD_A::_11)
    }
    #[doc = "Read Checksum"]
    #[inline(always)]
    pub fn _13(self) -> &'a mut W {
        self.variant(CMD_A::_13)
    }
    #[doc = "FLASH 32-bit Program"]
    #[inline(always)]
    pub fn _33(self) -> &'a mut W {
        self.variant(CMD_A::_33)
    }
    #[doc = "FLASH Page Erase"]
    #[inline(always)]
    pub fn _34(self) -> &'a mut W {
        self.variant(CMD_A::_34)
    }
    #[doc = "FLASH Multi-Word Program"]
    #[inline(always)]
    pub fn _39(self) -> &'a mut W {
        self.variant(CMD_A::_39)
    }
    #[doc = "Run Checksum Calculation"]
    #[inline(always)]
    pub fn _45(self) -> &'a mut W {
        self.variant(CMD_A::_45)
    }
    #[doc = "Vector Remap"]
    #[inline(always)]
    pub fn _46(self) -> &'a mut W {
        self.variant(CMD_A::_46)
    }
    #[doc = "FLASH 64-bit Read"]
    #[inline(always)]
    pub fn _64(self) -> &'a mut W {
        self.variant(CMD_A::_64)
    }
    #[doc = "FLASH 64-bit Program"]
    #[inline(always)]
    pub fn _97(self) -> &'a mut W {
        self.variant(CMD_A::_97)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - ISP CMD\\nISP command table is shown below:\\nThe other commands are invalid."]
    #[inline(always)]
    pub fn cmd(&self) -> CMD_R {
        CMD_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - ISP CMD\\nISP command table is shown below:\\nThe other commands are invalid."]
    #[inline(always)]
    pub fn cmd(&mut self) -> CMD_W {
        CMD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ISP CMD Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_ispcmd](index.html) module"]
pub struct FMC_ISPCMD_SPEC;
impl crate::RegisterSpec for FMC_ISPCMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmc_ispcmd::R](R) reader structure"]
impl crate::Readable for FMC_ISPCMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fmc_ispcmd::W](W) writer structure"]
impl crate::Writable for FMC_ISPCMD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FMC_ISPCMD to value 0"]
impl crate::Resettable for FMC_ISPCMD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
