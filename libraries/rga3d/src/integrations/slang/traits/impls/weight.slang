extension AntiScalar {
    func div(_rhs: weight) -> 
AntiScalar {
        this.weight()
    }
}
extension AntiScalar {
    func div=(const weight& _rhs) -> AntiScalar& {
        this = this.weight()
        return *this;
    }
}
extension AntiScalar: Weight {
    func weight(self) -> AntiScalar {
        return self;
    }
}
extension DualNum {
    func div(_rhs: weight) -> 
AntiScalar {
        this.weight()
    }
}
extension DualNum: Weight {
    func weight(self) -> AntiScalar {
        return AntiScalar::from_groups(
            /* e1234 */
            self[e1234]
        );
    }
}
extension Flector {
    func div(_rhs: weight) -> 
Flector {
        this.weight()
    }
}
extension Flector {
    func div=(const weight& _rhs) -> Flector& {
        this = this.weight()
        return *this;
    }
}
extension Flector: Weight {
    func weight(self) -> Flector {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), self[e4]), 
            /* e423, e431, e412, e321 */
            float4(self[e423], self[e431], self[e412], 0.0)
        );
    }
}
extension Line {
    func div(_rhs: weight) -> 
Line {
        this.weight()
    }
}
extension Line {
    func div=(const weight& _rhs) -> Line& {
        this = this.weight()
        return *this;
    }
}
extension Line: Weight {
    func weight(self) -> Line {
        return Line::from_groups(
            /* e41, e42, e43 */
            self.group0(), 
            /* e23, e31, e12 */
            float3(0.0)
        );
    }
}
extension Motor {
    func div(_rhs: weight) -> 
Motor {
        this.weight()
    }
}
extension Motor {
    func div=(const weight& _rhs) -> Motor& {
        this = this.weight()
        return *this;
    }
}
extension Motor: Weight {
    func weight(self) -> Motor {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            self.group0(), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
extension MultiVector {
    func div(_rhs: weight) -> 
MultiVector {
        this.weight()
    }
}
extension MultiVector {
    func div=(const weight& _rhs) -> MultiVector& {
        this = this.weight()
        return *this;
    }
}
extension MultiVector: Weight {
    func weight(self) -> MultiVector {
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
    func div(_rhs: weight) -> 
Origin {
        this.weight()
    }
}
extension Origin {
    func div=(const weight& _rhs) -> Origin& {
        this = this.weight()
        return *this;
    }
}
extension Origin: Weight {
    func weight(self) -> Origin {
        return self;
    }
}
extension Plane {
    func div(_rhs: weight) -> 
Plane {
        this.weight()
    }
}
extension Plane {
    func div=(const weight& _rhs) -> Plane& {
        this = this.weight()
        return *this;
    }
}
extension Plane: Weight {
    func weight(self) -> Plane {
        return Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4(self[e423], self[e431], self[e412], 0.0)
        );
    }
}
extension Point {
    func div(_rhs: weight) -> 
Origin {
        this.weight()
    }
}
extension Point: Weight {
    func weight(self) -> Origin {
        return Origin::from_groups(
            /* e4 */
            self[e4]
        );
    }
}
