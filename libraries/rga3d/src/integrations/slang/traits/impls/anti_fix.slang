extension AntiScalar: AntiFix {
    associatedtype Output = AntiScalar;
    func anti_fix() -> Output {
        return AntiScalar.from_groups(
            /* e1234 */
            1.0
        );
    }
    func operator/(_rhs: AntiFixInfix) -> AntiScalar {
        return this.anti_fix();
    }
    func operator/=(const AntiFixInfix& _rhs) -> AntiScalar& {
        this = this.anti_fix();
        return *this;
    }
}
extension Origin: AntiFix {
    associatedtype Output = Origin;
    func anti_fix() -> Output {
        return Origin.from_groups(
            /* e4 */
            1.0
        );
    }
    func operator/(_rhs: AntiFixInfix) -> Origin {
        return this.anti_fix();
    }
    func operator/=(const AntiFixInfix& _rhs) -> Origin& {
        this = this.anti_fix();
        return *this;
    }
}
extension Plane: AntiFix {
    associatedtype Output = Plane;
    func anti_fix() -> Output {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(powi(this.e423, 2) + powi(this.e431, 2) + powi(this.e412, 2)) * this.group0
        );
    }
    func operator/(_rhs: AntiFixInfix) -> Plane {
        return this.anti_fix();
    }
    func operator/=(const AntiFixInfix& _rhs) -> Plane& {
        this = this.anti_fix();
        return *this;
    }
}
extension Point: AntiFix {
    associatedtype Output = Point;
    func anti_fix() -> Output {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(powf((this.group0 * float4(-1.0))[3], -0.5) * powf(this.e4, -0.5) * -1.0) * this.group0
        );
    }
    func operator/(_rhs: AntiFixInfix) -> Point {
        return this.anti_fix();
    }
    func operator/=(const AntiFixInfix& _rhs) -> Point& {
        this = this.anti_fix();
        return *this;
    }
}
