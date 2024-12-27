using traits::GeometricProduct;
extension AntiScalar: Add<AntiScalar> {
    associatedtype Output = AntiScalar;
    func operator+(other: AntiScalar) -> Output {
        return AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(other.e1234 + this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiScalar {
    associatedtype Output = AntiScalar;
    func operator+=(const AntiScalar& other) {
        *this = AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(other.e1234 + this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiScalar: Add<DualNum> {
    associatedtype Output = DualNum;
    func operator+(other: DualNum) -> Output {
        return DualNum.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(0.0, this.e1234) + other.group0.xy, 0.0, 0.0)
        );
    }
}
extension AntiScalar: Add<Flector> {
    associatedtype Output = MultiVector;
    func operator+(other: Flector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(0.0, this.e1234), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            other.group0, 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            other.group1
        );
    }
}
extension AntiScalar: Add<Horizon> {
    associatedtype Output = MultiVector;
    func operator+(other: Horizon) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(0.0, this.e1234), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other.e321)
        );
    }
}
extension AntiScalar: Add<Line> {
    associatedtype Output = Motor;
    func operator+(other: Line) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.group0.xyz, this.e1234), 
            /* e23, e31, e12, scalar */
            float4(other.group1.xyz, 0.0)
        );
    }
}
extension AntiScalar: Add<Motor> {
    associatedtype Output = Motor;
    func operator+(other: Motor) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group0 + float4(float3(0.0), this.e1234), 
            /* e23, e31, e12, scalar */
            other.group1
        );
    }
}
extension AntiScalar: Add<MultiVector> {
    associatedtype Output = MultiVector;
    func operator+(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(0.0, this.e1234) + other.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            other.group1, 
            /* e41, e42, e43, 0 */
            float4(other.group2.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group3.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            other.group4
        );
    }
}
extension AntiScalar: Add<Origin> {
    associatedtype Output = MultiVector;
    func operator+(other: Origin) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(0.0, this.e1234), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e4), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
extension AntiScalar: Add<Plane> {
    associatedtype Output = MultiVector;
    func operator+(other: Plane) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(0.0, this.e1234), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            other.group0
        );
    }
}
extension AntiScalar: Add<Point> {
    associatedtype Output = MultiVector;
    func operator+(other: Point) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(0.0, this.e1234), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            other.group0, 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
extension AntiScalar: Add<Scalar> {
    associatedtype Output = DualNum;
    func operator+(other: Scalar) -> Output {
        return DualNum.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(other.scalar, this.e1234), 0.0, 0.0)
        );
    }
}
extension AntiScalar: Mul<DualNum> {
    associatedtype Output = AntiScalar;
    func operator*(other: DualNum) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiScalar {
    associatedtype Output = AntiScalar;
    func operator*=(const DualNum& other) {
        *this = this.geometric_product(other);
    }
}
extension AntiScalar: Mul<Flector> {
    associatedtype Output = Flector;
    func operator*(other: Flector) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiScalar: Mul<Horizon> {
    associatedtype Output = Origin;
    func operator*(other: Horizon) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiScalar: Mul<Line> {
    associatedtype Output = Line;
    func operator*(other: Line) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiScalar: Mul<Motor> {
    associatedtype Output = Motor;
    func operator*(other: Motor) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiScalar: Mul<MultiVector> {
    associatedtype Output = MultiVector;
    func operator*(other: MultiVector) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiScalar: Mul<Plane> {
    associatedtype Output = Origin;
    func operator*(other: Plane) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiScalar: Mul<Point> {
    associatedtype Output = Plane;
    func operator*(other: Point) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiScalar: Mul<Scalar> {
    associatedtype Output = AntiScalar;
    func operator*(other: Scalar) -> Output {
        return this.geometric_product(other);
    }
}
extension AntiScalar {
    associatedtype Output = AntiScalar;
    func operator*=(const Scalar& other) {
        *this = this.geometric_product(other);
    }
}
extension AntiScalar: Neg {
    associatedtype Output = AntiScalar;
    func neg() -> Output {
        return AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(this.e1234 * -1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiScalar: Sub<AntiScalar> {
    associatedtype Output = AntiScalar;
    func operator-(other: AntiScalar) -> Output {
        return AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(this.e1234 - other.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiScalar {
    associatedtype Output = AntiScalar;
    func operator-=(const AntiScalar& other) {
        *this = AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(this.e1234 - other.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiScalar: Sub<DualNum> {
    associatedtype Output = DualNum;
    func operator-(other: DualNum) -> Output {
        return DualNum.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(other.scalar, this.e1234 - other.e1234) * float2(-1.0, 1.0), 0.0, 0.0)
        );
    }
}
extension AntiScalar: Sub<Flector> {
    associatedtype Output = MultiVector;
    func operator-(other: Flector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(0.0, this.e1234), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            other.group0 * float4(-1.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            other.group1 * float4(-1.0)
        );
    }
}
extension AntiScalar: Sub<Horizon> {
    associatedtype Output = MultiVector;
    func operator-(other: Horizon) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(0.0, this.e1234), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other.e321 * -1.0)
        );
    }
}
extension AntiScalar: Sub<Line> {
    associatedtype Output = Motor;
    func operator-(other: Line) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.group0.xyz, this.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4((other.group1.xyz * float3(-1.0)), 0.0)
        );
    }
}
extension AntiScalar: Sub<Motor> {
    associatedtype Output = Motor;
    func operator-(other: Motor) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.group0.xyz, this.e1234 - other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            other.group1 * float4(-1.0)
        );
    }
}
extension AntiScalar: Sub<MultiVector> {
    associatedtype Output = MultiVector;
    func operator-(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(other.scalar, this.e1234 - other.e1234) * float2(-1.0, 1.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            other.group1 * float4(-1.0), 
            /* e41, e42, e43, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group3.xyz * float3(-1.0), 0.0), 
            /* e423, e431, e412, e321 */
            other.group4 * float4(-1.0)
        );
    }
}
extension AntiScalar: Sub<Origin> {
    associatedtype Output = MultiVector;
    func operator-(other: Origin) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(0.0, this.e1234), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e4 * -1.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
extension AntiScalar: Sub<Plane> {
    associatedtype Output = MultiVector;
    func operator-(other: Plane) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(0.0, this.e1234), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            other.group0 * float4(-1.0)
        );
    }
}
extension AntiScalar: Sub<Point> {
    associatedtype Output = MultiVector;
    func operator-(other: Point) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(0.0, this.e1234), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            other.group0 * float4(-1.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
extension AntiScalar: Sub<Scalar> {
    associatedtype Output = DualNum;
    func operator-(other: Scalar) -> Output {
        return DualNum.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(other.scalar, this.e1234) * float2(-1.0, 1.0), 0.0, 0.0)
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
        return Ok(AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(dual_num.e1234, 0.0, 0.0, 0.0)
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
        return Ok(AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(motor.e1234, 0.0, 0.0, 0.0)
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
        return Ok(AntiScalar.from_groups(
            /* e1234, 0, 0, 0 */
            float4(multi_vector.e1234, 0.0, 0.0, 0.0)
        ));
    }
}
