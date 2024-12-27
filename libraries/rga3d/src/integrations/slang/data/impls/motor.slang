using traits::GeometricProduct;
using traits::RightDual;
extension Motor: Add<AntiScalar> {
    associatedtype Output = Motor;
    func operator+(other: AntiScalar) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            this.group0 + float4(float3(0.0), other.e1234), 
            /* e23, e31, e12, scalar */
            this.group1
        );
    }
}
extension Motor {
    associatedtype Output = Motor;
    func operator+=(const AntiScalar& other) {
        *this = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            this.group0 + float4(float3(0.0), other.e1234), 
            /* e23, e31, e12, scalar */
            this.group1
        );
    }
}
extension Motor: Add<DualNum> {
    associatedtype Output = Motor;
    func operator+(other: DualNum) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.group0.xyz, other.e1234 + this.e1234), 
            /* e23, e31, e12, scalar */
            float4(this.group1.xyz, other.scalar + this.scalar)
        );
    }
}
extension Motor {
    associatedtype Output = Motor;
    func operator+=(const DualNum& other) {
        *this = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.group0.xyz, other.e1234 + this.e1234), 
            /* e23, e31, e12, scalar */
            float4(this.group1.xyz, other.scalar + this.scalar)
        );
    }
}
extension Motor: Add<Flector> {
    associatedtype Output = MultiVector;
    func operator+(other: Flector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(this.scalar, this.e1234), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            other.group0, 
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group1.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            other.group1
        );
    }
}
extension Motor: Add<Horizon> {
    associatedtype Output = MultiVector;
    func operator+(other: Horizon) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(this.scalar, this.e1234), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group1.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other.e321)
        );
    }
}
extension Motor: Add<Line> {
    associatedtype Output = Motor;
    func operator+(other: Line) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4((other.group0.xyz + this.group0.xyz), this.e1234), 
            /* e23, e31, e12, scalar */
            float4((other.group1.xyz + this.group1.xyz), this.scalar)
        );
    }
}
extension Motor {
    associatedtype Output = Motor;
    func operator+=(const Line& other) {
        *this = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4((other.group0.xyz + this.group0.xyz), this.e1234), 
            /* e23, e31, e12, scalar */
            float4((other.group1.xyz + this.group1.xyz), this.scalar)
        );
    }
}
extension Motor: Add<Motor> {
    associatedtype Output = Motor;
    func operator+(other: Motor) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group0 + this.group0, 
            /* e23, e31, e12, scalar */
            other.group1 + this.group1
        );
    }
}
extension Motor {
    associatedtype Output = Motor;
    func operator+=(const Motor& other) {
        *this = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group0 + this.group0, 
            /* e23, e31, e12, scalar */
            other.group1 + this.group1
        );
    }
}
extension Motor: Add<MultiVector> {
    associatedtype Output = MultiVector;
    func operator+(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(this.scalar, this.e1234) + other.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            other.group1, 
            /* e41, e42, e43, 0 */
            float4(other.group2.xyz + this.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group3.xyz + this.group1.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            other.group4
        );
    }
}
extension Motor: Add<Origin> {
    associatedtype Output = MultiVector;
    func operator+(other: Origin) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(this.scalar, this.e1234), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e4), 
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group1.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
extension Motor: Add<Plane> {
    associatedtype Output = MultiVector;
    func operator+(other: Plane) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(this.scalar, this.e1234), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group1.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            other.group0
        );
    }
}
extension Motor: Add<Point> {
    associatedtype Output = MultiVector;
    func operator+(other: Point) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(this.scalar, this.e1234), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            other.group0, 
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group1.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
extension Motor: Add<Scalar> {
    associatedtype Output = Motor;
    func operator+(other: Scalar) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            this.group0, 
            /* e23, e31, e12, scalar */
            this.group1 + float4(float3(0.0), other.scalar)
        );
    }
}
extension Motor {
    associatedtype Output = Motor;
    func operator+=(const Scalar& other) {
        *this = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            this.group0, 
            /* e23, e31, e12, scalar */
            this.group1 + float4(float3(0.0), other.scalar)
        );
    }
}

impl From<AntiScalar> for Motor {
    fn from(from_anti_scalar: AntiScalar) -> Self {
        use crate::elements::*;
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(float3(0.0), from_anti_scalar.e1234), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}

impl From<DualNum> for Motor {
    fn from(from_dual_num: DualNum) -> Self {
        use crate::elements::*;
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(float3(0.0), from_dual_num.e1234), 
            /* e23, e31, e12, scalar */
            float4(float3(0.0), from_dual_num.scalar)
        );
    }
}

impl From<Line> for Motor {
    fn from(from_line: Line) -> Self {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(from_line.group0.xyz, 0.0), 
            /* e23, e31, e12, scalar */
            float4(from_line.group1.xyz, 0.0)
        );
    }
}

impl From<Scalar> for Motor {
    fn from(from_scalar: Scalar) -> Self {
        use crate::elements::*;
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(0.0), 
            /* e23, e31, e12, scalar */
            float4(float3(0.0), from_scalar.scalar)
        );
    }
}
extension Motor: Mul<AntiScalar> {
    associatedtype Output = Motor;
    func operator*(other: AntiScalar) -> Output {
        return this.geometric_product(other);
    }
}
extension Motor {
    associatedtype Output = Motor;
    func operator*=(const AntiScalar& other) {
        *this = this.geometric_product(other);
    }
}
extension Motor: Mul<DualNum> {
    associatedtype Output = Motor;
    func operator*(other: DualNum) -> Output {
        return this.geometric_product(other);
    }
}
extension Motor {
    associatedtype Output = Motor;
    func operator*=(const DualNum& other) {
        *this = this.geometric_product(other);
    }
}
extension Motor: Mul<Flector> {
    associatedtype Output = Flector;
    func operator*(other: Flector) -> Output {
        return this.geometric_product(other);
    }
}
extension Motor: Mul<Horizon> {
    associatedtype Output = Flector;
    func operator*(other: Horizon) -> Output {
        return this.geometric_product(other);
    }
}
extension Motor: Mul<Line> {
    associatedtype Output = Motor;
    func operator*(other: Line) -> Output {
        return this.geometric_product(other);
    }
}
extension Motor {
    associatedtype Output = Motor;
    func operator*=(const Line& other) {
        *this = this.geometric_product(other);
    }
}
extension Motor: Mul<Motor> {
    associatedtype Output = Motor;
    func operator*(other: Motor) -> Output {
        return this.geometric_product(other);
    }
}
extension Motor {
    associatedtype Output = Motor;
    func operator*=(const Motor& other) {
        *this = this.geometric_product(other);
    }
}
extension Motor: Mul<MultiVector> {
    associatedtype Output = MultiVector;
    func operator*(other: MultiVector) -> Output {
        return this.geometric_product(other);
    }
}
extension Motor: Mul<Origin> {
    associatedtype Output = Flector;
    func operator*(other: Origin) -> Output {
        return this.geometric_product(other);
    }
}
extension Motor: Mul<Plane> {
    associatedtype Output = Flector;
    func operator*(other: Plane) -> Output {
        return this.geometric_product(other);
    }
}
extension Motor: Mul<Point> {
    associatedtype Output = Flector;
    func operator*(other: Point) -> Output {
        return this.geometric_product(other);
    }
}
extension Motor: Mul<Scalar> {
    associatedtype Output = Motor;
    func operator*(other: Scalar) -> Output {
        return this.geometric_product(other);
    }
}
extension Motor {
    associatedtype Output = Motor;
    func operator*=(const Scalar& other) {
        *this = this.geometric_product(other);
    }
}
extension Motor: Neg {
    associatedtype Output = Motor;
    func neg() -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            this.group0 * float4(-1.0), 
            /* e23, e31, e12, scalar */
            this.group1 * float4(-1.0)
        );
    }
}
extension Motor: Not {
    associatedtype Output = Motor;
    func not() -> Output {
        return this.right_dual();
    }
}
extension Motor: Sub<AntiScalar> {
    associatedtype Output = Motor;
    func operator-(other: AntiScalar) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            this.group0 + float4(float3(0.0), other.e1234 * -1.0), 
            /* e23, e31, e12, scalar */
            this.group1
        );
    }
}
extension Motor {
    associatedtype Output = Motor;
    func operator-=(const AntiScalar& other) {
        *this = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            this.group0 + float4(float3(0.0), other.e1234 * -1.0), 
            /* e23, e31, e12, scalar */
            this.group1
        );
    }
}
extension Motor: Sub<DualNum> {
    associatedtype Output = Motor;
    func operator-(other: DualNum) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            this.group0 + float4(float3(0.0), other.e1234 * -1.0), 
            /* e23, e31, e12, scalar */
            this.group1 + float4(float3(0.0), other.scalar * -1.0)
        );
    }
}
extension Motor {
    associatedtype Output = Motor;
    func operator-=(const DualNum& other) {
        *this = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            this.group0 + float4(float3(0.0), other.e1234 * -1.0), 
            /* e23, e31, e12, scalar */
            this.group1 + float4(float3(0.0), other.scalar * -1.0)
        );
    }
}
extension Motor: Sub<Flector> {
    associatedtype Output = MultiVector;
    func operator-(other: Flector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(this.scalar, this.e1234), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            other.group0 * float4(-1.0), 
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group1.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            other.group1 * float4(-1.0)
        );
    }
}
extension Motor: Sub<Horizon> {
    associatedtype Output = MultiVector;
    func operator-(other: Horizon) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(this.scalar, this.e1234), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group1.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other.e321 * -1.0)
        );
    }
}
extension Motor: Sub<Line> {
    associatedtype Output = Motor;
    func operator-(other: Line) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.e41 * -1.0, other.e42 * -1.0, other.e43 * -1.0, 0.0) + this.group0, 
            /* e23, e31, e12, scalar */
            float4(other.e23 * -1.0, other.e31 * -1.0, other.e12 * -1.0, 0.0) + this.group1
        );
    }
}
extension Motor {
    associatedtype Output = Motor;
    func operator-=(const Line& other) {
        *this = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.e41 * -1.0, other.e42 * -1.0, other.e43 * -1.0, 0.0) + this.group0, 
            /* e23, e31, e12, scalar */
            float4(other.e23 * -1.0, other.e31 * -1.0, other.e12 * -1.0, 0.0) + this.group1
        );
    }
}
extension Motor: Sub<Motor> {
    associatedtype Output = Motor;
    func operator-(other: Motor) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            this.group0 - other.group0, 
            /* e23, e31, e12, scalar */
            this.group1 - other.group1
        );
    }
}
extension Motor {
    associatedtype Output = Motor;
    func operator-=(const Motor& other) {
        *this = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            this.group0 - other.group0, 
            /* e23, e31, e12, scalar */
            this.group1 - other.group1
        );
    }
}
extension Motor: Sub<MultiVector> {
    associatedtype Output = MultiVector;
    func operator-(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(this.scalar, this.e1234) - other.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            other.group1 * float4(-1.0), 
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz - other.group2.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group1.xyz - other.group3.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            other.group4 * float4(-1.0)
        );
    }
}
extension Motor: Sub<Origin> {
    associatedtype Output = MultiVector;
    func operator-(other: Origin) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(this.scalar, this.e1234), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e4 * -1.0), 
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group1.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
extension Motor: Sub<Plane> {
    associatedtype Output = MultiVector;
    func operator-(other: Plane) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(this.scalar, this.e1234), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group1.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            other.group0 * float4(-1.0)
        );
    }
}
extension Motor: Sub<Point> {
    associatedtype Output = MultiVector;
    func operator-(other: Point) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(this.scalar, this.e1234), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            other.group0 * float4(-1.0), 
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group1.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
extension Motor: Sub<Scalar> {
    associatedtype Output = Motor;
    func operator-(other: Scalar) -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            this.group0, 
            /* e23, e31, e12, scalar */
            this.group1 + float4(float3(0.0), other.scalar * -1.0)
        );
    }
}
extension Motor {
    associatedtype Output = Motor;
    func operator-=(const Scalar& other) {
        *this = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            this.group0, 
            /* e23, e31, e12, scalar */
            this.group1 + float4(float3(0.0), other.scalar * -1.0)
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
        return Ok(Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(multi_vector.group2.xyz, multi_vector.e1234), 
            /* e23, e31, e12, scalar */
            float4(multi_vector.group3.xyz, multi_vector.scalar)
        ));
    }
}
