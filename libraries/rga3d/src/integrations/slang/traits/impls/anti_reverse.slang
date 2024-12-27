extension AntiScalar {
    func div(_rhs: anti_reverse) -> 
AntiScalar {
        this.anti_reverse()
    }
}
extension AntiScalar {
    func div=(const anti_reverse& _rhs) -> AntiScalar& {
        this = this.anti_reverse()
        return *this;
    }
}
extension AntiScalar: AntiReverse {
    func anti_reverse() -> AntiScalar {
        return this;
    }
}
extension DualNum {
    func div(_rhs: anti_reverse) -> 
DualNum {
        this.anti_reverse()
    }
}
extension DualNum {
    func div=(const anti_reverse& _rhs) -> DualNum& {
        this = this.anti_reverse()
        return *this;
    }
}
extension DualNum: AntiReverse {
    func anti_reverse() -> DualNum {
        return this;
    }
}
extension Flector {
    func div(_rhs: anti_reverse) -> 
Flector {
        this.anti_reverse()
    }
}
extension Flector {
    func div=(const anti_reverse& _rhs) -> Flector& {
        this = this.anti_reverse()
        return *this;
    }
}
extension Flector: AntiReverse {
    func anti_reverse() -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            this.group0 * float4(-1.0), 
            /* e423, e431, e412, e321 */
            this.group1
        );
    }
}
extension Horizon {
    func div(_rhs: anti_reverse) -> 
Horizon {
        this.anti_reverse()
    }
}
extension Horizon {
    func div=(const anti_reverse& _rhs) -> Horizon& {
        this = this.anti_reverse()
        return *this;
    }
}
extension Horizon: AntiReverse {
    func anti_reverse() -> Horizon {
        return this;
    }
}
extension Line {
    func div(_rhs: anti_reverse) -> 
Line {
        this.anti_reverse()
    }
}
extension Line {
    func div=(const anti_reverse& _rhs) -> Line& {
        this = this.anti_reverse()
        return *this;
    }
}
extension Line: AntiReverse {
    func anti_reverse() -> Line {
        return Line.from_groups(
            /* e41, e42, e43 */
            this.group0 * float3(-1.0), 
            /* e23, e31, e12 */
            this.group1 * float3(-1.0)
        );
    }
}
extension Motor {
    func div(_rhs: anti_reverse) -> 
Motor {
        this.anti_reverse()
    }
}
extension Motor {
    func div=(const anti_reverse& _rhs) -> Motor& {
        this = this.anti_reverse()
        return *this;
    }
}
extension Motor: AntiReverse {
    func anti_reverse() -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
extension MultiVector {
    func div(_rhs: anti_reverse) -> 
MultiVector {
        this.anti_reverse()
    }
}
extension MultiVector {
    func div=(const anti_reverse& _rhs) -> MultiVector& {
        this = this.anti_reverse()
        return *this;
    }
}
extension MultiVector: AntiReverse {
    func anti_reverse() -> MultiVector {
        return MultiVector.from_groups(
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
    }
}
extension Origin {
    func div(_rhs: anti_reverse) -> 
Origin {
        this.anti_reverse()
    }
}
extension Origin {
    func div=(const anti_reverse& _rhs) -> Origin& {
        this = this.anti_reverse()
        return *this;
    }
}
extension Origin: AntiReverse {
    func anti_reverse() -> Origin {
        return Origin.from_groups(
            /* e4 */
            this.e4 * -1.0
        );
    }
}
extension Plane {
    func div(_rhs: anti_reverse) -> 
Plane {
        this.anti_reverse()
    }
}
extension Plane {
    func div=(const anti_reverse& _rhs) -> Plane& {
        this = this.anti_reverse()
        return *this;
    }
}
extension Plane: AntiReverse {
    func anti_reverse() -> Plane {
        return this;
    }
}
extension Point {
    func div(_rhs: anti_reverse) -> 
Point {
        this.anti_reverse()
    }
}
extension Point {
    func div=(const anti_reverse& _rhs) -> Point& {
        this = this.anti_reverse()
        return *this;
    }
}
extension Point: AntiReverse {
    func anti_reverse() -> Point {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            this.group0 * float4(-1.0)
        );
    }
}
extension Scalar {
    func div(_rhs: anti_reverse) -> 
Scalar {
        this.anti_reverse()
    }
}
extension Scalar {
    func div=(const anti_reverse& _rhs) -> Scalar& {
        this = this.anti_reverse()
        return *this;
    }
}
extension Scalar: AntiReverse {
    func anti_reverse() -> Scalar {
        return this;
    }
}
