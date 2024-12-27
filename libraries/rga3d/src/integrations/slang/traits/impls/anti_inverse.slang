extension AntiScalar: AntiInverse {
    associatedtype Output = AntiScalar;
    func anti_inverse() -> Output {
        return AntiScalar.from_groups(
            /* e1234 */
            1.0/this.e1234
        );
    }
    func operator/(_rhs: AntiInverseInfix) -> AntiScalar {
        return this.anti_inverse();
    }
    func operator/=(const AntiInverseInfix& _rhs) -> AntiScalar& {
        this = this.anti_inverse();
        return *this;
    }
}
extension DualNum: AntiInverse {
    associatedtype Output = DualNum;
    func anti_inverse() -> Output {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(powi(this.e1234, -2)) * this.group0
        );
    }
    func operator/(_rhs: AntiInverseInfix) -> DualNum {
        return this.anti_inverse();
    }
    func operator/=(const AntiInverseInfix& _rhs) -> DualNum& {
        this = this.anti_inverse();
        return *this;
    }
}
extension Flector: AntiInverse {
    associatedtype Output = Flector;
    func anti_inverse() -> Output {
Flector anti_reverse = Flector.from_groups(
            /* e1, e2, e3, e4 */
            this.group0 * float4(-1.0), 
            /* e423, e431, e412, e321 */
            this.group1
        );
AntiScalar other = AntiScalar.from_groups(
            /* e1234 */
            powi(this.e4, 2) + powi(this.e423, 2) + powi(this.e431, 2) + powi(this.e412, 2)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.e1234) * anti_reverse.group0, 
            /* e423, e431, e412, e321 */
            float4(other.e1234) * anti_reverse.group1
        );
    }
    func operator/(_rhs: AntiInverseInfix) -> Flector {
        return this.anti_inverse();
    }
    func operator/=(const AntiInverseInfix& _rhs) -> Flector& {
        this = this.anti_inverse();
        return *this;
    }
}
extension Line: AntiInverse {
    associatedtype Output = Line;
    func anti_inverse() -> Output {
Line anti_reverse = Line.from_groups(
            /* e41, e42, e43 */
            this.group0 * float3(-1.0), 
            /* e23, e31, e12 */
            this.group1 * float3(-1.0)
        );
AntiScalar other = AntiScalar.from_groups(
            /* e1234 */
            powi(this.e41, 2) + powi(this.e42, 2) + powi(this.e43, 2)
        );
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(other.e1234) * anti_reverse.group0, 
            /* e23, e31, e12 */
            float3(other.e1234) * anti_reverse.group1
        );
    }
    func operator/(_rhs: AntiInverseInfix) -> Line {
        return this.anti_inverse();
    }
    func operator/=(const AntiInverseInfix& _rhs) -> Line& {
        this = this.anti_inverse();
        return *this;
    }
}
extension Motor: AntiInverse {
    associatedtype Output = Motor;
    func anti_inverse() -> Output {
Motor anti_reverse = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
AntiScalar other = AntiScalar.from_groups(
            /* e1234 */
            powi(this.e41, 2) + powi(this.e42, 2) + powi(this.e43, 2) + powi(this.e1234, 2)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.e1234) * anti_reverse.group0, 
            /* e23, e31, e12, scalar */
            float4(other.e1234) * anti_reverse.group1
        );
    }
    func operator/(_rhs: AntiInverseInfix) -> Motor {
        return this.anti_inverse();
    }
    func operator/=(const AntiInverseInfix& _rhs) -> Motor& {
        this = this.anti_inverse();
        return *this;
    }
}
extension MultiVector: AntiInverse {
    associatedtype Output = MultiVector;
    func anti_inverse() -> Output {
MultiVector anti_reverse = MultiVector.from_groups(
            /* scalar, e1234 */
            this.group0, 
            /* e1, e2, e3, e4 */
            this.group1 * float4(-1.0), 
            /* e41, e42, e43 */
            this.group2 * float3(-1.0), 
            /* e23, e31, e12 */
            this.group3 * float3(-1.0), 
            /* e423, e431, e412, e321 */
            this.group4
        );
AntiScalar other = AntiScalar.from_groups(
            /* e1234 */
            powi(this.e1234, 2) + powi(this.e4, 2) + powi(this.e41, 2) + powi(this.e42, 2) + powi(this.e43, 2) + powi(this.e423, 2) + powi(this.e431, 2) + powi(this.e412, 2)
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other.e1234) * anti_reverse.group0, 
            /* e1, e2, e3, e4 */
            float4(other.e1234) * anti_reverse.group1, 
            /* e41, e42, e43 */
            float3(other.e1234) * anti_reverse.group2, 
            /* e23, e31, e12 */
            float3(other.e1234) * anti_reverse.group3, 
            /* e423, e431, e412, e321 */
            float4(other.e1234) * anti_reverse.group4
        );
    }
    func operator/(_rhs: AntiInverseInfix) -> MultiVector {
        return this.anti_inverse();
    }
    func operator/=(const AntiInverseInfix& _rhs) -> MultiVector& {
        this = this.anti_inverse();
        return *this;
    }
}
extension Origin: AntiInverse {
    associatedtype Output = Origin;
    func anti_inverse() -> Output {
        return Origin.from_groups(
            /* e4 */
            1.0/this.e4 * -1.0
        );
    }
    func operator/(_rhs: AntiInverseInfix) -> Origin {
        return this.anti_inverse();
    }
    func operator/=(const AntiInverseInfix& _rhs) -> Origin& {
        this = this.anti_inverse();
        return *this;
    }
}
extension Plane: AntiInverse {
    associatedtype Output = Plane;
    func anti_inverse() -> Output {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(powi(this.e423, 2) + powi(this.e431, 2) + powi(this.e412, 2)) * this.group0
        );
    }
    func operator/(_rhs: AntiInverseInfix) -> Plane {
        return this.anti_inverse();
    }
    func operator/=(const AntiInverseInfix& _rhs) -> Plane& {
        this = this.anti_inverse();
        return *this;
    }
}
extension Point: AntiInverse {
    associatedtype Output = Point;
    func anti_inverse() -> Output {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(powi(this.e4, -2)) * float4(this.e1 * -1.0, this.e2 * -1.0, this.e3 * -1.0, this.e4 * -1.0)
        );
    }
    func operator/(_rhs: AntiInverseInfix) -> Point {
        return this.anti_inverse();
    }
    func operator/=(const AntiInverseInfix& _rhs) -> Point& {
        this = this.anti_inverse();
        return *this;
    }
}
