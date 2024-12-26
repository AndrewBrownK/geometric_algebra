extension AntiScalar {
    func div(_rhs: anti_project_orthogonally_onto) -> anti_project_orthogonally_onto_partial<AntiScalar> {
        anti_project_orthogonally_onto_partial(this)
    }
}
extension AntiScalar: AntiProjectOrthogonallyOnto<DualNum> {
    func anti_project_orthogonally_onto(self, other: DualNum) -> AntiScalar {
        return AntiScalar::from_groups(
            /* e1234 */
            powi(other[scalar], 2) * self[e1234]
        );
    }
}
extension AntiScalar: AntiProjectOrthogonallyOnto<Flector> {
    func anti_project_orthogonally_onto(self, other: Flector) -> Motor {
Flector right_anti_dual = Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other[e321] * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other[e1], other[e2], other[e3], 0.0)
        );
Flector anti_wedge = Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(self[e1234]) * right_anti_dual.group0(), 
            /* e423, e431, e412, e321 */
            float4(self[e1234]) * right_anti_dual.group1()
        );
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            (other.group0().wwwx * float4(anti_wedge.group0().xyz, anti_wedge[e423])) + float4(float3(0.0), (anti_wedge[e431] * other[e2]) + (anti_wedge[e412] * other[e3]) + (anti_wedge[e321] * other[e4]) - (anti_wedge[e2] * other[e431]) - (anti_wedge[e3] * other[e412]) - (anti_wedge[e4] * other[e321])) - (anti_wedge.group0().wwwx * float4(other.group0().xyz, other[e423])), 
            /* e23, e31, e12, scalar */
            float4(((anti_wedge.group0().zxy * other.group0().yzx) - (anti_wedge.group0().yzx * other.group0().zxy)), 0.0)
        );
    }
}
extension AntiScalar: AntiProjectOrthogonallyOnto<Horizon> {
    func anti_project_orthogonally_onto(self, other: Horizon) -> AntiScalar {
        return AntiScalar::from_groups(
            /* e1234 */
            powi(other[e321], 2) * self[e1234]
        );
    }
}
extension AntiScalar: AntiProjectOrthogonallyOnto<Line> {
    func anti_project_orthogonally_onto(self, other: Line) -> AntiScalar {
Line right_anti_dual = Line::from_groups(
            /* e41, e42, e43 */
            other.group1() * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
Line anti_wedge = Line::from_groups(
            /* e41, e42, e43 */
            float3(self[e1234]) * right_anti_dual.group0(), 
            /* e23, e31, e12 */
            float3(self[e1234]) * right_anti_dual.group1()
        );
        return AntiScalar::from_groups(
            /* e1234 */
            -(anti_wedge[e41] * other[e23]) - (anti_wedge[e42] * other[e31]) - (anti_wedge[e43] * other[e12]) - (anti_wedge[e23] * other[e41]) - (anti_wedge[e31] * other[e42]) - (anti_wedge[e12] * other[e43])
        );
    }
}
extension AntiScalar: AntiProjectOrthogonallyOnto<Motor> {
    func anti_project_orthogonally_onto(self, other: Motor) -> Motor {
Motor right_anti_dual = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            other.group1() * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
Motor anti_wedge = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(self[e1234]) * right_anti_dual.group0(), 
            /* e23, e31, e12, scalar */
            float4(self[e1234]) * right_anti_dual.group1()
        );
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            (float4(anti_wedge[scalar]) * other.group0()) + (float4(other[scalar]) * anti_wedge.group0()) + float4(float3(0.0), -(anti_wedge[e41] * other[e23]) - (anti_wedge[e42] * other[e31]) - (anti_wedge[e43] * other[e12]) - (anti_wedge[e23] * other[e41]) - (anti_wedge[e31] * other[e42]) - (anti_wedge[e12] * other[e43])), 
            /* e23, e31, e12, scalar */
            float4(float3(0.0), anti_wedge[scalar] * other[scalar])
        );
    }
}
extension AntiScalar: AntiProjectOrthogonallyOnto<MultiVector> {
    func anti_project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
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
MultiVector anti_wedge = MultiVector::from_groups(
            /* scalar, e1234 */
            float2(self[e1234]) * right_anti_dual.group0(), 
            /* e1, e2, e3, e4 */
            float4(self[e1234]) * right_anti_dual.group1(), 
            /* e41, e42, e43 */
            float3(self[e1234]) * right_anti_dual.group2(), 
            /* e23, e31, e12 */
            float3(self[e1234]) * right_anti_dual.group3(), 
            /* e423, e431, e412, e321 */
            float4(self[e1234]) * right_anti_dual.group4()
        );
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(anti_wedge[scalar] * other[scalar], (anti_wedge[scalar] * other[e1234]) + (anti_wedge[e1234] * other[scalar]) + (anti_wedge[e423] * other[e1]) + (anti_wedge[e431] * other[e2]) + (anti_wedge[e412] * other[e3]) + (anti_wedge[e321] * other[e4]) - (anti_wedge[e1] * other[e423]) - (anti_wedge[e2] * other[e431]) - (anti_wedge[e3] * other[e412]) - (anti_wedge[e4] * other[e321]) - (anti_wedge[e41] * other[e23]) - (anti_wedge[e42] * other[e31]) - (anti_wedge[e43] * other[e12]) - (anti_wedge[e23] * other[e41]) - (anti_wedge[e31] * other[e42]) - (anti_wedge[e12] * other[e43])), 
            /* e1, e2, e3, e4 */
            (float4(anti_wedge[scalar]) * other.group1()) + (float4(other[scalar]) * anti_wedge.group1()), 
            /* e41, e42, e43 */
            (float3(anti_wedge[scalar]) * other.group2()) + (float3(other[scalar]) * anti_wedge.group2()) + (float3(other[e4]) * anti_wedge.group1().xyz) - (float3(anti_wedge[e4]) * other.group1().xyz), 
            /* e23, e31, e12 */
            (float3(anti_wedge[scalar]) * other.group3()) + (float3(other[scalar]) * anti_wedge.group3()) + (anti_wedge.group1().zxy * other.group1().yzx) - (anti_wedge.group1().yzx * other.group1().zxy), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge[e3] * other[e42]) + (anti_wedge[e4] * other[e23]) + (anti_wedge[e42] * other[e3]) + (anti_wedge[e23] * other[e4]), (anti_wedge[e1] * other[e43]) + (anti_wedge[e4] * other[e31]) + (anti_wedge[e43] * other[e1]) + (anti_wedge[e31] * other[e4]), (anti_wedge[e2] * other[e41]) + (anti_wedge[e4] * other[e12]) + (anti_wedge[e41] * other[e2]) + (anti_wedge[e12] * other[e4]), -(anti_wedge[e1] * other[e23]) - (anti_wedge[e2] * other[e31]) - (anti_wedge[e3] * other[e12]) - (anti_wedge[e12] * other[e3])) + (float4(anti_wedge[scalar]) * other.group4()) + (float4(other[scalar]) * anti_wedge.group4()) - (other.group1().yzxx * float4(anti_wedge.group2().zxy, anti_wedge[e23])) - float4((other.group2().zxy * anti_wedge.group1().yzx), anti_wedge[e31] * other[e2])
        );
    }
}
extension AntiScalar: AntiProjectOrthogonallyOnto<Plane> {
    func anti_project_orthogonally_onto(self, other: Plane) -> AntiScalar {
        return AntiScalar::from_groups(
            /* e1234 */
            powi(other[e321], 2) * self[e1234]
        );
    }
}
extension AntiScalar: AntiProjectOrthogonallyOnto<Point> {
    func anti_project_orthogonally_onto(self, other: Point) -> AntiScalar {
Plane anti_wedge = Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4(self[e1234]) * float4(other[e1], other[e2], other[e3], 0.0)
        );
        return AntiScalar::from_groups(
            /* e1234 */
            (anti_wedge[e423] * other[e1]) + (anti_wedge[e431] * other[e2]) + (anti_wedge[e412] * other[e3]) + (anti_wedge[e321] * other[e4])
        );
    }
}
extension AntiScalar: AntiProjectOrthogonallyOnto<Scalar> {
    func anti_project_orthogonally_onto(self, other: Scalar) -> AntiScalar {
        return AntiScalar::from_groups(
            /* e1234 */
            powi(other[scalar], 2) * self[e1234]
        );
    }
}
extension DualNum {
    func div(_rhs: anti_project_orthogonally_onto) -> anti_project_orthogonally_onto_partial<DualNum> {
        anti_project_orthogonally_onto_partial(this)
    }
}
extension DualNum: AntiProjectOrthogonallyOnto<DualNum> {
    func anti_project_orthogonally_onto(self, other: DualNum) -> DualNum {
DualNum anti_wedge = DualNum::from_groups(
            /* scalar, e1234 */
            float2(other[scalar]) * self.group0()
        );
        return DualNum::from_groups(
            /* scalar, e1234 */
            float2(anti_wedge[scalar] * other[scalar], (anti_wedge[scalar] * other[e1234]) + (anti_wedge[e1234] * other[scalar]))
        );
    }
}
extension DualNum: AntiProjectOrthogonallyOnto<Flector> {
    func anti_project_orthogonally_onto(self, other: Flector) -> Motor {
Flector right_anti_dual = Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other[e321] * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other[e1], other[e2], other[e3], 0.0)
        );
Flector anti_wedge = Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(self[e1234]) * right_anti_dual.group0(), 
            /* e423, e431, e412, e321 */
            float4(self[e1234]) * right_anti_dual.group1()
        );
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            (other.group0().wwwx * float4(anti_wedge.group0().xyz, anti_wedge[e423])) + float4(float3(0.0), (anti_wedge[e431] * other[e2]) + (anti_wedge[e412] * other[e3]) + (anti_wedge[e321] * other[e4]) - (anti_wedge[e2] * other[e431]) - (anti_wedge[e3] * other[e412]) - (anti_wedge[e4] * other[e321])) - (anti_wedge.group0().wwwx * float4(other.group0().xyz, other[e423])), 
            /* e23, e31, e12, scalar */
            float4(((anti_wedge.group0().zxy * other.group0().yzx) - (anti_wedge.group0().yzx * other.group0().zxy)), 0.0)
        );
    }
}
extension DualNum: AntiProjectOrthogonallyOnto<Horizon> {
    func anti_project_orthogonally_onto(self, other: Horizon) -> AntiScalar {
        return AntiScalar::from_groups(
            /* e1234 */
            powi(other[e321], 2) * self[e1234]
        );
    }
}
extension DualNum: AntiProjectOrthogonallyOnto<Line> {
    func anti_project_orthogonally_onto(self, other: Line) -> AntiScalar {
Line right_anti_dual = Line::from_groups(
            /* e41, e42, e43 */
            other.group1() * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
Line anti_wedge = Line::from_groups(
            /* e41, e42, e43 */
            float3(self[e1234]) * right_anti_dual.group0(), 
            /* e23, e31, e12 */
            float3(self[e1234]) * right_anti_dual.group1()
        );
        return AntiScalar::from_groups(
            /* e1234 */
            -(anti_wedge[e41] * other[e23]) - (anti_wedge[e42] * other[e31]) - (anti_wedge[e43] * other[e12]) - (anti_wedge[e23] * other[e41]) - (anti_wedge[e31] * other[e42]) - (anti_wedge[e12] * other[e43])
        );
    }
}
extension DualNum: AntiProjectOrthogonallyOnto<Motor> {
    func anti_project_orthogonally_onto(self, other: Motor) -> Motor {
Motor right_anti_dual = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            other.group1() * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
Motor anti_wedge = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(self[e1234]) * right_anti_dual.group0(), 
            /* e23, e31, e12, scalar */
            float4(right_anti_dual[e23], right_anti_dual[e31], right_anti_dual[e12], 1.0) * float4(self.group0().yy, self[e1234], (self[scalar] * right_anti_dual[e1234]) + (self[e1234] * right_anti_dual[scalar]))
        );
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            (float4(anti_wedge[scalar]) * other.group0()) + (float4(other[scalar]) * anti_wedge.group0()) + float4(float3(0.0), -(anti_wedge[e41] * other[e23]) - (anti_wedge[e42] * other[e31]) - (anti_wedge[e43] * other[e12]) - (anti_wedge[e23] * other[e41]) - (anti_wedge[e31] * other[e42]) - (anti_wedge[e12] * other[e43])), 
            /* e23, e31, e12, scalar */
            float4(((float3(anti_wedge[scalar]) * other.group1().xyz) + (float3(other[scalar]) * anti_wedge.group1().xyz)), anti_wedge[scalar] * other[scalar])
        );
    }
}
extension DualNum: AntiProjectOrthogonallyOnto<MultiVector> {
    func anti_project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
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
MultiVector anti_wedge = MultiVector::from_groups(
            /* scalar, e1234 */
            float2((self[scalar] * right_anti_dual[e1234]) + (self[e1234] * right_anti_dual[scalar]), self[e1234] * right_anti_dual[e1234]), 
            /* e1, e2, e3, e4 */
            float4(self[e1234]) * right_anti_dual.group1(), 
            /* e41, e42, e43 */
            float3(self[e1234]) * right_anti_dual.group2(), 
            /* e23, e31, e12 */
            float3(self[e1234]) * right_anti_dual.group3(), 
            /* e423, e431, e412, e321 */
            float4(self[e1234]) * right_anti_dual.group4()
        );
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(anti_wedge[scalar] * other[scalar], (anti_wedge[scalar] * other[e1234]) + (anti_wedge[e1234] * other[scalar]) + (anti_wedge[e423] * other[e1]) + (anti_wedge[e431] * other[e2]) + (anti_wedge[e412] * other[e3]) + (anti_wedge[e321] * other[e4]) - (anti_wedge[e1] * other[e423]) - (anti_wedge[e2] * other[e431]) - (anti_wedge[e3] * other[e412]) - (anti_wedge[e4] * other[e321]) - (anti_wedge[e41] * other[e23]) - (anti_wedge[e42] * other[e31]) - (anti_wedge[e43] * other[e12]) - (anti_wedge[e23] * other[e41]) - (anti_wedge[e31] * other[e42]) - (anti_wedge[e12] * other[e43])), 
            /* e1, e2, e3, e4 */
            (float4(anti_wedge[scalar]) * other.group1()) + (float4(other[scalar]) * anti_wedge.group1()), 
            /* e41, e42, e43 */
            (float3(anti_wedge[scalar]) * other.group2()) + (float3(other[scalar]) * anti_wedge.group2()) + (float3(other[e4]) * anti_wedge.group1().xyz) - (float3(anti_wedge[e4]) * other.group1().xyz), 
            /* e23, e31, e12 */
            (float3(anti_wedge[scalar]) * other.group3()) + (float3(other[scalar]) * anti_wedge.group3()) + (anti_wedge.group1().zxy * other.group1().yzx) - (anti_wedge.group1().yzx * other.group1().zxy), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge[e3] * other[e42]) + (anti_wedge[e4] * other[e23]) + (anti_wedge[e42] * other[e3]) + (anti_wedge[e23] * other[e4]), (anti_wedge[e1] * other[e43]) + (anti_wedge[e4] * other[e31]) + (anti_wedge[e43] * other[e1]) + (anti_wedge[e31] * other[e4]), (anti_wedge[e2] * other[e41]) + (anti_wedge[e4] * other[e12]) + (anti_wedge[e41] * other[e2]) + (anti_wedge[e12] * other[e4]), -(anti_wedge[e1] * other[e23]) - (anti_wedge[e2] * other[e31]) - (anti_wedge[e3] * other[e12]) - (anti_wedge[e12] * other[e3])) + (float4(anti_wedge[scalar]) * other.group4()) + (float4(other[scalar]) * anti_wedge.group4()) - (other.group1().yzxx * float4(anti_wedge.group2().zxy, anti_wedge[e23])) - float4((other.group2().zxy * anti_wedge.group1().yzx), anti_wedge[e31] * other[e2])
        );
    }
}
extension DualNum: AntiProjectOrthogonallyOnto<Plane> {
    func anti_project_orthogonally_onto(self, other: Plane) -> AntiScalar {
        return AntiScalar::from_groups(
            /* e1234 */
            powi(other[e321], 2) * self[e1234]
        );
    }
}
extension DualNum: AntiProjectOrthogonallyOnto<Point> {
    func anti_project_orthogonally_onto(self, other: Point) -> AntiScalar {
Plane anti_wedge = Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4(self[e1234]) * float4(other[e1], other[e2], other[e3], 0.0)
        );
        return AntiScalar::from_groups(
            /* e1234 */
            (anti_wedge[e423] * other[e1]) + (anti_wedge[e431] * other[e2]) + (anti_wedge[e412] * other[e3]) + (anti_wedge[e321] * other[e4])
        );
    }
}
extension DualNum: AntiProjectOrthogonallyOnto<Scalar> {
    func anti_project_orthogonally_onto(self, other: Scalar) -> DualNum {
AntiScalar right_anti_dual = AntiScalar::from_groups(
            /* e1234 */
            other[scalar]
        );
        return DualNum::from_groups(
            /* scalar, e1234 */
            float2(other[scalar]) * float2(right_anti_dual[e1234] * self[scalar], right_anti_dual[e1234] * self[e1234])
        );
    }
}
extension Flector {
    func div(_rhs: anti_project_orthogonally_onto) -> anti_project_orthogonally_onto_partial<Flector> {
        anti_project_orthogonally_onto_partial(this)
    }
}
extension Flector: AntiProjectOrthogonallyOnto<DualNum> {
    func anti_project_orthogonally_onto(self, other: DualNum) -> Flector {
AntiScalar right_anti_dual = AntiScalar::from_groups(
            /* e1234 */
            other[scalar]
        );
Flector anti_wedge = Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(right_anti_dual[e1234]) * self.group0(), 
            /* e423, e431, e412, e321 */
            float4(right_anti_dual[e1234]) * self.group1()
        );
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(other[scalar]) * anti_wedge.group0(), 
            /* e423, e431, e412, e321 */
            float4(other[scalar]) * anti_wedge.group1()
        );
    }
}
extension Flector: AntiProjectOrthogonallyOnto<Flector> {
    func anti_project_orthogonally_onto(self, other: Flector) -> Flector {
Flector right_anti_dual = Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other[e321] * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other[e1], other[e2], other[e3], 0.0)
        );
Motor anti_wedge = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(((right_anti_dual.group1().yzx * self.group1().zxy) - (right_anti_dual.group1().zxy * self.group1().yzx)), 0.0), 
            /* e23, e31, e12, scalar */
            (right_anti_dual.group1().wwwx * float4(self.group1().xyz, self[e1])) + float4(float3(0.0), (right_anti_dual[e431] * self[e2]) + (right_anti_dual[e412] * self[e3]) + (right_anti_dual[e321] * self[e4]) - (right_anti_dual[e2] * self[e431]) - (right_anti_dual[e3] * self[e412]) - (right_anti_dual[e4] * self[e321])) - (self.group1().wwwx * float4(right_anti_dual.group1().xyz, right_anti_dual[e1]))
        );
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(anti_wedge[scalar]) * other.group0(), 
            /* e423, e431, e412, e321 */
            float4((other[e4] * anti_wedge[e23]) + (other[e423] * anti_wedge[scalar]), (other[e4] * anti_wedge[e31]) + (other[e431] * anti_wedge[scalar]), (other[e4] * anti_wedge[e12]) + (other[e412] * anti_wedge[scalar]), -(other[e2] * anti_wedge[e31]) - (other[e3] * anti_wedge[e12])) + float4((other.group0().zxy * anti_wedge.group0().yzx), other[e321] * anti_wedge[scalar]) - (other.group0().yzxx * float4(anti_wedge.group0().zxy, anti_wedge[e23]))
        );
    }
}
extension Flector: AntiProjectOrthogonallyOnto<Horizon> {
    func anti_project_orthogonally_onto(self, other: Horizon) -> Horizon {
        return Horizon::from_groups(
            /* e321 */
            powi(other[e321], 2) * self[e321]
        );
    }
}
extension Flector: AntiProjectOrthogonallyOnto<Line> {
    func anti_project_orthogonally_onto(self, other: Line) -> Plane {
Line right_anti_dual = Line::from_groups(
            /* e41, e42, e43 */
            other.group1() * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
Point anti_wedge = Point::from_groups(
            /* e1, e2, e3, e4 */
            float4((self[e412] * right_anti_dual[e31]) + (self[e321] * right_anti_dual[e41]), (self[e423] * right_anti_dual[e12]) + (self[e321] * right_anti_dual[e42]), (self[e431] * right_anti_dual[e23]) + (self[e321] * right_anti_dual[e43]), -(self[e431] * right_anti_dual[e42]) - (self[e412] * right_anti_dual[e43])) - (self.group1().yzxx * float4(right_anti_dual.group1().zxy, right_anti_dual[e41]))
        );
        return Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4((other[e42] * anti_wedge[e3]) + (other[e23] * anti_wedge[e4]), (other[e43] * anti_wedge[e1]) + (other[e31] * anti_wedge[e4]), (other[e41] * anti_wedge[e2]) + (other[e12] * anti_wedge[e4]), -(other[e31] * anti_wedge[e2]) - (other[e12] * anti_wedge[e3])) - (anti_wedge.group0().yzxx * float4(other.group0().zxy, other[e23]))
        );
    }
}
extension Flector: AntiProjectOrthogonallyOnto<Motor> {
    func anti_project_orthogonally_onto(self, other: Motor) -> Flector {
Motor right_anti_dual = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            other.group1() * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
Flector anti_wedge = Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4((self[e412] * right_anti_dual[e31]) + (self[e321] * right_anti_dual[e41]), (self[e423] * right_anti_dual[e12]) + (self[e321] * right_anti_dual[e42]), (self[e431] * right_anti_dual[e23]) + (self[e321] * right_anti_dual[e43]), -(self[e431] * right_anti_dual[e42]) - (self[e412] * right_anti_dual[e43])) + (float4(right_anti_dual[e1234]) * self.group0()) - (self.group1().yzxx * float4(right_anti_dual.group1().zxy, right_anti_dual[e41])), 
            /* e423, e431, e412, e321 */
            float4(right_anti_dual[e1234]) * self.group1()
        );
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(other[scalar]) * anti_wedge.group0(), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge[e4] * other[e23]) + (anti_wedge[e423] * other[scalar]), (anti_wedge[e4] * other[e31]) + (anti_wedge[e431] * other[scalar]), (anti_wedge[e4] * other[e12]) + (anti_wedge[e412] * other[scalar]), -(anti_wedge[e2] * other[e31]) - (anti_wedge[e3] * other[e12])) + float4((anti_wedge.group0().zxy * other.group0().yzx), anti_wedge[e321] * other[scalar]) - (anti_wedge.group0().yzxx * float4(other.group0().zxy, other[e23]))
        );
    }
}
extension Flector: AntiProjectOrthogonallyOnto<MultiVector> {
    func anti_project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
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
MultiVector anti_wedge = MultiVector::from_groups(
            /* scalar, e1234 */
            float2((self[e1] * right_anti_dual[e423]) + (self[e2] * right_anti_dual[e431]) + (self[e3] * right_anti_dual[e412]) + (self[e4] * right_anti_dual[e321]) - (self[e423] * right_anti_dual[e1]) - (self[e431] * right_anti_dual[e2]) - (self[e412] * right_anti_dual[e3]) - (self[e321] * right_anti_dual[e4]), 0.0), 
            /* e1, e2, e3, e4 */
            float4((self[e412] * right_anti_dual[e31]) + (self[e321] * right_anti_dual[e41]), (self[e423] * right_anti_dual[e12]) + (self[e321] * right_anti_dual[e42]), (self[e431] * right_anti_dual[e23]) + (self[e321] * right_anti_dual[e43]), -(self[e431] * right_anti_dual[e42]) - (self[e412] * right_anti_dual[e43])) + (float4(right_anti_dual[e1234]) * self.group0()) - (self.group1().yzxx * float4(right_anti_dual.group3().zxy, right_anti_dual[e41])), 
            /* e41, e42, e43 */
            (self.group1().zxy * right_anti_dual.group4().yzx) - (self.group1().yzx * right_anti_dual.group4().zxy), 
            /* e23, e31, e12 */
            (float3(right_anti_dual[e321]) * self.group1().xyz) - (float3(self[e321]) * right_anti_dual.group4().xyz), 
            /* e423, e431, e412, e321 */
            float4(right_anti_dual[e1234]) * self.group1()
        );
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(anti_wedge[scalar] * other[scalar], (anti_wedge[scalar] * other[e1234]) + (anti_wedge[e1234] * other[scalar]) + (anti_wedge[e423] * other[e1]) + (anti_wedge[e431] * other[e2]) + (anti_wedge[e412] * other[e3]) + (anti_wedge[e321] * other[e4]) - (anti_wedge[e1] * other[e423]) - (anti_wedge[e2] * other[e431]) - (anti_wedge[e3] * other[e412]) - (anti_wedge[e4] * other[e321]) - (anti_wedge[e41] * other[e23]) - (anti_wedge[e42] * other[e31]) - (anti_wedge[e43] * other[e12]) - (anti_wedge[e23] * other[e41]) - (anti_wedge[e31] * other[e42]) - (anti_wedge[e12] * other[e43])), 
            /* e1, e2, e3, e4 */
            (float4(anti_wedge[scalar]) * other.group1()) + (float4(other[scalar]) * anti_wedge.group1()), 
            /* e41, e42, e43 */
            (float3(anti_wedge[scalar]) * other.group2()) + (float3(other[scalar]) * anti_wedge.group2()) + (float3(other[e4]) * anti_wedge.group1().xyz) - (float3(anti_wedge[e4]) * other.group1().xyz), 
            /* e23, e31, e12 */
            (float3(anti_wedge[scalar]) * other.group3()) + (float3(other[scalar]) * anti_wedge.group3()) + (anti_wedge.group1().zxy * other.group1().yzx) - (anti_wedge.group1().yzx * other.group1().zxy), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge[e3] * other[e42]) + (anti_wedge[e4] * other[e23]) + (anti_wedge[e42] * other[e3]) + (anti_wedge[e23] * other[e4]), (anti_wedge[e1] * other[e43]) + (anti_wedge[e4] * other[e31]) + (anti_wedge[e43] * other[e1]) + (anti_wedge[e31] * other[e4]), (anti_wedge[e2] * other[e41]) + (anti_wedge[e4] * other[e12]) + (anti_wedge[e41] * other[e2]) + (anti_wedge[e12] * other[e4]), -(anti_wedge[e1] * other[e23]) - (anti_wedge[e2] * other[e31]) - (anti_wedge[e3] * other[e12]) - (anti_wedge[e12] * other[e3])) + (float4(anti_wedge[scalar]) * other.group4()) + (float4(other[scalar]) * anti_wedge.group4()) - (other.group1().yzxx * float4(anti_wedge.group2().zxy, anti_wedge[e23])) - float4((other.group2().zxy * anti_wedge.group1().yzx), anti_wedge[e31] * other[e2])
        );
    }
}
extension Flector: AntiProjectOrthogonallyOnto<Plane> {
    func anti_project_orthogonally_onto(self, other: Plane) -> Plane {
        return Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4(self[e321] * other[e321]) * other.group0()
        );
    }
}
extension Flector: AntiProjectOrthogonallyOnto<Point> {
    func anti_project_orthogonally_onto(self, other: Point) -> Flector {
Plane right_anti_dual = Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4(other[e1], other[e2], other[e3], 0.0)
        );
Motor anti_wedge = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(((self.group1().zxy * right_anti_dual.group0().yzx) - (self.group1().yzx * right_anti_dual.group0().zxy)), 0.0), 
            /* e23, e31, e12, scalar */
            float4(self[e321] * right_anti_dual[e423] * -1.0, self[e321] * right_anti_dual[e431] * -1.0, self[e321] * right_anti_dual[e412] * -1.0, (self[e2] * right_anti_dual[e431]) + (self[e3] * right_anti_dual[e412]) + (self[e4] * right_anti_dual[e321])) + (right_anti_dual.group0().wwwx * float4(self.group1().xyz, self[e1]))
        );
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(anti_wedge[scalar]) * other.group0(), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge[e42] * other[e3]) + (anti_wedge[e23] * other[e4]), (anti_wedge[e43] * other[e1]) + (anti_wedge[e31] * other[e4]), (anti_wedge[e41] * other[e2]) + (anti_wedge[e12] * other[e4]), -(anti_wedge[e31] * other[e2]) - (anti_wedge[e12] * other[e3])) - (other.group0().yzxx * float4(anti_wedge.group0().zxy, anti_wedge[e23]))
        );
    }
}
extension Flector: AntiProjectOrthogonallyOnto<Scalar> {
    func anti_project_orthogonally_onto(self, other: Scalar) -> Flector {
AntiScalar right_anti_dual = AntiScalar::from_groups(
            /* e1234 */
            other[scalar]
        );
Flector anti_wedge = Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(right_anti_dual[e1234]) * self.group0(), 
            /* e423, e431, e412, e321 */
            float4(right_anti_dual[e1234]) * self.group1()
        );
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(other[scalar]) * anti_wedge.group0(), 
            /* e423, e431, e412, e321 */
            float4(other[scalar]) * anti_wedge.group1()
        );
    }
}
extension Horizon {
    func div(_rhs: anti_project_orthogonally_onto) -> anti_project_orthogonally_onto_partial<Horizon> {
        anti_project_orthogonally_onto_partial(this)
    }
}
extension Horizon: AntiProjectOrthogonallyOnto<DualNum> {
    func anti_project_orthogonally_onto(self, other: DualNum) -> Horizon {
        return Horizon::from_groups(
            /* e321 */
            powi(other[scalar], 2) * self[e321]
        );
    }
}
extension Horizon: AntiProjectOrthogonallyOnto<Flector> {
    func anti_project_orthogonally_onto(self, other: Flector) -> Flector {
Flector right_anti_dual = Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other[e321] * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other[e1], other[e2], other[e3], 0.0)
        );
Motor anti_wedge = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(0.0), 
            /* e23, e31, e12, scalar */
            float4(self[e321]) * float4(right_anti_dual.group1().xyz, right_anti_dual[e4]) * float4(-1.0)
        );
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(anti_wedge[scalar]) * other.group0(), 
            /* e423, e431, e412, e321 */
            float4((other[e4] * anti_wedge[e23]) + (other[e423] * anti_wedge[scalar]), (other[e4] * anti_wedge[e31]) + (other[e431] * anti_wedge[scalar]), (other[e4] * anti_wedge[e12]) + (other[e412] * anti_wedge[scalar]), -(other[e2] * anti_wedge[e31]) - (other[e3] * anti_wedge[e12])) + float4((other.group0().zxy * anti_wedge.group0().yzx), other[e321] * anti_wedge[scalar]) - (other.group0().yzxx * float4(anti_wedge.group0().zxy, anti_wedge[e23]))
        );
    }
}
extension Horizon: AntiProjectOrthogonallyOnto<Horizon> {
    func anti_project_orthogonally_onto(self, other: Horizon) -> Horizon {
        return Horizon::from_groups(
            /* e321 */
            powi(other[e321], 2) * self[e321]
        );
    }
}
extension Horizon: AntiProjectOrthogonallyOnto<Line> {
    func anti_project_orthogonally_onto(self, other: Line) -> Plane {
Point anti_wedge = Point::from_groups(
            /* e1, e2, e3, e4 */
            float4(self[e321], self[e321], self[e321], 0.0) * float4(float3(other[e23] * -1.0, other[e31] * -1.0, other[e12] * -1.0), 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
        return Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4((other[e42] * anti_wedge[e3]) + (other[e23] * anti_wedge[e4]), (other[e43] * anti_wedge[e1]) + (other[e31] * anti_wedge[e4]), (other[e41] * anti_wedge[e2]) + (other[e12] * anti_wedge[e4]), -(other[e31] * anti_wedge[e2]) - (other[e12] * anti_wedge[e3])) - (anti_wedge.group0().yzxx * float4(other.group0().zxy, other[e23]))
        );
    }
}
extension Horizon: AntiProjectOrthogonallyOnto<Motor> {
    func anti_project_orthogonally_onto(self, other: Motor) -> Flector {
Motor right_anti_dual = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            other.group1() * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
Flector anti_wedge = Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(self[e321], self[e321], self[e321], 0.0) * float4(right_anti_dual.group0().xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), self[e321] * right_anti_dual[e1234])
        );
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(other[scalar]) * anti_wedge.group0(), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge[e4] * other[e23]) + (anti_wedge[e423] * other[scalar]), (anti_wedge[e4] * other[e31]) + (anti_wedge[e431] * other[scalar]), (anti_wedge[e4] * other[e12]) + (anti_wedge[e412] * other[scalar]), -(anti_wedge[e2] * other[e31]) - (anti_wedge[e3] * other[e12])) + float4((anti_wedge.group0().zxy * other.group0().yzx), anti_wedge[e321] * other[scalar]) - (anti_wedge.group0().yzxx * float4(other.group0().zxy, other[e23]))
        );
    }
}
extension Horizon: AntiProjectOrthogonallyOnto<MultiVector> {
    func anti_project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
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
MultiVector anti_wedge = MultiVector::from_groups(
            /* scalar, e1234 */
            float2(self[e321] * right_anti_dual[e4], 1.0) * float2(-1.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(self[e321], self[e321], self[e321], 0.0) * float4(right_anti_dual.group2(), 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(self[e321]) * right_anti_dual.group4().xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), self[e321] * right_anti_dual[e1234])
        );
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(anti_wedge[scalar] * other[scalar], (anti_wedge[scalar] * other[e1234]) + (anti_wedge[e1234] * other[scalar]) + (anti_wedge[e423] * other[e1]) + (anti_wedge[e431] * other[e2]) + (anti_wedge[e412] * other[e3]) + (anti_wedge[e321] * other[e4]) - (anti_wedge[e1] * other[e423]) - (anti_wedge[e2] * other[e431]) - (anti_wedge[e3] * other[e412]) - (anti_wedge[e4] * other[e321]) - (anti_wedge[e41] * other[e23]) - (anti_wedge[e42] * other[e31]) - (anti_wedge[e43] * other[e12]) - (anti_wedge[e23] * other[e41]) - (anti_wedge[e31] * other[e42]) - (anti_wedge[e12] * other[e43])), 
            /* e1, e2, e3, e4 */
            (float4(anti_wedge[scalar]) * other.group1()) + (float4(other[scalar]) * anti_wedge.group1()), 
            /* e41, e42, e43 */
            (float3(anti_wedge[scalar]) * other.group2()) + (float3(other[scalar]) * anti_wedge.group2()) + (float3(other[e4]) * anti_wedge.group1().xyz) - (float3(anti_wedge[e4]) * other.group1().xyz), 
            /* e23, e31, e12 */
            (float3(anti_wedge[scalar]) * other.group3()) + (float3(other[scalar]) * anti_wedge.group3()) + (anti_wedge.group1().zxy * other.group1().yzx) - (anti_wedge.group1().yzx * other.group1().zxy), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge[e3] * other[e42]) + (anti_wedge[e4] * other[e23]) + (anti_wedge[e42] * other[e3]) + (anti_wedge[e23] * other[e4]), (anti_wedge[e1] * other[e43]) + (anti_wedge[e4] * other[e31]) + (anti_wedge[e43] * other[e1]) + (anti_wedge[e31] * other[e4]), (anti_wedge[e2] * other[e41]) + (anti_wedge[e4] * other[e12]) + (anti_wedge[e41] * other[e2]) + (anti_wedge[e12] * other[e4]), -(anti_wedge[e1] * other[e23]) - (anti_wedge[e2] * other[e31]) - (anti_wedge[e3] * other[e12]) - (anti_wedge[e12] * other[e3])) + (float4(anti_wedge[scalar]) * other.group4()) + (float4(other[scalar]) * anti_wedge.group4()) - (other.group1().yzxx * float4(anti_wedge.group2().zxy, anti_wedge[e23])) - float4((other.group2().zxy * anti_wedge.group1().yzx), anti_wedge[e31] * other[e2])
        );
    }
}
extension Horizon: AntiProjectOrthogonallyOnto<Plane> {
    func anti_project_orthogonally_onto(self, other: Plane) -> Plane {
        return Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4(self[e321] * other[e321]) * other.group0()
        );
    }
}
extension Horizon: AntiProjectOrthogonallyOnto<Point> {
    func anti_project_orthogonally_onto(self, other: Point) -> Plane {
Line anti_wedge = Line::from_groups(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(self[e321]) * float4(other[e1], other[e2], other[e3], 0.0).xyz * float3(-1.0)
        );
        return Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4((anti_wedge[e42] * other[e3]) + (anti_wedge[e23] * other[e4]), (anti_wedge[e43] * other[e1]) + (anti_wedge[e31] * other[e4]), (anti_wedge[e41] * other[e2]) + (anti_wedge[e12] * other[e4]), -(anti_wedge[e31] * other[e2]) - (anti_wedge[e12] * other[e3])) - (other.group0().yzxx * float4(anti_wedge.group0().zxy, anti_wedge[e23]))
        );
    }
}
extension Horizon: AntiProjectOrthogonallyOnto<Scalar> {
    func anti_project_orthogonally_onto(self, other: Scalar) -> Horizon {
        return Horizon::from_groups(
            /* e321 */
            powi(other[scalar], 2) * self[e321]
        );
    }
}
extension Line {
    func div(_rhs: anti_project_orthogonally_onto) -> anti_project_orthogonally_onto_partial<Line> {
        anti_project_orthogonally_onto_partial(this)
    }
}
extension Line: AntiProjectOrthogonallyOnto<DualNum> {
    func anti_project_orthogonally_onto(self, other: DualNum) -> Line {
AntiScalar right_anti_dual = AntiScalar::from_groups(
            /* e1234 */
            other[scalar]
        );
Line anti_wedge = Line::from_groups(
            /* e41, e42, e43 */
            float3(right_anti_dual[e1234]) * self.group0(), 
            /* e23, e31, e12 */
            float3(right_anti_dual[e1234]) * self.group1()
        );
        return Line::from_groups(
            /* e41, e42, e43 */
            float3(other[scalar]) * anti_wedge.group0(), 
            /* e23, e31, e12 */
            float3(other[scalar]) * anti_wedge.group1()
        );
    }
}
extension Line: AntiProjectOrthogonallyOnto<Flector> {
    func anti_project_orthogonally_onto(self, other: Flector) -> Motor {
Flector right_anti_dual = Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other[e321] * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other[e1], other[e2], other[e3], 0.0)
        );
Point anti_wedge = Point::from_groups(
            /* e1, e2, e3, e4 */
            float4((right_anti_dual[e412] * self[e31]) + (right_anti_dual[e321] * self[e41]), (right_anti_dual[e423] * self[e12]) + (right_anti_dual[e321] * self[e42]), (right_anti_dual[e431] * self[e23]) + (right_anti_dual[e321] * self[e43]), -(right_anti_dual[e431] * self[e42]) - (right_anti_dual[e412] * self[e43])) - (right_anti_dual.group1().yzxx * float4(self.group1().zxy, self[e41]))
        );
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(other[e4] * anti_wedge[e1], other[e4] * anti_wedge[e2], other[e4] * anti_wedge[e3], -(other[e431] * anti_wedge[e2]) - (other[e412] * anti_wedge[e3]) - (other[e321] * anti_wedge[e4])) - (anti_wedge.group0().wwwx * float4(other.group0().xyz, other[e423])), 
            /* e23, e31, e12, scalar */
            float4(((other.group0().yzx * anti_wedge.group0().zxy) - (other.group0().zxy * anti_wedge.group0().yzx)), 0.0)
        );
    }
}
extension Line: AntiProjectOrthogonallyOnto<Line> {
    func anti_project_orthogonally_onto(self, other: Line) -> Line {
Line right_anti_dual = Line::from_groups(
            /* e41, e42, e43 */
            other.group1() * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
Scalar anti_wedge = Scalar::from_groups(
            /* scalar */
            -(right_anti_dual[e41] * self[e23]) - (right_anti_dual[e42] * self[e31]) - (right_anti_dual[e43] * self[e12]) - (right_anti_dual[e23] * self[e41]) - (right_anti_dual[e31] * self[e42]) - (right_anti_dual[e12] * self[e43])
        );
        return Line::from_groups(
            /* e41, e42, e43 */
            float3(anti_wedge[scalar]) * other.group0(), 
            /* e23, e31, e12 */
            float3(anti_wedge[scalar]) * other.group1()
        );
    }
}
extension Line: AntiProjectOrthogonallyOnto<Motor> {
    func anti_project_orthogonally_onto(self, other: Motor) -> Motor {
Motor right_anti_dual = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            other.group1() * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
Motor anti_wedge = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(float3(1.0), 0.0) * float4(self.group0(), 0.0) * float4(right_anti_dual.group0().www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e23, e31, e12, scalar */
            float4(right_anti_dual[e1234], right_anti_dual[e1234], right_anti_dual[e1234], 1.0) * float4(self.group1(), -(self[e41] * right_anti_dual[e23]) - (self[e42] * right_anti_dual[e31]) - (self[e43] * right_anti_dual[e12]) - (self[e23] * right_anti_dual[e41]) - (self[e31] * right_anti_dual[e42]) - (self[e12] * right_anti_dual[e43]))
        );
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            (float4(anti_wedge[scalar]) * other.group0()) + (float4(other[scalar]) * anti_wedge.group0()) + float4(float3(0.0), -(anti_wedge[e41] * other[e23]) - (anti_wedge[e42] * other[e31]) - (anti_wedge[e43] * other[e12]) - (anti_wedge[e23] * other[e41]) - (anti_wedge[e31] * other[e42]) - (anti_wedge[e12] * other[e43])), 
            /* e23, e31, e12, scalar */
            float4(((float3(anti_wedge[scalar]) * other.group1().xyz) + (float3(other[scalar]) * anti_wedge.group1().xyz)), anti_wedge[scalar] * other[scalar])
        );
    }
}
extension Line: AntiProjectOrthogonallyOnto<MultiVector> {
    func anti_project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
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
MultiVector anti_wedge = MultiVector::from_groups(
            /* scalar, e1234 */
            float2(-(self[e41] * right_anti_dual[e23]) - (self[e42] * right_anti_dual[e31]) - (self[e43] * right_anti_dual[e12]) - (self[e23] * right_anti_dual[e41]) - (self[e31] * right_anti_dual[e42]) - (self[e12] * right_anti_dual[e43]), 0.0), 
            /* e1, e2, e3, e4 */
            float4((self[e41] * right_anti_dual[e321]) + (self[e31] * right_anti_dual[e412]), (self[e42] * right_anti_dual[e321]) + (self[e12] * right_anti_dual[e423]), (self[e43] * right_anti_dual[e321]) + (self[e23] * right_anti_dual[e431]), -(self[e42] * right_anti_dual[e431]) - (self[e43] * right_anti_dual[e412])) - (right_anti_dual.group4().yzxx * float4(self.group1().zxy, self[e41])), 
            /* e41, e42, e43 */
            float3(right_anti_dual[e1234]) * self.group0(), 
            /* e23, e31, e12 */
            float3(right_anti_dual[e1234]) * self.group1(), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(anti_wedge[scalar] * other[scalar], (anti_wedge[scalar] * other[e1234]) + (anti_wedge[e1234] * other[scalar]) + (anti_wedge[e423] * other[e1]) + (anti_wedge[e431] * other[e2]) + (anti_wedge[e412] * other[e3]) + (anti_wedge[e321] * other[e4]) - (anti_wedge[e1] * other[e423]) - (anti_wedge[e2] * other[e431]) - (anti_wedge[e3] * other[e412]) - (anti_wedge[e4] * other[e321]) - (anti_wedge[e41] * other[e23]) - (anti_wedge[e42] * other[e31]) - (anti_wedge[e43] * other[e12]) - (anti_wedge[e23] * other[e41]) - (anti_wedge[e31] * other[e42]) - (anti_wedge[e12] * other[e43])), 
            /* e1, e2, e3, e4 */
            (float4(anti_wedge[scalar]) * other.group1()) + (float4(other[scalar]) * anti_wedge.group1()), 
            /* e41, e42, e43 */
            (float3(anti_wedge[scalar]) * other.group2()) + (float3(other[scalar]) * anti_wedge.group2()) + (float3(other[e4]) * anti_wedge.group1().xyz) - (float3(anti_wedge[e4]) * other.group1().xyz), 
            /* e23, e31, e12 */
            (float3(anti_wedge[scalar]) * other.group3()) + (float3(other[scalar]) * anti_wedge.group3()) + (anti_wedge.group1().zxy * other.group1().yzx) - (anti_wedge.group1().yzx * other.group1().zxy), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge[e3] * other[e42]) + (anti_wedge[e4] * other[e23]) + (anti_wedge[e42] * other[e3]) + (anti_wedge[e23] * other[e4]), (anti_wedge[e1] * other[e43]) + (anti_wedge[e4] * other[e31]) + (anti_wedge[e43] * other[e1]) + (anti_wedge[e31] * other[e4]), (anti_wedge[e2] * other[e41]) + (anti_wedge[e4] * other[e12]) + (anti_wedge[e41] * other[e2]) + (anti_wedge[e12] * other[e4]), -(anti_wedge[e1] * other[e23]) - (anti_wedge[e2] * other[e31]) - (anti_wedge[e3] * other[e12]) - (anti_wedge[e12] * other[e3])) + (float4(anti_wedge[scalar]) * other.group4()) + (float4(other[scalar]) * anti_wedge.group4()) - (other.group1().yzxx * float4(anti_wedge.group2().zxy, anti_wedge[e23])) - float4((other.group2().zxy * anti_wedge.group1().yzx), anti_wedge[e31] * other[e2])
        );
    }
}
extension Line: AntiProjectOrthogonallyOnto<Point> {
    func anti_project_orthogonally_onto(self, other: Point) -> Line {
Plane right_anti_dual = Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4(other[e1], other[e2], other[e3], 0.0)
        );
Point anti_wedge = Point::from_groups(
            /* e1, e2, e3, e4 */
            float4((self[e41] * right_anti_dual[e321]) + (self[e31] * right_anti_dual[e412]), (self[e42] * right_anti_dual[e321]) + (self[e12] * right_anti_dual[e423]), (self[e43] * right_anti_dual[e321]) + (self[e23] * right_anti_dual[e431]), -(self[e42] * right_anti_dual[e431]) - (self[e43] * right_anti_dual[e412])) - (right_anti_dual.group0().yzxx * float4(self.group1().zxy, self[e41]))
        );
        return Line::from_groups(
            /* e41, e42, e43 */
            (float3(other[e4]) * anti_wedge.group0().xyz) - (float3(anti_wedge[e4]) * other.group0().xyz), 
            /* e23, e31, e12 */
            (anti_wedge.group0().zxy * other.group0().yzx) - (anti_wedge.group0().yzx * other.group0().zxy)
        );
    }
}
extension Line: AntiProjectOrthogonallyOnto<Scalar> {
    func anti_project_orthogonally_onto(self, other: Scalar) -> Line {
AntiScalar right_anti_dual = AntiScalar::from_groups(
            /* e1234 */
            other[scalar]
        );
Line anti_wedge = Line::from_groups(
            /* e41, e42, e43 */
            float3(right_anti_dual[e1234]) * self.group0(), 
            /* e23, e31, e12 */
            float3(right_anti_dual[e1234]) * self.group1()
        );
        return Line::from_groups(
            /* e41, e42, e43 */
            float3(other[scalar]) * anti_wedge.group0(), 
            /* e23, e31, e12 */
            float3(other[scalar]) * anti_wedge.group1()
        );
    }
}
extension Motor {
    func div(_rhs: anti_project_orthogonally_onto) -> anti_project_orthogonally_onto_partial<Motor> {
        anti_project_orthogonally_onto_partial(this)
    }
}
extension Motor: AntiProjectOrthogonallyOnto<DualNum> {
    func anti_project_orthogonally_onto(self, other: DualNum) -> Motor {
AntiScalar right_anti_dual = AntiScalar::from_groups(
            /* e1234 */
            other[scalar]
        );
Motor anti_wedge = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(right_anti_dual[e1234]) * self.group0(), 
            /* e23, e31, e12, scalar */
            float4(right_anti_dual[e1234]) * self.group1()
        );
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(anti_wedge[e41], anti_wedge[e42], anti_wedge[e43], 1.0) * float4(other.group0().xx, other[scalar], (other[scalar] * anti_wedge[e1234]) + (other[e1234] * anti_wedge[scalar])), 
            /* e23, e31, e12, scalar */
            float4(other[scalar]) * anti_wedge.group1()
        );
    }
}
extension Motor: AntiProjectOrthogonallyOnto<Flector> {
    func anti_project_orthogonally_onto(self, other: Flector) -> Motor {
Flector right_anti_dual = Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other[e321] * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other[e1], other[e2], other[e3], 0.0)
        );
Flector anti_wedge = Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4((right_anti_dual[e412] * self[e31]) + (right_anti_dual[e321] * self[e41]), (right_anti_dual[e423] * self[e12]) + (right_anti_dual[e321] * self[e42]), (right_anti_dual[e431] * self[e23]) + (right_anti_dual[e321] * self[e43]), -(right_anti_dual[e431] * self[e42]) - (right_anti_dual[e412] * self[e43])) + (float4(self[e1234]) * right_anti_dual.group0()) - (right_anti_dual.group1().yzxx * float4(self.group1().zxy, self[e41])), 
            /* e423, e431, e412, e321 */
            float4(self[e1234]) * right_anti_dual.group1()
        );
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            (other.group0().wwwx * float4(anti_wedge.group0().xyz, anti_wedge[e423])) + float4(float3(0.0), (anti_wedge[e431] * other[e2]) + (anti_wedge[e412] * other[e3]) + (anti_wedge[e321] * other[e4]) - (anti_wedge[e2] * other[e431]) - (anti_wedge[e3] * other[e412]) - (anti_wedge[e4] * other[e321])) - (anti_wedge.group0().wwwx * float4(other.group0().xyz, other[e423])), 
            /* e23, e31, e12, scalar */
            float4(((anti_wedge.group0().zxy * other.group0().yzx) - (anti_wedge.group0().yzx * other.group0().zxy)), 0.0)
        );
    }
}
extension Motor: AntiProjectOrthogonallyOnto<Horizon> {
    func anti_project_orthogonally_onto(self, other: Horizon) -> AntiScalar {
        return AntiScalar::from_groups(
            /* e1234 */
            powi(other[e321], 2) * self[e1234]
        );
    }
}
extension Motor: AntiProjectOrthogonallyOnto<Line> {
    func anti_project_orthogonally_onto(self, other: Line) -> Motor {
Line right_anti_dual = Line::from_groups(
            /* e41, e42, e43 */
            other.group1() * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
Motor anti_wedge = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(float3(1.0), 0.0) * float4(right_anti_dual.group0(), 0.0) * float4(self.group0().www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e23, e31, e12, scalar */
            float4(self[e1234], self[e1234], self[e1234], 1.0) * float4(right_anti_dual.group1(), -(right_anti_dual[e41] * self[e23]) - (right_anti_dual[e42] * self[e31]) - (right_anti_dual[e43] * self[e12]) - (right_anti_dual[e23] * self[e41]) - (right_anti_dual[e31] * self[e42]) - (right_anti_dual[e12] * self[e43]))
        );
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(anti_wedge[scalar], anti_wedge[scalar], anti_wedge[scalar], 1.0) * float4(other.group0(), -(other[e41] * anti_wedge[e23]) - (other[e42] * anti_wedge[e31]) - (other[e43] * anti_wedge[e12]) - (other[e23] * anti_wedge[e41]) - (other[e31] * anti_wedge[e42]) - (other[e12] * anti_wedge[e43])), 
            /* e23, e31, e12, scalar */
            float4(float3(1.0), 0.0) * float4(other.group1(), 0.0) * float4(anti_wedge.group1().www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Motor: AntiProjectOrthogonallyOnto<Motor> {
    func anti_project_orthogonally_onto(self, other: Motor) -> Motor {
Motor right_anti_dual = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            other.group1() * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
Motor anti_wedge = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(((float3(right_anti_dual[e1234]) * self.group0().xyz) + (float3(self[e1234]) * right_anti_dual.group0().xyz)), right_anti_dual[e1234] * self[e1234]), 
            /* e23, e31, e12, scalar */
            (float4(right_anti_dual[e1234]) * self.group1()) + (float4(self[e1234]) * right_anti_dual.group1()) + float4(float3(0.0), -(right_anti_dual[e41] * self[e23]) - (right_anti_dual[e42] * self[e31]) - (right_anti_dual[e43] * self[e12]) - (right_anti_dual[e23] * self[e41]) - (right_anti_dual[e31] * self[e42]) - (right_anti_dual[e12] * self[e43]))
        );
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            (float4(anti_wedge[scalar]) * other.group0()) + (float4(other[scalar]) * anti_wedge.group0()) + float4(float3(0.0), -(anti_wedge[e41] * other[e23]) - (anti_wedge[e42] * other[e31]) - (anti_wedge[e43] * other[e12]) - (anti_wedge[e23] * other[e41]) - (anti_wedge[e31] * other[e42]) - (anti_wedge[e12] * other[e43])), 
            /* e23, e31, e12, scalar */
            float4(((float3(anti_wedge[scalar]) * other.group1().xyz) + (float3(other[scalar]) * anti_wedge.group1().xyz)), anti_wedge[scalar] * other[scalar])
        );
    }
}
extension Motor: AntiProjectOrthogonallyOnto<MultiVector> {
    func anti_project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
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
MultiVector anti_wedge = MultiVector::from_groups(
            /* scalar, e1234 */
            float2((self[e1234] * right_anti_dual[scalar]) + (self[scalar] * right_anti_dual[e1234]) - (self[e41] * right_anti_dual[e23]) - (self[e42] * right_anti_dual[e31]) - (self[e43] * right_anti_dual[e12]) - (self[e23] * right_anti_dual[e41]) - (self[e31] * right_anti_dual[e42]) - (self[e12] * right_anti_dual[e43]), self[e1234] * right_anti_dual[e1234]), 
            /* e1, e2, e3, e4 */
            float4((self[e1234] * right_anti_dual[e1]) + (self[e31] * right_anti_dual[e412]), (self[e1234] * right_anti_dual[e2]) + (self[e12] * right_anti_dual[e423]), (self[e1234] * right_anti_dual[e3]) + (self[e23] * right_anti_dual[e431]), -(self[e42] * right_anti_dual[e431]) - (self[e43] * right_anti_dual[e412])) + (self.group0() * float4(right_anti_dual.group4().www, right_anti_dual[e4])) - (right_anti_dual.group4().yzxx * float4(self.group1().zxy, self[e41])), 
            /* e41, e42, e43 */
            (float3(self[e1234]) * right_anti_dual.group2()) + (float3(right_anti_dual[e1234]) * self.group0().xyz), 
            /* e23, e31, e12 */
            (float3(self[e1234]) * right_anti_dual.group3()) + (float3(right_anti_dual[e1234]) * self.group1().xyz), 
            /* e423, e431, e412, e321 */
            float4(self[e1234]) * right_anti_dual.group4()
        );
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(anti_wedge[scalar] * other[scalar], (anti_wedge[scalar] * other[e1234]) + (anti_wedge[e1234] * other[scalar]) + (anti_wedge[e423] * other[e1]) + (anti_wedge[e431] * other[e2]) + (anti_wedge[e412] * other[e3]) + (anti_wedge[e321] * other[e4]) - (anti_wedge[e1] * other[e423]) - (anti_wedge[e2] * other[e431]) - (anti_wedge[e3] * other[e412]) - (anti_wedge[e4] * other[e321]) - (anti_wedge[e41] * other[e23]) - (anti_wedge[e42] * other[e31]) - (anti_wedge[e43] * other[e12]) - (anti_wedge[e23] * other[e41]) - (anti_wedge[e31] * other[e42]) - (anti_wedge[e12] * other[e43])), 
            /* e1, e2, e3, e4 */
            (float4(anti_wedge[scalar]) * other.group1()) + (float4(other[scalar]) * anti_wedge.group1()), 
            /* e41, e42, e43 */
            (float3(anti_wedge[scalar]) * other.group2()) + (float3(other[scalar]) * anti_wedge.group2()) + (float3(other[e4]) * anti_wedge.group1().xyz) - (float3(anti_wedge[e4]) * other.group1().xyz), 
            /* e23, e31, e12 */
            (float3(anti_wedge[scalar]) * other.group3()) + (float3(other[scalar]) * anti_wedge.group3()) + (anti_wedge.group1().zxy * other.group1().yzx) - (anti_wedge.group1().yzx * other.group1().zxy), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge[e3] * other[e42]) + (anti_wedge[e4] * other[e23]) + (anti_wedge[e42] * other[e3]) + (anti_wedge[e23] * other[e4]), (anti_wedge[e1] * other[e43]) + (anti_wedge[e4] * other[e31]) + (anti_wedge[e43] * other[e1]) + (anti_wedge[e31] * other[e4]), (anti_wedge[e2] * other[e41]) + (anti_wedge[e4] * other[e12]) + (anti_wedge[e41] * other[e2]) + (anti_wedge[e12] * other[e4]), -(anti_wedge[e1] * other[e23]) - (anti_wedge[e2] * other[e31]) - (anti_wedge[e3] * other[e12]) - (anti_wedge[e12] * other[e3])) + (float4(anti_wedge[scalar]) * other.group4()) + (float4(other[scalar]) * anti_wedge.group4()) - (other.group1().yzxx * float4(anti_wedge.group2().zxy, anti_wedge[e23])) - float4((other.group2().zxy * anti_wedge.group1().yzx), anti_wedge[e31] * other[e2])
        );
    }
}
extension Motor: AntiProjectOrthogonallyOnto<Plane> {
    func anti_project_orthogonally_onto(self, other: Plane) -> AntiScalar {
        return AntiScalar::from_groups(
            /* e1234 */
            powi(other[e321], 2) * self[e1234]
        );
    }
}
extension Motor: AntiProjectOrthogonallyOnto<Point> {
    func anti_project_orthogonally_onto(self, other: Point) -> Motor {
Plane right_anti_dual = Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4(other[e1], other[e2], other[e3], 0.0)
        );
Flector anti_wedge = Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4((self[e41] * right_anti_dual[e321]) + (self[e31] * right_anti_dual[e412]), (self[e42] * right_anti_dual[e321]) + (self[e12] * right_anti_dual[e423]), (self[e43] * right_anti_dual[e321]) + (self[e23] * right_anti_dual[e431]), -(self[e42] * right_anti_dual[e431]) - (self[e43] * right_anti_dual[e412])) - (right_anti_dual.group0().yzxx * float4(self.group1().zxy, self[e41])), 
            /* e423, e431, e412, e321 */
            float4(self[e1234]) * right_anti_dual.group0()
        );
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(anti_wedge[e4] * other[e1] * -1.0, anti_wedge[e4] * other[e2] * -1.0, anti_wedge[e4] * other[e3] * -1.0, (anti_wedge[e431] * other[e2]) + (anti_wedge[e412] * other[e3]) + (anti_wedge[e321] * other[e4])) + (other.group0().wwwx * float4(anti_wedge.group0().xyz, anti_wedge[e423])), 
            /* e23, e31, e12, scalar */
            float4(((anti_wedge.group0().zxy * other.group0().yzx) - (anti_wedge.group0().yzx * other.group0().zxy)), 0.0)
        );
    }
}
extension Motor: AntiProjectOrthogonallyOnto<Scalar> {
    func anti_project_orthogonally_onto(self, other: Scalar) -> Motor {
AntiScalar right_anti_dual = AntiScalar::from_groups(
            /* e1234 */
            other[scalar]
        );
Motor anti_wedge = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(right_anti_dual[e1234]) * self.group0(), 
            /* e23, e31, e12, scalar */
            float4(right_anti_dual[e1234]) * self.group1()
        );
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(other[scalar]) * anti_wedge.group0(), 
            /* e23, e31, e12, scalar */
            float4(other[scalar]) * anti_wedge.group1()
        );
    }
}
extension MultiVector {
    func div(_rhs: anti_project_orthogonally_onto) -> anti_project_orthogonally_onto_partial<MultiVector> {
        anti_project_orthogonally_onto_partial(this)
    }
}
extension MultiVector: AntiProjectOrthogonallyOnto<DualNum> {
    func anti_project_orthogonally_onto(self, other: DualNum) -> MultiVector {
AntiScalar right_anti_dual = AntiScalar::from_groups(
            /* e1234 */
            other[scalar]
        );
MultiVector anti_wedge = MultiVector::from_groups(
            /* scalar, e1234 */
            float2(right_anti_dual[e1234]) * self.group0(), 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual[e1234]) * self.group1(), 
            /* e41, e42, e43 */
            float3(right_anti_dual[e1234]) * self.group2(), 
            /* e23, e31, e12 */
            float3(right_anti_dual[e1234]) * self.group3(), 
            /* e423, e431, e412, e321 */
            float4(right_anti_dual[e1234]) * self.group4()
        );
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(other[scalar] * anti_wedge[scalar], (other[scalar] * anti_wedge[e1234]) + (other[e1234] * anti_wedge[scalar])), 
            /* e1, e2, e3, e4 */
            float4(other[scalar]) * anti_wedge.group1(), 
            /* e41, e42, e43 */
            float3(other[scalar]) * anti_wedge.group2(), 
            /* e23, e31, e12 */
            float3(other[scalar]) * anti_wedge.group3(), 
            /* e423, e431, e412, e321 */
            float4(other[scalar]) * anti_wedge.group4()
        );
    }
}
extension MultiVector: AntiProjectOrthogonallyOnto<Flector> {
    func anti_project_orthogonally_onto(self, other: Flector) -> MultiVector {
Flector right_anti_dual = Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other[e321] * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other[e1], other[e2], other[e3], 0.0)
        );
MultiVector anti_wedge = MultiVector::from_groups(
            /* scalar, e1234 */
            float2((right_anti_dual[e423] * self[e1]) + (right_anti_dual[e431] * self[e2]) + (right_anti_dual[e412] * self[e3]) + (right_anti_dual[e321] * self[e4]) - (right_anti_dual[e1] * self[e423]) - (right_anti_dual[e2] * self[e431]) - (right_anti_dual[e3] * self[e412]) - (right_anti_dual[e4] * self[e321]), 0.0), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual[e412] * self[e31]) + (right_anti_dual[e321] * self[e41]), (right_anti_dual[e423] * self[e12]) + (right_anti_dual[e321] * self[e42]), (right_anti_dual[e431] * self[e23]) + (right_anti_dual[e321] * self[e43]), -(right_anti_dual[e431] * self[e42]) - (right_anti_dual[e412] * self[e43])) + (float4(self[e1234]) * right_anti_dual.group0()) - (right_anti_dual.group1().yzxx * float4(self.group3().zxy, self[e41])), 
            /* e41, e42, e43 */
            (right_anti_dual.group1().yzx * self.group4().zxy) - (right_anti_dual.group1().zxy * self.group4().yzx), 
            /* e23, e31, e12 */
            (float3(right_anti_dual[e321]) * self.group4().xyz) - (float3(self[e321]) * right_anti_dual.group1().xyz), 
            /* e423, e431, e412, e321 */
            float4(self[e1234]) * right_anti_dual.group1()
        );
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(0.0, (other[e1] * anti_wedge[e423]) + (other[e2] * anti_wedge[e431]) + (other[e3] * anti_wedge[e412]) + (other[e4] * anti_wedge[e321]) - (other[e423] * anti_wedge[e1]) - (other[e431] * anti_wedge[e2]) - (other[e412] * anti_wedge[e3]) - (other[e321] * anti_wedge[e4])), 
            /* e1, e2, e3, e4 */
            float4(anti_wedge[scalar]) * other.group0(), 
            /* e41, e42, e43 */
            (float3(other[e4]) * anti_wedge.group1().xyz) - (float3(anti_wedge[e4]) * other.group0().xyz), 
            /* e23, e31, e12 */
            (other.group0().yzx * anti_wedge.group1().zxy) - (other.group0().zxy * anti_wedge.group1().yzx), 
            /* e423, e431, e412, e321 */
            float4((other[e3] * anti_wedge[e42]) + (other[e4] * anti_wedge[e23]), (other[e1] * anti_wedge[e43]) + (other[e4] * anti_wedge[e31]), (other[e2] * anti_wedge[e41]) + (other[e4] * anti_wedge[e12]), -(other[e2] * anti_wedge[e31]) - (other[e3] * anti_wedge[e12])) + (float4(anti_wedge[scalar]) * other.group1()) - (other.group0().yzxx * float4(anti_wedge.group2().zxy, anti_wedge[e23]))
        );
    }
}
extension MultiVector: AntiProjectOrthogonallyOnto<Horizon> {
    func anti_project_orthogonally_onto(self, other: Horizon) -> MultiVector {
Origin right_anti_dual = Origin::from_groups(
            /* e4 */
            other[e321] * -1.0
        );
MultiVector anti_wedge = MultiVector::from_groups(
            /* scalar, e1234 */
            float2(self[e321] * right_anti_dual[e4], 1.0) * float2(-1.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), self[e1234] * right_anti_dual[e4]), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(1.0, other[e321] * anti_wedge[e4]) * float2(0.0, -1.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other[e321] * anti_wedge[scalar])
        );
    }
}
extension MultiVector: AntiProjectOrthogonallyOnto<Line> {
    func anti_project_orthogonally_onto(self, other: Line) -> MultiVector {
Line right_anti_dual = Line::from_groups(
            /* e41, e42, e43 */
            other.group1() * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
MultiVector anti_wedge = MultiVector::from_groups(
            /* scalar, e1234 */
            float2(-(right_anti_dual[e41] * self[e23]) - (right_anti_dual[e42] * self[e31]) - (right_anti_dual[e43] * self[e12]) - (right_anti_dual[e23] * self[e41]) - (right_anti_dual[e31] * self[e42]) - (right_anti_dual[e12] * self[e43]), 0.0), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual[e41] * self[e321]) + (right_anti_dual[e31] * self[e412]), (right_anti_dual[e42] * self[e321]) + (right_anti_dual[e12] * self[e423]), (right_anti_dual[e43] * self[e321]) + (right_anti_dual[e23] * self[e431]), -(right_anti_dual[e42] * self[e431]) - (right_anti_dual[e43] * self[e412])) - (self.group4().yzxx * float4(right_anti_dual.group1().zxy, right_anti_dual[e41])), 
            /* e41, e42, e43 */
            float3(self[e1234]) * right_anti_dual.group0(), 
            /* e23, e31, e12 */
            float3(self[e1234]) * right_anti_dual.group1(), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(0.0, -(other[e41] * anti_wedge[e23]) - (other[e42] * anti_wedge[e31]) - (other[e43] * anti_wedge[e12]) - (other[e23] * anti_wedge[e41]) - (other[e31] * anti_wedge[e42]) - (other[e12] * anti_wedge[e43])), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(anti_wedge[scalar]) * other.group0(), 
            /* e23, e31, e12 */
            float3(anti_wedge[scalar]) * other.group1(), 
            /* e423, e431, e412, e321 */
            float4((other[e42] * anti_wedge[e3]) + (other[e23] * anti_wedge[e4]), (other[e43] * anti_wedge[e1]) + (other[e31] * anti_wedge[e4]), (other[e41] * anti_wedge[e2]) + (other[e12] * anti_wedge[e4]), -(other[e31] * anti_wedge[e2]) - (other[e12] * anti_wedge[e3])) - (anti_wedge.group1().yzxx * float4(other.group0().zxy, other[e23]))
        );
    }
}
extension MultiVector: AntiProjectOrthogonallyOnto<Motor> {
    func anti_project_orthogonally_onto(self, other: Motor) -> MultiVector {
Motor right_anti_dual = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            other.group1() * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
MultiVector anti_wedge = MultiVector::from_groups(
            /* scalar, e1234 */
            float2((right_anti_dual[e1234] * self[scalar]) + (right_anti_dual[scalar] * self[e1234]) - (right_anti_dual[e41] * self[e23]) - (right_anti_dual[e42] * self[e31]) - (right_anti_dual[e43] * self[e12]) - (right_anti_dual[e23] * self[e41]) - (right_anti_dual[e31] * self[e42]) - (right_anti_dual[e12] * self[e43]), right_anti_dual[e1234] * self[e1234]), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual[e1234] * self[e1]) + (right_anti_dual[e31] * self[e412]), (right_anti_dual[e1234] * self[e2]) + (right_anti_dual[e12] * self[e423]), (right_anti_dual[e1234] * self[e3]) + (right_anti_dual[e23] * self[e431]), -(right_anti_dual[e42] * self[e431]) - (right_anti_dual[e43] * self[e412])) + (right_anti_dual.group0() * float4(self.group4().www, self[e4])) - (self.group4().yzxx * float4(right_anti_dual.group1().zxy, right_anti_dual[e41])), 
            /* e41, e42, e43 */
            (float3(right_anti_dual[e1234]) * self.group2()) + (float3(self[e1234]) * right_anti_dual.group0().xyz), 
            /* e23, e31, e12 */
            (float3(right_anti_dual[e1234]) * self.group3()) + (float3(self[e1234]) * right_anti_dual.group1().xyz), 
            /* e423, e431, e412, e321 */
            float4(right_anti_dual[e1234]) * self.group4()
        );
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(other[scalar] * anti_wedge[scalar], (other[e1234] * anti_wedge[scalar]) + (other[scalar] * anti_wedge[e1234]) - (other[e41] * anti_wedge[e23]) - (other[e42] * anti_wedge[e31]) - (other[e43] * anti_wedge[e12]) - (other[e23] * anti_wedge[e41]) - (other[e31] * anti_wedge[e42]) - (other[e12] * anti_wedge[e43])), 
            /* e1, e2, e3, e4 */
            float4(other[scalar]) * anti_wedge.group1(), 
            /* e41, e42, e43 */
            (float3(other[scalar]) * anti_wedge.group2()) + (float3(anti_wedge[scalar]) * other.group0().xyz), 
            /* e23, e31, e12 */
            (float3(other[scalar]) * anti_wedge.group3()) + (float3(anti_wedge[scalar]) * other.group1().xyz), 
            /* e423, e431, e412, e321 */
            float4((other[e23] * anti_wedge[e4]) + (other[scalar] * anti_wedge[e423]), (other[e31] * anti_wedge[e4]) + (other[scalar] * anti_wedge[e431]), (other[e12] * anti_wedge[e4]) + (other[scalar] * anti_wedge[e412]), -(other[e31] * anti_wedge[e2]) - (other[e12] * anti_wedge[e3])) + float4((other.group0().yzx * anti_wedge.group1().zxy), other[scalar] * anti_wedge[e321]) - (anti_wedge.group1().yzxx * float4(other.group0().zxy, other[e23]))
        );
    }
}
extension MultiVector: AntiProjectOrthogonallyOnto<MultiVector> {
    func anti_project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
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
MultiVector anti_wedge = MultiVector::from_groups(
            /* scalar, e1234 */
            float2((right_anti_dual[scalar] * self[e1234]) + (right_anti_dual[e1234] * self[scalar]) + (right_anti_dual[e423] * self[e1]) + (right_anti_dual[e431] * self[e2]) + (right_anti_dual[e412] * self[e3]) + (right_anti_dual[e321] * self[e4]) - (right_anti_dual[e1] * self[e423]) - (right_anti_dual[e2] * self[e431]) - (right_anti_dual[e3] * self[e412]) - (right_anti_dual[e4] * self[e321]) - (right_anti_dual[e41] * self[e23]) - (right_anti_dual[e42] * self[e31]) - (right_anti_dual[e43] * self[e12]) - (right_anti_dual[e23] * self[e41]) - (right_anti_dual[e31] * self[e42]) - (right_anti_dual[e12] * self[e43]), right_anti_dual[e1234] * self[e1234]), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual[e41] * self[e321]) + (right_anti_dual[e31] * self[e412]) + (right_anti_dual[e412] * self[e31]) + (right_anti_dual[e321] * self[e41]), (right_anti_dual[e42] * self[e321]) + (right_anti_dual[e12] * self[e423]) + (right_anti_dual[e423] * self[e12]) + (right_anti_dual[e321] * self[e42]), (right_anti_dual[e43] * self[e321]) + (right_anti_dual[e23] * self[e431]) + (right_anti_dual[e431] * self[e23]) + (right_anti_dual[e321] * self[e43]), -(right_anti_dual[e43] * self[e412]) - (right_anti_dual[e423] * self[e41]) - (right_anti_dual[e431] * self[e42]) - (right_anti_dual[e412] * self[e43])) + (float4(right_anti_dual[e1234]) * self.group1()) + (float4(self[e1234]) * right_anti_dual.group1()) - (self.group4().yzxx * float4(right_anti_dual.group3().zxy, right_anti_dual[e41])) - float4((self.group3().zxy * right_anti_dual.group4().yzx), right_anti_dual[e42] * self[e431]), 
            /* e41, e42, e43 */
            (float3(right_anti_dual[e1234]) * self.group2()) + (float3(self[e1234]) * right_anti_dual.group2()) + (right_anti_dual.group4().yzx * self.group4().zxy) - (right_anti_dual.group4().zxy * self.group4().yzx), 
            /* e23, e31, e12 */
            (float3(right_anti_dual[e1234]) * self.group3()) + (float3(right_anti_dual[e321]) * self.group4().xyz) + (float3(self[e1234]) * right_anti_dual.group3()) - (float3(self[e321]) * right_anti_dual.group4().xyz), 
            /* e423, e431, e412, e321 */
            (float4(right_anti_dual[e1234]) * self.group4()) + (float4(self[e1234]) * right_anti_dual.group4())
        );
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(anti_wedge[scalar] * other[scalar], (anti_wedge[scalar] * other[e1234]) + (anti_wedge[e1234] * other[scalar]) + (anti_wedge[e423] * other[e1]) + (anti_wedge[e431] * other[e2]) + (anti_wedge[e412] * other[e3]) + (anti_wedge[e321] * other[e4]) - (anti_wedge[e1] * other[e423]) - (anti_wedge[e2] * other[e431]) - (anti_wedge[e3] * other[e412]) - (anti_wedge[e4] * other[e321]) - (anti_wedge[e41] * other[e23]) - (anti_wedge[e42] * other[e31]) - (anti_wedge[e43] * other[e12]) - (anti_wedge[e23] * other[e41]) - (anti_wedge[e31] * other[e42]) - (anti_wedge[e12] * other[e43])), 
            /* e1, e2, e3, e4 */
            (float4(anti_wedge[scalar]) * other.group1()) + (float4(other[scalar]) * anti_wedge.group1()), 
            /* e41, e42, e43 */
            (float3(anti_wedge[scalar]) * other.group2()) + (float3(other[scalar]) * anti_wedge.group2()) + (float3(other[e4]) * anti_wedge.group1().xyz) - (float3(anti_wedge[e4]) * other.group1().xyz), 
            /* e23, e31, e12 */
            (float3(anti_wedge[scalar]) * other.group3()) + (float3(other[scalar]) * anti_wedge.group3()) + (anti_wedge.group1().zxy * other.group1().yzx) - (anti_wedge.group1().yzx * other.group1().zxy), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge[e3] * other[e42]) + (anti_wedge[e4] * other[e23]) + (anti_wedge[e42] * other[e3]) + (anti_wedge[e23] * other[e4]), (anti_wedge[e1] * other[e43]) + (anti_wedge[e4] * other[e31]) + (anti_wedge[e43] * other[e1]) + (anti_wedge[e31] * other[e4]), (anti_wedge[e2] * other[e41]) + (anti_wedge[e4] * other[e12]) + (anti_wedge[e41] * other[e2]) + (anti_wedge[e12] * other[e4]), -(anti_wedge[e1] * other[e23]) - (anti_wedge[e2] * other[e31]) - (anti_wedge[e3] * other[e12]) - (anti_wedge[e12] * other[e3])) + (float4(anti_wedge[scalar]) * other.group4()) + (float4(other[scalar]) * anti_wedge.group4()) - (other.group1().yzxx * float4(anti_wedge.group2().zxy, anti_wedge[e23])) - float4((other.group2().zxy * anti_wedge.group1().yzx), anti_wedge[e31] * other[e2])
        );
    }
}
extension MultiVector: AntiProjectOrthogonallyOnto<Plane> {
    func anti_project_orthogonally_onto(self, other: Plane) -> MultiVector {
Origin right_anti_dual = Origin::from_groups(
            /* e4 */
            other[e321] * -1.0
        );
MultiVector anti_wedge = MultiVector::from_groups(
            /* scalar, e1234 */
            float2(self[e321] * right_anti_dual[e4], 1.0) * float2(-1.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), self[e1234] * right_anti_dual[e4]), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(0.0, -(anti_wedge[e1] * other[e423]) - (anti_wedge[e2] * other[e431]) - (anti_wedge[e3] * other[e412]) - (anti_wedge[e4] * other[e321])), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(anti_wedge[scalar]) * other.group0()
        );
    }
}
extension MultiVector: AntiProjectOrthogonallyOnto<Point> {
    func anti_project_orthogonally_onto(self, other: Point) -> MultiVector {
Plane right_anti_dual = Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4(other[e1], other[e2], other[e3], 0.0)
        );
MultiVector anti_wedge = MultiVector::from_groups(
            /* scalar, e1234 */
            float2((self[e1] * right_anti_dual[e423]) + (self[e2] * right_anti_dual[e431]) + (self[e3] * right_anti_dual[e412]) + (self[e4] * right_anti_dual[e321]), 0.0), 
            /* e1, e2, e3, e4 */
            float4((self[e41] * right_anti_dual[e321]) + (self[e31] * right_anti_dual[e412]), (self[e42] * right_anti_dual[e321]) + (self[e12] * right_anti_dual[e423]), (self[e43] * right_anti_dual[e321]) + (self[e23] * right_anti_dual[e431]), -(self[e42] * right_anti_dual[e431]) - (self[e43] * right_anti_dual[e412])) - (right_anti_dual.group0().yzxx * float4(self.group3().zxy, self[e41])), 
            /* e41, e42, e43 */
            (self.group4().zxy * right_anti_dual.group0().yzx) - (self.group4().yzx * right_anti_dual.group0().zxy), 
            /* e23, e31, e12 */
            (float3(right_anti_dual[e321]) * self.group4().xyz) - (float3(self[e321]) * right_anti_dual.group0().xyz), 
            /* e423, e431, e412, e321 */
            float4(self[e1234]) * right_anti_dual.group0()
        );
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(0.0, (anti_wedge[e423] * other[e1]) + (anti_wedge[e431] * other[e2]) + (anti_wedge[e412] * other[e3]) + (anti_wedge[e321] * other[e4])), 
            /* e1, e2, e3, e4 */
            float4(anti_wedge[scalar]) * other.group0(), 
            /* e41, e42, e43 */
            (float3(other[e4]) * anti_wedge.group1().xyz) - (float3(anti_wedge[e4]) * other.group0().xyz), 
            /* e23, e31, e12 */
            (anti_wedge.group1().zxy * other.group0().yzx) - (anti_wedge.group1().yzx * other.group0().zxy), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge[e42] * other[e3]) + (anti_wedge[e23] * other[e4]), (anti_wedge[e43] * other[e1]) + (anti_wedge[e31] * other[e4]), (anti_wedge[e41] * other[e2]) + (anti_wedge[e12] * other[e4]), -(anti_wedge[e31] * other[e2]) - (anti_wedge[e12] * other[e3])) - (other.group0().yzxx * float4(anti_wedge.group2().zxy, anti_wedge[e23]))
        );
    }
}
extension MultiVector: AntiProjectOrthogonallyOnto<Scalar> {
    func anti_project_orthogonally_onto(self, other: Scalar) -> MultiVector {
AntiScalar right_anti_dual = AntiScalar::from_groups(
            /* e1234 */
            other[scalar]
        );
MultiVector anti_wedge = MultiVector::from_groups(
            /* scalar, e1234 */
            float2(right_anti_dual[e1234]) * self.group0(), 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual[e1234]) * self.group1(), 
            /* e41, e42, e43 */
            float3(right_anti_dual[e1234]) * self.group2(), 
            /* e23, e31, e12 */
            float3(right_anti_dual[e1234]) * self.group3(), 
            /* e423, e431, e412, e321 */
            float4(right_anti_dual[e1234]) * self.group4()
        );
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(other[scalar]) * anti_wedge.group0(), 
            /* e1, e2, e3, e4 */
            float4(other[scalar]) * anti_wedge.group1(), 
            /* e41, e42, e43 */
            float3(other[scalar]) * anti_wedge.group2(), 
            /* e23, e31, e12 */
            float3(other[scalar]) * anti_wedge.group3(), 
            /* e423, e431, e412, e321 */
            float4(other[scalar]) * anti_wedge.group4()
        );
    }
}
extension Origin {
    func div(_rhs: anti_project_orthogonally_onto) -> anti_project_orthogonally_onto_partial<Origin> {
        anti_project_orthogonally_onto_partial(this)
    }
}
extension Origin: AntiProjectOrthogonallyOnto<DualNum> {
    func anti_project_orthogonally_onto(self, other: DualNum) -> Origin {
        return Origin::from_groups(
            /* e4 */
            powi(other[scalar], 2) * self[e4]
        );
    }
}
extension Origin: AntiProjectOrthogonallyOnto<Flector> {
    func anti_project_orthogonally_onto(self, other: Flector) -> Flector {
Scalar anti_wedge = Scalar::from_groups(
            /* scalar */
            self[e4] * 0.0
        );
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(anti_wedge[scalar]) * other.group0(), 
            /* e423, e431, e412, e321 */
            float4(anti_wedge[scalar]) * other.group1()
        );
    }
}
extension Origin: AntiProjectOrthogonallyOnto<Motor> {
    func anti_project_orthogonally_onto(self, other: Motor) -> Flector {
Origin anti_wedge = Origin::from_groups(
            /* e4 */
            (other.group1() * float4(-1.0, -1.0, -1.0, 1.0))[3] * self[e4]
        );
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other[scalar] * anti_wedge[e4]), 
            /* e423, e431, e412, e321 */
            float4(anti_wedge[e4], anti_wedge[e4], anti_wedge[e4], 0.0) * float4(other.group1().xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Origin: AntiProjectOrthogonallyOnto<MultiVector> {
    func anti_project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
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
MultiVector anti_wedge = MultiVector::from_groups(
            /* scalar, e1234 */
            float2(right_anti_dual[e321] * self[e4], 1.0) * float2(1.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), right_anti_dual[e1234] * self[e4]), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(anti_wedge[scalar] * other[scalar], (anti_wedge[scalar] * other[e1234]) + (anti_wedge[e1234] * other[scalar]) + (anti_wedge[e423] * other[e1]) + (anti_wedge[e431] * other[e2]) + (anti_wedge[e412] * other[e3]) + (anti_wedge[e321] * other[e4]) - (anti_wedge[e1] * other[e423]) - (anti_wedge[e2] * other[e431]) - (anti_wedge[e3] * other[e412]) - (anti_wedge[e4] * other[e321]) - (anti_wedge[e41] * other[e23]) - (anti_wedge[e42] * other[e31]) - (anti_wedge[e43] * other[e12]) - (anti_wedge[e23] * other[e41]) - (anti_wedge[e31] * other[e42]) - (anti_wedge[e12] * other[e43])), 
            /* e1, e2, e3, e4 */
            (float4(anti_wedge[scalar]) * other.group1()) + (float4(other[scalar]) * anti_wedge.group1()), 
            /* e41, e42, e43 */
            (float3(anti_wedge[scalar]) * other.group2()) + (float3(other[scalar]) * anti_wedge.group2()) + (float3(other[e4]) * anti_wedge.group1().xyz) - (float3(anti_wedge[e4]) * other.group1().xyz), 
            /* e23, e31, e12 */
            (float3(anti_wedge[scalar]) * other.group3()) + (float3(other[scalar]) * anti_wedge.group3()) + (anti_wedge.group1().zxy * other.group1().yzx) - (anti_wedge.group1().yzx * other.group1().zxy), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge[e3] * other[e42]) + (anti_wedge[e4] * other[e23]) + (anti_wedge[e42] * other[e3]) + (anti_wedge[e23] * other[e4]), (anti_wedge[e1] * other[e43]) + (anti_wedge[e4] * other[e31]) + (anti_wedge[e43] * other[e1]) + (anti_wedge[e31] * other[e4]), (anti_wedge[e2] * other[e41]) + (anti_wedge[e4] * other[e12]) + (anti_wedge[e41] * other[e2]) + (anti_wedge[e12] * other[e4]), -(anti_wedge[e1] * other[e23]) - (anti_wedge[e2] * other[e31]) - (anti_wedge[e3] * other[e12]) - (anti_wedge[e12] * other[e3])) + (float4(anti_wedge[scalar]) * other.group4()) + (float4(other[scalar]) * anti_wedge.group4()) - (other.group1().yzxx * float4(anti_wedge.group2().zxy, anti_wedge[e23])) - float4((other.group2().zxy * anti_wedge.group1().yzx), anti_wedge[e31] * other[e2])
        );
    }
}
extension Origin: AntiProjectOrthogonallyOnto<Point> {
    func anti_project_orthogonally_onto(self, other: Point) -> Point {
        return Point::from_groups(
            /* e1, e2, e3, e4 */
            float4(self[e4] * 0.0) * other.group0()
        );
    }
}
extension Origin: AntiProjectOrthogonallyOnto<Scalar> {
    func anti_project_orthogonally_onto(self, other: Scalar) -> Origin {
        return Origin::from_groups(
            /* e4 */
            powi(other[scalar], 2) * self[e4]
        );
    }
}
extension Plane {
    func div(_rhs: anti_project_orthogonally_onto) -> anti_project_orthogonally_onto_partial<Plane> {
        anti_project_orthogonally_onto_partial(this)
    }
}
extension Plane: AntiProjectOrthogonallyOnto<DualNum> {
    func anti_project_orthogonally_onto(self, other: DualNum) -> Plane {
AntiScalar right_anti_dual = AntiScalar::from_groups(
            /* e1234 */
            other[scalar]
        );
        return Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4(other[scalar]) * float4(right_anti_dual[e1234] * self[e423], right_anti_dual[e1234] * self[e431], right_anti_dual[e1234] * self[e412], right_anti_dual[e1234] * self[e321])
        );
    }
}
extension Plane: AntiProjectOrthogonallyOnto<Flector> {
    func anti_project_orthogonally_onto(self, other: Flector) -> Flector {
Flector right_anti_dual = Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other[e321] * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other[e1], other[e2], other[e3], 0.0)
        );
Motor anti_wedge = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(((right_anti_dual.group1().yzx * self.group0().zxy) - (right_anti_dual.group1().zxy * self.group0().yzx)), 0.0), 
            /* e23, e31, e12, scalar */
            float4(right_anti_dual[e321] * self[e423], right_anti_dual[e321] * self[e431], right_anti_dual[e321] * self[e412], -(right_anti_dual[e2] * self[e431]) - (right_anti_dual[e3] * self[e412]) - (right_anti_dual[e4] * self[e321])) - (self.group0().wwwx * float4(right_anti_dual.group1().xyz, right_anti_dual[e1]))
        );
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(anti_wedge[scalar]) * other.group0(), 
            /* e423, e431, e412, e321 */
            float4((other[e4] * anti_wedge[e23]) + (other[e423] * anti_wedge[scalar]), (other[e4] * anti_wedge[e31]) + (other[e431] * anti_wedge[scalar]), (other[e4] * anti_wedge[e12]) + (other[e412] * anti_wedge[scalar]), -(other[e2] * anti_wedge[e31]) - (other[e3] * anti_wedge[e12])) + float4((other.group0().zxy * anti_wedge.group0().yzx), other[e321] * anti_wedge[scalar]) - (other.group0().yzxx * float4(anti_wedge.group0().zxy, anti_wedge[e23]))
        );
    }
}
extension Plane: AntiProjectOrthogonallyOnto<Horizon> {
    func anti_project_orthogonally_onto(self, other: Horizon) -> Horizon {
        return Horizon::from_groups(
            /* e321 */
            powi(other[e321], 2) * self[e321]
        );
    }
}
extension Plane: AntiProjectOrthogonallyOnto<Line> {
    func anti_project_orthogonally_onto(self, other: Line) -> Plane {
Line right_anti_dual = Line::from_groups(
            /* e41, e42, e43 */
            other.group1() * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
Point anti_wedge = Point::from_groups(
            /* e1, e2, e3, e4 */
            float4((right_anti_dual[e41] * self[e321]) + (right_anti_dual[e31] * self[e412]), (right_anti_dual[e42] * self[e321]) + (right_anti_dual[e12] * self[e423]), (right_anti_dual[e43] * self[e321]) + (right_anti_dual[e23] * self[e431]), -(right_anti_dual[e42] * self[e431]) - (right_anti_dual[e43] * self[e412])) - (self.group0().yzxx * float4(right_anti_dual.group1().zxy, right_anti_dual[e41]))
        );
        return Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4((other[e42] * anti_wedge[e3]) + (other[e23] * anti_wedge[e4]), (other[e43] * anti_wedge[e1]) + (other[e31] * anti_wedge[e4]), (other[e41] * anti_wedge[e2]) + (other[e12] * anti_wedge[e4]), -(other[e31] * anti_wedge[e2]) - (other[e12] * anti_wedge[e3])) - (anti_wedge.group0().yzxx * float4(other.group0().zxy, other[e23]))
        );
    }
}
extension Plane: AntiProjectOrthogonallyOnto<Motor> {
    func anti_project_orthogonally_onto(self, other: Motor) -> Flector {
Motor right_anti_dual = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            other.group1() * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
Flector anti_wedge = Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4((right_anti_dual[e41] * self[e321]) + (right_anti_dual[e31] * self[e412]), (right_anti_dual[e42] * self[e321]) + (right_anti_dual[e12] * self[e423]), (right_anti_dual[e43] * self[e321]) + (right_anti_dual[e23] * self[e431]), -(right_anti_dual[e42] * self[e431]) - (right_anti_dual[e43] * self[e412])) - (self.group0().yzxx * float4(right_anti_dual.group1().zxy, right_anti_dual[e41])), 
            /* e423, e431, e412, e321 */
            float4(right_anti_dual[e1234]) * self.group0()
        );
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(other[scalar]) * anti_wedge.group0(), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge[e4] * other[e23]) + (anti_wedge[e423] * other[scalar]), (anti_wedge[e4] * other[e31]) + (anti_wedge[e431] * other[scalar]), (anti_wedge[e4] * other[e12]) + (anti_wedge[e412] * other[scalar]), -(anti_wedge[e2] * other[e31]) - (anti_wedge[e3] * other[e12])) + float4((anti_wedge.group0().zxy * other.group0().yzx), anti_wedge[e321] * other[scalar]) - (anti_wedge.group0().yzxx * float4(other.group0().zxy, other[e23]))
        );
    }
}
extension Plane: AntiProjectOrthogonallyOnto<MultiVector> {
    func anti_project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
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
MultiVector anti_wedge = MultiVector::from_groups(
            /* scalar, e1234 */
            float2(-(right_anti_dual[e1] * self[e423]) - (right_anti_dual[e2] * self[e431]) - (right_anti_dual[e3] * self[e412]) - (right_anti_dual[e4] * self[e321]), 0.0), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual[e41] * self[e321]) + (right_anti_dual[e31] * self[e412]), (right_anti_dual[e42] * self[e321]) + (right_anti_dual[e12] * self[e423]), (right_anti_dual[e43] * self[e321]) + (right_anti_dual[e23] * self[e431]), -(right_anti_dual[e42] * self[e431]) - (right_anti_dual[e43] * self[e412])) - (self.group0().yzxx * float4(right_anti_dual.group3().zxy, right_anti_dual[e41])), 
            /* e41, e42, e43 */
            (right_anti_dual.group4().yzx * self.group0().zxy) - (right_anti_dual.group4().zxy * self.group0().yzx), 
            /* e23, e31, e12 */
            (float3(right_anti_dual[e321]) * self.group0().xyz) - (float3(self[e321]) * right_anti_dual.group4().xyz), 
            /* e423, e431, e412, e321 */
            float4(right_anti_dual[e1234]) * self.group0()
        );
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(anti_wedge[scalar] * other[scalar], (anti_wedge[scalar] * other[e1234]) + (anti_wedge[e1234] * other[scalar]) + (anti_wedge[e423] * other[e1]) + (anti_wedge[e431] * other[e2]) + (anti_wedge[e412] * other[e3]) + (anti_wedge[e321] * other[e4]) - (anti_wedge[e1] * other[e423]) - (anti_wedge[e2] * other[e431]) - (anti_wedge[e3] * other[e412]) - (anti_wedge[e4] * other[e321]) - (anti_wedge[e41] * other[e23]) - (anti_wedge[e42] * other[e31]) - (anti_wedge[e43] * other[e12]) - (anti_wedge[e23] * other[e41]) - (anti_wedge[e31] * other[e42]) - (anti_wedge[e12] * other[e43])), 
            /* e1, e2, e3, e4 */
            (float4(anti_wedge[scalar]) * other.group1()) + (float4(other[scalar]) * anti_wedge.group1()), 
            /* e41, e42, e43 */
            (float3(anti_wedge[scalar]) * other.group2()) + (float3(other[scalar]) * anti_wedge.group2()) + (float3(other[e4]) * anti_wedge.group1().xyz) - (float3(anti_wedge[e4]) * other.group1().xyz), 
            /* e23, e31, e12 */
            (float3(anti_wedge[scalar]) * other.group3()) + (float3(other[scalar]) * anti_wedge.group3()) + (anti_wedge.group1().zxy * other.group1().yzx) - (anti_wedge.group1().yzx * other.group1().zxy), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge[e3] * other[e42]) + (anti_wedge[e4] * other[e23]) + (anti_wedge[e42] * other[e3]) + (anti_wedge[e23] * other[e4]), (anti_wedge[e1] * other[e43]) + (anti_wedge[e4] * other[e31]) + (anti_wedge[e43] * other[e1]) + (anti_wedge[e31] * other[e4]), (anti_wedge[e2] * other[e41]) + (anti_wedge[e4] * other[e12]) + (anti_wedge[e41] * other[e2]) + (anti_wedge[e12] * other[e4]), -(anti_wedge[e1] * other[e23]) - (anti_wedge[e2] * other[e31]) - (anti_wedge[e3] * other[e12]) - (anti_wedge[e12] * other[e3])) + (float4(anti_wedge[scalar]) * other.group4()) + (float4(other[scalar]) * anti_wedge.group4()) - (other.group1().yzxx * float4(anti_wedge.group2().zxy, anti_wedge[e23])) - float4((other.group2().zxy * anti_wedge.group1().yzx), anti_wedge[e31] * other[e2])
        );
    }
}
extension Plane: AntiProjectOrthogonallyOnto<Plane> {
    func anti_project_orthogonally_onto(self, other: Plane) -> Plane {
        return Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4(other[e321] * self[e321]) * other.group0()
        );
    }
}
extension Plane: AntiProjectOrthogonallyOnto<Point> {
    func anti_project_orthogonally_onto(self, other: Point) -> Plane {
Plane right_anti_dual = Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4(other[e1], other[e2], other[e3], 0.0)
        );
Line anti_wedge = Line::from_groups(
            /* e41, e42, e43 */
            (right_anti_dual.group0().yzx * self.group0().zxy) - (right_anti_dual.group0().zxy * self.group0().yzx), 
            /* e23, e31, e12 */
            (float3(right_anti_dual[e321]) * self.group0().xyz) - (float3(self[e321]) * right_anti_dual.group0().xyz)
        );
        return Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4((anti_wedge[e42] * other[e3]) + (anti_wedge[e23] * other[e4]), (anti_wedge[e43] * other[e1]) + (anti_wedge[e31] * other[e4]), (anti_wedge[e41] * other[e2]) + (anti_wedge[e12] * other[e4]), -(anti_wedge[e31] * other[e2]) - (anti_wedge[e12] * other[e3])) - (other.group0().yzxx * float4(anti_wedge.group0().zxy, anti_wedge[e23]))
        );
    }
}
extension Plane: AntiProjectOrthogonallyOnto<Scalar> {
    func anti_project_orthogonally_onto(self, other: Scalar) -> Plane {
AntiScalar right_anti_dual = AntiScalar::from_groups(
            /* e1234 */
            other[scalar]
        );
        return Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4(other[scalar]) * float4(right_anti_dual[e1234] * self[e423], right_anti_dual[e1234] * self[e431], right_anti_dual[e1234] * self[e412], right_anti_dual[e1234] * self[e321])
        );
    }
}
extension Point {
    func div(_rhs: anti_project_orthogonally_onto) -> anti_project_orthogonally_onto_partial<Point> {
        anti_project_orthogonally_onto_partial(this)
    }
}
extension Point: AntiProjectOrthogonallyOnto<DualNum> {
    func anti_project_orthogonally_onto(self, other: DualNum) -> Point {
AntiScalar right_anti_dual = AntiScalar::from_groups(
            /* e1234 */
            other[scalar]
        );
        return Point::from_groups(
            /* e1, e2, e3, e4 */
            float4(other[scalar]) * float4(right_anti_dual[e1234] * self[e1], right_anti_dual[e1234] * self[e2], right_anti_dual[e1234] * self[e3], right_anti_dual[e1234] * self[e4])
        );
    }
}
extension Point: AntiProjectOrthogonallyOnto<Flector> {
    func anti_project_orthogonally_onto(self, other: Flector) -> Flector {
Flector right_anti_dual = Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other[e321] * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other[e1], other[e2], other[e3], 0.0)
        );
Scalar anti_wedge = Scalar::from_groups(
            /* scalar */
            (right_anti_dual[e423] * self[e1]) + (right_anti_dual[e431] * self[e2]) + (right_anti_dual[e412] * self[e3]) + (right_anti_dual[e321] * self[e4])
        );
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(anti_wedge[scalar]) * other.group0(), 
            /* e423, e431, e412, e321 */
            float4(anti_wedge[scalar]) * other.group1()
        );
    }
}
extension Point: AntiProjectOrthogonallyOnto<Motor> {
    func anti_project_orthogonally_onto(self, other: Motor) -> Flector {
Point anti_wedge = Point::from_groups(
            /* e1, e2, e3, e4 */
            float4(other[scalar]) * self.group0()
        );
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(other[scalar]) * anti_wedge.group0(), 
            /* e423, e431, e412, e321 */
            float4((other[e42] * anti_wedge[e3]) + (other[e23] * anti_wedge[e4]), (other[e43] * anti_wedge[e1]) + (other[e31] * anti_wedge[e4]), (other[e41] * anti_wedge[e2]) + (other[e12] * anti_wedge[e4]), -(other[e31] * anti_wedge[e2]) - (other[e12] * anti_wedge[e3])) - (anti_wedge.group0().yzxx * float4(other.group0().zxy, other[e23]))
        );
    }
}
extension Point: AntiProjectOrthogonallyOnto<MultiVector> {
    func anti_project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
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
MultiVector anti_wedge = MultiVector::from_groups(
            /* scalar, e1234 */
            float2((right_anti_dual[e423] * self[e1]) + (right_anti_dual[e431] * self[e2]) + (right_anti_dual[e412] * self[e3]) + (right_anti_dual[e321] * self[e4]), 0.0), 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual[e1234]) * self.group0(), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(anti_wedge[scalar] * other[scalar], (anti_wedge[scalar] * other[e1234]) + (anti_wedge[e1234] * other[scalar]) + (anti_wedge[e423] * other[e1]) + (anti_wedge[e431] * other[e2]) + (anti_wedge[e412] * other[e3]) + (anti_wedge[e321] * other[e4]) - (anti_wedge[e1] * other[e423]) - (anti_wedge[e2] * other[e431]) - (anti_wedge[e3] * other[e412]) - (anti_wedge[e4] * other[e321]) - (anti_wedge[e41] * other[e23]) - (anti_wedge[e42] * other[e31]) - (anti_wedge[e43] * other[e12]) - (anti_wedge[e23] * other[e41]) - (anti_wedge[e31] * other[e42]) - (anti_wedge[e12] * other[e43])), 
            /* e1, e2, e3, e4 */
            (float4(anti_wedge[scalar]) * other.group1()) + (float4(other[scalar]) * anti_wedge.group1()), 
            /* e41, e42, e43 */
            (float3(anti_wedge[scalar]) * other.group2()) + (float3(other[scalar]) * anti_wedge.group2()) + (float3(other[e4]) * anti_wedge.group1().xyz) - (float3(anti_wedge[e4]) * other.group1().xyz), 
            /* e23, e31, e12 */
            (float3(anti_wedge[scalar]) * other.group3()) + (float3(other[scalar]) * anti_wedge.group3()) + (anti_wedge.group1().zxy * other.group1().yzx) - (anti_wedge.group1().yzx * other.group1().zxy), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge[e3] * other[e42]) + (anti_wedge[e4] * other[e23]) + (anti_wedge[e42] * other[e3]) + (anti_wedge[e23] * other[e4]), (anti_wedge[e1] * other[e43]) + (anti_wedge[e4] * other[e31]) + (anti_wedge[e43] * other[e1]) + (anti_wedge[e31] * other[e4]), (anti_wedge[e2] * other[e41]) + (anti_wedge[e4] * other[e12]) + (anti_wedge[e41] * other[e2]) + (anti_wedge[e12] * other[e4]), -(anti_wedge[e1] * other[e23]) - (anti_wedge[e2] * other[e31]) - (anti_wedge[e3] * other[e12]) - (anti_wedge[e12] * other[e3])) + (float4(anti_wedge[scalar]) * other.group4()) + (float4(other[scalar]) * anti_wedge.group4()) - (other.group1().yzxx * float4(anti_wedge.group2().zxy, anti_wedge[e23])) - float4((other.group2().zxy * anti_wedge.group1().yzx), anti_wedge[e31] * other[e2])
        );
    }
}
extension Point: AntiProjectOrthogonallyOnto<Point> {
    func anti_project_orthogonally_onto(self, other: Point) -> Point {
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
extension Point: AntiProjectOrthogonallyOnto<Scalar> {
    func anti_project_orthogonally_onto(self, other: Scalar) -> Point {
AntiScalar right_anti_dual = AntiScalar::from_groups(
            /* e1234 */
            other[scalar]
        );
        return Point::from_groups(
            /* e1, e2, e3, e4 */
            float4(other[scalar]) * float4(right_anti_dual[e1234] * self[e1], right_anti_dual[e1234] * self[e2], right_anti_dual[e1234] * self[e3], right_anti_dual[e1234] * self[e4])
        );
    }
}
extension Scalar {
    func div(_rhs: anti_project_orthogonally_onto) -> anti_project_orthogonally_onto_partial<Scalar> {
        anti_project_orthogonally_onto_partial(this)
    }
}
extension Scalar: AntiProjectOrthogonallyOnto<DualNum> {
    func anti_project_orthogonally_onto(self, other: DualNum) -> DualNum {
        return DualNum::from_groups(
            /* scalar, e1234 */
            float2(other[scalar] * self[scalar]) * other.group0()
        );
    }
}
extension Scalar: AntiProjectOrthogonallyOnto<Motor> {
    func anti_project_orthogonally_onto(self, other: Motor) -> Motor {
Scalar anti_wedge = Scalar::from_groups(
            /* scalar */
            (other.group1() * float4(-1.0, -1.0, -1.0, 1.0))[3] * self[scalar]
        );
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(anti_wedge[scalar]) * other.group0(), 
            /* e23, e31, e12, scalar */
            float4(anti_wedge[scalar]) * other.group1()
        );
    }
}
extension Scalar: AntiProjectOrthogonallyOnto<MultiVector> {
    func anti_project_orthogonally_onto(self, other: MultiVector) -> MultiVector {
Scalar anti_wedge = Scalar::from_groups(
            /* scalar */
            other[scalar] * self[scalar]
        );
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(anti_wedge[scalar]) * other.group0(), 
            /* e1, e2, e3, e4 */
            float4(anti_wedge[scalar]) * other.group1(), 
            /* e41, e42, e43 */
            float3(anti_wedge[scalar]) * other.group2(), 
            /* e23, e31, e12 */
            float3(anti_wedge[scalar]) * other.group3(), 
            /* e423, e431, e412, e321 */
            float4(anti_wedge[scalar]) * other.group4()
        );
    }
}
extension Scalar: AntiProjectOrthogonallyOnto<Scalar> {
    func anti_project_orthogonally_onto(self, other: Scalar) -> Scalar {
        return Scalar::from_groups(
            /* scalar */
            powi(other[scalar], 2) * self[scalar]
        );
    }
}
