extension DualNum: One {
    associatedtype Output = DualNum;
    func one() -> Output {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(1.0, 0.0)
        );
    }
}
extension Motor: One {
    associatedtype Output = Motor;
    func one() -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(0.0), 
            /* e23, e31, e12, scalar */
            float4(float3(0.0), 1.0)
        );
    }
}
extension MultiVector: One {
    associatedtype Output = MultiVector;
    func one() -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(1.0, 0.0), 
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
extension Scalar: One {
    associatedtype Output = Scalar;
    func one() -> Output {
        return Scalar.from_groups(
            /* scalar */
            1.0
        );
    }
}
