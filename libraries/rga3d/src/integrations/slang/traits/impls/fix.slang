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
    func fix(self) -> Horizon {
        return Horizon::from_groups(
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
    func fix(self) -> Plane {
        return Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4(powf((self.group0() * float4(-1.0))[3], -0.5) * powf(self[e321], -0.5) * -1.0) * self.group0()
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
    func fix(self) -> Point {
        return Point::from_groups(
            /* e1, e2, e3, e4 */
            float4(powi(self[e1], 2) + powi(self[e2], 2) + powi(self[e3], 2)) * self.group0()
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
    func fix(self) -> Scalar {
        return Scalar::from_groups(
            /* scalar */
            1.0
        );
    }
}
