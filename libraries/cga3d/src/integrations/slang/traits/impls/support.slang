extension AntiCircleRotor {
    // Fancy infix trick
    func operator/(_rhs: SupportPrefixOrPostfix) -> Sphere {
        return this.support();
    }
}
extension AntiCircleRotor: Support {
    associatedtype Output = Sphere;
    func support() -> Output {
        CircleRotor right_anti_dual = CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(this.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            this.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        RoundPoint self_2 = RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5, 0, 0, 0 */
            float4(0.0)
        );
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e425 * self_2.e3) + (right_anti_dual.e235 * self_2.e4), (right_anti_dual.e435 * self_2.e1) + (right_anti_dual.e315 * self_2.e4), (right_anti_dual.e415 * self_2.e2) + (right_anti_dual.e125 * self_2.e4), -(right_anti_dual.e315 * self_2.e2) - (right_anti_dual.e125 * self_2.e3)) - (float4(self_2.e5) * float4(right_anti_dual.group0.xyz, right_anti_dual.e321)) - (self_2.group0.yzxx * float4(right_anti_dual.group1.zxy, right_anti_dual.e235)), 
            /* e1234, 0, 0, 0 */
            float4((right_anti_dual.e423 * self_2.e1) + (right_anti_dual.e431 * self_2.e2) + (right_anti_dual.e412 * self_2.e3) + (right_anti_dual.e321 * self_2.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiDipoleInversion {
    // Fancy infix trick
    func operator/(_rhs: SupportPrefixOrPostfix) -> CircleRotor {
        return this.support();
    }
}
extension AntiDipoleInversion: Support {
    associatedtype Output = CircleRotor;
    func support() -> Output {
        DipoleInversion right_anti_dual = DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            this.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            this.group2 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group3 * float4(1.0, 1.0, 1.0, -1.0)
        );
        RoundPoint self_2 = RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5, 0, 0, 0 */
            float4(0.0)
        );
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4((float3(self_2.e4) * right_anti_dual.group1.xyz) + (right_anti_dual.group0.xyz.yzx * self_2.group0.zxy) - (right_anti_dual.group0.xyz.zxy * self_2.group0.yzx), 0.0), 
            /* e415, e425, e435, e321 */
            float4((right_anti_dual.e41 * self_2.e5) + (right_anti_dual.e15 * self_2.e4), (right_anti_dual.e42 * self_2.e5) + (right_anti_dual.e25 * self_2.e4), (right_anti_dual.e43 * self_2.e5) + (right_anti_dual.e35 * self_2.e4), -(right_anti_dual.e31 * self_2.e2) - (right_anti_dual.e12 * self_2.e3)) - (right_anti_dual.group1.wwwx * self_2.group0.xyzx), 
            /* e235, e315, e125, e12345 */
            float4(right_anti_dual.e25 * self_2.e3 * -1.0, right_anti_dual.e35 * self_2.e1 * -1.0, right_anti_dual.e15 * self_2.e2 * -1.0, (right_anti_dual.e4315 * self_2.e2) + (right_anti_dual.e4125 * self_2.e3) + (right_anti_dual.e3215 * self_2.e4)) + (float4(self_2.e5) * float4(right_anti_dual.group1.xyz, right_anti_dual.e1234)) + (self_2.group0.yzxx * float4(right_anti_dual.group2.zxy, right_anti_dual.e4235))
        );
    }
}
extension AntiDualNum {
    // Fancy infix trick
    func operator/(_rhs: SupportPrefixOrPostfix) -> FlatPoint {
        return this.support();
    }
}
extension AntiDualNum: Support {
    associatedtype Output = FlatPoint;
    func support() -> Output {
        return FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            float4(this.e3215) * float4(float3(0.0), 1.0)
        );
    }
}
extension AntiFlatPoint {
    // Fancy infix trick
    func operator/(_rhs: SupportPrefixOrPostfix) -> Line {
        return this.support();
    }
}
extension AntiFlatPoint: Support {
    associatedtype Output = Line;
    func support() -> Output {
        FlatPoint right_anti_dual = FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            this.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        RoundPoint self_2 = RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5, 0, 0, 0 */
            float4(0.0)
        );
        return Line.from_groups(
            /* e415, e425, e435, 0 */
            float4((float3(self_2.e4) * right_anti_dual.group0.xyz) - (float3(right_anti_dual.e45) * self_2.group0.xyz), 0.0), 
            /* e235, e315, e125, 0 */
            float4((right_anti_dual.group0.zxy * self_2.group0.yzx) - (right_anti_dual.group0.yzx * self_2.group0.zxy), 0.0)
        );
    }
}
extension AntiFlector {
    // Fancy infix trick
    func operator/(_rhs: SupportPrefixOrPostfix) -> Motor {
        return this.support();
    }
}
extension AntiFlector: Support {
    associatedtype Output = Motor;
    func support() -> Output {
        Flector right_anti_dual = Flector.from_groups(
            /* e15, e25, e35, e45 */
            this.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        RoundPoint self_2 = RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5, 0, 0, 0 */
            float4(0.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(right_anti_dual.e45 * self_2.e1 * -1.0, right_anti_dual.e45 * self_2.e2 * -1.0, right_anti_dual.e45 * self_2.e3 * -1.0, (right_anti_dual.e4315 * self_2.e2) + (right_anti_dual.e4125 * self_2.e3) + (right_anti_dual.e3215 * self_2.e4)) + (self_2.group0.wwwx * float4(right_anti_dual.group0.xyz, right_anti_dual.e4235)), 
            /* e235, e315, e125, e5 */
            float4(((right_anti_dual.group0.zxy * self_2.group0.yzx) - (right_anti_dual.group0.yzx * self_2.group0.zxy)), 0.0)
        );
    }
}
extension AntiLine {
    // Fancy infix trick
    func operator/(_rhs: SupportPrefixOrPostfix) -> Plane {
        return this.support();
    }
}
extension AntiLine: Support {
    associatedtype Output = Plane;
    func support() -> Output {
        Line right_anti_dual = Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(this.group0.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group1.xyz * float3(-1.0), 0.0)
        );
        RoundPoint self_2 = RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5, 0, 0, 0 */
            float4(0.0)
        );
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e425 * self_2.e3) + (right_anti_dual.e235 * self_2.e4), (right_anti_dual.e435 * self_2.e1) + (right_anti_dual.e315 * self_2.e4), (right_anti_dual.e415 * self_2.e2) + (right_anti_dual.e125 * self_2.e4), -(right_anti_dual.e315 * self_2.e2) - (right_anti_dual.e125 * self_2.e3)) - (self_2.group0.yzxx * float4(right_anti_dual.group0.xyz.zxy, right_anti_dual.e235))
        );
    }
}
extension AntiMotor {
    // Fancy infix trick
    func operator/(_rhs: SupportPrefixOrPostfix) -> Flector {
        return this.support();
    }
}
extension AntiMotor: Support {
    associatedtype Output = Flector;
    func support() -> Output {
        Motor right_anti_dual = Motor.from_groups(
            /* e415, e425, e435, e12345 */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        RoundPoint self_2 = RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5, 0, 0, 0 */
            float4(0.0)
        );
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(right_anti_dual.e5) * self_2.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e425 * self_2.e3) + (right_anti_dual.e235 * self_2.e4), (right_anti_dual.e435 * self_2.e1) + (right_anti_dual.e315 * self_2.e4), (right_anti_dual.e415 * self_2.e2) + (right_anti_dual.e125 * self_2.e4), -(right_anti_dual.e315 * self_2.e2) - (right_anti_dual.e125 * self_2.e3)) - (self_2.group0.yzxx * float4(right_anti_dual.group0.zxy, right_anti_dual.e235))
        );
    }
}
extension AntiPlane {
    // Fancy infix trick
    func operator/(_rhs: SupportPrefixOrPostfix) -> AntiScalar {
        return this.support();
    }
}
extension AntiPlane: Support {
    associatedtype Output = AntiScalar;
    func support() -> Output {
        Plane right_anti_dual = Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            this.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        RoundPoint self_2 = RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5, 0, 0, 0 */
            float4(0.0)
        );
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((right_anti_dual.e4235 * self_2.e1) + (right_anti_dual.e4315 * self_2.e2) + (right_anti_dual.e4125 * self_2.e3) + (right_anti_dual.e3215 * self_2.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension AntiScalar {
    // Fancy infix trick
    func operator/(_rhs: SupportPrefixOrPostfix) -> RoundPoint {
        return this.support();
    }
}
extension AntiScalar: Support {
    associatedtype Output = RoundPoint;
    func support() -> Output {
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e12345 * -1.0) * float4(float3(0.0), 1.0), 
            /* e5, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension Circle {
    // Fancy infix trick
    func operator/(_rhs: SupportPrefixOrPostfix) -> Circle {
        return this.support();
    }
    func operator/=(const SupportPrefixOrPostfix& _rhs) -> Circle& {
        this = this.support();
        return *this;
    }
}
extension Circle: Support {
    associatedtype Output = Circle;
    func support() -> Output {
        Dipole right_anti_dual = Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            this.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, 0 */
            float4(this.group2.xyz, 0.0)
        );
        RoundPoint self_2 = RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5, 0, 0, 0 */
            float4(0.0)
        );
        return Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4((float3(self_2.e4) * right_anti_dual.group1.xyz) + (right_anti_dual.group0.xyz.yzx * self_2.group0.zxy) - (right_anti_dual.group0.xyz.zxy * self_2.group0.yzx), 0.0), 
            /* e415, e425, e435, e321 */
            float4((right_anti_dual.e41 * self_2.e5) + (right_anti_dual.e15 * self_2.e4), (right_anti_dual.e42 * self_2.e5) + (right_anti_dual.e25 * self_2.e4), (right_anti_dual.e43 * self_2.e5) + (right_anti_dual.e35 * self_2.e4), -(right_anti_dual.e31 * self_2.e2) - (right_anti_dual.e12 * self_2.e3)) - (right_anti_dual.group1.wwwx * self_2.group0.xyzx), 
            /* e235, e315, e125, 0 */
            float4((float3(self_2.e5) * right_anti_dual.group1.xyz) + (right_anti_dual.group2.xyz.zxy * self_2.group0.yzx) - (right_anti_dual.group2.xyz.yzx * self_2.group0.zxy), 0.0)
        );
    }
}
extension CircleRotor {
    // Fancy infix trick
    func operator/(_rhs: SupportPrefixOrPostfix) -> AntiDipoleInversion {
        return this.support();
    }
}
extension CircleRotor: Support {
    associatedtype Output = AntiDipoleInversion;
    func support() -> Output {
        AntiCircleRotor right_anti_dual = AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            this.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            this.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        RoundPoint self_2 = RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5, 0, 0, 0 */
            float4(0.0)
        );
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4((float3(self_2.e4) * right_anti_dual.group1.xyz) + (right_anti_dual.group0.xyz.yzx * self_2.group0.zxy) - (right_anti_dual.group0.xyz.zxy * self_2.group0.yzx), 0.0), 
            /* e415, e425, e435, e321 */
            float4((right_anti_dual.e41 * self_2.e5) + (right_anti_dual.e15 * self_2.e4), (right_anti_dual.e42 * self_2.e5) + (right_anti_dual.e25 * self_2.e4), (right_anti_dual.e43 * self_2.e5) + (right_anti_dual.e35 * self_2.e4), -(right_anti_dual.e31 * self_2.e2) - (right_anti_dual.e12 * self_2.e3)) - (right_anti_dual.group1.wwwx * self_2.group0.xyzx), 
            /* e235, e315, e125, e4 */
            float4(((float3(self_2.e5) * right_anti_dual.group1.xyz) + (right_anti_dual.group2.zxy * self_2.group0.yzx) - (right_anti_dual.group2.yzx * self_2.group0.zxy)), right_anti_dual.scalar * self_2.e4), 
            /* e1, e2, e3, e5 */
            float4(float3(1.0), 0.0) * float4(right_anti_dual.group2.www, 0.0) * float4(self_2.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Dipole {
    // Fancy infix trick
    func operator/(_rhs: SupportPrefixOrPostfix) -> Sphere {
        return this.support();
    }
}
extension Dipole: Support {
    associatedtype Output = Sphere;
    func support() -> Output {
        Circle right_anti_dual = Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(this.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, 0 */
            float4(this.group2.xyz * float3(-1.0), 0.0)
        );
        RoundPoint self_2 = RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5, 0, 0, 0 */
            float4(0.0)
        );
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e425 * self_2.e3) + (right_anti_dual.e235 * self_2.e4), (right_anti_dual.e435 * self_2.e1) + (right_anti_dual.e315 * self_2.e4), (right_anti_dual.e415 * self_2.e2) + (right_anti_dual.e125 * self_2.e4), -(right_anti_dual.e321 * self_2.e5) - (right_anti_dual.e125 * self_2.e3)) - (self_2.group0.yzxy * float4(right_anti_dual.group1.zxy, right_anti_dual.e315)) - float4((float3(self_2.e5) * right_anti_dual.group0.xyz), right_anti_dual.e235 * self_2.e1), 
            /* e1234, 0, 0, 0 */
            float4((right_anti_dual.e423 * self_2.e1) + (right_anti_dual.e431 * self_2.e2) + (right_anti_dual.e412 * self_2.e3) + (right_anti_dual.e321 * self_2.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension DipoleInversion {
    // Fancy infix trick
    func operator/(_rhs: SupportPrefixOrPostfix) -> DipoleInversion {
        return this.support();
    }
    func operator/=(const SupportPrefixOrPostfix& _rhs) -> DipoleInversion& {
        this = this.support();
        return *this;
    }
}
extension DipoleInversion: Support {
    associatedtype Output = DipoleInversion;
    func support() -> Output {
        AntiDipoleInversion right_anti_dual = AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(this.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e4 */
            this.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            this.group3 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        RoundPoint self_2 = RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5, 0, 0, 0 */
            float4(0.0)
        );
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4((float3(self_2.e4) * right_anti_dual.group3.xyz) - (float3(right_anti_dual.e4) * self_2.group0.xyz), 0.0), 
            /* e23, e31, e12, e45 */
            (right_anti_dual.group3.zxyw * self_2.group0.yzxw) - float4((right_anti_dual.group3.yzx * self_2.group0.zxy), right_anti_dual.e4 * self_2.e5), 
            /* e15, e25, e35, e1234 */
            float4(right_anti_dual.e1 * self_2.e5 * -1.0, right_anti_dual.e2 * self_2.e5 * -1.0, right_anti_dual.e3 * self_2.e5 * -1.0, (right_anti_dual.e431 * self_2.e2) + (right_anti_dual.e412 * self_2.e3) + (right_anti_dual.e321 * self_2.e4)) + (self_2.group0.xyzx * float4(right_anti_dual.group3.www, right_anti_dual.e423)), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e425 * self_2.e3) + (right_anti_dual.e235 * self_2.e4), (right_anti_dual.e435 * self_2.e1) + (right_anti_dual.e315 * self_2.e4), (right_anti_dual.e415 * self_2.e2) + (right_anti_dual.e125 * self_2.e4), -(right_anti_dual.e315 * self_2.e2) - (right_anti_dual.e125 * self_2.e3)) - (float4(self_2.e5) * float4(right_anti_dual.group0.xyz, right_anti_dual.e321)) - (self_2.group0.yzxx * float4(right_anti_dual.group1.zxy, right_anti_dual.e235))
        );
    }
}
extension DualNum {
    // Fancy infix trick
    func operator/(_rhs: SupportPrefixOrPostfix) -> VersorEven {
        return this.support();
    }
}
extension DualNum: Support {
    associatedtype Output = VersorEven;
    func support() -> Output {
        AntiDualNum right_anti_dual = AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(this.group0.xy * float2(-1.0), 0.0, 0.0)
        );
        RoundPoint self_2 = RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5, 0, 0, 0 */
            float4(0.0)
        );
        return VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            float4(float3(0.0), right_anti_dual.e3215 * self_2.e4), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e5 */
            float4(0.0), 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual.scalar) * self_2.group0
        );
    }
}
extension FlatPoint {
    // Fancy infix trick
    func operator/(_rhs: SupportPrefixOrPostfix) -> Sphere {
        return this.support();
    }
}
extension FlatPoint: Support {
    associatedtype Output = Sphere;
    func support() -> Output {
        AntiFlatPoint right_anti_dual = AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        RoundPoint self_2 = RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5, 0, 0, 0 */
            float4(0.0)
        );
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4((float3(self_2.e4) * right_anti_dual.group0.xyz), -(right_anti_dual.e235 * self_2.e1) - (right_anti_dual.e315 * self_2.e2) - (right_anti_dual.e125 * self_2.e3) - (right_anti_dual.e321 * self_2.e5)), 
            /* e1234, 0, 0, 0 */
            float4(right_anti_dual.e321 * self_2.e4, 0.0, 0.0, 0.0)
        );
    }
}
extension Flector {
    // Fancy infix trick
    func operator/(_rhs: SupportPrefixOrPostfix) -> DipoleInversion {
        return this.support();
    }
}
extension Flector: Support {
    associatedtype Output = DipoleInversion;
    func support() -> Output {
        AntiFlector right_anti_dual = AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        RoundPoint self_2 = RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5, 0, 0, 0 */
            float4(0.0)
        );
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(self_2.e4) * right_anti_dual.group1.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(((right_anti_dual.group1.zxy * self_2.group0.yzx) - (right_anti_dual.group1.yzx * self_2.group0.zxy)), right_anti_dual.e5 * self_2.e4), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_anti_dual.e5) * self_2.group0.xyz) - (float3(self_2.e5) * right_anti_dual.group1.xyz)), right_anti_dual.e321 * self_2.e4), 
            /* e4235, e4315, e4125, e3215 */
            float4((float3(self_2.e4) * right_anti_dual.group0.xyz), -(right_anti_dual.e235 * self_2.e1) - (right_anti_dual.e315 * self_2.e2) - (right_anti_dual.e125 * self_2.e3) - (right_anti_dual.e321 * self_2.e5))
        );
    }
}
extension Line {
    // Fancy infix trick
    func operator/(_rhs: SupportPrefixOrPostfix) -> Circle {
        return this.support();
    }
}
extension Line: Support {
    associatedtype Output = Circle;
    func support() -> Output {
        AntiLine right_anti_dual = AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(this.group1.xyz, 0.0)
        );
        RoundPoint self_2 = RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5, 0, 0, 0 */
            float4(0.0)
        );
        return Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(self_2.e4) * right_anti_dual.group0.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4((float3(self_2.e4) * right_anti_dual.group1.xyz), -(right_anti_dual.e23 * self_2.e1) - (right_anti_dual.e31 * self_2.e2) - (right_anti_dual.e12 * self_2.e3)), 
            /* e235, e315, e125, 0 */
            float4((float3(self_2.e5) * right_anti_dual.group0.xyz) + (right_anti_dual.group1.xyz.zxy * self_2.group0.yzx) - (right_anti_dual.group1.xyz.yzx * self_2.group0.zxy), 0.0)
        );
    }
}
extension Motor {
    // Fancy infix trick
    func operator/(_rhs: SupportPrefixOrPostfix) -> VersorEven {
        return this.support();
    }
}
extension Motor: Support {
    associatedtype Output = VersorEven;
    func support() -> Output {
        AntiMotor right_anti_dual = AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            this.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            this.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        RoundPoint self_2 = RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5, 0, 0, 0 */
            float4(0.0)
        );
        return VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            float4(self_2.e4) * float4(right_anti_dual.group0.xyz, right_anti_dual.e3215), 
            /* e415, e425, e435, e321 */
            float4((float3(self_2.e4) * right_anti_dual.group1.xyz), -(right_anti_dual.e23 * self_2.e1) - (right_anti_dual.e31 * self_2.e2) - (right_anti_dual.e12 * self_2.e3)), 
            /* e235, e315, e125, e5 */
            float4((float3(self_2.e5) * right_anti_dual.group0.xyz), 0.0) + float4((right_anti_dual.group1.zxy * self_2.group0.yzx), 0.0) - float4((right_anti_dual.group1.yzx * self_2.group0.zxy), 0.0), 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual.scalar) * self_2.group0
        );
    }
}
extension MultiVector {
    // Fancy infix trick
    func operator/(_rhs: SupportPrefixOrPostfix) -> MultiVector {
        return this.support();
    }
    func operator/=(const SupportPrefixOrPostfix& _rhs) -> MultiVector& {
        this = this.support();
        return *this;
    }
}
extension MultiVector: Support {
    associatedtype Output = MultiVector;
    func support() -> Output {
        MultiVector right_anti_dual = MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(this.group0.xy.yx * float2(-1.0, 1.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(this.group9.xyz, this.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5, 0, 0, 0 */
            float4(this.e3215, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(this.group8.xyz, this.e321 * -1.0), 
            /* e41, e42, e43, 0 */
            float4(this.group7.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(this.group6.xyz, 0.0), 
            /* e415, e425, e435, e321 */
            float4(this.group5.xyz, this.e45) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e423, e431, e412, 0 */
            float4(this.group4.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group3.xyz * float3(-1.0), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.group1.xyz, this.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234, 0, 0, 0 */
            float4(this.e4 * -1.0, 0.0, 0.0, 0.0)
        );
        RoundPoint self_2 = RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5, 0, 0, 0 */
            float4(0.0)
        );
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(0.0, (right_anti_dual.e4235 * self_2.e1) + (right_anti_dual.e4315 * self_2.e2) + (right_anti_dual.e4125 * self_2.e3) + (right_anti_dual.e3215 * self_2.e4) + (right_anti_dual.e1234 * self_2.e5)), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual.scalar) * self_2.group0, 
            /* e5, 0, 0, 0 */
            float4(0.0), 
            /* e15, e25, e35, e45 */
            (float4(right_anti_dual.e5) * self_2.group0) - (float4(self_2.e5) * right_anti_dual.group1), 
            /* e41, e42, e43, 0 */
            float4((float3(self_2.e4) * right_anti_dual.group1.xyz) - (float3(right_anti_dual.e4) * self_2.group0.xyz), 0.0), 
            /* e23, e31, e12, 0 */
            float4((right_anti_dual.group1.zxy * self_2.group0.yzx) - (right_anti_dual.group1.yzx * self_2.group0.zxy), 0.0), 
            /* e415, e425, e435, e321 */
            float4((right_anti_dual.e15 * self_2.e4) + (right_anti_dual.e41 * self_2.e5), (right_anti_dual.e25 * self_2.e4) + (right_anti_dual.e42 * self_2.e5), (right_anti_dual.e35 * self_2.e4) + (right_anti_dual.e43 * self_2.e5), -(right_anti_dual.e31 * self_2.e2) - (right_anti_dual.e12 * self_2.e3)) - (self_2.group0.xyzx * float4(right_anti_dual.group3.www, right_anti_dual.e23)), 
            /* e423, e431, e412, 0 */
            float4((float3(self_2.e4) * right_anti_dual.group5.xyz) + (right_anti_dual.group4.xyz.yzx * self_2.group0.zxy) - (right_anti_dual.group4.xyz.zxy * self_2.group0.yzx), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(self_2.e5) * right_anti_dual.group5.xyz) + (right_anti_dual.group3.zxy * self_2.group0.yzx) - (right_anti_dual.group3.yzx * self_2.group0.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e425 * self_2.e3) + (right_anti_dual.e235 * self_2.e4), (right_anti_dual.e435 * self_2.e1) + (right_anti_dual.e315 * self_2.e4), (right_anti_dual.e415 * self_2.e2) + (right_anti_dual.e125 * self_2.e4), -(right_anti_dual.e321 * self_2.e5) - (right_anti_dual.e125 * self_2.e3)) - (self_2.group0.yzxy * float4(right_anti_dual.group6.zxy, right_anti_dual.e315)) - float4((float3(self_2.e5) * right_anti_dual.group7.xyz), right_anti_dual.e235 * self_2.e1), 
            /* e1234, 0, 0, 0 */
            float4((right_anti_dual.e321 * self_2.e4) + (right_anti_dual.e423 * self_2.e1) + (right_anti_dual.e431 * self_2.e2) + (right_anti_dual.e412 * self_2.e3), 0.0, 0.0, 0.0)
        );
    }
}
extension Plane {
    // Fancy infix trick
    func operator/(_rhs: SupportPrefixOrPostfix) -> Dipole {
        return this.support();
    }
}
extension Plane: Support {
    associatedtype Output = Dipole;
    func support() -> Output {
        AntiPlane right_anti_dual = AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        RoundPoint self_2 = RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5, 0, 0, 0 */
            float4(0.0)
        );
        return Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(self_2.e4) * right_anti_dual.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            float4(((right_anti_dual.group0.zxy * self_2.group0.yzx) - (right_anti_dual.group0.yzx * self_2.group0.zxy)), right_anti_dual.e5 * self_2.e4), 
            /* e15, e25, e35, 0 */
            float4((float3(right_anti_dual.e5) * self_2.group0.xyz) - (float3(self_2.e5) * right_anti_dual.group0.xyz), 0.0)
        );
    }
}
extension RoundPoint {
    // Fancy infix trick
    func operator/(_rhs: SupportPrefixOrPostfix) -> AntiScalar {
        return this.support();
    }
}
extension RoundPoint: Support {
    associatedtype Output = AntiScalar;
    func support() -> Output {
        Sphere right_anti_dual = Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(this.group0.xyz, this.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234, 0, 0, 0 */
            float4(this.e4 * -1.0, 0.0, 0.0, 0.0)
        );
        RoundPoint self_2 = RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5, 0, 0, 0 */
            float4(0.0)
        );
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4((self_2.e1 * right_anti_dual.e4235) + (self_2.e2 * right_anti_dual.e4315) + (self_2.e3 * right_anti_dual.e4125) + (self_2.e4 * right_anti_dual.e3215) + (self_2.e5 * right_anti_dual.e1234), 0.0, 0.0, 0.0)
        );
    }
}
extension Sphere {
    // Fancy infix trick
    func operator/(_rhs: SupportPrefixOrPostfix) -> Dipole {
        return this.support();
    }
}
extension Sphere: Support {
    associatedtype Output = Dipole;
    func support() -> Output {
        RoundPoint right_anti_dual = RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.group0.xyz, this.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5, 0, 0, 0 */
            float4(this.e3215, 0.0, 0.0, 0.0)
        );
        RoundPoint self_2 = RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5, 0, 0, 0 */
            float4(0.0)
        );
        return Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4((float3(self_2.e4) * right_anti_dual.group0.xyz) - (float3(right_anti_dual.e4) * self_2.group0.xyz), 0.0), 
            /* e23, e31, e12, e45 */
            float4((right_anti_dual.group0.zxy * self_2.group0.yzx), right_anti_dual.e5 * self_2.e4) - (right_anti_dual.group0.yzxw * float4(self_2.group0.zxy, self_2.e5)), 
            /* e15, e25, e35, 0 */
            float4((float3(right_anti_dual.e5) * self_2.group0.xyz) - (float3(self_2.e5) * right_anti_dual.group0.xyz), 0.0)
        );
    }
}
extension VersorEven {
    // Fancy infix trick
    func operator/(_rhs: SupportPrefixOrPostfix) -> VersorEven {
        return this.support();
    }
    func operator/=(const SupportPrefixOrPostfix& _rhs) -> VersorEven& {
        this = this.support();
        return *this;
    }
}
extension VersorEven: Support {
    associatedtype Output = VersorEven;
    func support() -> Output {
        VersorOdd right_anti_dual = VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            this.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e23, e31, e12, e45 */
            this.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            float4(this.group2.xyz, this.e4) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.group3.xyz, this.e5) * float4(1.0, 1.0, 1.0, -1.0)
        );
        RoundPoint self_2 = RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5, 0, 0, 0 */
            float4(0.0)
        );
        return VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            float4(self_2.e2 * right_anti_dual.e43 * -1.0, self_2.e3 * right_anti_dual.e41 * -1.0, self_2.e1 * right_anti_dual.e42 * -1.0, (self_2.e3 * right_anti_dual.e4125) + (self_2.e4 * right_anti_dual.e3215) + (self_2.e5 * right_anti_dual.e1234)) + (self_2.group0.zxyx * float4(right_anti_dual.group0.yzx, right_anti_dual.e4235)) + (self_2.group0.wwwy * float4(right_anti_dual.group1.xyz, right_anti_dual.e4315)), 
            /* e415, e425, e435, e321 */
            float4((self_2.e4 * right_anti_dual.e15) + (self_2.e5 * right_anti_dual.e41), (self_2.e4 * right_anti_dual.e25) + (self_2.e5 * right_anti_dual.e42), (self_2.e4 * right_anti_dual.e35) + (self_2.e5 * right_anti_dual.e43), -(self_2.e2 * right_anti_dual.e31) - (self_2.e3 * right_anti_dual.e12)) - (self_2.group0.xyzx * right_anti_dual.group1.wwwx), 
            /* e235, e315, e125, e5 */
            float4((float3(self_2.e5) * right_anti_dual.group1.xyz), 0.0) + float4((self_2.group0.yzx * right_anti_dual.group2.zxy), 0.0) - float4((self_2.group0.zxy * right_anti_dual.group2.yzx), 0.0), 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual.scalar) * self_2.group0
        );
    }
}
extension VersorOdd {
    // Fancy infix trick
    func operator/(_rhs: SupportPrefixOrPostfix) -> DipoleInversion {
        return this.support();
    }
}
extension VersorOdd: Support {
    associatedtype Output = DipoleInversion;
    func support() -> Output {
        VersorEven right_anti_dual = VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e415, e425, e435, e321 */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            float4(this.group2.xyz, this.e3215) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(this.group3.xyz, this.e1234) * float4(-1.0, -1.0, -1.0, 1.0)
        );
        RoundPoint self_2 = RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5, 0, 0, 0 */
            float4(0.0)
        );
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4((float3(self_2.e4) * right_anti_dual.group3.xyz) - (float3(right_anti_dual.e4) * self_2.group0.xyz), 0.0), 
            /* e23, e31, e12, e45 */
            (self_2.group0.yzxw * float4(right_anti_dual.group3.zxy, right_anti_dual.e5)) - float4((self_2.group0.zxy * right_anti_dual.group3.yzx), self_2.e5 * right_anti_dual.e4), 
            /* e15, e25, e35, e1234 */
            float4(self_2.e5 * right_anti_dual.e1 * -1.0, self_2.e5 * right_anti_dual.e2 * -1.0, self_2.e5 * right_anti_dual.e3 * -1.0, (self_2.e2 * right_anti_dual.e431) + (self_2.e3 * right_anti_dual.e412) + (self_2.e4 * right_anti_dual.e321)) + (self_2.group0.xyzx * float4(right_anti_dual.group2.www, right_anti_dual.e423)), 
            /* e4235, e4315, e4125, e3215 */
            float4((self_2.e3 * right_anti_dual.e425) + (self_2.e4 * right_anti_dual.e235), (self_2.e1 * right_anti_dual.e435) + (self_2.e4 * right_anti_dual.e315), (self_2.e2 * right_anti_dual.e415) + (self_2.e4 * right_anti_dual.e125), -(self_2.e3 * right_anti_dual.e125) - (self_2.e5 * right_anti_dual.e321)) - (self_2.group0.yzxx * float4(right_anti_dual.group1.zxy, right_anti_dual.e235)) - float4((float3(self_2.e5) * right_anti_dual.group0.xyz), self_2.e2 * right_anti_dual.e315)
        );
    }
}
