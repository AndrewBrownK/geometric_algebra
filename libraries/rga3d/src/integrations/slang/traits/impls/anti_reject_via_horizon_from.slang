extension AntiScalar {
    func div(_rhs: anti_reject_via_horizon_from) -> anti_reject_via_horizon_from_partial<AntiScalar> {
        anti_reject_via_horizon_from_partial(this)
    }
}
extension AntiScalar: AntiRejectViaHorizonFrom<DualNum> {
    func anti_reject_via_horizon_from(self, other: DualNum) -> AntiScalar {
        return AntiScalar::from_groups(
            /* e1234 */
            self[e1234] * powi(other[scalar], 2)
        );
    }
}
extension AntiScalar: AntiRejectViaHorizonFrom<Motor> {
    func anti_reject_via_horizon_from(self, other: Motor) -> Motor {
AntiScalar wedge = AntiScalar::from_groups(
            /* e1234 */
            self[e1234] * other[scalar]
        );
Motor right_dual = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            other.group1() * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(wedge[e1234]) * right_dual.group0(), 
            /* e23, e31, e12, scalar */
            float4(wedge[e1234]) * right_dual.group1()
        );
    }
}
extension AntiScalar: AntiRejectViaHorizonFrom<MultiVector> {
    func anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
AntiScalar wedge = AntiScalar::from_groups(
            /* e1234 */
            self[e1234] * other[scalar]
        );
MultiVector right_dual = MultiVector::from_groups(
            /* scalar, e1234 */
            float2(0.0, other[scalar]), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other[e321] * -1.0), 
            /* e41, e42, e43 */
            other.group3() * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(other[e1], other[e2], other[e3], 0.0)
        );
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(wedge[e1234]) * right_dual.group0(), 
            /* e1, e2, e3, e4 */
            float4(wedge[e1234]) * right_dual.group1(), 
            /* e41, e42, e43 */
            float3(wedge[e1234]) * right_dual.group2(), 
            /* e23, e31, e12 */
            float3(wedge[e1234]) * right_dual.group3(), 
            /* e423, e431, e412, e321 */
            float4(wedge[e1234]) * right_dual.group4()
        );
    }
}
extension AntiScalar: AntiRejectViaHorizonFrom<Scalar> {
    func anti_reject_via_horizon_from(self, other: Scalar) -> AntiScalar {
        return AntiScalar::from_groups(
            /* e1234 */
            self[e1234] * powi(other[scalar], 2)
        );
    }
}
extension DualNum {
    func div(_rhs: anti_reject_via_horizon_from) -> anti_reject_via_horizon_from_partial<DualNum> {
        anti_reject_via_horizon_from_partial(this)
    }
}
extension DualNum: AntiRejectViaHorizonFrom<DualNum> {
    func anti_reject_via_horizon_from(self, other: DualNum) -> DualNum {
        return DualNum::from_groups(
            /* scalar, e1234 */
            float2(other[scalar]) * float2(other[scalar] * self[scalar], (other[scalar] * self[e1234]) + (other[e1234] * self[scalar]))
        );
    }
}
extension DualNum: AntiRejectViaHorizonFrom<Flector> {
    func anti_reject_via_horizon_from(self, other: Flector) -> Motor {
Flector wedge = Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(self[scalar]) * other.group0(), 
            /* e423, e431, e412, e321 */
            float4(self[scalar]) * other.group1()
        );
Flector right_dual = Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other[e321] * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other[e1], other[e2], other[e3], 0.0)
        );
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(((right_dual.group1().yzx * wedge.group1().zxy) - (right_dual.group1().zxy * wedge.group1().yzx)), 0.0), 
            /* e23, e31, e12, scalar */
            (right_dual.group1().wwwx * float4(wedge.group1().xyz, wedge[e1])) + float4(float3(0.0), (right_dual[e431] * wedge[e2]) + (right_dual[e412] * wedge[e3]) + (right_dual[e321] * wedge[e4]) - (right_dual[e2] * wedge[e431]) - (right_dual[e3] * wedge[e412]) - (right_dual[e4] * wedge[e321])) - (wedge.group1().wwwx * float4(right_dual.group1().xyz, right_dual[e1]))
        );
    }
}
extension DualNum: AntiRejectViaHorizonFrom<Horizon> {
    func anti_reject_via_horizon_from(self, other: Horizon) -> Scalar {
        return Scalar::from_groups(
            /* scalar */
            self[scalar] * powi(other[e321], 2)
        );
    }
}
extension DualNum: AntiRejectViaHorizonFrom<Line> {
    func anti_reject_via_horizon_from(self, other: Line) -> Scalar {
Line wedge = Line::from_groups(
            /* e41, e42, e43 */
            float3(self[scalar]) * other.group0(), 
            /* e23, e31, e12 */
            float3(self[scalar]) * other.group1()
        );
Line right_dual = Line::from_groups(
            /* e41, e42, e43 */
            other.group1() * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
        return Scalar::from_groups(
            /* scalar */
            -(right_dual[e41] * wedge[e23]) - (right_dual[e42] * wedge[e31]) - (right_dual[e43] * wedge[e12]) - (right_dual[e23] * wedge[e41]) - (right_dual[e31] * wedge[e42]) - (right_dual[e12] * wedge[e43])
        );
    }
}
extension DualNum: AntiRejectViaHorizonFrom<Motor> {
    func anti_reject_via_horizon_from(self, other: Motor) -> Motor {
Motor wedge = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(other[e41], other[e42], other[e43], 1.0) * float4(self.group0().xx, self[scalar], (self[scalar] * other[e1234]) + (self[e1234] * other[scalar])), 
            /* e23, e31, e12, scalar */
            float4(self[scalar]) * other.group1()
        );
Motor right_dual = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            other.group1() * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(((float3(right_dual[e1234]) * wedge.group0().xyz) + (float3(wedge[e1234]) * right_dual.group0().xyz)), right_dual[e1234] * wedge[e1234]), 
            /* e23, e31, e12, scalar */
            (float4(right_dual[e1234]) * wedge.group1()) + (float4(wedge[e1234]) * right_dual.group1()) + float4(float3(0.0), -(right_dual[e41] * wedge[e23]) - (right_dual[e42] * wedge[e31]) - (right_dual[e43] * wedge[e12]) - (right_dual[e23] * wedge[e41]) - (right_dual[e31] * wedge[e42]) - (right_dual[e12] * wedge[e43]))
        );
    }
}
extension DualNum: AntiRejectViaHorizonFrom<MultiVector> {
    func anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
MultiVector wedge = MultiVector::from_groups(
            /* scalar, e1234 */
            float2(self[scalar] * other[scalar], (self[scalar] * other[e1234]) + (self[e1234] * other[scalar])), 
            /* e1, e2, e3, e4 */
            float4(self[scalar]) * other.group1(), 
            /* e41, e42, e43 */
            float3(self[scalar]) * other.group2(), 
            /* e23, e31, e12 */
            float3(self[scalar]) * other.group3(), 
            /* e423, e431, e412, e321 */
            float4(self[scalar]) * other.group4()
        );
MultiVector right_dual = MultiVector::from_groups(
            /* scalar, e1234 */
            float2(0.0, other[scalar]), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other[e321] * -1.0), 
            /* e41, e42, e43 */
            other.group3() * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(other[e1], other[e2], other[e3], 0.0)
        );
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2((right_dual[scalar] * wedge[e1234]) + (right_dual[e1234] * wedge[scalar]) + (right_dual[e423] * wedge[e1]) + (right_dual[e431] * wedge[e2]) + (right_dual[e412] * wedge[e3]) + (right_dual[e321] * wedge[e4]) - (right_dual[e1] * wedge[e423]) - (right_dual[e2] * wedge[e431]) - (right_dual[e3] * wedge[e412]) - (right_dual[e4] * wedge[e321]) - (right_dual[e41] * wedge[e23]) - (right_dual[e42] * wedge[e31]) - (right_dual[e43] * wedge[e12]) - (right_dual[e23] * wedge[e41]) - (right_dual[e31] * wedge[e42]) - (right_dual[e12] * wedge[e43]), right_dual[e1234] * wedge[e1234]), 
            /* e1, e2, e3, e4 */
            float4((right_dual[e41] * wedge[e321]) + (right_dual[e31] * wedge[e412]) + (right_dual[e412] * wedge[e31]) + (right_dual[e321] * wedge[e41]), (right_dual[e42] * wedge[e321]) + (right_dual[e12] * wedge[e423]) + (right_dual[e423] * wedge[e12]) + (right_dual[e321] * wedge[e42]), (right_dual[e43] * wedge[e321]) + (right_dual[e23] * wedge[e431]) + (right_dual[e431] * wedge[e23]) + (right_dual[e321] * wedge[e43]), -(right_dual[e43] * wedge[e412]) - (right_dual[e423] * wedge[e41]) - (right_dual[e431] * wedge[e42]) - (right_dual[e412] * wedge[e43])) + (float4(right_dual[e1234]) * wedge.group1()) + (float4(wedge[e1234]) * right_dual.group1()) - (wedge.group4().yzxx * float4(right_dual.group3().zxy, right_dual[e41])) - float4((wedge.group3().zxy * right_dual.group4().yzx), right_dual[e42] * wedge[e431]), 
            /* e41, e42, e43 */
            (float3(right_dual[e1234]) * wedge.group2()) + (float3(wedge[e1234]) * right_dual.group2()) + (right_dual.group4().yzx * wedge.group4().zxy) - (right_dual.group4().zxy * wedge.group4().yzx), 
            /* e23, e31, e12 */
            (float3(right_dual[e1234]) * wedge.group3()) + (float3(right_dual[e321]) * wedge.group4().xyz) + (float3(wedge[e1234]) * right_dual.group3()) - (float3(wedge[e321]) * right_dual.group4().xyz), 
            /* e423, e431, e412, e321 */
            (float4(right_dual[e1234]) * wedge.group4()) + (float4(wedge[e1234]) * right_dual.group4())
        );
    }
}
extension DualNum: AntiRejectViaHorizonFrom<Plane> {
    func anti_reject_via_horizon_from(self, other: Plane) -> Scalar {
        return Scalar::from_groups(
            /* scalar */
            self[scalar] * powi(other[e321], 2)
        );
    }
}
extension DualNum: AntiRejectViaHorizonFrom<Point> {
    func anti_reject_via_horizon_from(self, other: Point) -> Scalar {
Point wedge = Point::from_groups(
            /* e1, e2, e3, e4 */
            float4(self[scalar]) * other.group0()
        );
Plane right_dual = Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4(other[e1], other[e2], other[e3], 0.0)
        );
        return Scalar::from_groups(
            /* scalar */
            (right_dual[e423] * wedge[e1]) + (right_dual[e431] * wedge[e2]) + (right_dual[e412] * wedge[e3]) + (right_dual[e321] * wedge[e4])
        );
    }
}
extension DualNum: AntiRejectViaHorizonFrom<Scalar> {
    func anti_reject_via_horizon_from(self, other: Scalar) -> DualNum {
        return DualNum::from_groups(
            /* scalar, e1234 */
            float2(other[scalar]) * float2(self[scalar] * other[scalar], self[e1234] * other[scalar])
        );
    }
}
extension Flector {
    func div(_rhs: anti_reject_via_horizon_from) -> anti_reject_via_horizon_from_partial<Flector> {
        anti_reject_via_horizon_from_partial(this)
    }
}
extension Flector: AntiRejectViaHorizonFrom<DualNum> {
    func anti_reject_via_horizon_from(self, other: DualNum) -> Flector {
Flector wedge = Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(other[scalar]) * self.group0(), 
            /* e423, e431, e412, e321 */
            float4(other[scalar]) * self.group1()
        );
AntiScalar right_dual = AntiScalar::from_groups(
            /* e1234 */
            other[scalar]
        );
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(right_dual[e1234]) * wedge.group0(), 
            /* e423, e431, e412, e321 */
            float4(right_dual[e1234]) * wedge.group1()
        );
    }
}
extension Flector: AntiRejectViaHorizonFrom<Flector> {
    func anti_reject_via_horizon_from(self, other: Flector) -> Flector {
Motor wedge = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            (self.group0().wwwx * float4(other.group0().xyz, other[e423])) + float4(float3(0.0), (other[e431] * self[e2]) + (other[e412] * self[e3]) + (other[e321] * self[e4]) - (other[e2] * self[e431]) - (other[e3] * self[e412]) - (other[e4] * self[e321])) - (other.group0().wwwx * float4(self.group0().xyz, self[e423])), 
            /* e23, e31, e12, scalar */
            float4(((other.group0().zxy * self.group0().yzx) - (other.group0().yzx * self.group0().zxy)), 0.0)
        );
Flector right_dual = Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other[e321] * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other[e1], other[e2], other[e3], 0.0)
        );
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4((right_dual[e412] * wedge[e31]) + (right_dual[e321] * wedge[e41]), (right_dual[e423] * wedge[e12]) + (right_dual[e321] * wedge[e42]), (right_dual[e431] * wedge[e23]) + (right_dual[e321] * wedge[e43]), -(right_dual[e431] * wedge[e42]) - (right_dual[e412] * wedge[e43])) + (float4(wedge[e1234]) * right_dual.group0()) - (right_dual.group1().yzxx * float4(wedge.group1().zxy, wedge[e41])), 
            /* e423, e431, e412, e321 */
            float4(float3(1.0), 0.0) * float4(wedge.group0().www, 0.0) * float4(right_dual.group1().xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Flector: AntiRejectViaHorizonFrom<Horizon> {
    func anti_reject_via_horizon_from(self, other: Horizon) -> Origin {
        return Origin::from_groups(
            /* e4 */
            self[e4] * powi(other[e321], 2) * -1.0
        );
    }
}
extension Flector: AntiRejectViaHorizonFrom<Line> {
    func anti_reject_via_horizon_from(self, other: Line) -> Point {
Plane wedge = Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4((self[e3] * other[e42]) + (self[e4] * other[e23]), (self[e1] * other[e43]) + (self[e4] * other[e31]), (self[e2] * other[e41]) + (self[e4] * other[e12]), -(self[e2] * other[e31]) - (self[e3] * other[e12])) - (self.group0().yzxx * float4(other.group0().zxy, other[e23]))
        );
Line right_dual = Line::from_groups(
            /* e41, e42, e43 */
            other.group1() * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
        return Point::from_groups(
            /* e1, e2, e3, e4 */
            float4((right_dual[e41] * wedge[e321]) + (right_dual[e31] * wedge[e412]), (right_dual[e42] * wedge[e321]) + (right_dual[e12] * wedge[e423]), (right_dual[e43] * wedge[e321]) + (right_dual[e23] * wedge[e431]), -(right_dual[e42] * wedge[e431]) - (right_dual[e43] * wedge[e412])) - (wedge.group0().yzxx * float4(right_dual.group1().zxy, right_dual[e41]))
        );
    }
}
extension Flector: AntiRejectViaHorizonFrom<Motor> {
    func anti_reject_via_horizon_from(self, other: Motor) -> Flector {
Flector wedge = Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(other[scalar]) * self.group0(), 
            /* e423, e431, e412, e321 */
            float4((self[e4] * other[e23]) + (self[e423] * other[scalar]), (self[e4] * other[e31]) + (self[e431] * other[scalar]), (self[e4] * other[e12]) + (self[e412] * other[scalar]), -(self[e2] * other[e31]) - (self[e3] * other[e12])) + float4((self.group0().zxy * other.group0().yzx), self[e321] * other[scalar]) - (self.group0().yzxx * float4(other.group0().zxy, other[e23]))
        );
Motor right_dual = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            other.group1() * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4((wedge[e412] * right_dual[e31]) + (wedge[e321] * right_dual[e41]), (wedge[e423] * right_dual[e12]) + (wedge[e321] * right_dual[e42]), (wedge[e431] * right_dual[e23]) + (wedge[e321] * right_dual[e43]), -(wedge[e431] * right_dual[e42]) - (wedge[e412] * right_dual[e43])) + (float4(right_dual[e1234]) * wedge.group0()) - (wedge.group1().yzxx * float4(right_dual.group1().zxy, right_dual[e41])), 
            /* e423, e431, e412, e321 */
            float4(right_dual[e1234]) * wedge.group1()
        );
    }
}
extension Flector: AntiRejectViaHorizonFrom<MultiVector> {
    func anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
MultiVector wedge = MultiVector::from_groups(
            /* scalar, e1234 */
            float2(0.0, (self[e1] * other[e423]) + (self[e2] * other[e431]) + (self[e3] * other[e412]) + (self[e4] * other[e321]) - (self[e423] * other[e1]) - (self[e431] * other[e2]) - (self[e412] * other[e3]) - (self[e321] * other[e4])), 
            /* e1, e2, e3, e4 */
            float4(other[scalar]) * self.group0(), 
            /* e41, e42, e43 */
            (float3(self[e4]) * other.group1().xyz) - (float3(other[e4]) * self.group0().xyz), 
            /* e23, e31, e12 */
            (self.group0().yzx * other.group1().zxy) - (self.group0().zxy * other.group1().yzx), 
            /* e423, e431, e412, e321 */
            float4((self[e3] * other[e42]) + (self[e4] * other[e23]), (self[e1] * other[e43]) + (self[e4] * other[e31]), (self[e2] * other[e41]) + (self[e4] * other[e12]), -(self[e2] * other[e31]) - (self[e3] * other[e12])) + (float4(other[scalar]) * self.group1()) - (self.group0().yzxx * float4(other.group2().zxy, other[e23]))
        );
MultiVector right_dual = MultiVector::from_groups(
            /* scalar, e1234 */
            float2(0.0, other[scalar]), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other[e321] * -1.0), 
            /* e41, e42, e43 */
            other.group3() * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(other[e1], other[e2], other[e3], 0.0)
        );
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2((right_dual[scalar] * wedge[e1234]) + (right_dual[e1234] * wedge[scalar]) + (right_dual[e423] * wedge[e1]) + (right_dual[e431] * wedge[e2]) + (right_dual[e412] * wedge[e3]) + (right_dual[e321] * wedge[e4]) - (right_dual[e1] * wedge[e423]) - (right_dual[e2] * wedge[e431]) - (right_dual[e3] * wedge[e412]) - (right_dual[e4] * wedge[e321]) - (right_dual[e41] * wedge[e23]) - (right_dual[e42] * wedge[e31]) - (right_dual[e43] * wedge[e12]) - (right_dual[e23] * wedge[e41]) - (right_dual[e31] * wedge[e42]) - (right_dual[e12] * wedge[e43]), right_dual[e1234] * wedge[e1234]), 
            /* e1, e2, e3, e4 */
            float4((right_dual[e41] * wedge[e321]) + (right_dual[e31] * wedge[e412]) + (right_dual[e412] * wedge[e31]) + (right_dual[e321] * wedge[e41]), (right_dual[e42] * wedge[e321]) + (right_dual[e12] * wedge[e423]) + (right_dual[e423] * wedge[e12]) + (right_dual[e321] * wedge[e42]), (right_dual[e43] * wedge[e321]) + (right_dual[e23] * wedge[e431]) + (right_dual[e431] * wedge[e23]) + (right_dual[e321] * wedge[e43]), -(right_dual[e43] * wedge[e412]) - (right_dual[e423] * wedge[e41]) - (right_dual[e431] * wedge[e42]) - (right_dual[e412] * wedge[e43])) + (float4(right_dual[e1234]) * wedge.group1()) + (float4(wedge[e1234]) * right_dual.group1()) - (wedge.group4().yzxx * float4(right_dual.group3().zxy, right_dual[e41])) - float4((wedge.group3().zxy * right_dual.group4().yzx), right_dual[e42] * wedge[e431]), 
            /* e41, e42, e43 */
            (float3(right_dual[e1234]) * wedge.group2()) + (float3(wedge[e1234]) * right_dual.group2()) + (right_dual.group4().yzx * wedge.group4().zxy) - (right_dual.group4().zxy * wedge.group4().yzx), 
            /* e23, e31, e12 */
            (float3(right_dual[e1234]) * wedge.group3()) + (float3(right_dual[e321]) * wedge.group4().xyz) + (float3(wedge[e1234]) * right_dual.group3()) - (float3(wedge[e321]) * right_dual.group4().xyz), 
            /* e423, e431, e412, e321 */
            (float4(right_dual[e1234]) * wedge.group4()) + (float4(wedge[e1234]) * right_dual.group4())
        );
    }
}
extension Flector: AntiRejectViaHorizonFrom<Plane> {
    func anti_reject_via_horizon_from(self, other: Plane) -> Origin {
        return Origin::from_groups(
            /* e4 */
            -(powi(other[e321], 2) * self[e4]) - (self[e1] * other[e423] * other[e321]) - (self[e2] * other[e431] * other[e321]) - (self[e3] * other[e412] * other[e321])
        );
    }
}
extension Flector: AntiRejectViaHorizonFrom<Point> {
    func anti_reject_via_horizon_from(self, other: Point) -> Flector {
Motor wedge = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(self[e4] * other[e1], self[e4] * other[e2], self[e4] * other[e3], -(self[e431] * other[e2]) - (self[e412] * other[e3]) - (self[e321] * other[e4])) - (other.group0().wwwx * float4(self.group0().xyz, self[e423])), 
            /* e23, e31, e12, scalar */
            float4(((self.group0().yzx * other.group0().zxy) - (self.group0().zxy * other.group0().yzx)), 0.0)
        );
Plane right_dual = Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4(other[e1], other[e2], other[e3], 0.0)
        );
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4((wedge[e41] * right_dual[e321]) + (wedge[e31] * right_dual[e412]), (wedge[e42] * right_dual[e321]) + (wedge[e12] * right_dual[e423]), (wedge[e43] * right_dual[e321]) + (wedge[e23] * right_dual[e431]), -(wedge[e42] * right_dual[e431]) - (wedge[e43] * right_dual[e412])) - (right_dual.group0().yzxx * float4(wedge.group1().zxy, wedge[e41])), 
            /* e423, e431, e412, e321 */
            float4(float3(1.0), 0.0) * float4(wedge.group0().www, 0.0) * float4(right_dual.group0().xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Flector: AntiRejectViaHorizonFrom<Scalar> {
    func anti_reject_via_horizon_from(self, other: Scalar) -> Flector {
Flector wedge = Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(other[scalar]) * self.group0(), 
            /* e423, e431, e412, e321 */
            float4(other[scalar]) * self.group1()
        );
AntiScalar right_dual = AntiScalar::from_groups(
            /* e1234 */
            other[scalar]
        );
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(right_dual[e1234]) * wedge.group0(), 
            /* e423, e431, e412, e321 */
            float4(right_dual[e1234]) * wedge.group1()
        );
    }
}
extension Horizon {
    func div(_rhs: anti_reject_via_horizon_from) -> anti_reject_via_horizon_from_partial<Horizon> {
        anti_reject_via_horizon_from_partial(this)
    }
}
extension Horizon: AntiRejectViaHorizonFrom<DualNum> {
    func anti_reject_via_horizon_from(self, other: DualNum) -> Horizon {
        return Horizon::from_groups(
            /* e321 */
            powi(other[scalar], 2) * self[e321]
        );
    }
}
extension Horizon: AntiRejectViaHorizonFrom<Flector> {
    func anti_reject_via_horizon_from(self, other: Flector) -> Flector {
AntiScalar wedge = AntiScalar::from_groups(
            /* e1234 */
            other[e4] * self[e321] * -1.0
        );
Flector right_dual = Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other[e321] * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other[e1], other[e2], other[e3], 0.0)
        );
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(wedge[e1234]) * right_dual.group0(), 
            /* e423, e431, e412, e321 */
            float4(wedge[e1234]) * right_dual.group1()
        );
    }
}
extension Horizon: AntiRejectViaHorizonFrom<Motor> {
    func anti_reject_via_horizon_from(self, other: Motor) -> Flector {
Horizon wedge = Horizon::from_groups(
            /* e321 */
            self[e321] * other[scalar]
        );
Motor right_dual = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            other.group1() * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(wedge[e321], wedge[e321], wedge[e321], 0.0) * float4(right_dual.group0().xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), wedge[e321] * right_dual[e1234])
        );
    }
}
extension Horizon: AntiRejectViaHorizonFrom<MultiVector> {
    func anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
MultiVector wedge = MultiVector::from_groups(
            /* scalar, e1234 */
            float2(1.0, self[e321] * other[e4]) * float2(0.0, -1.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), self[e321] * other[scalar])
        );
MultiVector right_dual = MultiVector::from_groups(
            /* scalar, e1234 */
            float2(0.0, other[scalar]), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other[e321] * -1.0), 
            /* e41, e42, e43 */
            other.group3() * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(other[e1], other[e2], other[e3], 0.0)
        );
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2((right_dual[scalar] * wedge[e1234]) + (right_dual[e1234] * wedge[scalar]) + (right_dual[e423] * wedge[e1]) + (right_dual[e431] * wedge[e2]) + (right_dual[e412] * wedge[e3]) + (right_dual[e321] * wedge[e4]) - (right_dual[e1] * wedge[e423]) - (right_dual[e2] * wedge[e431]) - (right_dual[e3] * wedge[e412]) - (right_dual[e4] * wedge[e321]) - (right_dual[e41] * wedge[e23]) - (right_dual[e42] * wedge[e31]) - (right_dual[e43] * wedge[e12]) - (right_dual[e23] * wedge[e41]) - (right_dual[e31] * wedge[e42]) - (right_dual[e12] * wedge[e43]), right_dual[e1234] * wedge[e1234]), 
            /* e1, e2, e3, e4 */
            float4((right_dual[e41] * wedge[e321]) + (right_dual[e31] * wedge[e412]) + (right_dual[e412] * wedge[e31]) + (right_dual[e321] * wedge[e41]), (right_dual[e42] * wedge[e321]) + (right_dual[e12] * wedge[e423]) + (right_dual[e423] * wedge[e12]) + (right_dual[e321] * wedge[e42]), (right_dual[e43] * wedge[e321]) + (right_dual[e23] * wedge[e431]) + (right_dual[e431] * wedge[e23]) + (right_dual[e321] * wedge[e43]), -(right_dual[e43] * wedge[e412]) - (right_dual[e423] * wedge[e41]) - (right_dual[e431] * wedge[e42]) - (right_dual[e412] * wedge[e43])) + (float4(right_dual[e1234]) * wedge.group1()) + (float4(wedge[e1234]) * right_dual.group1()) - (wedge.group4().yzxx * float4(right_dual.group3().zxy, right_dual[e41])) - float4((wedge.group3().zxy * right_dual.group4().yzx), right_dual[e42] * wedge[e431]), 
            /* e41, e42, e43 */
            (float3(right_dual[e1234]) * wedge.group2()) + (float3(wedge[e1234]) * right_dual.group2()) + (right_dual.group4().yzx * wedge.group4().zxy) - (right_dual.group4().zxy * wedge.group4().yzx), 
            /* e23, e31, e12 */
            (float3(right_dual[e1234]) * wedge.group3()) + (float3(right_dual[e321]) * wedge.group4().xyz) + (float3(wedge[e1234]) * right_dual.group3()) - (float3(wedge[e321]) * right_dual.group4().xyz), 
            /* e423, e431, e412, e321 */
            (float4(right_dual[e1234]) * wedge.group4()) + (float4(wedge[e1234]) * right_dual.group4())
        );
    }
}
extension Horizon: AntiRejectViaHorizonFrom<Point> {
    func anti_reject_via_horizon_from(self, other: Point) -> Plane {
        return Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4(self[e321] * other[e4] * -1.0) * float4(other[e1], other[e2], other[e3], 0.0)
        );
    }
}
extension Horizon: AntiRejectViaHorizonFrom<Scalar> {
    func anti_reject_via_horizon_from(self, other: Scalar) -> Horizon {
        return Horizon::from_groups(
            /* e321 */
            self[e321] * powi(other[scalar], 2)
        );
    }
}
extension Line {
    func div(_rhs: anti_reject_via_horizon_from) -> anti_reject_via_horizon_from_partial<Line> {
        anti_reject_via_horizon_from_partial(this)
    }
}
extension Line: AntiRejectViaHorizonFrom<DualNum> {
    func anti_reject_via_horizon_from(self, other: DualNum) -> Line {
Line wedge = Line::from_groups(
            /* e41, e42, e43 */
            float3(other[scalar]) * self.group0(), 
            /* e23, e31, e12 */
            float3(other[scalar]) * self.group1()
        );
AntiScalar right_dual = AntiScalar::from_groups(
            /* e1234 */
            other[scalar]
        );
        return Line::from_groups(
            /* e41, e42, e43 */
            float3(right_dual[e1234]) * wedge.group0(), 
            /* e23, e31, e12 */
            float3(right_dual[e1234]) * wedge.group1()
        );
    }
}
extension Line: AntiRejectViaHorizonFrom<Flector> {
    func anti_reject_via_horizon_from(self, other: Flector) -> Motor {
Plane wedge = Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4((other[e3] * self[e42]) + (other[e4] * self[e23]), (other[e1] * self[e43]) + (other[e4] * self[e31]), (other[e2] * self[e41]) + (other[e4] * self[e12]), -(other[e2] * self[e31]) - (other[e3] * self[e12])) - (other.group0().yzxx * float4(self.group0().zxy, self[e23]))
        );
Flector right_dual = Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other[e321] * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other[e1], other[e2], other[e3], 0.0)
        );
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(((right_dual.group1().yzx * wedge.group0().zxy) - (right_dual.group1().zxy * wedge.group0().yzx)), 0.0), 
            /* e23, e31, e12, scalar */
            float4(right_dual[e321] * wedge[e423], right_dual[e321] * wedge[e431], right_dual[e321] * wedge[e412], -(right_dual[e2] * wedge[e431]) - (right_dual[e3] * wedge[e412]) - (right_dual[e4] * wedge[e321])) - (wedge.group0().wwwx * float4(right_dual.group1().xyz, right_dual[e1]))
        );
    }
}
extension Line: AntiRejectViaHorizonFrom<Line> {
    func anti_reject_via_horizon_from(self, other: Line) -> Line {
        return Line::from_groups(
            /* e41, e42, e43 */
            float3(-(other[e41] * self[e23]) - (other[e42] * self[e31]) - (other[e43] * self[e12]) - (other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43])) * float3(other[e23] * -1.0, other[e31] * -1.0, other[e12] * -1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
    }
}
extension Line: AntiRejectViaHorizonFrom<Motor> {
    func anti_reject_via_horizon_from(self, other: Motor) -> Motor {
Motor wedge = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(other[scalar], other[scalar], other[scalar], 1.0) * float4(self.group0(), -(self[e41] * other[e23]) - (self[e42] * other[e31]) - (self[e43] * other[e12]) - (self[e23] * other[e41]) - (self[e31] * other[e42]) - (self[e12] * other[e43])), 
            /* e23, e31, e12, scalar */
            float4(float3(1.0), 0.0) * float4(self.group1(), 0.0) * float4(other.group1().www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
Motor right_dual = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            other.group1() * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(((float3(right_dual[e1234]) * wedge.group0().xyz) + (float3(wedge[e1234]) * right_dual.group0().xyz)), right_dual[e1234] * wedge[e1234]), 
            /* e23, e31, e12, scalar */
            (float4(right_dual[e1234]) * wedge.group1()) + (float4(wedge[e1234]) * right_dual.group1()) + float4(float3(0.0), -(right_dual[e41] * wedge[e23]) - (right_dual[e42] * wedge[e31]) - (right_dual[e43] * wedge[e12]) - (right_dual[e23] * wedge[e41]) - (right_dual[e31] * wedge[e42]) - (right_dual[e12] * wedge[e43]))
        );
    }
}
extension Line: AntiRejectViaHorizonFrom<MultiVector> {
    func anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
MultiVector wedge = MultiVector::from_groups(
            /* scalar, e1234 */
            float2(0.0, -(self[e41] * other[e23]) - (self[e42] * other[e31]) - (self[e43] * other[e12]) - (self[e23] * other[e41]) - (self[e31] * other[e42]) - (self[e12] * other[e43])), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(other[scalar]) * self.group0(), 
            /* e23, e31, e12 */
            float3(other[scalar]) * self.group1(), 
            /* e423, e431, e412, e321 */
            float4((self[e42] * other[e3]) + (self[e23] * other[e4]), (self[e43] * other[e1]) + (self[e31] * other[e4]), (self[e41] * other[e2]) + (self[e12] * other[e4]), -(self[e31] * other[e2]) - (self[e12] * other[e3])) - (other.group1().yzxx * float4(self.group0().zxy, self[e23]))
        );
MultiVector right_dual = MultiVector::from_groups(
            /* scalar, e1234 */
            float2(0.0, other[scalar]), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other[e321] * -1.0), 
            /* e41, e42, e43 */
            other.group3() * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(other[e1], other[e2], other[e3], 0.0)
        );
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2((right_dual[scalar] * wedge[e1234]) + (right_dual[e1234] * wedge[scalar]) + (right_dual[e423] * wedge[e1]) + (right_dual[e431] * wedge[e2]) + (right_dual[e412] * wedge[e3]) + (right_dual[e321] * wedge[e4]) - (right_dual[e1] * wedge[e423]) - (right_dual[e2] * wedge[e431]) - (right_dual[e3] * wedge[e412]) - (right_dual[e4] * wedge[e321]) - (right_dual[e41] * wedge[e23]) - (right_dual[e42] * wedge[e31]) - (right_dual[e43] * wedge[e12]) - (right_dual[e23] * wedge[e41]) - (right_dual[e31] * wedge[e42]) - (right_dual[e12] * wedge[e43]), right_dual[e1234] * wedge[e1234]), 
            /* e1, e2, e3, e4 */
            float4((right_dual[e41] * wedge[e321]) + (right_dual[e31] * wedge[e412]) + (right_dual[e412] * wedge[e31]) + (right_dual[e321] * wedge[e41]), (right_dual[e42] * wedge[e321]) + (right_dual[e12] * wedge[e423]) + (right_dual[e423] * wedge[e12]) + (right_dual[e321] * wedge[e42]), (right_dual[e43] * wedge[e321]) + (right_dual[e23] * wedge[e431]) + (right_dual[e431] * wedge[e23]) + (right_dual[e321] * wedge[e43]), -(right_dual[e43] * wedge[e412]) - (right_dual[e423] * wedge[e41]) - (right_dual[e431] * wedge[e42]) - (right_dual[e412] * wedge[e43])) + (float4(right_dual[e1234]) * wedge.group1()) + (float4(wedge[e1234]) * right_dual.group1()) - (wedge.group4().yzxx * float4(right_dual.group3().zxy, right_dual[e41])) - float4((wedge.group3().zxy * right_dual.group4().yzx), right_dual[e42] * wedge[e431]), 
            /* e41, e42, e43 */
            (float3(right_dual[e1234]) * wedge.group2()) + (float3(wedge[e1234]) * right_dual.group2()) + (right_dual.group4().yzx * wedge.group4().zxy) - (right_dual.group4().zxy * wedge.group4().yzx), 
            /* e23, e31, e12 */
            (float3(right_dual[e1234]) * wedge.group3()) + (float3(right_dual[e321]) * wedge.group4().xyz) + (float3(wedge[e1234]) * right_dual.group3()) - (float3(wedge[e321]) * right_dual.group4().xyz), 
            /* e423, e431, e412, e321 */
            (float4(right_dual[e1234]) * wedge.group4()) + (float4(wedge[e1234]) * right_dual.group4())
        );
    }
}
extension Line: AntiRejectViaHorizonFrom<Point> {
    func anti_reject_via_horizon_from(self, other: Point) -> Line {
Plane wedge = Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4((self[e42] * other[e3]) + (self[e23] * other[e4]), (self[e43] * other[e1]) + (self[e31] * other[e4]), (self[e41] * other[e2]) + (self[e12] * other[e4]), -(self[e31] * other[e2]) - (self[e12] * other[e3])) - (other.group0().yzxx * float4(self.group0().zxy, self[e23]))
        );
Plane right_dual = Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4(other[e1], other[e2], other[e3], 0.0)
        );
        return Line::from_groups(
            /* e41, e42, e43 */
            (right_dual.group0().yzx * wedge.group0().zxy) - (right_dual.group0().zxy * wedge.group0().yzx), 
            /* e23, e31, e12 */
            (float3(right_dual[e321]) * wedge.group0().xyz) - (float3(wedge[e321]) * right_dual.group0().xyz)
        );
    }
}
extension Line: AntiRejectViaHorizonFrom<Scalar> {
    func anti_reject_via_horizon_from(self, other: Scalar) -> Line {
Line wedge = Line::from_groups(
            /* e41, e42, e43 */
            float3(other[scalar]) * self.group0(), 
            /* e23, e31, e12 */
            float3(other[scalar]) * self.group1()
        );
AntiScalar right_dual = AntiScalar::from_groups(
            /* e1234 */
            other[scalar]
        );
        return Line::from_groups(
            /* e41, e42, e43 */
            float3(right_dual[e1234]) * wedge.group0(), 
            /* e23, e31, e12 */
            float3(right_dual[e1234]) * wedge.group1()
        );
    }
}
extension Motor {
    func div(_rhs: anti_reject_via_horizon_from) -> anti_reject_via_horizon_from_partial<Motor> {
        anti_reject_via_horizon_from_partial(this)
    }
}
extension Motor: AntiRejectViaHorizonFrom<DualNum> {
    func anti_reject_via_horizon_from(self, other: DualNum) -> Motor {
Motor wedge = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(self[e41], self[e42], self[e43], 1.0) * float4(other.group0().xx, other[scalar], (other[scalar] * self[e1234]) + (other[e1234] * self[scalar])), 
            /* e23, e31, e12, scalar */
            float4(other[scalar]) * self.group1()
        );
AntiScalar right_dual = AntiScalar::from_groups(
            /* e1234 */
            other[scalar]
        );
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(right_dual[e1234]) * wedge.group0(), 
            /* e23, e31, e12, scalar */
            float4(right_dual[e1234]) * wedge.group1()
        );
    }
}
extension Motor: AntiRejectViaHorizonFrom<Flector> {
    func anti_reject_via_horizon_from(self, other: Flector) -> Motor {
Flector wedge = Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(self[scalar]) * other.group0(), 
            /* e423, e431, e412, e321 */
            float4((other[e4] * self[e23]) + (other[e423] * self[scalar]), (other[e4] * self[e31]) + (other[e431] * self[scalar]), (other[e4] * self[e12]) + (other[e412] * self[scalar]), -(other[e2] * self[e31]) - (other[e3] * self[e12])) + float4((other.group0().zxy * self.group0().yzx), other[e321] * self[scalar]) - (other.group0().yzxx * float4(self.group0().zxy, self[e23]))
        );
Flector right_dual = Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other[e321] * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other[e1], other[e2], other[e3], 0.0)
        );
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(((right_dual.group1().yzx * wedge.group1().zxy) - (right_dual.group1().zxy * wedge.group1().yzx)), 0.0), 
            /* e23, e31, e12, scalar */
            (right_dual.group1().wwwx * float4(wedge.group1().xyz, wedge[e1])) + float4(float3(0.0), (right_dual[e431] * wedge[e2]) + (right_dual[e412] * wedge[e3]) + (right_dual[e321] * wedge[e4]) - (right_dual[e2] * wedge[e431]) - (right_dual[e3] * wedge[e412]) - (right_dual[e4] * wedge[e321])) - (wedge.group1().wwwx * float4(right_dual.group1().xyz, right_dual[e1]))
        );
    }
}
extension Motor: AntiRejectViaHorizonFrom<Horizon> {
    func anti_reject_via_horizon_from(self, other: Horizon) -> Scalar {
        return Scalar::from_groups(
            /* scalar */
            powi(other[e321], 2) * self[scalar]
        );
    }
}
extension Motor: AntiRejectViaHorizonFrom<Line> {
    func anti_reject_via_horizon_from(self, other: Line) -> Motor {
Motor wedge = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(self[scalar], self[scalar], self[scalar], 1.0) * float4(other.group0(), -(other[e41] * self[e23]) - (other[e42] * self[e31]) - (other[e43] * self[e12]) - (other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43])), 
            /* e23, e31, e12, scalar */
            float4(float3(1.0), 0.0) * float4(other.group1(), 0.0) * float4(self.group1().www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
Line right_dual = Line::from_groups(
            /* e41, e42, e43 */
            other.group1() * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(float3(1.0), 0.0) * float4(right_dual.group0(), 0.0) * float4(wedge.group0().www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e23, e31, e12, scalar */
            float4(float3(0.0), -(right_dual[e41] * wedge[e23]) - (right_dual[e42] * wedge[e31]) - (right_dual[e43] * wedge[e12]) - (right_dual[e23] * wedge[e41]) - (right_dual[e31] * wedge[e42]) - (right_dual[e12] * wedge[e43]))
        );
    }
}
extension Motor: AntiRejectViaHorizonFrom<Motor> {
    func anti_reject_via_horizon_from(self, other: Motor) -> Motor {
Motor wedge = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            (float4(other[scalar]) * self.group0()) + (float4(self[scalar]) * other.group0()) + float4(float3(0.0), -(other[e41] * self[e23]) - (other[e42] * self[e31]) - (other[e43] * self[e12]) - (other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43])), 
            /* e23, e31, e12, scalar */
            float4(((float3(other[scalar]) * self.group1().xyz) + (float3(self[scalar]) * other.group1().xyz)), other[scalar] * self[scalar])
        );
Motor right_dual = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            other.group1() * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(((float3(right_dual[e1234]) * wedge.group0().xyz) + (float3(wedge[e1234]) * right_dual.group0().xyz)), right_dual[e1234] * wedge[e1234]), 
            /* e23, e31, e12, scalar */
            (float4(right_dual[e1234]) * wedge.group1()) + (float4(wedge[e1234]) * right_dual.group1()) + float4(float3(0.0), -(right_dual[e41] * wedge[e23]) - (right_dual[e42] * wedge[e31]) - (right_dual[e43] * wedge[e12]) - (right_dual[e23] * wedge[e41]) - (right_dual[e31] * wedge[e42]) - (right_dual[e12] * wedge[e43]))
        );
    }
}
extension Motor: AntiRejectViaHorizonFrom<MultiVector> {
    func anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
MultiVector wedge = MultiVector::from_groups(
            /* scalar, e1234 */
            float2(self[scalar] * other[scalar], (self[e1234] * other[scalar]) + (self[scalar] * other[e1234]) - (self[e41] * other[e23]) - (self[e42] * other[e31]) - (self[e43] * other[e12]) - (self[e23] * other[e41]) - (self[e31] * other[e42]) - (self[e12] * other[e43])), 
            /* e1, e2, e3, e4 */
            float4(self[scalar]) * other.group1(), 
            /* e41, e42, e43 */
            (float3(self[scalar]) * other.group2()) + (float3(other[scalar]) * self.group0().xyz), 
            /* e23, e31, e12 */
            (float3(self[scalar]) * other.group3()) + (float3(other[scalar]) * self.group1().xyz), 
            /* e423, e431, e412, e321 */
            float4((self[e23] * other[e4]) + (self[scalar] * other[e423]), (self[e31] * other[e4]) + (self[scalar] * other[e431]), (self[e12] * other[e4]) + (self[scalar] * other[e412]), -(self[e31] * other[e2]) - (self[e12] * other[e3])) + float4((self.group0().yzx * other.group1().zxy), self[scalar] * other[e321]) - (other.group1().yzxx * float4(self.group0().zxy, self[e23]))
        );
MultiVector right_dual = MultiVector::from_groups(
            /* scalar, e1234 */
            float2(0.0, other[scalar]), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other[e321] * -1.0), 
            /* e41, e42, e43 */
            other.group3() * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(other[e1], other[e2], other[e3], 0.0)
        );
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2((right_dual[scalar] * wedge[e1234]) + (right_dual[e1234] * wedge[scalar]) + (right_dual[e423] * wedge[e1]) + (right_dual[e431] * wedge[e2]) + (right_dual[e412] * wedge[e3]) + (right_dual[e321] * wedge[e4]) - (right_dual[e1] * wedge[e423]) - (right_dual[e2] * wedge[e431]) - (right_dual[e3] * wedge[e412]) - (right_dual[e4] * wedge[e321]) - (right_dual[e41] * wedge[e23]) - (right_dual[e42] * wedge[e31]) - (right_dual[e43] * wedge[e12]) - (right_dual[e23] * wedge[e41]) - (right_dual[e31] * wedge[e42]) - (right_dual[e12] * wedge[e43]), right_dual[e1234] * wedge[e1234]), 
            /* e1, e2, e3, e4 */
            float4((right_dual[e41] * wedge[e321]) + (right_dual[e31] * wedge[e412]) + (right_dual[e412] * wedge[e31]) + (right_dual[e321] * wedge[e41]), (right_dual[e42] * wedge[e321]) + (right_dual[e12] * wedge[e423]) + (right_dual[e423] * wedge[e12]) + (right_dual[e321] * wedge[e42]), (right_dual[e43] * wedge[e321]) + (right_dual[e23] * wedge[e431]) + (right_dual[e431] * wedge[e23]) + (right_dual[e321] * wedge[e43]), -(right_dual[e43] * wedge[e412]) - (right_dual[e423] * wedge[e41]) - (right_dual[e431] * wedge[e42]) - (right_dual[e412] * wedge[e43])) + (float4(right_dual[e1234]) * wedge.group1()) + (float4(wedge[e1234]) * right_dual.group1()) - (wedge.group4().yzxx * float4(right_dual.group3().zxy, right_dual[e41])) - float4((wedge.group3().zxy * right_dual.group4().yzx), right_dual[e42] * wedge[e431]), 
            /* e41, e42, e43 */
            (float3(right_dual[e1234]) * wedge.group2()) + (float3(wedge[e1234]) * right_dual.group2()) + (right_dual.group4().yzx * wedge.group4().zxy) - (right_dual.group4().zxy * wedge.group4().yzx), 
            /* e23, e31, e12 */
            (float3(right_dual[e1234]) * wedge.group3()) + (float3(right_dual[e321]) * wedge.group4().xyz) + (float3(wedge[e1234]) * right_dual.group3()) - (float3(wedge[e321]) * right_dual.group4().xyz), 
            /* e423, e431, e412, e321 */
            (float4(right_dual[e1234]) * wedge.group4()) + (float4(wedge[e1234]) * right_dual.group4())
        );
    }
}
extension Motor: AntiRejectViaHorizonFrom<Plane> {
    func anti_reject_via_horizon_from(self, other: Plane) -> Scalar {
        return Scalar::from_groups(
            /* scalar */
            self[scalar] * powi(other[e321], 2)
        );
    }
}
extension Motor: AntiRejectViaHorizonFrom<Point> {
    func anti_reject_via_horizon_from(self, other: Point) -> Motor {
Flector wedge = Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(self[scalar]) * other.group0(), 
            /* e423, e431, e412, e321 */
            float4((self[e42] * other[e3]) + (self[e23] * other[e4]), (self[e43] * other[e1]) + (self[e31] * other[e4]), (self[e41] * other[e2]) + (self[e12] * other[e4]), -(self[e31] * other[e2]) - (self[e12] * other[e3])) - (other.group0().yzxx * float4(self.group0().zxy, self[e23]))
        );
Plane right_dual = Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4(other[e1], other[e2], other[e3], 0.0)
        );
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(((wedge.group1().zxy * right_dual.group0().yzx) - (wedge.group1().yzx * right_dual.group0().zxy)), 0.0), 
            /* e23, e31, e12, scalar */
            float4(wedge[e321] * right_dual[e423] * -1.0, wedge[e321] * right_dual[e431] * -1.0, wedge[e321] * right_dual[e412] * -1.0, (wedge[e2] * right_dual[e431]) + (wedge[e3] * right_dual[e412]) + (wedge[e4] * right_dual[e321])) + (right_dual.group0().wwwx * float4(wedge.group1().xyz, wedge[e1]))
        );
    }
}
extension Motor: AntiRejectViaHorizonFrom<Scalar> {
    func anti_reject_via_horizon_from(self, other: Scalar) -> Motor {
Motor wedge = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(other[scalar]) * self.group0(), 
            /* e23, e31, e12, scalar */
            float4(other[scalar]) * self.group1()
        );
AntiScalar right_dual = AntiScalar::from_groups(
            /* e1234 */
            other[scalar]
        );
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(right_dual[e1234]) * wedge.group0(), 
            /* e23, e31, e12, scalar */
            float4(right_dual[e1234]) * wedge.group1()
        );
    }
}
extension MultiVector {
    func div(_rhs: anti_reject_via_horizon_from) -> anti_reject_via_horizon_from_partial<MultiVector> {
        anti_reject_via_horizon_from_partial(this)
    }
}
extension MultiVector: AntiRejectViaHorizonFrom<DualNum> {
    func anti_reject_via_horizon_from(self, other: DualNum) -> MultiVector {
MultiVector wedge = MultiVector::from_groups(
            /* scalar, e1234 */
            float2(other[scalar] * self[scalar], (other[scalar] * self[e1234]) + (other[e1234] * self[scalar])), 
            /* e1, e2, e3, e4 */
            float4(other[scalar]) * self.group1(), 
            /* e41, e42, e43 */
            float3(other[scalar]) * self.group2(), 
            /* e23, e31, e12 */
            float3(other[scalar]) * self.group3(), 
            /* e423, e431, e412, e321 */
            float4(other[scalar]) * self.group4()
        );
AntiScalar right_dual = AntiScalar::from_groups(
            /* e1234 */
            other[scalar]
        );
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(right_dual[e1234]) * wedge.group0(), 
            /* e1, e2, e3, e4 */
            float4(right_dual[e1234]) * wedge.group1(), 
            /* e41, e42, e43 */
            float3(right_dual[e1234]) * wedge.group2(), 
            /* e23, e31, e12 */
            float3(right_dual[e1234]) * wedge.group3(), 
            /* e423, e431, e412, e321 */
            float4(right_dual[e1234]) * wedge.group4()
        );
    }
}
extension MultiVector: AntiRejectViaHorizonFrom<Flector> {
    func anti_reject_via_horizon_from(self, other: Flector) -> MultiVector {
MultiVector wedge = MultiVector::from_groups(
            /* scalar, e1234 */
            float2(0.0, (other[e423] * self[e1]) + (other[e431] * self[e2]) + (other[e412] * self[e3]) + (other[e321] * self[e4]) - (other[e1] * self[e423]) - (other[e2] * self[e431]) - (other[e3] * self[e412]) - (other[e4] * self[e321])), 
            /* e1, e2, e3, e4 */
            float4(self[scalar]) * other.group0(), 
            /* e41, e42, e43 */
            (float3(self[e4]) * other.group0().xyz) - (float3(other[e4]) * self.group1().xyz), 
            /* e23, e31, e12 */
            (other.group0().zxy * self.group1().yzx) - (other.group0().yzx * self.group1().zxy), 
            /* e423, e431, e412, e321 */
            float4((other[e3] * self[e42]) + (other[e4] * self[e23]), (other[e1] * self[e43]) + (other[e4] * self[e31]), (other[e2] * self[e41]) + (other[e4] * self[e12]), -(other[e2] * self[e31]) - (other[e3] * self[e12])) + (float4(self[scalar]) * other.group1()) - (other.group0().yzxx * float4(self.group2().zxy, self[e23]))
        );
Flector right_dual = Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other[e321] * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other[e1], other[e2], other[e3], 0.0)
        );
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2((right_dual[e423] * wedge[e1]) + (right_dual[e431] * wedge[e2]) + (right_dual[e412] * wedge[e3]) + (right_dual[e321] * wedge[e4]) - (right_dual[e1] * wedge[e423]) - (right_dual[e2] * wedge[e431]) - (right_dual[e3] * wedge[e412]) - (right_dual[e4] * wedge[e321]), 0.0), 
            /* e1, e2, e3, e4 */
            float4((right_dual[e412] * wedge[e31]) + (right_dual[e321] * wedge[e41]), (right_dual[e423] * wedge[e12]) + (right_dual[e321] * wedge[e42]), (right_dual[e431] * wedge[e23]) + (right_dual[e321] * wedge[e43]), -(right_dual[e431] * wedge[e42]) - (right_dual[e412] * wedge[e43])) + (float4(wedge[e1234]) * right_dual.group0()) - (right_dual.group1().yzxx * float4(wedge.group3().zxy, wedge[e41])), 
            /* e41, e42, e43 */
            (right_dual.group1().yzx * wedge.group4().zxy) - (right_dual.group1().zxy * wedge.group4().yzx), 
            /* e23, e31, e12 */
            (float3(right_dual[e321]) * wedge.group4().xyz) - (float3(wedge[e321]) * right_dual.group1().xyz), 
            /* e423, e431, e412, e321 */
            float4(wedge.group0().yy, wedge[e1234], 0.0) * float4(float3(1.0), 0.0) * float4(right_dual.group1().xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension MultiVector: AntiRejectViaHorizonFrom<Horizon> {
    func anti_reject_via_horizon_from(self, other: Horizon) -> MultiVector {
MultiVector wedge = MultiVector::from_groups(
            /* scalar, e1234 */
            float2(1.0, other[e321] * self[e4]) * float2(0.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other[e321] * self[scalar])
        );
Origin right_dual = Origin::from_groups(
            /* e4 */
            other[e321] * -1.0
        );
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(wedge[e321] * right_dual[e4], 1.0) * float2(-1.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), wedge[e1234] * right_dual[e4]), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
extension MultiVector: AntiRejectViaHorizonFrom<Line> {
    func anti_reject_via_horizon_from(self, other: Line) -> MultiVector {
MultiVector wedge = MultiVector::from_groups(
            /* scalar, e1234 */
            float2(0.0, -(other[e41] * self[e23]) - (other[e42] * self[e31]) - (other[e43] * self[e12]) - (other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43])), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(self[scalar]) * other.group0(), 
            /* e23, e31, e12 */
            float3(self[scalar]) * other.group1(), 
            /* e423, e431, e412, e321 */
            float4((other[e42] * self[e3]) + (other[e23] * self[e4]), (other[e43] * self[e1]) + (other[e31] * self[e4]), (other[e41] * self[e2]) + (other[e12] * self[e4]), -(other[e31] * self[e2]) - (other[e12] * self[e3])) - (self.group1().yzxx * float4(other.group0().zxy, other[e23]))
        );
Line right_dual = Line::from_groups(
            /* e41, e42, e43 */
            other.group1() * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(-(right_dual[e41] * wedge[e23]) - (right_dual[e42] * wedge[e31]) - (right_dual[e43] * wedge[e12]) - (right_dual[e23] * wedge[e41]) - (right_dual[e31] * wedge[e42]) - (right_dual[e12] * wedge[e43]), 0.0), 
            /* e1, e2, e3, e4 */
            float4((right_dual[e41] * wedge[e321]) + (right_dual[e31] * wedge[e412]), (right_dual[e42] * wedge[e321]) + (right_dual[e12] * wedge[e423]), (right_dual[e43] * wedge[e321]) + (right_dual[e23] * wedge[e431]), -(right_dual[e42] * wedge[e431]) - (right_dual[e43] * wedge[e412])) - (wedge.group4().yzxx * float4(right_dual.group1().zxy, right_dual[e41])), 
            /* e41, e42, e43 */
            float3(wedge[e1234]) * right_dual.group0(), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
extension MultiVector: AntiRejectViaHorizonFrom<Motor> {
    func anti_reject_via_horizon_from(self, other: Motor) -> MultiVector {
MultiVector wedge = MultiVector::from_groups(
            /* scalar, e1234 */
            float2(other[scalar] * self[scalar], (other[e1234] * self[scalar]) + (other[scalar] * self[e1234]) - (other[e41] * self[e23]) - (other[e42] * self[e31]) - (other[e43] * self[e12]) - (other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43])), 
            /* e1, e2, e3, e4 */
            float4(other[scalar]) * self.group1(), 
            /* e41, e42, e43 */
            (float3(other[scalar]) * self.group2()) + (float3(self[scalar]) * other.group0().xyz), 
            /* e23, e31, e12 */
            (float3(other[scalar]) * self.group3()) + (float3(self[scalar]) * other.group1().xyz), 
            /* e423, e431, e412, e321 */
            float4((other[e23] * self[e4]) + (other[scalar] * self[e423]), (other[e31] * self[e4]) + (other[scalar] * self[e431]), (other[e12] * self[e4]) + (other[scalar] * self[e412]), -(other[e31] * self[e2]) - (other[e12] * self[e3])) + float4((other.group0().yzx * self.group1().zxy), other[scalar] * self[e321]) - (self.group1().yzxx * float4(other.group0().zxy, other[e23]))
        );
Motor right_dual = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            other.group1() * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2((right_dual[e1234] * wedge[scalar]) + (right_dual[scalar] * wedge[e1234]) - (right_dual[e41] * wedge[e23]) - (right_dual[e42] * wedge[e31]) - (right_dual[e43] * wedge[e12]) - (right_dual[e23] * wedge[e41]) - (right_dual[e31] * wedge[e42]) - (right_dual[e12] * wedge[e43]), right_dual[e1234] * wedge[e1234]), 
            /* e1, e2, e3, e4 */
            float4((right_dual[e1234] * wedge[e1]) + (right_dual[e31] * wedge[e412]), (right_dual[e1234] * wedge[e2]) + (right_dual[e12] * wedge[e423]), (right_dual[e1234] * wedge[e3]) + (right_dual[e23] * wedge[e431]), -(right_dual[e42] * wedge[e431]) - (right_dual[e43] * wedge[e412])) + (right_dual.group0() * float4(wedge.group4().www, wedge[e4])) - (wedge.group4().yzxx * float4(right_dual.group1().zxy, right_dual[e41])), 
            /* e41, e42, e43 */
            (float3(right_dual[e1234]) * wedge.group2()) + (float3(wedge[e1234]) * right_dual.group0().xyz), 
            /* e23, e31, e12 */
            (float3(right_dual[e1234]) * wedge.group3()) + (float3(wedge[e1234]) * right_dual.group1().xyz), 
            /* e423, e431, e412, e321 */
            float4(right_dual[e1234]) * wedge.group4()
        );
    }
}
extension MultiVector: AntiRejectViaHorizonFrom<MultiVector> {
    func anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
MultiVector wedge = MultiVector::from_groups(
            /* scalar, e1234 */
            float2(other[scalar] * self[scalar], (other[scalar] * self[e1234]) + (other[e1234] * self[scalar]) + (other[e423] * self[e1]) + (other[e431] * self[e2]) + (other[e412] * self[e3]) + (other[e321] * self[e4]) - (other[e1] * self[e423]) - (other[e2] * self[e431]) - (other[e3] * self[e412]) - (other[e4] * self[e321]) - (other[e41] * self[e23]) - (other[e42] * self[e31]) - (other[e43] * self[e12]) - (other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43])), 
            /* e1, e2, e3, e4 */
            (float4(other[scalar]) * self.group1()) + (float4(self[scalar]) * other.group1()), 
            /* e41, e42, e43 */
            (float3(other[scalar]) * self.group2()) + (float3(self[scalar]) * other.group2()) + (float3(self[e4]) * other.group1().xyz) - (float3(other[e4]) * self.group1().xyz), 
            /* e23, e31, e12 */
            (float3(other[scalar]) * self.group3()) + (float3(self[scalar]) * other.group3()) + (other.group1().zxy * self.group1().yzx) - (other.group1().yzx * self.group1().zxy), 
            /* e423, e431, e412, e321 */
            float4((other[e3] * self[e42]) + (other[e4] * self[e23]) + (other[e42] * self[e3]) + (other[e23] * self[e4]), (other[e1] * self[e43]) + (other[e4] * self[e31]) + (other[e43] * self[e1]) + (other[e31] * self[e4]), (other[e2] * self[e41]) + (other[e4] * self[e12]) + (other[e41] * self[e2]) + (other[e12] * self[e4]), -(other[e1] * self[e23]) - (other[e2] * self[e31]) - (other[e3] * self[e12]) - (other[e12] * self[e3])) + (float4(other[scalar]) * self.group4()) + (float4(self[scalar]) * other.group4()) - (self.group1().yzxx * float4(other.group2().zxy, other[e23])) - float4((self.group2().zxy * other.group1().yzx), other[e31] * self[e2])
        );
MultiVector right_dual = MultiVector::from_groups(
            /* scalar, e1234 */
            float2(0.0, other[scalar]), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other[e321] * -1.0), 
            /* e41, e42, e43 */
            other.group3() * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(other[e1], other[e2], other[e3], 0.0)
        );
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2((right_dual[scalar] * wedge[e1234]) + (right_dual[e1234] * wedge[scalar]) + (right_dual[e423] * wedge[e1]) + (right_dual[e431] * wedge[e2]) + (right_dual[e412] * wedge[e3]) + (right_dual[e321] * wedge[e4]) - (right_dual[e1] * wedge[e423]) - (right_dual[e2] * wedge[e431]) - (right_dual[e3] * wedge[e412]) - (right_dual[e4] * wedge[e321]) - (right_dual[e41] * wedge[e23]) - (right_dual[e42] * wedge[e31]) - (right_dual[e43] * wedge[e12]) - (right_dual[e23] * wedge[e41]) - (right_dual[e31] * wedge[e42]) - (right_dual[e12] * wedge[e43]), right_dual[e1234] * wedge[e1234]), 
            /* e1, e2, e3, e4 */
            float4((right_dual[e41] * wedge[e321]) + (right_dual[e31] * wedge[e412]) + (right_dual[e412] * wedge[e31]) + (right_dual[e321] * wedge[e41]), (right_dual[e42] * wedge[e321]) + (right_dual[e12] * wedge[e423]) + (right_dual[e423] * wedge[e12]) + (right_dual[e321] * wedge[e42]), (right_dual[e43] * wedge[e321]) + (right_dual[e23] * wedge[e431]) + (right_dual[e431] * wedge[e23]) + (right_dual[e321] * wedge[e43]), -(right_dual[e43] * wedge[e412]) - (right_dual[e423] * wedge[e41]) - (right_dual[e431] * wedge[e42]) - (right_dual[e412] * wedge[e43])) + (float4(right_dual[e1234]) * wedge.group1()) + (float4(wedge[e1234]) * right_dual.group1()) - (wedge.group4().yzxx * float4(right_dual.group3().zxy, right_dual[e41])) - float4((wedge.group3().zxy * right_dual.group4().yzx), right_dual[e42] * wedge[e431]), 
            /* e41, e42, e43 */
            (float3(right_dual[e1234]) * wedge.group2()) + (float3(wedge[e1234]) * right_dual.group2()) + (right_dual.group4().yzx * wedge.group4().zxy) - (right_dual.group4().zxy * wedge.group4().yzx), 
            /* e23, e31, e12 */
            (float3(right_dual[e1234]) * wedge.group3()) + (float3(right_dual[e321]) * wedge.group4().xyz) + (float3(wedge[e1234]) * right_dual.group3()) - (float3(wedge[e321]) * right_dual.group4().xyz), 
            /* e423, e431, e412, e321 */
            (float4(right_dual[e1234]) * wedge.group4()) + (float4(wedge[e1234]) * right_dual.group4())
        );
    }
}
extension MultiVector: AntiRejectViaHorizonFrom<Plane> {
    func anti_reject_via_horizon_from(self, other: Plane) -> MultiVector {
MultiVector wedge = MultiVector::from_groups(
            /* scalar, e1234 */
            float2(0.0, (self[e1] * other[e423]) + (self[e2] * other[e431]) + (self[e3] * other[e412]) + (self[e4] * other[e321])), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(self[scalar]) * other.group0()
        );
Origin right_dual = Origin::from_groups(
            /* e4 */
            other[e321] * -1.0
        );
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(wedge[e321] * right_dual[e4], 1.0) * float2(-1.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), wedge[e1234] * right_dual[e4]), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
extension MultiVector: AntiRejectViaHorizonFrom<Point> {
    func anti_reject_via_horizon_from(self, other: Point) -> MultiVector {
MultiVector wedge = MultiVector::from_groups(
            /* scalar, e1234 */
            float2(0.0, -(self[e423] * other[e1]) - (self[e431] * other[e2]) - (self[e412] * other[e3]) - (self[e321] * other[e4])), 
            /* e1, e2, e3, e4 */
            float4(self[scalar]) * other.group0(), 
            /* e41, e42, e43 */
            (float3(self[e4]) * other.group0().xyz) - (float3(other[e4]) * self.group1().xyz), 
            /* e23, e31, e12 */
            (self.group1().yzx * other.group0().zxy) - (self.group1().zxy * other.group0().yzx), 
            /* e423, e431, e412, e321 */
            float4((self[e42] * other[e3]) + (self[e23] * other[e4]), (self[e43] * other[e1]) + (self[e31] * other[e4]), (self[e41] * other[e2]) + (self[e12] * other[e4]), -(self[e31] * other[e2]) - (self[e12] * other[e3])) - (other.group0().yzxx * float4(self.group2().zxy, self[e23]))
        );
Plane right_dual = Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4(other[e1], other[e2], other[e3], 0.0)
        );
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2((wedge[e1] * right_dual[e423]) + (wedge[e2] * right_dual[e431]) + (wedge[e3] * right_dual[e412]) + (wedge[e4] * right_dual[e321]), 0.0), 
            /* e1, e2, e3, e4 */
            float4((wedge[e41] * right_dual[e321]) + (wedge[e31] * right_dual[e412]), (wedge[e42] * right_dual[e321]) + (wedge[e12] * right_dual[e423]), (wedge[e43] * right_dual[e321]) + (wedge[e23] * right_dual[e431]), -(wedge[e42] * right_dual[e431]) - (wedge[e43] * right_dual[e412])) - (right_dual.group0().yzxx * float4(wedge.group3().zxy, wedge[e41])), 
            /* e41, e42, e43 */
            (wedge.group4().zxy * right_dual.group0().yzx) - (wedge.group4().yzx * right_dual.group0().zxy), 
            /* e23, e31, e12 */
            (float3(right_dual[e321]) * wedge.group4().xyz) - (float3(wedge[e321]) * right_dual.group0().xyz), 
            /* e423, e431, e412, e321 */
            float4(wedge.group0().yy, wedge[e1234], 0.0) * float4(float3(1.0), 0.0) * float4(right_dual.group0().xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension MultiVector: AntiRejectViaHorizonFrom<Scalar> {
    func anti_reject_via_horizon_from(self, other: Scalar) -> MultiVector {
MultiVector wedge = MultiVector::from_groups(
            /* scalar, e1234 */
            float2(other[scalar]) * self.group0(), 
            /* e1, e2, e3, e4 */
            float4(other[scalar]) * self.group1(), 
            /* e41, e42, e43 */
            float3(other[scalar]) * self.group2(), 
            /* e23, e31, e12 */
            float3(other[scalar]) * self.group3(), 
            /* e423, e431, e412, e321 */
            float4(other[scalar]) * self.group4()
        );
AntiScalar right_dual = AntiScalar::from_groups(
            /* e1234 */
            other[scalar]
        );
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(right_dual[e1234]) * wedge.group0(), 
            /* e1, e2, e3, e4 */
            float4(right_dual[e1234]) * wedge.group1(), 
            /* e41, e42, e43 */
            float3(right_dual[e1234]) * wedge.group2(), 
            /* e23, e31, e12 */
            float3(right_dual[e1234]) * wedge.group3(), 
            /* e423, e431, e412, e321 */
            float4(right_dual[e1234]) * wedge.group4()
        );
    }
}
extension Origin {
    func div(_rhs: anti_reject_via_horizon_from) -> anti_reject_via_horizon_from_partial<Origin> {
        anti_reject_via_horizon_from_partial(this)
    }
}
extension Origin: AntiRejectViaHorizonFrom<DualNum> {
    func anti_reject_via_horizon_from(self, other: DualNum) -> Origin {
        return Origin::from_groups(
            /* e4 */
            powi(other[scalar], 2) * self[e4]
        );
    }
}
extension Origin: AntiRejectViaHorizonFrom<Flector> {
    func anti_reject_via_horizon_from(self, other: Flector) -> Flector {
Motor wedge = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(self[e4]) * float4(other.group0().xyz, other[e321]), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
Flector right_dual = Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other[e321] * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other[e1], other[e2], other[e3], 0.0)
        );
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4((right_dual[e412] * wedge[e31]) + (right_dual[e321] * wedge[e41]), (right_dual[e423] * wedge[e12]) + (right_dual[e321] * wedge[e42]), (right_dual[e431] * wedge[e23]) + (right_dual[e321] * wedge[e43]), -(right_dual[e431] * wedge[e42]) - (right_dual[e412] * wedge[e43])) + (float4(wedge[e1234]) * right_dual.group0()) - (right_dual.group1().yzxx * float4(wedge.group1().zxy, wedge[e41])), 
            /* e423, e431, e412, e321 */
            float4(wedge[e1234]) * right_dual.group1()
        );
    }
}
extension Origin: AntiRejectViaHorizonFrom<Horizon> {
    func anti_reject_via_horizon_from(self, other: Horizon) -> Origin {
        return Origin::from_groups(
            /* e4 */
            powi(other[e321], 2) * self[e4] * -1.0
        );
    }
}
extension Origin: AntiRejectViaHorizonFrom<Line> {
    func anti_reject_via_horizon_from(self, other: Line) -> Origin {
Plane wedge = Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4(self[e4], self[e4], self[e4], 0.0) * float4(other.group1(), 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
Line right_dual = Line::from_groups(
            /* e41, e42, e43 */
            other.group1() * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
        return Origin::from_groups(
            /* e4 */
            -(right_dual[e41] * wedge[e423]) - (right_dual[e42] * wedge[e431]) - (right_dual[e43] * wedge[e412])
        );
    }
}
extension Origin: AntiRejectViaHorizonFrom<Motor> {
    func anti_reject_via_horizon_from(self, other: Motor) -> Flector {
Flector wedge = Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other[scalar] * self[e4]), 
            /* e423, e431, e412, e321 */
            float4(self[e4], self[e4], self[e4], 0.0) * float4(other.group1().xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
Motor right_dual = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            other.group1() * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4((wedge[e412] * right_dual[e31]) + (wedge[e321] * right_dual[e41]), (wedge[e423] * right_dual[e12]) + (wedge[e321] * right_dual[e42]), (wedge[e431] * right_dual[e23]) + (wedge[e321] * right_dual[e43]), -(wedge[e431] * right_dual[e42]) - (wedge[e412] * right_dual[e43])) + (float4(right_dual[e1234]) * wedge.group0()) - (wedge.group1().yzxx * float4(right_dual.group1().zxy, right_dual[e41])), 
            /* e423, e431, e412, e321 */
            float4(right_dual[e1234]) * wedge.group1()
        );
    }
}
extension Origin: AntiRejectViaHorizonFrom<MultiVector> {
    func anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
MultiVector wedge = MultiVector::from_groups(
            /* scalar, e1234 */
            float2(1.0, other[e321] * self[e4]) * float2(0.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other[scalar] * self[e4]), 
            /* e41, e42, e43 */
            float3(self[e4]) * other.group1().xyz, 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(self[e4], self[e4], self[e4], 0.0) * float4(other.group3(), 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
MultiVector right_dual = MultiVector::from_groups(
            /* scalar, e1234 */
            float2(0.0, other[scalar]), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other[e321] * -1.0), 
            /* e41, e42, e43 */
            other.group3() * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(other[e1], other[e2], other[e3], 0.0)
        );
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2((right_dual[scalar] * wedge[e1234]) + (right_dual[e1234] * wedge[scalar]) + (right_dual[e423] * wedge[e1]) + (right_dual[e431] * wedge[e2]) + (right_dual[e412] * wedge[e3]) + (right_dual[e321] * wedge[e4]) - (right_dual[e1] * wedge[e423]) - (right_dual[e2] * wedge[e431]) - (right_dual[e3] * wedge[e412]) - (right_dual[e4] * wedge[e321]) - (right_dual[e41] * wedge[e23]) - (right_dual[e42] * wedge[e31]) - (right_dual[e43] * wedge[e12]) - (right_dual[e23] * wedge[e41]) - (right_dual[e31] * wedge[e42]) - (right_dual[e12] * wedge[e43]), right_dual[e1234] * wedge[e1234]), 
            /* e1, e2, e3, e4 */
            float4((right_dual[e41] * wedge[e321]) + (right_dual[e31] * wedge[e412]) + (right_dual[e412] * wedge[e31]) + (right_dual[e321] * wedge[e41]), (right_dual[e42] * wedge[e321]) + (right_dual[e12] * wedge[e423]) + (right_dual[e423] * wedge[e12]) + (right_dual[e321] * wedge[e42]), (right_dual[e43] * wedge[e321]) + (right_dual[e23] * wedge[e431]) + (right_dual[e431] * wedge[e23]) + (right_dual[e321] * wedge[e43]), -(right_dual[e43] * wedge[e412]) - (right_dual[e423] * wedge[e41]) - (right_dual[e431] * wedge[e42]) - (right_dual[e412] * wedge[e43])) + (float4(right_dual[e1234]) * wedge.group1()) + (float4(wedge[e1234]) * right_dual.group1()) - (wedge.group4().yzxx * float4(right_dual.group3().zxy, right_dual[e41])) - float4((wedge.group3().zxy * right_dual.group4().yzx), right_dual[e42] * wedge[e431]), 
            /* e41, e42, e43 */
            (float3(right_dual[e1234]) * wedge.group2()) + (float3(wedge[e1234]) * right_dual.group2()) + (right_dual.group4().yzx * wedge.group4().zxy) - (right_dual.group4().zxy * wedge.group4().yzx), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(((float3(right_dual[e1234]) * wedge.group4().xyz) + (float3(wedge[e1234]) * right_dual.group4().xyz)), 0.0)
        );
    }
}
extension Origin: AntiRejectViaHorizonFrom<Plane> {
    func anti_reject_via_horizon_from(self, other: Plane) -> Origin {
        return Origin::from_groups(
            /* e4 */
            self[e4] * powi(other[e321], 2) * -1.0
        );
    }
}
extension Origin: AntiRejectViaHorizonFrom<Point> {
    func anti_reject_via_horizon_from(self, other: Point) -> Origin {
Line wedge = Line::from_groups(
            /* e41, e42, e43 */
            float3(self[e4]) * other.group0().xyz, 
            /* e23, e31, e12 */
            float3(0.0)
        );
Plane right_dual = Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4(other[e1], other[e2], other[e3], 0.0)
        );
        return Origin::from_groups(
            /* e4 */
            -(wedge[e41] * right_dual[e423]) - (wedge[e42] * right_dual[e431]) - (wedge[e43] * right_dual[e412])
        );
    }
}
extension Origin: AntiRejectViaHorizonFrom<Scalar> {
    func anti_reject_via_horizon_from(self, other: Scalar) -> Origin {
        return Origin::from_groups(
            /* e4 */
            self[e4] * powi(other[scalar], 2)
        );
    }
}
extension Plane {
    func div(_rhs: anti_reject_via_horizon_from) -> anti_reject_via_horizon_from_partial<Plane> {
        anti_reject_via_horizon_from_partial(this)
    }
}
extension Plane: AntiRejectViaHorizonFrom<DualNum> {
    func anti_reject_via_horizon_from(self, other: DualNum) -> Plane {
        return Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4(other[scalar]) * float4(other[scalar] * self[e423], other[scalar] * self[e431], other[scalar] * self[e412], other[scalar] * self[e321])
        );
    }
}
extension Plane: AntiRejectViaHorizonFrom<Flector> {
    func anti_reject_via_horizon_from(self, other: Flector) -> Flector {
AntiScalar wedge = AntiScalar::from_groups(
            /* e1234 */
            -(other[e1] * self[e423]) - (other[e2] * self[e431]) - (other[e3] * self[e412]) - (other[e4] * self[e321])
        );
Flector right_dual = Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other[e321] * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other[e1], other[e2], other[e3], 0.0)
        );
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(wedge[e1234]) * right_dual.group0(), 
            /* e423, e431, e412, e321 */
            float4(wedge[e1234], wedge[e1234], wedge[e1234], 0.0) * float4(right_dual.group1().xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Plane: AntiRejectViaHorizonFrom<Motor> {
    func anti_reject_via_horizon_from(self, other: Motor) -> Flector {
Plane wedge = Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4(other[scalar]) * self.group0()
        );
Motor right_dual = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            other.group1() * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4((right_dual[e41] * wedge[e321]) + (right_dual[e31] * wedge[e412]), (right_dual[e42] * wedge[e321]) + (right_dual[e12] * wedge[e423]), (right_dual[e43] * wedge[e321]) + (right_dual[e23] * wedge[e431]), -(right_dual[e42] * wedge[e431]) - (right_dual[e43] * wedge[e412])) - (wedge.group0().yzxx * float4(right_dual.group1().zxy, right_dual[e41])), 
            /* e423, e431, e412, e321 */
            float4(right_dual[e1234]) * wedge.group0()
        );
    }
}
extension Plane: AntiRejectViaHorizonFrom<MultiVector> {
    func anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
MultiVector wedge = MultiVector::from_groups(
            /* scalar, e1234 */
            float2(0.0, -(other[e1] * self[e423]) - (other[e2] * self[e431]) - (other[e3] * self[e412]) - (other[e4] * self[e321])), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(other[scalar]) * self.group0()
        );
MultiVector right_dual = MultiVector::from_groups(
            /* scalar, e1234 */
            float2(0.0, other[scalar]), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other[e321] * -1.0), 
            /* e41, e42, e43 */
            other.group3() * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(other[e1], other[e2], other[e3], 0.0)
        );
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2((right_dual[scalar] * wedge[e1234]) + (right_dual[e1234] * wedge[scalar]) + (right_dual[e423] * wedge[e1]) + (right_dual[e431] * wedge[e2]) + (right_dual[e412] * wedge[e3]) + (right_dual[e321] * wedge[e4]) - (right_dual[e1] * wedge[e423]) - (right_dual[e2] * wedge[e431]) - (right_dual[e3] * wedge[e412]) - (right_dual[e4] * wedge[e321]) - (right_dual[e41] * wedge[e23]) - (right_dual[e42] * wedge[e31]) - (right_dual[e43] * wedge[e12]) - (right_dual[e23] * wedge[e41]) - (right_dual[e31] * wedge[e42]) - (right_dual[e12] * wedge[e43]), right_dual[e1234] * wedge[e1234]), 
            /* e1, e2, e3, e4 */
            float4((right_dual[e41] * wedge[e321]) + (right_dual[e31] * wedge[e412]) + (right_dual[e412] * wedge[e31]) + (right_dual[e321] * wedge[e41]), (right_dual[e42] * wedge[e321]) + (right_dual[e12] * wedge[e423]) + (right_dual[e423] * wedge[e12]) + (right_dual[e321] * wedge[e42]), (right_dual[e43] * wedge[e321]) + (right_dual[e23] * wedge[e431]) + (right_dual[e431] * wedge[e23]) + (right_dual[e321] * wedge[e43]), -(right_dual[e43] * wedge[e412]) - (right_dual[e423] * wedge[e41]) - (right_dual[e431] * wedge[e42]) - (right_dual[e412] * wedge[e43])) + (float4(right_dual[e1234]) * wedge.group1()) + (float4(wedge[e1234]) * right_dual.group1()) - (wedge.group4().yzxx * float4(right_dual.group3().zxy, right_dual[e41])) - float4((wedge.group3().zxy * right_dual.group4().yzx), right_dual[e42] * wedge[e431]), 
            /* e41, e42, e43 */
            (float3(right_dual[e1234]) * wedge.group2()) + (float3(wedge[e1234]) * right_dual.group2()) + (right_dual.group4().yzx * wedge.group4().zxy) - (right_dual.group4().zxy * wedge.group4().yzx), 
            /* e23, e31, e12 */
            (float3(right_dual[e1234]) * wedge.group3()) + (float3(right_dual[e321]) * wedge.group4().xyz) + (float3(wedge[e1234]) * right_dual.group3()) - (float3(wedge[e321]) * right_dual.group4().xyz), 
            /* e423, e431, e412, e321 */
            (float4(right_dual[e1234]) * wedge.group4()) + (float4(wedge[e1234]) * right_dual.group4())
        );
    }
}
extension Plane: AntiRejectViaHorizonFrom<Point> {
    func anti_reject_via_horizon_from(self, other: Point) -> Plane {
AntiScalar wedge = AntiScalar::from_groups(
            /* e1234 */
            -(self[e423] * other[e1]) - (self[e431] * other[e2]) - (self[e412] * other[e3]) - (self[e321] * other[e4])
        );
        return Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4(wedge[e1234], wedge[e1234], wedge[e1234], 0.0) * float4(float4(other[e1], other[e2], other[e3], 0.0).xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Plane: AntiRejectViaHorizonFrom<Scalar> {
    func anti_reject_via_horizon_from(self, other: Scalar) -> Plane {
        return Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4(other[scalar]) * float4(self[e423] * other[scalar], self[e431] * other[scalar], self[e412] * other[scalar], self[e321] * other[scalar])
        );
    }
}
extension Point {
    func div(_rhs: anti_reject_via_horizon_from) -> anti_reject_via_horizon_from_partial<Point> {
        anti_reject_via_horizon_from_partial(this)
    }
}
extension Point: AntiRejectViaHorizonFrom<DualNum> {
    func anti_reject_via_horizon_from(self, other: DualNum) -> Point {
        return Point::from_groups(
            /* e1, e2, e3, e4 */
            float4(other[scalar]) * float4(other[scalar] * self[e1], other[scalar] * self[e2], other[scalar] * self[e3], other[scalar] * self[e4])
        );
    }
}
extension Point: AntiRejectViaHorizonFrom<Flector> {
    func anti_reject_via_horizon_from(self, other: Flector) -> Flector {
Motor wedge = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(other[e4] * self[e1] * -1.0, other[e4] * self[e2] * -1.0, other[e4] * self[e3] * -1.0, (other[e431] * self[e2]) + (other[e412] * self[e3]) + (other[e321] * self[e4])) + (self.group0().wwwx * float4(other.group0().xyz, other[e423])), 
            /* e23, e31, e12, scalar */
            float4(((other.group0().zxy * self.group0().yzx) - (other.group0().yzx * self.group0().zxy)), 0.0)
        );
Flector right_dual = Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other[e321] * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other[e1], other[e2], other[e3], 0.0)
        );
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4((right_dual[e412] * wedge[e31]) + (right_dual[e321] * wedge[e41]), (right_dual[e423] * wedge[e12]) + (right_dual[e321] * wedge[e42]), (right_dual[e431] * wedge[e23]) + (right_dual[e321] * wedge[e43]), -(right_dual[e431] * wedge[e42]) - (right_dual[e412] * wedge[e43])) + (float4(wedge[e1234]) * right_dual.group0()) - (right_dual.group1().yzxx * float4(wedge.group1().zxy, wedge[e41])), 
            /* e423, e431, e412, e321 */
            float4(float3(1.0), 0.0) * float4(wedge.group0().www, 0.0) * float4(right_dual.group1().xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Point: AntiRejectViaHorizonFrom<Horizon> {
    func anti_reject_via_horizon_from(self, other: Horizon) -> Origin {
        return Origin::from_groups(
            /* e4 */
            powi(other[e321], 2) * self[e4] * -1.0
        );
    }
}
extension Point: AntiRejectViaHorizonFrom<Line> {
    func anti_reject_via_horizon_from(self, other: Line) -> Point {
Plane wedge = Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4((other[e42] * self[e3]) + (other[e23] * self[e4]), (other[e43] * self[e1]) + (other[e31] * self[e4]), (other[e41] * self[e2]) + (other[e12] * self[e4]), -(other[e31] * self[e2]) - (other[e12] * self[e3])) - (self.group0().yzxx * float4(other.group0().zxy, other[e23]))
        );
Line right_dual = Line::from_groups(
            /* e41, e42, e43 */
            other.group1() * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
        return Point::from_groups(
            /* e1, e2, e3, e4 */
            float4((right_dual[e41] * wedge[e321]) + (right_dual[e31] * wedge[e412]), (right_dual[e42] * wedge[e321]) + (right_dual[e12] * wedge[e423]), (right_dual[e43] * wedge[e321]) + (right_dual[e23] * wedge[e431]), -(right_dual[e42] * wedge[e431]) - (right_dual[e43] * wedge[e412])) - (wedge.group0().yzxx * float4(right_dual.group1().zxy, right_dual[e41]))
        );
    }
}
extension Point: AntiRejectViaHorizonFrom<Motor> {
    func anti_reject_via_horizon_from(self, other: Motor) -> Flector {
Flector wedge = Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(other[scalar]) * self.group0(), 
            /* e423, e431, e412, e321 */
            float4((other[e42] * self[e3]) + (other[e23] * self[e4]), (other[e43] * self[e1]) + (other[e31] * self[e4]), (other[e41] * self[e2]) + (other[e12] * self[e4]), -(other[e31] * self[e2]) - (other[e12] * self[e3])) - (self.group0().yzxx * float4(other.group0().zxy, other[e23]))
        );
Motor right_dual = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            other.group1() * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4((wedge[e412] * right_dual[e31]) + (wedge[e321] * right_dual[e41]), (wedge[e423] * right_dual[e12]) + (wedge[e321] * right_dual[e42]), (wedge[e431] * right_dual[e23]) + (wedge[e321] * right_dual[e43]), -(wedge[e431] * right_dual[e42]) - (wedge[e412] * right_dual[e43])) + (float4(right_dual[e1234]) * wedge.group0()) - (wedge.group1().yzxx * float4(right_dual.group1().zxy, right_dual[e41])), 
            /* e423, e431, e412, e321 */
            float4(right_dual[e1234]) * wedge.group1()
        );
    }
}
extension Point: AntiRejectViaHorizonFrom<MultiVector> {
    func anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
MultiVector wedge = MultiVector::from_groups(
            /* scalar, e1234 */
            float2(0.0, (other[e423] * self[e1]) + (other[e431] * self[e2]) + (other[e412] * self[e3]) + (other[e321] * self[e4])), 
            /* e1, e2, e3, e4 */
            float4(other[scalar]) * self.group0(), 
            /* e41, e42, e43 */
            (float3(self[e4]) * other.group1().xyz) - (float3(other[e4]) * self.group0().xyz), 
            /* e23, e31, e12 */
            (other.group1().zxy * self.group0().yzx) - (other.group1().yzx * self.group0().zxy), 
            /* e423, e431, e412, e321 */
            float4((other[e42] * self[e3]) + (other[e23] * self[e4]), (other[e43] * self[e1]) + (other[e31] * self[e4]), (other[e41] * self[e2]) + (other[e12] * self[e4]), -(other[e31] * self[e2]) - (other[e12] * self[e3])) - (self.group0().yzxx * float4(other.group2().zxy, other[e23]))
        );
MultiVector right_dual = MultiVector::from_groups(
            /* scalar, e1234 */
            float2(0.0, other[scalar]), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other[e321] * -1.0), 
            /* e41, e42, e43 */
            other.group3() * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(other[e1], other[e2], other[e3], 0.0)
        );
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2((right_dual[scalar] * wedge[e1234]) + (right_dual[e1234] * wedge[scalar]) + (right_dual[e423] * wedge[e1]) + (right_dual[e431] * wedge[e2]) + (right_dual[e412] * wedge[e3]) + (right_dual[e321] * wedge[e4]) - (right_dual[e1] * wedge[e423]) - (right_dual[e2] * wedge[e431]) - (right_dual[e3] * wedge[e412]) - (right_dual[e4] * wedge[e321]) - (right_dual[e41] * wedge[e23]) - (right_dual[e42] * wedge[e31]) - (right_dual[e43] * wedge[e12]) - (right_dual[e23] * wedge[e41]) - (right_dual[e31] * wedge[e42]) - (right_dual[e12] * wedge[e43]), right_dual[e1234] * wedge[e1234]), 
            /* e1, e2, e3, e4 */
            float4((right_dual[e41] * wedge[e321]) + (right_dual[e31] * wedge[e412]) + (right_dual[e412] * wedge[e31]) + (right_dual[e321] * wedge[e41]), (right_dual[e42] * wedge[e321]) + (right_dual[e12] * wedge[e423]) + (right_dual[e423] * wedge[e12]) + (right_dual[e321] * wedge[e42]), (right_dual[e43] * wedge[e321]) + (right_dual[e23] * wedge[e431]) + (right_dual[e431] * wedge[e23]) + (right_dual[e321] * wedge[e43]), -(right_dual[e43] * wedge[e412]) - (right_dual[e423] * wedge[e41]) - (right_dual[e431] * wedge[e42]) - (right_dual[e412] * wedge[e43])) + (float4(right_dual[e1234]) * wedge.group1()) + (float4(wedge[e1234]) * right_dual.group1()) - (wedge.group4().yzxx * float4(right_dual.group3().zxy, right_dual[e41])) - float4((wedge.group3().zxy * right_dual.group4().yzx), right_dual[e42] * wedge[e431]), 
            /* e41, e42, e43 */
            (float3(right_dual[e1234]) * wedge.group2()) + (float3(wedge[e1234]) * right_dual.group2()) + (right_dual.group4().yzx * wedge.group4().zxy) - (right_dual.group4().zxy * wedge.group4().yzx), 
            /* e23, e31, e12 */
            (float3(right_dual[e1234]) * wedge.group3()) + (float3(right_dual[e321]) * wedge.group4().xyz) + (float3(wedge[e1234]) * right_dual.group3()) - (float3(wedge[e321]) * right_dual.group4().xyz), 
            /* e423, e431, e412, e321 */
            (float4(right_dual[e1234]) * wedge.group4()) + (float4(wedge[e1234]) * right_dual.group4())
        );
    }
}
extension Point: AntiRejectViaHorizonFrom<Plane> {
    func anti_reject_via_horizon_from(self, other: Plane) -> Origin {
        return Origin::from_groups(
            /* e4 */
            -(powi(other[e321], 2) * self[e4]) - (other[e423] * other[e321] * self[e1]) - (other[e431] * other[e321] * self[e2]) - (other[e412] * other[e321] * self[e3])
        );
    }
}
extension Point: AntiRejectViaHorizonFrom<Point> {
    func anti_reject_via_horizon_from(self, other: Point) -> Point {
Line wedge = Line::from_groups(
            /* e41, e42, e43 */
            (float3(self[e4]) * other.group0().xyz) - (float3(other[e4]) * self.group0().xyz), 
            /* e23, e31, e12 */
            (other.group0().zxy * self.group0().yzx) - (other.group0().yzx * self.group0().zxy)
        );
Plane right_dual = Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4(other[e1], other[e2], other[e3], 0.0)
        );
        return Point::from_groups(
            /* e1, e2, e3, e4 */
            float4((wedge[e41] * right_dual[e321]) + (wedge[e31] * right_dual[e412]), (wedge[e42] * right_dual[e321]) + (wedge[e12] * right_dual[e423]), (wedge[e43] * right_dual[e321]) + (wedge[e23] * right_dual[e431]), -(wedge[e42] * right_dual[e431]) - (wedge[e43] * right_dual[e412])) - (right_dual.group0().yzxx * float4(wedge.group1().zxy, wedge[e41]))
        );
    }
}
extension Point: AntiRejectViaHorizonFrom<Scalar> {
    func anti_reject_via_horizon_from(self, other: Scalar) -> Point {
        return Point::from_groups(
            /* e1, e2, e3, e4 */
            float4(other[scalar]) * float4(self[e1] * other[scalar], self[e2] * other[scalar], self[e3] * other[scalar], self[e4] * other[scalar])
        );
    }
}
extension Scalar {
    func div(_rhs: anti_reject_via_horizon_from) -> anti_reject_via_horizon_from_partial<Scalar> {
        anti_reject_via_horizon_from_partial(this)
    }
}
extension Scalar: AntiRejectViaHorizonFrom<DualNum> {
    func anti_reject_via_horizon_from(self, other: DualNum) -> DualNum {
        return DualNum::from_groups(
            /* scalar, e1234 */
            float2(other[scalar]) * float2(other[scalar] * self[scalar], other[e1234] * self[scalar])
        );
    }
}
extension Scalar: AntiRejectViaHorizonFrom<Flector> {
    func anti_reject_via_horizon_from(self, other: Flector) -> Motor {
Flector wedge = Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(self[scalar]) * other.group0(), 
            /* e423, e431, e412, e321 */
            float4(self[scalar]) * other.group1()
        );
Flector right_dual = Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other[e321] * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other[e1], other[e2], other[e3], 0.0)
        );
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(((right_dual.group1().yzx * wedge.group1().zxy) - (right_dual.group1().zxy * wedge.group1().yzx)), 0.0), 
            /* e23, e31, e12, scalar */
            (right_dual.group1().wwwx * float4(wedge.group1().xyz, wedge[e1])) + float4(float3(0.0), (right_dual[e431] * wedge[e2]) + (right_dual[e412] * wedge[e3]) + (right_dual[e321] * wedge[e4]) - (right_dual[e2] * wedge[e431]) - (right_dual[e3] * wedge[e412]) - (right_dual[e4] * wedge[e321])) - (wedge.group1().wwwx * float4(right_dual.group1().xyz, right_dual[e1]))
        );
    }
}
extension Scalar: AntiRejectViaHorizonFrom<Horizon> {
    func anti_reject_via_horizon_from(self, other: Horizon) -> Scalar {
        return Scalar::from_groups(
            /* scalar */
            powi(other[e321], 2) * self[scalar]
        );
    }
}
extension Scalar: AntiRejectViaHorizonFrom<Line> {
    func anti_reject_via_horizon_from(self, other: Line) -> Scalar {
Line wedge = Line::from_groups(
            /* e41, e42, e43 */
            float3(self[scalar]) * other.group0(), 
            /* e23, e31, e12 */
            float3(self[scalar]) * other.group1()
        );
Line right_dual = Line::from_groups(
            /* e41, e42, e43 */
            other.group1() * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
        return Scalar::from_groups(
            /* scalar */
            -(right_dual[e41] * wedge[e23]) - (right_dual[e42] * wedge[e31]) - (right_dual[e43] * wedge[e12]) - (right_dual[e23] * wedge[e41]) - (right_dual[e31] * wedge[e42]) - (right_dual[e12] * wedge[e43])
        );
    }
}
extension Scalar: AntiRejectViaHorizonFrom<Motor> {
    func anti_reject_via_horizon_from(self, other: Motor) -> Motor {
Motor wedge = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(self[scalar]) * other.group0(), 
            /* e23, e31, e12, scalar */
            float4(self[scalar]) * other.group1()
        );
Motor right_dual = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            other.group1() * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(((float3(right_dual[e1234]) * wedge.group0().xyz) + (float3(wedge[e1234]) * right_dual.group0().xyz)), right_dual[e1234] * wedge[e1234]), 
            /* e23, e31, e12, scalar */
            (float4(right_dual[e1234]) * wedge.group1()) + (float4(wedge[e1234]) * right_dual.group1()) + float4(float3(0.0), -(right_dual[e41] * wedge[e23]) - (right_dual[e42] * wedge[e31]) - (right_dual[e43] * wedge[e12]) - (right_dual[e23] * wedge[e41]) - (right_dual[e31] * wedge[e42]) - (right_dual[e12] * wedge[e43]))
        );
    }
}
extension Scalar: AntiRejectViaHorizonFrom<MultiVector> {
    func anti_reject_via_horizon_from(self, other: MultiVector) -> MultiVector {
MultiVector wedge = MultiVector::from_groups(
            /* scalar, e1234 */
            float2(self[scalar]) * other.group0(), 
            /* e1, e2, e3, e4 */
            float4(self[scalar]) * other.group1(), 
            /* e41, e42, e43 */
            float3(self[scalar]) * other.group2(), 
            /* e23, e31, e12 */
            float3(self[scalar]) * other.group3(), 
            /* e423, e431, e412, e321 */
            float4(self[scalar]) * other.group4()
        );
MultiVector right_dual = MultiVector::from_groups(
            /* scalar, e1234 */
            float2(0.0, other[scalar]), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other[e321] * -1.0), 
            /* e41, e42, e43 */
            other.group3() * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(other[e1], other[e2], other[e3], 0.0)
        );
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2((right_dual[scalar] * wedge[e1234]) + (right_dual[e1234] * wedge[scalar]) + (right_dual[e423] * wedge[e1]) + (right_dual[e431] * wedge[e2]) + (right_dual[e412] * wedge[e3]) + (right_dual[e321] * wedge[e4]) - (right_dual[e1] * wedge[e423]) - (right_dual[e2] * wedge[e431]) - (right_dual[e3] * wedge[e412]) - (right_dual[e4] * wedge[e321]) - (right_dual[e41] * wedge[e23]) - (right_dual[e42] * wedge[e31]) - (right_dual[e43] * wedge[e12]) - (right_dual[e23] * wedge[e41]) - (right_dual[e31] * wedge[e42]) - (right_dual[e12] * wedge[e43]), right_dual[e1234] * wedge[e1234]), 
            /* e1, e2, e3, e4 */
            float4((right_dual[e41] * wedge[e321]) + (right_dual[e31] * wedge[e412]) + (right_dual[e412] * wedge[e31]) + (right_dual[e321] * wedge[e41]), (right_dual[e42] * wedge[e321]) + (right_dual[e12] * wedge[e423]) + (right_dual[e423] * wedge[e12]) + (right_dual[e321] * wedge[e42]), (right_dual[e43] * wedge[e321]) + (right_dual[e23] * wedge[e431]) + (right_dual[e431] * wedge[e23]) + (right_dual[e321] * wedge[e43]), -(right_dual[e43] * wedge[e412]) - (right_dual[e423] * wedge[e41]) - (right_dual[e431] * wedge[e42]) - (right_dual[e412] * wedge[e43])) + (float4(right_dual[e1234]) * wedge.group1()) + (float4(wedge[e1234]) * right_dual.group1()) - (wedge.group4().yzxx * float4(right_dual.group3().zxy, right_dual[e41])) - float4((wedge.group3().zxy * right_dual.group4().yzx), right_dual[e42] * wedge[e431]), 
            /* e41, e42, e43 */
            (float3(right_dual[e1234]) * wedge.group2()) + (float3(wedge[e1234]) * right_dual.group2()) + (right_dual.group4().yzx * wedge.group4().zxy) - (right_dual.group4().zxy * wedge.group4().yzx), 
            /* e23, e31, e12 */
            (float3(right_dual[e1234]) * wedge.group3()) + (float3(right_dual[e321]) * wedge.group4().xyz) + (float3(wedge[e1234]) * right_dual.group3()) - (float3(wedge[e321]) * right_dual.group4().xyz), 
            /* e423, e431, e412, e321 */
            (float4(right_dual[e1234]) * wedge.group4()) + (float4(wedge[e1234]) * right_dual.group4())
        );
    }
}
extension Scalar: AntiRejectViaHorizonFrom<Plane> {
    func anti_reject_via_horizon_from(self, other: Plane) -> Scalar {
        return Scalar::from_groups(
            /* scalar */
            powi(other[e321], 2) * self[scalar]
        );
    }
}
extension Scalar: AntiRejectViaHorizonFrom<Point> {
    func anti_reject_via_horizon_from(self, other: Point) -> Scalar {
Point wedge = Point::from_groups(
            /* e1, e2, e3, e4 */
            float4(self[scalar]) * other.group0()
        );
Plane right_dual = Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4(other[e1], other[e2], other[e3], 0.0)
        );
        return Scalar::from_groups(
            /* scalar */
            (right_dual[e423] * wedge[e1]) + (right_dual[e431] * wedge[e2]) + (right_dual[e412] * wedge[e3]) + (right_dual[e321] * wedge[e4])
        );
    }
}
extension Scalar: AntiRejectViaHorizonFrom<Scalar> {
    func anti_reject_via_horizon_from(self, other: Scalar) -> Scalar {
        return Scalar::from_groups(
            /* scalar */
            powi(other[scalar], 2) * self[scalar]
        );
    }
}
