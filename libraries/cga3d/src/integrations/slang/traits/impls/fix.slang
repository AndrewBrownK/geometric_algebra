extension AntiFlatPoint {
    // Fancy infix trick
    func operator/(_rhs: FixPrefixOrPostfix) -> AntiFlatPoint {
        return this.fix();
    }
    func operator/=(const FixPrefixOrPostfix& _rhs) -> AntiFlatPoint& {
        this = this.fix();
        return *this;
    }
}
extension AntiFlatPoint: Fix {
    associatedtype Output = AntiFlatPoint;
    func fix() -> Output {
        return AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            float4(powf((this.group0 * float4(-1.0))[3], -0.5) * powf(this.e321, -0.5) * -1.0) * this.group0
        );
    }
}
extension AntiPlane {
    // Fancy infix trick
    func operator/(_rhs: FixPrefixOrPostfix) -> AntiPlane {
        return this.fix();
    }
    func operator/=(const FixPrefixOrPostfix& _rhs) -> AntiPlane& {
        this = this.fix();
        return *this;
    }
}
extension AntiPlane: Fix {
    associatedtype Output = AntiPlane;
    func fix() -> Output {
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4(powi(this.e1, 2) + powi(this.e2, 2) + powi(this.e3, 2)) * this.group0
        );
    }
}
extension AntiScalar {
    // Fancy infix trick
    func operator/(_rhs: FixPrefixOrPostfix) -> AntiScalar {
        return this.fix();
    }
    func operator/=(const FixPrefixOrPostfix& _rhs) -> AntiScalar& {
        this = this.fix();
        return *this;
    }
}
extension AntiScalar: Fix {
    associatedtype Output = AntiScalar;
    func fix() -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(-1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension FlatPoint {
    // Fancy infix trick
    func operator/(_rhs: FixPrefixOrPostfix) -> FlatPoint {
        return this.fix();
    }
    func operator/=(const FixPrefixOrPostfix& _rhs) -> FlatPoint& {
        this = this.fix();
        return *this;
    }
}
extension FlatPoint: Fix {
    associatedtype Output = FlatPoint;
    func fix() -> Output {
        return FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            float4(powf((this.group0 * float4(-1.0))[3], -0.5) * powf(this.e45, -0.5)) * this.group0
        );
    }
}
extension Plane {
    // Fancy infix trick
    func operator/(_rhs: FixPrefixOrPostfix) -> Plane {
        return this.fix();
    }
    func operator/=(const FixPrefixOrPostfix& _rhs) -> Plane& {
        this = this.fix();
        return *this;
    }
}
extension Plane: Fix {
    associatedtype Output = Plane;
    func fix() -> Output {
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(-powi(this.e4235, 2) - powi(this.e4315, 2) - powi(this.e4125, 2)) * this.group0
        );
    }
}
extension RoundPoint {
    // Fancy infix trick
    func operator/(_rhs: FixPrefixOrPostfix) -> RoundPoint {
        return this.fix();
    }
    func operator/=(const FixPrefixOrPostfix& _rhs) -> RoundPoint& {
        this = this.fix();
        return *this;
    }
}
extension RoundPoint: Fix {
    associatedtype Output = RoundPoint;
    func fix() -> Output {
        Scalar geometric_product_2 = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(powi(this.e1, 2) + powi(this.e2, 2) + powi(this.e3, 2) - 2.0 * (this.e4 * this.e5), 0.0, 0.0, 0.0)
        );
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(geometric_product_2.scalar) * this.group0, 
            /* e5, 0, 0, 0 */
            float4(this.e5 * geometric_product_2.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension Scalar {
    // Fancy infix trick
    func operator/(_rhs: FixPrefixOrPostfix) -> Scalar {
        return this.fix();
    }
    func operator/=(const FixPrefixOrPostfix& _rhs) -> Scalar& {
        this = this.fix();
        return *this;
    }
}
extension Scalar: Fix {
    associatedtype Output = Scalar;
    func fix() -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension Sphere {
    // Fancy infix trick
    func operator/(_rhs: FixPrefixOrPostfix) -> Sphere {
        return this.fix();
    }
    func operator/=(const FixPrefixOrPostfix& _rhs) -> Sphere& {
        this = this.fix();
        return *this;
    }
}
extension Sphere: Fix {
    associatedtype Output = Sphere;
    func fix() -> Output {
        Scalar geometric_product_2 = Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(2.0*(this.e3215 * this.e1234) - powi(this.e4235, 2) - powi(this.e4315, 2) - powi(this.e4125, 2), 0.0, 0.0, 0.0)
        );
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(geometric_product_2.scalar) * this.group0, 
            /* e1234, 0, 0, 0 */
            float4(geometric_product_2.scalar * this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
