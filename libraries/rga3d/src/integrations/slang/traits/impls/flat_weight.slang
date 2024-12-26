extension AntiScalar {
    func div(_rhs: flat_weight) -> 
AntiScalar {
        this.flat_weight()
    }
}
extension AntiScalar {
    func div=(const flat_weight& _rhs) -> AntiScalar& {
        this = this.flat_weight()
        return *this;
    }
}
extension AntiScalar: FlatWeight {
    func flat_weight(self) -> AntiScalar {
        return self;
    }
}
extension DualNum {
    func div(_rhs: flat_weight) -> 
AntiScalar {
        this.flat_weight()
    }
}
extension DualNum: FlatWeight {
    func flat_weight(self) -> AntiScalar {
        return AntiScalar::from_groups(
            /* e1234 */
            self[e1234]
        );
    }
}
extension Flector {
    func div(_rhs: flat_weight) -> 
Flector {
        this.flat_weight()
    }
}
extension Flector {
    func div=(const flat_weight& _rhs) -> Flector& {
        this = this.flat_weight()
        return *this;
    }
}
extension Flector: FlatWeight {
    func flat_weight(self) -> Flector {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), self[e4]), 
            /* e423, e431, e412, e321 */
            float4(self[e423], self[e431], self[e412], 0.0)
        );
    }
}
extension Line {
    func div(_rhs: flat_weight) -> 
Line {
        this.flat_weight()
    }
}
extension Line {
    func div=(const flat_weight& _rhs) -> Line& {
        this = this.flat_weight()
        return *this;
    }
}
extension Line: FlatWeight {
    func flat_weight(self) -> Line {
        return Line::from_groups(
            /* e41, e42, e43 */
            self.group0(), 
            /* e23, e31, e12 */
            float3(0.0)
        );
    }
}
extension Motor {
    func div(_rhs: flat_weight) -> 
Motor {
        this.flat_weight()
    }
}
extension Motor {
    func div=(const flat_weight& _rhs) -> Motor& {
        this = this.flat_weight()
        return *this;
    }
}
extension Motor: FlatWeight {
    func flat_weight(self) -> Motor {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            self.group0(), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
extension MultiVector {
    func div(_rhs: flat_weight) -> 
MultiVector {
        this.flat_weight()
    }
}
extension MultiVector {
    func div=(const flat_weight& _rhs) -> MultiVector& {
        this = this.flat_weight()
        return *this;
    }
}
extension MultiVector: FlatWeight {
    func flat_weight(self) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(0.0, self[e1234]), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), self[e4]), 
            /* e41, e42, e43 */
            self.group2(), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(self[e423], self[e431], self[e412], 0.0)
        );
    }
}
extension Origin {
    func div(_rhs: flat_weight) -> 
Origin {
        this.flat_weight()
    }
}
extension Origin {
    func div=(const flat_weight& _rhs) -> Origin& {
        this = this.flat_weight()
        return *this;
    }
}
extension Origin: FlatWeight {
    func flat_weight(self) -> Origin {
        return self;
    }
}
extension Plane {
    func div(_rhs: flat_weight) -> 
Plane {
        this.flat_weight()
    }
}
extension Plane {
    func div=(const flat_weight& _rhs) -> Plane& {
        this = this.flat_weight()
        return *this;
    }
}
extension Plane: FlatWeight {
    func flat_weight(self) -> Plane {
        return Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4(self[e423], self[e431], self[e412], 0.0)
        );
    }
}
extension Point {
    func div(_rhs: flat_weight) -> 
Origin {
        this.flat_weight()
    }
}
extension Point: FlatWeight {
    func flat_weight(self) -> Origin {
        return Origin::from_groups(
            /* e4 */
            self[e4]
        );
    }
}
