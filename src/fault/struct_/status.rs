#[doc = "Reader of register STATUS"]
pub type R = crate::R<u32, super::STATUS>;
#[doc = "Writer for register STATUS"]
pub type W = crate::W<u32, super::STATUS>;
#[doc = "Register STATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::STATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "The fault source index for which fault information is captured in DATA0 through DATA3. The fault information is fault source specific and described below. Note: this register field (and associated fault source data in DATA0 through DATA3) should only be considered valid, when VALID is '1'.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDX_A {
    #[doc = "0: Bus master 0 MPU/SMPU.\nDATA0\\[31:0\\]: Violating address.\nDATA1\\[0\\]: User read.\nDATA1\\[1\\]: User write.\nDATA1\\[2\\]: User execute.\nDATA1\\[3\\]: Privileged read.\nDATA1\\[4\\]: Privileged write.\nDATA1\\[5\\]: Privileged execute.\nDATA1\\[6\\]: Non-secure.\nDATA1\\[11:8\\]: Master identifier.\nDATA1\\[15:12\\]: Protection context identifier.\nDATA1\\[31\\]: '0' MPU violation; '1': SMPU violation."]
    MPU_0,
    #[doc = "1: Bus master 1 MPU. See MPU_0 description."]
    MPU_1,
    #[doc = "2: Bus master 2 MPU. See MPU_0 description."]
    MPU_2,
    #[doc = "3: Bus master 3 MPU. See MPU_0 description."]
    MPU_3,
    #[doc = "4: Bus master 4 MPU. See MPU_0 description."]
    MPU_4,
    #[doc = "5: Bus master 5 MPU. See MPU_0 description."]
    MPU_5,
    #[doc = "6: Bus master 6 MPU. See MPU_0 description."]
    MPU_6,
    #[doc = "7: Bus master 7 MPU. See MPU_0 description."]
    MPU_7,
    #[doc = "8: Bus master 8 MPU. See MPU_0 description."]
    MPU_8,
    #[doc = "9: Bus master 9 MPU. See MPU_0 description."]
    MPU_9,
    #[doc = "10: Bus master 10 MPU. See MPU_0 description."]
    MPU_10,
    #[doc = "11: Bus master 11 MPU. See MPU_0 description."]
    MPU_11,
    #[doc = "12: Bus master 12 MPU. See MPU_0 description."]
    MPU_12,
    #[doc = "13: Bus master 13 MPU. See MPU_0 description."]
    MPU_13,
    #[doc = "14: Bus master 14 MPU. See MPU_0 description."]
    MPU_14,
    #[doc = "15: Bus master 15 MPU. See MPU_0 description."]
    MPU_15,
    #[doc = "16: CM4 system bus AHB-Lite interface MPU. See MPU_0 description."]
    CM4_SYS_MPU,
    #[doc = "28: Peripheral master interface 0 PPU.\nDATA0\\[31:0\\]: Violating address.\nDATA1\\[0\\]: User read.\nDATA1\\[1\\]: User write.\nDATA1\\[2\\]: User execute.\nDATA1\\[3\\]: Privileged read.\nDATA1\\[4\\]: Privileged write.\nDATA1\\[5\\]: Privileged execute.\nDATA1\\[6\\]: Non-secure.\nDATA1\\[11:8\\]: Master identifier.\nDATA1\\[15:12\\]: Protection context identifier.\nDATA1\\[31\\]: '0': PPU violation, '1': peripheral bus error."]
    MS_PPU_0,
    #[doc = "29: Peripheral master interface 0 PPU. See MS_PPU_0 description."]
    MS_PPU_1,
    #[doc = "30: Peripheral master interface 1 PPU. See MS_PPU_0 description."]
    MS_PPU_2,
    #[doc = "31: Peripheral master interface 2 PPU. See MS_PPU_0 description."]
    MS_PPU_3,
    #[doc = "32: Peripheral group 0 PPU.\nDATA0\\[31:0\\]: Violating address.\nDATA1\\[0\\]: User read.\nDATA1\\[1\\]: User write.\nDATA1\\[2\\]: User execute.\nDATA1\\[3\\]: Privileged read.\nDATA1\\[4\\]: Privileged write.\nDATA1\\[5\\]: Privileged execute.\nDATA1\\[6\\]: Non-secure.\nDATA1\\[11:8\\]: Master identifier.\nDATA1\\[15:12\\]: Protection context identifier.\nDATA1\\[31:30\\]: '0': PPU violation, '1': timeout detected, '2': peripheral bus error."]
    GROUP_PPU_0,
    #[doc = "33: Peripheral group 1 PPU. See GROUP_PPU_0 description."]
    GROUP_PPU_1,
    #[doc = "34: Peripheral group 2 PPU. See GROUP_PPU_0 description."]
    GROUP_PPU_2,
    #[doc = "35: Peripheral group 3 PPU. See GROUP_PPU_0 description."]
    GROUP_PPU_3,
    #[doc = "36: Peripheral group 4 PPU. See GROUP_PPU_0 description."]
    GROUP_PPU_4,
    #[doc = "37: Peripheral group 5 PPU. See GROUP_PPU_0 description."]
    GROUP_PPU_5,
    #[doc = "38: Peripheral group 6 PPU. See GROUP_PPU_0 description."]
    GROUP_PPU_6,
    #[doc = "39: Peripheral group 7 PPU. See GROUP_PPU_0 description."]
    GROUP_PPU_7,
    #[doc = "40: Peripheral group 8 PPU. See GROUP_PPU_0 description."]
    GROUP_PPU_8,
    #[doc = "41: Peripheral group 9 PPU. See GROUP_PPU_0 description."]
    GROUP_PPU_9,
    #[doc = "42: Peripheral group 10 PPU. See GROUP_PPU_0 description."]
    GROUP_PPU_10,
    #[doc = "43: Peripheral group 11 PPU. See GROUP_PPU_0 description."]
    GROUP_PPU_11,
    #[doc = "44: Peripheral group 12 PPU. See GROUP_PPU_0 description."]
    GROUP_PPU_12,
    #[doc = "45: Peripheral group 13 PPU. See GROUP_PPU_0 description."]
    GROUP_PPU_13,
    #[doc = "46: Peripheral group 14 PPU. See GROUP_PPU_0 description."]
    GROUP_PPU_14,
    #[doc = "47: Peripheral group 15 PPU. See GROUP_PPU_0 description."]
    GROUP_PPU_15,
    #[doc = "50: Flash controller, main interface, bus error:\nFAULT_DATA0\\[31:0\\]: Violating address.\nFAULT_DATA1\\[31\\]: '0': FLASH macro interface bus error; '1': memory hole.\nFAULT_DATA1\\[15:12\\]: Protection context identifier.\nFAULT_DATA1\\[11:8\\]: Master identifier."]
    FLASHC_MAIN_BUS_ERROR,
}
impl From<IDX_A> for u8 {
    #[inline(always)]
    fn from(variant: IDX_A) -> Self {
        match variant {
            IDX_A::MPU_0 => 0,
            IDX_A::MPU_1 => 1,
            IDX_A::MPU_2 => 2,
            IDX_A::MPU_3 => 3,
            IDX_A::MPU_4 => 4,
            IDX_A::MPU_5 => 5,
            IDX_A::MPU_6 => 6,
            IDX_A::MPU_7 => 7,
            IDX_A::MPU_8 => 8,
            IDX_A::MPU_9 => 9,
            IDX_A::MPU_10 => 10,
            IDX_A::MPU_11 => 11,
            IDX_A::MPU_12 => 12,
            IDX_A::MPU_13 => 13,
            IDX_A::MPU_14 => 14,
            IDX_A::MPU_15 => 15,
            IDX_A::CM4_SYS_MPU => 16,
            IDX_A::MS_PPU_0 => 28,
            IDX_A::MS_PPU_1 => 29,
            IDX_A::MS_PPU_2 => 30,
            IDX_A::MS_PPU_3 => 31,
            IDX_A::GROUP_PPU_0 => 32,
            IDX_A::GROUP_PPU_1 => 33,
            IDX_A::GROUP_PPU_2 => 34,
            IDX_A::GROUP_PPU_3 => 35,
            IDX_A::GROUP_PPU_4 => 36,
            IDX_A::GROUP_PPU_5 => 37,
            IDX_A::GROUP_PPU_6 => 38,
            IDX_A::GROUP_PPU_7 => 39,
            IDX_A::GROUP_PPU_8 => 40,
            IDX_A::GROUP_PPU_9 => 41,
            IDX_A::GROUP_PPU_10 => 42,
            IDX_A::GROUP_PPU_11 => 43,
            IDX_A::GROUP_PPU_12 => 44,
            IDX_A::GROUP_PPU_13 => 45,
            IDX_A::GROUP_PPU_14 => 46,
            IDX_A::GROUP_PPU_15 => 47,
            IDX_A::FLASHC_MAIN_BUS_ERROR => 50,
        }
    }
}
#[doc = "Reader of field `IDX`"]
pub type IDX_R = crate::R<u8, IDX_A>;
impl IDX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, IDX_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(IDX_A::MPU_0),
            1 => Val(IDX_A::MPU_1),
            2 => Val(IDX_A::MPU_2),
            3 => Val(IDX_A::MPU_3),
            4 => Val(IDX_A::MPU_4),
            5 => Val(IDX_A::MPU_5),
            6 => Val(IDX_A::MPU_6),
            7 => Val(IDX_A::MPU_7),
            8 => Val(IDX_A::MPU_8),
            9 => Val(IDX_A::MPU_9),
            10 => Val(IDX_A::MPU_10),
            11 => Val(IDX_A::MPU_11),
            12 => Val(IDX_A::MPU_12),
            13 => Val(IDX_A::MPU_13),
            14 => Val(IDX_A::MPU_14),
            15 => Val(IDX_A::MPU_15),
            16 => Val(IDX_A::CM4_SYS_MPU),
            28 => Val(IDX_A::MS_PPU_0),
            29 => Val(IDX_A::MS_PPU_1),
            30 => Val(IDX_A::MS_PPU_2),
            31 => Val(IDX_A::MS_PPU_3),
            32 => Val(IDX_A::GROUP_PPU_0),
            33 => Val(IDX_A::GROUP_PPU_1),
            34 => Val(IDX_A::GROUP_PPU_2),
            35 => Val(IDX_A::GROUP_PPU_3),
            36 => Val(IDX_A::GROUP_PPU_4),
            37 => Val(IDX_A::GROUP_PPU_5),
            38 => Val(IDX_A::GROUP_PPU_6),
            39 => Val(IDX_A::GROUP_PPU_7),
            40 => Val(IDX_A::GROUP_PPU_8),
            41 => Val(IDX_A::GROUP_PPU_9),
            42 => Val(IDX_A::GROUP_PPU_10),
            43 => Val(IDX_A::GROUP_PPU_11),
            44 => Val(IDX_A::GROUP_PPU_12),
            45 => Val(IDX_A::GROUP_PPU_13),
            46 => Val(IDX_A::GROUP_PPU_14),
            47 => Val(IDX_A::GROUP_PPU_15),
            50 => Val(IDX_A::FLASHC_MAIN_BUS_ERROR),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MPU_0`"]
    #[inline(always)]
    pub fn is_mpu_0(&self) -> bool {
        *self == IDX_A::MPU_0
    }
    #[doc = "Checks if the value of the field is `MPU_1`"]
    #[inline(always)]
    pub fn is_mpu_1(&self) -> bool {
        *self == IDX_A::MPU_1
    }
    #[doc = "Checks if the value of the field is `MPU_2`"]
    #[inline(always)]
    pub fn is_mpu_2(&self) -> bool {
        *self == IDX_A::MPU_2
    }
    #[doc = "Checks if the value of the field is `MPU_3`"]
    #[inline(always)]
    pub fn is_mpu_3(&self) -> bool {
        *self == IDX_A::MPU_3
    }
    #[doc = "Checks if the value of the field is `MPU_4`"]
    #[inline(always)]
    pub fn is_mpu_4(&self) -> bool {
        *self == IDX_A::MPU_4
    }
    #[doc = "Checks if the value of the field is `MPU_5`"]
    #[inline(always)]
    pub fn is_mpu_5(&self) -> bool {
        *self == IDX_A::MPU_5
    }
    #[doc = "Checks if the value of the field is `MPU_6`"]
    #[inline(always)]
    pub fn is_mpu_6(&self) -> bool {
        *self == IDX_A::MPU_6
    }
    #[doc = "Checks if the value of the field is `MPU_7`"]
    #[inline(always)]
    pub fn is_mpu_7(&self) -> bool {
        *self == IDX_A::MPU_7
    }
    #[doc = "Checks if the value of the field is `MPU_8`"]
    #[inline(always)]
    pub fn is_mpu_8(&self) -> bool {
        *self == IDX_A::MPU_8
    }
    #[doc = "Checks if the value of the field is `MPU_9`"]
    #[inline(always)]
    pub fn is_mpu_9(&self) -> bool {
        *self == IDX_A::MPU_9
    }
    #[doc = "Checks if the value of the field is `MPU_10`"]
    #[inline(always)]
    pub fn is_mpu_10(&self) -> bool {
        *self == IDX_A::MPU_10
    }
    #[doc = "Checks if the value of the field is `MPU_11`"]
    #[inline(always)]
    pub fn is_mpu_11(&self) -> bool {
        *self == IDX_A::MPU_11
    }
    #[doc = "Checks if the value of the field is `MPU_12`"]
    #[inline(always)]
    pub fn is_mpu_12(&self) -> bool {
        *self == IDX_A::MPU_12
    }
    #[doc = "Checks if the value of the field is `MPU_13`"]
    #[inline(always)]
    pub fn is_mpu_13(&self) -> bool {
        *self == IDX_A::MPU_13
    }
    #[doc = "Checks if the value of the field is `MPU_14`"]
    #[inline(always)]
    pub fn is_mpu_14(&self) -> bool {
        *self == IDX_A::MPU_14
    }
    #[doc = "Checks if the value of the field is `MPU_15`"]
    #[inline(always)]
    pub fn is_mpu_15(&self) -> bool {
        *self == IDX_A::MPU_15
    }
    #[doc = "Checks if the value of the field is `CM4_SYS_MPU`"]
    #[inline(always)]
    pub fn is_cm4_sys_mpu(&self) -> bool {
        *self == IDX_A::CM4_SYS_MPU
    }
    #[doc = "Checks if the value of the field is `MS_PPU_0`"]
    #[inline(always)]
    pub fn is_ms_ppu_0(&self) -> bool {
        *self == IDX_A::MS_PPU_0
    }
    #[doc = "Checks if the value of the field is `MS_PPU_1`"]
    #[inline(always)]
    pub fn is_ms_ppu_1(&self) -> bool {
        *self == IDX_A::MS_PPU_1
    }
    #[doc = "Checks if the value of the field is `MS_PPU_2`"]
    #[inline(always)]
    pub fn is_ms_ppu_2(&self) -> bool {
        *self == IDX_A::MS_PPU_2
    }
    #[doc = "Checks if the value of the field is `MS_PPU_3`"]
    #[inline(always)]
    pub fn is_ms_ppu_3(&self) -> bool {
        *self == IDX_A::MS_PPU_3
    }
    #[doc = "Checks if the value of the field is `GROUP_PPU_0`"]
    #[inline(always)]
    pub fn is_group_ppu_0(&self) -> bool {
        *self == IDX_A::GROUP_PPU_0
    }
    #[doc = "Checks if the value of the field is `GROUP_PPU_1`"]
    #[inline(always)]
    pub fn is_group_ppu_1(&self) -> bool {
        *self == IDX_A::GROUP_PPU_1
    }
    #[doc = "Checks if the value of the field is `GROUP_PPU_2`"]
    #[inline(always)]
    pub fn is_group_ppu_2(&self) -> bool {
        *self == IDX_A::GROUP_PPU_2
    }
    #[doc = "Checks if the value of the field is `GROUP_PPU_3`"]
    #[inline(always)]
    pub fn is_group_ppu_3(&self) -> bool {
        *self == IDX_A::GROUP_PPU_3
    }
    #[doc = "Checks if the value of the field is `GROUP_PPU_4`"]
    #[inline(always)]
    pub fn is_group_ppu_4(&self) -> bool {
        *self == IDX_A::GROUP_PPU_4
    }
    #[doc = "Checks if the value of the field is `GROUP_PPU_5`"]
    #[inline(always)]
    pub fn is_group_ppu_5(&self) -> bool {
        *self == IDX_A::GROUP_PPU_5
    }
    #[doc = "Checks if the value of the field is `GROUP_PPU_6`"]
    #[inline(always)]
    pub fn is_group_ppu_6(&self) -> bool {
        *self == IDX_A::GROUP_PPU_6
    }
    #[doc = "Checks if the value of the field is `GROUP_PPU_7`"]
    #[inline(always)]
    pub fn is_group_ppu_7(&self) -> bool {
        *self == IDX_A::GROUP_PPU_7
    }
    #[doc = "Checks if the value of the field is `GROUP_PPU_8`"]
    #[inline(always)]
    pub fn is_group_ppu_8(&self) -> bool {
        *self == IDX_A::GROUP_PPU_8
    }
    #[doc = "Checks if the value of the field is `GROUP_PPU_9`"]
    #[inline(always)]
    pub fn is_group_ppu_9(&self) -> bool {
        *self == IDX_A::GROUP_PPU_9
    }
    #[doc = "Checks if the value of the field is `GROUP_PPU_10`"]
    #[inline(always)]
    pub fn is_group_ppu_10(&self) -> bool {
        *self == IDX_A::GROUP_PPU_10
    }
    #[doc = "Checks if the value of the field is `GROUP_PPU_11`"]
    #[inline(always)]
    pub fn is_group_ppu_11(&self) -> bool {
        *self == IDX_A::GROUP_PPU_11
    }
    #[doc = "Checks if the value of the field is `GROUP_PPU_12`"]
    #[inline(always)]
    pub fn is_group_ppu_12(&self) -> bool {
        *self == IDX_A::GROUP_PPU_12
    }
    #[doc = "Checks if the value of the field is `GROUP_PPU_13`"]
    #[inline(always)]
    pub fn is_group_ppu_13(&self) -> bool {
        *self == IDX_A::GROUP_PPU_13
    }
    #[doc = "Checks if the value of the field is `GROUP_PPU_14`"]
    #[inline(always)]
    pub fn is_group_ppu_14(&self) -> bool {
        *self == IDX_A::GROUP_PPU_14
    }
    #[doc = "Checks if the value of the field is `GROUP_PPU_15`"]
    #[inline(always)]
    pub fn is_group_ppu_15(&self) -> bool {
        *self == IDX_A::GROUP_PPU_15
    }
    #[doc = "Checks if the value of the field is `FLASHC_MAIN_BUS_ERROR`"]
    #[inline(always)]
    pub fn is_flashc_main_bus_error(&self) -> bool {
        *self == IDX_A::FLASHC_MAIN_BUS_ERROR
    }
}
#[doc = "Reader of field `VALID`"]
pub type VALID_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VALID`"]
pub struct VALID_W<'a> {
    w: &'a mut W,
}
impl<'a> VALID_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - The fault source index for which fault information is captured in DATA0 through DATA3. The fault information is fault source specific and described below. Note: this register field (and associated fault source data in DATA0 through DATA3) should only be considered valid, when VALID is '1'."]
    #[inline(always)]
    pub fn idx(&self) -> IDX_R {
        IDX_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 31 - Valid indication: '0': Invalid. '1': Valid. HW sets this field to '1' when new fault source data is captured. New fault source data is ONLY captured when VALID is '0'. SW can clear this field to '0' when the fault is handled (by SW)."]
    #[inline(always)]
    pub fn valid(&self) -> VALID_R {
        VALID_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Valid indication: '0': Invalid. '1': Valid. HW sets this field to '1' when new fault source data is captured. New fault source data is ONLY captured when VALID is '0'. SW can clear this field to '0' when the fault is handled (by SW)."]
    #[inline(always)]
    pub fn valid(&mut self) -> VALID_W {
        VALID_W { w: self }
    }
}
