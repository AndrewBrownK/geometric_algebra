extension DualNum {
    func div(_rhs: anti_constraint_violation) -> 
Scalar {
        this.anti_constraint_violation()
    }
}
extension DualNum: AntiConstraintViolation {
    func anti_constraint_violation(self) -> Scalar {
        return Scalar::from_groups(
            /* scalar */
            self[scalar] * self[e1234] * 2.0
        );
    }
}
extension Flector {
    func div(_rhs: anti_constraint_violation) -> 
Scalar {
        this.anti_constraint_violation()
    }
}
extension Flector: AntiConstraintViolation {
    func anti_constraint_violation(self) -> Scalar {
Flector anti_reverse = Flector::from_groups(
            /* e1, e2, e3, e4 */
            self.group0() * float4(-1.0), 
            /* e423, e431, e412, e321 */
            self.group1()
        );
        return Scalar::from_groups(
            /* scalar */
            (anti_reverse[e423] * self[e1]) + (anti_reverse[e431] * self[e2]) + (anti_reverse[e412] * self[e3]) + (anti_reverse[e321] * self[e4]) - (anti_reverse[e1] * self[e423]) - (anti_reverse[e2] * self[e431]) - (anti_reverse[e3] * self[e412]) - (anti_reverse[e4] * self[e321])
        );
    }
}
extension Line {
    func div(_rhs: anti_constraint_violation) -> 
Scalar {
        this.anti_constraint_violation()
    }
}
extension Line: AntiConstraintViolation {
    func anti_constraint_violation(self) -> Scalar {
Line anti_reverse = Line::from_groups(
            /* e41, e42, e43 */
            self.group0() * float3(-1.0), 
            /* e23, e31, e12 */
            self.group1() * float3(-1.0)
        );
        return Scalar::from_groups(
            /* scalar */
            -(anti_reverse[e41] * self[e23]) - (anti_reverse[e42] * self[e31]) - (anti_reverse[e43] * self[e12]) - (anti_reverse[e23] * self[e41]) - (anti_reverse[e31] * self[e42]) - (anti_reverse[e12] * self[e43])
        );
    }
}
extension Motor {
    func div(_rhs: anti_constraint_violation) -> 
Scalar {
        this.anti_constraint_violation()
    }
}
extension Motor: AntiConstraintViolation {
    func anti_constraint_violation(self) -> Scalar {
Motor anti_reverse = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            self.group0() * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            self.group1() * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Scalar::from_groups(
            /* scalar */
            (anti_reverse[e1234] * self[scalar]) + (anti_reverse[scalar] * self[e1234]) - (anti_reverse[e41] * self[e23]) - (anti_reverse[e42] * self[e31]) - (anti_reverse[e43] * self[e12]) - (anti_reverse[e23] * self[e41]) - (anti_reverse[e31] * self[e42]) - (anti_reverse[e12] * self[e43])
        );
    }
}
extension MultiVector {
    func div(_rhs: anti_constraint_violation) -> 
MultiVector {
        this.anti_constraint_violation()
    }
}
extension MultiVector {
    func div=(const anti_constraint_violation& _rhs) -> MultiVector& {
        this = this.anti_constraint_violation()
        return *this;
    }
}
extension MultiVector: AntiConstraintViolation {
    func anti_constraint_violation(self) -> MultiVector {
MultiVector anti_reverse = MultiVector::from_groups(
            /* scalar, e1234 */
            self.group0(), 
            /* e1, e2, e3, e4 */
            self.group1() * float4(-1.0), 
            /* e41, e42, e43 */
            self.group2() * float3(-1.0), 
            /* e23, e31, e12 */
            self.group3() * float3(-1.0), 
            /* e423, e431, e412, e321 */
            self.group4()
        );
MultiVector geometric_anti_product = MultiVector::from_groups(
            /* scalar, e1234 */
            float2((anti_reverse[e1234] * self[scalar]) + (anti_reverse[e321] * self[e4]) - (anti_reverse[e2] * self[e431]) - (anti_reverse[e3] * self[e412]) - (anti_reverse[e4] * self[e321]) - (anti_reverse[e23] * self[e41]) - (anti_reverse[e31] * self[e42]) - (anti_reverse[e12] * self[e43]), 0.0) + (float2(anti_reverse[e423]) * float2(self[e1], self[e423])) + (float2(anti_reverse[e431]) * float2(self[e2], self[e431])) + (float2(anti_reverse[e412]) * float2(self[e3], self[e412])) + (float2(self[e1234]) * anti_reverse.group0()) - (float2(anti_reverse[e41]) * float2(self[e23], self[e41])) - (float2(anti_reverse[e42]) * float2(self[e31], self[e42])) - (float2(anti_reverse[e43]) * float2(self[e12], self[e43])) - (float2(self[e423], self[e4]) * anti_reverse.group1().xw), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            (float4(anti_reverse[e1234]) * self.group4()) + (float4(self[e4], self[e412], self[e423], anti_reverse[e321]) * float4(anti_reverse.group2().xxy, self[e1234])) + (float4(self[e431], self[e4], self[e4], self[e423]) * float4(anti_reverse.group2().zyz, anti_reverse[e23])) + (float4(self.group0().yy, self[e1234], self[scalar]) * float4(anti_reverse.group4().xyz, anti_reverse[e4])) + (float4(anti_reverse.group1().ww, anti_reverse[e431], self[e431]) * float4(self.group2().xyx, anti_reverse[e31])) + (float4(anti_reverse.group4().zx, anti_reverse[e4], self[e412]) * float4(self.group2().yzz, anti_reverse[e12])) + float4(float3(0.0), -(anti_reverse[e1] * self[e41]) - (anti_reverse[e2] * self[e42]) - (anti_reverse[e3] * self[e43]) - (anti_reverse[e42] * self[e2]) - (anti_reverse[e43] * self[e3]) - (anti_reverse[e423] * self[e23]) - (anti_reverse[e431] * self[e31]) - (anti_reverse[e412] * self[e12])) - float4((anti_reverse.group2().yzx * self.group4().zxy), anti_reverse[scalar] * self[e4]) - float4((self.group2().zxy * anti_reverse.group4().yzx), anti_reverse[e41] * self[e1])
        );
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(geometric_anti_product[scalar], 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            geometric_anti_product.group4()
        );
    }
}
