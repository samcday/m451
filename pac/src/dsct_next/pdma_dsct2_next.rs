#[doc = "Register `PDMA_DSCT2_NEXT` reader"]
pub struct R(crate::R<PDMA_DSCT2_NEXT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDMA_DSCT2_NEXT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PDMA_DSCT2_NEXT_SPEC>> for R {
    fn from(reader: crate::R<PDMA_DSCT2_NEXT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDMA_DSCT2_NEXT` writer"]
pub struct W(crate::W<PDMA_DSCT2_NEXT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDMA_DSCT2_NEXT_SPEC>;
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
impl core::convert::From<crate::W<PDMA_DSCT2_NEXT_SPEC>> for W {
    fn from(writer: crate::W<PDMA_DSCT2_NEXT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NEXT` reader - PDMA Next Descriptor Table Offset Address Register\\nThis field indicates the offset of next descriptor table address in system memory. The system memory based address is 0x2000_0000 (PDMA_SCATBA), if the next descriptor table is 0x2000_0100, then this field must fill in 0x0100.\\nNote1: The next descriptor table address must be word boundary.\\nNote2: Before filled transfer task in the descriptor table, user must check if the descriptor table is complete."]
pub struct NEXT_R(crate::FieldReader<u16, u16>);
impl NEXT_R {
    pub(crate) fn new(bits: u16) -> Self {
        NEXT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NEXT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NEXT` writer - PDMA Next Descriptor Table Offset Address Register\\nThis field indicates the offset of next descriptor table address in system memory. The system memory based address is 0x2000_0000 (PDMA_SCATBA), if the next descriptor table is 0x2000_0100, then this field must fill in 0x0100.\\nNote1: The next descriptor table address must be word boundary.\\nNote2: Before filled transfer task in the descriptor table, user must check if the descriptor table is complete."]
pub struct NEXT_W<'a> {
    w: &'a mut W,
}
impl<'a> NEXT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff << 2)) | ((value as u32 & 0x3fff) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:15 - PDMA Next Descriptor Table Offset Address Register\\nThis field indicates the offset of next descriptor table address in system memory. The system memory based address is 0x2000_0000 (PDMA_SCATBA), if the next descriptor table is 0x2000_0100, then this field must fill in 0x0100.\\nNote1: The next descriptor table address must be word boundary.\\nNote2: Before filled transfer task in the descriptor table, user must check if the descriptor table is complete."]
    #[inline(always)]
    pub fn next(&self) -> NEXT_R {
        NEXT_R::new(((self.bits >> 2) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 2:15 - PDMA Next Descriptor Table Offset Address Register\\nThis field indicates the offset of next descriptor table address in system memory. The system memory based address is 0x2000_0000 (PDMA_SCATBA), if the next descriptor table is 0x2000_0100, then this field must fill in 0x0100.\\nNote1: The next descriptor table address must be word boundary.\\nNote2: Before filled transfer task in the descriptor table, user must check if the descriptor table is complete."]
    #[inline(always)]
    pub fn next(&mut self) -> NEXT_W {
        NEXT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "First Scatter-gather Descriptor Table Offset Address of PDMA Channel n\\n(M45xD/M45xC Only Support Channel 0~7)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_dsct2_next](index.html) module"]
pub struct PDMA_DSCT2_NEXT_SPEC;
impl crate::RegisterSpec for PDMA_DSCT2_NEXT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdma_dsct2_next::R](R) reader structure"]
impl crate::Readable for PDMA_DSCT2_NEXT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdma_dsct2_next::W](W) writer structure"]
impl crate::Writable for PDMA_DSCT2_NEXT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PDMA_DSCT2_NEXT to value 0"]
impl crate::Resettable for PDMA_DSCT2_NEXT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
