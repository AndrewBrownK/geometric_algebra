using traits::GeometricProduct;
using traits::RightDual;
extension Line: Add<AntiScalar> {
    func operator+(self, other: AntiScalar) -> Motor {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(self.group0(), other[e1234]), 
            /* e23, e31, e12, scalar */
            float4(self.group1(), 0.0)
        );
    }
}
extension Line: Add<DualNum> {
    func operator+(self, other: DualNum) -> Motor {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(self.group0(), other[e1234]), 
            /* e23, e31, e12, scalar */
            float4(self.group1(), other[scalar])
        );
    }
}
extension Line: Add<Flector> {
    func operator+(self, other: Flector) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            other.group0(), 
            /* e41, e42, e43 */
            self.group0(), 
            /* e23, e31, e12 */
            self.group1(), 
            /* e423, e431, e412, e321 */
            other.group1()
        );
    }
}
extension Line: Add<Horizon> {
    func operator+(self, other: Horizon) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            self.group0(), 
            /* e23, e31, e12 */
            self.group1(), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other[e321])
        );
    }
}
extension Line: Add<Line> {
    func operator+(self, other: Line) -> Line {
        return Line::from_groups(
            /* e41, e42, e43 */
            other.group0() + self.group0(), 
            /* e23, e31, e12 */
            other.group1() + self.group1()
        );
    }
}
extension Line {
    func operator+=(const Line& other) {
        *this = Line::from_groups(
            /* e41, e42, e43 */
            other.group0() + self.group0(), 
            /* e23, e31, e12 */
            other.group1() + self.group1()
        );
    }
}
extension Line: Add<Motor> {
    func operator+(self, other: Motor) -> Motor {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(other[e41], other[e42], other[e43], 0.0) + float4(self.group0(), other[e1234]), 
            /* e23, e31, e12, scalar */
            float4(other[e23], other[e31], other[e12], 0.0) + float4(self.group1(), other[scalar])
        );
    }
}
extension Line: Add<MultiVector> {
    func operator+(self, other: MultiVector) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            other.group0(), 
            /* e1, e2, e3, e4 */
            other.group1(), 
            /* e41, e42, e43 */
            self.group0() + other.group2(), 
            /* e23, e31, e12 */
            self.group1() + other.group3(), 
            /* e423, e431, e412, e321 */
            other.group4()
        );
    }
}
extension Line: Add<Origin> {
    func operator+(self, other: Origin) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other[e4]), 
            /* e41, e42, e43 */
            self.group0(), 
            /* e23, e31, e12 */
            self.group1(), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
extension Line: Add<Plane> {
    func operator+(self, other: Plane) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            self.group0(), 
            /* e23, e31, e12 */
            self.group1(), 
            /* e423, e431, e412, e321 */
            other.group0()
        );
    }
}
extension Line: Add<Point> {
    func operator+(self, other: Point) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            other.group0(), 
            /* e41, e42, e43 */
            self.group0(), 
            /* e23, e31, e12 */
            self.group1(), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
extension Line: Add<Scalar> {
    func operator+(self, other: Scalar) -> Motor {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(self.group0(), 0.0), 
            /* e23, e31, e12, scalar */
            float4(self.group1(), other[scalar])
        );
    }
}
extension Line: Mul<AntiScalar> {
    func operator*(self, other: AntiScalar) -> Line {
        return self.geometric_product(other);
    }
}
extension Line {
    func operator*=(const AntiScalar& other) {
        *this = self.geometric_product(other);
    }
}
extension Line: Mul<DualNum> {
    func operator*(self, other: DualNum) -> Line {
        return self.geometric_product(other);
    }
}
extension Line {
    func operator*=(const DualNum& other) {
        *this = self.geometric_product(other);
    }
}
extension Line: Mul<Flector> {
    func operator*(self, other: Flector) -> Flector {
        return self.geometric_product(other);
    }
}
extension Line: Mul<Horizon> {
    func operator*(self, other: Horizon) -> Flector {
        return self.geometric_product(other);
    }
}
extension Line: Mul<Line> {
    func operator*(self, other: Line) -> Motor {
        return self.geometric_product(other);
    }
}
extension Line: Mul<Motor> {
    func operator*(self, other: Motor) -> Motor {
        return self.geometric_product(other);
    }
}
extension Line: Mul<MultiVector> {
    func operator*(self, other: MultiVector) -> MultiVector {
        return self.geometric_product(other);
    }
}
extension Line: Mul<Origin> {
    func operator*(self, other: Origin) -> Plane {
        return self.geometric_product(other);
    }
}
extension Line: Mul<Plane> {
    func operator*(self, other: Plane) -> Flector {
        return self.geometric_product(other);
    }
}
extension Line: Mul<Point> {
    func operator*(self, other: Point) -> Flector {
        return self.geometric_product(other);
    }
}
extension Line: Mul<Scalar> {
    func operator*(self, other: Scalar) -> Line {
        return self.geometric_product(other);
    }
}
extension Line {
    func operator*=(const Scalar& other) {
        *this = self.geometric_product(other);
    }
}
extension Line: Neg {
    func neg(self) -> Line {
        return Line::from_groups(
            /* e41, e42, e43 */
            self.group0() * float3(-1.0), 
            /* e23, e31, e12 */
            self.group1() * float3(-1.0)
        );
    }
}
extension Line: Not {
    func not(self) -> Line {
        return self.right_dual();
    }
}
extension Line: Sub<AntiScalar> {
    func operator-(self, other: AntiScalar) -> Motor {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(self.group0(), other[e1234] * -1.0), 
            /* e23, e31, e12, scalar */
            float4(self.group1(), 0.0)
        );
    }
}
extension Line: Sub<DualNum> {
    func operator-(self, other: DualNum) -> Motor {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(self.group0(), other[e1234] * -1.0), 
            /* e23, e31, e12, scalar */
            float4(self.group1(), other[scalar] * -1.0)
        );
    }
}
extension Line: Sub<Flector> {
    func operator-(self, other: Flector) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            other.group0() * float4(-1.0), 
            /* e41, e42, e43 */
            self.group0(), 
            /* e23, e31, e12 */
            self.group1(), 
            /* e423, e431, e412, e321 */
            other.group1() * float4(-1.0)
        );
    }
}
extension Line: Sub<Horizon> {
    func operator-(self, other: Horizon) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            self.group0(), 
            /* e23, e31, e12 */
            self.group1(), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other[e321] * -1.0)
        );
    }
}
extension Line: Sub<Line> {
    func operator-(self, other: Line) -> Line {
        return Line::from_groups(
            /* e41, e42, e43 */
            self.group0() - other.group0(), 
            /* e23, e31, e12 */
            self.group1() - other.group1()
        );
    }
}
extension Line {
    func operator-=(const Line& other) {
        *this = Line::from_groups(
            /* e41, e42, e43 */
            self.group0() - other.group0(), 
            /* e23, e31, e12 */
            self.group1() - other.group1()
        );
    }
}
extension Line: Sub<Motor> {
    func operator-(self, other: Motor) -> Motor {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4((self.group0() - other.group0().xyz), other[e1234]) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e23, e31, e12, scalar */
            float4((self.group1() - other.group1().xyz), other[scalar]) * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
extension Line: Sub<MultiVector> {
    func operator-(self, other: MultiVector) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            other.group0() * float2(-1.0), 
            /* e1, e2, e3, e4 */
            other.group1() * float4(-1.0), 
            /* e41, e42, e43 */
            self.group0() - other.group2(), 
            /* e23, e31, e12 */
            self.group1() - other.group3(), 
            /* e423, e431, e412, e321 */
            other.group4() * float4(-1.0)
        );
    }
}
extension Line: Sub<Origin> {
    func operator-(self, other: Origin) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other[e4] * -1.0), 
            /* e41, e42, e43 */
            self.group0(), 
            /* e23, e31, e12 */
            self.group1(), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
extension Line: Sub<Plane> {
    func operator-(self, other: Plane) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            self.group0(), 
            /* e23, e31, e12 */
            self.group1(), 
            /* e423, e431, e412, e321 */
            other.group0() * float4(-1.0)
        );
    }
}
extension Line: Sub<Point> {
    func operator-(self, other: Point) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            other.group0() * float4(-1.0), 
            /* e41, e42, e43 */
            self.group0(), 
            /* e23, e31, e12 */
            self.group1(), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
extension Line: Sub<Scalar> {
    func operator-(self, other: Scalar) -> Motor {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(self.group0(), 0.0), 
            /* e23, e31, e12, scalar */
            float4(self.group1(), other[scalar] * -1.0)
        );
    }
}

impl TryFrom<Motor> for Line {
    type Error = String;
    fn try_from(motor: Motor) -> Result<Self, Self::Error> {        let mut error_string = String::new();
        let mut fail = false;
        let el = motor[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1234: ");
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
            let mut error = "Elements from Motor do not fit into Line { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(Line::from_groups(
            /* e41, e42, e43 */
            motor.group0().xyz, 
            /* e23, e31, e12 */
            motor.group1().xyz
        ));
    }
}

impl TryFrom<MultiVector> for Line {
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
            let mut error = "Elements from MultiVector do not fit into Line { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(Line::from_groups(
            /* e41, e42, e43 */
            multi_vector.group2(), 
            /* e23, e31, e12 */
            multi_vector.group3()
        ));
    }
}
