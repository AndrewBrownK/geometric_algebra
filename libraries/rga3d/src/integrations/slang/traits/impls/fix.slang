extension Horizon {
    func div(_rhs: fix) -> 
Horizon {
        this.fix()
    }
}
extension Horizon {
    func div=(const fix& _rhs) -> Horizon& {
        this = this.fix()
        return *this;
    }
}
extension Horizon: Fix {
    func fix() -> Horizon {
        return Horizon.from_groups(
            /* e321 */
            1.0
        );
    }
}
extension Plane {
    func div(_rhs: fix) -> 
Plane {
        this.fix()
    }
}
extension Plane {
    func div=(const fix& _rhs) -> Plane& {
        this = this.fix()
        return *this;
    }
}
extension Plane: Fix {
    func fix() -> Plane {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(powf((this.group0 * float4(-1.0))[3], -0.5) * powf(this.e321, -0.5) * -1.0) * this.group0
        );
    }
}
extension Point {
    func div(_rhs: fix) -> 
Point {
        this.fix()
    }
}
extension Point {
    func div=(const fix& _rhs) -> Point& {
        this = this.fix()
        return *this;
    }
}
extension Point: Fix {
    func fix() -> Point {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(powi(this.e1, 2) + powi(this.e2, 2) + powi(this.e3, 2)) * this.group0
        );
    }
}
extension Scalar {
    func div(_rhs: fix) -> 
Scalar {
        this.fix()
    }
}
extension Scalar {
    func div=(const fix& _rhs) -> Scalar& {
        this = this.fix()
        return *this;
    }
}
extension Scalar: Fix {
    func fix() -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            1.0
        );
    }
}
