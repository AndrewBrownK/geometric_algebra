extension AntiScalar {
    func div(_rhs: anti_fix) -> 
AntiScalar {
        this.anti_fix()
    }
}
extension AntiScalar {
    func div=(const anti_fix& _rhs) -> AntiScalar& {
        this = this.anti_fix()
        return *this;
    }
}
extension AntiScalar: AntiFix {
    func anti_fix() -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            1.0
        );
    }
}
extension Origin {
    func div(_rhs: anti_fix) -> 
Origin {
        this.anti_fix()
    }
}
extension Origin {
    func div=(const anti_fix& _rhs) -> Origin& {
        this = this.anti_fix()
        return *this;
    }
}
extension Origin: AntiFix {
    func anti_fix() -> Origin {
        return Origin.from_groups(
            /* e4 */
            1.0
        );
    }
}
extension Plane {
    func div(_rhs: anti_fix) -> 
Plane {
        this.anti_fix()
    }
}
extension Plane {
    func div=(const anti_fix& _rhs) -> Plane& {
        this = this.anti_fix()
        return *this;
    }
}
extension Plane: AntiFix {
    func anti_fix() -> Plane {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(powi(this.e423, 2) + powi(this.e431, 2) + powi(this.e412, 2)) * this.group0
        );
    }
}
extension Point {
    func div(_rhs: anti_fix) -> 
Point {
        this.anti_fix()
    }
}
extension Point {
    func div=(const anti_fix& _rhs) -> Point& {
        this = this.anti_fix()
        return *this;
    }
}
extension Point: AntiFix {
    func anti_fix() -> Point {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(powf((this.group0 * float4(-1.0))[3], -0.5) * powf(this.e4, -0.5) * -1.0) * this.group0
        );
    }
}
