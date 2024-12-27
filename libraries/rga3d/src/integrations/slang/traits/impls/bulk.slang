extension DualNum {
    func div(_rhs: bulk) -> 
Scalar {
        this.bulk()
    }
}
extension DualNum: Bulk {
    func bulk() -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            this.scalar
        );
    }
}
extension Flector {
    func div(_rhs: bulk) -> 
Flector {
        this.bulk()
    }
}
extension Flector {
    func div=(const bulk& _rhs) -> Flector& {
        this = this.bulk()
        return *this;
    }
}
extension Flector: Bulk {
    func bulk() -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e1, this.e2, this.e3, 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.e321)
        );
    }
}
extension Horizon {
    func div(_rhs: bulk) -> 
Horizon {
        this.bulk()
    }
}
extension Horizon {
    func div=(const bulk& _rhs) -> Horizon& {
        this = this.bulk()
        return *this;
    }
}
extension Horizon: Bulk {
    func bulk() -> Horizon {
        return this;
    }
}
extension Line {
    func div(_rhs: bulk) -> 
Line {
        this.bulk()
    }
}
extension Line {
    func div=(const bulk& _rhs) -> Line& {
        this = this.bulk()
        return *this;
    }
}
extension Line: Bulk {
    func bulk() -> Line {
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            this.group1
        );
    }
}
extension Motor {
    func div(_rhs: bulk) -> 
Motor {
        this.bulk()
    }
}
extension Motor {
    func div=(const bulk& _rhs) -> Motor& {
        this = this.bulk()
        return *this;
    }
}
extension Motor: Bulk {
    func bulk() -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(0.0), 
            /* e23, e31, e12, scalar */
            this.group1
        );
    }
}
extension MultiVector {
    func div(_rhs: bulk) -> 
MultiVector {
        this.bulk()
    }
}
extension MultiVector {
    func div=(const bulk& _rhs) -> MultiVector& {
        this = this.bulk()
        return *this;
    }
}
extension MultiVector: Bulk {
    func bulk() -> MultiVector {
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
    func div(_rhs: bulk) -> 
Horizon {
        this.bulk()
    }
}
extension Plane: Bulk {
    func bulk() -> Horizon {
        return Horizon.from_groups(
            /* e321 */
            this.e321
        );
    }
}
extension Point {
    func div(_rhs: bulk) -> 
Point {
        this.bulk()
    }
}
extension Point {
    func div=(const bulk& _rhs) -> Point& {
        this = this.bulk()
        return *this;
    }
}
extension Point: Bulk {
    func bulk() -> Point {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e1, this.e2, this.e3, 0.0)
        );
    }
}
extension Scalar {
    func div(_rhs: bulk) -> 
Scalar {
        this.bulk()
    }
}
extension Scalar {
    func div=(const bulk& _rhs) -> Scalar& {
        this = this.bulk()
        return *this;
    }
}
extension Scalar: Bulk {
    func bulk() -> Scalar {
        return this;
    }
}
