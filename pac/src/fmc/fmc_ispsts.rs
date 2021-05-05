#[doc = "Register `FMC_ISPSTS` reader"]
pub struct R(crate::R<FMC_ISPSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMC_ISPSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<FMC_ISPSTS_SPEC>> for R {
    fn from(reader: crate::R<FMC_ISPSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FMC_ISPSTS` writer"]
pub struct W(crate::W<FMC_ISPSTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FMC_ISPSTS_SPEC>;
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
impl core::convert::From<crate::W<FMC_ISPSTS_SPEC>> for W {
    fn from(writer: crate::W<FMC_ISPSTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "ISP Busy Flag (Read Only)\\nWrite 1 to start ISP operation and this bit will be cleared to 0 by hardware automatically when ISP operation is finished.\\nThis bit is the mirror of ISPGO(FMC_ISPTRG\\[0\\]).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISPBUSY_A {
    #[doc = "0: ISP operation is finished"]
    _0 = 0,
    #[doc = "1: ISP is progressed"]
    _1 = 1,
}
impl From<ISPBUSY_A> for bool {
    #[inline(always)]
    fn from(variant: ISPBUSY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISPBUSY` reader - ISP Busy Flag (Read Only)\\nWrite 1 to start ISP operation and this bit will be cleared to 0 by hardware automatically when ISP operation is finished.\\nThis bit is the mirror of ISPGO(FMC_ISPTRG\\[0\\])."]
pub struct ISPBUSY_R(crate::FieldReader<bool, ISPBUSY_A>);
impl ISPBUSY_R {
    pub(crate) fn new(bits: bool) -> Self {
        ISPBUSY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISPBUSY_A {
        match self.bits {
            false => ISPBUSY_A::_0,
            true => ISPBUSY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ISPBUSY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ISPBUSY_A::_1
    }
}
impl core::ops::Deref for ISPBUSY_R {
    type Target = crate::FieldReader<bool, ISPBUSY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Boot Selection of CONFIG (Read Only)\\nThis bit is initiated with the CBS (CONFIG0\\[7:6\\]) after any reset is happened except CPU reset (CPU is 1) or system reset (SYS) is happened.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CBS_A {
    #[doc = "0: LDROM with IAP mode"]
    _0 = 0,
    #[doc = "1: LDROM without IAP mode"]
    _1 = 1,
    #[doc = "2: APROM with IAP mode"]
    _2 = 2,
    #[doc = "3: APROM without IAP mode"]
    _3 = 3,
}
impl From<CBS_A> for u8 {
    #[inline(always)]
    fn from(variant: CBS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CBS` reader - Boot Selection of CONFIG (Read Only)\\nThis bit is initiated with the CBS (CONFIG0\\[7:6\\]) after any reset is happened except CPU reset (CPU is 1) or system reset (SYS) is happened."]
pub struct CBS_R(crate::FieldReader<u8, CBS_A>);
impl CBS_R {
    pub(crate) fn new(bits: u8) -> Self {
        CBS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CBS_A {
        match self.bits {
            0 => CBS_A::_0,
            1 => CBS_A::_1,
            2 => CBS_A::_2,
            3 => CBS_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CBS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CBS_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        **self == CBS_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        **self == CBS_A::_3
    }
}
impl core::ops::Deref for CBS_R {
    type Target = crate::FieldReader<u8, CBS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Flash Program with Fast Verification Flag (Read Only)\\nThis bit is set if data is mismatched at ISP programming verification. This bit is clear by performing ISP flash erase or ISP read CID operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PGFF_A {
    #[doc = "0: Flash Program is success"]
    _0 = 0,
    #[doc = "1: Flash Program is fail. Program data is different with data in the flash memory"]
    _1 = 1,
}
impl From<PGFF_A> for bool {
    #[inline(always)]
    fn from(variant: PGFF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PGFF` reader - Flash Program with Fast Verification Flag (Read Only)\\nThis bit is set if data is mismatched at ISP programming verification. This bit is clear by performing ISP flash erase or ISP read CID operation"]
pub struct PGFF_R(crate::FieldReader<bool, PGFF_A>);
impl PGFF_R {
    pub(crate) fn new(bits: bool) -> Self {
        PGFF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PGFF_A {
        match self.bits {
            false => PGFF_A::_0,
            true => PGFF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PGFF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PGFF_A::_1
    }
}
impl core::ops::Deref for PGFF_R {
    type Target = crate::FieldReader<bool, PGFF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISPFF` reader - ISP Fail Flag (Write Protect)\\nThis bit is the mirror of ISPFF (FMC_ISPCTL\\[6\\]), it needs to be cleared by writing 1 to FMC_ISPCTL\\[6\\]
or FMC_ISPSTS\\[6\\]. This bit is set by hardware when a triggered ISP meets any of the following conditions:\\n(1) APROM writes to itself if APUEN is set to 0.\\n(2) LDROM writes to itself if LDUEN is set to 0.\\n(3) CONFIG is erased/programmed if CFGUEN is set to 0.\\n(4) Page Erase command at LOCK mode with ICE connection\\n(5) Erase or Program command at brown-out detected\\n(6) Destination address is illegal, such as over an available range.\\n(7) Invalid ISP commands\\n\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct ISPFF_R(crate::FieldReader<bool, bool>);
impl ISPFF_R {
    pub(crate) fn new(bits: bool) -> Self {
        ISPFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISPFF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISPFF` writer - ISP Fail Flag (Write Protect)\\nThis bit is the mirror of ISPFF (FMC_ISPCTL\\[6\\]), it needs to be cleared by writing 1 to FMC_ISPCTL\\[6\\]
or FMC_ISPSTS\\[6\\]. This bit is set by hardware when a triggered ISP meets any of the following conditions:\\n(1) APROM writes to itself if APUEN is set to 0.\\n(2) LDROM writes to itself if LDUEN is set to 0.\\n(3) CONFIG is erased/programmed if CFGUEN is set to 0.\\n(4) Page Erase command at LOCK mode with ICE connection\\n(5) Erase or Program command at brown-out detected\\n(6) Destination address is illegal, such as over an available range.\\n(7) Invalid ISP commands\\n\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
pub struct ISPFF_W<'a> {
    w: &'a mut W,
}
impl<'a> ISPFF_W<'a> {
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
#[doc = "Field `VECMAP` reader - Vector Page Mapping Address (Read Only)\\nAll access to 0x0000_0000~0x0000_01FF is remapped to the flash memory address {VECMAP\\[14:0\\], 9'h000} ~ {VECMAP\\[14:0\\], 9'h1FF}"]
pub struct VECMAP_R(crate::FieldReader<u16, u16>);
impl VECMAP_R {
    pub(crate) fn new(bits: u16) -> Self {
        VECMAP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VECMAP_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - ISP Busy Flag (Read Only)\\nWrite 1 to start ISP operation and this bit will be cleared to 0 by hardware automatically when ISP operation is finished.\\nThis bit is the mirror of ISPGO(FMC_ISPTRG\\[0\\])."]
    #[inline(always)]
    pub fn ispbusy(&self) -> ISPBUSY_R {
        ISPBUSY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - Boot Selection of CONFIG (Read Only)\\nThis bit is initiated with the CBS (CONFIG0\\[7:6\\]) after any reset is happened except CPU reset (CPU is 1) or system reset (SYS) is happened."]
    #[inline(always)]
    pub fn cbs(&self) -> CBS_R {
        CBS_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 5 - Flash Program with Fast Verification Flag (Read Only)\\nThis bit is set if data is mismatched at ISP programming verification. This bit is clear by performing ISP flash erase or ISP read CID operation"]
    #[inline(always)]
    pub fn pgff(&self) -> PGFF_R {
        PGFF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - ISP Fail Flag (Write Protect)\\nThis bit is the mirror of ISPFF (FMC_ISPCTL\\[6\\]), it needs to be cleared by writing 1 to FMC_ISPCTL\\[6\\]
or FMC_ISPSTS\\[6\\]. This bit is set by hardware when a triggered ISP meets any of the following conditions:\\n(1) APROM writes to itself if APUEN is set to 0.\\n(2) LDROM writes to itself if LDUEN is set to 0.\\n(3) CONFIG is erased/programmed if CFGUEN is set to 0.\\n(4) Page Erase command at LOCK mode with ICE connection\\n(5) Erase or Program command at brown-out detected\\n(6) Destination address is illegal, such as over an available range.\\n(7) Invalid ISP commands\\n\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn ispff(&self) -> ISPFF_R {
        ISPFF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 9:23 - Vector Page Mapping Address (Read Only)\\nAll access to 0x0000_0000~0x0000_01FF is remapped to the flash memory address {VECMAP\\[14:0\\], 9'h000} ~ {VECMAP\\[14:0\\], 9'h1FF}"]
    #[inline(always)]
    pub fn vecmap(&self) -> VECMAP_R {
        VECMAP_R::new(((self.bits >> 9) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bit 6 - ISP Fail Flag (Write Protect)\\nThis bit is the mirror of ISPFF (FMC_ISPCTL\\[6\\]), it needs to be cleared by writing 1 to FMC_ISPCTL\\[6\\]
or FMC_ISPSTS\\[6\\]. This bit is set by hardware when a triggered ISP meets any of the following conditions:\\n(1) APROM writes to itself if APUEN is set to 0.\\n(2) LDROM writes to itself if LDUEN is set to 0.\\n(3) CONFIG is erased/programmed if CFGUEN is set to 0.\\n(4) Page Erase command at LOCK mode with ICE connection\\n(5) Erase or Program command at brown-out detected\\n(6) Destination address is illegal, such as over an available range.\\n(7) Invalid ISP commands\\n\\nNote: This bit is write protected. Refer to the SYS_REGLCTL register."]
    #[inline(always)]
    pub fn ispff(&mut self) -> ISPFF_W {
        ISPFF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ISP Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_ispsts](index.html) module"]
pub struct FMC_ISPSTS_SPEC;
impl crate::RegisterSpec for FMC_ISPSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmc_ispsts::R](R) reader structure"]
impl crate::Readable for FMC_ISPSTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fmc_ispsts::W](W) writer structure"]
impl crate::Writable for FMC_ISPSTS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FMC_ISPSTS to value 0"]
impl crate::Resettable for FMC_ISPSTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
