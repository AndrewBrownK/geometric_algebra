extension AntiScalar {
    // Fancy infix trick
    func operator/(_rhs: AntiProjectOrthogonallyOntoInfix) -> AntiProjectOrthogonallyOntoInfixPartial<AntiScalar> {
        return AntiProjectOrthogonallyOntoInfixPartial { a: this };
    }
}
extension AntiScalar: AntiProjectOrthogonallyOnto<DualNum> {
    associatedtype Output = AntiScalar;
    func anti_project_orthogonally_onto(other: DualNum) -> Output {
        return AntiScalar.from_groups(
            /* e1234 */
            powi(other.scalar, 2) * this.e1234
        );
    }
}
extension AntiScalar: AntiProjectOrthogonallyOnto<Flector> {
    associatedtype Output = Motor;
    func anti_project_orthogonally_onto(other: Flector) -> Output {
Flector right_anti_dual = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.e1, other.e2, other.e3, 0.0)
        );
Flector anti_wedge = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e1234) * right_anti_dual.group0, 
            /* e423, e431, e412, e321 */
            float4(this.e1234) * right_anti_dual.group1
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            (other.group0.wwwx * float4(anti_wedge.group0.xyz, anti_wedge.e423)) + float4(float3(0.0), (anti_wedge.e431 * other.e2) + (anti_wedge.e412 * other.e3) + (anti_wedge.e321 * other.e4) - (anti_wedge.e2 * other.e431) - (anti_wedge.e3 * other.e412) - (anti_wedge.e4 * other.e321)) - (anti_wedge.group0.wwwx * float4(other.group0.xyz, other.e423)), 
            /* e23, e31, e12, scalar */
            float4(((anti_wedge.group0.zxy * other.group0.yzx) - (anti_wedge.group0.yzx * other.group0.zxy)), 0.0)
        );
    }
}
extension AntiScalar: AntiProjectOrthogonallyOnto<Horizon> {
    associatedtype Output = AntiScalar;
    func anti_project_orthogonally_onto(other: Horizon) -> Output {
        return AntiScalar.from_groups(
            /* e1234 */
            powi(other.e321, 2) * this.e1234
        );
    }
}
extension AntiScalar: AntiProjectOrthogonallyOnto<Line> {
    associatedtype Output = AntiScalar;
    func anti_project_orthogonally_onto(other: Line) -> Output {
Line right_anti_dual = Line.from_groups(
            /* e41, e42, e43 */
            other.group1 * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
Line anti_wedge = Line.from_groups(
            /* e41, e42, e43 */
            float3(this.e1234) * right_anti_dual.group0, 
            /* e23, e31, e12 */
            float3(this.e1234) * right_anti_dual.group1
        );
        return AntiScalar.from_groups(
            /* e1234 */
            -(anti_wedge.e41 * other.e23) - (anti_wedge.e42 * other.e31) - (anti_wedge.e43 * other.e12) - (anti_wedge.e23 * other.e41) - (anti_wedge.e31 * other.e42) - (anti_wedge.e12 * other.e43)
        );
    }
}
extension AntiScalar: AntiProjectOrthogonallyOnto<Motor> {
    associatedtype Output = Motor;
    func anti_project_orthogonally_onto(other: Motor) -> Output {
Motor right_anti_dual = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
Motor anti_wedge = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.e1234) * right_anti_dual.group0, 
            /* e23, e31, e12, scalar */
            float4(this.e1234) * right_anti_dual.group1
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            (float4(anti_wedge.scalar) * other.group0) + (float4(other.scalar) * anti_wedge.group0) + float4(float3(0.0), -(anti_wedge.e41 * other.e23) - (anti_wedge.e42 * other.e31) - (anti_wedge.e43 * other.e12) - (anti_wedge.e23 * other.e41) - (anti_wedge.e31 * other.e42) - (anti_wedge.e12 * other.e43)), 
            /* e23, e31, e12, scalar */
            float4(float3(0.0), anti_wedge.scalar * other.scalar)
        );
    }
}
extension AntiScalar: AntiProjectOrthogonallyOnto<MultiVector> {
    associatedtype Output = MultiVector;
    func anti_project_orthogonally_onto(other: MultiVector) -> Output {
MultiVector right_anti_dual = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, other.scalar), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e41, e42, e43 */
            other.group3 * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(other.e1, other.e2, other.e3, 0.0)
        );
MultiVector anti_wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(this.e1234) * right_anti_dual.group0, 
            /* e1, e2, e3, e4 */
            float4(this.e1234) * right_anti_dual.group1, 
            /* e41, e42, e43 */
            float3(this.e1234) * right_anti_dual.group2, 
            /* e23, e31, e12 */
            float3(this.e1234) * right_anti_dual.group3, 
            /* e423, e431, e412, e321 */
            float4(this.e1234) * right_anti_dual.group4
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(anti_wedge.scalar * other.scalar, (anti_wedge.scalar * other.e1234) + (anti_wedge.e1234 * other.scalar) + (anti_wedge.e423 * other.e1) + (anti_wedge.e431 * other.e2) + (anti_wedge.e412 * other.e3) + (anti_wedge.e321 * other.e4) - (anti_wedge.e1 * other.e423) - (anti_wedge.e2 * other.e431) - (anti_wedge.e3 * other.e412) - (anti_wedge.e4 * other.e321) - (anti_wedge.e41 * other.e23) - (anti_wedge.e42 * other.e31) - (anti_wedge.e43 * other.e12) - (anti_wedge.e23 * other.e41) - (anti_wedge.e31 * other.e42) - (anti_wedge.e12 * other.e43)), 
            /* e1, e2, e3, e4 */
            (float4(anti_wedge.scalar) * other.group1) + (float4(other.scalar) * anti_wedge.group1), 
            /* e41, e42, e43 */
            (float3(anti_wedge.scalar) * other.group2) + (float3(other.scalar) * anti_wedge.group2) + (float3(other.e4) * anti_wedge.group1.xyz) - (float3(anti_wedge.e4) * other.group1.xyz), 
            /* e23, e31, e12 */
            (float3(anti_wedge.scalar) * other.group3) + (float3(other.scalar) * anti_wedge.group3) + (anti_wedge.group1.zxy * other.group1.yzx) - (anti_wedge.group1.yzx * other.group1.zxy), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge.e3 * other.e42) + (anti_wedge.e4 * other.e23) + (anti_wedge.e42 * other.e3) + (anti_wedge.e23 * other.e4), (anti_wedge.e1 * other.e43) + (anti_wedge.e4 * other.e31) + (anti_wedge.e43 * other.e1) + (anti_wedge.e31 * other.e4), (anti_wedge.e2 * other.e41) + (anti_wedge.e4 * other.e12) + (anti_wedge.e41 * other.e2) + (anti_wedge.e12 * other.e4), -(anti_wedge.e1 * other.e23) - (anti_wedge.e2 * other.e31) - (anti_wedge.e3 * other.e12) - (anti_wedge.e12 * other.e3)) + (float4(anti_wedge.scalar) * other.group4) + (float4(other.scalar) * anti_wedge.group4) - (other.group1.yzxx * float4(anti_wedge.group2.zxy, anti_wedge.e23)) - float4((other.group2.zxy * anti_wedge.group1.yzx), anti_wedge.e31 * other.e2)
        );
    }
}
extension AntiScalar: AntiProjectOrthogonallyOnto<Plane> {
    associatedtype Output = AntiScalar;
    func anti_project_orthogonally_onto(other: Plane) -> Output {
        return AntiScalar.from_groups(
            /* e1234 */
            powi(other.e321, 2) * this.e1234
        );
    }
}
extension AntiScalar: AntiProjectOrthogonallyOnto<Point> {
    associatedtype Output = AntiScalar;
    func anti_project_orthogonally_onto(other: Point) -> Output {
Plane anti_wedge = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(this.e1234) * float4(other.e1, other.e2, other.e3, 0.0)
        );
        return AntiScalar.from_groups(
            /* e1234 */
            (anti_wedge.e423 * other.e1) + (anti_wedge.e431 * other.e2) + (anti_wedge.e412 * other.e3) + (anti_wedge.e321 * other.e4)
        );
    }
}
extension AntiScalar: AntiProjectOrthogonallyOnto<Scalar> {
    associatedtype Output = AntiScalar;
    func anti_project_orthogonally_onto(other: Scalar) -> Output {
        return AntiScalar.from_groups(
            /* e1234 */
            powi(other.scalar, 2) * this.e1234
        );
    }
}
extension DualNum {
    // Fancy infix trick
    func operator/(_rhs: AntiProjectOrthogonallyOntoInfix) -> AntiProjectOrthogonallyOntoInfixPartial<DualNum> {
        return AntiProjectOrthogonallyOntoInfixPartial { a: this };
    }
}
extension DualNum: AntiProjectOrthogonallyOnto<DualNum> {
    associatedtype Output = DualNum;
    func anti_project_orthogonally_onto(other: DualNum) -> Output {
DualNum anti_wedge = DualNum.from_groups(
            /* scalar, e1234 */
            float2(other.scalar) * this.group0
        );
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(anti_wedge.scalar * other.scalar, (anti_wedge.scalar * other.e1234) + (anti_wedge.e1234 * other.scalar))
        );
    }
}
extension DualNum: AntiProjectOrthogonallyOnto<Flector> {
    associatedtype Output = Motor;
    func anti_project_orthogonally_onto(other: Flector) -> Output {
Flector right_anti_dual = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.e1, other.e2, other.e3, 0.0)
        );
Flector anti_wedge = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e1234) * right_anti_dual.group0, 
            /* e423, e431, e412, e321 */
            float4(this.e1234) * right_anti_dual.group1
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            (other.group0.wwwx * float4(anti_wedge.group0.xyz, anti_wedge.e423)) + float4(float3(0.0), (anti_wedge.e431 * other.e2) + (anti_wedge.e412 * other.e3) + (anti_wedge.e321 * other.e4) - (anti_wedge.e2 * other.e431) - (anti_wedge.e3 * other.e412) - (anti_wedge.e4 * other.e321)) - (anti_wedge.group0.wwwx * float4(other.group0.xyz, other.e423)), 
            /* e23, e31, e12, scalar */
            float4(((anti_wedge.group0.zxy * other.group0.yzx) - (anti_wedge.group0.yzx * other.group0.zxy)), 0.0)
        );
    }
}
extension DualNum: AntiProjectOrthogonallyOnto<Horizon> {
    associatedtype Output = AntiScalar;
    func anti_project_orthogonally_onto(other: Horizon) -> Output {
        return AntiScalar.from_groups(
            /* e1234 */
            powi(other.e321, 2) * this.e1234
        );
    }
}
extension DualNum: AntiProjectOrthogonallyOnto<Line> {
    associatedtype Output = AntiScalar;
    func anti_project_orthogonally_onto(other: Line) -> Output {
Line right_anti_dual = Line.from_groups(
            /* e41, e42, e43 */
            other.group1 * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
Line anti_wedge = Line.from_groups(
            /* e41, e42, e43 */
            float3(this.e1234) * right_anti_dual.group0, 
            /* e23, e31, e12 */
            float3(this.e1234) * right_anti_dual.group1
        );
        return AntiScalar.from_groups(
            /* e1234 */
            -(anti_wedge.e41 * other.e23) - (anti_wedge.e42 * other.e31) - (anti_wedge.e43 * other.e12) - (anti_wedge.e23 * other.e41) - (anti_wedge.e31 * other.e42) - (anti_wedge.e12 * other.e43)
        );
    }
}
extension DualNum: AntiProjectOrthogonallyOnto<Motor> {
    associatedtype Output = Motor;
    func anti_project_orthogonally_onto(other: Motor) -> Output {
Motor right_anti_dual = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
Motor anti_wedge = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.e1234) * right_anti_dual.group0, 
            /* e23, e31, e12, scalar */
            float4(right_anti_dual.e23, right_anti_dual.e31, right_anti_dual.e12, 1.0) * float4(this.group0.yy, this.e1234, (this.scalar * right_anti_dual.e1234) + (this.e1234 * right_anti_dual.scalar))
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            (float4(anti_wedge.scalar) * other.group0) + (float4(other.scalar) * anti_wedge.group0) + float4(float3(0.0), -(anti_wedge.e41 * other.e23) - (anti_wedge.e42 * other.e31) - (anti_wedge.e43 * other.e12) - (anti_wedge.e23 * other.e41) - (anti_wedge.e31 * other.e42) - (anti_wedge.e12 * other.e43)), 
            /* e23, e31, e12, scalar */
            float4(((float3(anti_wedge.scalar) * other.group1.xyz) + (float3(other.scalar) * anti_wedge.group1.xyz)), anti_wedge.scalar * other.scalar)
        );
    }
}
extension DualNum: AntiProjectOrthogonallyOnto<MultiVector> {
    associatedtype Output = MultiVector;
    func anti_project_orthogonally_onto(other: MultiVector) -> Output {
MultiVector right_anti_dual = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, other.scalar), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e41, e42, e43 */
            other.group3 * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(other.e1, other.e2, other.e3, 0.0)
        );
MultiVector anti_wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2((this.scalar * right_anti_dual.e1234) + (this.e1234 * right_anti_dual.scalar), this.e1234 * right_anti_dual.e1234), 
            /* e1, e2, e3, e4 */
            float4(this.e1234) * right_anti_dual.group1, 
            /* e41, e42, e43 */
            float3(this.e1234) * right_anti_dual.group2, 
            /* e23, e31, e12 */
            float3(this.e1234) * right_anti_dual.group3, 
            /* e423, e431, e412, e321 */
            float4(this.e1234) * right_anti_dual.group4
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(anti_wedge.scalar * other.scalar, (anti_wedge.scalar * other.e1234) + (anti_wedge.e1234 * other.scalar) + (anti_wedge.e423 * other.e1) + (anti_wedge.e431 * other.e2) + (anti_wedge.e412 * other.e3) + (anti_wedge.e321 * other.e4) - (anti_wedge.e1 * other.e423) - (anti_wedge.e2 * other.e431) - (anti_wedge.e3 * other.e412) - (anti_wedge.e4 * other.e321) - (anti_wedge.e41 * other.e23) - (anti_wedge.e42 * other.e31) - (anti_wedge.e43 * other.e12) - (anti_wedge.e23 * other.e41) - (anti_wedge.e31 * other.e42) - (anti_wedge.e12 * other.e43)), 
            /* e1, e2, e3, e4 */
            (float4(anti_wedge.scalar) * other.group1) + (float4(other.scalar) * anti_wedge.group1), 
            /* e41, e42, e43 */
            (float3(anti_wedge.scalar) * other.group2) + (float3(other.scalar) * anti_wedge.group2) + (float3(other.e4) * anti_wedge.group1.xyz) - (float3(anti_wedge.e4) * other.group1.xyz), 
            /* e23, e31, e12 */
            (float3(anti_wedge.scalar) * other.group3) + (float3(other.scalar) * anti_wedge.group3) + (anti_wedge.group1.zxy * other.group1.yzx) - (anti_wedge.group1.yzx * other.group1.zxy), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge.e3 * other.e42) + (anti_wedge.e4 * other.e23) + (anti_wedge.e42 * other.e3) + (anti_wedge.e23 * other.e4), (anti_wedge.e1 * other.e43) + (anti_wedge.e4 * other.e31) + (anti_wedge.e43 * other.e1) + (anti_wedge.e31 * other.e4), (anti_wedge.e2 * other.e41) + (anti_wedge.e4 * other.e12) + (anti_wedge.e41 * other.e2) + (anti_wedge.e12 * other.e4), -(anti_wedge.e1 * other.e23) - (anti_wedge.e2 * other.e31) - (anti_wedge.e3 * other.e12) - (anti_wedge.e12 * other.e3)) + (float4(anti_wedge.scalar) * other.group4) + (float4(other.scalar) * anti_wedge.group4) - (other.group1.yzxx * float4(anti_wedge.group2.zxy, anti_wedge.e23)) - float4((other.group2.zxy * anti_wedge.group1.yzx), anti_wedge.e31 * other.e2)
        );
    }
}
extension DualNum: AntiProjectOrthogonallyOnto<Plane> {
    associatedtype Output = AntiScalar;
    func anti_project_orthogonally_onto(other: Plane) -> Output {
        return AntiScalar.from_groups(
            /* e1234 */
            powi(other.e321, 2) * this.e1234
        );
    }
}
extension DualNum: AntiProjectOrthogonallyOnto<Point> {
    associatedtype Output = AntiScalar;
    func anti_project_orthogonally_onto(other: Point) -> Output {
Plane anti_wedge = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(this.e1234) * float4(other.e1, other.e2, other.e3, 0.0)
        );
        return AntiScalar.from_groups(
            /* e1234 */
            (anti_wedge.e423 * other.e1) + (anti_wedge.e431 * other.e2) + (anti_wedge.e412 * other.e3) + (anti_wedge.e321 * other.e4)
        );
    }
}
extension DualNum: AntiProjectOrthogonallyOnto<Scalar> {
    associatedtype Output = DualNum;
    func anti_project_orthogonally_onto(other: Scalar) -> Output {
AntiScalar right_anti_dual = AntiScalar.from_groups(
            /* e1234 */
            other.scalar
        );
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(other.scalar) * float2(right_anti_dual.e1234 * this.scalar, right_anti_dual.e1234 * this.e1234)
        );
    }
}
extension Flector {
    // Fancy infix trick
    func operator/(_rhs: AntiProjectOrthogonallyOntoInfix) -> AntiProjectOrthogonallyOntoInfixPartial<Flector> {
        return AntiProjectOrthogonallyOntoInfixPartial { a: this };
    }
}
extension Flector: AntiProjectOrthogonallyOnto<DualNum> {
    associatedtype Output = Flector;
    func anti_project_orthogonally_onto(other: DualNum) -> Output {
AntiScalar right_anti_dual = AntiScalar.from_groups(
            /* e1234 */
            other.scalar
        );
Flector anti_wedge = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(right_anti_dual.e1234) * this.group0, 
            /* e423, e431, e412, e321 */
            float4(right_anti_dual.e1234) * this.group1
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * anti_wedge.group0, 
            /* e423, e431, e412, e321 */
            float4(other.scalar) * anti_wedge.group1
        );
    }
}
extension Flector: AntiProjectOrthogonallyOnto<Flector> {
    associatedtype Output = Flector;
    func anti_project_orthogonally_onto(other: Flector) -> Output {
Flector right_anti_dual = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.e1, other.e2, other.e3, 0.0)
        );
Motor anti_wedge = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(((right_anti_dual.group1.yzx * this.group1.zxy) - (right_anti_dual.group1.zxy * this.group1.yzx)), 0.0), 
            /* e23, e31, e12, scalar */
            (right_anti_dual.group1.wwwx * float4(this.group1.xyz, this.e1)) + float4(float3(0.0), (right_anti_dual.e431 * this.e2) + (right_anti_dual.e412 * this.e3) + (right_anti_dual.e321 * this.e4) - (right_anti_dual.e2 * this.e431) - (right_anti_dual.e3 * this.e412) - (right_anti_dual.e4 * this.e321)) - (this.group1.wwwx * float4(right_anti_dual.group1.xyz, right_anti_dual.e1))
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(anti_wedge.scalar) * other.group0, 
            /* e423, e431, e412, e321 */
            float4((other.e4 * anti_wedge.e23) + (other.e423 * anti_wedge.scalar), (other.e4 * anti_wedge.e31) + (other.e431 * anti_wedge.scalar), (other.e4 * anti_wedge.e12) + (other.e412 * anti_wedge.scalar), -(other.e2 * anti_wedge.e31) - (other.e3 * anti_wedge.e12)) + float4((other.group0.zxy * anti_wedge.group0.yzx), other.e321 * anti_wedge.scalar) - (other.group0.yzxx * float4(anti_wedge.group0.zxy, anti_wedge.e23))
        );
    }
}
extension Flector: AntiProjectOrthogonallyOnto<Horizon> {
    associatedtype Output = Horizon;
    func anti_project_orthogonally_onto(other: Horizon) -> Output {
        return Horizon.from_groups(
            /* e321 */
            powi(other.e321, 2) * this.e321
        );
    }
}
extension Flector: AntiProjectOrthogonallyOnto<Line> {
    associatedtype Output = Plane;
    func anti_project_orthogonally_onto(other: Line) -> Output {
Line right_anti_dual = Line.from_groups(
            /* e41, e42, e43 */
            other.group1 * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
Point anti_wedge = Point.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e412 * right_anti_dual.e31) + (this.e321 * right_anti_dual.e41), (this.e423 * right_anti_dual.e12) + (this.e321 * right_anti_dual.e42), (this.e431 * right_anti_dual.e23) + (this.e321 * right_anti_dual.e43), -(this.e431 * right_anti_dual.e42) - (this.e412 * right_anti_dual.e43)) - (this.group1.yzxx * float4(right_anti_dual.group1.zxy, right_anti_dual.e41))
        );
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((other.e42 * anti_wedge.e3) + (other.e23 * anti_wedge.e4), (other.e43 * anti_wedge.e1) + (other.e31 * anti_wedge.e4), (other.e41 * anti_wedge.e2) + (other.e12 * anti_wedge.e4), -(other.e31 * anti_wedge.e2) - (other.e12 * anti_wedge.e3)) - (anti_wedge.group0.yzxx * float4(other.group0.zxy, other.e23))
        );
    }
}
extension Flector: AntiProjectOrthogonallyOnto<Motor> {
    associatedtype Output = Flector;
    func anti_project_orthogonally_onto(other: Motor) -> Output {
Motor right_anti_dual = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
Flector anti_wedge = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e412 * right_anti_dual.e31) + (this.e321 * right_anti_dual.e41), (this.e423 * right_anti_dual.e12) + (this.e321 * right_anti_dual.e42), (this.e431 * right_anti_dual.e23) + (this.e321 * right_anti_dual.e43), -(this.e431 * right_anti_dual.e42) - (this.e412 * right_anti_dual.e43)) + (float4(right_anti_dual.e1234) * this.group0) - (this.group1.yzxx * float4(right_anti_dual.group1.zxy, right_anti_dual.e41)), 
            /* e423, e431, e412, e321 */
            float4(right_anti_dual.e1234) * this.group1
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * anti_wedge.group0, 
            /* e423, e431, e412, e321 */
            float4((anti_wedge.e4 * other.e23) + (anti_wedge.e423 * other.scalar), (anti_wedge.e4 * other.e31) + (anti_wedge.e431 * other.scalar), (anti_wedge.e4 * other.e12) + (anti_wedge.e412 * other.scalar), -(anti_wedge.e2 * other.e31) - (anti_wedge.e3 * other.e12)) + float4((anti_wedge.group0.zxy * other.group0.yzx), anti_wedge.e321 * other.scalar) - (anti_wedge.group0.yzxx * float4(other.group0.zxy, other.e23))
        );
    }
}
extension Flector: AntiProjectOrthogonallyOnto<MultiVector> {
    associatedtype Output = MultiVector;
    func anti_project_orthogonally_onto(other: MultiVector) -> Output {
MultiVector right_anti_dual = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, other.scalar), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e41, e42, e43 */
            other.group3 * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(other.e1, other.e2, other.e3, 0.0)
        );
MultiVector anti_wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2((this.e1 * right_anti_dual.e423) + (this.e2 * right_anti_dual.e431) + (this.e3 * right_anti_dual.e412) + (this.e4 * right_anti_dual.e321) - (this.e423 * right_anti_dual.e1) - (this.e431 * right_anti_dual.e2) - (this.e412 * right_anti_dual.e3) - (this.e321 * right_anti_dual.e4), 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.e412 * right_anti_dual.e31) + (this.e321 * right_anti_dual.e41), (this.e423 * right_anti_dual.e12) + (this.e321 * right_anti_dual.e42), (this.e431 * right_anti_dual.e23) + (this.e321 * right_anti_dual.e43), -(this.e431 * right_anti_dual.e42) - (this.e412 * right_anti_dual.e43)) + (float4(right_anti_dual.e1234) * this.group0) - (this.group1.yzxx * float4(right_anti_dual.group3.zxy, right_anti_dual.e41)), 
            /* e41, e42, e43 */
            (this.group1.zxy * right_anti_dual.group4.yzx) - (this.group1.yzx * right_anti_dual.group4.zxy), 
            /* e23, e31, e12 */
            (float3(right_anti_dual.e321) * this.group1.xyz) - (float3(this.e321) * right_anti_dual.group4.xyz), 
            /* e423, e431, e412, e321 */
            float4(right_anti_dual.e1234) * this.group1
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(anti_wedge.scalar * other.scalar, (anti_wedge.scalar * other.e1234) + (anti_wedge.e1234 * other.scalar) + (anti_wedge.e423 * other.e1) + (anti_wedge.e431 * other.e2) + (anti_wedge.e412 * other.e3) + (anti_wedge.e321 * other.e4) - (anti_wedge.e1 * other.e423) - (anti_wedge.e2 * other.e431) - (anti_wedge.e3 * other.e412) - (anti_wedge.e4 * other.e321) - (anti_wedge.e41 * other.e23) - (anti_wedge.e42 * other.e31) - (anti_wedge.e43 * other.e12) - (anti_wedge.e23 * other.e41) - (anti_wedge.e31 * other.e42) - (anti_wedge.e12 * other.e43)), 
            /* e1, e2, e3, e4 */
            (float4(anti_wedge.scalar) * other.group1) + (float4(other.scalar) * anti_wedge.group1), 
            /* e41, e42, e43 */
            (float3(anti_wedge.scalar) * other.group2) + (float3(other.scalar) * anti_wedge.group2) + (float3(other.e4) * anti_wedge.group1.xyz) - (float3(anti_wedge.e4) * other.group1.xyz), 
            /* e23, e31, e12 */
            (float3(anti_wedge.scalar) * other.group3) + (float3(other.scalar) * anti_wedge.group3) + (anti_wedge.group1.zxy * other.group1.yzx) - (anti_wedge.group1.yzx * other.group1.zxy), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge.e3 * other.e42) + (anti_wedge.e4 * other.e23) + (anti_wedge.e42 * other.e3) + (anti_wedge.e23 * other.e4), (anti_wedge.e1 * other.e43) + (anti_wedge.e4 * other.e31) + (anti_wedge.e43 * other.e1) + (anti_wedge.e31 * other.e4), (anti_wedge.e2 * other.e41) + (anti_wedge.e4 * other.e12) + (anti_wedge.e41 * other.e2) + (anti_wedge.e12 * other.e4), -(anti_wedge.e1 * other.e23) - (anti_wedge.e2 * other.e31) - (anti_wedge.e3 * other.e12) - (anti_wedge.e12 * other.e3)) + (float4(anti_wedge.scalar) * other.group4) + (float4(other.scalar) * anti_wedge.group4) - (other.group1.yzxx * float4(anti_wedge.group2.zxy, anti_wedge.e23)) - float4((other.group2.zxy * anti_wedge.group1.yzx), anti_wedge.e31 * other.e2)
        );
    }
}
extension Flector: AntiProjectOrthogonallyOnto<Plane> {
    associatedtype Output = Plane;
    func anti_project_orthogonally_onto(other: Plane) -> Output {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(this.e321 * other.e321) * other.group0
        );
    }
}
extension Flector: AntiProjectOrthogonallyOnto<Point> {
    associatedtype Output = Flector;
    func anti_project_orthogonally_onto(other: Point) -> Output {
Plane right_anti_dual = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.e1, other.e2, other.e3, 0.0)
        );
Motor anti_wedge = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(((this.group1.zxy * right_anti_dual.group0.yzx) - (this.group1.yzx * right_anti_dual.group0.zxy)), 0.0), 
            /* e23, e31, e12, scalar */
            float4(this.e321 * right_anti_dual.e423 * -1.0, this.e321 * right_anti_dual.e431 * -1.0, this.e321 * right_anti_dual.e412 * -1.0, (this.e2 * right_anti_dual.e431) + (this.e3 * right_anti_dual.e412) + (this.e4 * right_anti_dual.e321)) + (right_anti_dual.group0.wwwx * float4(this.group1.xyz, this.e1))
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(anti_wedge.scalar) * other.group0, 
            /* e423, e431, e412, e321 */
            float4((anti_wedge.e42 * other.e3) + (anti_wedge.e23 * other.e4), (anti_wedge.e43 * other.e1) + (anti_wedge.e31 * other.e4), (anti_wedge.e41 * other.e2) + (anti_wedge.e12 * other.e4), -(anti_wedge.e31 * other.e2) - (anti_wedge.e12 * other.e3)) - (other.group0.yzxx * float4(anti_wedge.group0.zxy, anti_wedge.e23))
        );
    }
}
extension Flector: AntiProjectOrthogonallyOnto<Scalar> {
    associatedtype Output = Flector;
    func anti_project_orthogonally_onto(other: Scalar) -> Output {
AntiScalar right_anti_dual = AntiScalar.from_groups(
            /* e1234 */
            other.scalar
        );
Flector anti_wedge = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(right_anti_dual.e1234) * this.group0, 
            /* e423, e431, e412, e321 */
            float4(right_anti_dual.e1234) * this.group1
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * anti_wedge.group0, 
            /* e423, e431, e412, e321 */
            float4(other.scalar) * anti_wedge.group1
        );
    }
}
extension Horizon {
    // Fancy infix trick
    func operator/(_rhs: AntiProjectOrthogonallyOntoInfix) -> AntiProjectOrthogonallyOntoInfixPartial<Horizon> {
        return AntiProjectOrthogonallyOntoInfixPartial { a: this };
    }
}
extension Horizon: AntiProjectOrthogonallyOnto<DualNum> {
    associatedtype Output = Horizon;
    func anti_project_orthogonally_onto(other: DualNum) -> Output {
        return Horizon.from_groups(
            /* e321 */
            powi(other.scalar, 2) * this.e321
        );
    }
}
extension Horizon: AntiProjectOrthogonallyOnto<Flector> {
    associatedtype Output = Flector;
    func anti_project_orthogonally_onto(other: Flector) -> Output {
Flector right_anti_dual = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.e1, other.e2, other.e3, 0.0)
        );
Motor anti_wedge = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(0.0), 
            /* e23, e31, e12, scalar */
            float4(this.e321) * float4(right_anti_dual.group1.xyz, right_anti_dual.e4) * float4(-1.0)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(anti_wedge.scalar) * other.group0, 
            /* e423, e431, e412, e321 */
            float4((other.e4 * anti_wedge.e23) + (other.e423 * anti_wedge.scalar), (other.e4 * anti_wedge.e31) + (other.e431 * anti_wedge.scalar), (other.e4 * anti_wedge.e12) + (other.e412 * anti_wedge.scalar), -(other.e2 * anti_wedge.e31) - (other.e3 * anti_wedge.e12)) + float4((other.group0.zxy * anti_wedge.group0.yzx), other.e321 * anti_wedge.scalar) - (other.group0.yzxx * float4(anti_wedge.group0.zxy, anti_wedge.e23))
        );
    }
}
extension Horizon: AntiProjectOrthogonallyOnto<Horizon> {
    associatedtype Output = Horizon;
    func anti_project_orthogonally_onto(other: Horizon) -> Output {
        return Horizon.from_groups(
            /* e321 */
            powi(other.e321, 2) * this.e321
        );
    }
}
extension Horizon: AntiProjectOrthogonallyOnto<Line> {
    associatedtype Output = Plane;
    func anti_project_orthogonally_onto(other: Line) -> Output {
Point anti_wedge = Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e321, this.e321, this.e321, 0.0) * float4(float3(other.e23 * -1.0, other.e31 * -1.0, other.e12 * -1.0), 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((other.e42 * anti_wedge.e3) + (other.e23 * anti_wedge.e4), (other.e43 * anti_wedge.e1) + (other.e31 * anti_wedge.e4), (other.e41 * anti_wedge.e2) + (other.e12 * anti_wedge.e4), -(other.e31 * anti_wedge.e2) - (other.e12 * anti_wedge.e3)) - (anti_wedge.group0.yzxx * float4(other.group0.zxy, other.e23))
        );
    }
}
extension Horizon: AntiProjectOrthogonallyOnto<Motor> {
    associatedtype Output = Flector;
    func anti_project_orthogonally_onto(other: Motor) -> Output {
Motor right_anti_dual = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
Flector anti_wedge = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e321, this.e321, this.e321, 0.0) * float4(right_anti_dual.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.e321 * right_anti_dual.e1234)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * anti_wedge.group0, 
            /* e423, e431, e412, e321 */
            float4((anti_wedge.e4 * other.e23) + (anti_wedge.e423 * other.scalar), (anti_wedge.e4 * other.e31) + (anti_wedge.e431 * other.scalar), (anti_wedge.e4 * other.e12) + (anti_wedge.e412 * other.scalar), -(anti_wedge.e2 * other.e31) - (anti_wedge.e3 * other.e12)) + float4((anti_wedge.group0.zxy * other.group0.yzx), anti_wedge.e321 * other.scalar) - (anti_wedge.group0.yzxx * float4(other.group0.zxy, other.e23))
        );
    }
}
extension Horizon: AntiProjectOrthogonallyOnto<MultiVector> {
    associatedtype Output = MultiVector;
    func anti_project_orthogonally_onto(other: MultiVector) -> Output {
MultiVector right_anti_dual = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, other.scalar), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e41, e42, e43 */
            other.group3 * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(other.e1, other.e2, other.e3, 0.0)
        );
MultiVector anti_wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(this.e321 * right_anti_dual.e4, 1.0) * float2(-1.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.e321, this.e321, this.e321, 0.0) * float4(right_anti_dual.group2, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(this.e321) * right_anti_dual.group4.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.e321 * right_anti_dual.e1234)
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(anti_wedge.scalar * other.scalar, (anti_wedge.scalar * other.e1234) + (anti_wedge.e1234 * other.scalar) + (anti_wedge.e423 * other.e1) + (anti_wedge.e431 * other.e2) + (anti_wedge.e412 * other.e3) + (anti_wedge.e321 * other.e4) - (anti_wedge.e1 * other.e423) - (anti_wedge.e2 * other.e431) - (anti_wedge.e3 * other.e412) - (anti_wedge.e4 * other.e321) - (anti_wedge.e41 * other.e23) - (anti_wedge.e42 * other.e31) - (anti_wedge.e43 * other.e12) - (anti_wedge.e23 * other.e41) - (anti_wedge.e31 * other.e42) - (anti_wedge.e12 * other.e43)), 
            /* e1, e2, e3, e4 */
            (float4(anti_wedge.scalar) * other.group1) + (float4(other.scalar) * anti_wedge.group1), 
            /* e41, e42, e43 */
            (float3(anti_wedge.scalar) * other.group2) + (float3(other.scalar) * anti_wedge.group2) + (float3(other.e4) * anti_wedge.group1.xyz) - (float3(anti_wedge.e4) * other.group1.xyz), 
            /* e23, e31, e12 */
            (float3(anti_wedge.scalar) * other.group3) + (float3(other.scalar) * anti_wedge.group3) + (anti_wedge.group1.zxy * other.group1.yzx) - (anti_wedge.group1.yzx * other.group1.zxy), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge.e3 * other.e42) + (anti_wedge.e4 * other.e23) + (anti_wedge.e42 * other.e3) + (anti_wedge.e23 * other.e4), (anti_wedge.e1 * other.e43) + (anti_wedge.e4 * other.e31) + (anti_wedge.e43 * other.e1) + (anti_wedge.e31 * other.e4), (anti_wedge.e2 * other.e41) + (anti_wedge.e4 * other.e12) + (anti_wedge.e41 * other.e2) + (anti_wedge.e12 * other.e4), -(anti_wedge.e1 * other.e23) - (anti_wedge.e2 * other.e31) - (anti_wedge.e3 * other.e12) - (anti_wedge.e12 * other.e3)) + (float4(anti_wedge.scalar) * other.group4) + (float4(other.scalar) * anti_wedge.group4) - (other.group1.yzxx * float4(anti_wedge.group2.zxy, anti_wedge.e23)) - float4((other.group2.zxy * anti_wedge.group1.yzx), anti_wedge.e31 * other.e2)
        );
    }
}
extension Horizon: AntiProjectOrthogonallyOnto<Plane> {
    associatedtype Output = Plane;
    func anti_project_orthogonally_onto(other: Plane) -> Output {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(this.e321 * other.e321) * other.group0
        );
    }
}
extension Horizon: AntiProjectOrthogonallyOnto<Point> {
    associatedtype Output = Plane;
    func anti_project_orthogonally_onto(other: Point) -> Output {
Line anti_wedge = Line.from_groups(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(this.e321) * float4(other.e1, other.e2, other.e3, 0.0).xyz * float3(-1.0)
        );
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((anti_wedge.e42 * other.e3) + (anti_wedge.e23 * other.e4), (anti_wedge.e43 * other.e1) + (anti_wedge.e31 * other.e4), (anti_wedge.e41 * other.e2) + (anti_wedge.e12 * other.e4), -(anti_wedge.e31 * other.e2) - (anti_wedge.e12 * other.e3)) - (other.group0.yzxx * float4(anti_wedge.group0.zxy, anti_wedge.e23))
        );
    }
}
extension Horizon: AntiProjectOrthogonallyOnto<Scalar> {
    associatedtype Output = Horizon;
    func anti_project_orthogonally_onto(other: Scalar) -> Output {
        return Horizon.from_groups(
            /* e321 */
            powi(other.scalar, 2) * this.e321
        );
    }
}
extension Line {
    // Fancy infix trick
    func operator/(_rhs: AntiProjectOrthogonallyOntoInfix) -> AntiProjectOrthogonallyOntoInfixPartial<Line> {
        return AntiProjectOrthogonallyOntoInfixPartial { a: this };
    }
}
extension Line: AntiProjectOrthogonallyOnto<DualNum> {
    associatedtype Output = Line;
    func anti_project_orthogonally_onto(other: DualNum) -> Output {
AntiScalar right_anti_dual = AntiScalar.from_groups(
            /* e1234 */
            other.scalar
        );
Line anti_wedge = Line.from_groups(
            /* e41, e42, e43 */
            float3(right_anti_dual.e1234) * this.group0, 
            /* e23, e31, e12 */
            float3(right_anti_dual.e1234) * this.group1
        );
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(other.scalar) * anti_wedge.group0, 
            /* e23, e31, e12 */
            float3(other.scalar) * anti_wedge.group1
        );
    }
}
extension Line: AntiProjectOrthogonallyOnto<Flector> {
    associatedtype Output = Motor;
    func anti_project_orthogonally_onto(other: Flector) -> Output {
Flector right_anti_dual = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.e1, other.e2, other.e3, 0.0)
        );
Point anti_wedge = Point.from_groups(
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e412 * this.e31) + (right_anti_dual.e321 * this.e41), (right_anti_dual.e423 * this.e12) + (right_anti_dual.e321 * this.e42), (right_anti_dual.e431 * this.e23) + (right_anti_dual.e321 * this.e43), -(right_anti_dual.e431 * this.e42) - (right_anti_dual.e412 * this.e43)) - (right_anti_dual.group1.yzxx * float4(this.group1.zxy, this.e41))
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.e4 * anti_wedge.e1, other.e4 * anti_wedge.e2, other.e4 * anti_wedge.e3, -(other.e431 * anti_wedge.e2) - (other.e412 * anti_wedge.e3) - (other.e321 * anti_wedge.e4)) - (anti_wedge.group0.wwwx * float4(other.group0.xyz, other.e423)), 
            /* e23, e31, e12, scalar */
            float4(((other.group0.yzx * anti_wedge.group0.zxy) - (other.group0.zxy * anti_wedge.group0.yzx)), 0.0)
        );
    }
}
extension Line: AntiProjectOrthogonallyOnto<Line> {
    associatedtype Output = Line;
    func anti_project_orthogonally_onto(other: Line) -> Output {
Line right_anti_dual = Line.from_groups(
            /* e41, e42, e43 */
            other.group1 * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
Scalar anti_wedge = Scalar.from_groups(
            /* scalar */
            -(right_anti_dual.e41 * this.e23) - (right_anti_dual.e42 * this.e31) - (right_anti_dual.e43 * this.e12) - (right_anti_dual.e23 * this.e41) - (right_anti_dual.e31 * this.e42) - (right_anti_dual.e12 * this.e43)
        );
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(anti_wedge.scalar) * other.group0, 
            /* e23, e31, e12 */
            float3(anti_wedge.scalar) * other.group1
        );
    }
}
extension Line: AntiProjectOrthogonallyOnto<Motor> {
    associatedtype Output = Motor;
    func anti_project_orthogonally_onto(other: Motor) -> Output {
Motor right_anti_dual = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
Motor anti_wedge = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(float3(1.0), 0.0) * float4(this.group0, 0.0) * float4(right_anti_dual.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e23, e31, e12, scalar */
            float4(right_anti_dual.e1234, right_anti_dual.e1234, right_anti_dual.e1234, 1.0) * float4(this.group1, -(this.e41 * right_anti_dual.e23) - (this.e42 * right_anti_dual.e31) - (this.e43 * right_anti_dual.e12) - (this.e23 * right_anti_dual.e41) - (this.e31 * right_anti_dual.e42) - (this.e12 * right_anti_dual.e43))
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            (float4(anti_wedge.scalar) * other.group0) + (float4(other.scalar) * anti_wedge.group0) + float4(float3(0.0), -(anti_wedge.e41 * other.e23) - (anti_wedge.e42 * other.e31) - (anti_wedge.e43 * other.e12) - (anti_wedge.e23 * other.e41) - (anti_wedge.e31 * other.e42) - (anti_wedge.e12 * other.e43)), 
            /* e23, e31, e12, scalar */
            float4(((float3(anti_wedge.scalar) * other.group1.xyz) + (float3(other.scalar) * anti_wedge.group1.xyz)), anti_wedge.scalar * other.scalar)
        );
    }
}
extension Line: AntiProjectOrthogonallyOnto<MultiVector> {
    associatedtype Output = MultiVector;
    func anti_project_orthogonally_onto(other: MultiVector) -> Output {
MultiVector right_anti_dual = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, other.scalar), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e41, e42, e43 */
            other.group3 * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(other.e1, other.e2, other.e3, 0.0)
        );
MultiVector anti_wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(-(this.e41 * right_anti_dual.e23) - (this.e42 * right_anti_dual.e31) - (this.e43 * right_anti_dual.e12) - (this.e23 * right_anti_dual.e41) - (this.e31 * right_anti_dual.e42) - (this.e12 * right_anti_dual.e43), 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.e41 * right_anti_dual.e321) + (this.e31 * right_anti_dual.e412), (this.e42 * right_anti_dual.e321) + (this.e12 * right_anti_dual.e423), (this.e43 * right_anti_dual.e321) + (this.e23 * right_anti_dual.e431), -(this.e42 * right_anti_dual.e431) - (this.e43 * right_anti_dual.e412)) - (right_anti_dual.group4.yzxx * float4(this.group1.zxy, this.e41)), 
            /* e41, e42, e43 */
            float3(right_anti_dual.e1234) * this.group0, 
            /* e23, e31, e12 */
            float3(right_anti_dual.e1234) * this.group1, 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(anti_wedge.scalar * other.scalar, (anti_wedge.scalar * other.e1234) + (anti_wedge.e1234 * other.scalar) + (anti_wedge.e423 * other.e1) + (anti_wedge.e431 * other.e2) + (anti_wedge.e412 * other.e3) + (anti_wedge.e321 * other.e4) - (anti_wedge.e1 * other.e423) - (anti_wedge.e2 * other.e431) - (anti_wedge.e3 * other.e412) - (anti_wedge.e4 * other.e321) - (anti_wedge.e41 * other.e23) - (anti_wedge.e42 * other.e31) - (anti_wedge.e43 * other.e12) - (anti_wedge.e23 * other.e41) - (anti_wedge.e31 * other.e42) - (anti_wedge.e12 * other.e43)), 
            /* e1, e2, e3, e4 */
            (float4(anti_wedge.scalar) * other.group1) + (float4(other.scalar) * anti_wedge.group1), 
            /* e41, e42, e43 */
            (float3(anti_wedge.scalar) * other.group2) + (float3(other.scalar) * anti_wedge.group2) + (float3(other.e4) * anti_wedge.group1.xyz) - (float3(anti_wedge.e4) * other.group1.xyz), 
            /* e23, e31, e12 */
            (float3(anti_wedge.scalar) * other.group3) + (float3(other.scalar) * anti_wedge.group3) + (anti_wedge.group1.zxy * other.group1.yzx) - (anti_wedge.group1.yzx * other.group1.zxy), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge.e3 * other.e42) + (anti_wedge.e4 * other.e23) + (anti_wedge.e42 * other.e3) + (anti_wedge.e23 * other.e4), (anti_wedge.e1 * other.e43) + (anti_wedge.e4 * other.e31) + (anti_wedge.e43 * other.e1) + (anti_wedge.e31 * other.e4), (anti_wedge.e2 * other.e41) + (anti_wedge.e4 * other.e12) + (anti_wedge.e41 * other.e2) + (anti_wedge.e12 * other.e4), -(anti_wedge.e1 * other.e23) - (anti_wedge.e2 * other.e31) - (anti_wedge.e3 * other.e12) - (anti_wedge.e12 * other.e3)) + (float4(anti_wedge.scalar) * other.group4) + (float4(other.scalar) * anti_wedge.group4) - (other.group1.yzxx * float4(anti_wedge.group2.zxy, anti_wedge.e23)) - float4((other.group2.zxy * anti_wedge.group1.yzx), anti_wedge.e31 * other.e2)
        );
    }
}
extension Line: AntiProjectOrthogonallyOnto<Point> {
    associatedtype Output = Line;
    func anti_project_orthogonally_onto(other: Point) -> Output {
Plane right_anti_dual = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.e1, other.e2, other.e3, 0.0)
        );
Point anti_wedge = Point.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e41 * right_anti_dual.e321) + (this.e31 * right_anti_dual.e412), (this.e42 * right_anti_dual.e321) + (this.e12 * right_anti_dual.e423), (this.e43 * right_anti_dual.e321) + (this.e23 * right_anti_dual.e431), -(this.e42 * right_anti_dual.e431) - (this.e43 * right_anti_dual.e412)) - (right_anti_dual.group0.yzxx * float4(this.group1.zxy, this.e41))
        );
        return Line.from_groups(
            /* e41, e42, e43 */
            (float3(other.e4) * anti_wedge.group0.xyz) - (float3(anti_wedge.e4) * other.group0.xyz), 
            /* e23, e31, e12 */
            (anti_wedge.group0.zxy * other.group0.yzx) - (anti_wedge.group0.yzx * other.group0.zxy)
        );
    }
}
extension Line: AntiProjectOrthogonallyOnto<Scalar> {
    associatedtype Output = Line;
    func anti_project_orthogonally_onto(other: Scalar) -> Output {
AntiScalar right_anti_dual = AntiScalar.from_groups(
            /* e1234 */
            other.scalar
        );
Line anti_wedge = Line.from_groups(
            /* e41, e42, e43 */
            float3(right_anti_dual.e1234) * this.group0, 
            /* e23, e31, e12 */
            float3(right_anti_dual.e1234) * this.group1
        );
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(other.scalar) * anti_wedge.group0, 
            /* e23, e31, e12 */
            float3(other.scalar) * anti_wedge.group1
        );
    }
}
extension Motor {
    // Fancy infix trick
    func operator/(_rhs: AntiProjectOrthogonallyOntoInfix) -> AntiProjectOrthogonallyOntoInfixPartial<Motor> {
        return AntiProjectOrthogonallyOntoInfixPartial { a: this };
    }
}
extension Motor: AntiProjectOrthogonallyOnto<DualNum> {
    associatedtype Output = Motor;
    func anti_project_orthogonally_onto(other: DualNum) -> Output {
AntiScalar right_anti_dual = AntiScalar.from_groups(
            /* e1234 */
            other.scalar
        );
Motor anti_wedge = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(right_anti_dual.e1234) * this.group0, 
            /* e23, e31, e12, scalar */
            float4(right_anti_dual.e1234) * this.group1
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(anti_wedge.e41, anti_wedge.e42, anti_wedge.e43, 1.0) * float4(other.group0.xx, other.scalar, (other.scalar * anti_wedge.e1234) + (other.e1234 * anti_wedge.scalar)), 
            /* e23, e31, e12, scalar */
            float4(other.scalar) * anti_wedge.group1
        );
    }
}
extension Motor: AntiProjectOrthogonallyOnto<Flector> {
    associatedtype Output = Motor;
    func anti_project_orthogonally_onto(other: Flector) -> Output {
Flector right_anti_dual = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.e1, other.e2, other.e3, 0.0)
        );
Flector anti_wedge = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e412 * this.e31) + (right_anti_dual.e321 * this.e41), (right_anti_dual.e423 * this.e12) + (right_anti_dual.e321 * this.e42), (right_anti_dual.e431 * this.e23) + (right_anti_dual.e321 * this.e43), -(right_anti_dual.e431 * this.e42) - (right_anti_dual.e412 * this.e43)) + (float4(this.e1234) * right_anti_dual.group0) - (right_anti_dual.group1.yzxx * float4(this.group1.zxy, this.e41)), 
            /* e423, e431, e412, e321 */
            float4(this.e1234) * right_anti_dual.group1
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            (other.group0.wwwx * float4(anti_wedge.group0.xyz, anti_wedge.e423)) + float4(float3(0.0), (anti_wedge.e431 * other.e2) + (anti_wedge.e412 * other.e3) + (anti_wedge.e321 * other.e4) - (anti_wedge.e2 * other.e431) - (anti_wedge.e3 * other.e412) - (anti_wedge.e4 * other.e321)) - (anti_wedge.group0.wwwx * float4(other.group0.xyz, other.e423)), 
            /* e23, e31, e12, scalar */
            float4(((anti_wedge.group0.zxy * other.group0.yzx) - (anti_wedge.group0.yzx * other.group0.zxy)), 0.0)
        );
    }
}
extension Motor: AntiProjectOrthogonallyOnto<Horizon> {
    associatedtype Output = AntiScalar;
    func anti_project_orthogonally_onto(other: Horizon) -> Output {
        return AntiScalar.from_groups(
            /* e1234 */
            powi(other.e321, 2) * this.e1234
        );
    }
}
extension Motor: AntiProjectOrthogonallyOnto<Line> {
    associatedtype Output = Motor;
    func anti_project_orthogonally_onto(other: Line) -> Output {
Line right_anti_dual = Line.from_groups(
            /* e41, e42, e43 */
            other.group1 * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
Motor anti_wedge = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(float3(1.0), 0.0) * float4(right_anti_dual.group0, 0.0) * float4(this.group0.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e23, e31, e12, scalar */
            float4(this.e1234, this.e1234, this.e1234, 1.0) * float4(right_anti_dual.group1, -(right_anti_dual.e41 * this.e23) - (right_anti_dual.e42 * this.e31) - (right_anti_dual.e43 * this.e12) - (right_anti_dual.e23 * this.e41) - (right_anti_dual.e31 * this.e42) - (right_anti_dual.e12 * this.e43))
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(anti_wedge.scalar, anti_wedge.scalar, anti_wedge.scalar, 1.0) * float4(other.group0, -(other.e41 * anti_wedge.e23) - (other.e42 * anti_wedge.e31) - (other.e43 * anti_wedge.e12) - (other.e23 * anti_wedge.e41) - (other.e31 * anti_wedge.e42) - (other.e12 * anti_wedge.e43)), 
            /* e23, e31, e12, scalar */
            float4(float3(1.0), 0.0) * float4(other.group1, 0.0) * float4(anti_wedge.group1.www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Motor: AntiProjectOrthogonallyOnto<Motor> {
    associatedtype Output = Motor;
    func anti_project_orthogonally_onto(other: Motor) -> Output {
Motor right_anti_dual = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
Motor anti_wedge = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(((float3(right_anti_dual.e1234) * this.group0.xyz) + (float3(this.e1234) * right_anti_dual.group0.xyz)), right_anti_dual.e1234 * this.e1234), 
            /* e23, e31, e12, scalar */
            (float4(right_anti_dual.e1234) * this.group1) + (float4(this.e1234) * right_anti_dual.group1) + float4(float3(0.0), -(right_anti_dual.e41 * this.e23) - (right_anti_dual.e42 * this.e31) - (right_anti_dual.e43 * this.e12) - (right_anti_dual.e23 * this.e41) - (right_anti_dual.e31 * this.e42) - (right_anti_dual.e12 * this.e43))
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            (float4(anti_wedge.scalar) * other.group0) + (float4(other.scalar) * anti_wedge.group0) + float4(float3(0.0), -(anti_wedge.e41 * other.e23) - (anti_wedge.e42 * other.e31) - (anti_wedge.e43 * other.e12) - (anti_wedge.e23 * other.e41) - (anti_wedge.e31 * other.e42) - (anti_wedge.e12 * other.e43)), 
            /* e23, e31, e12, scalar */
            float4(((float3(anti_wedge.scalar) * other.group1.xyz) + (float3(other.scalar) * anti_wedge.group1.xyz)), anti_wedge.scalar * other.scalar)
        );
    }
}
extension Motor: AntiProjectOrthogonallyOnto<MultiVector> {
    associatedtype Output = MultiVector;
    func anti_project_orthogonally_onto(other: MultiVector) -> Output {
MultiVector right_anti_dual = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, other.scalar), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e41, e42, e43 */
            other.group3 * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(other.e1, other.e2, other.e3, 0.0)
        );
MultiVector anti_wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2((this.e1234 * right_anti_dual.scalar) + (this.scalar * right_anti_dual.e1234) - (this.e41 * right_anti_dual.e23) - (this.e42 * right_anti_dual.e31) - (this.e43 * right_anti_dual.e12) - (this.e23 * right_anti_dual.e41) - (this.e31 * right_anti_dual.e42) - (this.e12 * right_anti_dual.e43), this.e1234 * right_anti_dual.e1234), 
            /* e1, e2, e3, e4 */
            float4((this.e1234 * right_anti_dual.e1) + (this.e31 * right_anti_dual.e412), (this.e1234 * right_anti_dual.e2) + (this.e12 * right_anti_dual.e423), (this.e1234 * right_anti_dual.e3) + (this.e23 * right_anti_dual.e431), -(this.e42 * right_anti_dual.e431) - (this.e43 * right_anti_dual.e412)) + (this.group0 * float4(right_anti_dual.group4.www, right_anti_dual.e4)) - (right_anti_dual.group4.yzxx * float4(this.group1.zxy, this.e41)), 
            /* e41, e42, e43 */
            (float3(this.e1234) * right_anti_dual.group2) + (float3(right_anti_dual.e1234) * this.group0.xyz), 
            /* e23, e31, e12 */
            (float3(this.e1234) * right_anti_dual.group3) + (float3(right_anti_dual.e1234) * this.group1.xyz), 
            /* e423, e431, e412, e321 */
            float4(this.e1234) * right_anti_dual.group4
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(anti_wedge.scalar * other.scalar, (anti_wedge.scalar * other.e1234) + (anti_wedge.e1234 * other.scalar) + (anti_wedge.e423 * other.e1) + (anti_wedge.e431 * other.e2) + (anti_wedge.e412 * other.e3) + (anti_wedge.e321 * other.e4) - (anti_wedge.e1 * other.e423) - (anti_wedge.e2 * other.e431) - (anti_wedge.e3 * other.e412) - (anti_wedge.e4 * other.e321) - (anti_wedge.e41 * other.e23) - (anti_wedge.e42 * other.e31) - (anti_wedge.e43 * other.e12) - (anti_wedge.e23 * other.e41) - (anti_wedge.e31 * other.e42) - (anti_wedge.e12 * other.e43)), 
            /* e1, e2, e3, e4 */
            (float4(anti_wedge.scalar) * other.group1) + (float4(other.scalar) * anti_wedge.group1), 
            /* e41, e42, e43 */
            (float3(anti_wedge.scalar) * other.group2) + (float3(other.scalar) * anti_wedge.group2) + (float3(other.e4) * anti_wedge.group1.xyz) - (float3(anti_wedge.e4) * other.group1.xyz), 
            /* e23, e31, e12 */
            (float3(anti_wedge.scalar) * other.group3) + (float3(other.scalar) * anti_wedge.group3) + (anti_wedge.group1.zxy * other.group1.yzx) - (anti_wedge.group1.yzx * other.group1.zxy), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge.e3 * other.e42) + (anti_wedge.e4 * other.e23) + (anti_wedge.e42 * other.e3) + (anti_wedge.e23 * other.e4), (anti_wedge.e1 * other.e43) + (anti_wedge.e4 * other.e31) + (anti_wedge.e43 * other.e1) + (anti_wedge.e31 * other.e4), (anti_wedge.e2 * other.e41) + (anti_wedge.e4 * other.e12) + (anti_wedge.e41 * other.e2) + (anti_wedge.e12 * other.e4), -(anti_wedge.e1 * other.e23) - (anti_wedge.e2 * other.e31) - (anti_wedge.e3 * other.e12) - (anti_wedge.e12 * other.e3)) + (float4(anti_wedge.scalar) * other.group4) + (float4(other.scalar) * anti_wedge.group4) - (other.group1.yzxx * float4(anti_wedge.group2.zxy, anti_wedge.e23)) - float4((other.group2.zxy * anti_wedge.group1.yzx), anti_wedge.e31 * other.e2)
        );
    }
}
extension Motor: AntiProjectOrthogonallyOnto<Plane> {
    associatedtype Output = AntiScalar;
    func anti_project_orthogonally_onto(other: Plane) -> Output {
        return AntiScalar.from_groups(
            /* e1234 */
            powi(other.e321, 2) * this.e1234
        );
    }
}
extension Motor: AntiProjectOrthogonallyOnto<Point> {
    associatedtype Output = Motor;
    func anti_project_orthogonally_onto(other: Point) -> Output {
Plane right_anti_dual = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.e1, other.e2, other.e3, 0.0)
        );
Flector anti_wedge = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e41 * right_anti_dual.e321) + (this.e31 * right_anti_dual.e412), (this.e42 * right_anti_dual.e321) + (this.e12 * right_anti_dual.e423), (this.e43 * right_anti_dual.e321) + (this.e23 * right_anti_dual.e431), -(this.e42 * right_anti_dual.e431) - (this.e43 * right_anti_dual.e412)) - (right_anti_dual.group0.yzxx * float4(this.group1.zxy, this.e41)), 
            /* e423, e431, e412, e321 */
            float4(this.e1234) * right_anti_dual.group0
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(anti_wedge.e4 * other.e1 * -1.0, anti_wedge.e4 * other.e2 * -1.0, anti_wedge.e4 * other.e3 * -1.0, (anti_wedge.e431 * other.e2) + (anti_wedge.e412 * other.e3) + (anti_wedge.e321 * other.e4)) + (other.group0.wwwx * float4(anti_wedge.group0.xyz, anti_wedge.e423)), 
            /* e23, e31, e12, scalar */
            float4(((anti_wedge.group0.zxy * other.group0.yzx) - (anti_wedge.group0.yzx * other.group0.zxy)), 0.0)
        );
    }
}
extension Motor: AntiProjectOrthogonallyOnto<Scalar> {
    associatedtype Output = Motor;
    func anti_project_orthogonally_onto(other: Scalar) -> Output {
AntiScalar right_anti_dual = AntiScalar.from_groups(
            /* e1234 */
            other.scalar
        );
Motor anti_wedge = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(right_anti_dual.e1234) * this.group0, 
            /* e23, e31, e12, scalar */
            float4(right_anti_dual.e1234) * this.group1
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.scalar) * anti_wedge.group0, 
            /* e23, e31, e12, scalar */
            float4(other.scalar) * anti_wedge.group1
        );
    }
}
extension MultiVector {
    // Fancy infix trick
    func operator/(_rhs: AntiProjectOrthogonallyOntoInfix) -> AntiProjectOrthogonallyOntoInfixPartial<MultiVector> {
        return AntiProjectOrthogonallyOntoInfixPartial { a: this };
    }
}
extension MultiVector: AntiProjectOrthogonallyOnto<DualNum> {
    associatedtype Output = MultiVector;
    func anti_project_orthogonally_onto(other: DualNum) -> Output {
AntiScalar right_anti_dual = AntiScalar.from_groups(
            /* e1234 */
            other.scalar
        );
MultiVector anti_wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(right_anti_dual.e1234) * this.group0, 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual.e1234) * this.group1, 
            /* e41, e42, e43 */
            float3(right_anti_dual.e1234) * this.group2, 
            /* e23, e31, e12 */
            float3(right_anti_dual.e1234) * this.group3, 
            /* e423, e431, e412, e321 */
            float4(right_anti_dual.e1234) * this.group4
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other.scalar * anti_wedge.scalar, (other.scalar * anti_wedge.e1234) + (other.e1234 * anti_wedge.scalar)), 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * anti_wedge.group1, 
            /* e41, e42, e43 */
            float3(other.scalar) * anti_wedge.group2, 
            /* e23, e31, e12 */
            float3(other.scalar) * anti_wedge.group3, 
            /* e423, e431, e412, e321 */
            float4(other.scalar) * anti_wedge.group4
        );
    }
}
extension MultiVector: AntiProjectOrthogonallyOnto<Flector> {
    associatedtype Output = MultiVector;
    func anti_project_orthogonally_onto(other: Flector) -> Output {
Flector right_anti_dual = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.e1, other.e2, other.e3, 0.0)
        );
MultiVector anti_wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2((right_anti_dual.e423 * this.e1) + (right_anti_dual.e431 * this.e2) + (right_anti_dual.e412 * this.e3) + (right_anti_dual.e321 * this.e4) - (right_anti_dual.e1 * this.e423) - (right_anti_dual.e2 * this.e431) - (right_anti_dual.e3 * this.e412) - (right_anti_dual.e4 * this.e321), 0.0), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e412 * this.e31) + (right_anti_dual.e321 * this.e41), (right_anti_dual.e423 * this.e12) + (right_anti_dual.e321 * this.e42), (right_anti_dual.e431 * this.e23) + (right_anti_dual.e321 * this.e43), -(right_anti_dual.e431 * this.e42) - (right_anti_dual.e412 * this.e43)) + (float4(this.e1234) * right_anti_dual.group0) - (right_anti_dual.group1.yzxx * float4(this.group3.zxy, this.e41)), 
            /* e41, e42, e43 */
            (right_anti_dual.group1.yzx * this.group4.zxy) - (right_anti_dual.group1.zxy * this.group4.yzx), 
            /* e23, e31, e12 */
            (float3(right_anti_dual.e321) * this.group4.xyz) - (float3(this.e321) * right_anti_dual.group1.xyz), 
            /* e423, e431, e412, e321 */
            float4(this.e1234) * right_anti_dual.group1
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, (other.e1 * anti_wedge.e423) + (other.e2 * anti_wedge.e431) + (other.e3 * anti_wedge.e412) + (other.e4 * anti_wedge.e321) - (other.e423 * anti_wedge.e1) - (other.e431 * anti_wedge.e2) - (other.e412 * anti_wedge.e3) - (other.e321 * anti_wedge.e4)), 
            /* e1, e2, e3, e4 */
            float4(anti_wedge.scalar) * other.group0, 
            /* e41, e42, e43 */
            (float3(other.e4) * anti_wedge.group1.xyz) - (float3(anti_wedge.e4) * other.group0.xyz), 
            /* e23, e31, e12 */
            (other.group0.yzx * anti_wedge.group1.zxy) - (other.group0.zxy * anti_wedge.group1.yzx), 
            /* e423, e431, e412, e321 */
            float4((other.e3 * anti_wedge.e42) + (other.e4 * anti_wedge.e23), (other.e1 * anti_wedge.e43) + (other.e4 * anti_wedge.e31), (other.e2 * anti_wedge.e41) + (other.e4 * anti_wedge.e12), -(other.e2 * anti_wedge.e31) - (other.e3 * anti_wedge.e12)) + (float4(anti_wedge.scalar) * other.group1) - (other.group0.yzxx * float4(anti_wedge.group2.zxy, anti_wedge.e23))
        );
    }
}
extension MultiVector: AntiProjectOrthogonallyOnto<Horizon> {
    associatedtype Output = MultiVector;
    func anti_project_orthogonally_onto(other: Horizon) -> Output {
Origin right_anti_dual = Origin.from_groups(
            /* e4 */
            other.e321 * -1.0
        );
MultiVector anti_wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(this.e321 * right_anti_dual.e4, 1.0) * float2(-1.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e1234 * right_anti_dual.e4), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(1.0, other.e321 * anti_wedge.e4) * float2(0.0, -1.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other.e321 * anti_wedge.scalar)
        );
    }
}
extension MultiVector: AntiProjectOrthogonallyOnto<Line> {
    associatedtype Output = MultiVector;
    func anti_project_orthogonally_onto(other: Line) -> Output {
Line right_anti_dual = Line.from_groups(
            /* e41, e42, e43 */
            other.group1 * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
MultiVector anti_wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(-(right_anti_dual.e41 * this.e23) - (right_anti_dual.e42 * this.e31) - (right_anti_dual.e43 * this.e12) - (right_anti_dual.e23 * this.e41) - (right_anti_dual.e31 * this.e42) - (right_anti_dual.e12 * this.e43), 0.0), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e41 * this.e321) + (right_anti_dual.e31 * this.e412), (right_anti_dual.e42 * this.e321) + (right_anti_dual.e12 * this.e423), (right_anti_dual.e43 * this.e321) + (right_anti_dual.e23 * this.e431), -(right_anti_dual.e42 * this.e431) - (right_anti_dual.e43 * this.e412)) - (this.group4.yzxx * float4(right_anti_dual.group1.zxy, right_anti_dual.e41)), 
            /* e41, e42, e43 */
            float3(this.e1234) * right_anti_dual.group0, 
            /* e23, e31, e12 */
            float3(this.e1234) * right_anti_dual.group1, 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, -(other.e41 * anti_wedge.e23) - (other.e42 * anti_wedge.e31) - (other.e43 * anti_wedge.e12) - (other.e23 * anti_wedge.e41) - (other.e31 * anti_wedge.e42) - (other.e12 * anti_wedge.e43)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(anti_wedge.scalar) * other.group0, 
            /* e23, e31, e12 */
            float3(anti_wedge.scalar) * other.group1, 
            /* e423, e431, e412, e321 */
            float4((other.e42 * anti_wedge.e3) + (other.e23 * anti_wedge.e4), (other.e43 * anti_wedge.e1) + (other.e31 * anti_wedge.e4), (other.e41 * anti_wedge.e2) + (other.e12 * anti_wedge.e4), -(other.e31 * anti_wedge.e2) - (other.e12 * anti_wedge.e3)) - (anti_wedge.group1.yzxx * float4(other.group0.zxy, other.e23))
        );
    }
}
extension MultiVector: AntiProjectOrthogonallyOnto<Motor> {
    associatedtype Output = MultiVector;
    func anti_project_orthogonally_onto(other: Motor) -> Output {
Motor right_anti_dual = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
MultiVector anti_wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2((right_anti_dual.e1234 * this.scalar) + (right_anti_dual.scalar * this.e1234) - (right_anti_dual.e41 * this.e23) - (right_anti_dual.e42 * this.e31) - (right_anti_dual.e43 * this.e12) - (right_anti_dual.e23 * this.e41) - (right_anti_dual.e31 * this.e42) - (right_anti_dual.e12 * this.e43), right_anti_dual.e1234 * this.e1234), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e1234 * this.e1) + (right_anti_dual.e31 * this.e412), (right_anti_dual.e1234 * this.e2) + (right_anti_dual.e12 * this.e423), (right_anti_dual.e1234 * this.e3) + (right_anti_dual.e23 * this.e431), -(right_anti_dual.e42 * this.e431) - (right_anti_dual.e43 * this.e412)) + (right_anti_dual.group0 * float4(this.group4.www, this.e4)) - (this.group4.yzxx * float4(right_anti_dual.group1.zxy, right_anti_dual.e41)), 
            /* e41, e42, e43 */
            (float3(right_anti_dual.e1234) * this.group2) + (float3(this.e1234) * right_anti_dual.group0.xyz), 
            /* e23, e31, e12 */
            (float3(right_anti_dual.e1234) * this.group3) + (float3(this.e1234) * right_anti_dual.group1.xyz), 
            /* e423, e431, e412, e321 */
            float4(right_anti_dual.e1234) * this.group4
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other.scalar * anti_wedge.scalar, (other.e1234 * anti_wedge.scalar) + (other.scalar * anti_wedge.e1234) - (other.e41 * anti_wedge.e23) - (other.e42 * anti_wedge.e31) - (other.e43 * anti_wedge.e12) - (other.e23 * anti_wedge.e41) - (other.e31 * anti_wedge.e42) - (other.e12 * anti_wedge.e43)), 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * anti_wedge.group1, 
            /* e41, e42, e43 */
            (float3(other.scalar) * anti_wedge.group2) + (float3(anti_wedge.scalar) * other.group0.xyz), 
            /* e23, e31, e12 */
            (float3(other.scalar) * anti_wedge.group3) + (float3(anti_wedge.scalar) * other.group1.xyz), 
            /* e423, e431, e412, e321 */
            float4((other.e23 * anti_wedge.e4) + (other.scalar * anti_wedge.e423), (other.e31 * anti_wedge.e4) + (other.scalar * anti_wedge.e431), (other.e12 * anti_wedge.e4) + (other.scalar * anti_wedge.e412), -(other.e31 * anti_wedge.e2) - (other.e12 * anti_wedge.e3)) + float4((other.group0.yzx * anti_wedge.group1.zxy), other.scalar * anti_wedge.e321) - (anti_wedge.group1.yzxx * float4(other.group0.zxy, other.e23))
        );
    }
}
extension MultiVector: AntiProjectOrthogonallyOnto<MultiVector> {
    associatedtype Output = MultiVector;
    func anti_project_orthogonally_onto(other: MultiVector) -> Output {
MultiVector right_anti_dual = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, other.scalar), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e41, e42, e43 */
            other.group3 * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(other.e1, other.e2, other.e3, 0.0)
        );
MultiVector anti_wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2((right_anti_dual.scalar * this.e1234) + (right_anti_dual.e1234 * this.scalar) + (right_anti_dual.e423 * this.e1) + (right_anti_dual.e431 * this.e2) + (right_anti_dual.e412 * this.e3) + (right_anti_dual.e321 * this.e4) - (right_anti_dual.e1 * this.e423) - (right_anti_dual.e2 * this.e431) - (right_anti_dual.e3 * this.e412) - (right_anti_dual.e4 * this.e321) - (right_anti_dual.e41 * this.e23) - (right_anti_dual.e42 * this.e31) - (right_anti_dual.e43 * this.e12) - (right_anti_dual.e23 * this.e41) - (right_anti_dual.e31 * this.e42) - (right_anti_dual.e12 * this.e43), right_anti_dual.e1234 * this.e1234), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e41 * this.e321) + (right_anti_dual.e31 * this.e412) + (right_anti_dual.e412 * this.e31) + (right_anti_dual.e321 * this.e41), (right_anti_dual.e42 * this.e321) + (right_anti_dual.e12 * this.e423) + (right_anti_dual.e423 * this.e12) + (right_anti_dual.e321 * this.e42), (right_anti_dual.e43 * this.e321) + (right_anti_dual.e23 * this.e431) + (right_anti_dual.e431 * this.e23) + (right_anti_dual.e321 * this.e43), -(right_anti_dual.e43 * this.e412) - (right_anti_dual.e423 * this.e41) - (right_anti_dual.e431 * this.e42) - (right_anti_dual.e412 * this.e43)) + (float4(right_anti_dual.e1234) * this.group1) + (float4(this.e1234) * right_anti_dual.group1) - (this.group4.yzxx * float4(right_anti_dual.group3.zxy, right_anti_dual.e41)) - float4((this.group3.zxy * right_anti_dual.group4.yzx), right_anti_dual.e42 * this.e431), 
            /* e41, e42, e43 */
            (float3(right_anti_dual.e1234) * this.group2) + (float3(this.e1234) * right_anti_dual.group2) + (right_anti_dual.group4.yzx * this.group4.zxy) - (right_anti_dual.group4.zxy * this.group4.yzx), 
            /* e23, e31, e12 */
            (float3(right_anti_dual.e1234) * this.group3) + (float3(right_anti_dual.e321) * this.group4.xyz) + (float3(this.e1234) * right_anti_dual.group3) - (float3(this.e321) * right_anti_dual.group4.xyz), 
            /* e423, e431, e412, e321 */
            (float4(right_anti_dual.e1234) * this.group4) + (float4(this.e1234) * right_anti_dual.group4)
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(anti_wedge.scalar * other.scalar, (anti_wedge.scalar * other.e1234) + (anti_wedge.e1234 * other.scalar) + (anti_wedge.e423 * other.e1) + (anti_wedge.e431 * other.e2) + (anti_wedge.e412 * other.e3) + (anti_wedge.e321 * other.e4) - (anti_wedge.e1 * other.e423) - (anti_wedge.e2 * other.e431) - (anti_wedge.e3 * other.e412) - (anti_wedge.e4 * other.e321) - (anti_wedge.e41 * other.e23) - (anti_wedge.e42 * other.e31) - (anti_wedge.e43 * other.e12) - (anti_wedge.e23 * other.e41) - (anti_wedge.e31 * other.e42) - (anti_wedge.e12 * other.e43)), 
            /* e1, e2, e3, e4 */
            (float4(anti_wedge.scalar) * other.group1) + (float4(other.scalar) * anti_wedge.group1), 
            /* e41, e42, e43 */
            (float3(anti_wedge.scalar) * other.group2) + (float3(other.scalar) * anti_wedge.group2) + (float3(other.e4) * anti_wedge.group1.xyz) - (float3(anti_wedge.e4) * other.group1.xyz), 
            /* e23, e31, e12 */
            (float3(anti_wedge.scalar) * other.group3) + (float3(other.scalar) * anti_wedge.group3) + (anti_wedge.group1.zxy * other.group1.yzx) - (anti_wedge.group1.yzx * other.group1.zxy), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge.e3 * other.e42) + (anti_wedge.e4 * other.e23) + (anti_wedge.e42 * other.e3) + (anti_wedge.e23 * other.e4), (anti_wedge.e1 * other.e43) + (anti_wedge.e4 * other.e31) + (anti_wedge.e43 * other.e1) + (anti_wedge.e31 * other.e4), (anti_wedge.e2 * other.e41) + (anti_wedge.e4 * other.e12) + (anti_wedge.e41 * other.e2) + (anti_wedge.e12 * other.e4), -(anti_wedge.e1 * other.e23) - (anti_wedge.e2 * other.e31) - (anti_wedge.e3 * other.e12) - (anti_wedge.e12 * other.e3)) + (float4(anti_wedge.scalar) * other.group4) + (float4(other.scalar) * anti_wedge.group4) - (other.group1.yzxx * float4(anti_wedge.group2.zxy, anti_wedge.e23)) - float4((other.group2.zxy * anti_wedge.group1.yzx), anti_wedge.e31 * other.e2)
        );
    }
}
extension MultiVector: AntiProjectOrthogonallyOnto<Plane> {
    associatedtype Output = MultiVector;
    func anti_project_orthogonally_onto(other: Plane) -> Output {
Origin right_anti_dual = Origin.from_groups(
            /* e4 */
            other.e321 * -1.0
        );
MultiVector anti_wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(this.e321 * right_anti_dual.e4, 1.0) * float2(-1.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e1234 * right_anti_dual.e4), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, -(anti_wedge.e1 * other.e423) - (anti_wedge.e2 * other.e431) - (anti_wedge.e3 * other.e412) - (anti_wedge.e4 * other.e321)), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(anti_wedge.scalar) * other.group0
        );
    }
}
extension MultiVector: AntiProjectOrthogonallyOnto<Point> {
    associatedtype Output = MultiVector;
    func anti_project_orthogonally_onto(other: Point) -> Output {
Plane right_anti_dual = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.e1, other.e2, other.e3, 0.0)
        );
MultiVector anti_wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2((this.e1 * right_anti_dual.e423) + (this.e2 * right_anti_dual.e431) + (this.e3 * right_anti_dual.e412) + (this.e4 * right_anti_dual.e321), 0.0), 
            /* e1, e2, e3, e4 */
            float4((this.e41 * right_anti_dual.e321) + (this.e31 * right_anti_dual.e412), (this.e42 * right_anti_dual.e321) + (this.e12 * right_anti_dual.e423), (this.e43 * right_anti_dual.e321) + (this.e23 * right_anti_dual.e431), -(this.e42 * right_anti_dual.e431) - (this.e43 * right_anti_dual.e412)) - (right_anti_dual.group0.yzxx * float4(this.group3.zxy, this.e41)), 
            /* e41, e42, e43 */
            (this.group4.zxy * right_anti_dual.group0.yzx) - (this.group4.yzx * right_anti_dual.group0.zxy), 
            /* e23, e31, e12 */
            (float3(right_anti_dual.e321) * this.group4.xyz) - (float3(this.e321) * right_anti_dual.group0.xyz), 
            /* e423, e431, e412, e321 */
            float4(this.e1234) * right_anti_dual.group0
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, (anti_wedge.e423 * other.e1) + (anti_wedge.e431 * other.e2) + (anti_wedge.e412 * other.e3) + (anti_wedge.e321 * other.e4)), 
            /* e1, e2, e3, e4 */
            float4(anti_wedge.scalar) * other.group0, 
            /* e41, e42, e43 */
            (float3(other.e4) * anti_wedge.group1.xyz) - (float3(anti_wedge.e4) * other.group0.xyz), 
            /* e23, e31, e12 */
            (anti_wedge.group1.zxy * other.group0.yzx) - (anti_wedge.group1.yzx * other.group0.zxy), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge.e42 * other.e3) + (anti_wedge.e23 * other.e4), (anti_wedge.e43 * other.e1) + (anti_wedge.e31 * other.e4), (anti_wedge.e41 * other.e2) + (anti_wedge.e12 * other.e4), -(anti_wedge.e31 * other.e2) - (anti_wedge.e12 * other.e3)) - (other.group0.yzxx * float4(anti_wedge.group2.zxy, anti_wedge.e23))
        );
    }
}
extension MultiVector: AntiProjectOrthogonallyOnto<Scalar> {
    associatedtype Output = MultiVector;
    func anti_project_orthogonally_onto(other: Scalar) -> Output {
AntiScalar right_anti_dual = AntiScalar.from_groups(
            /* e1234 */
            other.scalar
        );
MultiVector anti_wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(right_anti_dual.e1234) * this.group0, 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual.e1234) * this.group1, 
            /* e41, e42, e43 */
            float3(right_anti_dual.e1234) * this.group2, 
            /* e23, e31, e12 */
            float3(right_anti_dual.e1234) * this.group3, 
            /* e423, e431, e412, e321 */
            float4(right_anti_dual.e1234) * this.group4
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other.scalar) * anti_wedge.group0, 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * anti_wedge.group1, 
            /* e41, e42, e43 */
            float3(other.scalar) * anti_wedge.group2, 
            /* e23, e31, e12 */
            float3(other.scalar) * anti_wedge.group3, 
            /* e423, e431, e412, e321 */
            float4(other.scalar) * anti_wedge.group4
        );
    }
}
extension Origin {
    // Fancy infix trick
    func operator/(_rhs: AntiProjectOrthogonallyOntoInfix) -> AntiProjectOrthogonallyOntoInfixPartial<Origin> {
        return AntiProjectOrthogonallyOntoInfixPartial { a: this };
    }
}
extension Origin: AntiProjectOrthogonallyOnto<DualNum> {
    associatedtype Output = Origin;
    func anti_project_orthogonally_onto(other: DualNum) -> Output {
        return Origin.from_groups(
            /* e4 */
            powi(other.scalar, 2) * this.e4
        );
    }
}
extension Origin: AntiProjectOrthogonallyOnto<Flector> {
    associatedtype Output = Flector;
    func anti_project_orthogonally_onto(other: Flector) -> Output {
Scalar anti_wedge = Scalar.from_groups(
            /* scalar */
            this.e4 * 0.0
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(anti_wedge.scalar) * other.group0, 
            /* e423, e431, e412, e321 */
            float4(anti_wedge.scalar) * other.group1
        );
    }
}
extension Origin: AntiProjectOrthogonallyOnto<Motor> {
    associatedtype Output = Flector;
    func anti_project_orthogonally_onto(other: Motor) -> Output {
Origin anti_wedge = Origin.from_groups(
            /* e4 */
            (other.group1 * float4(-1.0, -1.0, -1.0, 1.0))[3] * this.e4
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.scalar * anti_wedge.e4), 
            /* e423, e431, e412, e321 */
            float4(anti_wedge.e4, anti_wedge.e4, anti_wedge.e4, 0.0) * float4(other.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Origin: AntiProjectOrthogonallyOnto<MultiVector> {
    associatedtype Output = MultiVector;
    func anti_project_orthogonally_onto(other: MultiVector) -> Output {
MultiVector right_anti_dual = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, other.scalar), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e41, e42, e43 */
            other.group3 * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(other.e1, other.e2, other.e3, 0.0)
        );
MultiVector anti_wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(right_anti_dual.e321 * this.e4, 1.0) * float2(1.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), right_anti_dual.e1234 * this.e4), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(anti_wedge.scalar * other.scalar, (anti_wedge.scalar * other.e1234) + (anti_wedge.e1234 * other.scalar) + (anti_wedge.e423 * other.e1) + (anti_wedge.e431 * other.e2) + (anti_wedge.e412 * other.e3) + (anti_wedge.e321 * other.e4) - (anti_wedge.e1 * other.e423) - (anti_wedge.e2 * other.e431) - (anti_wedge.e3 * other.e412) - (anti_wedge.e4 * other.e321) - (anti_wedge.e41 * other.e23) - (anti_wedge.e42 * other.e31) - (anti_wedge.e43 * other.e12) - (anti_wedge.e23 * other.e41) - (anti_wedge.e31 * other.e42) - (anti_wedge.e12 * other.e43)), 
            /* e1, e2, e3, e4 */
            (float4(anti_wedge.scalar) * other.group1) + (float4(other.scalar) * anti_wedge.group1), 
            /* e41, e42, e43 */
            (float3(anti_wedge.scalar) * other.group2) + (float3(other.scalar) * anti_wedge.group2) + (float3(other.e4) * anti_wedge.group1.xyz) - (float3(anti_wedge.e4) * other.group1.xyz), 
            /* e23, e31, e12 */
            (float3(anti_wedge.scalar) * other.group3) + (float3(other.scalar) * anti_wedge.group3) + (anti_wedge.group1.zxy * other.group1.yzx) - (anti_wedge.group1.yzx * other.group1.zxy), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge.e3 * other.e42) + (anti_wedge.e4 * other.e23) + (anti_wedge.e42 * other.e3) + (anti_wedge.e23 * other.e4), (anti_wedge.e1 * other.e43) + (anti_wedge.e4 * other.e31) + (anti_wedge.e43 * other.e1) + (anti_wedge.e31 * other.e4), (anti_wedge.e2 * other.e41) + (anti_wedge.e4 * other.e12) + (anti_wedge.e41 * other.e2) + (anti_wedge.e12 * other.e4), -(anti_wedge.e1 * other.e23) - (anti_wedge.e2 * other.e31) - (anti_wedge.e3 * other.e12) - (anti_wedge.e12 * other.e3)) + (float4(anti_wedge.scalar) * other.group4) + (float4(other.scalar) * anti_wedge.group4) - (other.group1.yzxx * float4(anti_wedge.group2.zxy, anti_wedge.e23)) - float4((other.group2.zxy * anti_wedge.group1.yzx), anti_wedge.e31 * other.e2)
        );
    }
}
extension Origin: AntiProjectOrthogonallyOnto<Point> {
    associatedtype Output = Point;
    func anti_project_orthogonally_onto(other: Point) -> Output {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e4 * 0.0) * other.group0
        );
    }
}
extension Origin: AntiProjectOrthogonallyOnto<Scalar> {
    associatedtype Output = Origin;
    func anti_project_orthogonally_onto(other: Scalar) -> Output {
        return Origin.from_groups(
            /* e4 */
            powi(other.scalar, 2) * this.e4
        );
    }
}
extension Plane {
    // Fancy infix trick
    func operator/(_rhs: AntiProjectOrthogonallyOntoInfix) -> AntiProjectOrthogonallyOntoInfixPartial<Plane> {
        return AntiProjectOrthogonallyOntoInfixPartial { a: this };
    }
}
extension Plane: AntiProjectOrthogonallyOnto<DualNum> {
    associatedtype Output = Plane;
    func anti_project_orthogonally_onto(other: DualNum) -> Output {
AntiScalar right_anti_dual = AntiScalar.from_groups(
            /* e1234 */
            other.scalar
        );
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.scalar) * float4(right_anti_dual.e1234 * this.e423, right_anti_dual.e1234 * this.e431, right_anti_dual.e1234 * this.e412, right_anti_dual.e1234 * this.e321)
        );
    }
}
extension Plane: AntiProjectOrthogonallyOnto<Flector> {
    associatedtype Output = Flector;
    func anti_project_orthogonally_onto(other: Flector) -> Output {
Flector right_anti_dual = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.e1, other.e2, other.e3, 0.0)
        );
Motor anti_wedge = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(((right_anti_dual.group1.yzx * this.group0.zxy) - (right_anti_dual.group1.zxy * this.group0.yzx)), 0.0), 
            /* e23, e31, e12, scalar */
            float4(right_anti_dual.e321 * this.e423, right_anti_dual.e321 * this.e431, right_anti_dual.e321 * this.e412, -(right_anti_dual.e2 * this.e431) - (right_anti_dual.e3 * this.e412) - (right_anti_dual.e4 * this.e321)) - (this.group0.wwwx * float4(right_anti_dual.group1.xyz, right_anti_dual.e1))
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(anti_wedge.scalar) * other.group0, 
            /* e423, e431, e412, e321 */
            float4((other.e4 * anti_wedge.e23) + (other.e423 * anti_wedge.scalar), (other.e4 * anti_wedge.e31) + (other.e431 * anti_wedge.scalar), (other.e4 * anti_wedge.e12) + (other.e412 * anti_wedge.scalar), -(other.e2 * anti_wedge.e31) - (other.e3 * anti_wedge.e12)) + float4((other.group0.zxy * anti_wedge.group0.yzx), other.e321 * anti_wedge.scalar) - (other.group0.yzxx * float4(anti_wedge.group0.zxy, anti_wedge.e23))
        );
    }
}
extension Plane: AntiProjectOrthogonallyOnto<Horizon> {
    associatedtype Output = Horizon;
    func anti_project_orthogonally_onto(other: Horizon) -> Output {
        return Horizon.from_groups(
            /* e321 */
            powi(other.e321, 2) * this.e321
        );
    }
}
extension Plane: AntiProjectOrthogonallyOnto<Line> {
    associatedtype Output = Plane;
    func anti_project_orthogonally_onto(other: Line) -> Output {
Line right_anti_dual = Line.from_groups(
            /* e41, e42, e43 */
            other.group1 * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
Point anti_wedge = Point.from_groups(
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e41 * this.e321) + (right_anti_dual.e31 * this.e412), (right_anti_dual.e42 * this.e321) + (right_anti_dual.e12 * this.e423), (right_anti_dual.e43 * this.e321) + (right_anti_dual.e23 * this.e431), -(right_anti_dual.e42 * this.e431) - (right_anti_dual.e43 * this.e412)) - (this.group0.yzxx * float4(right_anti_dual.group1.zxy, right_anti_dual.e41))
        );
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((other.e42 * anti_wedge.e3) + (other.e23 * anti_wedge.e4), (other.e43 * anti_wedge.e1) + (other.e31 * anti_wedge.e4), (other.e41 * anti_wedge.e2) + (other.e12 * anti_wedge.e4), -(other.e31 * anti_wedge.e2) - (other.e12 * anti_wedge.e3)) - (anti_wedge.group0.yzxx * float4(other.group0.zxy, other.e23))
        );
    }
}
extension Plane: AntiProjectOrthogonallyOnto<Motor> {
    associatedtype Output = Flector;
    func anti_project_orthogonally_onto(other: Motor) -> Output {
Motor right_anti_dual = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            other.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
Flector anti_wedge = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e41 * this.e321) + (right_anti_dual.e31 * this.e412), (right_anti_dual.e42 * this.e321) + (right_anti_dual.e12 * this.e423), (right_anti_dual.e43 * this.e321) + (right_anti_dual.e23 * this.e431), -(right_anti_dual.e42 * this.e431) - (right_anti_dual.e43 * this.e412)) - (this.group0.yzxx * float4(right_anti_dual.group1.zxy, right_anti_dual.e41)), 
            /* e423, e431, e412, e321 */
            float4(right_anti_dual.e1234) * this.group0
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * anti_wedge.group0, 
            /* e423, e431, e412, e321 */
            float4((anti_wedge.e4 * other.e23) + (anti_wedge.e423 * other.scalar), (anti_wedge.e4 * other.e31) + (anti_wedge.e431 * other.scalar), (anti_wedge.e4 * other.e12) + (anti_wedge.e412 * other.scalar), -(anti_wedge.e2 * other.e31) - (anti_wedge.e3 * other.e12)) + float4((anti_wedge.group0.zxy * other.group0.yzx), anti_wedge.e321 * other.scalar) - (anti_wedge.group0.yzxx * float4(other.group0.zxy, other.e23))
        );
    }
}
extension Plane: AntiProjectOrthogonallyOnto<MultiVector> {
    associatedtype Output = MultiVector;
    func anti_project_orthogonally_onto(other: MultiVector) -> Output {
MultiVector right_anti_dual = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, other.scalar), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e41, e42, e43 */
            other.group3 * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(other.e1, other.e2, other.e3, 0.0)
        );
MultiVector anti_wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(-(right_anti_dual.e1 * this.e423) - (right_anti_dual.e2 * this.e431) - (right_anti_dual.e3 * this.e412) - (right_anti_dual.e4 * this.e321), 0.0), 
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e41 * this.e321) + (right_anti_dual.e31 * this.e412), (right_anti_dual.e42 * this.e321) + (right_anti_dual.e12 * this.e423), (right_anti_dual.e43 * this.e321) + (right_anti_dual.e23 * this.e431), -(right_anti_dual.e42 * this.e431) - (right_anti_dual.e43 * this.e412)) - (this.group0.yzxx * float4(right_anti_dual.group3.zxy, right_anti_dual.e41)), 
            /* e41, e42, e43 */
            (right_anti_dual.group4.yzx * this.group0.zxy) - (right_anti_dual.group4.zxy * this.group0.yzx), 
            /* e23, e31, e12 */
            (float3(right_anti_dual.e321) * this.group0.xyz) - (float3(this.e321) * right_anti_dual.group4.xyz), 
            /* e423, e431, e412, e321 */
            float4(right_anti_dual.e1234) * this.group0
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(anti_wedge.scalar * other.scalar, (anti_wedge.scalar * other.e1234) + (anti_wedge.e1234 * other.scalar) + (anti_wedge.e423 * other.e1) + (anti_wedge.e431 * other.e2) + (anti_wedge.e412 * other.e3) + (anti_wedge.e321 * other.e4) - (anti_wedge.e1 * other.e423) - (anti_wedge.e2 * other.e431) - (anti_wedge.e3 * other.e412) - (anti_wedge.e4 * other.e321) - (anti_wedge.e41 * other.e23) - (anti_wedge.e42 * other.e31) - (anti_wedge.e43 * other.e12) - (anti_wedge.e23 * other.e41) - (anti_wedge.e31 * other.e42) - (anti_wedge.e12 * other.e43)), 
            /* e1, e2, e3, e4 */
            (float4(anti_wedge.scalar) * other.group1) + (float4(other.scalar) * anti_wedge.group1), 
            /* e41, e42, e43 */
            (float3(anti_wedge.scalar) * other.group2) + (float3(other.scalar) * anti_wedge.group2) + (float3(other.e4) * anti_wedge.group1.xyz) - (float3(anti_wedge.e4) * other.group1.xyz), 
            /* e23, e31, e12 */
            (float3(anti_wedge.scalar) * other.group3) + (float3(other.scalar) * anti_wedge.group3) + (anti_wedge.group1.zxy * other.group1.yzx) - (anti_wedge.group1.yzx * other.group1.zxy), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge.e3 * other.e42) + (anti_wedge.e4 * other.e23) + (anti_wedge.e42 * other.e3) + (anti_wedge.e23 * other.e4), (anti_wedge.e1 * other.e43) + (anti_wedge.e4 * other.e31) + (anti_wedge.e43 * other.e1) + (anti_wedge.e31 * other.e4), (anti_wedge.e2 * other.e41) + (anti_wedge.e4 * other.e12) + (anti_wedge.e41 * other.e2) + (anti_wedge.e12 * other.e4), -(anti_wedge.e1 * other.e23) - (anti_wedge.e2 * other.e31) - (anti_wedge.e3 * other.e12) - (anti_wedge.e12 * other.e3)) + (float4(anti_wedge.scalar) * other.group4) + (float4(other.scalar) * anti_wedge.group4) - (other.group1.yzxx * float4(anti_wedge.group2.zxy, anti_wedge.e23)) - float4((other.group2.zxy * anti_wedge.group1.yzx), anti_wedge.e31 * other.e2)
        );
    }
}
extension Plane: AntiProjectOrthogonallyOnto<Plane> {
    associatedtype Output = Plane;
    func anti_project_orthogonally_onto(other: Plane) -> Output {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.e321 * this.e321) * other.group0
        );
    }
}
extension Plane: AntiProjectOrthogonallyOnto<Point> {
    associatedtype Output = Plane;
    func anti_project_orthogonally_onto(other: Point) -> Output {
Plane right_anti_dual = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.e1, other.e2, other.e3, 0.0)
        );
Line anti_wedge = Line.from_groups(
            /* e41, e42, e43 */
            (right_anti_dual.group0.yzx * this.group0.zxy) - (right_anti_dual.group0.zxy * this.group0.yzx), 
            /* e23, e31, e12 */
            (float3(right_anti_dual.e321) * this.group0.xyz) - (float3(this.e321) * right_anti_dual.group0.xyz)
        );
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4((anti_wedge.e42 * other.e3) + (anti_wedge.e23 * other.e4), (anti_wedge.e43 * other.e1) + (anti_wedge.e31 * other.e4), (anti_wedge.e41 * other.e2) + (anti_wedge.e12 * other.e4), -(anti_wedge.e31 * other.e2) - (anti_wedge.e12 * other.e3)) - (other.group0.yzxx * float4(anti_wedge.group0.zxy, anti_wedge.e23))
        );
    }
}
extension Plane: AntiProjectOrthogonallyOnto<Scalar> {
    associatedtype Output = Plane;
    func anti_project_orthogonally_onto(other: Scalar) -> Output {
AntiScalar right_anti_dual = AntiScalar.from_groups(
            /* e1234 */
            other.scalar
        );
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.scalar) * float4(right_anti_dual.e1234 * this.e423, right_anti_dual.e1234 * this.e431, right_anti_dual.e1234 * this.e412, right_anti_dual.e1234 * this.e321)
        );
    }
}
extension Point {
    // Fancy infix trick
    func operator/(_rhs: AntiProjectOrthogonallyOntoInfix) -> AntiProjectOrthogonallyOntoInfixPartial<Point> {
        return AntiProjectOrthogonallyOntoInfixPartial { a: this };
    }
}
extension Point: AntiProjectOrthogonallyOnto<DualNum> {
    associatedtype Output = Point;
    func anti_project_orthogonally_onto(other: DualNum) -> Output {
AntiScalar right_anti_dual = AntiScalar.from_groups(
            /* e1234 */
            other.scalar
        );
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * float4(right_anti_dual.e1234 * this.e1, right_anti_dual.e1234 * this.e2, right_anti_dual.e1234 * this.e3, right_anti_dual.e1234 * this.e4)
        );
    }
}
extension Point: AntiProjectOrthogonallyOnto<Flector> {
    associatedtype Output = Flector;
    func anti_project_orthogonally_onto(other: Flector) -> Output {
Flector right_anti_dual = Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.e1, other.e2, other.e3, 0.0)
        );
Scalar anti_wedge = Scalar.from_groups(
            /* scalar */
            (right_anti_dual.e423 * this.e1) + (right_anti_dual.e431 * this.e2) + (right_anti_dual.e412 * this.e3) + (right_anti_dual.e321 * this.e4)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(anti_wedge.scalar) * other.group0, 
            /* e423, e431, e412, e321 */
            float4(anti_wedge.scalar) * other.group1
        );
    }
}
extension Point: AntiProjectOrthogonallyOnto<Motor> {
    associatedtype Output = Flector;
    func anti_project_orthogonally_onto(other: Motor) -> Output {
Point anti_wedge = Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group0
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * anti_wedge.group0, 
            /* e423, e431, e412, e321 */
            float4((other.e42 * anti_wedge.e3) + (other.e23 * anti_wedge.e4), (other.e43 * anti_wedge.e1) + (other.e31 * anti_wedge.e4), (other.e41 * anti_wedge.e2) + (other.e12 * anti_wedge.e4), -(other.e31 * anti_wedge.e2) - (other.e12 * anti_wedge.e3)) - (anti_wedge.group0.yzxx * float4(other.group0.zxy, other.e23))
        );
    }
}
extension Point: AntiProjectOrthogonallyOnto<MultiVector> {
    associatedtype Output = MultiVector;
    func anti_project_orthogonally_onto(other: MultiVector) -> Output {
MultiVector right_anti_dual = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, other.scalar), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e321 * -1.0), 
            /* e41, e42, e43 */
            other.group3 * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(other.e1, other.e2, other.e3, 0.0)
        );
MultiVector anti_wedge = MultiVector.from_groups(
            /* scalar, e1234 */
            float2((right_anti_dual.e423 * this.e1) + (right_anti_dual.e431 * this.e2) + (right_anti_dual.e412 * this.e3) + (right_anti_dual.e321 * this.e4), 0.0), 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual.e1234) * this.group0, 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(anti_wedge.scalar * other.scalar, (anti_wedge.scalar * other.e1234) + (anti_wedge.e1234 * other.scalar) + (anti_wedge.e423 * other.e1) + (anti_wedge.e431 * other.e2) + (anti_wedge.e412 * other.e3) + (anti_wedge.e321 * other.e4) - (anti_wedge.e1 * other.e423) - (anti_wedge.e2 * other.e431) - (anti_wedge.e3 * other.e412) - (anti_wedge.e4 * other.e321) - (anti_wedge.e41 * other.e23) - (anti_wedge.e42 * other.e31) - (anti_wedge.e43 * other.e12) - (anti_wedge.e23 * other.e41) - (anti_wedge.e31 * other.e42) - (anti_wedge.e12 * other.e43)), 
            /* e1, e2, e3, e4 */
            (float4(anti_wedge.scalar) * other.group1) + (float4(other.scalar) * anti_wedge.group1), 
            /* e41, e42, e43 */
            (float3(anti_wedge.scalar) * other.group2) + (float3(other.scalar) * anti_wedge.group2) + (float3(other.e4) * anti_wedge.group1.xyz) - (float3(anti_wedge.e4) * other.group1.xyz), 
            /* e23, e31, e12 */
            (float3(anti_wedge.scalar) * other.group3) + (float3(other.scalar) * anti_wedge.group3) + (anti_wedge.group1.zxy * other.group1.yzx) - (anti_wedge.group1.yzx * other.group1.zxy), 
            /* e423, e431, e412, e321 */
            float4((anti_wedge.e3 * other.e42) + (anti_wedge.e4 * other.e23) + (anti_wedge.e42 * other.e3) + (anti_wedge.e23 * other.e4), (anti_wedge.e1 * other.e43) + (anti_wedge.e4 * other.e31) + (anti_wedge.e43 * other.e1) + (anti_wedge.e31 * other.e4), (anti_wedge.e2 * other.e41) + (anti_wedge.e4 * other.e12) + (anti_wedge.e41 * other.e2) + (anti_wedge.e12 * other.e4), -(anti_wedge.e1 * other.e23) - (anti_wedge.e2 * other.e31) - (anti_wedge.e3 * other.e12) - (anti_wedge.e12 * other.e3)) + (float4(anti_wedge.scalar) * other.group4) + (float4(other.scalar) * anti_wedge.group4) - (other.group1.yzxx * float4(anti_wedge.group2.zxy, anti_wedge.e23)) - float4((other.group2.zxy * anti_wedge.group1.yzx), anti_wedge.e31 * other.e2)
        );
    }
}
extension Point: AntiProjectOrthogonallyOnto<Point> {
    associatedtype Output = Point;
    func anti_project_orthogonally_onto(other: Point) -> Output {
Plane right_anti_dual = Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.e1, other.e2, other.e3, 0.0)
        );
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4((right_anti_dual.e423 * this.e1) + (right_anti_dual.e431 * this.e2) + (right_anti_dual.e412 * this.e3) + (right_anti_dual.e321 * this.e4)) * other.group0
        );
    }
}
extension Point: AntiProjectOrthogonallyOnto<Scalar> {
    associatedtype Output = Point;
    func anti_project_orthogonally_onto(other: Scalar) -> Output {
AntiScalar right_anti_dual = AntiScalar.from_groups(
            /* e1234 */
            other.scalar
        );
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * float4(right_anti_dual.e1234 * this.e1, right_anti_dual.e1234 * this.e2, right_anti_dual.e1234 * this.e3, right_anti_dual.e1234 * this.e4)
        );
    }
}
extension Scalar {
    // Fancy infix trick
    func operator/(_rhs: AntiProjectOrthogonallyOntoInfix) -> AntiProjectOrthogonallyOntoInfixPartial<Scalar> {
        return AntiProjectOrthogonallyOntoInfixPartial { a: this };
    }
}
extension Scalar: AntiProjectOrthogonallyOnto<DualNum> {
    associatedtype Output = DualNum;
    func anti_project_orthogonally_onto(other: DualNum) -> Output {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(other.scalar * this.scalar) * other.group0
        );
    }
}
extension Scalar: AntiProjectOrthogonallyOnto<Motor> {
    associatedtype Output = Motor;
    func anti_project_orthogonally_onto(other: Motor) -> Output {
Scalar anti_wedge = Scalar.from_groups(
            /* scalar */
            (other.group1 * float4(-1.0, -1.0, -1.0, 1.0))[3] * this.scalar
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(anti_wedge.scalar) * other.group0, 
            /* e23, e31, e12, scalar */
            float4(anti_wedge.scalar) * other.group1
        );
    }
}
extension Scalar: AntiProjectOrthogonallyOnto<MultiVector> {
    associatedtype Output = MultiVector;
    func anti_project_orthogonally_onto(other: MultiVector) -> Output {
Scalar anti_wedge = Scalar.from_groups(
            /* scalar */
            other.scalar * this.scalar
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(anti_wedge.scalar) * other.group0, 
            /* e1, e2, e3, e4 */
            float4(anti_wedge.scalar) * other.group1, 
            /* e41, e42, e43 */
            float3(anti_wedge.scalar) * other.group2, 
            /* e23, e31, e12 */
            float3(anti_wedge.scalar) * other.group3, 
            /* e423, e431, e412, e321 */
            float4(anti_wedge.scalar) * other.group4
        );
    }
}
extension Scalar: AntiProjectOrthogonallyOnto<Scalar> {
    associatedtype Output = Scalar;
    func anti_project_orthogonally_onto(other: Scalar) -> Output {
        return Scalar.from_groups(
            /* scalar */
            powi(other.scalar, 2) * this.scalar
        );
    }
}
