using traits::GeometricProduct;
using traits::RightDual;
extension Scalar: Add<AntiScalar> {
    associatedtype Output = DualNum;
    func operator+(other: AntiScalar) -> Output {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(this.scalar, other.e1234)
        );
    }
}
extension Scalar: Add<DualNum> {
    associatedtype Output = DualNum;
    func operator+(other: DualNum) -> Output {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(this.scalar, 0.0) + other.group0
        );
    }
}
extension Scalar: Add<Flector> {
    associatedtype Output = MultiVector;
    func operator+(other: Flector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(this.scalar, 0.0), 
            /* e1, e2, e3, e4 */
            other.group0, 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            other.group1
        );
    }
}
extension Scalar: Add<Horizon> {
    associatedtype Output = MultiVector;
    func operator+(other: Horizon) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(this.scalar, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other.e321)
        );
    }
}
extension Scalar: Add<Line> {
    associatedtype Output = Motor;
    func operator+(other: Line) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.group0, 0.0), 
            /* e23, e31, e12, scalar */
            float4(other.group1, this.scalar)
        );
    }
}
extension Scalar: Add<Motor> {
    associatedtype Output = Motor;
    func operator+(other: Motor) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group0, 
            /* e23, e31, e12, scalar */
            other.group1 + float4(float3(0.0), this.scalar)
        );
    }
}
extension Scalar: Add<MultiVector> {
    associatedtype Output = MultiVector;
    func operator+(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(this.scalar, 0.0) + other.group0, 
            /* e1, e2, e3, e4 */
            other.group1, 
            /* e41, e42, e43 */
            other.group2, 
            /* e23, e31, e12 */
            other.group3, 
            /* e423, e431, e412, e321 */
            other.group4
        );
    }
}
extension Scalar: Add<Origin> {
    associatedtype Output = MultiVector;
    func operator+(other: Origin) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(this.scalar, 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e4), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
extension Scalar: Add<Plane> {
    associatedtype Output = MultiVector;
    func operator+(other: Plane) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(this.scalar, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            other.group0
        );
    }
}
extension Scalar: Add<Point> {
    associatedtype Output = MultiVector;
    func operator+(other: Point) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(this.scalar, 0.0), 
            /* e1, e2, e3, e4 */
            other.group0, 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
extension Scalar: Add<Scalar> {
    associatedtype Output = Scalar;
    func operator+(other: Scalar) -> Output {
        return Scalar.from_groups(
            /* scalar */
            other.scalar + this.scalar
        );
    }
}
extension Scalar {
    associatedtype Output = Scalar;
    func operator+=(const Scalar& other) {
        *this = Scalar.from_groups(
            /* scalar */
            other.scalar + this.scalar
        );
    }
}
extension Scalar: Mul<AntiScalar> {
    associatedtype Output = AntiScalar;
    func operator*(other: AntiScalar) -> Output {
        return this.geometric_product(other);
    }
}
extension Scalar: Mul<DualNum> {
    associatedtype Output = DualNum;
    func operator*(other: DualNum) -> Output {
        return this.geometric_product(other);
    }
}
extension Scalar: Mul<Flector> {
    associatedtype Output = Flector;
    func operator*(other: Flector) -> Output {
        return this.geometric_product(other);
    }
}
extension Scalar: Mul<Horizon> {
    associatedtype Output = Horizon;
    func operator*(other: Horizon) -> Output {
        return this.geometric_product(other);
    }
}
extension Scalar: Mul<Line> {
    associatedtype Output = Line;
    func operator*(other: Line) -> Output {
        return this.geometric_product(other);
    }
}
extension Scalar: Mul<Motor> {
    associatedtype Output = Motor;
    func operator*(other: Motor) -> Output {
        return this.geometric_product(other);
    }
}
extension Scalar: Mul<MultiVector> {
    associatedtype Output = MultiVector;
    func operator*(other: MultiVector) -> Output {
        return this.geometric_product(other);
    }
}
extension Scalar: Mul<Origin> {
    associatedtype Output = Origin;
    func operator*(other: Origin) -> Output {
        return this.geometric_product(other);
    }
}
extension Scalar: Mul<Plane> {
    associatedtype Output = Plane;
    func operator*(other: Plane) -> Output {
        return this.geometric_product(other);
    }
}
extension Scalar: Mul<Point> {
    associatedtype Output = Point;
    func operator*(other: Point) -> Output {
        return this.geometric_product(other);
    }
}
extension Scalar: Mul<Scalar> {
    associatedtype Output = Scalar;
    func operator*(other: Scalar) -> Output {
        return this.geometric_product(other);
    }
}
extension Scalar {
    associatedtype Output = Scalar;
    func operator*=(const Scalar& other) {
        *this = this.geometric_product(other);
    }
}
extension Scalar: Neg {
    associatedtype Output = Scalar;
    func neg() -> Output {
        return Scalar.from_groups(
            /* scalar */
            this.scalar * -1.0
        );
    }
}
extension Scalar: Not {
    associatedtype Output = AntiScalar;
    func not() -> Output {
        return this.right_dual();
    }
}
extension Scalar: Sub<AntiScalar> {
    associatedtype Output = DualNum;
    func operator-(other: AntiScalar) -> Output {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(this.scalar, other.e1234) * float2(1.0, -1.0)
        );
    }
}
extension Scalar: Sub<DualNum> {
    associatedtype Output = DualNum;
    func operator-(other: DualNum) -> Output {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(this.scalar - other.scalar, other.e1234) * float2(1.0, -1.0)
        );
    }
}
extension Scalar: Sub<Flector> {
    associatedtype Output = MultiVector;
    func operator-(other: Flector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(this.scalar, 0.0), 
            /* e1, e2, e3, e4 */
            other.group0 * float4(-1.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            other.group1 * float4(-1.0)
        );
    }
}
extension Scalar: Sub<Horizon> {
    associatedtype Output = MultiVector;
    func operator-(other: Horizon) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(this.scalar, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other.e321 * -1.0)
        );
    }
}
extension Scalar: Sub<Line> {
    associatedtype Output = Motor;
    func operator-(other: Line) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4((other.group0 * float3(-1.0)), 0.0), 
            /* e23, e31, e12, scalar */
            float4(other.group1, this.scalar) * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
extension Scalar: Sub<Motor> {
    associatedtype Output = Motor;
    func operator-(other: Motor) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group0 * float4(-1.0), 
            /* e23, e31, e12, scalar */
            float4(other.group1.xyz, this.scalar - other.scalar) * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
extension Scalar: Sub<MultiVector> {
    associatedtype Output = MultiVector;
    func operator-(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(this.scalar - other.scalar, other.e1234) * float2(1.0, -1.0), 
            /* e1, e2, e3, e4 */
            other.group1 * float4(-1.0), 
            /* e41, e42, e43 */
            other.group2 * float3(-1.0), 
            /* e23, e31, e12 */
            other.group3 * float3(-1.0), 
            /* e423, e431, e412, e321 */
            other.group4 * float4(-1.0)
        );
    }
}
extension Scalar: Sub<Origin> {
    associatedtype Output = MultiVector;
    func operator-(other: Origin) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(this.scalar, 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e4 * -1.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
extension Scalar: Sub<Plane> {
    associatedtype Output = MultiVector;
    func operator-(other: Plane) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(this.scalar, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            other.group0 * float4(-1.0)
        );
    }
}
extension Scalar: Sub<Point> {
    associatedtype Output = MultiVector;
    func operator-(other: Point) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(this.scalar, 0.0), 
            /* e1, e2, e3, e4 */
            other.group0 * float4(-1.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
extension Scalar: Sub<Scalar> {
    associatedtype Output = Scalar;
    func operator-(other: Scalar) -> Output {
        return Scalar.from_groups(
            /* scalar */
            this.scalar - other.scalar
        );
    }
}
extension Scalar {
    associatedtype Output = Scalar;
    func operator-=(const Scalar& other) {
        *this = Scalar.from_groups(
            /* scalar */
            this.scalar - other.scalar
        );
    }
}

impl TryFrom<DualNum> for Scalar {
    type Error = String;
    fn try_from(dual_num: DualNum) -> Result<Self, Self::Error> {        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = dual_num[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1234: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from DualNum do not fit into Scalar { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(Scalar.from_groups(
            /* scalar */
            dual_num.scalar
        ));
    }
}

impl TryFrom<Motor> for Scalar {
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
        let el = motor[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1234: ");
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
            let mut error = "Elements from Motor do not fit into Scalar { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(Scalar.from_groups(
            /* scalar */
            motor.scalar
        ));
    }
}

impl TryFrom<MultiVector> for Scalar {
    type Error = String;
    fn try_from(multi_vector: MultiVector) -> Result<Self, Self::Error> {        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
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
            let mut error = "Elements from MultiVector do not fit into Scalar { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(Scalar.from_groups(
            /* scalar */
            multi_vector.scalar
        ));
    }
}
