extension AntiScalar: AntiOne {
    associatedtype Output = AntiScalar;
    func anti_one() -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension CircleRotor: AntiOne {
    associatedtype Output = CircleRotor;
    func anti_one() -> Output {
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e12345 */
            float4(float3(0.0), 1.0)
        );
    }
}
extension DualNum: AntiOne {
    associatedtype Output = DualNum;
    func anti_one() -> Output {
        return DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(float2(0.0, 1.0), 0.0, 0.0)
        );
    }
}
extension Motor: AntiOne {
    associatedtype Output = Motor;
    func anti_one() -> Output {
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(float3(0.0), 1.0), 
            /* e235, e315, e125, e5 */
            float4(0.0)
        );
    }
}
extension MultiVector: AntiOne {
    associatedtype Output = MultiVector;
    func anti_one() -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, 1.0), 0.0, 0.0), 
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
extension VersorEven: AntiOne {
    associatedtype Output = VersorEven;
    func anti_one() -> Output {
        return VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            float4(float3(0.0), 1.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e5 */
            float4(0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0)
        );
    }
}
