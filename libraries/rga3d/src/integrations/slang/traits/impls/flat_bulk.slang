extension DualNum {
    func div(_rhs: flat_bulk) -> 
Scalar {
        this.flat_bulk()
    }
}
extension DualNum: FlatBulk {
    func flat_bulk() -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            this.scalar
        );
    }
}
extension Flector {
    func div(_rhs: flat_bulk) -> 
Flector {
        this.flat_bulk()
    }
}
extension Flector {
    func div=(const flat_bulk& _rhs) -> Flector& {
        this = this.flat_bulk()
        return *this;
    }
}
extension Flector: FlatBulk {
    func flat_bulk() -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e1, this.e2, this.e3, 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.e321)
        );
    }
}
extension Horizon {
    func div(_rhs: flat_bulk) -> 
Horizon {
        this.flat_bulk()
    }
}
extension Horizon {
    func div=(const flat_bulk& _rhs) -> Horizon& {
        this = this.flat_bulk()
        return *this;
    }
}
extension Horizon: FlatBulk {
    func flat_bulk() -> Horizon {
        return this;
    }
}
extension Line {
    func div(_rhs: flat_bulk) -> 
Line {
        this.flat_bulk()
    }
}
extension Line {
    func div=(const flat_bulk& _rhs) -> Line& {
        this = this.flat_bulk()
        return *this;
    }
}
extension Line: FlatBulk {
    func flat_bulk() -> Line {
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            this.group1
        );
    }
}
extension Motor {
    func div(_rhs: flat_bulk) -> 
Motor {
        this.flat_bulk()
    }
}
extension Motor {
    func div=(const flat_bulk& _rhs) -> Motor& {
        this = this.flat_bulk()
        return *this;
    }
}
extension Motor: FlatBulk {
    func flat_bulk() -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(0.0), 
            /* e23, e31, e12, scalar */
            this.group1
        );
    }
}
extension MultiVector {
    func div(_rhs: flat_bulk) -> 
MultiVector {
        this.flat_bulk()
    }
}
extension MultiVector {
    func div=(const flat_bulk& _rhs) -> MultiVector& {
        this = this.flat_bulk()
        return *this;
    }
}
extension MultiVector: FlatBulk {
    func flat_bulk() -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(this.scalar, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.e1, this.e2, this.e3, 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            this.group3, 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.e321)
        );
    }
}
extension Plane {
    func div(_rhs: flat_bulk) -> 
Horizon {
        this.flat_bulk()
    }
}
extension Plane: FlatBulk {
    func flat_bulk() -> Horizon {
        return Horizon.from_groups(
            /* e321 */
            this.e321
        );
    }
}
extension Point {
    func div(_rhs: flat_bulk) -> 
Point {
        this.flat_bulk()
    }
}
extension Point {
    func div=(const flat_bulk& _rhs) -> Point& {
        this = this.flat_bulk()
        return *this;
    }
}
extension Point: FlatBulk {
    func flat_bulk() -> Point {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e1, this.e2, this.e3, 0.0)
        );
    }
}
extension Scalar {
    func div(_rhs: flat_bulk) -> 
Scalar {
        this.flat_bulk()
    }
}
extension Scalar {
    func div=(const flat_bulk& _rhs) -> Scalar& {
        this = this.flat_bulk()
        return *this;
    }
}
extension Scalar: FlatBulk {
    func flat_bulk() -> Scalar {
        return this;
    }
}
