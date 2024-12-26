using traits::GeometricProduct;
extension AntiScalar: Add<AntiScalar> {
    func operator+(self, other: AntiScalar) -> AntiScalar {
        return AntiScalar::from_groups(
            /* e1234 */
            other[e1234] + self[e1234]
        );
    }
}
extension AntiScalar {
    func operator+=(const AntiScalar& other) {
        *this = AntiScalar::from_groups(
            /* e1234 */
            other[e1234] + self[e1234]
        );
    }
}
extension AntiScalar: Add<DualNum> {
    func operator+(self, other: DualNum) -> DualNum {
        return DualNum::from_groups(
            /* scalar, e1234 */
            float2(0.0, self[e1234]) + other.group0()
        );
    }
}
extension AntiScalar: Add<Flector> {
    func operator+(self, other: Flector) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(0.0, self[e1234]), 
            /* e1, e2, e3, e4 */
            other.group0(), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            other.group1()
        );
    }
}
extension AntiScalar: Add<Horizon> {
    func operator+(self, other: Horizon) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(0.0, self[e1234]), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other[e321])
        );
    }
}
extension AntiScalar: Add<Line> {
    func operator+(self, other: Line) -> Motor {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.group0(), self[e1234]), 
            /* e23, e31, e12, scalar */
            float4(other.group1(), 0.0)
        );
    }
}
extension AntiScalar: Add<Motor> {
    func operator+(self, other: Motor) -> Motor {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            other.group0() + float4(float3(0.0), self[e1234]), 
            /* e23, e31, e12, scalar */
            other.group1()
        );
    }
}
extension AntiScalar: Add<MultiVector> {
    func operator+(self, other: MultiVector) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(0.0, self[e1234]) + other.group0(), 
            /* e1, e2, e3, e4 */
            other.group1(), 
            /* e41, e42, e43 */
            other.group2(), 
            /* e23, e31, e12 */
            other.group3(), 
            /* e423, e431, e412, e321 */
            other.group4()
        );
    }
}
extension AntiScalar: Add<Origin> {
    func operator+(self, other: Origin) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(0.0, self[e1234]), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other[e4]), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
extension AntiScalar: Add<Plane> {
    func operator+(self, other: Plane) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(0.0, self[e1234]), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            other.group0()
        );
    }
}
extension AntiScalar: Add<Point> {
    func operator+(self, other: Point) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(0.0, self[e1234]), 
            /* e1, e2, e3, e4 */
            other.group0(), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
extension AntiScalar: Add<Scalar> {
    func operator+(self, other: Scalar) -> DualNum {
        return DualNum::from_groups(
            /* scalar, e1234 */
            float2(other[scalar], self[e1234])
        );
    }
}
extension AntiScalar: Mul<DualNum> {
    func operator*(self, other: DualNum) -> AntiScalar {
        return self.geometric_product(other);
    }
}
extension AntiScalar {
    func operator*=(const DualNum& other) {
        *this = self.geometric_product(other);
    }
}
extension AntiScalar: Mul<Flector> {
    func operator*(self, other: Flector) -> Flector {
        return self.geometric_product(other);
    }
}
extension AntiScalar: Mul<Horizon> {
    func operator*(self, other: Horizon) -> Origin {
        return self.geometric_product(other);
    }
}
extension AntiScalar: Mul<Line> {
    func operator*(self, other: Line) -> Line {
        return self.geometric_product(other);
    }
}
extension AntiScalar: Mul<Motor> {
    func operator*(self, other: Motor) -> Motor {
        return self.geometric_product(other);
    }
}
extension AntiScalar: Mul<MultiVector> {
    func operator*(self, other: MultiVector) -> MultiVector {
        return self.geometric_product(other);
    }
}
extension AntiScalar: Mul<Plane> {
    func operator*(self, other: Plane) -> Origin {
        return self.geometric_product(other);
    }
}
extension AntiScalar: Mul<Point> {
    func operator*(self, other: Point) -> Plane {
        return self.geometric_product(other);
    }
}
extension AntiScalar: Mul<Scalar> {
    func operator*(self, other: Scalar) -> AntiScalar {
        return self.geometric_product(other);
    }
}
extension AntiScalar {
    func operator*=(const Scalar& other) {
        *this = self.geometric_product(other);
    }
}
extension AntiScalar: Neg {
    func neg(self) -> AntiScalar {
        return AntiScalar::from_groups(
            /* e1234 */
            self[e1234] * -1.0
        );
    }
}
extension AntiScalar: Sub<AntiScalar> {
    func operator-(self, other: AntiScalar) -> AntiScalar {
        return AntiScalar::from_groups(
            /* e1234 */
            self[e1234] - other[e1234]
        );
    }
}
extension AntiScalar {
    func operator-=(const AntiScalar& other) {
        *this = AntiScalar::from_groups(
            /* e1234 */
            self[e1234] - other[e1234]
        );
    }
}
extension AntiScalar: Sub<DualNum> {
    func operator-(self, other: DualNum) -> DualNum {
        return DualNum::from_groups(
            /* scalar, e1234 */
            float2(other[scalar], self[e1234] - other[e1234]) * float2(-1.0, 1.0)
        );
    }
}
extension AntiScalar: Sub<Flector> {
    func operator-(self, other: Flector) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(0.0, self[e1234]), 
            /* e1, e2, e3, e4 */
            other.group0() * float4(-1.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            other.group1() * float4(-1.0)
        );
    }
}
extension AntiScalar: Sub<Horizon> {
    func operator-(self, other: Horizon) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(0.0, self[e1234]), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other[e321] * -1.0)
        );
    }
}
extension AntiScalar: Sub<Line> {
    func operator-(self, other: Line) -> Motor {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.group0(), self[e1234]) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4((other.group1() * float3(-1.0)), 0.0)
        );
    }
}
extension AntiScalar: Sub<Motor> {
    func operator-(self, other: Motor) -> Motor {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.group0().xyz, self[e1234] - other[e1234]) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            other.group1() * float4(-1.0)
        );
    }
}
extension AntiScalar: Sub<MultiVector> {
    func operator-(self, other: MultiVector) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(other[scalar], self[e1234] - other[e1234]) * float2(-1.0, 1.0), 
            /* e1, e2, e3, e4 */
            other.group1() * float4(-1.0), 
            /* e41, e42, e43 */
            other.group2() * float3(-1.0), 
            /* e23, e31, e12 */
            other.group3() * float3(-1.0), 
            /* e423, e431, e412, e321 */
            other.group4() * float4(-1.0)
        );
    }
}
extension AntiScalar: Sub<Origin> {
    func operator-(self, other: Origin) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(0.0, self[e1234]), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other[e4] * -1.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
extension AntiScalar: Sub<Plane> {
    func operator-(self, other: Plane) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(0.0, self[e1234]), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            other.group0() * float4(-1.0)
        );
    }
}
extension AntiScalar: Sub<Point> {
    func operator-(self, other: Point) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(0.0, self[e1234]), 
            /* e1, e2, e3, e4 */
            other.group0() * float4(-1.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
extension AntiScalar: Sub<Scalar> {
    func operator-(self, other: Scalar) -> DualNum {
        return DualNum::from_groups(
            /* scalar, e1234 */
            float2(other[scalar], self[e1234]) * float2(-1.0, 1.0)
        );
    }
}

impl TryFrom<DualNum> for AntiScalar {
    type Error = String;
    fn try_from(dual_num: DualNum) -> Result<Self, Self::Error> {        use crate::elements::*;
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
            let mut error = "Elements from DualNum do not fit into AntiScalar { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiScalar::from_groups(
            /* e1234 */
            dual_num[e1234]
        ));
    }
}

impl TryFrom<Motor> for AntiScalar {
    type Error = String;
    fn try_from(motor: Motor) -> Result<Self, Self::Error> {        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = motor[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e41: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = motor[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e42: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = motor[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e43: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = motor[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e23: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = motor[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e31: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = motor[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = motor[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("scalar: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from Motor do not fit into AntiScalar { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiScalar::from_groups(
            /* e1234 */
            motor[e1234]
        ));
    }
}

impl TryFrom<MultiVector> for AntiScalar {
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
            let mut error = "Elements from MultiVector do not fit into AntiScalar { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiScalar::from_groups(
            /* e1234 */
            multi_vector[e1234]
        ));
    }
}
