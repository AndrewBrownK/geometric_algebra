extension AntiScalar {
    func div(_rhs: reverse) -> 
AntiScalar {
        this.reverse()
    }
}
extension AntiScalar {
    func div=(const reverse& _rhs) -> AntiScalar& {
        this = this.reverse()
        return *this;
    }
}
extension AntiScalar: Reverse {
    func reverse() -> AntiScalar {
        return this;
    }
}
extension DualNum {
    func div(_rhs: reverse) -> 
DualNum {
        this.reverse()
    }
}
extension DualNum {
    func div=(const reverse& _rhs) -> DualNum& {
        this = this.reverse()
        return *this;
    }
}
extension DualNum: Reverse {
    func reverse() -> DualNum {
        return this;
    }
}
extension Flector {
    func div(_rhs: reverse) -> 
Flector {
        this.reverse()
    }
}
extension Flector {
    func div=(const reverse& _rhs) -> Flector& {
        this = this.reverse()
        return *this;
    }
}
extension Flector: Reverse {
    func reverse() -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            this.group0, 
            /* e423, e431, e412, e321 */
            this.group1 * float4(-1.0)
        );
    }
}
extension Horizon {
    func div(_rhs: reverse) -> 
Horizon {
        this.reverse()
    }
}
extension Horizon {
    func div=(const reverse& _rhs) -> Horizon& {
        this = this.reverse()
        return *this;
    }
}
extension Horizon: Reverse {
    func reverse() -> Horizon {
        return Horizon.from_groups(
            /* e321 */
            this.e321 * -1.0
        );
    }
}
extension Line {
    func div(_rhs: reverse) -> 
Line {
        this.reverse()
    }
}
extension Line {
    func div=(const reverse& _rhs) -> Line& {
        this = this.reverse()
        return *this;
    }
}
extension Line: Reverse {
    func reverse() -> Line {
        return Line.from_groups(
            /* e41, e42, e43 */
            this.group0 * float3(-1.0), 
            /* e23, e31, e12 */
            this.group1 * float3(-1.0)
        );
    }
}
extension Motor {
    func div(_rhs: reverse) -> 
Motor {
        this.reverse()
    }
}
extension Motor {
    func div=(const reverse& _rhs) -> Motor& {
        this = this.reverse()
        return *this;
    }
}
extension Motor: Reverse {
    func reverse() -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
extension MultiVector {
    func div(_rhs: reverse) -> 
MultiVector {
        this.reverse()
    }
}
extension MultiVector {
    func div=(const reverse& _rhs) -> MultiVector& {
        this = this.reverse()
        return *this;
    }
}
extension MultiVector: Reverse {
    func reverse() -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            this.group0, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43 */
            this.group2 * float3(-1.0), 
            /* e23, e31, e12 */
            this.group3 * float3(-1.0), 
            /* e423, e431, e412, e321 */
            this.group4 * float4(-1.0)
        );
    }
}
extension Origin {
    func div(_rhs: reverse) -> 
Origin {
        this.reverse()
    }
}
extension Origin {
    func div=(const reverse& _rhs) -> Origin& {
        this = this.reverse()
        return *this;
    }
}
extension Origin: Reverse {
    func reverse() -> Origin {
        return this;
    }
}
extension Plane {
    func div(_rhs: reverse) -> 
Plane {
        this.reverse()
    }
}
extension Plane {
    func div=(const reverse& _rhs) -> Plane& {
        this = this.reverse()
        return *this;
    }
}
extension Plane: Reverse {
    func reverse() -> Plane {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            this.group0 * float4(-1.0)
        );
    }
}
extension Point {
    func div(_rhs: reverse) -> 
Point {
        this.reverse()
    }
}
extension Point {
    func div=(const reverse& _rhs) -> Point& {
        this = this.reverse()
        return *this;
    }
}
extension Point: Reverse {
    func reverse() -> Point {
        return this;
    }
}
extension Scalar {
    func div(_rhs: reverse) -> 
Scalar {
        this.reverse()
    }
}
extension Scalar {
    func div=(const reverse& _rhs) -> Scalar& {
        this = this.reverse()
        return *this;
    }
}
extension Scalar: Reverse {
    func reverse() -> Scalar {
        return this;
    }
}
