using traits::GeometricProduct;
using traits::RightDual;
extension Plane: Add<AntiScalar> {
    associatedtype Output = MultiVector;
    func operator+(other: AntiScalar) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(0.0, other.e1234), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            this.group0
        );
    }
}
extension Plane: Add<DualNum> {
    associatedtype Output = MultiVector;
    func operator+(other: DualNum) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(other.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            this.group0
        );
    }
}
extension Plane: Add<Flector> {
    associatedtype Output = Flector;
    func operator+(other: Flector) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            other.group0, 
            /* e423, e431, e412, e321 */
            other.group1 + this.group0
        );
    }
}
extension Plane: Add<Horizon> {
    associatedtype Output = Plane;
    func operator+(other: Horizon) -> Output {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            this.group0 + float4(float3(0.0), other.e321)
        );
    }
}
extension Plane {
    associatedtype Output = Plane;
    func operator+=(const Horizon& other) {
        *this = Plane.from_groups(
            /* e423, e431, e412, e321 */
            this.group0 + float4(float3(0.0), other.e321)
        );
    }
}
extension Plane: Add<Line> {
    associatedtype Output = MultiVector;
    func operator+(other: Line) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group1.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            this.group0
        );
    }
}
extension Plane: Add<Motor> {
    associatedtype Output = MultiVector;
    func operator+(other: Motor) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(other.scalar, other.e1234), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group1.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            this.group0
        );
    }
}
extension Plane: Add<MultiVector> {
    associatedtype Output = MultiVector;
    func operator+(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(other.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            other.group1, 
            /* e41, e42, e43, 0 */
            float4(other.group2.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group3.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            other.group4 + this.group0
        );
    }
}
extension Plane: Add<Origin> {
    associatedtype Output = Flector;
    func operator+(other: Origin) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e4), 
            /* e423, e431, e412, e321 */
            this.group0
        );
    }
}
extension Plane: Add<Plane> {
    associatedtype Output = Plane;
    func operator+(other: Plane) -> Output {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            other.group0 + this.group0
        );
    }
}
extension Plane {
    associatedtype Output = Plane;
    func operator+=(const Plane& other) {
        *this = Plane.from_groups(
            /* e423, e431, e412, e321 */
            other.group0 + this.group0
        );
    }
}
extension Plane: Add<Point> {
    associatedtype Output = Flector;
    func operator+(other: Point) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            other.group0, 
            /* e423, e431, e412, e321 */
            this.group0
        );
    }
}
extension Plane: Add<Scalar> {
    associatedtype Output = MultiVector;
    func operator+(other: Scalar) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(other.scalar, 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            this.group0
        );
    }
}

impl From<Horizon> for Plane {
    fn from(from_horizon: Horizon) -> Self {
        use crate::elements::*;
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(float3(0.0), from_horizon.e321)
        );
    }
}
extension Plane: Mul<AntiScalar> {
    associatedtype Output = Origin;
    func operator*(other: AntiScalar) -> Output {
        return this.geometric_product(other);
    }
}
extension Plane: Mul<DualNum> {
    associatedtype Output = Flector;
    func operator*(other: DualNum) -> Output {
        return this.geometric_product(other);
    }
}
extension Plane: Mul<Flector> {
    associatedtype Output = Motor;
    func operator*(other: Flector) -> Output {
        return this.geometric_product(other);
    }
}
extension Plane: Mul<Horizon> {
    associatedtype Output = Motor;
    func operator*(other: Horizon) -> Output {
        return this.geometric_product(other);
    }
}
extension Plane: Mul<Line> {
    associatedtype Output = Flector;
    func operator*(other: Line) -> Output {
        return this.geometric_product(other);
    }
}
extension Plane: Mul<Motor> {
    associatedtype Output = Flector;
    func operator*(other: Motor) -> Output {
        return this.geometric_product(other);
    }
}
extension Plane: Mul<MultiVector> {
    associatedtype Output = MultiVector;
    func operator*(other: MultiVector) -> Output {
        return this.geometric_product(other);
    }
}
extension Plane: Mul<Origin> {
    associatedtype Output = AntiScalar;
    func operator*(other: Origin) -> Output {
        return this.geometric_product(other);
    }
}
extension Plane: Mul<Plane> {
    associatedtype Output = Motor;
    func operator*(other: Plane) -> Output {
        return this.geometric_product(other);
    }
}
extension Plane: Mul<Point> {
    associatedtype Output = Motor;
    func operator*(other: Point) -> Output {
        return this.geometric_product(other);
    }
}
extension Plane: Mul<Scalar> {
    associatedtype Output = Plane;
    func operator*(other: Scalar) -> Output {
        return this.geometric_product(other);
    }
}
extension Plane {
    associatedtype Output = Plane;
    func operator*=(const Scalar& other) {
        *this = this.geometric_product(other);
    }
}
extension Plane: Neg {
    associatedtype Output = Plane;
    func neg() -> Output {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            this.group0 * float4(-1.0)
        );
    }
}
extension Plane: Not {
    associatedtype Output = Origin;
    func not() -> Output {
        return this.right_dual();
    }
}
extension Plane: Sub<AntiScalar> {
    associatedtype Output = MultiVector;
    func operator-(other: AntiScalar) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(1.0, other.e1234) * float2(0.0, -1.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            this.group0
        );
    }
}
extension Plane: Sub<DualNum> {
    associatedtype Output = MultiVector;
    func operator-(other: DualNum) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(other.group0.xy * float2(-1.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            this.group0
        );
    }
}
extension Plane: Sub<Flector> {
    associatedtype Output = Flector;
    func operator-(other: Flector) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            other.group0 * float4(-1.0), 
            /* e423, e431, e412, e321 */
            this.group0 - other.group1
        );
    }
}
extension Plane: Sub<Horizon> {
    associatedtype Output = Plane;
    func operator-(other: Horizon) -> Output {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            this.group0 + float4(float3(0.0), other.e321 * -1.0)
        );
    }
}
extension Plane {
    associatedtype Output = Plane;
    func operator-=(const Horizon& other) {
        *this = Plane.from_groups(
            /* e423, e431, e412, e321 */
            this.group0 + float4(float3(0.0), other.e321 * -1.0)
        );
    }
}
extension Plane: Sub<Line> {
    associatedtype Output = MultiVector;
    func operator-(other: Line) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0), 
            /* e423, e431, e412, e321 */
            this.group0
        );
    }
}
extension Plane: Sub<Motor> {
    associatedtype Output = MultiVector;
    func operator-(other: Motor) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(other.scalar, other.e1234) * float2(-1.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43, 0 */
            float4(other.group0.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group1.xyz * float3(-1.0), 0.0), 
            /* e423, e431, e412, e321 */
            this.group0
        );
    }
}
extension Plane: Sub<MultiVector> {
    associatedtype Output = MultiVector;
    func operator-(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(other.group0.xy * float2(-1.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            other.group1 * float4(-1.0), 
            /* e41, e42, e43, 0 */
            float4(other.group2.xyz * float3(-1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(other.group3.xyz * float3(-1.0), 0.0), 
            /* e423, e431, e412, e321 */
            this.group0 - other.group4
        );
    }
}
extension Plane: Sub<Origin> {
    associatedtype Output = Flector;
    func operator-(other: Origin) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e4 * -1.0), 
            /* e423, e431, e412, e321 */
            this.group0
        );
    }
}
extension Plane: Sub<Plane> {
    associatedtype Output = Plane;
    func operator-(other: Plane) -> Output {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            this.group0 - other.group0
        );
    }
}
extension Plane {
    associatedtype Output = Plane;
    func operator-=(const Plane& other) {
        *this = Plane.from_groups(
            /* e423, e431, e412, e321 */
            this.group0 - other.group0
        );
    }
}
extension Plane: Sub<Point> {
    associatedtype Output = Flector;
    func operator-(other: Point) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            other.group0 * float4(-1.0), 
            /* e423, e431, e412, e321 */
            this.group0
        );
    }
}
extension Plane: Sub<Scalar> {
    associatedtype Output = MultiVector;
    func operator-(other: Scalar) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(other.scalar, 1.0) * float2(-1.0, 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            this.group0
        );
    }
}

impl TryFrom<Flector> for Plane {
    type Error = String;
    fn try_from(flector: Flector) -> Result<Self, Self::Error> {        let mut error_string = String::new();
        let mut fail = false;
        let el = flector[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = flector[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e2: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = flector[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = flector[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from Flector do not fit into Plane { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(Plane.from_groups(
            /* e423, e431, e412, e321 */
            flector.group1
        ));
    }
}

impl TryFrom<MultiVector> for Plane {
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
            let mut error = "Elements from MultiVector do not fit into Plane { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(Plane.from_groups(
            /* e423, e431, e412, e321 */
            multi_vector.group4
        ));
    }
}
