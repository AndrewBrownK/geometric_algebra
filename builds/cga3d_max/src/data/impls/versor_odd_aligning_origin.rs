impl From<DipoleAligningOrigin> for VersorOddAligningOrigin {
    fn from(dipole_aligning_origin: DipoleAligningOrigin) -> Self {
        use crate::elements::*;
        return VersorOddAligningOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([dipole_aligning_origin[e41], dipole_aligning_origin[e42], dipole_aligning_origin[e43], dipole_aligning_origin[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from([dipole_aligning_origin[e15], dipole_aligning_origin[e25], dipole_aligning_origin[e35], 0.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}

impl From<DipoleAtOrigin> for VersorOddAligningOrigin {
    fn from(dipole_at_origin: DipoleAtOrigin) -> Self {
        use crate::elements::*;
        return VersorOddAligningOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([dipole_at_origin[e41], dipole_at_origin[e42], dipole_at_origin[e43], 0.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([dipole_at_origin[e15], dipole_at_origin[e25], dipole_at_origin[e35], 0.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}

impl From<DipoleOnOrigin> for VersorOddAligningOrigin {
    fn from(dipole_on_origin: DipoleOnOrigin) -> Self {
        use crate::elements::*;
        return VersorOddAligningOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([dipole_on_origin[e41], dipole_on_origin[e42], dipole_on_origin[e43], dipole_on_origin[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}

impl From<FlatOrigin> for VersorOddAligningOrigin {
    fn from(flat_origin: FlatOrigin) -> Self {
        use crate::elements::*;
        return VersorOddAligningOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, flat_origin[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}

impl From<FlatPoint> for VersorOddAligningOrigin {
    fn from(flat_point: FlatPoint) -> Self {
        use crate::elements::*;
        return VersorOddAligningOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, flat_point[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from([flat_point[e15], flat_point[e25], flat_point[e35], 0.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}

impl From<FlatPointAtInfinity> for VersorOddAligningOrigin {
    fn from(flat_point_at_infinity: FlatPointAtInfinity) -> Self {
        use crate::elements::*;
        return VersorOddAligningOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from([flat_point_at_infinity[e15], flat_point_at_infinity[e25], flat_point_at_infinity[e35], 0.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}

impl From<Flector> for VersorOddAligningOrigin {
    fn from(flector: Flector) -> Self {
        use crate::elements::*;
        return VersorOddAligningOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, flector[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from([flector[e15], flector[e25], flector[e35], 0.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([flector[e4235], flector[e4315], flector[e4125], flector[e3215]]),
        );
    }
}

impl From<FlectorAtInfinity> for VersorOddAligningOrigin {
    fn from(flector_at_infinity: FlectorAtInfinity) -> Self {
        use crate::elements::*;
        return VersorOddAligningOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from([flector_at_infinity[e15], flector_at_infinity[e25], flector_at_infinity[e35], 0.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, flector_at_infinity[e3215]]),
        );
    }
}

impl From<FlectorOnOrigin> for VersorOddAligningOrigin {
    fn from(flector_on_origin: FlectorOnOrigin) -> Self {
        use crate::elements::*;
        return VersorOddAligningOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, flector_on_origin[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([flector_on_origin[e4235], flector_on_origin[e4315], flector_on_origin[e4125], 0.0]),
        );
    }
}

impl From<Horizon> for VersorOddAligningOrigin {
    fn from(horizon: Horizon) -> Self {
        use crate::elements::*;
        return VersorOddAligningOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, horizon[e3215]]),
        );
    }
}

impl From<NullDipoleAtOrigin> for VersorOddAligningOrigin {
    fn from(null_dipole_at_origin: NullDipoleAtOrigin) -> Self {
        use crate::elements::*;
        return VersorOddAligningOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([null_dipole_at_origin[e41], null_dipole_at_origin[e42], null_dipole_at_origin[e43], 0.0]),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}

impl From<NullSphereAtOrigin> for VersorOddAligningOrigin {
    fn from(null_sphere_at_origin: NullSphereAtOrigin) -> Self {
        use crate::elements::*;
        return VersorOddAligningOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, null_sphere_at_origin[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}

impl From<NullVersorOddAtOrigin> for VersorOddAligningOrigin {
    fn from(null_versor_odd_at_origin: NullVersorOddAtOrigin) -> Self {
        use crate::elements::*;
        return VersorOddAligningOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([null_versor_odd_at_origin[e41], null_versor_odd_at_origin[e42], null_versor_odd_at_origin[e43], 0.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, null_versor_odd_at_origin[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}

impl From<Plane> for VersorOddAligningOrigin {
    fn from(plane: Plane) -> Self {
        use crate::elements::*;
        return VersorOddAligningOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([plane[e4235], plane[e4315], plane[e4125], plane[e3215]]),
        );
    }
}

impl From<PlaneOnOrigin> for VersorOddAligningOrigin {
    fn from(plane_on_origin: PlaneOnOrigin) -> Self {
        use crate::elements::*;
        return VersorOddAligningOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([plane_on_origin[e4235], plane_on_origin[e4315], plane_on_origin[e4125], 0.0]),
        );
    }
}

impl From<Sphere> for VersorOddAligningOrigin {
    fn from(sphere: Sphere) -> Self {
        use crate::elements::*;
        return VersorOddAligningOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, sphere[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([sphere[e4235], sphere[e4315], sphere[e4125], sphere[e3215]]),
        );
    }
}

impl From<SphereAtOrigin> for VersorOddAligningOrigin {
    fn from(sphere_at_origin: SphereAtOrigin) -> Self {
        use crate::elements::*;
        return VersorOddAligningOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, sphere_at_origin[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, sphere_at_origin[e3215]]),
        );
    }
}

impl From<SphereOnOrigin> for VersorOddAligningOrigin {
    fn from(sphere_on_origin: SphereOnOrigin) -> Self {
        use crate::elements::*;
        return VersorOddAligningOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, sphere_on_origin[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([sphere_on_origin[e4235], sphere_on_origin[e4315], sphere_on_origin[e4125], 0.0]),
        );
    }
}

impl From<VersorOddAtOrigin> for VersorOddAligningOrigin {
    fn from(versor_odd_at_origin: VersorOddAtOrigin) -> Self {
        use crate::elements::*;
        return VersorOddAligningOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([versor_odd_at_origin[e41], versor_odd_at_origin[e42], versor_odd_at_origin[e43], 0.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([versor_odd_at_origin[e15], versor_odd_at_origin[e25], versor_odd_at_origin[e35], versor_odd_at_origin[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, versor_odd_at_origin[e3215]]),
        );
    }
}

impl From<VersorOddOnOrigin> for VersorOddAligningOrigin {
    fn from(versor_odd_on_origin: VersorOddOnOrigin) -> Self {
        use crate::elements::*;
        return VersorOddAligningOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([versor_odd_on_origin[e41], versor_odd_on_origin[e42], versor_odd_on_origin[e43], versor_odd_on_origin[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, versor_odd_on_origin[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([versor_odd_on_origin[e4235], versor_odd_on_origin[e4315], versor_odd_on_origin[e4125], 0.0]),
        );
    }
}

impl TryFrom<AntiCircleOnOrigin> for VersorOddAligningOrigin {
    type Error = String;
    fn try_from(anti_circle_on_origin: AntiCircleOnOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_circle_on_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e23: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_circle_on_origin[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e31: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_circle_on_origin[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiCircleOnOrigin do not fit into VersorOddAligningOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorOddAligningOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([anti_circle_on_origin[e41], anti_circle_on_origin[e42], anti_circle_on_origin[e43], 0.0]),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<AntiDualNum> for VersorOddAligningOrigin {
    type Error = String;
    fn try_from(anti_dual_num: AntiDualNum) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_dual_num[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("scalar: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiDualNum do not fit into VersorOddAligningOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorOddAligningOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, anti_dual_num[e3215]]),
        ));
    }
}

impl TryFrom<AntiLine> for VersorOddAligningOrigin {
    type Error = String;
    fn try_from(anti_line: AntiLine) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_line[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e23: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_line[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e31: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_line[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiLine do not fit into VersorOddAligningOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorOddAligningOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from([anti_line[e15], anti_line[e25], anti_line[e35], 0.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<AntiMotor> for VersorOddAligningOrigin {
    type Error = String;
    fn try_from(anti_motor: AntiMotor) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_motor[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e23: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_motor[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e31: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_motor[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_motor[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("scalar: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiMotor do not fit into VersorOddAligningOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorOddAligningOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from([anti_motor[e15], anti_motor[e25], anti_motor[e35], 0.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, anti_motor[e3215]]),
        ));
    }
}

impl TryFrom<AntiVersorEvenOnOrigin> for VersorOddAligningOrigin {
    type Error = String;
    fn try_from(anti_versor_even_on_origin: AntiVersorEvenOnOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_versor_even_on_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("scalar: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_versor_even_on_origin[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e23: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_versor_even_on_origin[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e31: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_versor_even_on_origin[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiVersorEvenOnOrigin do not fit into VersorOddAligningOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorOddAligningOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([anti_versor_even_on_origin[e41], anti_versor_even_on_origin[e42], anti_versor_even_on_origin[e43], 0.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, anti_versor_even_on_origin[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<Dipole> for VersorOddAligningOrigin {
    type Error = String;
    fn try_from(dipole: Dipole) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = dipole[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e23: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e31: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from Dipole do not fit into VersorOddAligningOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorOddAligningOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([dipole[e41], dipole[e42], dipole[e43], dipole[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from([dipole[e15], dipole[e25], dipole[e35], 0.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<DipoleAtInfinity> for VersorOddAligningOrigin {
    type Error = String;
    fn try_from(dipole_at_infinity: DipoleAtInfinity) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = dipole_at_infinity[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e23: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_at_infinity[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e31: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_at_infinity[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from DipoleAtInfinity do not fit into VersorOddAligningOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorOddAligningOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, dipole_at_infinity[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from([dipole_at_infinity[e15], dipole_at_infinity[e25], dipole_at_infinity[e35], 0.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<DipoleOrthogonalOrigin> for VersorOddAligningOrigin {
    type Error = String;
    fn try_from(dipole_orthogonal_origin: DipoleOrthogonalOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = dipole_orthogonal_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e23: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_orthogonal_origin[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e31: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_orthogonal_origin[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from DipoleOrthogonalOrigin do not fit into VersorOddAligningOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorOddAligningOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([dipole_orthogonal_origin[e41], dipole_orthogonal_origin[e42], dipole_orthogonal_origin[e43], 0.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([dipole_orthogonal_origin[e15], dipole_orthogonal_origin[e25], dipole_orthogonal_origin[e35], 0.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        ));
    }
}

impl TryFrom<MultiVector> for VersorOddAligningOrigin {
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
        if fail {
            let mut error = "Elements from MultiVector do not fit into VersorOddAligningOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorOddAligningOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([multi_vector[e41], multi_vector[e42], multi_vector[e43], multi_vector[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from([multi_vector[e15], multi_vector[e25], multi_vector[e35], multi_vector[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([multi_vector[e4235], multi_vector[e4315], multi_vector[e4125], multi_vector[e3215]]),
        ));
    }
}

impl TryFrom<VersorOdd> for VersorOddAligningOrigin {
    type Error = String;
    fn try_from(versor_odd: VersorOdd) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = versor_odd[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("scalar: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e23: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e31: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorOdd do not fit into VersorOddAligningOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorOddAligningOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([versor_odd[e41], versor_odd[e42], versor_odd[e43], versor_odd[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from([versor_odd[e15], versor_odd[e25], versor_odd[e35], versor_odd[e1234]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([versor_odd[e4235], versor_odd[e4315], versor_odd[e4125], versor_odd[e3215]]),
        ));
    }
}

impl TryFrom<VersorOddAtInfinity> for VersorOddAligningOrigin {
    type Error = String;
    fn try_from(versor_odd_at_infinity: VersorOddAtInfinity) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = versor_odd_at_infinity[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("scalar: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd_at_infinity[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e23: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd_at_infinity[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e31: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd_at_infinity[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorOddAtInfinity do not fit into VersorOddAligningOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorOddAligningOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, versor_odd_at_infinity[e45]]),
            // e15, e25, e35, e1234
            Simd32x4::from([versor_odd_at_infinity[e15], versor_odd_at_infinity[e25], versor_odd_at_infinity[e35], 0.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                versor_odd_at_infinity[e4235],
                versor_odd_at_infinity[e4315],
                versor_odd_at_infinity[e4125],
                versor_odd_at_infinity[e3215],
            ]),
        ));
    }
}

impl TryFrom<VersorOddOrthogonalOrigin> for VersorOddAligningOrigin {
    type Error = String;
    fn try_from(versor_odd_orthogonal_origin: VersorOddOrthogonalOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = versor_odd_orthogonal_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("scalar: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd_orthogonal_origin[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e23: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd_orthogonal_origin[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e31: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd_orthogonal_origin[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorOddOrthogonalOrigin do not fit into VersorOddAligningOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(VersorOddAligningOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([versor_odd_orthogonal_origin[e41], versor_odd_orthogonal_origin[e42], versor_odd_orthogonal_origin[e43], 0.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                versor_odd_orthogonal_origin[e15],
                versor_odd_orthogonal_origin[e25],
                versor_odd_orthogonal_origin[e35],
                versor_odd_orthogonal_origin[e1234],
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, versor_odd_orthogonal_origin[e3215]]),
        ));
    }
}
