impl From<AntiDipoleOnOrigin> for VersorEvenOrthogonalOrigin {
    fn from(anti_dipole_on_origin: AntiDipoleOnOrigin) -> Self {
        use crate::elements::*;
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from([anti_dipole_on_origin[e423], anti_dipole_on_origin[e431], anti_dipole_on_origin[e412], anti_dipole_on_origin[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}

impl From<AntiFlatOrigin> for VersorEvenOrthogonalOrigin {
    fn from(anti_flat_origin: AntiFlatOrigin) -> Self {
        use crate::elements::*;
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from([0.0, 0.0, 0.0, anti_flat_origin[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}

impl From<AntiFlatPoint> for VersorEvenOrthogonalOrigin {
    fn from(anti_flat_point: AntiFlatPoint) -> Self {
        use crate::elements::*;
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from([0.0, 0.0, 0.0, anti_flat_point[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from([anti_flat_point[e235], anti_flat_point[e315], anti_flat_point[e125], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}

impl From<AntiFlector> for VersorEvenOrthogonalOrigin {
    fn from(anti_flector: AntiFlector) -> Self {
        use crate::elements::*;
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from([0.0, 0.0, 0.0, anti_flector[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from([anti_flector[e235], anti_flector[e315], anti_flector[e125], anti_flector[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from([anti_flector[e1], anti_flector[e2], anti_flector[e3], 0.0]),
        );
    }
}

impl From<AntiFlectorOnOrigin> for VersorEvenOrthogonalOrigin {
    fn from(anti_flector_on_origin: AntiFlectorOnOrigin) -> Self {
        use crate::elements::*;
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from([0.0, 0.0, 0.0, anti_flector_on_origin[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([anti_flector_on_origin[e1], anti_flector_on_origin[e2], anti_flector_on_origin[e3], 0.0]),
        );
    }
}

impl From<AntiPlane> for VersorEvenOrthogonalOrigin {
    fn from(anti_plane: AntiPlane) -> Self {
        use crate::elements::*;
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, anti_plane[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from([anti_plane[e1], anti_plane[e2], anti_plane[e3], 0.0]),
        );
    }
}

impl From<AntiPlaneOnOrigin> for VersorEvenOrthogonalOrigin {
    fn from(anti_plane_on_origin: AntiPlaneOnOrigin) -> Self {
        use crate::elements::*;
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([anti_plane_on_origin[e1], anti_plane_on_origin[e2], anti_plane_on_origin[e3], 0.0]),
        );
    }
}

impl From<AntiSphereOnOrigin> for VersorEvenOrthogonalOrigin {
    fn from(anti_sphere_on_origin: AntiSphereOnOrigin) -> Self {
        use crate::elements::*;
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([anti_sphere_on_origin[e1], anti_sphere_on_origin[e2], anti_sphere_on_origin[e3], anti_sphere_on_origin[e4]]),
        );
    }
}

impl From<AntiVersorOddOnOrigin> for VersorEvenOrthogonalOrigin {
    fn from(anti_versor_odd_on_origin: AntiVersorOddOnOrigin) -> Self {
        use crate::elements::*;
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from([
                anti_versor_odd_on_origin[e423],
                anti_versor_odd_on_origin[e431],
                anti_versor_odd_on_origin[e412],
                anti_versor_odd_on_origin[e321],
            ]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([
                anti_versor_odd_on_origin[e1],
                anti_versor_odd_on_origin[e2],
                anti_versor_odd_on_origin[e3],
                anti_versor_odd_on_origin[e4],
            ]),
        );
    }
}

impl From<CircleAtOrigin> for VersorEvenOrthogonalOrigin {
    fn from(circle_at_origin: CircleAtOrigin) -> Self {
        use crate::elements::*;
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from([circle_at_origin[e423], circle_at_origin[e431], circle_at_origin[e412], 0.0]),
            // e235, e315, e125, e5
            Simd32x4::from([circle_at_origin[e235], circle_at_origin[e315], circle_at_origin[e125], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}

impl From<CircleOrthogonalOrigin> for VersorEvenOrthogonalOrigin {
    fn from(circle_orthogonal_origin: CircleOrthogonalOrigin) -> Self {
        use crate::elements::*;
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from([
                circle_orthogonal_origin[e423],
                circle_orthogonal_origin[e431],
                circle_orthogonal_origin[e412],
                circle_orthogonal_origin[e321],
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([circle_orthogonal_origin[e235], circle_orthogonal_origin[e315], circle_orthogonal_origin[e125], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}

impl From<Infinity> for VersorEvenOrthogonalOrigin {
    fn from(infinity: Infinity) -> Self {
        use crate::elements::*;
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, infinity[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}

impl From<LineAtInfinity> for VersorEvenOrthogonalOrigin {
    fn from(line_at_infinity: LineAtInfinity) -> Self {
        use crate::elements::*;
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([line_at_infinity[e235], line_at_infinity[e315], line_at_infinity[e125], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}

impl From<MotorAtInfinity> for VersorEvenOrthogonalOrigin {
    fn from(motor_at_infinity: MotorAtInfinity) -> Self {
        use crate::elements::*;
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([motor_at_infinity[e235], motor_at_infinity[e315], motor_at_infinity[e125], motor_at_infinity[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}

impl From<NullCircleAtOrigin> for VersorEvenOrthogonalOrigin {
    fn from(null_circle_at_origin: NullCircleAtOrigin) -> Self {
        use crate::elements::*;
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from([null_circle_at_origin[e423], null_circle_at_origin[e431], null_circle_at_origin[e412], 0.0]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}

impl From<NullVersorEvenAtOrigin> for VersorEvenOrthogonalOrigin {
    fn from(null_versor_even_at_origin: NullVersorEvenAtOrigin) -> Self {
        use crate::elements::*;
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from([null_versor_even_at_origin[e423], null_versor_even_at_origin[e431], null_versor_even_at_origin[e412], 0.0]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, null_versor_even_at_origin[e4]]),
        );
    }
}

impl From<Origin> for VersorEvenOrthogonalOrigin {
    fn from(origin: Origin) -> Self {
        use crate::elements::*;
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, origin[e4]]),
        );
    }
}

impl From<RoundPoint> for VersorEvenOrthogonalOrigin {
    fn from(round_point: RoundPoint) -> Self {
        use crate::elements::*;
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, round_point[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from([round_point[e1], round_point[e2], round_point[e3], round_point[e4]]),
        );
    }
}

impl From<RoundPointAtOrigin> for VersorEvenOrthogonalOrigin {
    fn from(round_point_at_origin: RoundPointAtOrigin) -> Self {
        use crate::elements::*;
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, round_point_at_origin[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, round_point_at_origin[e4]]),
        );
    }
}

impl From<VersorEvenAtOrigin> for VersorEvenOrthogonalOrigin {
    fn from(versor_even_at_origin: VersorEvenAtOrigin) -> Self {
        use crate::elements::*;
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from([versor_even_at_origin[e423], versor_even_at_origin[e431], versor_even_at_origin[e412], 0.0]),
            // e235, e315, e125, e5
            Simd32x4::from([versor_even_at_origin[e235], versor_even_at_origin[e315], versor_even_at_origin[e125], versor_even_at_origin[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, versor_even_at_origin[e4]]),
        );
    }
}

impl TryFrom<Circle> for VersorEvenOrthogonalOrigin {
    type Error = String;
    fn try_from(circle: Circle) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = circle[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e415: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e425: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e435: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from Circle do not fit into VersorEvenOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from([circle[e423], circle[e431], circle[e412], circle[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from([circle[e235], circle[e315], circle[e125], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<CircleAligningOrigin> for VersorEvenOrthogonalOrigin {
    type Error = String;
    fn try_from(circle_aligning_origin: CircleAligningOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = circle_aligning_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e415: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_aligning_origin[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e425: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_aligning_origin[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e435: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from CircleAligningOrigin do not fit into VersorEvenOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from([circle_aligning_origin[e423], circle_aligning_origin[e431], circle_aligning_origin[e412], 0.0]),
            // e235, e315, e125, e5
            Simd32x4::from([circle_aligning_origin[e235], circle_aligning_origin[e315], circle_aligning_origin[e125], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<CircleAtInfinity> for VersorEvenOrthogonalOrigin {
    type Error = String;
    fn try_from(circle_at_infinity: CircleAtInfinity) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = circle_at_infinity[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e415: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_at_infinity[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e425: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_at_infinity[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e435: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from CircleAtInfinity do not fit into VersorEvenOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from([0.0, 0.0, 0.0, circle_at_infinity[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from([circle_at_infinity[e235], circle_at_infinity[e315], circle_at_infinity[e125], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<CircleOnOrigin> for VersorEvenOrthogonalOrigin {
    type Error = String;
    fn try_from(circle_on_origin: CircleOnOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = circle_on_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e415: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_on_origin[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e425: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = circle_on_origin[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e435: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from CircleOnOrigin do not fit into VersorEvenOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from([circle_on_origin[e423], circle_on_origin[e431], circle_on_origin[e412], 0.0]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<DualNum> for VersorEvenOrthogonalOrigin {
    type Error = String;
    fn try_from(dual_num: DualNum) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = dual_num[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12345: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from DualNum do not fit into VersorEvenOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, dual_num[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<Line> for VersorEvenOrthogonalOrigin {
    type Error = String;
    fn try_from(line: Line) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = line[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e415: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = line[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e425: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = line[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e435: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from Line do not fit into VersorEvenOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([line[e235], line[e315], line[e125], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<Motor> for VersorEvenOrthogonalOrigin {
    type Error = String;
    fn try_from(motor: Motor) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = motor[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e415: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = motor[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e425: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = motor[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e435: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = motor[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12345: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from Motor do not fit into VersorEvenOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([motor[e235], motor[e315], motor[e125], motor[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<MultiVector> for VersorEvenOrthogonalOrigin {
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
            let mut error = "Elements from MultiVector do not fit into VersorEvenOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from([multi_vector[e423], multi_vector[e431], multi_vector[e412], multi_vector[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from([multi_vector[e235], multi_vector[e315], multi_vector[e125], multi_vector[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from([multi_vector[e1], multi_vector[e2], multi_vector[e3], multi_vector[e4]]),
        ));
    }
}

impl TryFrom<VersorEven> for VersorEvenOrthogonalOrigin {
    type Error = String;
    fn try_from(versor_even: VersorEven) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = versor_even[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12345: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e415: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e425: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e435: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorEven do not fit into VersorEvenOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from([versor_even[e423], versor_even[e431], versor_even[e412], versor_even[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from([versor_even[e235], versor_even[e315], versor_even[e125], versor_even[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from([versor_even[e1], versor_even[e2], versor_even[e3], versor_even[e4]]),
        ));
    }
}

impl TryFrom<VersorEvenAligningOrigin> for VersorEvenOrthogonalOrigin {
    type Error = String;
    fn try_from(versor_even_aligning_origin: VersorEvenAligningOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = versor_even_aligning_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12345: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_aligning_origin[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e415: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_aligning_origin[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e425: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_aligning_origin[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e435: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorEvenAligningOrigin do not fit into VersorEvenOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from([versor_even_aligning_origin[e423], versor_even_aligning_origin[e431], versor_even_aligning_origin[e412], 0.0]),
            // e235, e315, e125, e5
            Simd32x4::from([
                versor_even_aligning_origin[e235],
                versor_even_aligning_origin[e315],
                versor_even_aligning_origin[e125],
                versor_even_aligning_origin[e5],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, versor_even_aligning_origin[e4]]),
        ));
    }
}

impl TryFrom<VersorEvenAtInfinity> for VersorEvenOrthogonalOrigin {
    type Error = String;
    fn try_from(versor_even_at_infinity: VersorEvenAtInfinity) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = versor_even_at_infinity[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12345: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_at_infinity[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e415: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_at_infinity[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e425: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_at_infinity[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e435: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorEvenAtInfinity do not fit into VersorEvenOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from([0.0, 0.0, 0.0, versor_even_at_infinity[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from([versor_even_at_infinity[e235], versor_even_at_infinity[e315], versor_even_at_infinity[e125], versor_even_at_infinity[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from([versor_even_at_infinity[e1], versor_even_at_infinity[e2], versor_even_at_infinity[e3], 0.0]),
        ));
    }
}

impl TryFrom<VersorEvenOnOrigin> for VersorEvenOrthogonalOrigin {
    type Error = String;
    fn try_from(versor_even_on_origin: VersorEvenOnOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = versor_even_on_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12345: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_on_origin[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e415: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_on_origin[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e425: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_on_origin[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e435: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorEvenOnOrigin do not fit into VersorEvenOrthogonalOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from([versor_even_on_origin[e423], versor_even_on_origin[e431], versor_even_on_origin[e412], 0.0]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, versor_even_on_origin[e4]]),
        ));
    }
}
