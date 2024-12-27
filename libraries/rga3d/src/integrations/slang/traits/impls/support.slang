extension Flector: Support {
    associatedtype Output = Line;
    func support() -> Output {
        return Line.from_groups(
            /* e41, e42, e43 */
            float4(float3(0.0), this.e321 * -1.0).xyz, 
            /* e23, e31, e12 */
            float3(0.0)
        );
    }
    func operator/(_rhs: SupportInfix) -> Line {
        return this.support();
    }
}
extension MultiVector: Support {
    associatedtype Output = Line;
    func support() -> Output {
        return Line.from_groups(
            /* e41, e42, e43 */
            float4(float3(0.0), this.e321 * -1.0).xyz, 
            /* e23, e31, e12 */
            float3(0.0)
        );
    }
    func operator/(_rhs: SupportInfix) -> Line {
        return this.support();
    }
}
