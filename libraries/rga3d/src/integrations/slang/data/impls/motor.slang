using traits::GeometricProduct;
using traits::RightDual;
extension Motor: Add<AntiScalar> {
    func operator+(self, other: AntiScalar) -> Motor {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            self.group0() + float4(float3(0.0), other[e1234]), 
            /* e23, e31, e12, scalar */
            self.group1()
        );
    }
}
extension Motor {
    func operator+=(const AntiScalar& other) {
        *this = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            self.group0() + float4(float3(0.0), other[e1234]), 
            /* e23, e31, e12, scalar */
            self.group1()
        );
    }
}
extension Motor: Add<DualNum> {
    func operator+(self, other: DualNum) -> Motor {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(self.group0().xyz, other[e1234] + self[e1234]), 
            /* e23, e31, e12, scalar */
            float4(self.group1().xyz, other[scalar] + self[scalar])
        );
    }
}
extension Motor {
    func operator+=(const DualNum& other) {
        *this = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(self.group0().xyz, other[e1234] + self[e1234]), 
            /* e23, e31, e12, scalar */
            float4(self.group1().xyz, other[scalar] + self[scalar])
        );
    }
}
extension Motor: Add<Flector> {
    func operator+(self, other: Flector) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(self[scalar], self[e1234]), 
            /* e1, e2, e3, e4 */
            other.group0(), 
            /* e41, e42, e43 */
            self.group0().xyz, 
            /* e23, e31, e12 */
            self.group1().xyz, 
            /* e423, e431, e412, e321 */
            other.group1()
        );
    }
}
extension Motor: Add<Horizon> {
    func operator+(self, other: Horizon) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(self[scalar], self[e1234]), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            self.group0().xyz, 
            /* e23, e31, e12 */
            self.group1().xyz, 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other[e321])
        );
    }
}
extension Motor: Add<Line> {
    func operator+(self, other: Line) -> Motor {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(self[e41], self[e42], self[e43], 0.0) + float4(other.group0(), self[e1234]), 
            /* e23, e31, e12, scalar */
            float4(self[e23], self[e31], self[e12], 0.0) + float4(other.group1(), self[scalar])
        );
    }
}
extension Motor {
    func operator+=(const Line& other) {
        *this = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(self[e41], self[e42], self[e43], 0.0) + float4(other.group0(), self[e1234]), 
            /* e23, e31, e12, scalar */
            float4(self[e23], self[e31], self[e12], 0.0) + float4(other.group1(), self[scalar])
        );
    }
}
extension Motor: Add<Motor> {
    func operator+(self, other: Motor) -> Motor {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            other.group0() + self.group0(), 
            /* e23, e31, e12, scalar */
            other.group1() + self.group1()
        );
    }
}
extension Motor {
    func operator+=(const Motor& other) {
        *this = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            other.group0() + self.group0(), 
            /* e23, e31, e12, scalar */
            other.group1() + self.group1()
        );
    }
}
extension Motor: Add<MultiVector> {
    func operator+(self, other: MultiVector) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(self[scalar], self[e1234]) + other.group0(), 
            /* e1, e2, e3, e4 */
            other.group1(), 
            /* e41, e42, e43 */
            other.group2() + self.group0().xyz, 
            /* e23, e31, e12 */
            other.group3() + self.group1().xyz, 
            /* e423, e431, e412, e321 */
            other.group4()
        );
    }
}
extension Motor: Add<Origin> {
    func operator+(self, other: Origin) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(self[scalar], self[e1234]), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other[e4]), 
            /* e41, e42, e43 */
            self.group0().xyz, 
            /* e23, e31, e12 */
            self.group1().xyz, 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
extension Motor: Add<Plane> {
    func operator+(self, other: Plane) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(self[scalar], self[e1234]), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            self.group0().xyz, 
            /* e23, e31, e12 */
            self.group1().xyz, 
            /* e423, e431, e412, e321 */
            other.group0()
        );
    }
}
extension Motor: Add<Point> {
    func operator+(self, other: Point) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(self[scalar], self[e1234]), 
            /* e1, e2, e3, e4 */
            other.group0(), 
            /* e41, e42, e43 */
            self.group0().xyz, 
            /* e23, e31, e12 */
            self.group1().xyz, 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
extension Motor: Add<Scalar> {
    func operator+(self, other: Scalar) -> Motor {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            self.group0(), 
            /* e23, e31, e12, scalar */
            self.group1() + float4(float3(0.0), other[scalar])
        );
    }
}
extension Motor {
    func operator+=(const Scalar& other) {
        *this = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            self.group0(), 
            /* e23, e31, e12, scalar */
            self.group1() + float4(float3(0.0), other[scalar])
        );
    }
}

impl From<AntiScalar> for Motor {
    fn from(from_anti_scalar: AntiScalar) -> Self {
        use crate::elements::*;
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(float3(0.0), from_anti_scalar[e1234]), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}

impl From<DualNum> for Motor {
    fn from(from_dual_num: DualNum) -> Self {
        use crate::elements::*;
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(float3(0.0), from_dual_num[e1234]), 
            /* e23, e31, e12, scalar */
            float4(float3(0.0), from_dual_num[scalar])
        );
    }
}

impl From<Line> for Motor {
    fn from(from_line: Line) -> Self {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(from_line.group0(), 0.0), 
            /* e23, e31, e12, scalar */
            float4(from_line.group1(), 0.0)
        );
    }
}

impl From<Scalar> for Motor {
    fn from(from_scalar: Scalar) -> Self {
        use crate::elements::*;
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(0.0), 
            /* e23, e31, e12, scalar */
            float4(float3(0.0), from_scalar[scalar])
        );
    }
}
extension Motor: Mul<AntiScalar> {
    func operator*(self, other: AntiScalar) -> Motor {
        return self.geometric_product(other);
    }
}
extension Motor {
    func operator*=(const AntiScalar& other) {
        *this = self.geometric_product(other);
    }
}
extension Motor: Mul<DualNum> {
    func operator*(self, other: DualNum) -> Motor {
        return self.geometric_product(other);
    }
}
extension Motor {
    func operator*=(const DualNum& other) {
        *this = self.geometric_product(other);
    }
}
extension Motor: Mul<Flector> {
    func operator*(self, other: Flector) -> Flector {
        return self.geometric_product(other);
    }
}
extension Motor: Mul<Horizon> {
    func operator*(self, other: Horizon) -> Flector {
        return self.geometric_product(other);
    }
}
extension Motor: Mul<Line> {
    func operator*(self, other: Line) -> Motor {
        return self.geometric_product(other);
    }
}
extension Motor {
    func operator*=(const Line& other) {
        *this = self.geometric_product(other);
    }
}
extension Motor: Mul<Motor> {
    func operator*(self, other: Motor) -> Motor {
        return self.geometric_product(other);
    }
}
extension Motor {
    func operator*=(const Motor& other) {
        *this = self.geometric_product(other);
    }
}
extension Motor: Mul<MultiVector> {
    func operator*(self, other: MultiVector) -> MultiVector {
        return self.geometric_product(other);
    }
}
extension Motor: Mul<Origin> {
    func operator*(self, other: Origin) -> Flector {
        return self.geometric_product(other);
    }
}
extension Motor: Mul<Plane> {
    func operator*(self, other: Plane) -> Flector {
        return self.geometric_product(other);
    }
}
extension Motor: Mul<Point> {
    func operator*(self, other: Point) -> Flector {
        return self.geometric_product(other);
    }
}
extension Motor: Mul<Scalar> {
    func operator*(self, other: Scalar) -> Motor {
        return self.geometric_product(other);
    }
}
extension Motor {
    func operator*=(const Scalar& other) {
        *this = self.geometric_product(other);
    }
}
extension Motor: Neg {
    func neg(self) -> Motor {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            self.group0() * float4(-1.0), 
            /* e23, e31, e12, scalar */
            self.group1() * float4(-1.0)
        );
    }
}
extension Motor: Not {
    func not(self) -> Motor {
        return self.right_dual();
    }
}
extension Motor: Sub<AntiScalar> {
    func operator-(self, other: AntiScalar) -> Motor {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            self.group0() + float4(float3(0.0), other[e1234] * -1.0), 
            /* e23, e31, e12, scalar */
            self.group1()
        );
    }
}
extension Motor {
    func operator-=(const AntiScalar& other) {
        *this = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            self.group0() + float4(float3(0.0), other[e1234] * -1.0), 
            /* e23, e31, e12, scalar */
            self.group1()
        );
    }
}
extension Motor: Sub<DualNum> {
    func operator-(self, other: DualNum) -> Motor {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            self.group0() + float4(float3(0.0), other[e1234] * -1.0), 
            /* e23, e31, e12, scalar */
            self.group1() + float4(float3(0.0), other[scalar] * -1.0)
        );
    }
}
extension Motor {
    func operator-=(const DualNum& other) {
        *this = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            self.group0() + float4(float3(0.0), other[e1234] * -1.0), 
            /* e23, e31, e12, scalar */
            self.group1() + float4(float3(0.0), other[scalar] * -1.0)
        );
    }
}
extension Motor: Sub<Flector> {
    func operator-(self, other: Flector) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(self[scalar], self[e1234]), 
            /* e1, e2, e3, e4 */
            other.group0() * float4(-1.0), 
            /* e41, e42, e43 */
            self.group0().xyz, 
            /* e23, e31, e12 */
            self.group1().xyz, 
            /* e423, e431, e412, e321 */
            other.group1() * float4(-1.0)
        );
    }
}
extension Motor: Sub<Horizon> {
    func operator-(self, other: Horizon) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(self[scalar], self[e1234]), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            self.group0().xyz, 
            /* e23, e31, e12 */
            self.group1().xyz, 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other[e321] * -1.0)
        );
    }
}
extension Motor: Sub<Line> {
    func operator-(self, other: Line) -> Motor {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(other[e41] * -1.0, other[e42] * -1.0, other[e43] * -1.0, 0.0) + self.group0(), 
            /* e23, e31, e12, scalar */
            float4(other[e23] * -1.0, other[e31] * -1.0, other[e12] * -1.0, 0.0) + self.group1()
        );
    }
}
extension Motor {
    func operator-=(const Line& other) {
        *this = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(other[e41] * -1.0, other[e42] * -1.0, other[e43] * -1.0, 0.0) + self.group0(), 
            /* e23, e31, e12, scalar */
            float4(other[e23] * -1.0, other[e31] * -1.0, other[e12] * -1.0, 0.0) + self.group1()
        );
    }
}
extension Motor: Sub<Motor> {
    func operator-(self, other: Motor) -> Motor {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            self.group0() - other.group0(), 
            /* e23, e31, e12, scalar */
            self.group1() - other.group1()
        );
    }
}
extension Motor {
    func operator-=(const Motor& other) {
        *this = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            self.group0() - other.group0(), 
            /* e23, e31, e12, scalar */
            self.group1() - other.group1()
        );
    }
}
extension Motor: Sub<MultiVector> {
    func operator-(self, other: MultiVector) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(self[scalar], self[e1234]) - other.group0(), 
            /* e1, e2, e3, e4 */
            other.group1() * float4(-1.0), 
            /* e41, e42, e43 */
            self.group0().xyz - other.group2(), 
            /* e23, e31, e12 */
            self.group1().xyz - other.group3(), 
            /* e423, e431, e412, e321 */
            other.group4() * float4(-1.0)
        );
    }
}
extension Motor: Sub<Origin> {
    func operator-(self, other: Origin) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(self[scalar], self[e1234]), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other[e4] * -1.0), 
            /* e41, e42, e43 */
            self.group0().xyz, 
            /* e23, e31, e12 */
            self.group1().xyz, 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
extension Motor: Sub<Plane> {
    func operator-(self, other: Plane) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(self[scalar], self[e1234]), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            self.group0().xyz, 
            /* e23, e31, e12 */
            self.group1().xyz, 
            /* e423, e431, e412, e321 */
            other.group0() * float4(-1.0)
        );
    }
}
extension Motor: Sub<Point> {
    func operator-(self, other: Point) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(self[scalar], self[e1234]), 
            /* e1, e2, e3, e4 */
            other.group0() * float4(-1.0), 
            /* e41, e42, e43 */
            self.group0().xyz, 
            /* e23, e31, e12 */
            self.group1().xyz, 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
extension Motor: Sub<Scalar> {
    func operator-(self, other: Scalar) -> Motor {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            self.group0(), 
            /* e23, e31, e12, scalar */
            self.group1() + float4(float3(0.0), other[scalar] * -1.0)
        );
    }
}
extension Motor {
    func operator-=(const Scalar& other) {
        *this = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            self.group0(), 
            /* e23, e31, e12, scalar */
            self.group1() + float4(float3(0.0), other[scalar] * -1.0)
        );
    }
}

impl TryFrom<MultiVector> for Motor {
    type Error = String;
    fn try_from(multi_vector: MultiVector) -> Result<Self, Self::Error> {        use crate::elements::*;
        let mut error_string = String::new();
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
            let mut error = "Elements from MultiVector do not fit into Motor { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(multi_vector.group2(), multi_vector[e1234]), 
            /* e23, e31, e12, scalar */
            float4(multi_vector.group3(), multi_vector[scalar])
        ));
    }
}
