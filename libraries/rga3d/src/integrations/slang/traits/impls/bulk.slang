extension DualNum {
    func div(_rhs: bulk) -> 
Scalar {
        this.bulk()
    }
}
extension DualNum: Bulk {
    func bulk(self) -> Scalar {
        return Scalar::from_groups(
            /* scalar */
            self[scalar]
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
    func bulk(self) -> Flector {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(self[e1], self[e2], self[e3], 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), self[e321])
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
    func bulk(self) -> Horizon {
        return self;
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
    func bulk(self) -> Line {
        return Line::from_groups(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            self.group1()
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
    func bulk(self) -> Motor {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(0.0), 
            /* e23, e31, e12, scalar */
            self.group1()
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
    func bulk(self) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(self[scalar], 0.0), 
            /* e1, e2, e3, e4 */
            float4(self[e1], self[e2], self[e3], 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            self.group3(), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), self[e321])
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
    func bulk(self) -> Horizon {
        return Horizon::from_groups(
            /* e321 */
            self[e321]
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
    func bulk(self) -> Point {
        return Point::from_groups(
            /* e1, e2, e3, e4 */
            float4(self[e1], self[e2], self[e3], 0.0)
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
    func bulk(self) -> Scalar {
        return self;
    }
}
