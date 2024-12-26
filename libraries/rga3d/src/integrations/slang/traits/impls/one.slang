extension DualNum: One {
    func one() -> DualNum {
        return DualNum::from_groups(
            /* scalar, e1234 */
            float2(1.0, 0.0)
        );
    }
}
extension Motor: One {
    func one() -> Motor {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(0.0), 
            /* e23, e31, e12, scalar */
            float4(float3(0.0), 1.0)
        );
    }
}
extension MultiVector: One {
    func one() -> MultiVector {
        return MultiVector::from_groups(
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
    func one() -> Scalar {
        return Scalar::from_groups(
            /* scalar */
            1.0
        );
    }
}
