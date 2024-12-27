extension AntiCircleRotor {
    // Fancy infix trick
    func operator/(_rhs: FlatBulkPrefixOrPostfix) -> FlatPoint {
        return this.flat_bulk();
    }
}
extension AntiCircleRotor: FlatBulk {
    associatedtype Output = FlatPoint;
    func flat_bulk() -> Output {
        return FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            float4(this.group2.xyz, 0.0)
        );
    }
}
extension AntiDipoleInversion {
    // Fancy infix trick
    func operator/(_rhs: FlatBulkPrefixOrPostfix) -> AntiFlector {
        return this.flat_bulk();
    }
}
extension AntiDipoleInversion: FlatBulk {
    associatedtype Output = AntiFlector;
    func flat_bulk() -> Output {
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(this.group2.xyz, 0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), this.e5)
        );
    }
}
extension AntiDualNum {
    // Fancy infix trick
    func operator/(_rhs: FlatBulkPrefixOrPostfix) -> AntiDualNum {
        return this.flat_bulk();
    }
    func operator/=(const FlatBulkPrefixOrPostfix& _rhs) -> AntiDualNum& {
        this = this.flat_bulk();
        return *this;
    }
}
extension AntiDualNum: FlatBulk {
    associatedtype Output = AntiDualNum;
    func flat_bulk() -> Output {
        return AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(float2(this.e3215, 0.0), 0.0, 0.0)
        );
    }
}
extension AntiFlatPoint {
    // Fancy infix trick
    func operator/(_rhs: FlatBulkPrefixOrPostfix) -> AntiFlatPoint {
        return this.flat_bulk();
    }
    func operator/=(const FlatBulkPrefixOrPostfix& _rhs) -> AntiFlatPoint& {
        this = this.flat_bulk();
        return *this;
    }
}
extension AntiFlatPoint: FlatBulk {
    associatedtype Output = AntiFlatPoint;
    func flat_bulk() -> Output {
        return AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            float4(this.group0.xyz, 0.0)
        );
    }
}
extension AntiFlector {
    // Fancy infix trick
    func operator/(_rhs: FlatBulkPrefixOrPostfix) -> AntiFlector {
        return this.flat_bulk();
    }
    func operator/=(const FlatBulkPrefixOrPostfix& _rhs) -> AntiFlector& {
        this = this.flat_bulk();
        return *this;
    }
}
extension AntiFlector: FlatBulk {
    associatedtype Output = AntiFlector;
    func flat_bulk() -> Output {
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(this.group0.xyz, 0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), this.e5)
        );
    }
}
extension AntiLine {
    // Fancy infix trick
    func operator/(_rhs: FlatBulkPrefixOrPostfix) -> FlatPoint {
        return this.flat_bulk();
    }
}
extension AntiLine: FlatBulk {
    associatedtype Output = FlatPoint;
    func flat_bulk() -> Output {
        return FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            float4(this.group1.xyz, 0.0)
        );
    }
}
extension AntiMotor {
    // Fancy infix trick
    func operator/(_rhs: FlatBulkPrefixOrPostfix) -> Flector {
        return this.flat_bulk();
    }
}
extension AntiMotor: FlatBulk {
    associatedtype Output = Flector;
    func flat_bulk() -> Output {
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(this.group1.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), this.e3215)
        );
    }
}
extension AntiPlane {
    // Fancy infix trick
    func operator/(_rhs: FlatBulkPrefixOrPostfix) -> DualNum {
        return this.flat_bulk();
    }
}
extension AntiPlane: FlatBulk {
    associatedtype Output = DualNum;
    func flat_bulk() -> Output {
        return DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(float2(this.e5, 0.0), 0.0, 0.0)
        );
    }
}
extension Circle {
    // Fancy infix trick
    func operator/(_rhs: FlatBulkPrefixOrPostfix) -> AntiFlatPoint {
        return this.flat_bulk();
    }
}
extension Circle: FlatBulk {
    associatedtype Output = AntiFlatPoint;
    func flat_bulk() -> Output {
        return AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            float4(this.group2.xyz, 0.0)
        );
    }
}
extension CircleRotor {
    // Fancy infix trick
    func operator/(_rhs: FlatBulkPrefixOrPostfix) -> AntiFlatPoint {
        return this.flat_bulk();
    }
}
extension CircleRotor: FlatBulk {
    associatedtype Output = AntiFlatPoint;
    func flat_bulk() -> Output {
        return AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            float4(this.group2.xyz, 0.0)
        );
    }
}
extension Dipole {
    // Fancy infix trick
    func operator/(_rhs: FlatBulkPrefixOrPostfix) -> FlatPoint {
        return this.flat_bulk();
    }
}
extension Dipole: FlatBulk {
    associatedtype Output = FlatPoint;
    func flat_bulk() -> Output {
        return FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            float4(this.group2.xyz, 0.0)
        );
    }
}
extension DipoleInversion {
    // Fancy infix trick
    func operator/(_rhs: FlatBulkPrefixOrPostfix) -> Flector {
        return this.flat_bulk();
    }
}
extension DipoleInversion: FlatBulk {
    associatedtype Output = Flector;
    func flat_bulk() -> Output {
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(this.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), this.e3215)
        );
    }
}
extension DualNum {
    // Fancy infix trick
    func operator/(_rhs: FlatBulkPrefixOrPostfix) -> DualNum {
        return this.flat_bulk();
    }
    func operator/=(const FlatBulkPrefixOrPostfix& _rhs) -> DualNum& {
        this = this.flat_bulk();
        return *this;
    }
}
extension DualNum: FlatBulk {
    associatedtype Output = DualNum;
    func flat_bulk() -> Output {
        return DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(float2(this.e5, 0.0), 0.0, 0.0)
        );
    }
}
extension FlatPoint {
    // Fancy infix trick
    func operator/(_rhs: FlatBulkPrefixOrPostfix) -> FlatPoint {
        return this.flat_bulk();
    }
    func operator/=(const FlatBulkPrefixOrPostfix& _rhs) -> FlatPoint& {
        this = this.flat_bulk();
        return *this;
    }
}
extension FlatPoint: FlatBulk {
    associatedtype Output = FlatPoint;
    func flat_bulk() -> Output {
        return FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            float4(this.group0.xyz, 0.0)
        );
    }
}
extension Flector {
    // Fancy infix trick
    func operator/(_rhs: FlatBulkPrefixOrPostfix) -> Flector {
        return this.flat_bulk();
    }
    func operator/=(const FlatBulkPrefixOrPostfix& _rhs) -> Flector& {
        this = this.flat_bulk();
        return *this;
    }
}
extension Flector: FlatBulk {
    associatedtype Output = Flector;
    func flat_bulk() -> Output {
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(this.group0.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), this.e3215)
        );
    }
}
extension Line {
    // Fancy infix trick
    func operator/(_rhs: FlatBulkPrefixOrPostfix) -> AntiFlatPoint {
        return this.flat_bulk();
    }
}
extension Line: FlatBulk {
    associatedtype Output = AntiFlatPoint;
    func flat_bulk() -> Output {
        return AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            float4(this.group1.xyz, 0.0)
        );
    }
}
extension Motor {
    // Fancy infix trick
    func operator/(_rhs: FlatBulkPrefixOrPostfix) -> AntiFlector {
        return this.flat_bulk();
    }
}
extension Motor: FlatBulk {
    associatedtype Output = AntiFlector;
    func flat_bulk() -> Output {
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(this.group1.xyz, 0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), this.e5)
        );
    }
}
extension MultiVector {
    // Fancy infix trick
    func operator/(_rhs: FlatBulkPrefixOrPostfix) -> MultiVector {
        return this.flat_bulk();
    }
    func operator/=(const FlatBulkPrefixOrPostfix& _rhs) -> MultiVector& {
        this = this.flat_bulk();
        return *this;
    }
}
extension MultiVector: FlatBulk {
    associatedtype Output = MultiVector;
    func flat_bulk() -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(this.e5, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(this.group3.xyz, 0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group8.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), this.e3215), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension Plane {
    // Fancy infix trick
    func operator/(_rhs: FlatBulkPrefixOrPostfix) -> AntiDualNum {
        return this.flat_bulk();
    }
}
extension Plane: FlatBulk {
    associatedtype Output = AntiDualNum;
    func flat_bulk() -> Output {
        return AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(float2(this.e3215, 0.0), 0.0, 0.0)
        );
    }
}
extension RoundPoint {
    // Fancy infix trick
    func operator/(_rhs: FlatBulkPrefixOrPostfix) -> DualNum {
        return this.flat_bulk();
    }
}
extension RoundPoint: FlatBulk {
    associatedtype Output = DualNum;
    func flat_bulk() -> Output {
        return DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(float2(this.e5, 0.0), 0.0, 0.0)
        );
    }
}
extension Sphere {
    // Fancy infix trick
    func operator/(_rhs: FlatBulkPrefixOrPostfix) -> AntiDualNum {
        return this.flat_bulk();
    }
}
extension Sphere: FlatBulk {
    associatedtype Output = AntiDualNum;
    func flat_bulk() -> Output {
        return AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(float2(this.e3215, 0.0), 0.0, 0.0)
        );
    }
}
extension VersorEven {
    // Fancy infix trick
    func operator/(_rhs: FlatBulkPrefixOrPostfix) -> AntiFlector {
        return this.flat_bulk();
    }
}
extension VersorEven: FlatBulk {
    associatedtype Output = AntiFlector;
    func flat_bulk() -> Output {
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(this.group2.xyz, 0.0), 
            /* e1, e2, e3, e5 */
            float4(float3(0.0), this.e5)
        );
    }
}
extension VersorOdd {
    // Fancy infix trick
    func operator/(_rhs: FlatBulkPrefixOrPostfix) -> Flector {
        return this.flat_bulk();
    }
}
extension VersorOdd: FlatBulk {
    associatedtype Output = Flector;
    func flat_bulk() -> Output {
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(this.group2.xyz, 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), this.e3215)
        );
    }
}
