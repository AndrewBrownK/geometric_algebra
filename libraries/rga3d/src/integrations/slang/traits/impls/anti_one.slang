extension AntiScalar: AntiOne {
    associatedtype Output = AntiScalar;
    func anti_one() -> Output {
        return AntiScalar.from_groups(
            /* e1234 */
            1.0
        );
    }
}
extension DualNum: AntiOne {
    associatedtype Output = DualNum;
    func anti_one() -> Output {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(0.0, 1.0)
        );
    }
}
extension Motor: AntiOne {
    associatedtype Output = Motor;
    func anti_one() -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(float3(0.0), 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
extension MultiVector: AntiOne {
    associatedtype Output = MultiVector;
    func anti_one() -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
