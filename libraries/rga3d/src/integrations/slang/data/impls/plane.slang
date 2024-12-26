using traits::GeometricProduct;
using traits::RightDual;
extension Plane: Add<AntiScalar> {
    func operator+(self, other: AntiScalar) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(0.0, other[e1234]), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            self.group0()
        );
    }
}
extension Plane: Add<DualNum> {
    func operator+(self, other: DualNum) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            other.group0(), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            self.group0()
        );
    }
}
extension Plane: Add<Flector> {
    func operator+(self, other: Flector) -> Flector {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            other.group0(), 
            /* e423, e431, e412, e321 */
            other.group1() + self.group0()
        );
    }
}
extension Plane: Add<Horizon> {
    func operator+(self, other: Horizon) -> Plane {
        return Plane::from_groups(
            /* e423, e431, e412, e321 */
            self.group0() + float4(float3(0.0), other[e321])
        );
    }
}
extension Plane {
    func operator+=(const Horizon& other) {
        *this = Plane::from_groups(
            /* e423, e431, e412, e321 */
            self.group0() + float4(float3(0.0), other[e321])
        );
    }
}
extension Plane: Add<Line> {
    func operator+(self, other: Line) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            other.group0(), 
            /* e23, e31, e12 */
            other.group1(), 
            /* e423, e431, e412, e321 */
            self.group0()
        );
    }
}
extension Plane: Add<Motor> {
    func operator+(self, other: Motor) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(other[scalar], other[e1234]), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            other.group0().xyz, 
            /* e23, e31, e12 */
            other.group1().xyz, 
            /* e423, e431, e412, e321 */
            self.group0()
        );
    }
}
extension Plane: Add<MultiVector> {
    func operator+(self, other: MultiVector) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            other.group0(), 
            /* e1, e2, e3, e4 */
            other.group1(), 
            /* e41, e42, e43 */
            other.group2(), 
            /* e23, e31, e12 */
            other.group3(), 
            /* e423, e431, e412, e321 */
            other.group4() + self.group0()
        );
    }
}
extension Plane: Add<Origin> {
    func operator+(self, other: Origin) -> Flector {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other[e4]), 
            /* e423, e431, e412, e321 */
            self.group0()
        );
    }
}
extension Plane: Add<Plane> {
    func operator+(self, other: Plane) -> Plane {
        return Plane::from_groups(
            /* e423, e431, e412, e321 */
            other.group0() + self.group0()
        );
    }
}
extension Plane {
    func operator+=(const Plane& other) {
        *this = Plane::from_groups(
            /* e423, e431, e412, e321 */
            other.group0() + self.group0()
        );
    }
}
extension Plane: Add<Point> {
    func operator+(self, other: Point) -> Flector {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            other.group0(), 
            /* e423, e431, e412, e321 */
            self.group0()
        );
    }
}
extension Plane: Add<Scalar> {
    func operator+(self, other: Scalar) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(other[scalar], 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            self.group0()
        );
    }
}

impl From<Horizon> for Plane {
    fn from(from_horizon: Horizon) -> Self {
        use crate::elements::*;
        return Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4(float3(0.0), from_horizon[e321])
        );
    }
}
extension Plane: Mul<AntiScalar> {
    func operator*(self, other: AntiScalar) -> Origin {
        return self.geometric_product(other);
    }
}
extension Plane: Mul<DualNum> {
    func operator*(self, other: DualNum) -> Flector {
        return self.geometric_product(other);
    }
}
extension Plane: Mul<Flector> {
    func operator*(self, other: Flector) -> Motor {
        return self.geometric_product(other);
    }
}
extension Plane: Mul<Horizon> {
    func operator*(self, other: Horizon) -> Motor {
        return self.geometric_product(other);
    }
}
extension Plane: Mul<Line> {
    func operator*(self, other: Line) -> Flector {
        return self.geometric_product(other);
    }
}
extension Plane: Mul<Motor> {
    func operator*(self, other: Motor) -> Flector {
        return self.geometric_product(other);
    }
}
extension Plane: Mul<MultiVector> {
    func operator*(self, other: MultiVector) -> MultiVector {
        return self.geometric_product(other);
    }
}
extension Plane: Mul<Origin> {
    func operator*(self, other: Origin) -> AntiScalar {
        return self.geometric_product(other);
    }
}
extension Plane: Mul<Plane> {
    func operator*(self, other: Plane) -> Motor {
        return self.geometric_product(other);
    }
}
extension Plane: Mul<Point> {
    func operator*(self, other: Point) -> Motor {
        return self.geometric_product(other);
    }
}
extension Plane: Mul<Scalar> {
    func operator*(self, other: Scalar) -> Plane {
        return self.geometric_product(other);
    }
}
extension Plane {
    func operator*=(const Scalar& other) {
        *this = self.geometric_product(other);
    }
}
extension Plane: Neg {
    func neg(self) -> Plane {
        return Plane::from_groups(
            /* e423, e431, e412, e321 */
            self.group0() * float4(-1.0)
        );
    }
}
extension Plane: Not {
    func not(self) -> Origin {
        return self.right_dual();
    }
}
extension Plane: Sub<AntiScalar> {
    func operator-(self, other: AntiScalar) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(1.0, other[e1234]) * float2(0.0, -1.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            self.group0()
        );
    }
}
extension Plane: Sub<DualNum> {
    func operator-(self, other: DualNum) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            other.group0() * float2(-1.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            self.group0()
        );
    }
}
extension Plane: Sub<Flector> {
    func operator-(self, other: Flector) -> Flector {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            other.group0() * float4(-1.0), 
            /* e423, e431, e412, e321 */
            self.group0() - other.group1()
        );
    }
}
extension Plane: Sub<Horizon> {
    func operator-(self, other: Horizon) -> Plane {
        return Plane::from_groups(
            /* e423, e431, e412, e321 */
            self.group0() + float4(float3(0.0), other[e321] * -1.0)
        );
    }
}
extension Plane {
    func operator-=(const Horizon& other) {
        *this = Plane::from_groups(
            /* e423, e431, e412, e321 */
            self.group0() + float4(float3(0.0), other[e321] * -1.0)
        );
    }
}
extension Plane: Sub<Line> {
    func operator-(self, other: Line) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            other.group0() * float3(-1.0), 
            /* e23, e31, e12 */
            other.group1() * float3(-1.0), 
            /* e423, e431, e412, e321 */
            self.group0()
        );
    }
}
extension Plane: Sub<Motor> {
    func operator-(self, other: Motor) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(other[scalar], other[e1234]) * float2(-1.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            other.group0().xyz * float3(-1.0), 
            /* e23, e31, e12 */
            other.group1().xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            self.group0()
        );
    }
}
extension Plane: Sub<MultiVector> {
    func operator-(self, other: MultiVector) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            other.group0() * float2(-1.0), 
            /* e1, e2, e3, e4 */
            other.group1() * float4(-1.0), 
            /* e41, e42, e43 */
            other.group2() * float3(-1.0), 
            /* e23, e31, e12 */
            other.group3() * float3(-1.0), 
            /* e423, e431, e412, e321 */
            self.group0() - other.group4()
        );
    }
}
extension Plane: Sub<Origin> {
    func operator-(self, other: Origin) -> Flector {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other[e4] * -1.0), 
            /* e423, e431, e412, e321 */
            self.group0()
        );
    }
}
extension Plane: Sub<Plane> {
    func operator-(self, other: Plane) -> Plane {
        return Plane::from_groups(
            /* e423, e431, e412, e321 */
            self.group0() - other.group0()
        );
    }
}
extension Plane {
    func operator-=(const Plane& other) {
        *this = Plane::from_groups(
            /* e423, e431, e412, e321 */
            self.group0() - other.group0()
        );
    }
}
extension Plane: Sub<Point> {
    func operator-(self, other: Point) -> Flector {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            other.group0() * float4(-1.0), 
            /* e423, e431, e412, e321 */
            self.group0()
        );
    }
}
extension Plane: Sub<Scalar> {
    func operator-(self, other: Scalar) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(other[scalar], 1.0) * float2(-1.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            self.group0()
        );
    }
}

impl TryFrom<Flector> for Plane {
    type Error = String;
    fn try_from(flector: Flector) -> Result<Self, Self::Error> {        let mut error_string = String::new();
        let mut fail = false;
        let el = flector[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = flector[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e2: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = flector[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = flector[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from Flector do not fit into Plane { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(Plane::from_groups(
            /* e423, e431, e412, e321 */
            flector.group1()
        ));
    }
}

impl TryFrom<MultiVector> for Plane {
    type Error = String;
    fn try_from(multi_vector: MultiVector) -> Result<Self, Self::Error> {        let mut error_string = String::new();
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
            error_string.push_str("e1234: ");
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
            error_string.push_str("e41: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e42: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[8];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e43: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[9];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e23: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[10];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e31: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[11];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from MultiVector do not fit into Plane { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(Plane::from_groups(
            /* e423, e431, e412, e321 */
            multi_vector.group4()
        ));
    }
}
