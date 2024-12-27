extension AntiFlatPoint {
    // Fancy infix trick
    func operator/(_rhs: AntiFixPrefixOrPostfix) -> AntiFlatPoint {
        return this.anti_fix();
    }
    func operator/=(const AntiFixPrefixOrPostfix& _rhs) -> AntiFlatPoint& {
        this = this.anti_fix();
        return *this;
    }
}
extension AntiFlatPoint: AntiFix {
    associatedtype Output = AntiFlatPoint;
    func anti_fix() -> Output {
        return AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            float4(powf((this.group0 * float4(-1.0))[3], -0.5) * powf(this.e321, -0.5)) * this.group0
        );
    }
}
extension AntiPlane {
    // Fancy infix trick
    func operator/(_rhs: AntiFixPrefixOrPostfix) -> AntiPlane {
        return this.anti_fix();
    }
    func operator/=(const AntiFixPrefixOrPostfix& _rhs) -> AntiPlane& {
        this = this.anti_fix();
        return *this;
    }
}
extension AntiPlane: AntiFix {
    associatedtype Output = AntiPlane;
    func anti_fix() -> Output {
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4(-powi(this.e1, 2) - powi(this.e2, 2) - powi(this.e3, 2)) * this.group0
        );
    }
}
extension AntiScalar {
    // Fancy infix trick
    func operator/(_rhs: AntiFixPrefixOrPostfix) -> AntiScalar {
        return this.anti_fix();
    }
    func operator/=(const AntiFixPrefixOrPostfix& _rhs) -> AntiScalar& {
        this = this.anti_fix();
        return *this;
    }
}
extension AntiScalar: AntiFix {
    associatedtype Output = AntiScalar;
    func anti_fix() -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension FlatPoint {
    // Fancy infix trick
    func operator/(_rhs: AntiFixPrefixOrPostfix) -> FlatPoint {
        return this.anti_fix();
    }
    func operator/=(const AntiFixPrefixOrPostfix& _rhs) -> FlatPoint& {
        this = this.anti_fix();
        return *this;
    }
}
extension FlatPoint: AntiFix {
    associatedtype Output = FlatPoint;
    func anti_fix() -> Output {
        return FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            float4(powf((this.group0 * float4(-1.0))[3], -0.5) * powf(this.e45, -0.5) * -1.0) * this.group0
        );
    }
}
extension Plane {
    // Fancy infix trick
    func operator/(_rhs: AntiFixPrefixOrPostfix) -> Plane {
        return this.anti_fix();
    }
    func operator/=(const AntiFixPrefixOrPostfix& _rhs) -> Plane& {
        this = this.anti_fix();
        return *this;
    }
}
extension Plane: AntiFix {
    associatedtype Output = Plane;
    func anti_fix() -> Output {
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(powi(this.e4235, 2) + powi(this.e4315, 2) + powi(this.e4125, 2)) * this.group0
        );
    }
}
extension RoundPoint {
    // Fancy infix trick
    func operator/(_rhs: AntiFixPrefixOrPostfix) -> RoundPoint {
        return this.anti_fix();
    }
    func operator/=(const AntiFixPrefixOrPostfix& _rhs) -> RoundPoint& {
        this = this.anti_fix();
        return *this;
    }
}
extension RoundPoint: AntiFix {
    associatedtype Output = RoundPoint;
    func anti_fix() -> Output {
        AntiScalar geometric_anti_product_2 = AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(2.0*(this.e4 * this.e5) - powi(this.e1, 2) - powi(this.e2, 2) - powi(this.e3, 2), 0.0, 0.0, 0.0)
        );
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(geometric_anti_product_2.e12345) * this.group0, 
            /* e5, 0, 0, 0 */
            float4(geometric_anti_product_2.e12345 * this.e5, 0.0, 0.0, 0.0)
        );
    }
}
extension Scalar {
    // Fancy infix trick
    func operator/(_rhs: AntiFixPrefixOrPostfix) -> Scalar {
        return this.anti_fix();
    }
    func operator/=(const AntiFixPrefixOrPostfix& _rhs) -> Scalar& {
        this = this.anti_fix();
        return *this;
    }
}
extension Scalar: AntiFix {
    associatedtype Output = Scalar;
    func anti_fix() -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(-1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension Sphere {
    // Fancy infix trick
    func operator/(_rhs: AntiFixPrefixOrPostfix) -> Sphere {
        return this.anti_fix();
    }
    func operator/=(const AntiFixPrefixOrPostfix& _rhs) -> Sphere& {
        this = this.anti_fix();
        return *this;
    }
}
extension Sphere: AntiFix {
    associatedtype Output = Sphere;
    func anti_fix() -> Output {
        AntiScalar geometric_anti_product_2 = AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(powi(this.e4235, 2) + powi(this.e4315, 2) + powi(this.e4125, 2) - 2.0 * (this.e3215 * this.e1234), 0.0, 0.0, 0.0)
        );
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(geometric_anti_product_2.e12345) * this.group0, 
            /* e1234, 0, 0, 0 */
            float4(geometric_anti_product_2.e12345 * this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
