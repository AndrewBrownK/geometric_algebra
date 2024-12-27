extension AntiCircleRotor {
    // Fancy infix trick
    func operator/(_rhs: RoundBulkPrefixOrPostfix) -> AntiMotor {
        return this.round_bulk();
    }
}
extension AntiCircleRotor: RoundBulk {
    associatedtype Output = AntiMotor;
    func round_bulk() -> Output {
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(this.e23, this.e31, this.e12, this.scalar), 
            /* e15, e25, e35, e3215 */
            float4(0.0)
        );
    }
}
extension AntiDipoleInversion {
    // Fancy infix trick
    func operator/(_rhs: RoundBulkPrefixOrPostfix) -> AntiFlector {
        return this.round_bulk();
    }
}
extension AntiDipoleInversion: RoundBulk {
    associatedtype Output = AntiFlector;
    func round_bulk() -> Output {
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(float3(0.0), this.e321), 
            /* e1, e2, e3, e5 */
            float4(this.group3.xyz, 0.0)
        );
    }
}
extension AntiDualNum {
    // Fancy infix trick
    func operator/(_rhs: RoundBulkPrefixOrPostfix) -> Scalar {
        return this.round_bulk();
    }
}
extension AntiDualNum: RoundBulk {
    associatedtype Output = Scalar;
    func round_bulk() -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(this.scalar, 0.0, 0.0, 0.0)
        );
    }
}
extension AntiFlatPoint {
    // Fancy infix trick
    func operator/(_rhs: RoundBulkPrefixOrPostfix) -> AntiFlatPoint {
        return this.round_bulk();
    }
    func operator/=(const RoundBulkPrefixOrPostfix& _rhs) -> AntiFlatPoint& {
        this = this.round_bulk();
        return *this;
    }
}
extension AntiFlatPoint: RoundBulk {
    associatedtype Output = AntiFlatPoint;
    func round_bulk() -> Output {
        return AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            float4(float3(0.0), this.e321)
        );
    }
}
extension AntiFlector {
    // Fancy infix trick
    func operator/(_rhs: RoundBulkPrefixOrPostfix) -> AntiFlector {
        return this.round_bulk();
    }
    func operator/=(const RoundBulkPrefixOrPostfix& _rhs) -> AntiFlector& {
        this = this.round_bulk();
        return *this;
    }
}
extension AntiFlector: RoundBulk {
    associatedtype Output = AntiFlector;
    func round_bulk() -> Output {
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(float3(0.0), this.e321), 
            /* e1, e2, e3, e5 */
            float4(this.group1.xyz, 0.0)
        );
    }
}
extension AntiLine {
    // Fancy infix trick
    func operator/(_rhs: RoundBulkPrefixOrPostfix) -> AntiLine {
        return this.round_bulk();
    }
    func operator/=(const RoundBulkPrefixOrPostfix& _rhs) -> AntiLine& {
        this = this.round_bulk();
        return *this;
    }
}
extension AntiLine: RoundBulk {
    associatedtype Output = AntiLine;
    func round_bulk() -> Output {
        return AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(0.0)
        );
    }
}
extension AntiMotor {
    // Fancy infix trick
    func operator/(_rhs: RoundBulkPrefixOrPostfix) -> AntiMotor {
        return this.round_bulk();
    }
    func operator/=(const RoundBulkPrefixOrPostfix& _rhs) -> AntiMotor& {
        this = this.round_bulk();
        return *this;
    }
}
extension AntiMotor: RoundBulk {
    associatedtype Output = AntiMotor;
    func round_bulk() -> Output {
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            this.group0, 
            /* e15, e25, e35, e3215 */
            float4(0.0)
        );
    }
}
extension AntiPlane {
    // Fancy infix trick
    func operator/(_rhs: RoundBulkPrefixOrPostfix) -> AntiPlane {
        return this.round_bulk();
    }
    func operator/=(const RoundBulkPrefixOrPostfix& _rhs) -> AntiPlane& {
        this = this.round_bulk();
        return *this;
    }
}
extension AntiPlane: RoundBulk {
    associatedtype Output = AntiPlane;
    func round_bulk() -> Output {
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4(this.group0.xyz, 0.0)
        );
    }
}
extension Circle {
    // Fancy infix trick
    func operator/(_rhs: RoundBulkPrefixOrPostfix) -> AntiFlatPoint {
        return this.round_bulk();
    }
}
extension Circle: RoundBulk {
    associatedtype Output = AntiFlatPoint;
    func round_bulk() -> Output {
        return AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            float4(float3(0.0), this.e321)
        );
    }
}
extension CircleRotor {
    // Fancy infix trick
    func operator/(_rhs: RoundBulkPrefixOrPostfix) -> AntiFlatPoint {
        return this.round_bulk();
    }
}
extension CircleRotor: RoundBulk {
    associatedtype Output = AntiFlatPoint;
    func round_bulk() -> Output {
        return AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            float4(float3(0.0), this.e321)
        );
    }
}
extension Dipole {
    // Fancy infix trick
    func operator/(_rhs: RoundBulkPrefixOrPostfix) -> AntiLine {
        return this.round_bulk();
    }
}
extension Dipole: RoundBulk {
    associatedtype Output = AntiLine;
    func round_bulk() -> Output {
        return AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(this.group1.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(0.0)
        );
    }
}
extension DipoleInversion {
    // Fancy infix trick
    func operator/(_rhs: RoundBulkPrefixOrPostfix) -> AntiLine {
        return this.round_bulk();
    }
}
extension DipoleInversion: RoundBulk {
    associatedtype Output = AntiLine;
    func round_bulk() -> Output {
        return AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(this.group1.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(0.0)
        );
    }
}
extension MultiVector {
    // Fancy infix trick
    func operator/(_rhs: RoundBulkPrefixOrPostfix) -> MultiVector {
        return this.round_bulk();
    }
    func operator/=(const RoundBulkPrefixOrPostfix& _rhs) -> MultiVector& {
        this = this.round_bulk();
        return *this;
    }
}
extension MultiVector: RoundBulk {
    associatedtype Output = MultiVector;
    func round_bulk() -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(this.scalar, 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.group1.xyz, 0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group5.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(float3(0.0), this.e321), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension RoundPoint {
    // Fancy infix trick
    func operator/(_rhs: RoundBulkPrefixOrPostfix) -> AntiPlane {
        return this.round_bulk();
    }
}
extension RoundPoint: RoundBulk {
    associatedtype Output = AntiPlane;
    func round_bulk() -> Output {
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4(this.group0.xyz, 0.0)
        );
    }
}
extension Scalar {
    // Fancy infix trick
    func operator/(_rhs: RoundBulkPrefixOrPostfix) -> Scalar {
        return this.round_bulk();
    }
    func operator/=(const RoundBulkPrefixOrPostfix& _rhs) -> Scalar& {
        this = this.round_bulk();
        return *this;
    }
}
extension Scalar: RoundBulk {
    associatedtype Output = Scalar;
    func round_bulk() -> Output {
        return this;
    }
}
extension VersorEven {
    // Fancy infix trick
    func operator/(_rhs: RoundBulkPrefixOrPostfix) -> AntiFlector {
        return this.round_bulk();
    }
}
extension VersorEven: RoundBulk {
    associatedtype Output = AntiFlector;
    func round_bulk() -> Output {
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(float3(0.0), this.e321), 
            /* e1, e2, e3, e5 */
            float4(this.group3.xyz, 0.0)
        );
    }
}
extension VersorOdd {
    // Fancy infix trick
    func operator/(_rhs: RoundBulkPrefixOrPostfix) -> AntiMotor {
        return this.round_bulk();
    }
}
extension VersorOdd: RoundBulk {
    associatedtype Output = AntiMotor;
    func round_bulk() -> Output {
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(this.e23, this.e31, this.e12, this.scalar), 
            /* e15, e25, e35, e3215 */
            float4(0.0)
        );
    }
}
