extension AntiCircleRotor {
    // Fancy infix trick
    func operator/(_rhs: FlatWeightPrefixOrPostfix) -> FlatPoint {
        return this.flat_weight();
    }
}
extension AntiCircleRotor: FlatWeight {
    associatedtype Output = FlatPoint;
    func flat_weight() -> Output {
        return FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            float4(float3(0.0), this.e45)
        );
    }
}
extension AntiDipoleInversion {
    // Fancy infix trick
    func operator/(_rhs: FlatWeightPrefixOrPostfix) -> Line {
        return this.flat_weight();
    }
}
extension AntiDipoleInversion: FlatWeight {
    associatedtype Output = Line;
    func flat_weight() -> Output {
        return Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(this.group1.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(0.0)
        );
    }
}
extension AntiScalar {
    // Fancy infix trick
    func operator/(_rhs: FlatWeightPrefixOrPostfix) -> AntiScalar {
        return this.flat_weight();
    }
    func operator/=(const FlatWeightPrefixOrPostfix& _rhs) -> AntiScalar& {
        this = this.flat_weight();
        return *this;
    }
}
extension AntiScalar: FlatWeight {
    associatedtype Output = AntiScalar;
    func flat_weight() -> Output {
        return this;
    }
}
extension Circle {
    // Fancy infix trick
    func operator/(_rhs: FlatWeightPrefixOrPostfix) -> Line {
        return this.flat_weight();
    }
}
extension Circle: FlatWeight {
    associatedtype Output = Line;
    func flat_weight() -> Output {
        return Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(this.group1.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(0.0)
        );
    }
}
extension CircleRotor {
    // Fancy infix trick
    func operator/(_rhs: FlatWeightPrefixOrPostfix) -> Motor {
        return this.flat_weight();
    }
}
extension CircleRotor: FlatWeight {
    associatedtype Output = Motor;
    func flat_weight() -> Output {
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(this.e415, this.e425, this.e435, this.e12345), 
            /* e235, e315, e125, e5 */
            float4(0.0)
        );
    }
}
extension Dipole {
    // Fancy infix trick
    func operator/(_rhs: FlatWeightPrefixOrPostfix) -> FlatPoint {
        return this.flat_weight();
    }
}
extension Dipole: FlatWeight {
    associatedtype Output = FlatPoint;
    func flat_weight() -> Output {
        return FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            float4(float3(0.0), this.e45)
        );
    }
}
extension DipoleInversion {
    // Fancy infix trick
    func operator/(_rhs: FlatWeightPrefixOrPostfix) -> Flector {
        return this.flat_weight();
    }
}
extension DipoleInversion: FlatWeight {
    associatedtype Output = Flector;
    func flat_weight() -> Output {
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(float3(0.0), this.e45), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.group3.xyz, 0.0)
        );
    }
}
extension DualNum {
    // Fancy infix trick
    func operator/(_rhs: FlatWeightPrefixOrPostfix) -> AntiScalar {
        return this.flat_weight();
    }
}
extension DualNum: FlatWeight {
    associatedtype Output = AntiScalar;
    func flat_weight() -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(this.e12345, 0.0, 0.0, 0.0)
        );
    }
}
extension FlatPoint {
    // Fancy infix trick
    func operator/(_rhs: FlatWeightPrefixOrPostfix) -> FlatPoint {
        return this.flat_weight();
    }
    func operator/=(const FlatWeightPrefixOrPostfix& _rhs) -> FlatPoint& {
        this = this.flat_weight();
        return *this;
    }
}
extension FlatPoint: FlatWeight {
    associatedtype Output = FlatPoint;
    func flat_weight() -> Output {
        return FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            float4(float3(0.0), this.e45)
        );
    }
}
extension Flector {
    // Fancy infix trick
    func operator/(_rhs: FlatWeightPrefixOrPostfix) -> Flector {
        return this.flat_weight();
    }
    func operator/=(const FlatWeightPrefixOrPostfix& _rhs) -> Flector& {
        this = this.flat_weight();
        return *this;
    }
}
extension Flector: FlatWeight {
    associatedtype Output = Flector;
    func flat_weight() -> Output {
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(float3(0.0), this.e45), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.group1.xyz, 0.0)
        );
    }
}
extension Line {
    // Fancy infix trick
    func operator/(_rhs: FlatWeightPrefixOrPostfix) -> Line {
        return this.flat_weight();
    }
    func operator/=(const FlatWeightPrefixOrPostfix& _rhs) -> Line& {
        this = this.flat_weight();
        return *this;
    }
}
extension Line: FlatWeight {
    associatedtype Output = Line;
    func flat_weight() -> Output {
        return Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e235, e315, e125, 0 */
            float4(0.0)
        );
    }
}
extension Motor {
    // Fancy infix trick
    func operator/(_rhs: FlatWeightPrefixOrPostfix) -> Motor {
        return this.flat_weight();
    }
    func operator/=(const FlatWeightPrefixOrPostfix& _rhs) -> Motor& {
        this = this.flat_weight();
        return *this;
    }
}
extension Motor: FlatWeight {
    associatedtype Output = Motor;
    func flat_weight() -> Output {
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            this.group0, 
            /* e235, e315, e125, e5 */
            float4(0.0)
        );
    }
}
extension MultiVector {
    // Fancy infix trick
    func operator/(_rhs: FlatWeightPrefixOrPostfix) -> MultiVector {
        return this.flat_weight();
    }
    func operator/=(const FlatWeightPrefixOrPostfix& _rhs) -> MultiVector& {
        this = this.flat_weight();
        return *this;
    }
}
extension MultiVector: FlatWeight {
    associatedtype Output = MultiVector;
    func flat_weight() -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, this.e12345), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
            /* e15, e25, e35, e45 */
            float4(float3(0.0), this.e45), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(this.group6.xyz, 0.0), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.group9.xyz, 0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension Plane {
    // Fancy infix trick
    func operator/(_rhs: FlatWeightPrefixOrPostfix) -> Plane {
        return this.flat_weight();
    }
    func operator/=(const FlatWeightPrefixOrPostfix& _rhs) -> Plane& {
        this = this.flat_weight();
        return *this;
    }
}
extension Plane: FlatWeight {
    associatedtype Output = Plane;
    func flat_weight() -> Output {
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(this.group0.xyz, 0.0)
        );
    }
}
extension Sphere {
    // Fancy infix trick
    func operator/(_rhs: FlatWeightPrefixOrPostfix) -> Plane {
        return this.flat_weight();
    }
}
extension Sphere: FlatWeight {
    associatedtype Output = Plane;
    func flat_weight() -> Output {
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(this.group0.xyz, 0.0)
        );
    }
}
extension VersorEven {
    // Fancy infix trick
    func operator/(_rhs: FlatWeightPrefixOrPostfix) -> Motor {
        return this.flat_weight();
    }
}
extension VersorEven: FlatWeight {
    associatedtype Output = Motor;
    func flat_weight() -> Output {
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(this.e415, this.e425, this.e435, this.e12345), 
            /* e235, e315, e125, e5 */
            float4(0.0)
        );
    }
}
extension VersorOdd {
    // Fancy infix trick
    func operator/(_rhs: FlatWeightPrefixOrPostfix) -> Flector {
        return this.flat_weight();
    }
}
extension VersorOdd: FlatWeight {
    associatedtype Output = Flector;
    func flat_weight() -> Output {
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(float3(0.0), this.e45), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.group3.xyz, 0.0)
        );
    }
}
