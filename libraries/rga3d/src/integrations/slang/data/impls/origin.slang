using traits::GeometricProduct;
extension Origin: Add<AntiScalar> {
    func operator+(self, other: AntiScalar) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(0.0, other[e1234]), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), self[e4]), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
extension Origin: Add<DualNum> {
    func operator+(self, other: DualNum) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            other.group0(), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), self[e4]), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
extension Origin: Add<Flector> {
    func operator+(self, other: Flector) -> Flector {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            other.group0() + float4(float3(0.0), self[e4]), 
            /* e423, e431, e412, e321 */
            other.group1()
        );
    }
}
extension Origin: Add<Horizon> {
    func operator+(self, other: Horizon) -> Flector {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), self[e4]), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other[e321])
        );
    }
}
extension Origin: Add<Line> {
    func operator+(self, other: Line) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), self[e4]), 
            /* e41, e42, e43 */
            other.group0(), 
            /* e23, e31, e12 */
            other.group1(), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
extension Origin: Add<Motor> {
    func operator+(self, other: Motor) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(other[scalar], other[e1234]), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), self[e4]), 
            /* e41, e42, e43 */
            other.group0().xyz, 
            /* e23, e31, e12 */
            other.group1().xyz, 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
extension Origin: Add<MultiVector> {
    func operator+(self, other: MultiVector) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            other.group0(), 
            /* e1, e2, e3, e4 */
            other.group1() + float4(float3(0.0), self[e4]), 
            /* e41, e42, e43 */
            other.group2(), 
            /* e23, e31, e12 */
            other.group3(), 
            /* e423, e431, e412, e321 */
            other.group4()
        );
    }
}
extension Origin: Add<Origin> {
    func operator+(self, other: Origin) -> Origin {
        return Origin::from_groups(
            /* e4 */
            other[e4] + self[e4]
        );
    }
}
extension Origin {
    func operator+=(const Origin& other) {
        *this = Origin::from_groups(
            /* e4 */
            other[e4] + self[e4]
        );
    }
}
extension Origin: Add<Plane> {
    func operator+(self, other: Plane) -> Flector {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), self[e4]), 
            /* e423, e431, e412, e321 */
            other.group0()
        );
    }
}
extension Origin: Add<Point> {
    func operator+(self, other: Point) -> Point {
        return Point::from_groups(
            /* e1, e2, e3, e4 */
            other.group0() + float4(float3(0.0), self[e4])
        );
    }
}
extension Origin: Add<Scalar> {
    func operator+(self, other: Scalar) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(other[scalar], 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), self[e4]), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
extension Origin: Mul<DualNum> {
    func operator*(self, other: DualNum) -> Origin {
        return self.geometric_product(other);
    }
}
extension Origin {
    func operator*=(const DualNum& other) {
        *this = self.geometric_product(other);
    }
}
extension Origin: Mul<Flector> {
    func operator*(self, other: Flector) -> Motor {
        return self.geometric_product(other);
    }
}
extension Origin: Mul<Horizon> {
    func operator*(self, other: Horizon) -> AntiScalar {
        return self.geometric_product(other);
    }
}
extension Origin: Mul<Line> {
    func operator*(self, other: Line) -> Plane {
        return self.geometric_product(other);
    }
}
extension Origin: Mul<Motor> {
    func operator*(self, other: Motor) -> Flector {
        return self.geometric_product(other);
    }
}
extension Origin: Mul<MultiVector> {
    func operator*(self, other: MultiVector) -> MultiVector {
        return self.geometric_product(other);
    }
}
extension Origin: Mul<Plane> {
    func operator*(self, other: Plane) -> AntiScalar {
        return self.geometric_product(other);
    }
}
extension Origin: Mul<Point> {
    func operator*(self, other: Point) -> Line {
        return self.geometric_product(other);
    }
}
extension Origin: Mul<Scalar> {
    func operator*(self, other: Scalar) -> Origin {
        return self.geometric_product(other);
    }
}
extension Origin {
    func operator*=(const Scalar& other) {
        *this = self.geometric_product(other);
    }
}
extension Origin: Neg {
    func neg(self) -> Origin {
        return Origin::from_groups(
            /* e4 */
            self[e4] * -1.0
        );
    }
}
extension Origin: Sub<AntiScalar> {
    func operator-(self, other: AntiScalar) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(1.0, other[e1234]) * float2(0.0, -1.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), self[e4]), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
extension Origin: Sub<DualNum> {
    func operator-(self, other: DualNum) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            other.group0() * float2(-1.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), self[e4]), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
extension Origin: Sub<Flector> {
    func operator-(self, other: Flector) -> Flector {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(other.group0().xyz, self[e4] - other[e4]) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e423, e431, e412, e321 */
            other.group1() * float4(-1.0)
        );
    }
}
extension Origin: Sub<Horizon> {
    func operator-(self, other: Horizon) -> Flector {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), self[e4]), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other[e321] * -1.0)
        );
    }
}
extension Origin: Sub<Line> {
    func operator-(self, other: Line) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), self[e4]), 
            /* e41, e42, e43 */
            other.group0() * float3(-1.0), 
            /* e23, e31, e12 */
            other.group1() * float3(-1.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
extension Origin: Sub<Motor> {
    func operator-(self, other: Motor) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(other[scalar], other[e1234]) * float2(-1.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), self[e4]), 
            /* e41, e42, e43 */
            other.group0().xyz * float3(-1.0), 
            /* e23, e31, e12 */
            other.group1().xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
extension Origin: Sub<MultiVector> {
    func operator-(self, other: MultiVector) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            other.group0() * float2(-1.0), 
            /* e1, e2, e3, e4 */
            float4(other.group1().xyz, self[e4] - other[e4]) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e41, e42, e43 */
            other.group2() * float3(-1.0), 
            /* e23, e31, e12 */
            other.group3() * float3(-1.0), 
            /* e423, e431, e412, e321 */
            other.group4() * float4(-1.0)
        );
    }
}
extension Origin: Sub<Origin> {
    func operator-(self, other: Origin) -> Origin {
        return Origin::from_groups(
            /* e4 */
            self[e4] - other[e4]
        );
    }
}
extension Origin {
    func operator-=(const Origin& other) {
        *this = Origin::from_groups(
            /* e4 */
            self[e4] - other[e4]
        );
    }
}
extension Origin: Sub<Plane> {
    func operator-(self, other: Plane) -> Flector {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), self[e4]), 
            /* e423, e431, e412, e321 */
            other.group0() * float4(-1.0)
        );
    }
}
extension Origin: Sub<Point> {
    func operator-(self, other: Point) -> Point {
        return Point::from_groups(
            /* e1, e2, e3, e4 */
            float4(other.group0().xyz, self[e4] - other[e4]) * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
extension Origin: Sub<Scalar> {
    func operator-(self, other: Scalar) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(other[scalar], 1.0) * float2(-1.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), self[e4]), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}

impl TryFrom<Flector> for Origin {
    type Error = String;
    fn try_from(flector: Flector) -> Result<Self, Self::Error> {        use crate::elements::*;
        let mut error_string = String::new();
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
        let el = flector[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e423: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = flector[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e431: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = flector[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e412: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = flector[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from Flector do not fit into Origin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(Origin::from_groups(
            /* e4 */
            flector[e4]
        ));
    }
}

impl TryFrom<MultiVector> for Origin {
    type Error = String;
    fn try_from(multi_vector: MultiVector) -> Result<Self, Self::Error> {        use crate::elements::*;
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
        let el = multi_vector[12];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e423: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[13];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e431: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[14];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e412: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[15];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from MultiVector do not fit into Origin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(Origin::from_groups(
            /* e4 */
            multi_vector[e4]
        ));
    }
}

impl TryFrom<Point> for Origin {
    type Error = String;
    fn try_from(point: Point) -> Result<Self, Self::Error> {        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = point[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = point[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e2: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = point[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from Point do not fit into Origin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(Origin::from_groups(
            /* e4 */
            point[e4]
        ));
    }
}
