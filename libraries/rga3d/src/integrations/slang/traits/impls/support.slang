extension Flector {
    // Fancy infix trick
    func operator/(_rhs: SupportPrefixOrPostfix) -> Line {
        return this.support();
    }
}
extension Flector: Support {
    associatedtype Output = Line;
    func support() -> Output {
        return Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(float4(float3(0.0), this.e321 * -1.0).xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0)
        );
    }
}
extension MultiVector {
    // Fancy infix trick
    func operator/(_rhs: SupportPrefixOrPostfix) -> Line {
        return this.support();
    }
}
extension MultiVector: Support {
    associatedtype Output = Line;
    func support() -> Output {
        return Line.from_groups(
            /* e41, e42, e43, 0 */
            float4(float4(float3(0.0), this.e321 * -1.0).xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0)
        );
    }
}
