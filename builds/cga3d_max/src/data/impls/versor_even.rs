impl From<AntiDipoleOnOrigin> for VersorEven {
    fn from(anti_dipole_on_origin: AntiDipoleOnOrigin) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([anti_dipole_on_origin[e423], anti_dipole_on_origin[e431], anti_dipole_on_origin[e412], 0.0]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, anti_dipole_on_origin[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}

impl From<AntiFlatOrigin> for VersorEven {
    fn from(anti_flat_origin: AntiFlatOrigin) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, anti_flat_origin[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}

impl From<AntiFlatPoint> for VersorEven {
    fn from(anti_flat_point: AntiFlatPoint) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, anti_flat_point[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from([anti_flat_point[e235], anti_flat_point[e315], anti_flat_point[e125], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}

impl From<AntiFlector> for VersorEven {
    fn from(anti_flector: AntiFlector) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, anti_flector[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from([anti_flector[e235], anti_flector[e315], anti_flector[e125], anti_flector[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from([anti_flector[e1], anti_flector[e2], anti_flector[e3], 0.0]),
        );
    }
}

impl From<AntiFlectorOnOrigin> for VersorEven {
    fn from(anti_flector_on_origin: AntiFlectorOnOrigin) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, anti_flector_on_origin[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([anti_flector_on_origin[e1], anti_flector_on_origin[e2], anti_flector_on_origin[e3], 0.0]),
        );
    }
}

impl From<AntiPlane> for VersorEven {
    fn from(anti_plane: AntiPlane) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, anti_plane[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from([anti_plane[e1], anti_plane[e2], anti_plane[e3], 0.0]),
        );
    }
}

impl From<AntiPlaneOnOrigin> for VersorEven {
    fn from(anti_plane_on_origin: AntiPlaneOnOrigin) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([anti_plane_on_origin[e1], anti_plane_on_origin[e2], anti_plane_on_origin[e3], 0.0]),
        );
    }
}

impl From<AntiScalar> for VersorEven {
    fn from(anti_scalar: AntiScalar) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([0.0, 0.0, 0.0, anti_scalar[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}

impl From<AntiSphereOnOrigin> for VersorEven {
    fn from(anti_sphere_on_origin: AntiSphereOnOrigin) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([anti_sphere_on_origin[e1], anti_sphere_on_origin[e2], anti_sphere_on_origin[e3], anti_sphere_on_origin[e4]]),
        );
    }
}

impl From<AntiVersorOddOnOrigin> for VersorEven {
    fn from(anti_versor_odd_on_origin: AntiVersorOddOnOrigin) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([anti_versor_odd_on_origin[e423], anti_versor_odd_on_origin[e431], anti_versor_odd_on_origin[e412], 0.0]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, anti_versor_odd_on_origin[e321]]),
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

impl From<Circle> for VersorEven {
    fn from(circle: Circle) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([circle[e423], circle[e431], circle[e412], 0.0]),
            // e415, e425, e435, e321
            Simd32x4::from([circle[e415], circle[e425], circle[e435], circle[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from([circle[e235], circle[e315], circle[e125], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}

impl From<CircleAligningOrigin> for VersorEven {
    fn from(circle_aligning_origin: CircleAligningOrigin) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([circle_aligning_origin[e423], circle_aligning_origin[e431], circle_aligning_origin[e412], 0.0]),
            // e415, e425, e435, e321
            Simd32x4::from([circle_aligning_origin[e415], circle_aligning_origin[e425], circle_aligning_origin[e435], 0.0]),
            // e235, e315, e125, e5
            Simd32x4::from([circle_aligning_origin[e235], circle_aligning_origin[e315], circle_aligning_origin[e125], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}

impl From<CircleAtInfinity> for VersorEven {
    fn from(circle_at_infinity: CircleAtInfinity) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([circle_at_infinity[e415], circle_at_infinity[e425], circle_at_infinity[e435], circle_at_infinity[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from([circle_at_infinity[e235], circle_at_infinity[e315], circle_at_infinity[e125], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}

impl From<CircleAtOrigin> for VersorEven {
    fn from(circle_at_origin: CircleAtOrigin) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([circle_at_origin[e423], circle_at_origin[e431], circle_at_origin[e412], 0.0]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([circle_at_origin[e235], circle_at_origin[e315], circle_at_origin[e125], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}

impl From<CircleOnOrigin> for VersorEven {
    fn from(circle_on_origin: CircleOnOrigin) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([circle_on_origin[e423], circle_on_origin[e431], circle_on_origin[e412], 0.0]),
            // e415, e425, e435, e321
            Simd32x4::from([circle_on_origin[e415], circle_on_origin[e425], circle_on_origin[e435], 0.0]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}

impl From<CircleOrthogonalOrigin> for VersorEven {
    fn from(circle_orthogonal_origin: CircleOrthogonalOrigin) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([circle_orthogonal_origin[e423], circle_orthogonal_origin[e431], circle_orthogonal_origin[e412], 0.0]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, circle_orthogonal_origin[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from([circle_orthogonal_origin[e235], circle_orthogonal_origin[e315], circle_orthogonal_origin[e125], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}

impl From<DualNum> for VersorEven {
    fn from(dual_num: DualNum) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([0.0, 0.0, 0.0, dual_num[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, dual_num[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}

impl From<Infinity> for VersorEven {
    fn from(infinity: Infinity) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, infinity[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}

impl From<Line> for VersorEven {
    fn from(line: Line) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([line[e415], line[e425], line[e435], 0.0]),
            // e235, e315, e125, e5
            Simd32x4::from([line[e235], line[e315], line[e125], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}

impl From<LineAtInfinity> for VersorEven {
    fn from(line_at_infinity: LineAtInfinity) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([line_at_infinity[e235], line_at_infinity[e315], line_at_infinity[e125], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}

impl From<LineOnOrigin> for VersorEven {
    fn from(line_on_origin: LineOnOrigin) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([line_on_origin[e415], line_on_origin[e425], line_on_origin[e435], 0.0]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}

impl From<Motor> for VersorEven {
    fn from(motor: Motor) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([0.0, 0.0, 0.0, motor[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from([motor[e415], motor[e425], motor[e435], 0.0]),
            // e235, e315, e125, e5
            Simd32x4::from([motor[e235], motor[e315], motor[e125], motor[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}

impl From<MotorAtInfinity> for VersorEven {
    fn from(motor_at_infinity: MotorAtInfinity) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([motor_at_infinity[e235], motor_at_infinity[e315], motor_at_infinity[e125], motor_at_infinity[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}

impl From<MotorOnOrigin> for VersorEven {
    fn from(motor_on_origin: MotorOnOrigin) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([0.0, 0.0, 0.0, motor_on_origin[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from([motor_on_origin[e415], motor_on_origin[e425], motor_on_origin[e435], 0.0]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}

impl From<NullCircleAtOrigin> for VersorEven {
    fn from(null_circle_at_origin: NullCircleAtOrigin) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([null_circle_at_origin[e423], null_circle_at_origin[e431], null_circle_at_origin[e412], 0.0]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}

impl From<NullVersorEvenAtOrigin> for VersorEven {
    fn from(null_versor_even_at_origin: NullVersorEvenAtOrigin) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([null_versor_even_at_origin[e423], null_versor_even_at_origin[e431], null_versor_even_at_origin[e412], 0.0]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, null_versor_even_at_origin[e4]]),
        );
    }
}

impl From<Origin> for VersorEven {
    fn from(origin: Origin) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, origin[e4]]),
        );
    }
}

impl From<RoundPoint> for VersorEven {
    fn from(round_point: RoundPoint) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, round_point[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from([round_point[e1], round_point[e2], round_point[e3], round_point[e4]]),
        );
    }
}

impl From<RoundPointAtOrigin> for VersorEven {
    fn from(round_point_at_origin: RoundPointAtOrigin) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, round_point_at_origin[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, round_point_at_origin[e4]]),
        );
    }
}

impl From<VersorEvenAligningOrigin> for VersorEven {
    fn from(versor_even_aligning_origin: VersorEvenAligningOrigin) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                versor_even_aligning_origin[e423],
                versor_even_aligning_origin[e431],
                versor_even_aligning_origin[e412],
                versor_even_aligning_origin[e12345],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from([versor_even_aligning_origin[e415], versor_even_aligning_origin[e425], versor_even_aligning_origin[e435], 0.0]),
            // e235, e315, e125, e5
            Simd32x4::from([
                versor_even_aligning_origin[e235],
                versor_even_aligning_origin[e315],
                versor_even_aligning_origin[e125],
                versor_even_aligning_origin[e5],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, versor_even_aligning_origin[e4]]),
        );
    }
}

impl From<VersorEvenAtInfinity> for VersorEven {
    fn from(versor_even_at_infinity: VersorEvenAtInfinity) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([0.0, 0.0, 0.0, versor_even_at_infinity[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from([
                versor_even_at_infinity[e415],
                versor_even_at_infinity[e425],
                versor_even_at_infinity[e435],
                versor_even_at_infinity[e321],
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([versor_even_at_infinity[e235], versor_even_at_infinity[e315], versor_even_at_infinity[e125], versor_even_at_infinity[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from([versor_even_at_infinity[e1], versor_even_at_infinity[e2], versor_even_at_infinity[e3], 0.0]),
        );
    }
}

impl From<VersorEvenAtOrigin> for VersorEven {
    fn from(versor_even_at_origin: VersorEvenAtOrigin) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([versor_even_at_origin[e423], versor_even_at_origin[e431], versor_even_at_origin[e412], 0.0]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([versor_even_at_origin[e235], versor_even_at_origin[e315], versor_even_at_origin[e125], versor_even_at_origin[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, versor_even_at_origin[e4]]),
        );
    }
}

impl From<VersorEvenOnOrigin> for VersorEven {
    fn from(versor_even_on_origin: VersorEvenOnOrigin) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([versor_even_on_origin[e423], versor_even_on_origin[e431], versor_even_on_origin[e412], versor_even_on_origin[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from([versor_even_on_origin[e415], versor_even_on_origin[e425], versor_even_on_origin[e435], 0.0]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, versor_even_on_origin[e4]]),
        );
    }
}

impl From<VersorEvenOrthogonalOrigin> for VersorEven {
    fn from(versor_even_orthogonal_origin: VersorEvenOrthogonalOrigin) -> Self {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([versor_even_orthogonal_origin[e423], versor_even_orthogonal_origin[e431], versor_even_orthogonal_origin[e412], 0.0]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, versor_even_orthogonal_origin[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from([
                versor_even_orthogonal_origin[e235],
                versor_even_orthogonal_origin[e315],
                versor_even_orthogonal_origin[e125],
                versor_even_orthogonal_origin[e5],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                versor_even_orthogonal_origin[e1],
                versor_even_orthogonal_origin[e2],
                versor_even_orthogonal_origin[e3],
                versor_even_orthogonal_origin[e4],
            ]),
        );
    }
}

impl TryFrom<MultiVector> for VersorEven {
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
            let mut error = "Elements from MultiVector do not fit into VersorEven { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([multi_vector[e423], multi_vector[e431], multi_vector[e412], multi_vector[e12345]]),
            // e415, e425, e435, e321
            Simd32x4::from([multi_vector[e415], multi_vector[e425], multi_vector[e435], multi_vector[e321]]),
            // e235, e315, e125, e5
            Simd32x4::from([multi_vector[e235], multi_vector[e315], multi_vector[e125], multi_vector[e5]]),
            // e1, e2, e3, e4
            Simd32x4::from([multi_vector[e1], multi_vector[e2], multi_vector[e3], multi_vector[e4]]),
        ));
    }
}
