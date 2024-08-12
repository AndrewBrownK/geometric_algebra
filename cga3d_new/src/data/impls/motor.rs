impl From<AntiScalar> for Motor {
    fn from(anti_scalar: AntiScalar) -> Self {
        use crate::elements::*;
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([0.0, 0.0, 0.0, anti_scalar[e12345]]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
        );
    }
}

impl From<Infinity> for Motor {
    fn from(infinity: Infinity) -> Self {
        use crate::elements::*;
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, infinity[e5]]),
        );
    }
}

impl From<Line> for Motor {
    fn from(line: Line) -> Self {
        use crate::elements::*;
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([line[e415], line[e425], line[e435], 0.0]),
            // e235, e315, e125, e5
            Simd32x4::from([line[e235], line[e315], line[e125], 0.0]),
        );
    }
}

impl From<LineAtInfinity> for Motor {
    fn from(line_at_infinity: LineAtInfinity) -> Self {
        use crate::elements::*;
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([line_at_infinity[e235], line_at_infinity[e315], line_at_infinity[e125], 0.0]),
        );
    }
}

impl From<LineOnOrigin> for Motor {
    fn from(line_on_origin: LineOnOrigin) -> Self {
        use crate::elements::*;
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([line_on_origin[e415], line_on_origin[e425], line_on_origin[e435], 0.0]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
        );
    }
}

impl From<MotorAtInfinity> for Motor {
    fn from(motor_at_infinity: MotorAtInfinity) -> Self {
        use crate::elements::*;
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([motor_at_infinity[e235], motor_at_infinity[e315], motor_at_infinity[e125], motor_at_infinity[e5]]),
        );
    }
}

impl From<MotorOnOrigin> for Motor {
    fn from(motor_on_origin: MotorOnOrigin) -> Self {
        use crate::elements::*;
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([motor_on_origin[e415], motor_on_origin[e425], motor_on_origin[e435], motor_on_origin[e12345]]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
        );
    }
}

impl TryFrom<AntiFlatPoint> for Motor {
    type Error = String;
    fn try_from(anti_flat_point: AntiFlatPoint) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_flat_point[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiFlatPoint do not fit into Motor { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([anti_flat_point[e235], anti_flat_point[e315], anti_flat_point[e125], 0.0]),
        ));
    }
}

impl TryFrom<AntiFlector> for Motor {
    type Error = String;
    fn try_from(anti_flector: AntiFlector) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_flector[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_flector[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_flector[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e2: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_flector[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiFlector do not fit into Motor { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([anti_flector[e235], anti_flector[e315], anti_flector[e125], anti_flector[e5]]),
        ));
    }
}

impl TryFrom<AntiPlane> for Motor {
    type Error = String;
    fn try_from(anti_plane: AntiPlane) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_plane[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_plane[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e2: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_plane[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiPlane do not fit into Motor { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, anti_plane[e5]]),
        ));
    }
}

impl TryFrom<Circle> for Motor {
    type Error = String;
    fn try_from(circle: Circle) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = circle[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e423: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e431: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e412: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from Circle do not fit into Motor { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([circle[e415], circle[e425], circle[e435], 0.0]),
            // e235, e315, e125, e5
            Simd32x4::from([circle[e235], circle[e315], circle[e125], 0.0]),
        ));
    }
}

impl TryFrom<CircleAligningOrigin> for Motor {
    type Error = String;
    fn try_from(circle_aligning_origin: CircleAligningOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = circle_aligning_origin[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e423: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_aligning_origin[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e431: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_aligning_origin[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e412: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from CircleAligningOrigin do not fit into Motor { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([circle_aligning_origin[e415], circle_aligning_origin[e425], circle_aligning_origin[e435], 0.0]),
            // e235, e315, e125, e5
            Simd32x4::from([circle_aligning_origin[e235], circle_aligning_origin[e315], circle_aligning_origin[e125], 0.0]),
        ));
    }
}

impl TryFrom<CircleAtInfinity> for Motor {
    type Error = String;
    fn try_from(circle_at_infinity: CircleAtInfinity) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = circle_at_infinity[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from CircleAtInfinity do not fit into Motor { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([circle_at_infinity[e415], circle_at_infinity[e425], circle_at_infinity[e435], 0.0]),
            // e235, e315, e125, e5
            Simd32x4::from([circle_at_infinity[e235], circle_at_infinity[e315], circle_at_infinity[e125], 0.0]),
        ));
    }
}

impl TryFrom<CircleAtOrigin> for Motor {
    type Error = String;
    fn try_from(circle_at_origin: CircleAtOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = circle_at_origin[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e423: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_at_origin[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e431: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_at_origin[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e412: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from CircleAtOrigin do not fit into Motor { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([circle_at_origin[e235], circle_at_origin[e315], circle_at_origin[e125], 0.0]),
        ));
    }
}

impl TryFrom<CircleOnOrigin> for Motor {
    type Error = String;
    fn try_from(circle_on_origin: CircleOnOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = circle_on_origin[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e423: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_on_origin[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e431: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_on_origin[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e412: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from CircleOnOrigin do not fit into Motor { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([circle_on_origin[e415], circle_on_origin[e425], circle_on_origin[e435], 0.0]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<CircleOrthogonalOrigin> for Motor {
    type Error = String;
    fn try_from(circle_orthogonal_origin: CircleOrthogonalOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = circle_orthogonal_origin[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e423: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_orthogonal_origin[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e431: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_orthogonal_origin[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e412: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_orthogonal_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from CircleOrthogonalOrigin do not fit into Motor { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([circle_orthogonal_origin[e235], circle_orthogonal_origin[e315], circle_orthogonal_origin[e125], 0.0]),
        ));
    }
}

impl TryFrom<DualNum> for Motor {
    type Error = String;
    fn try_from(dual_num: DualNum) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = dual_num[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("scalar: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from DualNum do not fit into Motor { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([0.0, 0.0, 0.0, dual_num[e12345]]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<MultiVector> for Motor {
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
        let el = multi_vector[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e15: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[8];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e25: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[9];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e35: ");
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
            error_string.push_str("e41: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[12];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e42: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[13];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e43: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[14];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e23: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[15];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e31: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[16];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12: ");
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
            let mut error = "Elements from MultiVector do not fit into Motor { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([multi_vector[e415], multi_vector[e425], multi_vector[e435], multi_vector[e12345]]),
            // e235, e315, e125, e5
            Simd32x4::from([multi_vector[e235], multi_vector[e315], multi_vector[e125], multi_vector[e5]]),
        ));
    }
}

impl TryFrom<RoundPoint> for Motor {
    type Error = String;
    fn try_from(round_point: RoundPoint) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = round_point[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = round_point[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e2: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = round_point[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = round_point[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from RoundPoint do not fit into Motor { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, round_point[e5]]),
        ));
    }
}

impl TryFrom<RoundPointAtOrigin> for Motor {
    type Error = String;
    fn try_from(round_point_at_origin: RoundPointAtOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = round_point_at_origin[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from RoundPointAtOrigin do not fit into Motor { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, round_point_at_origin[e5]]),
        ));
    }
}
