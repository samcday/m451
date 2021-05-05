#[doc = "Register `FMC_MPSTS` reader"]
pub struct R(crate::R<FMC_MPSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMC_MPSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<FMC_MPSTS_SPEC>> for R {
    fn from(reader: crate::R<FMC_MPSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "ISP Multi-word Program Busy Flag (Read Only)\\nWrite 1 to start ISP Multi-Word program operation and this bit will be cleared to 0 by hardware automatically when ISP Multi-Word program operation is finished.\\nThis bit is the mirror of ISPGO(FMC_ISPTRG\\[0\\]).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MPBUSY_A {
    #[doc = "0: ISP Multi-Word program operation is finished"]
    _0 = 0,
    #[doc = "1: ISP Multi-Word program operation is progressed"]
    _1 = 1,
}
impl From<MPBUSY_A> for bool {
    #[inline(always)]
    fn from(variant: MPBUSY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MPBUSY` reader - ISP Multi-word Program Busy Flag (Read Only)\\nWrite 1 to start ISP Multi-Word program operation and this bit will be cleared to 0 by hardware automatically when ISP Multi-Word program operation is finished.\\nThis bit is the mirror of ISPGO(FMC_ISPTRG\\[0\\])."]
pub struct MPBUSY_R(crate::FieldReader<bool, MPBUSY_A>);
impl MPBUSY_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPBUSY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPBUSY_A {
        match self.bits {
            false => MPBUSY_A::_0,
            true => MPBUSY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == MPBUSY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == MPBUSY_A::_1
    }
}
impl core::ops::Deref for MPBUSY_R {
    type Target = crate::FieldReader<bool, MPBUSY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "ISP Multi-program Status (Read Only)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PPGO_A {
    #[doc = "0: ISP multi-word program operation is not active"]
    _0 = 0,
    #[doc = "1: ISP multi-word program operation is in progress"]
    _1 = 1,
}
impl From<PPGO_A> for bool {
    #[inline(always)]
    fn from(variant: PPGO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PPGO` reader - ISP Multi-program Status (Read Only)"]
pub struct PPGO_R(crate::FieldReader<bool, PPGO_A>);
impl PPGO_R {
    pub(crate) fn new(bits: bool) -> Self {
        PPGO_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PPGO_A {
        match self.bits {
            false => PPGO_A::_0,
            true => PPGO_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PPGO_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PPGO_A::_1
    }
}
impl core::ops::Deref for PPGO_R {
    type Target = crate::FieldReader<bool, PPGO_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISPFF` reader - ISP Fail Flag (Read Only)\\nThis bit is the mirror of ISPFF (FMC_ISPCTL\\[6\\]), it needs to be cleared by writing 1 to FMC_ISPCTL\\[6\\]
or FMC_ISPSTS\\[6\\]. This bit is set by hardware when a triggered ISP meets any of the following conditions:\\n(1) APROM writes to itself if APUEN is set to 0.\\n(2) LDROM writes to itself if LDUEN is set to 0.\\n(3) CONFIG is erased/programmed if CFGUEN is set to 0.\\n(4) Page Erase command at LOCK mode with ICE connection\\n(5) Erase or Program command at brown-out detected\\n(6) Destination address is illegal, such as over an available range.\\n(7) Invalid ISP commands"]
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
#[doc = "ISP DATA 0 Flag (Read Only)\\nThis bit is set when FMC_MPDAT0 is written and auto-clear to 0 when the FMC_MPDAT0 data is programmed to flash complete.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum D0_A {
    #[doc = "0: FMC_MPDAT0 register is empty, or program to flash complete"]
    _0 = 0,
    #[doc = "1: FMC_MPDAT0 register has been written, and not program to flash complete"]
    _1 = 1,
}
impl From<D0_A> for bool {
    #[inline(always)]
    fn from(variant: D0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `D0` reader - ISP DATA 0 Flag (Read Only)\\nThis bit is set when FMC_MPDAT0 is written and auto-clear to 0 when the FMC_MPDAT0 data is programmed to flash complete."]
pub struct D0_R(crate::FieldReader<bool, D0_A>);
impl D0_R {
    pub(crate) fn new(bits: bool) -> Self {
        D0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> D0_A {
        match self.bits {
            false => D0_A::_0,
            true => D0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == D0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == D0_A::_1
    }
}
impl core::ops::Deref for D0_R {
    type Target = crate::FieldReader<bool, D0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "ISP DATA 1 Flag (Read Only)\\nThis bit is set when FMC_MPDAT1 is written and auto-clear to 0 when the FMC_MPDAT1 data is programmed to flash complete.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum D1_A {
    #[doc = "0: FMC_MPDAT1 register is empty, or program to flash complete"]
    _0 = 0,
    #[doc = "1: FMC_MPDAT1 register has been written, and not program to flash complete"]
    _1 = 1,
}
impl From<D1_A> for bool {
    #[inline(always)]
    fn from(variant: D1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `D1` reader - ISP DATA 1 Flag (Read Only)\\nThis bit is set when FMC_MPDAT1 is written and auto-clear to 0 when the FMC_MPDAT1 data is programmed to flash complete."]
pub struct D1_R(crate::FieldReader<bool, D1_A>);
impl D1_R {
    pub(crate) fn new(bits: bool) -> Self {
        D1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> D1_A {
        match self.bits {
            false => D1_A::_0,
            true => D1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == D1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == D1_A::_1
    }
}
impl core::ops::Deref for D1_R {
    type Target = crate::FieldReader<bool, D1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "ISP DATA 2 Flag (Read Only)\\nThis bit is set when FMC_MPDAT2 is written and auto-clear to 0 when the FMC_MPDAT2 data is programmed to flash complete.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum D2_A {
    #[doc = "0: FMC_MPDAT2 register is empty, or program to flash complete"]
    _0 = 0,
    #[doc = "1: FMC_MPDAT2 register has been written, and not program to flash complete"]
    _1 = 1,
}
impl From<D2_A> for bool {
    #[inline(always)]
    fn from(variant: D2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `D2` reader - ISP DATA 2 Flag (Read Only)\\nThis bit is set when FMC_MPDAT2 is written and auto-clear to 0 when the FMC_MPDAT2 data is programmed to flash complete."]
pub struct D2_R(crate::FieldReader<bool, D2_A>);
impl D2_R {
    pub(crate) fn new(bits: bool) -> Self {
        D2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> D2_A {
        match self.bits {
            false => D2_A::_0,
            true => D2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == D2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == D2_A::_1
    }
}
impl core::ops::Deref for D2_R {
    type Target = crate::FieldReader<bool, D2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "ISP DATA 3 Flag (Read Only)\\nThis bit is set when FMC_MPDAT3 is written and auto-clear to 0 when the FMC_MPDAT3 data is programmed to flash complete.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum D3_A {
    #[doc = "0: FMC_MPDAT3 register is empty, or program to flash complete"]
    _0 = 0,
    #[doc = "1: FMC_MPDAT3 register has been written, and not program to flash complete"]
    _1 = 1,
}
impl From<D3_A> for bool {
    #[inline(always)]
    fn from(variant: D3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `D3` reader - ISP DATA 3 Flag (Read Only)\\nThis bit is set when FMC_MPDAT3 is written and auto-clear to 0 when the FMC_MPDAT3 data is programmed to flash complete."]
pub struct D3_R(crate::FieldReader<bool, D3_A>);
impl D3_R {
    pub(crate) fn new(bits: bool) -> Self {
        D3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> D3_A {
        match self.bits {
            false => D3_A::_0,
            true => D3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == D3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == D3_A::_1
    }
}
impl core::ops::Deref for D3_R {
    type Target = crate::FieldReader<bool, D3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - ISP Multi-word Program Busy Flag (Read Only)\\nWrite 1 to start ISP Multi-Word program operation and this bit will be cleared to 0 by hardware automatically when ISP Multi-Word program operation is finished.\\nThis bit is the mirror of ISPGO(FMC_ISPTRG\\[0\\])."]
    #[inline(always)]
    pub fn mpbusy(&self) -> MPBUSY_R {
        MPBUSY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ISP Multi-program Status (Read Only)"]
    #[inline(always)]
    pub fn ppgo(&self) -> PPGO_R {
        PPGO_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - ISP Fail Flag (Read Only)\\nThis bit is the mirror of ISPFF (FMC_ISPCTL\\[6\\]), it needs to be cleared by writing 1 to FMC_ISPCTL\\[6\\]
or FMC_ISPSTS\\[6\\]. This bit is set by hardware when a triggered ISP meets any of the following conditions:\\n(1) APROM writes to itself if APUEN is set to 0.\\n(2) LDROM writes to itself if LDUEN is set to 0.\\n(3) CONFIG is erased/programmed if CFGUEN is set to 0.\\n(4) Page Erase command at LOCK mode with ICE connection\\n(5) Erase or Program command at brown-out detected\\n(6) Destination address is illegal, such as over an available range.\\n(7) Invalid ISP commands"]
    #[inline(always)]
    pub fn ispff(&self) -> ISPFF_R {
        ISPFF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - ISP DATA 0 Flag (Read Only)\\nThis bit is set when FMC_MPDAT0 is written and auto-clear to 0 when the FMC_MPDAT0 data is programmed to flash complete."]
    #[inline(always)]
    pub fn d0(&self) -> D0_R {
        D0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - ISP DATA 1 Flag (Read Only)\\nThis bit is set when FMC_MPDAT1 is written and auto-clear to 0 when the FMC_MPDAT1 data is programmed to flash complete."]
    #[inline(always)]
    pub fn d1(&self) -> D1_R {
        D1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - ISP DATA 2 Flag (Read Only)\\nThis bit is set when FMC_MPDAT2 is written and auto-clear to 0 when the FMC_MPDAT2 data is programmed to flash complete."]
    #[inline(always)]
    pub fn d2(&self) -> D2_R {
        D2_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - ISP DATA 3 Flag (Read Only)\\nThis bit is set when FMC_MPDAT3 is written and auto-clear to 0 when the FMC_MPDAT3 data is programmed to flash complete."]
    #[inline(always)]
    pub fn d3(&self) -> D3_R {
        D3_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
#[doc = "ISP Multi-program Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_mpsts](index.html) module"]
pub struct FMC_MPSTS_SPEC;
impl crate::RegisterSpec for FMC_MPSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmc_mpsts::R](R) reader structure"]
impl crate::Readable for FMC_MPSTS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FMC_MPSTS to value 0"]
impl crate::Resettable for FMC_MPSTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
