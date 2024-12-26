extension AntiScalar {
    func div(_rhs: project_orthogonally_onto) -> project_orthogonally_onto_partial<AntiScalar> {
        project_orthogonally_onto_partial(this)
    }
}
extension AntiScalar: ProjectOrthogonallyOnto<Motor> {
    func project_orthogonally_onto(self, other: Motor) -> Motor {
AntiScalar wedge = AntiScalar::from_groups(
            /* e1234 */
            self[e1234] * 0.0
        );
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(wedge[e1234]) * other.group0(), 
            /* e23, e31, e12, scalar */
            float4(wedge[e1234]) * other.group1()
        );
    }
}
extension AntiScalar: ProjectOrthogonallyOnto<MultiVector> {
    func project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
AntiScalar wedge = AntiScalar::from_groups(
            /* e1234 */
            self[e1234] * 0.0
        );
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(wedge[e1234]) * other.group0(), 
            /* e1, e2, e3, e4 */
            float4(wedge[e1234]) * other.group1(), 
            /* e41, e42, e43 */
            float3(wedge[e1234]) * other.group2(), 
            /* e23, e31, e12 */
            float3(wedge[e1234]) * other.group3(), 
            /* e423, e431, e412, e321 */
            float4(wedge[e1234]) * other.group4()
        );
    }
}
extension DualNum {
    func div(_rhs: project_orthogonally_onto) -> project_orthogonally_onto_partial<DualNum> {
        project_orthogonally_onto_partial(this)
    }
}
extension DualNum: ProjectOrthogonallyOnto<DualNum> {
    func project_orthogonally_onto(self, other: DualNum) -> DualNum {
        return DualNum::from_groups(
            /* scalar, e1234 */
            float2(other[scalar] * self[scalar]) * other.group0()
        );
    }
}
extension DualNum: ProjectOrthogonallyOnto<Flector> {
    func project_orthogonally_onto(self, other: Flector) -> Motor {
Flector right_anti_dual = Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other[e321] * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other[e1], other[e2], other[e3], 0.0)
        );
Flector wedge = Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(self[scalar]) * right_anti_dual.group0(), 
            /* e423, e431, e412, e321 */
            float4(self[scalar]) * right_anti_dual.group1()
        );
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(((other.group1().zxy * wedge.group1().yzx) - (other.group1().yzx * wedge.group1().zxy)), 0.0), 
            /* e23, e31, e12, scalar */
            (wedge.group1().wwwx * float4(other.group1().xyz, other[e1])) + float4(float3(0.0), (other[e2] * wedge[e431]) + (other[e3] * wedge[e412]) + (other[e4] * wedge[e321]) - (other[e431] * wedge[e2]) - (other[e412] * wedge[e3]) - (other[e321] * wedge[e4])) - (other.group1().wwwx * float4(wedge.group1().xyz, wedge[e1]))
        );
    }
}
extension DualNum: ProjectOrthogonallyOnto<Horizon> {
    func project_orthogonally_onto(self, other: Horizon) -> Scalar {
        return Scalar::from_groups(
            /* scalar */
            powi(other[e321], 2) * self[scalar]
        );
    }
}
extension DualNum: ProjectOrthogonallyOnto<Line> {
    func project_orthogonally_onto(self, other: Line) -> Scalar {
Line right_anti_dual = Line::from_groups(
            /* e41, e42, e43 */
            other.group1() * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
Line wedge = Line::from_groups(
            /* e41, e42, e43 */
            float3(self[scalar]) * right_anti_dual.group0(), 
            /* e23, e31, e12 */
            float3(self[scalar]) * right_anti_dual.group1()
        );
        return Scalar::from_groups(
            /* scalar */
            -(other[e41] * wedge[e23]) - (other[e42] * wedge[e31]) - (other[e43] * wedge[e12]) - (other[e23] * wedge[e41]) - (other[e31] * wedge[e42]) - (other[e12] * wedge[e43])
        );
    }
}
extension DualNum: ProjectOrthogonallyOnto<Motor> {
    func project_orthogonally_onto(self, other: Motor) -> Motor {
Motor right_anti_dual = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            other.group1() * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
Motor wedge = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(right_anti_dual[e41], right_anti_dual[e42], right_anti_dual[e43], 1.0) * float4(self.group0().xx, self[scalar], (self[scalar] * right_anti_dual[e1234]) + (self[e1234] * right_anti_dual[scalar])), 
            /* e23, e31, e12, scalar */
            float4(self[scalar]) * right_anti_dual.group1()
        );
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(((float3(other[e1234]) * wedge.group0().xyz) + (float3(wedge[e1234]) * other.group0().xyz)), other[e1234] * wedge[e1234]), 
            /* e23, e31, e12, scalar */
            (float4(other[e1234]) * wedge.group1()) + (float4(wedge[e1234]) * other.group1()) + float4(float3(0.0), -(other[e41] * wedge[e23]) - (other[e42] * wedge[e31]) - (other[e43] * wedge[e12]) - (other[e23] * wedge[e41]) - (other[e31] * wedge[e42]) - (other[e12] * wedge[e43]))
        );
    }
}
extension DualNum: ProjectOrthogonallyOnto<MultiVector> {
    func project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
MultiVector right_anti_dual = MultiVector::from_groups(
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
MultiVector wedge = MultiVector::from_groups(
            /* scalar, e1234 */
            float2(self[scalar] * right_anti_dual[scalar], (self[scalar] * right_anti_dual[e1234]) + (self[e1234] * right_anti_dual[scalar])), 
            /* e1, e2, e3, e4 */
            float4(self[scalar]) * right_anti_dual.group1(), 
            /* e41, e42, e43 */
            float3(self[scalar]) * right_anti_dual.group2(), 
            /* e23, e31, e12 */
            float3(self[scalar]) * right_anti_dual.group3(), 
            /* e423, e431, e412, e321 */
            float4(self[scalar]) * right_anti_dual.group4()
        );
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2((other[scalar] * wedge[e1234]) + (other[e1234] * wedge[scalar]) + (other[e1] * wedge[e423]) + (other[e2] * wedge[e431]) + (other[e3] * wedge[e412]) + (other[e4] * wedge[e321]) - (other[e41] * wedge[e23]) - (other[e42] * wedge[e31]) - (other[e43] * wedge[e12]) - (other[e23] * wedge[e41]) - (other[e31] * wedge[e42]) - (other[e12] * wedge[e43]) - (other[e423] * wedge[e1]) - (other[e431] * wedge[e2]) - (other[e412] * wedge[e3]) - (other[e321] * wedge[e4]), other[e1234] * wedge[e1234]), 
            /* e1, e2, e3, e4 */
            float4((other[e41] * wedge[e321]) + (other[e31] * wedge[e412]) + (other[e412] * wedge[e31]) + (other[e321] * wedge[e41]), (other[e42] * wedge[e321]) + (other[e12] * wedge[e423]) + (other[e423] * wedge[e12]) + (other[e321] * wedge[e42]), (other[e43] * wedge[e321]) + (other[e23] * wedge[e431]) + (other[e431] * wedge[e23]) + (other[e321] * wedge[e43]), -(other[e43] * wedge[e412]) - (other[e423] * wedge[e41]) - (other[e431] * wedge[e42]) - (other[e412] * wedge[e43])) + (float4(other[e1234]) * wedge.group1()) + (float4(wedge[e1234]) * other.group1()) - (wedge.group4().yzxx * float4(other.group3().zxy, other[e41])) - float4((wedge.group3().zxy * other.group4().yzx), other[e42] * wedge[e431]), 
            /* e41, e42, e43 */
            (float3(other[e1234]) * wedge.group2()) + (float3(wedge[e1234]) * other.group2()) + (other.group4().zxy * wedge.group4().yzx) - (other.group4().yzx * wedge.group4().zxy), 
            /* e23, e31, e12 */
            (float3(other[e1234]) * wedge.group3()) + (float3(wedge[e1234]) * other.group3()) + (float3(wedge[e321]) * other.group4().xyz) - (float3(other[e321]) * wedge.group4().xyz), 
            /* e423, e431, e412, e321 */
            (float4(other[e1234]) * wedge.group4()) + (float4(wedge[e1234]) * other.group4())
        );
    }
}
extension DualNum: ProjectOrthogonallyOnto<Plane> {
    func project_orthogonally_onto(self, other: Plane) -> Scalar {
        return Scalar::from_groups(
            /* scalar */
            powi(other[e321], 2) * self[scalar]
        );
    }
}
extension DualNum: ProjectOrthogonallyOnto<Point> {
    func project_orthogonally_onto(self, other: Point) -> Scalar {
Plane wedge = Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4(self[scalar]) * float4(other[e1], other[e2], other[e3], 0.0)
        );
        return Scalar::from_groups(
            /* scalar */
            (wedge[e423] * other[e1]) + (wedge[e431] * other[e2]) + (wedge[e412] * other[e3]) + (wedge[e321] * other[e4])
        );
    }
}
extension DualNum: ProjectOrthogonallyOnto<Scalar> {
    func project_orthogonally_onto(self, other: Scalar) -> Scalar {
        return Scalar::from_groups(
            /* scalar */
            powi(other[scalar], 2) * self[scalar]
        );
    }
}
extension Flector {
    func div(_rhs: project_orthogonally_onto) -> project_orthogonally_onto_partial<Flector> {
        project_orthogonally_onto_partial(this)
    }
}
extension Flector: ProjectOrthogonallyOnto<Flector> {
    func project_orthogonally_onto(self, other: Flector) -> Flector {
Flector right_anti_dual = Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other[e321] * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other[e1], other[e2], other[e3], 0.0)
        );
Motor wedge = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            (self.group0().wwwx * float4(right_anti_dual.group0().xyz, right_anti_dual[e423])) + float4(float3(0.0), (right_anti_dual[e431] * self[e2]) + (right_anti_dual[e412] * self[e3]) + (right_anti_dual[e321] * self[e4]) - (right_anti_dual[e2] * self[e431]) - (right_anti_dual[e3] * self[e412]) - (right_anti_dual[e4] * self[e321])) - (right_anti_dual.group0().wwwx * float4(self.group0().xyz, self[e423])), 
            /* e23, e31, e12, scalar */
            float4(((right_anti_dual.group0().zxy * self.group0().yzx) - (right_anti_dual.group0().yzx * self.group0().zxy)), 0.0)
        );
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4((other[e412] * wedge[e31]) + (other[e321] * wedge[e41]), (other[e423] * wedge[e12]) + (other[e321] * wedge[e42]), (other[e431] * wedge[e23]) + (other[e321] * wedge[e43]), -(other[e431] * wedge[e42]) - (other[e412] * wedge[e43])) + (float4(wedge[e1234]) * other.group0()) - (other.group1().yzxx * float4(wedge.group1().zxy, wedge[e41])), 
            /* e423, e431, e412, e321 */
            float4(wedge[e1234]) * other.group1()
        );
    }
}
extension Flector: ProjectOrthogonallyOnto<Horizon> {
    func project_orthogonally_onto(self, other: Horizon) -> Flector {
Motor wedge = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(other[e321] * -1.0) * float4(self.group0().xyz, self[e321]) * float4(-1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(other[e321], other[e321], other[e321], 0.0) * float4(wedge.group0().xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other[e321] * wedge[e1234])
        );
    }
}
extension Flector: ProjectOrthogonallyOnto<Line> {
    func project_orthogonally_onto(self, other: Line) -> Point {
Line right_anti_dual = Line::from_groups(
            /* e41, e42, e43 */
            other.group1() * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
Plane wedge = Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4((float3(self[e4]) * right_anti_dual.group1()), 0.0) + float4((right_anti_dual.group0().yzx * self.group0().zxy), 0.0) - float4((right_anti_dual.group0().zxy * self.group0().yzx), 0.0)
        );
        return Point::from_groups(
            /* e1, e2, e3, e4 */
            float4((other[e41] * wedge[e321]) + (other[e31] * wedge[e412]), (other[e42] * wedge[e321]) + (other[e12] * wedge[e423]), (other[e43] * wedge[e321]) + (other[e23] * wedge[e431]), -(other[e42] * wedge[e431]) - (other[e43] * wedge[e412])) - (wedge.group0().yzxx * float4(other.group1().zxy, other[e41]))
        );
    }
}
extension Flector: ProjectOrthogonallyOnto<Motor> {
    func project_orthogonally_onto(self, other: Motor) -> Flector {
Motor right_anti_dual = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            other.group1() * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
Flector wedge = Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(right_anti_dual[scalar]) * self.group0(), 
            /* e423, e431, e412, e321 */
            float4((float3(self[e4]) * right_anti_dual.group1().xyz), 0.0) + float4((float3(right_anti_dual[scalar]) * self.group1().xyz), 0.0) + float4((self.group0().zxy * right_anti_dual.group0().yzx), 0.0) - float4((self.group0().yzx * right_anti_dual.group0().zxy), 0.0)
        );
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4((wedge[e412] * other[e31]) + (wedge[e321] * other[e41]), (wedge[e423] * other[e12]) + (wedge[e321] * other[e42]), (wedge[e431] * other[e23]) + (wedge[e321] * other[e43]), -(wedge[e431] * other[e42]) - (wedge[e412] * other[e43])) + (float4(other[e1234]) * wedge.group0()) - (wedge.group1().yzxx * float4(other.group1().zxy, other[e41])), 
            /* e423, e431, e412, e321 */
            float4(other[e1234]) * wedge.group1()
        );
    }
}
extension Flector: ProjectOrthogonallyOnto<MultiVector> {
    func project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
MultiVector right_anti_dual = MultiVector::from_groups(
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
MultiVector wedge = MultiVector::from_groups(
            /* scalar, e1234 */
            float2(0.0, (self[e1] * right_anti_dual[e423]) + (self[e2] * right_anti_dual[e431]) + (self[e3] * right_anti_dual[e412]) + (self[e4] * right_anti_dual[e321]) - (self[e423] * right_anti_dual[e1]) - (self[e431] * right_anti_dual[e2]) - (self[e412] * right_anti_dual[e3]) - (self[e321] * right_anti_dual[e4])), 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual[scalar]) * self.group0(), 
            /* e41, e42, e43 */
            (float3(self[e4]) * right_anti_dual.group1().xyz) - (float3(right_anti_dual[e4]) * self.group0().xyz), 
            /* e23, e31, e12 */
            (self.group0().yzx * right_anti_dual.group1().zxy) - (self.group0().zxy * right_anti_dual.group1().yzx), 
            /* e423, e431, e412, e321 */
            float4((float3(self[e4]) * right_anti_dual.group3()), 0.0) + float4((float3(right_anti_dual[scalar]) * self.group1().xyz), 0.0) + float4((right_anti_dual.group2().yzx * self.group0().zxy), 0.0) - float4((right_anti_dual.group2().zxy * self.group0().yzx), 0.0)
        );
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2((other[scalar] * wedge[e1234]) + (other[e1234] * wedge[scalar]) + (other[e1] * wedge[e423]) + (other[e2] * wedge[e431]) + (other[e3] * wedge[e412]) + (other[e4] * wedge[e321]) - (other[e41] * wedge[e23]) - (other[e42] * wedge[e31]) - (other[e43] * wedge[e12]) - (other[e23] * wedge[e41]) - (other[e31] * wedge[e42]) - (other[e12] * wedge[e43]) - (other[e423] * wedge[e1]) - (other[e431] * wedge[e2]) - (other[e412] * wedge[e3]) - (other[e321] * wedge[e4]), other[e1234] * wedge[e1234]), 
            /* e1, e2, e3, e4 */
            float4((other[e41] * wedge[e321]) + (other[e31] * wedge[e412]) + (other[e412] * wedge[e31]) + (other[e321] * wedge[e41]), (other[e42] * wedge[e321]) + (other[e12] * wedge[e423]) + (other[e423] * wedge[e12]) + (other[e321] * wedge[e42]), (other[e43] * wedge[e321]) + (other[e23] * wedge[e431]) + (other[e431] * wedge[e23]) + (other[e321] * wedge[e43]), -(other[e43] * wedge[e412]) - (other[e423] * wedge[e41]) - (other[e431] * wedge[e42]) - (other[e412] * wedge[e43])) + (float4(other[e1234]) * wedge.group1()) + (float4(wedge[e1234]) * other.group1()) - (wedge.group4().yzxx * float4(other.group3().zxy, other[e41])) - float4((wedge.group3().zxy * other.group4().yzx), other[e42] * wedge[e431]), 
            /* e41, e42, e43 */
            (float3(other[e1234]) * wedge.group2()) + (float3(wedge[e1234]) * other.group2()) + (other.group4().zxy * wedge.group4().yzx) - (other.group4().yzx * wedge.group4().zxy), 
            /* e23, e31, e12 */
            (float3(other[e1234]) * wedge.group3()) + (float3(wedge[e1234]) * other.group3()) + (float3(wedge[e321]) * other.group4().xyz) - (float3(other[e321]) * wedge.group4().xyz), 
            /* e423, e431, e412, e321 */
            (float4(other[e1234]) * wedge.group4()) + (float4(wedge[e1234]) * other.group4())
        );
    }
}
extension Flector: ProjectOrthogonallyOnto<Plane> {
    func project_orthogonally_onto(self, other: Plane) -> Flector {
Motor wedge = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(other[e321] * -1.0) * float4(self.group0().xyz, self[e321]) * float4(-1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4((wedge[e41] * other[e321]) + (wedge[e31] * other[e412]), (wedge[e42] * other[e321]) + (wedge[e12] * other[e423]), (wedge[e43] * other[e321]) + (wedge[e23] * other[e431]), -(wedge[e42] * other[e431]) - (wedge[e43] * other[e412])) - (other.group0().yzxx * float4(wedge.group1().zxy, wedge[e41])), 
            /* e423, e431, e412, e321 */
            float4(wedge[e1234]) * other.group0()
        );
    }
}
extension Flector: ProjectOrthogonallyOnto<Point> {
    func project_orthogonally_onto(self, other: Point) -> Point {
Plane right_anti_dual = Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4(other[e1], other[e2], other[e3], 0.0)
        );
        return Point::from_groups(
            /* e1, e2, e3, e4 */
            float4((self[e1] * right_anti_dual[e423]) + (self[e2] * right_anti_dual[e431]) + (self[e3] * right_anti_dual[e412]) + (self[e4] * right_anti_dual[e321])) * other.group0()
        );
    }
}
extension Horizon {
    func div(_rhs: project_orthogonally_onto) -> project_orthogonally_onto_partial<Horizon> {
        project_orthogonally_onto_partial(this)
    }
}
extension Horizon: ProjectOrthogonallyOnto<Flector> {
    func project_orthogonally_onto(self, other: Flector) -> Flector {
AntiScalar wedge = AntiScalar::from_groups(
            /* e1234 */
            float4(float3(0.0), other[e321] * -1.0)[3] * self[e321] * -1.0
        );
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(wedge[e1234]) * other.group0(), 
            /* e423, e431, e412, e321 */
            float4(wedge[e1234]) * other.group1()
        );
    }
}
extension Horizon: ProjectOrthogonallyOnto<Horizon> {
    func project_orthogonally_onto(self, other: Horizon) -> Horizon {
        return Horizon::from_groups(
            /* e321 */
            powi(other[e321], 2) * self[e321]
        );
    }
}
extension Horizon: ProjectOrthogonallyOnto<Motor> {
    func project_orthogonally_onto(self, other: Motor) -> Flector {
Horizon wedge = Horizon::from_groups(
            /* e321 */
            self[e321] * 0.0
        );
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(wedge[e321], wedge[e321], wedge[e321], 0.0) * float4(other.group0().xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), wedge[e321] * other[e1234])
        );
    }
}
extension Horizon: ProjectOrthogonallyOnto<MultiVector> {
    func project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
MultiVector right_anti_dual = MultiVector::from_groups(
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
MultiVector wedge = MultiVector::from_groups(
            /* scalar, e1234 */
            float2(1.0, self[e321] * right_anti_dual[e4]) * float2(0.0, -1.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), self[e321] * right_anti_dual[scalar])
        );
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2((other[scalar] * wedge[e1234]) + (other[e1234] * wedge[scalar]) + (other[e1] * wedge[e423]) + (other[e2] * wedge[e431]) + (other[e3] * wedge[e412]) + (other[e4] * wedge[e321]) - (other[e41] * wedge[e23]) - (other[e42] * wedge[e31]) - (other[e43] * wedge[e12]) - (other[e23] * wedge[e41]) - (other[e31] * wedge[e42]) - (other[e12] * wedge[e43]) - (other[e423] * wedge[e1]) - (other[e431] * wedge[e2]) - (other[e412] * wedge[e3]) - (other[e321] * wedge[e4]), other[e1234] * wedge[e1234]), 
            /* e1, e2, e3, e4 */
            float4((other[e41] * wedge[e321]) + (other[e31] * wedge[e412]) + (other[e412] * wedge[e31]) + (other[e321] * wedge[e41]), (other[e42] * wedge[e321]) + (other[e12] * wedge[e423]) + (other[e423] * wedge[e12]) + (other[e321] * wedge[e42]), (other[e43] * wedge[e321]) + (other[e23] * wedge[e431]) + (other[e431] * wedge[e23]) + (other[e321] * wedge[e43]), -(other[e43] * wedge[e412]) - (other[e423] * wedge[e41]) - (other[e431] * wedge[e42]) - (other[e412] * wedge[e43])) + (float4(other[e1234]) * wedge.group1()) + (float4(wedge[e1234]) * other.group1()) - (wedge.group4().yzxx * float4(other.group3().zxy, other[e41])) - float4((wedge.group3().zxy * other.group4().yzx), other[e42] * wedge[e431]), 
            /* e41, e42, e43 */
            (float3(other[e1234]) * wedge.group2()) + (float3(wedge[e1234]) * other.group2()) + (other.group4().zxy * wedge.group4().yzx) - (other.group4().yzx * wedge.group4().zxy), 
            /* e23, e31, e12 */
            (float3(other[e1234]) * wedge.group3()) + (float3(wedge[e1234]) * other.group3()) + (float3(wedge[e321]) * other.group4().xyz) - (float3(other[e321]) * wedge.group4().xyz), 
            /* e423, e431, e412, e321 */
            (float4(other[e1234]) * wedge.group4()) + (float4(wedge[e1234]) * other.group4())
        );
    }
}
extension Horizon: ProjectOrthogonallyOnto<Plane> {
    func project_orthogonally_onto(self, other: Plane) -> Plane {
        return Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4(self[e321] * other[e321]) * other.group0()
        );
    }
}
extension Line {
    func div(_rhs: project_orthogonally_onto) -> project_orthogonally_onto_partial<Line> {
        project_orthogonally_onto_partial(this)
    }
}
extension Line: ProjectOrthogonallyOnto<Flector> {
    func project_orthogonally_onto(self, other: Flector) -> Motor {
Flector right_anti_dual = Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other[e321] * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other[e1], other[e2], other[e3], 0.0)
        );
Plane wedge = Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4((right_anti_dual[e3] * self[e42]) + (right_anti_dual[e4] * self[e23]), (right_anti_dual[e1] * self[e43]) + (right_anti_dual[e4] * self[e31]), (right_anti_dual[e2] * self[e41]) + (right_anti_dual[e4] * self[e12]), -(right_anti_dual[e2] * self[e31]) - (right_anti_dual[e3] * self[e12])) - (right_anti_dual.group0().yzxx * float4(self.group0().zxy, self[e23]))
        );
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(((other.group1().zxy * wedge.group0().yzx) - (other.group1().yzx * wedge.group0().zxy)), 0.0), 
            /* e23, e31, e12, scalar */
            float4(other[e321] * wedge[e423] * -1.0, other[e321] * wedge[e431] * -1.0, other[e321] * wedge[e412] * -1.0, (other[e2] * wedge[e431]) + (other[e3] * wedge[e412]) + (other[e4] * wedge[e321])) + (wedge.group0().wwwx * float4(other.group1().xyz, other[e1]))
        );
    }
}
extension Line: ProjectOrthogonallyOnto<Horizon> {
    func project_orthogonally_onto(self, other: Horizon) -> Line {
Origin right_anti_dual = Origin::from_groups(
            /* e4 */
            other[e321] * -1.0
        );
        return Line::from_groups(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(other[e321]) * float4(self[e23] * right_anti_dual[e4], self[e31] * right_anti_dual[e4], self[e12] * right_anti_dual[e4], 0.0).xyz * float3(-1.0)
        );
    }
}
extension Line: ProjectOrthogonallyOnto<Line> {
    func project_orthogonally_onto(self, other: Line) -> Line {
Line right_anti_dual = Line::from_groups(
            /* e41, e42, e43 */
            other.group1() * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
AntiScalar wedge = AntiScalar::from_groups(
            /* e1234 */
            -(right_anti_dual[e41] * self[e23]) - (right_anti_dual[e42] * self[e31]) - (right_anti_dual[e43] * self[e12]) - (right_anti_dual[e23] * self[e41]) - (right_anti_dual[e31] * self[e42]) - (right_anti_dual[e12] * self[e43])
        );
        return Line::from_groups(
            /* e41, e42, e43 */
            float3(wedge[e1234]) * other.group0(), 
            /* e23, e31, e12 */
            float3(wedge[e1234]) * other.group1()
        );
    }
}
extension Line: ProjectOrthogonallyOnto<Motor> {
    func project_orthogonally_onto(self, other: Motor) -> Motor {
Motor right_anti_dual = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            other.group1() * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
Motor wedge = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(right_anti_dual[scalar], right_anti_dual[scalar], right_anti_dual[scalar], 1.0) * float4(self.group0(), -(self[e41] * right_anti_dual[e23]) - (self[e42] * right_anti_dual[e31]) - (self[e43] * right_anti_dual[e12]) - (self[e23] * right_anti_dual[e41]) - (self[e31] * right_anti_dual[e42]) - (self[e12] * right_anti_dual[e43])), 
            /* e23, e31, e12, scalar */
            float4(float3(1.0), 0.0) * float4(self.group1(), 0.0) * float4(right_anti_dual.group1().www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(((float3(other[e1234]) * wedge.group0().xyz) + (float3(wedge[e1234]) * other.group0().xyz)), other[e1234] * wedge[e1234]), 
            /* e23, e31, e12, scalar */
            (float4(other[e1234]) * wedge.group1()) + (float4(wedge[e1234]) * other.group1()) + float4(float3(0.0), -(other[e41] * wedge[e23]) - (other[e42] * wedge[e31]) - (other[e43] * wedge[e12]) - (other[e23] * wedge[e41]) - (other[e31] * wedge[e42]) - (other[e12] * wedge[e43]))
        );
    }
}
extension Line: ProjectOrthogonallyOnto<MultiVector> {
    func project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
MultiVector right_anti_dual = MultiVector::from_groups(
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
MultiVector wedge = MultiVector::from_groups(
            /* scalar, e1234 */
            float2(0.0, -(self[e41] * right_anti_dual[e23]) - (self[e42] * right_anti_dual[e31]) - (self[e43] * right_anti_dual[e12]) - (self[e23] * right_anti_dual[e41]) - (self[e31] * right_anti_dual[e42]) - (self[e12] * right_anti_dual[e43])), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(right_anti_dual[scalar]) * self.group0(), 
            /* e23, e31, e12 */
            float3(right_anti_dual[scalar]) * self.group1(), 
            /* e423, e431, e412, e321 */
            float4((self[e42] * right_anti_dual[e3]) + (self[e23] * right_anti_dual[e4]), (self[e43] * right_anti_dual[e1]) + (self[e31] * right_anti_dual[e4]), (self[e41] * right_anti_dual[e2]) + (self[e12] * right_anti_dual[e4]), -(self[e31] * right_anti_dual[e2]) - (self[e12] * right_anti_dual[e3])) - (right_anti_dual.group1().yzxx * float4(self.group0().zxy, self[e23]))
        );
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2((other[scalar] * wedge[e1234]) + (other[e1234] * wedge[scalar]) + (other[e1] * wedge[e423]) + (other[e2] * wedge[e431]) + (other[e3] * wedge[e412]) + (other[e4] * wedge[e321]) - (other[e41] * wedge[e23]) - (other[e42] * wedge[e31]) - (other[e43] * wedge[e12]) - (other[e23] * wedge[e41]) - (other[e31] * wedge[e42]) - (other[e12] * wedge[e43]) - (other[e423] * wedge[e1]) - (other[e431] * wedge[e2]) - (other[e412] * wedge[e3]) - (other[e321] * wedge[e4]), other[e1234] * wedge[e1234]), 
            /* e1, e2, e3, e4 */
            float4((other[e41] * wedge[e321]) + (other[e31] * wedge[e412]) + (other[e412] * wedge[e31]) + (other[e321] * wedge[e41]), (other[e42] * wedge[e321]) + (other[e12] * wedge[e423]) + (other[e423] * wedge[e12]) + (other[e321] * wedge[e42]), (other[e43] * wedge[e321]) + (other[e23] * wedge[e431]) + (other[e431] * wedge[e23]) + (other[e321] * wedge[e43]), -(other[e43] * wedge[e412]) - (other[e423] * wedge[e41]) - (other[e431] * wedge[e42]) - (other[e412] * wedge[e43])) + (float4(other[e1234]) * wedge.group1()) + (float4(wedge[e1234]) * other.group1()) - (wedge.group4().yzxx * float4(other.group3().zxy, other[e41])) - float4((wedge.group3().zxy * other.group4().yzx), other[e42] * wedge[e431]), 
            /* e41, e42, e43 */
            (float3(other[e1234]) * wedge.group2()) + (float3(wedge[e1234]) * other.group2()) + (other.group4().zxy * wedge.group4().yzx) - (other.group4().yzx * wedge.group4().zxy), 
            /* e23, e31, e12 */
            (float3(other[e1234]) * wedge.group3()) + (float3(wedge[e1234]) * other.group3()) + (float3(wedge[e321]) * other.group4().xyz) - (float3(other[e321]) * wedge.group4().xyz), 
            /* e423, e431, e412, e321 */
            (float4(other[e1234]) * wedge.group4()) + (float4(wedge[e1234]) * other.group4())
        );
    }
}
extension Line: ProjectOrthogonallyOnto<Plane> {
    func project_orthogonally_onto(self, other: Plane) -> Line {
Origin right_anti_dual = Origin::from_groups(
            /* e4 */
            other[e321] * -1.0
        );
Plane wedge = Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4(right_anti_dual[e4], right_anti_dual[e4], right_anti_dual[e4], 0.0) * float4(self.group1(), 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
        return Line::from_groups(
            /* e41, e42, e43 */
            (other.group0().zxy * wedge.group0().yzx) - (other.group0().yzx * wedge.group0().zxy), 
            /* e23, e31, e12 */
            (float3(wedge[e321]) * other.group0().xyz) - (float3(other[e321]) * wedge.group0().xyz)
        );
    }
}
extension Motor {
    func div(_rhs: project_orthogonally_onto) -> project_orthogonally_onto_partial<Motor> {
        project_orthogonally_onto_partial(this)
    }
}
extension Motor: ProjectOrthogonallyOnto<DualNum> {
    func project_orthogonally_onto(self, other: DualNum) -> DualNum {
        return DualNum::from_groups(
            /* scalar, e1234 */
            float2(other[scalar] * self[scalar]) * other.group0()
        );
    }
}
extension Motor: ProjectOrthogonallyOnto<Flector> {
    func project_orthogonally_onto(self, other: Flector) -> Motor {
Flector right_anti_dual = Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other[e321] * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other[e1], other[e2], other[e3], 0.0)
        );
Flector wedge = Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(self[scalar]) * right_anti_dual.group0(), 
            /* e423, e431, e412, e321 */
            float4((right_anti_dual[e4] * self[e23]) + (right_anti_dual[e423] * self[scalar]), (right_anti_dual[e4] * self[e31]) + (right_anti_dual[e431] * self[scalar]), (right_anti_dual[e4] * self[e12]) + (right_anti_dual[e412] * self[scalar]), -(right_anti_dual[e2] * self[e31]) - (right_anti_dual[e3] * self[e12])) + float4((right_anti_dual.group0().zxy * self.group0().yzx), right_anti_dual[e321] * self[scalar]) - (right_anti_dual.group0().yzxx * float4(self.group0().zxy, self[e23]))
        );
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(((other.group1().zxy * wedge.group1().yzx) - (other.group1().yzx * wedge.group1().zxy)), 0.0), 
            /* e23, e31, e12, scalar */
            (wedge.group1().wwwx * float4(other.group1().xyz, other[e1])) + float4(float3(0.0), (other[e2] * wedge[e431]) + (other[e3] * wedge[e412]) + (other[e4] * wedge[e321]) - (other[e431] * wedge[e2]) - (other[e412] * wedge[e3]) - (other[e321] * wedge[e4])) - (other.group1().wwwx * float4(wedge.group1().xyz, wedge[e1]))
        );
    }
}
extension Motor: ProjectOrthogonallyOnto<Horizon> {
    func project_orthogonally_onto(self, other: Horizon) -> Motor {
Origin right_anti_dual = Origin::from_groups(
            /* e4 */
            other[e321] * -1.0
        );
Flector wedge = Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), self[scalar] * right_anti_dual[e4]), 
            /* e423, e431, e412, e321 */
            float4(right_anti_dual[e4], right_anti_dual[e4], right_anti_dual[e4], 0.0) * float4(self.group1().xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(0.0), 
            /* e23, e31, e12, scalar */
            float4(other[e321]) * float4(wedge.group1().xyz, wedge[e4]) * float4(-1.0)
        );
    }
}
extension Motor: ProjectOrthogonallyOnto<Line> {
    func project_orthogonally_onto(self, other: Line) -> Motor {
Line right_anti_dual = Line::from_groups(
            /* e41, e42, e43 */
            other.group1() * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
Motor wedge = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(self[scalar], self[scalar], self[scalar], 1.0) * float4(right_anti_dual.group0(), -(right_anti_dual[e41] * self[e23]) - (right_anti_dual[e42] * self[e31]) - (right_anti_dual[e43] * self[e12]) - (right_anti_dual[e23] * self[e41]) - (right_anti_dual[e31] * self[e42]) - (right_anti_dual[e12] * self[e43])), 
            /* e23, e31, e12, scalar */
            float4(float3(1.0), 0.0) * float4(right_anti_dual.group1(), 0.0) * float4(self.group1().www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(float3(1.0), 0.0) * float4(other.group0(), 0.0) * float4(wedge.group0().www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e23, e31, e12, scalar */
            float4(wedge[e1234], wedge[e1234], wedge[e1234], 1.0) * float4(other.group1(), -(other[e41] * wedge[e23]) - (other[e42] * wedge[e31]) - (other[e43] * wedge[e12]) - (other[e23] * wedge[e41]) - (other[e31] * wedge[e42]) - (other[e12] * wedge[e43]))
        );
    }
}
extension Motor: ProjectOrthogonallyOnto<Motor> {
    func project_orthogonally_onto(self, other: Motor) -> Motor {
Motor right_anti_dual = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            other.group1() * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
Motor wedge = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            (float4(right_anti_dual[scalar]) * self.group0()) + (float4(self[scalar]) * right_anti_dual.group0()) + float4(float3(0.0), -(right_anti_dual[e41] * self[e23]) - (right_anti_dual[e42] * self[e31]) - (right_anti_dual[e43] * self[e12]) - (right_anti_dual[e23] * self[e41]) - (right_anti_dual[e31] * self[e42]) - (right_anti_dual[e12] * self[e43])), 
            /* e23, e31, e12, scalar */
            float4(float3(0.0), right_anti_dual[scalar] * self[scalar])
        );
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(((float3(other[e1234]) * wedge.group0().xyz) + (float3(wedge[e1234]) * other.group0().xyz)), other[e1234] * wedge[e1234]), 
            /* e23, e31, e12, scalar */
            (float4(other[e1234]) * wedge.group1()) + (float4(wedge[e1234]) * other.group1()) + float4(float3(0.0), -(other[e41] * wedge[e23]) - (other[e42] * wedge[e31]) - (other[e43] * wedge[e12]) - (other[e23] * wedge[e41]) - (other[e31] * wedge[e42]) - (other[e12] * wedge[e43]))
        );
    }
}
extension Motor: ProjectOrthogonallyOnto<MultiVector> {
    func project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
MultiVector right_anti_dual = MultiVector::from_groups(
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
MultiVector wedge = MultiVector::from_groups(
            /* scalar, e1234 */
            float2(self[scalar] * right_anti_dual[scalar], (self[e1234] * right_anti_dual[scalar]) + (self[scalar] * right_anti_dual[e1234]) - (self[e41] * right_anti_dual[e23]) - (self[e42] * right_anti_dual[e31]) - (self[e43] * right_anti_dual[e12]) - (self[e23] * right_anti_dual[e41]) - (self[e31] * right_anti_dual[e42]) - (self[e12] * right_anti_dual[e43])), 
            /* e1, e2, e3, e4 */
            float4(self[scalar]) * right_anti_dual.group1(), 
            /* e41, e42, e43 */
            (float3(self[scalar]) * right_anti_dual.group2()) + (float3(right_anti_dual[scalar]) * self.group0().xyz), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4((self[e23] * right_anti_dual[e4]) + (self[scalar] * right_anti_dual[e423]), (self[e31] * right_anti_dual[e4]) + (self[scalar] * right_anti_dual[e431]), (self[e12] * right_anti_dual[e4]) + (self[scalar] * right_anti_dual[e412]), -(self[e31] * right_anti_dual[e2]) - (self[e12] * right_anti_dual[e3])) + float4((self.group0().yzx * right_anti_dual.group1().zxy), self[scalar] * right_anti_dual[e321]) - (right_anti_dual.group1().yzxx * float4(self.group0().zxy, self[e23]))
        );
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2((other[scalar] * wedge[e1234]) + (other[e1234] * wedge[scalar]) + (other[e1] * wedge[e423]) + (other[e2] * wedge[e431]) + (other[e3] * wedge[e412]) + (other[e4] * wedge[e321]) - (other[e41] * wedge[e23]) - (other[e42] * wedge[e31]) - (other[e43] * wedge[e12]) - (other[e23] * wedge[e41]) - (other[e31] * wedge[e42]) - (other[e12] * wedge[e43]) - (other[e423] * wedge[e1]) - (other[e431] * wedge[e2]) - (other[e412] * wedge[e3]) - (other[e321] * wedge[e4]), other[e1234] * wedge[e1234]), 
            /* e1, e2, e3, e4 */
            float4((other[e41] * wedge[e321]) + (other[e31] * wedge[e412]) + (other[e412] * wedge[e31]) + (other[e321] * wedge[e41]), (other[e42] * wedge[e321]) + (other[e12] * wedge[e423]) + (other[e423] * wedge[e12]) + (other[e321] * wedge[e42]), (other[e43] * wedge[e321]) + (other[e23] * wedge[e431]) + (other[e431] * wedge[e23]) + (other[e321] * wedge[e43]), -(other[e43] * wedge[e412]) - (other[e423] * wedge[e41]) - (other[e431] * wedge[e42]) - (other[e412] * wedge[e43])) + (float4(other[e1234]) * wedge.group1()) + (float4(wedge[e1234]) * other.group1()) - (wedge.group4().yzxx * float4(other.group3().zxy, other[e41])) - float4((wedge.group3().zxy * other.group4().yzx), other[e42] * wedge[e431]), 
            /* e41, e42, e43 */
            (float3(other[e1234]) * wedge.group2()) + (float3(wedge[e1234]) * other.group2()) + (other.group4().zxy * wedge.group4().yzx) - (other.group4().yzx * wedge.group4().zxy), 
            /* e23, e31, e12 */
            (float3(other[e1234]) * wedge.group3()) + (float3(wedge[e1234]) * other.group3()) + (float3(wedge[e321]) * other.group4().xyz) - (float3(other[e321]) * wedge.group4().xyz), 
            /* e423, e431, e412, e321 */
            (float4(other[e1234]) * wedge.group4()) + (float4(wedge[e1234]) * other.group4())
        );
    }
}
extension Motor: ProjectOrthogonallyOnto<Plane> {
    func project_orthogonally_onto(self, other: Plane) -> Motor {
Origin right_anti_dual = Origin::from_groups(
            /* e4 */
            other[e321] * -1.0
        );
Flector wedge = Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), self[scalar] * right_anti_dual[e4]), 
            /* e423, e431, e412, e321 */
            float4(right_anti_dual[e4], right_anti_dual[e4], right_anti_dual[e4], 0.0) * float4(self.group1().xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(((wedge.group1().yzx * other.group0().zxy) - (wedge.group1().zxy * other.group0().yzx)), 0.0), 
            /* e23, e31, e12, scalar */
            float4(wedge[e321] * other[e423], wedge[e321] * other[e431], wedge[e321] * other[e412], -(wedge[e2] * other[e431]) - (wedge[e3] * other[e412]) - (wedge[e4] * other[e321])) - (other.group0().wwwx * float4(wedge.group1().xyz, wedge[e1]))
        );
    }
}
extension Motor: ProjectOrthogonallyOnto<Point> {
    func project_orthogonally_onto(self, other: Point) -> Scalar {
Plane wedge = Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4(self[scalar]) * float4(other[e1], other[e2], other[e3], 0.0)
        );
        return Scalar::from_groups(
            /* scalar */
            (wedge[e423] * other[e1]) + (wedge[e431] * other[e2]) + (wedge[e412] * other[e3]) + (wedge[e321] * other[e4])
        );
    }
}
extension Motor: ProjectOrthogonallyOnto<Scalar> {
    func project_orthogonally_onto(self, other: Scalar) -> Scalar {
        return Scalar::from_groups(
            /* scalar */
            powi(other[scalar], 2) * self[scalar]
        );
    }
}
extension MultiVector {
    func div(_rhs: project_orthogonally_onto) -> project_orthogonally_onto_partial<MultiVector> {
        project_orthogonally_onto_partial(this)
    }
}
extension MultiVector: ProjectOrthogonallyOnto<DualNum> {
    func project_orthogonally_onto(self, other: DualNum) -> DualNum {
        return DualNum::from_groups(
            /* scalar, e1234 */
            float2(other[scalar] * self[scalar]) * other.group0()
        );
    }
}
extension MultiVector: ProjectOrthogonallyOnto<Flector> {
    func project_orthogonally_onto(self, other: Flector) -> MultiVector {
Flector right_anti_dual = Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other[e321] * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other[e1], other[e2], other[e3], 0.0)
        );
MultiVector wedge = MultiVector::from_groups(
            /* scalar, e1234 */
            float2(0.0, (right_anti_dual[e423] * self[e1]) + (right_anti_dual[e431] * self[e2]) + (right_anti_dual[e412] * self[e3]) + (right_anti_dual[e321] * self[e4]) - (right_anti_dual[e1] * self[e423]) - (right_anti_dual[e2] * self[e431]) - (right_anti_dual[e3] * self[e412]) - (right_anti_dual[e4] * self[e321])), 
            /* e1, e2, e3, e4 */
            float4(self[scalar]) * right_anti_dual.group0(), 
            /* e41, e42, e43 */
            (float3(self[e4]) * right_anti_dual.group0().xyz) - (float3(right_anti_dual[e4]) * self.group1().xyz), 
            /* e23, e31, e12 */
            (right_anti_dual.group0().zxy * self.group1().yzx) - (right_anti_dual.group0().yzx * self.group1().zxy), 
            /* e423, e431, e412, e321 */
            float4((right_anti_dual[e3] * self[e42]) + (right_anti_dual[e4] * self[e23]), (right_anti_dual[e1] * self[e43]) + (right_anti_dual[e4] * self[e31]), (right_anti_dual[e2] * self[e41]) + (right_anti_dual[e4] * self[e12]), -(right_anti_dual[e2] * self[e31]) - (right_anti_dual[e3] * self[e12])) + (float4(self[scalar]) * right_anti_dual.group1()) - (right_anti_dual.group0().yzxx * float4(self.group2().zxy, self[e23]))
        );
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2((other[e1] * wedge[e423]) + (other[e2] * wedge[e431]) + (other[e3] * wedge[e412]) + (other[e4] * wedge[e321]) - (other[e423] * wedge[e1]) - (other[e431] * wedge[e2]) - (other[e412] * wedge[e3]) - (other[e321] * wedge[e4]), 0.0), 
            /* e1, e2, e3, e4 */
            float4((other[e412] * wedge[e31]) + (other[e321] * wedge[e41]), (other[e423] * wedge[e12]) + (other[e321] * wedge[e42]), (other[e431] * wedge[e23]) + (other[e321] * wedge[e43]), -(other[e431] * wedge[e42]) - (other[e412] * wedge[e43])) + (float4(wedge[e1234]) * other.group0()) - (other.group1().yzxx * float4(wedge.group3().zxy, wedge[e41])), 
            /* e41, e42, e43 */
            (other.group1().zxy * wedge.group4().yzx) - (other.group1().yzx * wedge.group4().zxy), 
            /* e23, e31, e12 */
            (float3(wedge[e321]) * other.group1().xyz) - (float3(other[e321]) * wedge.group4().xyz), 
            /* e423, e431, e412, e321 */
            float4(wedge[e1234]) * other.group1()
        );
    }
}
extension MultiVector: ProjectOrthogonallyOnto<Horizon> {
    func project_orthogonally_onto(self, other: Horizon) -> MultiVector {
Origin right_anti_dual = Origin::from_groups(
            /* e4 */
            other[e321] * -1.0
        );
MultiVector wedge = MultiVector::from_groups(
            /* scalar, e1234 */
            float2(1.0, self[e321] * right_anti_dual[e4]) * float2(0.0, -1.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), self[scalar] * right_anti_dual[e4]), 
            /* e41, e42, e43 */
            float3(right_anti_dual[e4]) * self.group1().xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(right_anti_dual[e4], right_anti_dual[e4], right_anti_dual[e4], 0.0) * float4(self.group3(), 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(other[e321] * wedge[e4], 1.0) * float2(-1.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(other[e321], other[e321], other[e321], 0.0) * float4(wedge.group2(), 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(other[e321]) * wedge.group4().xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other[e321] * wedge[e1234])
        );
    }
}
extension MultiVector: ProjectOrthogonallyOnto<Line> {
    func project_orthogonally_onto(self, other: Line) -> MultiVector {
Line right_anti_dual = Line::from_groups(
            /* e41, e42, e43 */
            other.group1() * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
MultiVector wedge = MultiVector::from_groups(
            /* scalar, e1234 */
            float2(0.0, -(right_anti_dual[e41] * self[e23]) - (right_anti_dual[e42] * self[e31]) - (right_anti_dual[e43] * self[e12]) - (right_anti_dual[e23] * self[e41]) - (right_anti_dual[e31] * self[e42]) - (right_anti_dual[e12] * self[e43])), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(self[scalar]) * right_anti_dual.group0(), 
            /* e23, e31, e12 */
            float3(self[scalar]) * right_anti_dual.group1(), 
            /* e423, e431, e412, e321 */
            float4((float3(self[e4]) * right_anti_dual.group1()), 0.0) + float4((right_anti_dual.group0().yzx * self.group1().zxy), 0.0) - float4((right_anti_dual.group0().zxy * self.group1().yzx), 0.0)
        );
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(-(other[e41] * wedge[e23]) - (other[e42] * wedge[e31]) - (other[e43] * wedge[e12]) - (other[e23] * wedge[e41]) - (other[e31] * wedge[e42]) - (other[e12] * wedge[e43]), 0.0), 
            /* e1, e2, e3, e4 */
            float4((other[e41] * wedge[e321]) + (other[e31] * wedge[e412]), (other[e42] * wedge[e321]) + (other[e12] * wedge[e423]), (other[e43] * wedge[e321]) + (other[e23] * wedge[e431]), -(other[e42] * wedge[e431]) - (other[e43] * wedge[e412])) - (wedge.group4().yzxx * float4(other.group1().zxy, other[e41])), 
            /* e41, e42, e43 */
            float3(wedge[e1234]) * other.group0(), 
            /* e23, e31, e12 */
            float3(wedge[e1234]) * other.group1(), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
extension MultiVector: ProjectOrthogonallyOnto<Motor> {
    func project_orthogonally_onto(self, other: Motor) -> MultiVector {
Motor right_anti_dual = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            other.group1() * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
MultiVector wedge = MultiVector::from_groups(
            /* scalar, e1234 */
            float2(right_anti_dual[scalar] * self[scalar], (right_anti_dual[e1234] * self[scalar]) + (right_anti_dual[scalar] * self[e1234]) - (right_anti_dual[e41] * self[e23]) - (right_anti_dual[e42] * self[e31]) - (right_anti_dual[e43] * self[e12]) - (right_anti_dual[e23] * self[e41]) - (right_anti_dual[e31] * self[e42]) - (right_anti_dual[e12] * self[e43])), 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual[scalar]) * self.group1(), 
            /* e41, e42, e43 */
            (float3(right_anti_dual[scalar]) * self.group2()) + (float3(self[scalar]) * right_anti_dual.group0().xyz), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4((float3(right_anti_dual[scalar]) * self.group4().xyz), 0.0) + float4((float3(self[e4]) * right_anti_dual.group1().xyz), 0.0) + float4((right_anti_dual.group0().yzx * self.group1().zxy), 0.0) - float4((right_anti_dual.group0().zxy * self.group1().yzx), 0.0)
        );
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2((other[e1234] * wedge[scalar]) + (other[scalar] * wedge[e1234]) - (other[e41] * wedge[e23]) - (other[e42] * wedge[e31]) - (other[e43] * wedge[e12]) - (other[e23] * wedge[e41]) - (other[e31] * wedge[e42]) - (other[e12] * wedge[e43]), other[e1234] * wedge[e1234]), 
            /* e1, e2, e3, e4 */
            float4((other[e1234] * wedge[e1]) + (other[e31] * wedge[e412]), (other[e1234] * wedge[e2]) + (other[e12] * wedge[e423]), (other[e1234] * wedge[e3]) + (other[e23] * wedge[e431]), -(other[e42] * wedge[e431]) - (other[e43] * wedge[e412])) + (other.group0() * float4(wedge.group4().www, wedge[e4])) - (wedge.group4().yzxx * float4(other.group1().zxy, other[e41])), 
            /* e41, e42, e43 */
            (float3(other[e1234]) * wedge.group2()) + (float3(wedge[e1234]) * other.group0().xyz), 
            /* e23, e31, e12 */
            (float3(other[e1234]) * wedge.group3()) + (float3(wedge[e1234]) * other.group1().xyz), 
            /* e423, e431, e412, e321 */
            float4(other[e1234]) * wedge.group4()
        );
    }
}
extension MultiVector: ProjectOrthogonallyOnto<MultiVector> {
    func project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
MultiVector right_anti_dual = MultiVector::from_groups(
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
MultiVector wedge = MultiVector::from_groups(
            /* scalar, e1234 */
            float2(right_anti_dual[scalar] * self[scalar], (right_anti_dual[scalar] * self[e1234]) + (right_anti_dual[e1234] * self[scalar]) + (right_anti_dual[e423] * self[e1]) + (right_anti_dual[e431] * self[e2]) + (right_anti_dual[e412] * self[e3]) + (right_anti_dual[e321] * self[e4]) - (right_anti_dual[e1] * self[e423]) - (right_anti_dual[e2] * self[e431]) - (right_anti_dual[e3] * self[e412]) - (right_anti_dual[e4] * self[e321]) - (right_anti_dual[e41] * self[e23]) - (right_anti_dual[e42] * self[e31]) - (right_anti_dual[e43] * self[e12]) - (right_anti_dual[e23] * self[e41]) - (right_anti_dual[e31] * self[e42]) - (right_anti_dual[e12] * self[e43])), 
            /* e1, e2, e3, e4 */
            (float4(right_anti_dual[scalar]) * self.group1()) + (float4(self[scalar]) * right_anti_dual.group1()), 
            /* e41, e42, e43 */
            (float3(right_anti_dual[scalar]) * self.group2()) + (float3(self[scalar]) * right_anti_dual.group2()) + (float3(self[e4]) * right_anti_dual.group1().xyz) - (float3(right_anti_dual[e4]) * self.group1().xyz), 
            /* e23, e31, e12 */
            (float3(right_anti_dual[scalar]) * self.group3()) + (float3(self[scalar]) * right_anti_dual.group3()) + (right_anti_dual.group1().zxy * self.group1().yzx) - (right_anti_dual.group1().yzx * self.group1().zxy), 
            /* e423, e431, e412, e321 */
            float4((right_anti_dual[e3] * self[e42]) + (right_anti_dual[e4] * self[e23]) + (right_anti_dual[e42] * self[e3]) + (right_anti_dual[e23] * self[e4]), (right_anti_dual[e1] * self[e43]) + (right_anti_dual[e4] * self[e31]) + (right_anti_dual[e43] * self[e1]) + (right_anti_dual[e31] * self[e4]), (right_anti_dual[e2] * self[e41]) + (right_anti_dual[e4] * self[e12]) + (right_anti_dual[e41] * self[e2]) + (right_anti_dual[e12] * self[e4]), -(right_anti_dual[e1] * self[e23]) - (right_anti_dual[e2] * self[e31]) - (right_anti_dual[e3] * self[e12]) - (right_anti_dual[e12] * self[e3])) + (float4(right_anti_dual[scalar]) * self.group4()) + (float4(self[scalar]) * right_anti_dual.group4()) - (self.group1().yzxx * float4(right_anti_dual.group2().zxy, right_anti_dual[e23])) - float4((self.group2().zxy * right_anti_dual.group1().yzx), right_anti_dual[e31] * self[e2])
        );
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2((other[scalar] * wedge[e1234]) + (other[e1234] * wedge[scalar]) + (other[e1] * wedge[e423]) + (other[e2] * wedge[e431]) + (other[e3] * wedge[e412]) + (other[e4] * wedge[e321]) - (other[e41] * wedge[e23]) - (other[e42] * wedge[e31]) - (other[e43] * wedge[e12]) - (other[e23] * wedge[e41]) - (other[e31] * wedge[e42]) - (other[e12] * wedge[e43]) - (other[e423] * wedge[e1]) - (other[e431] * wedge[e2]) - (other[e412] * wedge[e3]) - (other[e321] * wedge[e4]), other[e1234] * wedge[e1234]), 
            /* e1, e2, e3, e4 */
            float4((other[e41] * wedge[e321]) + (other[e31] * wedge[e412]) + (other[e412] * wedge[e31]) + (other[e321] * wedge[e41]), (other[e42] * wedge[e321]) + (other[e12] * wedge[e423]) + (other[e423] * wedge[e12]) + (other[e321] * wedge[e42]), (other[e43] * wedge[e321]) + (other[e23] * wedge[e431]) + (other[e431] * wedge[e23]) + (other[e321] * wedge[e43]), -(other[e43] * wedge[e412]) - (other[e423] * wedge[e41]) - (other[e431] * wedge[e42]) - (other[e412] * wedge[e43])) + (float4(other[e1234]) * wedge.group1()) + (float4(wedge[e1234]) * other.group1()) - (wedge.group4().yzxx * float4(other.group3().zxy, other[e41])) - float4((wedge.group3().zxy * other.group4().yzx), other[e42] * wedge[e431]), 
            /* e41, e42, e43 */
            (float3(other[e1234]) * wedge.group2()) + (float3(wedge[e1234]) * other.group2()) + (other.group4().zxy * wedge.group4().yzx) - (other.group4().yzx * wedge.group4().zxy), 
            /* e23, e31, e12 */
            (float3(other[e1234]) * wedge.group3()) + (float3(wedge[e1234]) * other.group3()) + (float3(wedge[e321]) * other.group4().xyz) - (float3(other[e321]) * wedge.group4().xyz), 
            /* e423, e431, e412, e321 */
            (float4(other[e1234]) * wedge.group4()) + (float4(wedge[e1234]) * other.group4())
        );
    }
}
extension MultiVector: ProjectOrthogonallyOnto<Plane> {
    func project_orthogonally_onto(self, other: Plane) -> MultiVector {
Origin right_anti_dual = Origin::from_groups(
            /* e4 */
            other[e321] * -1.0
        );
MultiVector wedge = MultiVector::from_groups(
            /* scalar, e1234 */
            float2(1.0, self[e321] * right_anti_dual[e4]) * float2(0.0, -1.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), self[scalar] * right_anti_dual[e4]), 
            /* e41, e42, e43 */
            float3(right_anti_dual[e4]) * self.group1().xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(right_anti_dual[e4], right_anti_dual[e4], right_anti_dual[e4], 0.0) * float4(self.group3(), 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(-(wedge[e1] * other[e423]) - (wedge[e2] * other[e431]) - (wedge[e3] * other[e412]) - (wedge[e4] * other[e321]), 0.0), 
            /* e1, e2, e3, e4 */
            float4((wedge[e41] * other[e321]) + (wedge[e31] * other[e412]), (wedge[e42] * other[e321]) + (wedge[e12] * other[e423]), (wedge[e43] * other[e321]) + (wedge[e23] * other[e431]), -(wedge[e42] * other[e431]) - (wedge[e43] * other[e412])) - (other.group0().yzxx * float4(wedge.group3().zxy, wedge[e41])), 
            /* e41, e42, e43 */
            (wedge.group4().yzx * other.group0().zxy) - (wedge.group4().zxy * other.group0().yzx), 
            /* e23, e31, e12 */
            (float3(wedge[e321]) * other.group0().xyz) - (float3(other[e321]) * wedge.group4().xyz), 
            /* e423, e431, e412, e321 */
            float4(wedge[e1234]) * other.group0()
        );
    }
}
extension MultiVector: ProjectOrthogonallyOnto<Point> {
    func project_orthogonally_onto(self, other: Point) -> MultiVector {
Plane right_anti_dual = Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4(other[e1], other[e2], other[e3], 0.0)
        );
MultiVector wedge = MultiVector::from_groups(
            /* scalar, e1234 */
            float2(0.0, (self[e1] * right_anti_dual[e423]) + (self[e2] * right_anti_dual[e431]) + (self[e3] * right_anti_dual[e412]) + (self[e4] * right_anti_dual[e321])), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(self[scalar]) * right_anti_dual.group0()
        );
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2((wedge[e423] * other[e1]) + (wedge[e431] * other[e2]) + (wedge[e412] * other[e3]) + (wedge[e321] * other[e4]), 0.0), 
            /* e1, e2, e3, e4 */
            float4(wedge[e1234]) * other.group0(), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
extension MultiVector: ProjectOrthogonallyOnto<Scalar> {
    func project_orthogonally_onto(self, other: Scalar) -> Scalar {
        return Scalar::from_groups(
            /* scalar */
            powi(other[scalar], 2) * self[scalar]
        );
    }
}
extension Origin {
    func div(_rhs: project_orthogonally_onto) -> project_orthogonally_onto_partial<Origin> {
        project_orthogonally_onto_partial(this)
    }
}
extension Origin: ProjectOrthogonallyOnto<Flector> {
    func project_orthogonally_onto(self, other: Flector) -> Flector {
Flector right_anti_dual = Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other[e321] * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other[e1], other[e2], other[e3], 0.0)
        );
Motor wedge = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(self[e4]) * float4(right_anti_dual.group0().xyz, right_anti_dual[e321]), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4((other[e412] * wedge[e31]) + (other[e321] * wedge[e41]), (other[e423] * wedge[e12]) + (other[e321] * wedge[e42]), (other[e431] * wedge[e23]) + (other[e321] * wedge[e43]), -(other[e431] * wedge[e42]) - (other[e412] * wedge[e43])) + (float4(wedge[e1234]) * other.group0()) - (other.group1().yzxx * float4(wedge.group1().zxy, wedge[e41])), 
            /* e423, e431, e412, e321 */
            float4(wedge[e1234]) * other.group1()
        );
    }
}
extension Origin: ProjectOrthogonallyOnto<Motor> {
    func project_orthogonally_onto(self, other: Motor) -> Flector {
Motor right_anti_dual = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            other.group1() * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
Flector wedge = Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), right_anti_dual[scalar] * self[e4]), 
            /* e423, e431, e412, e321 */
            float4(self[e4], self[e4], self[e4], 0.0) * float4(right_anti_dual.group1().xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4((wedge[e412] * other[e31]) + (wedge[e321] * other[e41]), (wedge[e423] * other[e12]) + (wedge[e321] * other[e42]), (wedge[e431] * other[e23]) + (wedge[e321] * other[e43]), -(wedge[e431] * other[e42]) - (wedge[e412] * other[e43])) + (float4(other[e1234]) * wedge.group0()) - (wedge.group1().yzxx * float4(other.group1().zxy, other[e41])), 
            /* e423, e431, e412, e321 */
            float4(other[e1234]) * wedge.group1()
        );
    }
}
extension Origin: ProjectOrthogonallyOnto<MultiVector> {
    func project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
MultiVector right_anti_dual = MultiVector::from_groups(
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
MultiVector wedge = MultiVector::from_groups(
            /* scalar, e1234 */
            float2(1.0, right_anti_dual[e321] * self[e4]) * float2(0.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), right_anti_dual[scalar] * self[e4]), 
            /* e41, e42, e43 */
            float3(self[e4]) * right_anti_dual.group1().xyz, 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(self[e4], self[e4], self[e4], 0.0) * float4(right_anti_dual.group3(), 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2((other[scalar] * wedge[e1234]) + (other[e1234] * wedge[scalar]) + (other[e1] * wedge[e423]) + (other[e2] * wedge[e431]) + (other[e3] * wedge[e412]) + (other[e4] * wedge[e321]) - (other[e41] * wedge[e23]) - (other[e42] * wedge[e31]) - (other[e43] * wedge[e12]) - (other[e23] * wedge[e41]) - (other[e31] * wedge[e42]) - (other[e12] * wedge[e43]) - (other[e423] * wedge[e1]) - (other[e431] * wedge[e2]) - (other[e412] * wedge[e3]) - (other[e321] * wedge[e4]), other[e1234] * wedge[e1234]), 
            /* e1, e2, e3, e4 */
            float4((other[e41] * wedge[e321]) + (other[e31] * wedge[e412]) + (other[e412] * wedge[e31]) + (other[e321] * wedge[e41]), (other[e42] * wedge[e321]) + (other[e12] * wedge[e423]) + (other[e423] * wedge[e12]) + (other[e321] * wedge[e42]), (other[e43] * wedge[e321]) + (other[e23] * wedge[e431]) + (other[e431] * wedge[e23]) + (other[e321] * wedge[e43]), -(other[e43] * wedge[e412]) - (other[e423] * wedge[e41]) - (other[e431] * wedge[e42]) - (other[e412] * wedge[e43])) + (float4(other[e1234]) * wedge.group1()) + (float4(wedge[e1234]) * other.group1()) - (wedge.group4().yzxx * float4(other.group3().zxy, other[e41])) - float4((wedge.group3().zxy * other.group4().yzx), other[e42] * wedge[e431]), 
            /* e41, e42, e43 */
            (float3(other[e1234]) * wedge.group2()) + (float3(wedge[e1234]) * other.group2()) + (other.group4().zxy * wedge.group4().yzx) - (other.group4().yzx * wedge.group4().zxy), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
extension Origin: ProjectOrthogonallyOnto<Point> {
    func project_orthogonally_onto(self, other: Point) -> Point {
        return Point::from_groups(
            /* e1, e2, e3, e4 */
            float4(self[e4] * 0.0) * other.group0()
        );
    }
}
extension Plane {
    func div(_rhs: project_orthogonally_onto) -> project_orthogonally_onto_partial<Plane> {
        project_orthogonally_onto_partial(this)
    }
}
extension Plane: ProjectOrthogonallyOnto<Flector> {
    func project_orthogonally_onto(self, other: Flector) -> Flector {
Flector right_anti_dual = Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other[e321] * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other[e1], other[e2], other[e3], 0.0)
        );
AntiScalar wedge = AntiScalar::from_groups(
            /* e1234 */
            -(right_anti_dual[e1] * self[e423]) - (right_anti_dual[e2] * self[e431]) - (right_anti_dual[e3] * self[e412]) - (right_anti_dual[e4] * self[e321])
        );
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(wedge[e1234]) * other.group0(), 
            /* e423, e431, e412, e321 */
            float4(wedge[e1234]) * other.group1()
        );
    }
}
extension Plane: ProjectOrthogonallyOnto<Horizon> {
    func project_orthogonally_onto(self, other: Horizon) -> Horizon {
        return Horizon::from_groups(
            /* e321 */
            powi(other[e321], 2) * self[e321]
        );
    }
}
extension Plane: ProjectOrthogonallyOnto<Motor> {
    func project_orthogonally_onto(self, other: Motor) -> Plane {
Motor right_anti_dual = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            other.group1() * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
        return Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4(other[e1234]) * float4(right_anti_dual[scalar] * self[e423], right_anti_dual[scalar] * self[e431], right_anti_dual[scalar] * self[e412], right_anti_dual[scalar] * self[e321])
        );
    }
}
extension Plane: ProjectOrthogonallyOnto<MultiVector> {
    func project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
MultiVector right_anti_dual = MultiVector::from_groups(
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
MultiVector wedge = MultiVector::from_groups(
            /* scalar, e1234 */
            float2(0.0, -(right_anti_dual[e1] * self[e423]) - (right_anti_dual[e2] * self[e431]) - (right_anti_dual[e3] * self[e412]) - (right_anti_dual[e4] * self[e321])), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(right_anti_dual[scalar]) * self.group0()
        );
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2((other[scalar] * wedge[e1234]) + (other[e1234] * wedge[scalar]) + (other[e1] * wedge[e423]) + (other[e2] * wedge[e431]) + (other[e3] * wedge[e412]) + (other[e4] * wedge[e321]) - (other[e41] * wedge[e23]) - (other[e42] * wedge[e31]) - (other[e43] * wedge[e12]) - (other[e23] * wedge[e41]) - (other[e31] * wedge[e42]) - (other[e12] * wedge[e43]) - (other[e423] * wedge[e1]) - (other[e431] * wedge[e2]) - (other[e412] * wedge[e3]) - (other[e321] * wedge[e4]), other[e1234] * wedge[e1234]), 
            /* e1, e2, e3, e4 */
            float4((other[e41] * wedge[e321]) + (other[e31] * wedge[e412]) + (other[e412] * wedge[e31]) + (other[e321] * wedge[e41]), (other[e42] * wedge[e321]) + (other[e12] * wedge[e423]) + (other[e423] * wedge[e12]) + (other[e321] * wedge[e42]), (other[e43] * wedge[e321]) + (other[e23] * wedge[e431]) + (other[e431] * wedge[e23]) + (other[e321] * wedge[e43]), -(other[e43] * wedge[e412]) - (other[e423] * wedge[e41]) - (other[e431] * wedge[e42]) - (other[e412] * wedge[e43])) + (float4(other[e1234]) * wedge.group1()) + (float4(wedge[e1234]) * other.group1()) - (wedge.group4().yzxx * float4(other.group3().zxy, other[e41])) - float4((wedge.group3().zxy * other.group4().yzx), other[e42] * wedge[e431]), 
            /* e41, e42, e43 */
            (float3(other[e1234]) * wedge.group2()) + (float3(wedge[e1234]) * other.group2()) + (other.group4().zxy * wedge.group4().yzx) - (other.group4().yzx * wedge.group4().zxy), 
            /* e23, e31, e12 */
            (float3(other[e1234]) * wedge.group3()) + (float3(wedge[e1234]) * other.group3()) + (float3(wedge[e321]) * other.group4().xyz) - (float3(other[e321]) * wedge.group4().xyz), 
            /* e423, e431, e412, e321 */
            (float4(other[e1234]) * wedge.group4()) + (float4(wedge[e1234]) * other.group4())
        );
    }
}
extension Plane: ProjectOrthogonallyOnto<Plane> {
    func project_orthogonally_onto(self, other: Plane) -> Plane {
        return Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4(other[e321] * self[e321]) * other.group0()
        );
    }
}
extension Point {
    func div(_rhs: project_orthogonally_onto) -> project_orthogonally_onto_partial<Point> {
        project_orthogonally_onto_partial(this)
    }
}
extension Point: ProjectOrthogonallyOnto<Flector> {
    func project_orthogonally_onto(self, other: Flector) -> Flector {
Flector right_anti_dual = Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other[e321] * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other[e1], other[e2], other[e3], 0.0)
        );
Motor wedge = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(right_anti_dual[e4] * self[e1] * -1.0, right_anti_dual[e4] * self[e2] * -1.0, right_anti_dual[e4] * self[e3] * -1.0, (right_anti_dual[e431] * self[e2]) + (right_anti_dual[e412] * self[e3]) + (right_anti_dual[e321] * self[e4])) + (self.group0().wwwx * float4(right_anti_dual.group0().xyz, right_anti_dual[e423])), 
            /* e23, e31, e12, scalar */
            float4(((right_anti_dual.group0().zxy * self.group0().yzx) - (right_anti_dual.group0().yzx * self.group0().zxy)), 0.0)
        );
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4((other[e412] * wedge[e31]) + (other[e321] * wedge[e41]), (other[e423] * wedge[e12]) + (other[e321] * wedge[e42]), (other[e431] * wedge[e23]) + (other[e321] * wedge[e43]), -(other[e431] * wedge[e42]) - (other[e412] * wedge[e43])) + (float4(wedge[e1234]) * other.group0()) - (other.group1().yzxx * float4(wedge.group1().zxy, wedge[e41])), 
            /* e423, e431, e412, e321 */
            float4(wedge[e1234]) * other.group1()
        );
    }
}
extension Point: ProjectOrthogonallyOnto<Horizon> {
    func project_orthogonally_onto(self, other: Horizon) -> Point {
Origin right_anti_dual = Origin::from_groups(
            /* e4 */
            other[e321] * -1.0
        );
        return Point::from_groups(
            /* e1, e2, e3, e4 */
            float4(other[e321], other[e321], other[e321], 0.0) * float4(float3(right_anti_dual[e4] * self[e1] * -1.0, right_anti_dual[e4] * self[e2] * -1.0, right_anti_dual[e4] * self[e3] * -1.0), 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Point: ProjectOrthogonallyOnto<Line> {
    func project_orthogonally_onto(self, other: Line) -> Point {
Line right_anti_dual = Line::from_groups(
            /* e41, e42, e43 */
            other.group1() * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
Plane wedge = Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4((float3(self[e4]) * right_anti_dual.group1()), 0.0) + float4((right_anti_dual.group0().yzx * self.group0().zxy), 0.0) - float4((right_anti_dual.group0().zxy * self.group0().yzx), 0.0)
        );
        return Point::from_groups(
            /* e1, e2, e3, e4 */
            float4((other[e41] * wedge[e321]) + (other[e31] * wedge[e412]), (other[e42] * wedge[e321]) + (other[e12] * wedge[e423]), (other[e43] * wedge[e321]) + (other[e23] * wedge[e431]), -(other[e42] * wedge[e431]) - (other[e43] * wedge[e412])) - (wedge.group0().yzxx * float4(other.group1().zxy, other[e41]))
        );
    }
}
extension Point: ProjectOrthogonallyOnto<Motor> {
    func project_orthogonally_onto(self, other: Motor) -> Flector {
Motor right_anti_dual = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            other.group1() * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
Flector wedge = Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(right_anti_dual[scalar]) * self.group0(), 
            /* e423, e431, e412, e321 */
            float4((float3(self[e4]) * right_anti_dual.group1().xyz), 0.0) + float4((right_anti_dual.group0().yzx * self.group0().zxy), 0.0) - float4((right_anti_dual.group0().zxy * self.group0().yzx), 0.0)
        );
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4((wedge[e412] * other[e31]) + (wedge[e321] * other[e41]), (wedge[e423] * other[e12]) + (wedge[e321] * other[e42]), (wedge[e431] * other[e23]) + (wedge[e321] * other[e43]), -(wedge[e431] * other[e42]) - (wedge[e412] * other[e43])) + (float4(other[e1234]) * wedge.group0()) - (wedge.group1().yzxx * float4(other.group1().zxy, other[e41])), 
            /* e423, e431, e412, e321 */
            float4(other[e1234]) * wedge.group1()
        );
    }
}
extension Point: ProjectOrthogonallyOnto<MultiVector> {
    func project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
MultiVector right_anti_dual = MultiVector::from_groups(
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
MultiVector wedge = MultiVector::from_groups(
            /* scalar, e1234 */
            float2(0.0, (right_anti_dual[e423] * self[e1]) + (right_anti_dual[e431] * self[e2]) + (right_anti_dual[e412] * self[e3]) + (right_anti_dual[e321] * self[e4])), 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual[scalar]) * self.group0(), 
            /* e41, e42, e43 */
            (float3(self[e4]) * right_anti_dual.group1().xyz) - (float3(right_anti_dual[e4]) * self.group0().xyz), 
            /* e23, e31, e12 */
            (right_anti_dual.group1().zxy * self.group0().yzx) - (right_anti_dual.group1().yzx * self.group0().zxy), 
            /* e423, e431, e412, e321 */
            float4((float3(self[e4]) * right_anti_dual.group3()), 0.0) + float4((right_anti_dual.group2().yzx * self.group0().zxy), 0.0) - float4((right_anti_dual.group2().zxy * self.group0().yzx), 0.0)
        );
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2((other[scalar] * wedge[e1234]) + (other[e1234] * wedge[scalar]) + (other[e1] * wedge[e423]) + (other[e2] * wedge[e431]) + (other[e3] * wedge[e412]) + (other[e4] * wedge[e321]) - (other[e41] * wedge[e23]) - (other[e42] * wedge[e31]) - (other[e43] * wedge[e12]) - (other[e23] * wedge[e41]) - (other[e31] * wedge[e42]) - (other[e12] * wedge[e43]) - (other[e423] * wedge[e1]) - (other[e431] * wedge[e2]) - (other[e412] * wedge[e3]) - (other[e321] * wedge[e4]), other[e1234] * wedge[e1234]), 
            /* e1, e2, e3, e4 */
            float4((other[e41] * wedge[e321]) + (other[e31] * wedge[e412]) + (other[e412] * wedge[e31]) + (other[e321] * wedge[e41]), (other[e42] * wedge[e321]) + (other[e12] * wedge[e423]) + (other[e423] * wedge[e12]) + (other[e321] * wedge[e42]), (other[e43] * wedge[e321]) + (other[e23] * wedge[e431]) + (other[e431] * wedge[e23]) + (other[e321] * wedge[e43]), -(other[e43] * wedge[e412]) - (other[e423] * wedge[e41]) - (other[e431] * wedge[e42]) - (other[e412] * wedge[e43])) + (float4(other[e1234]) * wedge.group1()) + (float4(wedge[e1234]) * other.group1()) - (wedge.group4().yzxx * float4(other.group3().zxy, other[e41])) - float4((wedge.group3().zxy * other.group4().yzx), other[e42] * wedge[e431]), 
            /* e41, e42, e43 */
            (float3(other[e1234]) * wedge.group2()) + (float3(wedge[e1234]) * other.group2()) + (other.group4().zxy * wedge.group4().yzx) - (other.group4().yzx * wedge.group4().zxy), 
            /* e23, e31, e12 */
            (float3(other[e1234]) * wedge.group3()) + (float3(wedge[e1234]) * other.group3()) + (float3(wedge[e321]) * other.group4().xyz) - (float3(other[e321]) * wedge.group4().xyz), 
            /* e423, e431, e412, e321 */
            (float4(other[e1234]) * wedge.group4()) + (float4(wedge[e1234]) * other.group4())
        );
    }
}
extension Point: ProjectOrthogonallyOnto<Plane> {
    func project_orthogonally_onto(self, other: Plane) -> Point {
Line wedge = Line::from_groups(
            /* e41, e42, e43 */
            float3(other[e321] * -1.0) * self.group0().xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
        return Point::from_groups(
            /* e1, e2, e3, e4 */
            float4((wedge[e41] * other[e321]) + (wedge[e31] * other[e412]), (wedge[e42] * other[e321]) + (wedge[e12] * other[e423]), (wedge[e43] * other[e321]) + (wedge[e23] * other[e431]), -(wedge[e42] * other[e431]) - (wedge[e43] * other[e412])) - (other.group0().yzxx * float4(wedge.group1().zxy, wedge[e41]))
        );
    }
}
extension Point: ProjectOrthogonallyOnto<Point> {
    func project_orthogonally_onto(self, other: Point) -> Point {
Plane right_anti_dual = Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4(other[e1], other[e2], other[e3], 0.0)
        );
        return Point::from_groups(
            /* e1, e2, e3, e4 */
            float4((right_anti_dual[e423] * self[e1]) + (right_anti_dual[e431] * self[e2]) + (right_anti_dual[e412] * self[e3]) + (right_anti_dual[e321] * self[e4])) * other.group0()
        );
    }
}
extension Scalar {
    func div(_rhs: project_orthogonally_onto) -> project_orthogonally_onto_partial<Scalar> {
        project_orthogonally_onto_partial(this)
    }
}
extension Scalar: ProjectOrthogonallyOnto<DualNum> {
    func project_orthogonally_onto(self, other: DualNum) -> DualNum {
        return DualNum::from_groups(
            /* scalar, e1234 */
            float2(other[scalar] * self[scalar]) * other.group0()
        );
    }
}
extension Scalar: ProjectOrthogonallyOnto<Flector> {
    func project_orthogonally_onto(self, other: Flector) -> Motor {
Flector right_anti_dual = Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other[e321] * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other[e1], other[e2], other[e3], 0.0)
        );
Flector wedge = Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(self[scalar]) * right_anti_dual.group0(), 
            /* e423, e431, e412, e321 */
            float4(self[scalar]) * right_anti_dual.group1()
        );
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(((other.group1().zxy * wedge.group1().yzx) - (other.group1().yzx * wedge.group1().zxy)), 0.0), 
            /* e23, e31, e12, scalar */
            (wedge.group1().wwwx * float4(other.group1().xyz, other[e1])) + float4(float3(0.0), (other[e2] * wedge[e431]) + (other[e3] * wedge[e412]) + (other[e4] * wedge[e321]) - (other[e431] * wedge[e2]) - (other[e412] * wedge[e3]) - (other[e321] * wedge[e4])) - (other.group1().wwwx * float4(wedge.group1().xyz, wedge[e1]))
        );
    }
}
extension Scalar: ProjectOrthogonallyOnto<Horizon> {
    func project_orthogonally_onto(self, other: Horizon) -> Scalar {
        return Scalar::from_groups(
            /* scalar */
            powi(other[e321], 2) * self[scalar]
        );
    }
}
extension Scalar: ProjectOrthogonallyOnto<Line> {
    func project_orthogonally_onto(self, other: Line) -> Scalar {
Line right_anti_dual = Line::from_groups(
            /* e41, e42, e43 */
            other.group1() * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
Line wedge = Line::from_groups(
            /* e41, e42, e43 */
            float3(self[scalar]) * right_anti_dual.group0(), 
            /* e23, e31, e12 */
            float3(self[scalar]) * right_anti_dual.group1()
        );
        return Scalar::from_groups(
            /* scalar */
            -(other[e41] * wedge[e23]) - (other[e42] * wedge[e31]) - (other[e43] * wedge[e12]) - (other[e23] * wedge[e41]) - (other[e31] * wedge[e42]) - (other[e12] * wedge[e43])
        );
    }
}
extension Scalar: ProjectOrthogonallyOnto<Motor> {
    func project_orthogonally_onto(self, other: Motor) -> Motor {
Motor right_anti_dual = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            other.group1() * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
Motor wedge = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(self[scalar]) * right_anti_dual.group0(), 
            /* e23, e31, e12, scalar */
            float4(self[scalar]) * right_anti_dual.group1()
        );
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(((float3(other[e1234]) * wedge.group0().xyz) + (float3(wedge[e1234]) * other.group0().xyz)), other[e1234] * wedge[e1234]), 
            /* e23, e31, e12, scalar */
            (float4(other[e1234]) * wedge.group1()) + (float4(wedge[e1234]) * other.group1()) + float4(float3(0.0), -(other[e41] * wedge[e23]) - (other[e42] * wedge[e31]) - (other[e43] * wedge[e12]) - (other[e23] * wedge[e41]) - (other[e31] * wedge[e42]) - (other[e12] * wedge[e43]))
        );
    }
}
extension Scalar: ProjectOrthogonallyOnto<MultiVector> {
    func project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
MultiVector right_anti_dual = MultiVector::from_groups(
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
MultiVector wedge = MultiVector::from_groups(
            /* scalar, e1234 */
            float2(self[scalar]) * right_anti_dual.group0(), 
            /* e1, e2, e3, e4 */
            float4(self[scalar]) * right_anti_dual.group1(), 
            /* e41, e42, e43 */
            float3(self[scalar]) * right_anti_dual.group2(), 
            /* e23, e31, e12 */
            float3(self[scalar]) * right_anti_dual.group3(), 
            /* e423, e431, e412, e321 */
            float4(self[scalar]) * right_anti_dual.group4()
        );
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2((other[scalar] * wedge[e1234]) + (other[e1234] * wedge[scalar]) + (other[e1] * wedge[e423]) + (other[e2] * wedge[e431]) + (other[e3] * wedge[e412]) + (other[e4] * wedge[e321]) - (other[e41] * wedge[e23]) - (other[e42] * wedge[e31]) - (other[e43] * wedge[e12]) - (other[e23] * wedge[e41]) - (other[e31] * wedge[e42]) - (other[e12] * wedge[e43]) - (other[e423] * wedge[e1]) - (other[e431] * wedge[e2]) - (other[e412] * wedge[e3]) - (other[e321] * wedge[e4]), other[e1234] * wedge[e1234]), 
            /* e1, e2, e3, e4 */
            float4((other[e41] * wedge[e321]) + (other[e31] * wedge[e412]) + (other[e412] * wedge[e31]) + (other[e321] * wedge[e41]), (other[e42] * wedge[e321]) + (other[e12] * wedge[e423]) + (other[e423] * wedge[e12]) + (other[e321] * wedge[e42]), (other[e43] * wedge[e321]) + (other[e23] * wedge[e431]) + (other[e431] * wedge[e23]) + (other[e321] * wedge[e43]), -(other[e43] * wedge[e412]) - (other[e423] * wedge[e41]) - (other[e431] * wedge[e42]) - (other[e412] * wedge[e43])) + (float4(other[e1234]) * wedge.group1()) + (float4(wedge[e1234]) * other.group1()) - (wedge.group4().yzxx * float4(other.group3().zxy, other[e41])) - float4((wedge.group3().zxy * other.group4().yzx), other[e42] * wedge[e431]), 
            /* e41, e42, e43 */
            (float3(other[e1234]) * wedge.group2()) + (float3(wedge[e1234]) * other.group2()) + (other.group4().zxy * wedge.group4().yzx) - (other.group4().yzx * wedge.group4().zxy), 
            /* e23, e31, e12 */
            (float3(other[e1234]) * wedge.group3()) + (float3(wedge[e1234]) * other.group3()) + (float3(wedge[e321]) * other.group4().xyz) - (float3(other[e321]) * wedge.group4().xyz), 
            /* e423, e431, e412, e321 */
            (float4(other[e1234]) * wedge.group4()) + (float4(wedge[e1234]) * other.group4())
        );
    }
}
extension Scalar: ProjectOrthogonallyOnto<Plane> {
    func project_orthogonally_onto(self, other: Plane) -> Scalar {
        return Scalar::from_groups(
            /* scalar */
            powi(other[e321], 2) * self[scalar]
        );
    }
}
extension Scalar: ProjectOrthogonallyOnto<Point> {
    func project_orthogonally_onto(self, other: Point) -> Scalar {
Plane wedge = Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4(self[scalar]) * float4(other[e1], other[e2], other[e3], 0.0)
        );
        return Scalar::from_groups(
            /* scalar */
            (wedge[e423] * other[e1]) + (wedge[e431] * other[e2]) + (wedge[e412] * other[e3]) + (wedge[e321] * other[e4])
        );
    }
}
extension Scalar: ProjectOrthogonallyOnto<Scalar> {
    func project_orthogonally_onto(self, other: Scalar) -> Scalar {
        return Scalar::from_groups(
            /* scalar */
            powi(other[scalar], 2) * self[scalar]
        );
    }
}
