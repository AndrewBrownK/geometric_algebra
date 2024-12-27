using traits::GeometricProduct;
using traits::RightDual;
extension Flector: Add<AntiScalar> {
    associatedtype Output = MultiVector;
    func operator+(other: AntiScalar) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, other.e1234), 
            /* e1, e2, e3, e4 */
            this.group0, 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            this.group1
        );
    }
}
extension Flector: Add<DualNum> {
    associatedtype Output = MultiVector;
    func operator+(other: DualNum) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            other.group0, 
            /* e1, e2, e3, e4 */
            this.group0, 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            this.group1
        );
    }
}
extension Flector: Add<Flector> {
    associatedtype Output = Flector;
    func operator+(other: Flector) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            other.group0 + this.group0, 
            /* e423, e431, e412, e321 */
            other.group1 + this.group1
        );
    }
}
extension Flector {
    associatedtype Output = Flector;
    func operator+=(const Flector& other) {
        *this = Flector.from_groups(
            /* e1, e2, e3, e4 */
            other.group0 + this.group0, 
            /* e423, e431, e412, e321 */
            other.group1 + this.group1
        );
    }
}
extension Flector: Add<Horizon> {
    associatedtype Output = Flector;
    func operator+(other: Horizon) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            this.group0, 
            /* e423, e431, e412, e321 */
            this.group1 + float4(float3(0.0), other.e321)
        );
    }
}
extension Flector {
    associatedtype Output = Flector;
    func operator+=(const Horizon& other) {
        *this = Flector.from_groups(
            /* e1, e2, e3, e4 */
            this.group0, 
            /* e423, e431, e412, e321 */
            this.group1 + float4(float3(0.0), other.e321)
        );
    }
}
extension Flector: Add<Line> {
    associatedtype Output = MultiVector;
    func operator+(other: Line) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            this.group0, 
            /* e41, e42, e43 */
            other.group0, 
            /* e23, e31, e12 */
            other.group1, 
            /* e423, e431, e412, e321 */
            this.group1
        );
    }
}
extension Flector: Add<Motor> {
    associatedtype Output = MultiVector;
    func operator+(other: Motor) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other.scalar, other.e1234), 
            /* e1, e2, e3, e4 */
            this.group0, 
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12 */
            other.group1.xyz, 
            /* e423, e431, e412, e321 */
            this.group1
        );
    }
}
extension Flector: Add<MultiVector> {
    associatedtype Output = MultiVector;
    func operator+(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            other.group0, 
            /* e1, e2, e3, e4 */
            this.group0 + other.group1, 
            /* e41, e42, e43 */
            other.group2, 
            /* e23, e31, e12 */
            other.group3, 
            /* e423, e431, e412, e321 */
            this.group1 + other.group4
        );
    }
}
extension Flector: Add<Origin> {
    associatedtype Output = Flector;
    func operator+(other: Origin) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            this.group0 + float4(float3(0.0), other.e4), 
            /* e423, e431, e412, e321 */
            this.group1
        );
    }
}
extension Flector {
    associatedtype Output = Flector;
    func operator+=(const Origin& other) {
        *this = Flector.from_groups(
            /* e1, e2, e3, e4 */
            this.group0 + float4(float3(0.0), other.e4), 
            /* e423, e431, e412, e321 */
            this.group1
        );
    }
}
extension Flector: Add<Plane> {
    associatedtype Output = Flector;
    func operator+(other: Plane) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            this.group0, 
            /* e423, e431, e412, e321 */
            this.group1 + other.group0
        );
    }
}
extension Flector {
    associatedtype Output = Flector;
    func operator+=(const Plane& other) {
        *this = Flector.from_groups(
            /* e1, e2, e3, e4 */
            this.group0, 
            /* e423, e431, e412, e321 */
            this.group1 + other.group0
        );
    }
}
extension Flector: Add<Point> {
    associatedtype Output = Flector;
    func operator+(other: Point) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            this.group0 + other.group0, 
            /* e423, e431, e412, e321 */
            this.group1
        );
    }
}
extension Flector {
    associatedtype Output = Flector;
    func operator+=(const Point& other) {
        *this = Flector.from_groups(
            /* e1, e2, e3, e4 */
            this.group0 + other.group0, 
            /* e423, e431, e412, e321 */
            this.group1
        );
    }
}
extension Flector: Add<Scalar> {
    associatedtype Output = MultiVector;
    func operator+(other: Scalar) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other.scalar, 0.0), 
            /* e1, e2, e3, e4 */
            this.group0, 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            this.group1
        );
    }
}

impl From<Horizon> for Flector {
    fn from(from_horizon: Horizon) -> Self {
        use crate::elements::*;
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), from_horizon.e321)
        );
    }
}

impl From<Origin> for Flector {
    fn from(from_origin: Origin) -> Self {
        use crate::elements::*;
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), from_origin.e4), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}

impl From<Plane> for Flector {
    fn from(from_plane: Plane) -> Self {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            from_plane.group0
        );
    }
}

impl From<Point> for Flector {
    fn from(from_point: Point) -> Self {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            from_point.group0, 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
extension Flector: Mul<AntiScalar> {
    associatedtype Output = Flector;
    func operator*(other: AntiScalar) -> Output {
        return this.geometric_product(other);
    }
}
extension Flector {
    associatedtype Output = Flector;
    func operator*=(const AntiScalar& other) {
        *this = this.geometric_product(other);
    }
}
extension Flector: Mul<DualNum> {
    associatedtype Output = Flector;
    func operator*(other: DualNum) -> Output {
        return this.geometric_product(other);
    }
}
extension Flector {
    associatedtype Output = Flector;
    func operator*=(const DualNum& other) {
        *this = this.geometric_product(other);
    }
}
extension Flector: Mul<Flector> {
    associatedtype Output = Motor;
    func operator*(other: Flector) -> Output {
        return this.geometric_product(other);
    }
}
extension Flector: Mul<Horizon> {
    associatedtype Output = Motor;
    func operator*(other: Horizon) -> Output {
        return this.geometric_product(other);
    }
}
extension Flector: Mul<Line> {
    associatedtype Output = Flector;
    func operator*(other: Line) -> Output {
        return this.geometric_product(other);
    }
}
extension Flector {
    associatedtype Output = Flector;
    func operator*=(const Line& other) {
        *this = this.geometric_product(other);
    }
}
extension Flector: Mul<Motor> {
    associatedtype Output = Flector;
    func operator*(other: Motor) -> Output {
        return this.geometric_product(other);
    }
}
extension Flector {
    associatedtype Output = Flector;
    func operator*=(const Motor& other) {
        *this = this.geometric_product(other);
    }
}
extension Flector: Mul<MultiVector> {
    associatedtype Output = MultiVector;
    func operator*(other: MultiVector) -> Output {
        return this.geometric_product(other);
    }
}
extension Flector: Mul<Origin> {
    associatedtype Output = Motor;
    func operator*(other: Origin) -> Output {
        return this.geometric_product(other);
    }
}
extension Flector: Mul<Plane> {
    associatedtype Output = Motor;
    func operator*(other: Plane) -> Output {
        return this.geometric_product(other);
    }
}
extension Flector: Mul<Point> {
    associatedtype Output = Motor;
    func operator*(other: Point) -> Output {
        return this.geometric_product(other);
    }
}
extension Flector: Mul<Scalar> {
    associatedtype Output = Flector;
    func operator*(other: Scalar) -> Output {
        return this.geometric_product(other);
    }
}
extension Flector {
    associatedtype Output = Flector;
    func operator*=(const Scalar& other) {
        *this = this.geometric_product(other);
    }
}
extension Flector: Neg {
    associatedtype Output = Flector;
    func neg() -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            this.group0 * float4(-1.0), 
            /* e423, e431, e412, e321 */
            this.group1 * float4(-1.0)
        );
    }
}
extension Flector: Not {
    associatedtype Output = Flector;
    func not() -> Output {
        return this.right_dual();
    }
}
extension Flector: Sub<AntiScalar> {
    associatedtype Output = MultiVector;
    func operator-(other: AntiScalar) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(1.0, other.e1234) * float2(0.0, -1.0), 
            /* e1, e2, e3, e4 */
            this.group0, 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            this.group1
        );
    }
}
extension Flector: Sub<DualNum> {
    associatedtype Output = MultiVector;
    func operator-(other: DualNum) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            other.group0 * float2(-1.0), 
            /* e1, e2, e3, e4 */
            this.group0, 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            this.group1
        );
    }
}
extension Flector: Sub<Flector> {
    associatedtype Output = Flector;
    func operator-(other: Flector) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            this.group0 - other.group0, 
            /* e423, e431, e412, e321 */
            this.group1 - other.group1
        );
    }
}
extension Flector {
    associatedtype Output = Flector;
    func operator-=(const Flector& other) {
        *this = Flector.from_groups(
            /* e1, e2, e3, e4 */
            this.group0 - other.group0, 
            /* e423, e431, e412, e321 */
            this.group1 - other.group1
        );
    }
}
extension Flector: Sub<Horizon> {
    associatedtype Output = Flector;
    func operator-(other: Horizon) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            this.group0, 
            /* e423, e431, e412, e321 */
            this.group1 + float4(float3(0.0), other.e321 * -1.0)
        );
    }
}
extension Flector {
    associatedtype Output = Flector;
    func operator-=(const Horizon& other) {
        *this = Flector.from_groups(
            /* e1, e2, e3, e4 */
            this.group0, 
            /* e423, e431, e412, e321 */
            this.group1 + float4(float3(0.0), other.e321 * -1.0)
        );
    }
}
extension Flector: Sub<Line> {
    associatedtype Output = MultiVector;
    func operator-(other: Line) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            this.group0, 
            /* e41, e42, e43 */
            other.group0 * float3(-1.0), 
            /* e23, e31, e12 */
            other.group1 * float3(-1.0), 
            /* e423, e431, e412, e321 */
            this.group1
        );
    }
}
extension Flector: Sub<Motor> {
    associatedtype Output = MultiVector;
    func operator-(other: Motor) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other.scalar, other.e1234) * float2(-1.0), 
            /* e1, e2, e3, e4 */
            this.group0, 
            /* e41, e42, e43 */
            other.group0.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            other.group1.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            this.group1
        );
    }
}
extension Flector: Sub<MultiVector> {
    associatedtype Output = MultiVector;
    func operator-(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            other.group0 * float2(-1.0), 
            /* e1, e2, e3, e4 */
            this.group0 - other.group1, 
            /* e41, e42, e43 */
            other.group2 * float3(-1.0), 
            /* e23, e31, e12 */
            other.group3 * float3(-1.0), 
            /* e423, e431, e412, e321 */
            this.group1 - other.group4
        );
    }
}
extension Flector: Sub<Origin> {
    associatedtype Output = Flector;
    func operator-(other: Origin) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            this.group0 + float4(float3(0.0), other.e4 * -1.0), 
            /* e423, e431, e412, e321 */
            this.group1
        );
    }
}
extension Flector {
    associatedtype Output = Flector;
    func operator-=(const Origin& other) {
        *this = Flector.from_groups(
            /* e1, e2, e3, e4 */
            this.group0 + float4(float3(0.0), other.e4 * -1.0), 
            /* e423, e431, e412, e321 */
            this.group1
        );
    }
}
extension Flector: Sub<Plane> {
    associatedtype Output = Flector;
    func operator-(other: Plane) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            this.group0, 
            /* e423, e431, e412, e321 */
            this.group1 - other.group0
        );
    }
}
extension Flector {
    associatedtype Output = Flector;
    func operator-=(const Plane& other) {
        *this = Flector.from_groups(
            /* e1, e2, e3, e4 */
            this.group0, 
            /* e423, e431, e412, e321 */
            this.group1 - other.group0
        );
    }
}
extension Flector: Sub<Point> {
    associatedtype Output = Flector;
    func operator-(other: Point) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            this.group0 - other.group0, 
            /* e423, e431, e412, e321 */
            this.group1
        );
    }
}
extension Flector {
    associatedtype Output = Flector;
    func operator-=(const Point& other) {
        *this = Flector.from_groups(
            /* e1, e2, e3, e4 */
            this.group0 - other.group0, 
            /* e423, e431, e412, e321 */
            this.group1
        );
    }
}
extension Flector: Sub<Scalar> {
    associatedtype Output = MultiVector;
    func operator-(other: Scalar) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other.scalar, 1.0) * float2(-1.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group0, 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            this.group1
        );
    }
}

impl TryFrom<MultiVector> for Flector {
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
            let mut error = "Elements from MultiVector do not fit into Flector { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(Flector.from_groups(
            /* e1, e2, e3, e4 */
            multi_vector.group1, 
            /* e423, e431, e412, e321 */
            multi_vector.group4
        ));
    }
}
