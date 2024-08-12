impl From<AntiCircleOnOrigin> for Dipole {
    fn from(anti_circle_on_origin: AntiCircleOnOrigin) -> Self {
        use crate::elements::*;
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from([anti_circle_on_origin[e41], anti_circle_on_origin[e42], anti_circle_on_origin[e43]]),
            // e23, e31, e12
            Simd32x3::from([anti_circle_on_origin[e23], anti_circle_on_origin[e31], anti_circle_on_origin[e12]]),
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
        );
    }
}

impl From<AntiLine> for Dipole {
    fn from(anti_line: AntiLine) -> Self {
        use crate::elements::*;
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from([anti_line[e23], anti_line[e31], anti_line[e12]]),
            // e15, e25, e35, e45
            Simd32x4::from([anti_line[e15], anti_line[e25], anti_line[e35], 0.0]),
        );
    }
}

impl From<AntiLineOnOrigin> for Dipole {
    fn from(anti_line_on_origin: AntiLineOnOrigin) -> Self {
        use crate::elements::*;
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from([anti_line_on_origin[e23], anti_line_on_origin[e31], anti_line_on_origin[e12]]),
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
        );
    }
}

impl From<DipoleAligningOrigin> for Dipole {
    fn from(dipole_aligning_origin: DipoleAligningOrigin) -> Self {
        use crate::elements::*;
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from([dipole_aligning_origin[e41], dipole_aligning_origin[e42], dipole_aligning_origin[e43]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e15, e25, e35, e45
            Simd32x4::from([dipole_aligning_origin[e15], dipole_aligning_origin[e25], dipole_aligning_origin[e35], dipole_aligning_origin[e45]]),
        );
    }
}

impl From<DipoleAtInfinity> for Dipole {
    fn from(dipole_at_infinity: DipoleAtInfinity) -> Self {
        use crate::elements::*;
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from([dipole_at_infinity[e23], dipole_at_infinity[e31], dipole_at_infinity[e12]]),
            // e15, e25, e35, e45
            Simd32x4::from([dipole_at_infinity[e15], dipole_at_infinity[e25], dipole_at_infinity[e35], dipole_at_infinity[e45]]),
        );
    }
}

impl From<DipoleAtOrigin> for Dipole {
    fn from(dipole_at_origin: DipoleAtOrigin) -> Self {
        use crate::elements::*;
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from([dipole_at_origin[e41], dipole_at_origin[e42], dipole_at_origin[e43]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e15, e25, e35, e45
            Simd32x4::from([dipole_at_origin[e15], dipole_at_origin[e25], dipole_at_origin[e35], 0.0]),
        );
    }
}

impl From<DipoleOnOrigin> for Dipole {
    fn from(dipole_on_origin: DipoleOnOrigin) -> Self {
        use crate::elements::*;
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from([dipole_on_origin[e41], dipole_on_origin[e42], dipole_on_origin[e43]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e15, e25, e35, e45
            Simd32x4::from([0.0, 0.0, 0.0, dipole_on_origin[e45]]),
        );
    }
}

impl From<DipoleOrthogonalOrigin> for Dipole {
    fn from(dipole_orthogonal_origin: DipoleOrthogonalOrigin) -> Self {
        use crate::elements::*;
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from([dipole_orthogonal_origin[e41], dipole_orthogonal_origin[e42], dipole_orthogonal_origin[e43]]),
            // e23, e31, e12
            Simd32x3::from([dipole_orthogonal_origin[e23], dipole_orthogonal_origin[e31], dipole_orthogonal_origin[e12]]),
            // e15, e25, e35, e45
            Simd32x4::from([dipole_orthogonal_origin[e15], dipole_orthogonal_origin[e25], dipole_orthogonal_origin[e35], 0.0]),
        );
    }
}

impl From<FlatOrigin> for Dipole {
    fn from(flat_origin: FlatOrigin) -> Self {
        use crate::elements::*;
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e15, e25, e35, e45
            Simd32x4::from([0.0, 0.0, 0.0, flat_origin[e45]]),
        );
    }
}

impl From<FlatPoint> for Dipole {
    fn from(flat_point: FlatPoint) -> Self {
        use crate::elements::*;
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e15, e25, e35, e45
            Simd32x4::from([flat_point[e15], flat_point[e25], flat_point[e35], flat_point[e45]]),
        );
    }
}

impl From<FlatPointAtInfinity> for Dipole {
    fn from(flat_point_at_infinity: FlatPointAtInfinity) -> Self {
        use crate::elements::*;
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e15, e25, e35, e45
            Simd32x4::from([flat_point_at_infinity[e15], flat_point_at_infinity[e25], flat_point_at_infinity[e35], 0.0]),
        );
    }
}

impl From<NullDipoleAtOrigin> for Dipole {
    fn from(null_dipole_at_origin: NullDipoleAtOrigin) -> Self {
        use crate::elements::*;
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from([null_dipole_at_origin[e41], null_dipole_at_origin[e42], null_dipole_at_origin[e43]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
        );
    }
}

impl TryFrom<AntiMotor> for Dipole {
    type Error = String;
    fn try_from(anti_motor: AntiMotor) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_motor[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("scalar: ");
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
            let mut error = "Elements from AntiMotor do not fit into Dipole { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from([anti_motor[e23], anti_motor[e31], anti_motor[e12]]),
            // e15, e25, e35, e45
            Simd32x4::from([anti_motor[e15], anti_motor[e25], anti_motor[e35], 0.0]),
        ));
    }
}

impl TryFrom<AntiMotorOnOrigin> for Dipole {
    type Error = String;
    fn try_from(anti_motor_on_origin: AntiMotorOnOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_motor_on_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("scalar: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiMotorOnOrigin do not fit into Dipole { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from([anti_motor_on_origin[e23], anti_motor_on_origin[e31], anti_motor_on_origin[e12]]),
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<Flector> for Dipole {
    type Error = String;
    fn try_from(flector: Flector) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = flector[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = flector[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = flector[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = flector[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3215: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from Flector do not fit into Dipole { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e15, e25, e35, e45
            Simd32x4::from([flector[e15], flector[e25], flector[e35], flector[e45]]),
        ));
    }
}

impl TryFrom<FlectorAtInfinity> for Dipole {
    type Error = String;
    fn try_from(flector_at_infinity: FlectorAtInfinity) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = flector_at_infinity[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3215: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from FlectorAtInfinity do not fit into Dipole { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e15, e25, e35, e45
            Simd32x4::from([flector_at_infinity[e15], flector_at_infinity[e25], flector_at_infinity[e35], 0.0]),
        ));
    }
}

impl TryFrom<FlectorOnOrigin> for Dipole {
    type Error = String;
    fn try_from(flector_on_origin: FlectorOnOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = flector_on_origin[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = flector_on_origin[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = flector_on_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from FlectorOnOrigin do not fit into Dipole { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e15, e25, e35, e45
            Simd32x4::from([0.0, 0.0, 0.0, flector_on_origin[e45]]),
        ));
    }
}

impl TryFrom<MultiVector> for Dipole {
    type Error = String;
    fn try_from(multi_vector: MultiVector) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = multi_vector[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("scalar: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
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
            error_string.push_str("e5: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[17];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e423: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[18];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e431: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[19];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e412: ");
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
            error_string.push_str("e415: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[22];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e425: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[23];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e435: ");
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
        let el = multi_vector[27];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[28];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[29];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[30];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3215: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[31];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1234: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from MultiVector do not fit into Dipole { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from([multi_vector[e41], multi_vector[e42], multi_vector[e43]]),
            // e23, e31, e12
            Simd32x3::from([multi_vector[e23], multi_vector[e31], multi_vector[e12]]),
            // e15, e25, e35, e45
            Simd32x4::from([multi_vector[e15], multi_vector[e25], multi_vector[e35], multi_vector[e45]]),
        ));
    }
}
