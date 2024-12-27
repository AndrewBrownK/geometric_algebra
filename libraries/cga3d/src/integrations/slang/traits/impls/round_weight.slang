extension AntiCircleRotor {
    // Fancy infix trick
    func operator/(_rhs: RoundWeightPrefixOrPostfix) -> Dipole {
        return this.round_weight();
    }
}
extension AntiCircleRotor: RoundWeight {
    associatedtype Output = Dipole;
    func round_weight() -> Output {
        return Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(0.0), 
            /* e15, e25, e35, 0 */
            float4(0.0)
        );
    }
}
extension AntiDipoleInversion {
    // Fancy infix trick
    func operator/(_rhs: RoundWeightPrefixOrPostfix) -> AntiDipoleInversion {
        return this.round_weight();
    }
    func operator/=(const RoundWeightPrefixOrPostfix& _rhs) -> AntiDipoleInversion& {
        this = this.round_weight();
        return *this;
    }
}
extension AntiDipoleInversion: RoundWeight {
    associatedtype Output = AntiDipoleInversion;
    func round_weight() -> Output {
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e4 */
            float4(float3(0.0), this.e4), 
            /* e1, e2, e3, e5 */
            float4(0.0)
        );
    }
}
extension Circle {
    // Fancy infix trick
    func operator/(_rhs: RoundWeightPrefixOrPostfix) -> Circle {
        return this.round_weight();
    }
    func operator/=(const RoundWeightPrefixOrPostfix& _rhs) -> Circle& {
        this = this.round_weight();
        return *this;
    }
}
extension Circle: RoundWeight {
    associatedtype Output = Circle;
    func round_weight() -> Output {
        return Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(0.0)
        );
    }
}
extension CircleRotor {
    // Fancy infix trick
    func operator/(_rhs: RoundWeightPrefixOrPostfix) -> Circle {
        return this.round_weight();
    }
}
extension CircleRotor: RoundWeight {
    associatedtype Output = Circle;
    func round_weight() -> Output {
        return Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(0.0)
        );
    }
}
extension Dipole {
    // Fancy infix trick
    func operator/(_rhs: RoundWeightPrefixOrPostfix) -> Dipole {
        return this.round_weight();
    }
    func operator/=(const RoundWeightPrefixOrPostfix& _rhs) -> Dipole& {
        this = this.round_weight();
        return *this;
    }
}
extension Dipole: RoundWeight {
    associatedtype Output = Dipole;
    func round_weight() -> Output {
        return Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(0.0), 
            /* e15, e25, e35, 0 */
            float4(0.0)
        );
    }
}
extension DipoleInversion {
    // Fancy infix trick
    func operator/(_rhs: RoundWeightPrefixOrPostfix) -> DipoleInversion {
        return this.round_weight();
    }
    func operator/=(const RoundWeightPrefixOrPostfix& _rhs) -> DipoleInversion& {
        this = this.round_weight();
        return *this;
    }
}
extension DipoleInversion: RoundWeight {
    associatedtype Output = DipoleInversion;
    func round_weight() -> Output {
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(0.0), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), this.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0)
        );
    }
}
extension MultiVector {
    // Fancy infix trick
    func operator/(_rhs: RoundWeightPrefixOrPostfix) -> MultiVector {
        return this.round_weight();
    }
    func operator/=(const RoundWeightPrefixOrPostfix& _rhs) -> MultiVector& {
        this = this.round_weight();
        return *this;
    }
}
extension MultiVector: RoundWeight {
    associatedtype Output = MultiVector;
    func round_weight() -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e4), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43, 0 */
            float4(this.group4.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412, 0 */
            float4(this.group7.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension RoundPoint {
    // Fancy infix trick
    func operator/(_rhs: RoundWeightPrefixOrPostfix) -> RoundPoint {
        return this.round_weight();
    }
    func operator/=(const RoundWeightPrefixOrPostfix& _rhs) -> RoundPoint& {
        this = this.round_weight();
        return *this;
    }
}
extension RoundPoint: RoundWeight {
    associatedtype Output = RoundPoint;
    func round_weight() -> Output {
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e4), 
            /* e5, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension Sphere {
    // Fancy infix trick
    func operator/(_rhs: RoundWeightPrefixOrPostfix) -> Sphere {
        return this.round_weight();
    }
    func operator/=(const RoundWeightPrefixOrPostfix& _rhs) -> Sphere& {
        this = this.round_weight();
        return *this;
    }
}
extension Sphere: RoundWeight {
    associatedtype Output = Sphere;
    func round_weight() -> Output {
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(this.e1234, 0.0, 0.0, 0.0)
        );
    }
}
extension VersorEven {
    // Fancy infix trick
    func operator/(_rhs: RoundWeightPrefixOrPostfix) -> AntiDipoleInversion {
        return this.round_weight();
    }
}
extension VersorEven: RoundWeight {
    associatedtype Output = AntiDipoleInversion;
    func round_weight() -> Output {
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e4 */
            float4(float3(0.0), this.e4), 
            /* e1, e2, e3, e5 */
            float4(0.0)
        );
    }
}
extension VersorOdd {
    // Fancy infix trick
    func operator/(_rhs: RoundWeightPrefixOrPostfix) -> DipoleInversion {
        return this.round_weight();
    }
}
extension VersorOdd: RoundWeight {
    associatedtype Output = DipoleInversion;
    func round_weight() -> Output {
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(0.0), 
            /* e15, e25, e35, e1234 */
            float4(float3(0.0), this.e1234), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0)
        );
    }
}
