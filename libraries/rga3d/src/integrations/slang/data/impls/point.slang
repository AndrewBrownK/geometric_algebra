using traits::GeometricProduct;
using traits::RightDual;
extension Point: Add<AntiScalar> {
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
            float4(0.0)
        );
    }
}
extension Point: Add<DualNum> {
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
            float4(0.0)
        );
    }
}
extension Point: Add<Flector> {
    associatedtype Output = Flector;
    func operator+(other: Flector) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            other.group0 + this.group0, 
            /* e423, e431, e412, e321 */
            other.group1
        );
    }
}
extension Point: Add<Horizon> {
    associatedtype Output = Flector;
    func operator+(other: Horizon) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            this.group0, 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other.e321)
        );
    }
}
extension Point: Add<Line> {
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
            float4(0.0)
        );
    }
}
extension Point: Add<Motor> {
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
            float4(0.0)
        );
    }
}
extension Point: Add<MultiVector> {
    associatedtype Output = MultiVector;
    func operator+(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            other.group0, 
            /* e1, e2, e3, e4 */
            other.group1 + this.group0, 
            /* e41, e42, e43 */
            other.group2, 
            /* e23, e31, e12 */
            other.group3, 
            /* e423, e431, e412, e321 */
            other.group4
        );
    }
}
extension Point: Add<Origin> {
    associatedtype Output = Point;
    func operator+(other: Origin) -> Output {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            this.group0 + float4(float3(0.0), other.e4)
        );
    }
}
extension Point {
    associatedtype Output = Point;
    func operator+=(const Origin& other) {
        *this = Point.from_groups(
            /* e1, e2, e3, e4 */
            this.group0 + float4(float3(0.0), other.e4)
        );
    }
}
extension Point: Add<Plane> {
    associatedtype Output = Flector;
    func operator+(other: Plane) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            this.group0, 
            /* e423, e431, e412, e321 */
            other.group0
        );
    }
}
extension Point: Add<Point> {
    associatedtype Output = Point;
    func operator+(other: Point) -> Output {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            other.group0 + this.group0
        );
    }
}
extension Point {
    associatedtype Output = Point;
    func operator+=(const Point& other) {
        *this = Point.from_groups(
            /* e1, e2, e3, e4 */
            other.group0 + this.group0
        );
    }
}
extension Point: Add<Scalar> {
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
            float4(0.0)
        );
    }
}

impl From<Origin> for Point {
    fn from(from_origin: Origin) -> Self {
        use crate::elements::*;
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), from_origin.e4)
        );
    }
}
extension Point: Mul<AntiScalar> {
    associatedtype Output = Plane;
    func operator*(other: AntiScalar) -> Output {
        return this.geometric_product(other);
    }
}
extension Point: Mul<DualNum> {
    associatedtype Output = Flector;
    func operator*(other: DualNum) -> Output {
        return this.geometric_product(other);
    }
}
extension Point: Mul<Flector> {
    associatedtype Output = Motor;
    func operator*(other: Flector) -> Output {
        return this.geometric_product(other);
    }
}
extension Point: Mul<Horizon> {
    associatedtype Output = Motor;
    func operator*(other: Horizon) -> Output {
        return this.geometric_product(other);
    }
}
extension Point: Mul<Line> {
    associatedtype Output = Flector;
    func operator*(other: Line) -> Output {
        return this.geometric_product(other);
    }
}
extension Point: Mul<Motor> {
    associatedtype Output = Flector;
    func operator*(other: Motor) -> Output {
        return this.geometric_product(other);
    }
}
extension Point: Mul<MultiVector> {
    associatedtype Output = MultiVector;
    func operator*(other: MultiVector) -> Output {
        return this.geometric_product(other);
    }
}
extension Point: Mul<Origin> {
    associatedtype Output = Line;
    func operator*(other: Origin) -> Output {
        return this.geometric_product(other);
    }
}
extension Point: Mul<Plane> {
    associatedtype Output = Motor;
    func operator*(other: Plane) -> Output {
        return this.geometric_product(other);
    }
}
extension Point: Mul<Point> {
    associatedtype Output = Motor;
    func operator*(other: Point) -> Output {
        return this.geometric_product(other);
    }
}
extension Point: Mul<Scalar> {
    associatedtype Output = Point;
    func operator*(other: Scalar) -> Output {
        return this.geometric_product(other);
    }
}
extension Point {
    associatedtype Output = Point;
    func operator*=(const Scalar& other) {
        *this = this.geometric_product(other);
    }
}
extension Point: Neg {
    associatedtype Output = Point;
    func neg() -> Output {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            this.group0 * float4(-1.0)
        );
    }
}
extension Point: Not {
    associatedtype Output = Plane;
    func not() -> Output {
        return this.right_dual();
    }
}
extension Point: Sub<AntiScalar> {
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
            float4(0.0)
        );
    }
}
extension Point: Sub<DualNum> {
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
            float4(0.0)
        );
    }
}
extension Point: Sub<Flector> {
    associatedtype Output = Flector;
    func operator-(other: Flector) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            this.group0 - other.group0, 
            /* e423, e431, e412, e321 */
            other.group1 * float4(-1.0)
        );
    }
}
extension Point: Sub<Horizon> {
    associatedtype Output = Flector;
    func operator-(other: Horizon) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            this.group0, 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other.e321 * -1.0)
        );
    }
}
extension Point: Sub<Line> {
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
            float4(0.0)
        );
    }
}
extension Point: Sub<Motor> {
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
            float4(0.0)
        );
    }
}
extension Point: Sub<MultiVector> {
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
            other.group4 * float4(-1.0)
        );
    }
}
extension Point: Sub<Origin> {
    associatedtype Output = Point;
    func operator-(other: Origin) -> Output {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            this.group0 + float4(float3(0.0), other.e4 * -1.0)
        );
    }
}
extension Point {
    associatedtype Output = Point;
    func operator-=(const Origin& other) {
        *this = Point.from_groups(
            /* e1, e2, e3, e4 */
            this.group0 + float4(float3(0.0), other.e4 * -1.0)
        );
    }
}
extension Point: Sub<Plane> {
    associatedtype Output = Flector;
    func operator-(other: Plane) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            this.group0, 
            /* e423, e431, e412, e321 */
            other.group0 * float4(-1.0)
        );
    }
}
extension Point: Sub<Point> {
    associatedtype Output = Point;
    func operator-(other: Point) -> Output {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            this.group0 - other.group0
        );
    }
}
extension Point {
    associatedtype Output = Point;
    func operator-=(const Point& other) {
        *this = Point.from_groups(
            /* e1, e2, e3, e4 */
            this.group0 - other.group0
        );
    }
}
extension Point: Sub<Scalar> {
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
            float4(0.0)
        );
    }
}

impl TryFrom<Flector> for Point {
    type Error = String;
    fn try_from(flector: Flector) -> Result<Self, Self::Error> {        let mut error_string = String::new();
        let mut fail = false;
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
            let mut error = "Elements from Flector do not fit into Point { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(Point.from_groups(
            /* e1, e2, e3, e4 */
            flector.group0
        ));
    }
}

impl TryFrom<MultiVector> for Point {
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
            let mut error = "Elements from MultiVector do not fit into Point { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(Point.from_groups(
            /* e1, e2, e3, e4 */
            multi_vector.group1
        ));
    }
}
