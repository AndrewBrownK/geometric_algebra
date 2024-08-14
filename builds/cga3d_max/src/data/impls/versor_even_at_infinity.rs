impl From<AntiFlatOrigin> for VersorEvenAtInfinity {
    fn from(anti_flat_origin: AntiFlatOrigin) -> Self {
        use crate::elements::*;
        return VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, anti_flat_origin[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
        );
    }
}

impl From<AntiFlatPoint> for VersorEvenAtInfinity {
    fn from(anti_flat_point: AntiFlatPoint) -> Self {
        use crate::elements::*;
        return VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, anti_flat_point[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from([anti_flat_point[e235], anti_flat_point[e315], anti_flat_point[e125], 0.0]),
        );
    }
}

impl From<AntiFlector> for VersorEvenAtInfinity {
    fn from(anti_flector: AntiFlector) -> Self {
        use crate::elements::*;
        return VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([0.0, anti_flector[e1], anti_flector[e2], anti_flector[e3]]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, anti_flector[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from([anti_flector[e235], anti_flector[e315], anti_flector[e125], anti_flector[e5]]),
        );
    }
}

impl From<AntiFlectorOnOrigin> for VersorEvenAtInfinity {
    fn from(anti_flector_on_origin: AntiFlectorOnOrigin) -> Self {
        use crate::elements::*;
        return VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([0.0, anti_flector_on_origin[e1], anti_flector_on_origin[e2], anti_flector_on_origin[e3]]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, anti_flector_on_origin[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
        );
    }
}

impl From<AntiPlane> for VersorEvenAtInfinity {
    fn from(anti_plane: AntiPlane) -> Self {
        use crate::elements::*;
        return VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([0.0, anti_plane[e1], anti_plane[e2], anti_plane[e3]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, anti_plane[e5]]),
        );
    }
}

impl From<AntiPlaneOnOrigin> for VersorEvenAtInfinity {
    fn from(anti_plane_on_origin: AntiPlaneOnOrigin) -> Self {
        use crate::elements::*;
        return VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([0.0, anti_plane_on_origin[e1], anti_plane_on_origin[e2], anti_plane_on_origin[e3]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
        );
    }
}

impl From<AntiScalar> for VersorEvenAtInfinity {
    fn from(anti_scalar: AntiScalar) -> Self {
        use crate::elements::*;
        return VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([anti_scalar[e12345], 0.0, 0.0, 0.0]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
        );
    }
}

impl From<CircleAtInfinity> for VersorEvenAtInfinity {
    fn from(circle_at_infinity: CircleAtInfinity) -> Self {
        use crate::elements::*;
        return VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([circle_at_infinity[e415], circle_at_infinity[e425], circle_at_infinity[e435], circle_at_infinity[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from([circle_at_infinity[e235], circle_at_infinity[e315], circle_at_infinity[e125], 0.0]),
        );
    }
}

impl From<DualNum> for VersorEvenAtInfinity {
    fn from(dual_num: DualNum) -> Self {
        use crate::elements::*;
        return VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([dual_num[e12345], 0.0, 0.0, 0.0]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, dual_num[e5]]),
        );
    }
}

impl From<Infinity> for VersorEvenAtInfinity {
    fn from(infinity: Infinity) -> Self {
        use crate::elements::*;
        return VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, infinity[e5]]),
        );
    }
}

impl From<Line> for VersorEvenAtInfinity {
    fn from(line: Line) -> Self {
        use crate::elements::*;
        return VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([line[e415], line[e425], line[e435], 0.0]),
            // e235, e315, e125, e5
            Simd32x4::from([line[e235], line[e315], line[e125], 0.0]),
        );
    }
}

impl From<LineAtInfinity> for VersorEvenAtInfinity {
    fn from(line_at_infinity: LineAtInfinity) -> Self {
        use crate::elements::*;
        return VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([line_at_infinity[e235], line_at_infinity[e315], line_at_infinity[e125], 0.0]),
        );
    }
}

impl From<LineOnOrigin> for VersorEvenAtInfinity {
    fn from(line_on_origin: LineOnOrigin) -> Self {
        use crate::elements::*;
        return VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([line_on_origin[e415], line_on_origin[e425], line_on_origin[e435], 0.0]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
        );
    }
}

impl From<Motor> for VersorEvenAtInfinity {
    fn from(motor: Motor) -> Self {
        use crate::elements::*;
        return VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([motor[e12345], 0.0, 0.0, 0.0]),
            // e415, e425, e435, e321
            Simd32x4::from([motor[e415], motor[e425], motor[e435], 0.0]),
            // e235, e315, e125, e5
            Simd32x4::from([motor[e235], motor[e315], motor[e125], motor[e5]]),
        );
    }
}

impl From<MotorAtInfinity> for VersorEvenAtInfinity {
    fn from(motor_at_infinity: MotorAtInfinity) -> Self {
        use crate::elements::*;
        return VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([motor_at_infinity[e235], motor_at_infinity[e315], motor_at_infinity[e125], motor_at_infinity[e5]]),
        );
    }
}

impl From<MotorOnOrigin> for VersorEvenAtInfinity {
    fn from(motor_on_origin: MotorOnOrigin) -> Self {
        use crate::elements::*;
        return VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([motor_on_origin[e12345], 0.0, 0.0, 0.0]),
            // e415, e425, e435, e321
            Simd32x4::from([motor_on_origin[e415], motor_on_origin[e425], motor_on_origin[e435], 0.0]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
        );
    }
}

impl TryFrom<AntiDipoleOnOrigin> for VersorEvenAtInfinity {
    type Error = String;
    fn try_from(anti_dipole_on_origin: AntiDipoleOnOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_dipole_on_origin[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e423: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_on_origin[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e431: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_on_origin[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e412: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiDipoleOnOrigin do not fit into VersorEvenAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, anti_dipole_on_origin[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<AntiSphereOnOrigin> for VersorEvenAtInfinity {
    type Error = String;
    fn try_from(anti_sphere_on_origin: AntiSphereOnOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_sphere_on_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiSphereOnOrigin do not fit into VersorEvenAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([0.0, anti_sphere_on_origin[e1], anti_sphere_on_origin[e2], anti_sphere_on_origin[e3]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<AntiVersorOddOnOrigin> for VersorEvenAtInfinity {
    type Error = String;
    fn try_from(anti_versor_odd_on_origin: AntiVersorOddOnOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_versor_odd_on_origin[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e423: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_versor_odd_on_origin[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e431: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_versor_odd_on_origin[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e412: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_versor_odd_on_origin[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiVersorOddOnOrigin do not fit into VersorEvenAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([0.0, anti_versor_odd_on_origin[e1], anti_versor_odd_on_origin[e2], anti_versor_odd_on_origin[e3]]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, anti_versor_odd_on_origin[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<Circle> for VersorEvenAtInfinity {
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
        if fail {
            let mut error = "Elements from Circle do not fit into VersorEvenAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([circle[e415], circle[e425], circle[e435], circle[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from([circle[e235], circle[e315], circle[e125], 0.0]),
        ));
    }
}

impl TryFrom<CircleAligningOrigin> for VersorEvenAtInfinity {
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
            let mut error = "Elements from CircleAligningOrigin do not fit into VersorEvenAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([circle_aligning_origin[e415], circle_aligning_origin[e425], circle_aligning_origin[e435], 0.0]),
            // e235, e315, e125, e5
            Simd32x4::from([circle_aligning_origin[e235], circle_aligning_origin[e315], circle_aligning_origin[e125], 0.0]),
        ));
    }
}

impl TryFrom<CircleAtOrigin> for VersorEvenAtInfinity {
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
            let mut error = "Elements from CircleAtOrigin do not fit into VersorEvenAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([circle_at_origin[e235], circle_at_origin[e315], circle_at_origin[e125], 0.0]),
        ));
    }
}

impl TryFrom<CircleOnOrigin> for VersorEvenAtInfinity {
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
            let mut error = "Elements from CircleOnOrigin do not fit into VersorEvenAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([circle_on_origin[e415], circle_on_origin[e425], circle_on_origin[e435], 0.0]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<CircleOrthogonalOrigin> for VersorEvenAtInfinity {
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
        if fail {
            let mut error = "Elements from CircleOrthogonalOrigin do not fit into VersorEvenAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, circle_orthogonal_origin[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from([circle_orthogonal_origin[e235], circle_orthogonal_origin[e315], circle_orthogonal_origin[e125], 0.0]),
        ));
    }
}

impl TryFrom<MultiVector> for VersorEvenAtInfinity {
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
        let el = multi_vector[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e41: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[8];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e42: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[9];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e43: ");
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
        let el = multi_vector[27];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1234: ");
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
            let mut error = "Elements from MultiVector do not fit into VersorEvenAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([multi_vector[e12345], multi_vector[e1], multi_vector[e2], multi_vector[e3]]),
            // e415, e425, e435, e321
            Simd32x4::from([multi_vector[e415], multi_vector[e425], multi_vector[e435], multi_vector[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from([multi_vector[e235], multi_vector[e315], multi_vector[e125], multi_vector[e5]]),
        ));
    }
}

impl TryFrom<RoundPoint> for VersorEvenAtInfinity {
    type Error = String;
    fn try_from(round_point: RoundPoint) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = round_point[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from RoundPoint do not fit into VersorEvenAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([0.0, round_point[e1], round_point[e2], round_point[e3]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, round_point[e5]]),
        ));
    }
}

impl TryFrom<RoundPointAtOrigin> for VersorEvenAtInfinity {
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
            let mut error = "Elements from RoundPointAtOrigin do not fit into VersorEvenAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, round_point_at_origin[e5]]),
        ));
    }
}

impl TryFrom<VersorEven> for VersorEvenAtInfinity {
    type Error = String;
    fn try_from(versor_even: VersorEven) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = versor_even[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e423: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e431: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e412: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even[15];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorEven do not fit into VersorEvenAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([versor_even[e12345], versor_even[e1], versor_even[e2], versor_even[e3]]),
            // e415, e425, e435, e321
            Simd32x4::from([versor_even[e415], versor_even[e425], versor_even[e435], versor_even[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from([versor_even[e235], versor_even[e315], versor_even[e125], versor_even[e5]]),
        ));
    }
}

impl TryFrom<VersorEvenAligningOrigin> for VersorEvenAtInfinity {
    type Error = String;
    fn try_from(versor_even_aligning_origin: VersorEvenAligningOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = versor_even_aligning_origin[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e423: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_aligning_origin[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e431: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_aligning_origin[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e412: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_aligning_origin[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorEvenAligningOrigin do not fit into VersorEvenAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([versor_even_aligning_origin[e12345], 0.0, 0.0, 0.0]),
            // e415, e425, e435, e321
            Simd32x4::from([versor_even_aligning_origin[e415], versor_even_aligning_origin[e425], versor_even_aligning_origin[e435], 0.0]),
            // e235, e315, e125, e5
            Simd32x4::from([
                versor_even_aligning_origin[e235],
                versor_even_aligning_origin[e315],
                versor_even_aligning_origin[e125],
                versor_even_aligning_origin[e5],
            ]),
        ));
    }
}

impl TryFrom<VersorEvenAtOrigin> for VersorEvenAtInfinity {
    type Error = String;
    fn try_from(versor_even_at_origin: VersorEvenAtOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = versor_even_at_origin[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e423: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_at_origin[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e431: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_at_origin[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e412: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_at_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorEvenAtOrigin do not fit into VersorEvenAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([versor_even_at_origin[e235], versor_even_at_origin[e315], versor_even_at_origin[e125], versor_even_at_origin[e5]]),
        ));
    }
}

impl TryFrom<VersorEvenOnOrigin> for VersorEvenAtInfinity {
    type Error = String;
    fn try_from(versor_even_on_origin: VersorEvenOnOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = versor_even_on_origin[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e423: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_on_origin[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e431: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_on_origin[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e412: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_on_origin[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorEvenOnOrigin do not fit into VersorEvenAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([versor_even_on_origin[e12345], 0.0, 0.0, 0.0]),
            // e415, e425, e435, e321
            Simd32x4::from([versor_even_on_origin[e415], versor_even_on_origin[e425], versor_even_on_origin[e435], 0.0]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<VersorEvenOrthogonalOrigin> for VersorEvenAtInfinity {
    type Error = String;
    fn try_from(versor_even_orthogonal_origin: VersorEvenOrthogonalOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = versor_even_orthogonal_origin[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e423: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_orthogonal_origin[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e431: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_orthogonal_origin[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e412: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_orthogonal_origin[11];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorEvenOrthogonalOrigin do not fit into VersorEvenAtInfinity { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([0.0, versor_even_orthogonal_origin[e1], versor_even_orthogonal_origin[e2], versor_even_orthogonal_origin[e3]]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, versor_even_orthogonal_origin[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from([
                versor_even_orthogonal_origin[e235],
                versor_even_orthogonal_origin[e315],
                versor_even_orthogonal_origin[e125],
                versor_even_orthogonal_origin[e5],
            ]),
        ));
    }
}
