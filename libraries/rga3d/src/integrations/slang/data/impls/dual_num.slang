using traits::GeometricProduct;
using traits::RightDual;
extension DualNum: Add<AntiScalar> {
    func operator+(self, other: AntiScalar) -> DualNum {
        return DualNum::from_groups(
            /* scalar, e1234 */
            float2(0.0, other[e1234]) + self.group0()
        );
    }
}
extension DualNum {
    func operator+=(const AntiScalar& other) {
        *this = DualNum::from_groups(
            /* scalar, e1234 */
            float2(0.0, other[e1234]) + self.group0()
        );
    }
}
extension DualNum: Add<DualNum> {
    func operator+(self, other: DualNum) -> DualNum {
        return DualNum::from_groups(
            /* scalar, e1234 */
            other.group0() + self.group0()
        );
    }
}
extension DualNum {
    func operator+=(const DualNum& other) {
        *this = DualNum::from_groups(
            /* scalar, e1234 */
            other.group0() + self.group0()
        );
    }
}
extension DualNum: Add<Flector> {
    func operator+(self, other: Flector) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            self.group0(), 
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
extension DualNum: Add<Horizon> {
    func operator+(self, other: Horizon) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            self.group0(), 
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
extension DualNum: Add<Line> {
    func operator+(self, other: Line) -> Motor {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.group0(), self[e1234]), 
            /* e23, e31, e12, scalar */
            float4(other.group1(), self[scalar])
        );
    }
}
extension DualNum: Add<Motor> {
    func operator+(self, other: Motor) -> Motor {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.group0().xyz, self[e1234] + other[e1234]), 
            /* e23, e31, e12, scalar */
            float4(other.group1().xyz, self[scalar] + other[scalar])
        );
    }
}
extension DualNum: Add<MultiVector> {
    func operator+(self, other: MultiVector) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            self.group0() + other.group0(), 
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
extension DualNum: Add<Origin> {
    func operator+(self, other: Origin) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            self.group0(), 
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
extension DualNum: Add<Plane> {
    func operator+(self, other: Plane) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            self.group0(), 
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
extension DualNum: Add<Point> {
    func operator+(self, other: Point) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            self.group0(), 
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
extension DualNum: Add<Scalar> {
    func operator+(self, other: Scalar) -> DualNum {
        return DualNum::from_groups(
            /* scalar, e1234 */
            float2(other[scalar], 0.0) + self.group0()
        );
    }
}
extension DualNum {
    func operator+=(const Scalar& other) {
        *this = DualNum::from_groups(
            /* scalar, e1234 */
            float2(other[scalar], 0.0) + self.group0()
        );
    }
}

impl From<AntiScalar> for DualNum {
    fn from(from_anti_scalar: AntiScalar) -> Self {
        use crate::elements::*;
        return DualNum::from_groups(
            /* scalar, e1234 */
            float2(0.0, from_anti_scalar[e1234])
        );
    }
}

impl From<Scalar> for DualNum {
    fn from(from_scalar: Scalar) -> Self {
        use crate::elements::*;
        return DualNum::from_groups(
            /* scalar, e1234 */
            float2(from_scalar[scalar], 0.0)
        );
    }
}
extension DualNum: Mul<AntiScalar> {
    func operator*(self, other: AntiScalar) -> AntiScalar {
        return self.geometric_product(other);
    }
}
extension DualNum: Mul<DualNum> {
    func operator*(self, other: DualNum) -> DualNum {
        return self.geometric_product(other);
    }
}
extension DualNum {
    func operator*=(const DualNum& other) {
        *this = self.geometric_product(other);
    }
}
extension DualNum: Mul<Flector> {
    func operator*(self, other: Flector) -> Flector {
        return self.geometric_product(other);
    }
}
extension DualNum: Mul<Horizon> {
    func operator*(self, other: Horizon) -> Flector {
        return self.geometric_product(other);
    }
}
extension DualNum: Mul<Line> {
    func operator*(self, other: Line) -> Line {
        return self.geometric_product(other);
    }
}
extension DualNum: Mul<Motor> {
    func operator*(self, other: Motor) -> Motor {
        return self.geometric_product(other);
    }
}
extension DualNum: Mul<MultiVector> {
    func operator*(self, other: MultiVector) -> MultiVector {
        return self.geometric_product(other);
    }
}
extension DualNum: Mul<Origin> {
    func operator*(self, other: Origin) -> Origin {
        return self.geometric_product(other);
    }
}
extension DualNum: Mul<Plane> {
    func operator*(self, other: Plane) -> Flector {
        return self.geometric_product(other);
    }
}
extension DualNum: Mul<Point> {
    func operator*(self, other: Point) -> Flector {
        return self.geometric_product(other);
    }
}
extension DualNum: Mul<Scalar> {
    func operator*(self, other: Scalar) -> DualNum {
        return self.geometric_product(other);
    }
}
extension DualNum {
    func operator*=(const Scalar& other) {
        *this = self.geometric_product(other);
    }
}
extension DualNum: Neg {
    func neg(self) -> DualNum {
        return DualNum::from_groups(
            /* scalar, e1234 */
            self.group0() * float2(-1.0)
        );
    }
}
extension DualNum: Not {
    func not(self) -> AntiScalar {
        return self.right_dual();
    }
}
extension DualNum: Sub<AntiScalar> {
    func operator-(self, other: AntiScalar) -> DualNum {
        return DualNum::from_groups(
            /* scalar, e1234 */
            float2(0.0, other[e1234] * -1.0) + self.group0()
        );
    }
}
extension DualNum {
    func operator-=(const AntiScalar& other) {
        *this = DualNum::from_groups(
            /* scalar, e1234 */
            float2(0.0, other[e1234] * -1.0) + self.group0()
        );
    }
}
extension DualNum: Sub<DualNum> {
    func operator-(self, other: DualNum) -> DualNum {
        return DualNum::from_groups(
            /* scalar, e1234 */
            self.group0() - other.group0()
        );
    }
}
extension DualNum {
    func operator-=(const DualNum& other) {
        *this = DualNum::from_groups(
            /* scalar, e1234 */
            self.group0() - other.group0()
        );
    }
}
extension DualNum: Sub<Flector> {
    func operator-(self, other: Flector) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            self.group0(), 
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
extension DualNum: Sub<Horizon> {
    func operator-(self, other: Horizon) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            self.group0(), 
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
extension DualNum: Sub<Line> {
    func operator-(self, other: Line) -> Motor {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.group0(), self[e1234]) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(other.group1(), self[scalar]) * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
extension DualNum: Sub<Motor> {
    func operator-(self, other: Motor) -> Motor {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.group0().xyz, self[e1234] - other[e1234]) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(other.group1().xyz, self[scalar] - other[scalar]) * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
extension DualNum: Sub<MultiVector> {
    func operator-(self, other: MultiVector) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            self.group0() - other.group0(), 
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
extension DualNum: Sub<Origin> {
    func operator-(self, other: Origin) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            self.group0(), 
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
extension DualNum: Sub<Plane> {
    func operator-(self, other: Plane) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            self.group0(), 
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
extension DualNum: Sub<Point> {
    func operator-(self, other: Point) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            self.group0(), 
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
extension DualNum: Sub<Scalar> {
    func operator-(self, other: Scalar) -> DualNum {
        return DualNum::from_groups(
            /* scalar, e1234 */
            float2(other[scalar] * -1.0, 0.0) + self.group0()
        );
    }
}
extension DualNum {
    func operator-=(const Scalar& other) {
        *this = DualNum::from_groups(
            /* scalar, e1234 */
            float2(other[scalar] * -1.0, 0.0) + self.group0()
        );
    }
}

impl TryFrom<Motor> for DualNum {
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
        if fail {
            let mut error = "Elements from Motor do not fit into DualNum { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DualNum::from_groups(
            /* scalar, e1234 */
            float2(motor[scalar], motor[e1234])
        ));
    }
}

impl TryFrom<MultiVector> for DualNum {
    type Error = String;
    fn try_from(multi_vector: MultiVector) -> Result<Self, Self::Error> {        let mut error_string = String::new();
        let mut fail = false;
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
            let mut error = "Elements from MultiVector do not fit into DualNum { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DualNum::from_groups(
            /* scalar, e1234 */
            multi_vector.group0()
        ));
    }
}
