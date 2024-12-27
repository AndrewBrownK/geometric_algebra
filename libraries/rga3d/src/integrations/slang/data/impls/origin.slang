using traits::GeometricProduct;
extension Origin: Add<AntiScalar> {
    associatedtype Output = MultiVector;
    func operator+(other: AntiScalar) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, other.e1234), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e4), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
extension Origin: Add<DualNum> {
    associatedtype Output = MultiVector;
    func operator+(other: DualNum) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            other.group0, 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e4), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
extension Origin: Add<Flector> {
    associatedtype Output = Flector;
    func operator+(other: Flector) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            other.group0 + float4(float3(0.0), this.e4), 
            /* e423, e431, e412, e321 */
            other.group1
        );
    }
}
extension Origin: Add<Horizon> {
    associatedtype Output = Flector;
    func operator+(other: Horizon) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e4), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other.e321)
        );
    }
}
extension Origin: Add<Line> {
    associatedtype Output = MultiVector;
    func operator+(other: Line) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e4), 
            /* e41, e42, e43 */
            other.group0, 
            /* e23, e31, e12 */
            other.group1, 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
extension Origin: Add<Motor> {
    associatedtype Output = MultiVector;
    func operator+(other: Motor) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other.scalar, other.e1234), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e4), 
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12 */
            other.group1.xyz, 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
extension Origin: Add<MultiVector> {
    associatedtype Output = MultiVector;
    func operator+(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            other.group0, 
            /* e1, e2, e3, e4 */
            other.group1 + float4(float3(0.0), this.e4), 
            /* e41, e42, e43 */
            other.group2, 
            /* e23, e31, e12 */
            other.group3, 
            /* e423, e431, e412, e321 */
            other.group4
        );
    }
}
extension Origin: Add<Origin> {
    associatedtype Output = Origin;
    func operator+(other: Origin) -> Output {
        return Origin.from_groups(
            /* e4 */
            other.e4 + this.e4
        );
    }
}
extension Origin {
    associatedtype Output = Origin;
    func operator+=(const Origin& other) {
        *this = Origin.from_groups(
            /* e4 */
            other.e4 + this.e4
        );
    }
}
extension Origin: Add<Plane> {
    associatedtype Output = Flector;
    func operator+(other: Plane) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e4), 
            /* e423, e431, e412, e321 */
            other.group0
        );
    }
}
extension Origin: Add<Point> {
    associatedtype Output = Point;
    func operator+(other: Point) -> Output {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            other.group0 + float4(float3(0.0), this.e4)
        );
    }
}
extension Origin: Add<Scalar> {
    associatedtype Output = MultiVector;
    func operator+(other: Scalar) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other.scalar, 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e4), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
extension Origin: Mul<DualNum> {
    associatedtype Output = Origin;
    func operator*(other: DualNum) -> Output {
        return this.geometric_product(other);
    }
}
extension Origin {
    associatedtype Output = Origin;
    func operator*=(const DualNum& other) {
        *this = this.geometric_product(other);
    }
}
extension Origin: Mul<Flector> {
    associatedtype Output = Motor;
    func operator*(other: Flector) -> Output {
        return this.geometric_product(other);
    }
}
extension Origin: Mul<Horizon> {
    associatedtype Output = AntiScalar;
    func operator*(other: Horizon) -> Output {
        return this.geometric_product(other);
    }
}
extension Origin: Mul<Line> {
    associatedtype Output = Plane;
    func operator*(other: Line) -> Output {
        return this.geometric_product(other);
    }
}
extension Origin: Mul<Motor> {
    associatedtype Output = Flector;
    func operator*(other: Motor) -> Output {
        return this.geometric_product(other);
    }
}
extension Origin: Mul<MultiVector> {
    associatedtype Output = MultiVector;
    func operator*(other: MultiVector) -> Output {
        return this.geometric_product(other);
    }
}
extension Origin: Mul<Plane> {
    associatedtype Output = AntiScalar;
    func operator*(other: Plane) -> Output {
        return this.geometric_product(other);
    }
}
extension Origin: Mul<Point> {
    associatedtype Output = Line;
    func operator*(other: Point) -> Output {
        return this.geometric_product(other);
    }
}
extension Origin: Mul<Scalar> {
    associatedtype Output = Origin;
    func operator*(other: Scalar) -> Output {
        return this.geometric_product(other);
    }
}
extension Origin {
    associatedtype Output = Origin;
    func operator*=(const Scalar& other) {
        *this = this.geometric_product(other);
    }
}
extension Origin: Neg {
    associatedtype Output = Origin;
    func neg() -> Output {
        return Origin.from_groups(
            /* e4 */
            this.e4 * -1.0
        );
    }
}
extension Origin: Sub<AntiScalar> {
    associatedtype Output = MultiVector;
    func operator-(other: AntiScalar) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(1.0, other.e1234) * float2(0.0, -1.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e4), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
extension Origin: Sub<DualNum> {
    associatedtype Output = MultiVector;
    func operator-(other: DualNum) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            other.group0 * float2(-1.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e4), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
extension Origin: Sub<Flector> {
    associatedtype Output = Flector;
    func operator-(other: Flector) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, this.e4 - other.e4) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e423, e431, e412, e321 */
            other.group1 * float4(-1.0)
        );
    }
}
extension Origin: Sub<Horizon> {
    associatedtype Output = Flector;
    func operator-(other: Horizon) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e4), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other.e321 * -1.0)
        );
    }
}
extension Origin: Sub<Line> {
    associatedtype Output = MultiVector;
    func operator-(other: Line) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e4), 
            /* e41, e42, e43 */
            other.group0 * float3(-1.0), 
            /* e23, e31, e12 */
            other.group1 * float3(-1.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
extension Origin: Sub<Motor> {
    associatedtype Output = MultiVector;
    func operator-(other: Motor) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other.scalar, other.e1234) * float2(-1.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e4), 
            /* e41, e42, e43 */
            other.group0.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            other.group1.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
extension Origin: Sub<MultiVector> {
    associatedtype Output = MultiVector;
    func operator-(other: MultiVector) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            other.group0 * float2(-1.0), 
            /* e1, e2, e3, e4 */
            float4(other.group1.xyz, this.e4 - other.e4) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e41, e42, e43 */
            other.group2 * float3(-1.0), 
            /* e23, e31, e12 */
            other.group3 * float3(-1.0), 
            /* e423, e431, e412, e321 */
            other.group4 * float4(-1.0)
        );
    }
}
extension Origin: Sub<Origin> {
    associatedtype Output = Origin;
    func operator-(other: Origin) -> Output {
        return Origin.from_groups(
            /* e4 */
            this.e4 - other.e4
        );
    }
}
extension Origin {
    associatedtype Output = Origin;
    func operator-=(const Origin& other) {
        *this = Origin.from_groups(
            /* e4 */
            this.e4 - other.e4
        );
    }
}
extension Origin: Sub<Plane> {
    associatedtype Output = Flector;
    func operator-(other: Plane) -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e4), 
            /* e423, e431, e412, e321 */
            other.group0 * float4(-1.0)
        );
    }
}
extension Origin: Sub<Point> {
    associatedtype Output = Point;
    func operator-(other: Point) -> Output {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, this.e4 - other.e4) * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
extension Origin: Sub<Scalar> {
    associatedtype Output = MultiVector;
    func operator-(other: Scalar) -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other.scalar, 1.0) * float2(-1.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e4), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}

impl TryFrom<Flector> for Origin {
    type Error = String;
    fn try_from(flector: Flector) -> Result<Self, Self::Error> {        use crate::elements::*;
        let mut error_string = String::new();
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
            let mut error = "Elements from Flector do not fit into Origin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(Origin.from_groups(
            /* e4 */
            flector.e4
        ));
    }
}

impl TryFrom<MultiVector> for Origin {
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
            let mut error = "Elements from MultiVector do not fit into Origin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(Origin.from_groups(
            /* e4 */
            multi_vector.e4
        ));
    }
}

impl TryFrom<Point> for Origin {
    type Error = String;
    fn try_from(point: Point) -> Result<Self, Self::Error> {        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = point[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = point[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e2: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = point[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from Point do not fit into Origin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(Origin.from_groups(
            /* e4 */
            point.e4
        ));
    }
}
