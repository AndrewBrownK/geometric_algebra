extension Flector {
    func div(_rhs: support) -> 
Line {
        this.support()
    }
}
extension Flector: Support {
    func support(self) -> Line {
        return Line::from_groups(
            /* e41, e42, e43 */
            float4(float3(0.0), self[e321] * -1.0).xyz, 
            /* e23, e31, e12 */
            float3(0.0)
        );
    }
}
extension MultiVector {
    func div(_rhs: support) -> 
Line {
        this.support()
    }
}
extension MultiVector: Support {
    func support(self) -> Line {
        return Line::from_groups(
            /* e41, e42, e43 */
            float4(float3(0.0), self[e321] * -1.0).xyz, 
            /* e23, e31, e12 */
            float3(0.0)
        );
    }
}
