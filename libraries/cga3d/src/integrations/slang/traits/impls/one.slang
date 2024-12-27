extension AntiCircleRotor: One {
    associatedtype Output = AntiCircleRotor;
    func one() -> Output {
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, e45 */
            float4(0.0), 
            /* e15, e25, e35, scalar */
            float4(float3(0.0), 1.0)
        );
    }
}
extension AntiDualNum: One {
    associatedtype Output = AntiDualNum;
    func one() -> Output {
        return AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(float2(0.0, 1.0), 0.0, 0.0)
        );
    }
}
extension AntiMotor: One {
    associatedtype Output = AntiMotor;
    func one() -> Output {
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(float3(0.0), 1.0), 
            /* e15, e25, e35, e3215 */
            float4(0.0)
        );
    }
}
extension MultiVector: One {
    associatedtype Output = MultiVector;
    func one() -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(1.0, 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
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
extension Scalar: One {
    associatedtype Output = Scalar;
    func one() -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension VersorOdd: One {
    associatedtype Output = VersorOdd;
    func one() -> Output {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            float4(float3(0.0), 1.0), 
            /* e23, e31, e12, e45 */
            float4(0.0), 
            /* e15, e25, e35, e1234 */
            float4(0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0)
        );
    }
}
