extension DualNum {
    func div(_rhs: constraint_violation) -> 
AntiScalar {
        this.constraint_violation()
    }
}
extension DualNum: ConstraintViolation {
    func constraint_violation(self) -> AntiScalar {
        return AntiScalar::from_groups(
            /* e1234 */
            self[scalar] * self[e1234] * 2.0
        );
    }
}
extension Flector {
    func div(_rhs: constraint_violation) -> 
AntiScalar {
        this.constraint_violation()
    }
}
extension Flector: ConstraintViolation {
    func constraint_violation(self) -> AntiScalar {
Flector reverse = Flector::from_groups(
            /* e1, e2, e3, e4 */
            self.group0(), 
            /* e423, e431, e412, e321 */
            self.group1() * float4(-1.0)
        );
        return AntiScalar::from_groups(
            /* e1234 */
            (reverse[e423] * self[e1]) + (reverse[e431] * self[e2]) + (reverse[e412] * self[e3]) + (reverse[e321] * self[e4]) - (reverse[e1] * self[e423]) - (reverse[e2] * self[e431]) - (reverse[e3] * self[e412]) - (reverse[e4] * self[e321])
        );
    }
}
extension Line {
    func div(_rhs: constraint_violation) -> 
AntiScalar {
        this.constraint_violation()
    }
}
extension Line: ConstraintViolation {
    func constraint_violation(self) -> AntiScalar {
Line reverse = Line::from_groups(
            /* e41, e42, e43 */
            self.group0() * float3(-1.0), 
            /* e23, e31, e12 */
            self.group1() * float3(-1.0)
        );
        return AntiScalar::from_groups(
            /* e1234 */
            -(reverse[e41] * self[e23]) - (reverse[e42] * self[e31]) - (reverse[e43] * self[e12]) - (reverse[e23] * self[e41]) - (reverse[e31] * self[e42]) - (reverse[e12] * self[e43])
        );
    }
}
extension Motor {
    func div(_rhs: constraint_violation) -> 
AntiScalar {
        this.constraint_violation()
    }
}
extension Motor: ConstraintViolation {
    func constraint_violation(self) -> AntiScalar {
Motor reverse = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            self.group0() * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            self.group1() * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiScalar::from_groups(
            /* e1234 */
            (reverse[e1234] * self[scalar]) + (reverse[scalar] * self[e1234]) - (reverse[e41] * self[e23]) - (reverse[e42] * self[e31]) - (reverse[e43] * self[e12]) - (reverse[e23] * self[e41]) - (reverse[e31] * self[e42]) - (reverse[e12] * self[e43])
        );
    }
}
extension MultiVector {
    func div(_rhs: constraint_violation) -> 
MultiVector {
        this.constraint_violation()
    }
}
extension MultiVector {
    func div=(const constraint_violation& _rhs) -> MultiVector& {
        this = this.constraint_violation()
        return *this;
    }
}
extension MultiVector: ConstraintViolation {
    func constraint_violation(self) -> MultiVector {
MultiVector reverse = MultiVector::from_groups(
            /* scalar, e1234 */
            self.group0(), 
            /* e1, e2, e3, e4 */
            self.group1(), 
            /* e41, e42, e43 */
            self.group2() * float3(-1.0), 
            /* e23, e31, e12 */
            self.group3() * float3(-1.0), 
            /* e423, e431, e412, e321 */
            self.group4() * float4(-1.0)
        );
MultiVector geometric_product = MultiVector::from_groups(
            /* scalar, e1234 */
            float2(0.0, (reverse[e1234] * self[scalar]) + (reverse[e321] * self[e4]) - (reverse[e2] * self[e431]) - (reverse[e3] * self[e412]) - (reverse[e4] * self[e321]) - (reverse[e23] * self[e41]) - (reverse[e31] * self[e42]) - (reverse[e12] * self[e43])) + (float2(reverse[scalar]) * self.group0()) + (float2(self[e1]) * float2(reverse[e1], reverse[e423])) + (float2(self[e2]) * float2(reverse[e2], reverse[e431])) + (float2(self[e3]) * float2(reverse[e3], reverse[e412])) - (float2(self[e23]) * float2(reverse[e23], reverse[e41])) - (float2(self[e31]) * float2(reverse[e31], reverse[e42])) - (float2(self[e12]) * float2(reverse[e12], reverse[e43])) - (float2(reverse[e321], reverse[e1]) * self.group4().wx), 
            /* e1, e2, e3, e4 */
            (float4(reverse[scalar]) * self.group1()) + (float4(reverse[e2], reverse[e321], reverse[e321], reverse[e3]) * float4(self.group3().zyz, self[e43])) + (float4(reverse[e321], reverse[e3], reverse[e1], reverse[e2]) * float4(self.group3().xxy, self[e42])) + (float4(self.group0().xx, self[scalar], reverse[e1234]) * float4(reverse.group1().xyz, self[e321])) + (float4(self.group1().zx, self[e321], reverse[e1]) * float4(reverse.group3().yzz, self[e41])) + (float4(self.group4().ww, self[e2], reverse[e4]) * float4(reverse.group3().xyx, self[scalar])) + float4(float3(0.0), -(reverse[e42] * self[e2]) - (reverse[e43] * self[e3]) - (reverse[e23] * self[e423]) - (reverse[e31] * self[e431]) - (reverse[e12] * self[e412]) - (reverse[e423] * self[e23]) - (reverse[e431] * self[e31]) - (reverse[e412] * self[e12])) - float4((reverse.group3().zxy * self.group1().yzx), reverse[e321] * self[e1234]) - float4((self.group3().yzx * reverse.group1().zxy), reverse[e41] * self[e1]), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(0.0, geometric_product[e1234]), 
            /* e1, e2, e3, e4 */
            geometric_product.group1(), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
