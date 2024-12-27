extension Horizon: Fix {
    associatedtype Output = Horizon;
    func fix() -> Output {
        return Horizon.from_groups(
            /* e321 */
            1.0
        );
    }
    func operator/(_rhs: FixInfix) -> Horizon {
        return this.fix();
    }
    func operator/=(const FixInfix& _rhs) -> Horizon& {
        this = this.fix();
        return *this;
    }
}
extension Plane: Fix {
    associatedtype Output = Plane;
    func fix() -> Output {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(powf((this.group0 * float4(-1.0))[3], -0.5) * powf(this.e321, -0.5) * -1.0) * this.group0
        );
    }
    func operator/(_rhs: FixInfix) -> Plane {
        return this.fix();
    }
    func operator/=(const FixInfix& _rhs) -> Plane& {
        this = this.fix();
        return *this;
    }
}
extension Point: Fix {
    associatedtype Output = Point;
    func fix() -> Output {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(powi(this.e1, 2) + powi(this.e2, 2) + powi(this.e3, 2)) * this.group0
        );
    }
    func operator/(_rhs: FixInfix) -> Point {
        return this.fix();
    }
    func operator/=(const FixInfix& _rhs) -> Point& {
        this = this.fix();
        return *this;
    }
}
extension Scalar: Fix {
    associatedtype Output = Scalar;
    func fix() -> Output {
        return Scalar.from_groups(
            /* scalar */
            1.0
        );
    }
    func operator/(_rhs: FixInfix) -> Scalar {
        return this.fix();
    }
    func operator/=(const FixInfix& _rhs) -> Scalar& {
        this = this.fix();
        return *this;
    }
}
