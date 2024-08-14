// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)

impl From<AntiCircleOnOrigin> for AntiVersorEvenOnOrigin {
    fn from(anti_circle_on_origin: AntiCircleOnOrigin) -> Self {
        use crate::elements::*;
        return AntiVersorEvenOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([anti_circle_on_origin[e41], anti_circle_on_origin[e42], anti_circle_on_origin[e43], 0.0]),
            // e23, e31, e12, e1234
            Simd32x4::from([anti_circle_on_origin[e23], anti_circle_on_origin[e31], anti_circle_on_origin[e12], 0.0]),
        );
    }
}

impl From<AntiLineOnOrigin> for AntiVersorEvenOnOrigin {
    fn from(anti_line_on_origin: AntiLineOnOrigin) -> Self {
        use crate::elements::*;
        return AntiVersorEvenOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e1234
            Simd32x4::from([anti_line_on_origin[e23], anti_line_on_origin[e31], anti_line_on_origin[e12], 0.0]),
        );
    }
}

impl From<AntiMotorOnOrigin> for AntiVersorEvenOnOrigin {
    fn from(anti_motor_on_origin: AntiMotorOnOrigin) -> Self {
        use crate::elements::*;
        return AntiVersorEvenOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, anti_motor_on_origin[scalar]]),
            // e23, e31, e12, e1234
            Simd32x4::from([anti_motor_on_origin[e23], anti_motor_on_origin[e31], anti_motor_on_origin[e12], 0.0]),
        );
    }
}

impl From<NullDipoleAtOrigin> for AntiVersorEvenOnOrigin {
    fn from(null_dipole_at_origin: NullDipoleAtOrigin) -> Self {
        use crate::elements::*;
        return AntiVersorEvenOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([null_dipole_at_origin[e41], null_dipole_at_origin[e42], null_dipole_at_origin[e43], 0.0]),
            // e23, e31, e12, e1234
            Simd32x4::from(0.0),
        );
    }
}

impl From<NullSphereAtOrigin> for AntiVersorEvenOnOrigin {
    fn from(null_sphere_at_origin: NullSphereAtOrigin) -> Self {
        use crate::elements::*;
        return AntiVersorEvenOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e1234
            Simd32x4::from([0.0, 0.0, 0.0, null_sphere_at_origin[e1234]]),
        );
    }
}

impl From<NullVersorOddAtOrigin> for AntiVersorEvenOnOrigin {
    fn from(null_versor_odd_at_origin: NullVersorOddAtOrigin) -> Self {
        use crate::elements::*;
        return AntiVersorEvenOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([null_versor_odd_at_origin[e41], null_versor_odd_at_origin[e42], null_versor_odd_at_origin[e43], 0.0]),
            // e23, e31, e12, e1234
            Simd32x4::from([0.0, 0.0, 0.0, null_versor_odd_at_origin[e1234]]),
        );
    }
}

impl From<Scalar> for AntiVersorEvenOnOrigin {
    fn from(scalar: Scalar) -> Self {
        use crate::elements::*;
        return AntiVersorEvenOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, scalar[scalar]]),
            // e23, e31, e12, e1234
            Simd32x4::from(0.0),
        );
    }
}

impl TryFrom<AntiDualNum> for AntiVersorEvenOnOrigin {
    type Error = String;
    fn try_from(anti_dual_num: AntiDualNum) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_dual_num[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3215: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiDualNum do not fit into AntiVersorEvenOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiVersorEvenOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, anti_dual_num[scalar]]),
            // e23, e31, e12, e1234
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<AntiLine> for AntiVersorEvenOnOrigin {
    type Error = String;
    fn try_from(anti_line: AntiLine) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_line[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e15: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_line[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e25: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_line[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e35: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiLine do not fit into AntiVersorEvenOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiVersorEvenOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e1234
            Simd32x4::from([anti_line[e23], anti_line[e31], anti_line[e12], 0.0]),
        ));
    }
}

impl TryFrom<AntiMotor> for AntiVersorEvenOnOrigin {
    type Error = String;
    fn try_from(anti_motor: AntiMotor) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_motor[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e15: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_motor[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e25: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_motor[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e35: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_motor[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3215: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiMotor do not fit into AntiVersorEvenOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiVersorEvenOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, anti_motor[scalar]]),
            // e23, e31, e12, e1234
            Simd32x4::from([anti_motor[e23], anti_motor[e31], anti_motor[e12], 0.0]),
        ));
    }
}

impl TryFrom<Dipole> for AntiVersorEvenOnOrigin {
    type Error = String;
    fn try_from(dipole: Dipole) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = dipole[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e45: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e15: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole[8];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e25: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole[9];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e35: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from Dipole do not fit into AntiVersorEvenOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiVersorEvenOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([dipole[e41], dipole[e42], dipole[e43], 0.0]),
            // e23, e31, e12, e1234
            Simd32x4::from([dipole[e23], dipole[e31], dipole[e12], 0.0]),
        ));
    }
}

impl TryFrom<DipoleAligningOrigin> for AntiVersorEvenOnOrigin {
    type Error = String;
    fn try_from(dipole_aligning_origin: DipoleAligningOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = dipole_aligning_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e45: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_aligning_origin[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e15: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_aligning_origin[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e25: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_aligning_origin[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e35: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from DipoleAligningOrigin do not fit into AntiVersorEvenOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiVersorEvenOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([dipole_aligning_origin[e41], dipole_aligning_origin[e42], dipole_aligning_origin[e43], 0.0]),
            // e23, e31, e12, e1234
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<DipoleAtInfinity> for AntiVersorEvenOnOrigin {
    type Error = String;
    fn try_from(dipole_at_infinity: DipoleAtInfinity) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = dipole_at_infinity[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e45: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_at_infinity[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e15: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_at_infinity[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e25: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_at_infinity[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e35: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from DipoleAtInfinity do not fit into AntiVersorEvenOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiVersorEvenOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e1234
            Simd32x4::from([dipole_at_infinity[e23], dipole_at_infinity[e31], dipole_at_infinity[e12], 0.0]),
        ));
    }
}

impl TryFrom<DipoleAtOrigin> for AntiVersorEvenOnOrigin {
    type Error = String;
    fn try_from(dipole_at_origin: DipoleAtOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = dipole_at_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e15: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_at_origin[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e25: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_at_origin[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e35: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from DipoleAtOrigin do not fit into AntiVersorEvenOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiVersorEvenOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([dipole_at_origin[e41], dipole_at_origin[e42], dipole_at_origin[e43], 0.0]),
            // e23, e31, e12, e1234
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<DipoleOnOrigin> for AntiVersorEvenOnOrigin {
    type Error = String;
    fn try_from(dipole_on_origin: DipoleOnOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = dipole_on_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e45: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from DipoleOnOrigin do not fit into AntiVersorEvenOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiVersorEvenOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([dipole_on_origin[e41], dipole_on_origin[e42], dipole_on_origin[e43], 0.0]),
            // e23, e31, e12, e1234
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<DipoleOrthogonalOrigin> for AntiVersorEvenOnOrigin {
    type Error = String;
    fn try_from(dipole_orthogonal_origin: DipoleOrthogonalOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = dipole_orthogonal_origin[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e15: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_orthogonal_origin[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e25: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_orthogonal_origin[8];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e35: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from DipoleOrthogonalOrigin do not fit into AntiVersorEvenOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiVersorEvenOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([dipole_orthogonal_origin[e41], dipole_orthogonal_origin[e42], dipole_orthogonal_origin[e43], 0.0]),
            // e23, e31, e12, e1234
            Simd32x4::from([dipole_orthogonal_origin[e23], dipole_orthogonal_origin[e31], dipole_orthogonal_origin[e12], 0.0]),
        ));
    }
}

impl TryFrom<MultiVector> for AntiVersorEvenOnOrigin {
    type Error = String;
    fn try_from(multi_vector: MultiVector) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = multi_vector[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12345: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e2: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e5: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[10];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e45: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[11];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e15: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[12];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e25: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[13];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e35: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[17];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e415: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[18];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e425: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[19];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e435: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[20];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[21];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e423: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[22];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e431: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[23];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e412: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[24];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[25];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[26];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[28];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[29];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[30];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[31];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3215: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from MultiVector do not fit into AntiVersorEvenOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiVersorEvenOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([multi_vector[e41], multi_vector[e42], multi_vector[e43], multi_vector[scalar]]),
            // e23, e31, e12, e1234
            Simd32x4::from([multi_vector[e23], multi_vector[e31], multi_vector[e12], multi_vector[e1234]]),
        ));
    }
}

impl TryFrom<Sphere> for AntiVersorEvenOnOrigin {
    type Error = String;
    fn try_from(sphere: Sphere) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = sphere[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = sphere[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = sphere[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = sphere[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3215: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from Sphere do not fit into AntiVersorEvenOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiVersorEvenOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e1234
            Simd32x4::from([0.0, 0.0, 0.0, sphere[e1234]]),
        ));
    }
}

impl TryFrom<SphereAtOrigin> for AntiVersorEvenOnOrigin {
    type Error = String;
    fn try_from(sphere_at_origin: SphereAtOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = sphere_at_origin[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3215: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from SphereAtOrigin do not fit into AntiVersorEvenOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiVersorEvenOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e1234
            Simd32x4::from([0.0, 0.0, 0.0, sphere_at_origin[e1234]]),
        ));
    }
}

impl TryFrom<SphereOnOrigin> for AntiVersorEvenOnOrigin {
    type Error = String;
    fn try_from(sphere_on_origin: SphereOnOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = sphere_on_origin[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = sphere_on_origin[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = sphere_on_origin[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from SphereOnOrigin do not fit into AntiVersorEvenOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiVersorEvenOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e1234
            Simd32x4::from([0.0, 0.0, 0.0, sphere_on_origin[e1234]]),
        ));
    }
}

impl TryFrom<VersorOdd> for AntiVersorEvenOnOrigin {
    type Error = String;
    fn try_from(versor_odd: VersorOdd) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = versor_odd[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e45: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd[8];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e15: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd[9];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e25: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd[10];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e35: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd[12];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd[13];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd[14];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd[15];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3215: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorOdd do not fit into AntiVersorEvenOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiVersorEvenOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([versor_odd[e41], versor_odd[e42], versor_odd[e43], versor_odd[scalar]]),
            // e23, e31, e12, e1234
            Simd32x4::from([versor_odd[e23], versor_odd[e31], versor_odd[e12], versor_odd[e1234]]),
        ));
    }
}

impl TryFrom<VersorOddAligningOrigin> for AntiVersorEvenOnOrigin {
    type Error = String;
    fn try_from(versor_odd_aligning_origin: VersorOddAligningOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = versor_odd_aligning_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e45: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd_aligning_origin[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e15: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd_aligning_origin[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e25: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd_aligning_origin[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e35: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd_aligning_origin[8];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd_aligning_origin[9];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd_aligning_origin[10];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd_aligning_origin[11];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3215: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorOddAligningOrigin do not fit into AntiVersorEvenOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiVersorEvenOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([versor_odd_aligning_origin[e41], versor_odd_aligning_origin[e42], versor_odd_aligning_origin[e43], 0.0]),
            // e23, e31, e12, e1234
            Simd32x4::from([0.0, 0.0, 0.0, versor_odd_aligning_origin[e1234]]),
        ));
    }
}

impl TryFrom<VersorOddAtInfinity> for AntiVersorEvenOnOrigin {
    type Error = String;
    fn try_from(versor_odd_at_infinity: VersorOddAtInfinity) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = versor_odd_at_infinity[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e15: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd_at_infinity[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e25: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd_at_infinity[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e35: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd_at_infinity[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e45: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd_at_infinity[8];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd_at_infinity[9];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd_at_infinity[10];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd_at_infinity[11];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3215: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorOddAtInfinity do not fit into AntiVersorEvenOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiVersorEvenOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, versor_odd_at_infinity[scalar]]),
            // e23, e31, e12, e1234
            Simd32x4::from([versor_odd_at_infinity[e23], versor_odd_at_infinity[e31], versor_odd_at_infinity[e12], 0.0]),
        ));
    }
}

impl TryFrom<VersorOddAtOrigin> for AntiVersorEvenOnOrigin {
    type Error = String;
    fn try_from(versor_odd_at_origin: VersorOddAtOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = versor_odd_at_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3215: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd_at_origin[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e15: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd_at_origin[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e25: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd_at_origin[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e35: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorOddAtOrigin do not fit into AntiVersorEvenOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiVersorEvenOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([versor_odd_at_origin[e41], versor_odd_at_origin[e42], versor_odd_at_origin[e43], 0.0]),
            // e23, e31, e12, e1234
            Simd32x4::from([0.0, 0.0, 0.0, versor_odd_at_origin[e1234]]),
        ));
    }
}

impl TryFrom<VersorOddOnOrigin> for AntiVersorEvenOnOrigin {
    type Error = String;
    fn try_from(versor_odd_on_origin: VersorOddOnOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = versor_odd_on_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e45: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd_on_origin[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd_on_origin[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd_on_origin[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorOddOnOrigin do not fit into AntiVersorEvenOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiVersorEvenOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([versor_odd_on_origin[e41], versor_odd_on_origin[e42], versor_odd_on_origin[e43], 0.0]),
            // e23, e31, e12, e1234
            Simd32x4::from([0.0, 0.0, 0.0, versor_odd_on_origin[e1234]]),
        ));
    }
}

impl TryFrom<VersorOddOrthogonalOrigin> for AntiVersorEvenOnOrigin {
    type Error = String;
    fn try_from(versor_odd_orthogonal_origin: VersorOddOrthogonalOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = versor_odd_orthogonal_origin[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3215: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd_orthogonal_origin[8];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e15: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd_orthogonal_origin[9];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e25: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd_orthogonal_origin[10];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e35: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorOddOrthogonalOrigin do not fit into AntiVersorEvenOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiVersorEvenOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                versor_odd_orthogonal_origin[e41],
                versor_odd_orthogonal_origin[e42],
                versor_odd_orthogonal_origin[e43],
                versor_odd_orthogonal_origin[scalar],
            ]),
            // e23, e31, e12, e1234
            Simd32x4::from([
                versor_odd_orthogonal_origin[e23],
                versor_odd_orthogonal_origin[e31],
                versor_odd_orthogonal_origin[e12],
                versor_odd_orthogonal_origin[e1234],
            ]),
        ));
    }
}
