using traits::GeometricProduct;
using traits::RightDual;
extension Line: Add<AntiScalar> {
    func operator+(other: AntiScalar) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.group0, other.e1234), 
            /* e23, e31, e12, scalar */
            float4(this.group1, 0.0)
        );
    }
}
extension Line: Add<DualNum> {
    func operator+(other: DualNum) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.group0, other.e1234), 
            /* e23, e31, e12, scalar */
            float4(this.group1, other.scalar)
        );
    }
}
extension Line: Add<Flector> {
    func operator+(other: Flector) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            other.group0, 
            /* e41, e42, e43 */
            this.group0, 
            /* e23, e31, e12 */
            this.group1, 
            /* e423, e431, e412, e321 */
            other.group1
        );
    }
}
extension Line: Add<Horizon> {
    func operator+(other: Horizon) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            this.group0, 
            /* e23, e31, e12 */
            this.group1, 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other.e321)
        );
    }
}
extension Line: Add<Line> {
    func operator+(other: Line) -> Line {
        return Line.from_groups(
            /* e41, e42, e43 */
            other.group0 + this.group0, 
            /* e23, e31, e12 */
            other.group1 + this.group1
        );
    }
}
extension Line {
    func operator+=(const Line& other) {
        *this = Line.from_groups(
            /* e41, e42, e43 */
            other.group0 + this.group0, 
            /* e23, e31, e12 */
            other.group1 + this.group1
        );
    }
}
extension Line: Add<Motor> {
    func operator+(other: Motor) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.e41, other.e42, other.e43, 0.0) + float4(this.group0, other.e1234), 
            /* e23, e31, e12, scalar */
            float4(other.e23, other.e31, other.e12, 0.0) + float4(this.group1, other.scalar)
        );
    }
}
extension Line: Add<MultiVector> {
    func operator+(other: MultiVector) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            other.group0, 
            /* e1, e2, e3, e4 */
            other.group1, 
            /* e41, e42, e43 */
            this.group0 + other.group2, 
            /* e23, e31, e12 */
            this.group1 + other.group3, 
            /* e423, e431, e412, e321 */
            other.group4
        );
    }
}
extension Line: Add<Origin> {
    func operator+(other: Origin) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e4), 
            /* e41, e42, e43 */
            this.group0, 
            /* e23, e31, e12 */
            this.group1, 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
extension Line: Add<Plane> {
    func operator+(other: Plane) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            this.group0, 
            /* e23, e31, e12 */
            this.group1, 
            /* e423, e431, e412, e321 */
            other.group0
        );
    }
}
extension Line: Add<Point> {
    func operator+(other: Point) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            other.group0, 
            /* e41, e42, e43 */
            this.group0, 
            /* e23, e31, e12 */
            this.group1, 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
extension Line: Add<Scalar> {
    func operator+(other: Scalar) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.group0, 0.0), 
            /* e23, e31, e12, scalar */
            float4(this.group1, other.scalar)
        );
    }
}
extension Line: Mul<AntiScalar> {
    func operator*(other: AntiScalar) -> Line {
        return this.geometric_product(other);
    }
}
extension Line {
    func operator*=(const AntiScalar& other) {
        *this = this.geometric_product(other);
    }
}
extension Line: Mul<DualNum> {
    func operator*(other: DualNum) -> Line {
        return this.geometric_product(other);
    }
}
extension Line {
    func operator*=(const DualNum& other) {
        *this = this.geometric_product(other);
    }
}
extension Line: Mul<Flector> {
    func operator*(other: Flector) -> Flector {
        return this.geometric_product(other);
    }
}
extension Line: Mul<Horizon> {
    func operator*(other: Horizon) -> Flector {
        return this.geometric_product(other);
    }
}
extension Line: Mul<Line> {
    func operator*(other: Line) -> Motor {
        return this.geometric_product(other);
    }
}
extension Line: Mul<Motor> {
    func operator*(other: Motor) -> Motor {
        return this.geometric_product(other);
    }
}
extension Line: Mul<MultiVector> {
    func operator*(other: MultiVector) -> MultiVector {
        return this.geometric_product(other);
    }
}
extension Line: Mul<Origin> {
    func operator*(other: Origin) -> Plane {
        return this.geometric_product(other);
    }
}
extension Line: Mul<Plane> {
    func operator*(other: Plane) -> Flector {
        return this.geometric_product(other);
    }
}
extension Line: Mul<Point> {
    func operator*(other: Point) -> Flector {
        return this.geometric_product(other);
    }
}
extension Line: Mul<Scalar> {
    func operator*(other: Scalar) -> Line {
        return this.geometric_product(other);
    }
}
extension Line {
    func operator*=(const Scalar& other) {
        *this = this.geometric_product(other);
    }
}
extension Line: Neg {
    func neg() -> Line {
        return Line.from_groups(
            /* e41, e42, e43 */
            this.group0 * float3(-1.0), 
            /* e23, e31, e12 */
            this.group1 * float3(-1.0)
        );
    }
}
extension Line: Not {
    func not() -> Line {
        return this.right_dual();
    }
}
extension Line: Sub<AntiScalar> {
    func operator-(other: AntiScalar) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.group0, other.e1234 * -1.0), 
            /* e23, e31, e12, scalar */
            float4(this.group1, 0.0)
        );
    }
}
extension Line: Sub<DualNum> {
    func operator-(other: DualNum) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.group0, other.e1234 * -1.0), 
            /* e23, e31, e12, scalar */
            float4(this.group1, other.scalar * -1.0)
        );
    }
}
extension Line: Sub<Flector> {
    func operator-(other: Flector) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            other.group0 * float4(-1.0), 
            /* e41, e42, e43 */
            this.group0, 
            /* e23, e31, e12 */
            this.group1, 
            /* e423, e431, e412, e321 */
            other.group1 * float4(-1.0)
        );
    }
}
extension Line: Sub<Horizon> {
    func operator-(other: Horizon) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            this.group0, 
            /* e23, e31, e12 */
            this.group1, 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other.e321 * -1.0)
        );
    }
}
extension Line: Sub<Line> {
    func operator-(other: Line) -> Line {
        return Line.from_groups(
            /* e41, e42, e43 */
            this.group0 - other.group0, 
            /* e23, e31, e12 */
            this.group1 - other.group1
        );
    }
}
extension Line {
    func operator-=(const Line& other) {
        *this = Line.from_groups(
            /* e41, e42, e43 */
            this.group0 - other.group0, 
            /* e23, e31, e12 */
            this.group1 - other.group1
        );
    }
}
extension Line: Sub<Motor> {
    func operator-(other: Motor) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4((this.group0 - other.group0.xyz), other.e1234) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e23, e31, e12, scalar */
            float4((this.group1 - other.group1.xyz), other.scalar) * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
extension Line: Sub<MultiVector> {
    func operator-(other: MultiVector) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            other.group0 * float2(-1.0), 
            /* e1, e2, e3, e4 */
            other.group1 * float4(-1.0), 
            /* e41, e42, e43 */
            this.group0 - other.group2, 
            /* e23, e31, e12 */
            this.group1 - other.group3, 
            /* e423, e431, e412, e321 */
            other.group4 * float4(-1.0)
        );
    }
}
extension Line: Sub<Origin> {
    func operator-(other: Origin) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e4 * -1.0), 
            /* e41, e42, e43 */
            this.group0, 
            /* e23, e31, e12 */
            this.group1, 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
extension Line: Sub<Plane> {
    func operator-(other: Plane) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            this.group0, 
            /* e23, e31, e12 */
            this.group1, 
            /* e423, e431, e412, e321 */
            other.group0 * float4(-1.0)
        );
    }
}
extension Line: Sub<Point> {
    func operator-(other: Point) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            other.group0 * float4(-1.0), 
            /* e41, e42, e43 */
            this.group0, 
            /* e23, e31, e12 */
            this.group1, 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
extension Line: Sub<Scalar> {
    func operator-(other: Scalar) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.group0, 0.0), 
            /* e23, e31, e12, scalar */
            float4(this.group1, other.scalar * -1.0)
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
        return Ok(Line.from_groups(
            /* e41, e42, e43 */
            motor.group0.xyz, 
            /* e23, e31, e12 */
            motor.group1.xyz
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
        return Ok(Line.from_groups(
            /* e41, e42, e43 */
            multi_vector.group2, 
            /* e23, e31, e12 */
            multi_vector.group3
        ));
    }
}
