extension AntiCircleRotor {
    // Fancy infix trick
    func operator/(_rhs: AntiSupportPrefixOrPostfix) -> DipoleInversion {
        return this.anti_support();
    }
}
extension AntiCircleRotor: AntiSupport {
    associatedtype Output = DipoleInversion;
    func anti_support() -> Output {
        Plane right_complement = Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), 1.0)
        );
        CircleRotor right_dual = CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(this.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            this.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4((right_dual.group0.xyz.zxy * right_complement.group0.yzx) - (right_dual.group0.xyz.yzx * right_complement.group0.zxy), 0.0), 
            /* e23, e31, e12, e45 */
            float4(right_dual.e423 * right_complement.e3215, right_dual.e431 * right_complement.e3215, right_dual.e412 * right_complement.e3215, -(right_dual.e425 * right_complement.e4315) - (right_dual.e435 * right_complement.e4125)) - (right_dual.group1.wwwx * right_complement.group0.xyzx), 
            /* e15, e25, e35, e1234 */
            float4((float3(right_complement.e3215) * right_dual.group1.xyz), 0.0) + float4((right_dual.group2.yzx * right_complement.group0.zxy), 0.0) - float4((right_dual.group2.zxy * right_complement.group0.yzx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.e12345) * right_complement.group0
        );
    }
}
extension AntiDipoleInversion {
    // Fancy infix trick
    func operator/(_rhs: AntiSupportPrefixOrPostfix) -> AntiDipoleInversion {
        return this.anti_support();
    }
    func operator/=(const AntiSupportPrefixOrPostfix& _rhs) -> AntiDipoleInversion& {
        this = this.anti_support();
        return *this;
    }
}
extension AntiDipoleInversion: AntiSupport {
    associatedtype Output = AntiDipoleInversion;
    func anti_support() -> Output {
        Plane right_complement = Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), 1.0)
        );
        DipoleInversion right_dual = DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            this.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            this.group2 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group3 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(right_dual.e1234) * right_complement.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            float4(((right_dual.group3.yzx * right_complement.group0.zxy) - (right_dual.group3.zxy * right_complement.group0.yzx)), right_dual.e1234 * right_complement.e3215 * -1.0), 
            /* e235, e315, e125, e4 */
            float4(right_dual.e4235 * right_complement.e3215 * -1.0, right_dual.e4315 * right_complement.e3215 * -1.0, right_dual.e4125 * right_complement.e3215 * -1.0, (right_dual.e42 * right_complement.e4315) + (right_dual.e43 * right_complement.e4125)) + (right_complement.group0.xyzx * float4(right_dual.group3.www, right_dual.e41)), 
            /* e1, e2, e3, e5 */
            float4(right_dual.e12 * right_complement.e4315, right_dual.e23 * right_complement.e4125, right_dual.e31 * right_complement.e4235, -(right_dual.e25 * right_complement.e4315) - (right_dual.e35 * right_complement.e4125)) - (float4(right_complement.e3215) * float4(right_dual.group0.xyz, right_dual.e45)) - (right_complement.group0.zxyx * float4(right_dual.group1.yzx, right_dual.e15))
        );
    }
}
extension AntiDualNum {
    // Fancy infix trick
    func operator/(_rhs: AntiSupportPrefixOrPostfix) -> Plane {
        return this.anti_support();
    }
}
extension AntiDualNum: AntiSupport {
    associatedtype Output = Plane;
    func anti_support() -> Output {
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * float4(float3(0.0), 1.0)
        );
    }
}
extension AntiFlatPoint {
    // Fancy infix trick
    func operator/(_rhs: AntiSupportPrefixOrPostfix) -> DualNum {
        return this.anti_support();
    }
}
extension AntiFlatPoint: AntiSupport {
    associatedtype Output = DualNum;
    func anti_support() -> Output {
        Plane right_complement = Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), 1.0)
        );
        FlatPoint right_dual = FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            this.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(float2(-(right_dual.e15 * right_complement.e4235) - (right_dual.e25 * right_complement.e4315) - (right_dual.e35 * right_complement.e4125) - (right_dual.e45 * right_complement.e3215), 0.0), 0.0, 0.0)
        );
    }
}
extension AntiFlector {
    // Fancy infix trick
    func operator/(_rhs: AntiSupportPrefixOrPostfix) -> Motor {
        return this.anti_support();
    }
}
extension AntiFlector: AntiSupport {
    associatedtype Output = Motor;
    func anti_support() -> Output {
        Plane right_complement = Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), 1.0)
        );
        Flector right_dual = Flector.from_groups(
            /* e15, e25, e35, e45 */
            this.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(((right_dual.group1.yzx * right_complement.group0.zxy) - (right_dual.group1.zxy * right_complement.group0.yzx)), 0.0), 
            /* e235, e315, e125, e5 */
            float4(right_dual.e3215 * right_complement.e4235, right_dual.e3215 * right_complement.e4315, right_dual.e3215 * right_complement.e4125, -(right_dual.e25 * right_complement.e4315) - (right_dual.e35 * right_complement.e4125) - (right_dual.e45 * right_complement.e3215)) - (right_complement.group0.wwwx * float4(right_dual.group1.xyz, right_dual.e15))
        );
    }
}
extension AntiLine {
    // Fancy infix trick
    func operator/(_rhs: AntiSupportPrefixOrPostfix) -> FlatPoint {
        return this.anti_support();
    }
}
extension AntiLine: AntiSupport {
    associatedtype Output = FlatPoint;
    func anti_support() -> Output {
        Plane right_complement = Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), 1.0)
        );
        Line right_dual = Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(this.group0.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(this.group1.xyz * float3(-1.0), 0.0)
        );
        return FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            float4((right_dual.e415 * right_complement.e3215) + (right_dual.e315 * right_complement.e4125), (right_dual.e425 * right_complement.e3215) + (right_dual.e125 * right_complement.e4235), (right_dual.e435 * right_complement.e3215) + (right_dual.e235 * right_complement.e4315), -(right_dual.e425 * right_complement.e4315) - (right_dual.e435 * right_complement.e4125)) - (right_complement.group0.yzxx * float4(right_dual.group1.xyz.zxy, right_dual.e415))
        );
    }
}
extension AntiMotor {
    // Fancy infix trick
    func operator/(_rhs: AntiSupportPrefixOrPostfix) -> Flector {
        return this.anti_support();
    }
}
extension AntiMotor: AntiSupport {
    associatedtype Output = Flector;
    func anti_support() -> Output {
        Plane right_complement = Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), 1.0)
        );
        Motor right_dual = Motor.from_groups(
            /* e415, e425, e435, e12345 */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4((right_dual.e415 * right_complement.e3215) + (right_dual.e315 * right_complement.e4125), (right_dual.e425 * right_complement.e3215) + (right_dual.e125 * right_complement.e4235), (right_dual.e435 * right_complement.e3215) + (right_dual.e235 * right_complement.e4315), -(right_dual.e425 * right_complement.e4315) - (right_dual.e435 * right_complement.e4125)) - (right_complement.group0.yzxx * float4(right_dual.group1.zxy, right_dual.e415)), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.e12345) * right_complement.group0
        );
    }
}
extension AntiPlane {
    // Fancy infix trick
    func operator/(_rhs: AntiSupportPrefixOrPostfix) -> Line {
        return this.anti_support();
    }
}
extension AntiPlane: AntiSupport {
    associatedtype Output = Line;
    func anti_support() -> Output {
        Plane right_complement = Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), 1.0)
        );
        Plane right_dual = Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            this.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Line.from_groups(
            /* e415, e425, e435, 0 */
            float4((right_complement.group0.zxy * right_dual.group0.yzx) - (right_complement.group0.yzx * right_dual.group0.zxy), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(right_dual.e3215) * right_complement.group0.xyz) - (float3(right_complement.e3215) * right_dual.group0.xyz), 0.0)
        );
    }
}
extension Circle {
    // Fancy infix trick
    func operator/(_rhs: AntiSupportPrefixOrPostfix) -> RoundPoint {
        return this.anti_support();
    }
}
extension Circle: AntiSupport {
    associatedtype Output = RoundPoint;
    func anti_support() -> Output {
        Plane right_complement = Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), 1.0)
        );
        Dipole right_dual = Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            this.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, 0 */
            float4(this.group2.xyz, 0.0)
        );
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(-(right_dual.e41 * right_complement.e3215) - (right_dual.e31 * right_complement.e4125), -(right_dual.e42 * right_complement.e3215) - (right_dual.e12 * right_complement.e4235), -(right_dual.e43 * right_complement.e3215) - (right_dual.e23 * right_complement.e4315), (right_dual.e42 * right_complement.e4315) + (right_dual.e43 * right_complement.e4125)) + (right_complement.group0.yzxx * float4(right_dual.group1.zxy, right_dual.e41)), 
            /* e5, 0, 0, 0 */
            float4(-(right_dual.e45 * right_complement.e3215) - (right_dual.e15 * right_complement.e4235) - (right_dual.e25 * right_complement.e4315) - (right_dual.e35 * right_complement.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension CircleRotor {
    // Fancy infix trick
    func operator/(_rhs: AntiSupportPrefixOrPostfix) -> RoundPoint {
        return this.anti_support();
    }
}
extension CircleRotor: AntiSupport {
    associatedtype Output = RoundPoint;
    func anti_support() -> Output {
        Plane right_complement = Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), 1.0)
        );
        AntiCircleRotor right_dual = AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e23, e31, e12, e45 */
            this.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            this.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(-(right_dual.e41 * right_complement.e3215) - (right_dual.e31 * right_complement.e4125), -(right_dual.e42 * right_complement.e3215) - (right_dual.e12 * right_complement.e4235), -(right_dual.e43 * right_complement.e3215) - (right_dual.e23 * right_complement.e4315), (right_dual.e42 * right_complement.e4315) + (right_dual.e43 * right_complement.e4125)) + (right_complement.group0.yzxx * float4(right_dual.group1.zxy, right_dual.e41)), 
            /* e5, 0, 0, 0 */
            float4(-(right_dual.e45 * right_complement.e3215) - (right_dual.e15 * right_complement.e4235) - (right_dual.e25 * right_complement.e4315) - (right_dual.e35 * right_complement.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension Dipole {
    // Fancy infix trick
    func operator/(_rhs: AntiSupportPrefixOrPostfix) -> Dipole {
        return this.anti_support();
    }
    func operator/=(const AntiSupportPrefixOrPostfix& _rhs) -> Dipole& {
        this = this.anti_support();
        return *this;
    }
}
extension Dipole: AntiSupport {
    associatedtype Output = Dipole;
    func anti_support() -> Output {
        Plane right_complement = Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), 1.0)
        );
        Circle right_dual = Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(this.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, 0 */
            float4(this.group2.xyz * float3(-1.0), 0.0)
        );
        return Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4((right_dual.group0.xyz.zxy * right_complement.group0.yzx) - (right_dual.group0.xyz.yzx * right_complement.group0.zxy), 0.0), 
            /* e23, e31, e12, e45 */
            float4(right_dual.e423 * right_complement.e3215, right_dual.e431 * right_complement.e3215, right_dual.e412 * right_complement.e3215, -(right_dual.e425 * right_complement.e4315) - (right_dual.e435 * right_complement.e4125)) - (right_dual.group1.wwwx * right_complement.group0.xyzx), 
            /* e15, e25, e35, 0 */
            float4((float3(right_complement.e3215) * right_dual.group1.xyz) + (right_dual.group2.xyz.yzx * right_complement.group0.zxy) - (right_dual.group2.xyz.zxy * right_complement.group0.yzx), 0.0)
        );
    }
}
extension DipoleInversion {
    // Fancy infix trick
    func operator/(_rhs: AntiSupportPrefixOrPostfix) -> AntiCircleRotor {
        return this.anti_support();
    }
}
extension DipoleInversion: AntiSupport {
    associatedtype Output = AntiCircleRotor;
    func anti_support() -> Output {
        Plane right_complement = Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), 1.0)
        );
        AntiDipoleInversion right_dual = AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(this.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e4 */
            this.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            this.group3 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4((right_dual.group0.xyz.zxy * right_complement.group0.yzx) - (right_dual.group0.xyz.yzx * right_complement.group0.zxy), 0.0), 
            /* e23, e31, e12, e45 */
            float4(right_dual.e423 * right_complement.e3215, right_dual.e431 * right_complement.e3215, right_dual.e412 * right_complement.e3215, -(right_dual.e425 * right_complement.e4315) - (right_dual.e435 * right_complement.e4125)) - (right_dual.group1.wwwx * right_complement.group0.xyzx), 
            /* e15, e25, e35, scalar */
            float4(right_dual.e125 * right_complement.e4315 * -1.0, right_dual.e235 * right_complement.e4125 * -1.0, right_dual.e315 * right_complement.e4235 * -1.0, (right_dual.e2 * right_complement.e4315) + (right_dual.e3 * right_complement.e4125)) + (float4(right_complement.e3215) * float4(right_dual.group1.xyz, right_dual.e4)) + (right_complement.group0.zxyx * float4(right_dual.group2.yzx, right_dual.e1))
        );
    }
}
extension DualNum {
    // Fancy infix trick
    func operator/(_rhs: AntiSupportPrefixOrPostfix) -> AntiFlatPoint {
        return this.anti_support();
    }
}
extension DualNum: AntiSupport {
    associatedtype Output = AntiFlatPoint;
    func anti_support() -> Output {
        AntiDualNum right_dual = AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(this.group0.xy * float2(-1.0), 0.0, 0.0)
        );
        return AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            float4(right_dual.group0.xy.xx, right_dual.e3215, 0.0) * float4(float3(0.0), 0.0) * float4(float3(1.0), 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension FlatPoint {
    // Fancy infix trick
    func operator/(_rhs: AntiSupportPrefixOrPostfix) -> AntiLine {
        return this.anti_support();
    }
}
extension FlatPoint: AntiSupport {
    associatedtype Output = AntiLine;
    func anti_support() -> Output {
        Plane right_complement = Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), 1.0)
        );
        AntiFlatPoint right_dual = AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(float3(right_dual.e321) * right_complement.group0.xyz * float3(-1.0), 0.0), 
            /* e15, e25, e35, 0 */
            float4((right_dual.group0.yzx * right_complement.group0.zxy) - (right_dual.group0.zxy * right_complement.group0.yzx), 0.0)
        );
    }
}
extension Flector {
    // Fancy infix trick
    func operator/(_rhs: AntiSupportPrefixOrPostfix) -> AntiMotor {
        return this.anti_support();
    }
}
extension Flector: AntiSupport {
    associatedtype Output = AntiMotor;
    func anti_support() -> Output {
        Plane right_complement = Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), 1.0)
        );
        AntiFlector right_dual = AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4((right_dual.group0.www * right_complement.group0.xyz * float3(-1.0)), (right_dual.e1 * right_complement.e4235) + (right_dual.e2 * right_complement.e4315) + (right_dual.e3 * right_complement.e4125)), 
            /* e15, e25, e35, e3215 */
            float4(((right_dual.group0.yzx * right_complement.group0.zxy) - (right_dual.group0.zxy * right_complement.group0.yzx)), 0.0)
        );
    }
}
extension Line {
    // Fancy infix trick
    func operator/(_rhs: AntiSupportPrefixOrPostfix) -> AntiPlane {
        return this.anti_support();
    }
}
extension Line: AntiSupport {
    associatedtype Output = AntiPlane;
    func anti_support() -> Output {
        Plane right_complement = Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), 1.0)
        );
        AntiLine right_dual = AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(this.group0.xyz, 0.0), 
            /* e15, e25, e35, 0 */
            float4(this.group1.xyz, 0.0)
        );
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4(right_dual.e12 * right_complement.e4315, right_dual.e23 * right_complement.e4125, right_dual.e31 * right_complement.e4235, -(right_dual.e25 * right_complement.e4315) - (right_dual.e35 * right_complement.e4125)) - (right_complement.group0.zxyx * float4(right_dual.group0.xyz.yzx, right_dual.e15))
        );
    }
}
extension Motor {
    // Fancy infix trick
    func operator/(_rhs: AntiSupportPrefixOrPostfix) -> AntiFlector {
        return this.anti_support();
    }
}
extension Motor: AntiSupport {
    associatedtype Output = AntiFlector;
    func anti_support() -> Output {
        Plane right_complement = Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), 1.0)
        );
        AntiMotor right_dual = AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            this.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            this.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(float3(1.0), 0.0) * float4(right_dual.group1.www, 0.0) * float4(right_complement.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e1, e2, e3, e5 */
            float4(right_dual.e12 * right_complement.e4315, right_dual.e23 * right_complement.e4125, right_dual.e31 * right_complement.e4235, -(right_dual.e25 * right_complement.e4315) - (right_dual.e35 * right_complement.e4125)) - (right_complement.group0.zxyx * float4(right_dual.group0.yzx, right_dual.e15))
        );
    }
}
extension MultiVector {
    // Fancy infix trick
    func operator/(_rhs: AntiSupportPrefixOrPostfix) -> MultiVector {
        return this.anti_support();
    }
    func operator/=(const AntiSupportPrefixOrPostfix& _rhs) -> MultiVector& {
        this = this.anti_support();
        return *this;
    }
}
extension MultiVector: AntiSupport {
    associatedtype Output = MultiVector;
    func anti_support() -> Output {
        Plane right_complement = Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), 1.0)
        );
        MultiVector right_dual = MultiVector.from_groups(
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
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2((right_dual.e1 * right_complement.e4235) + (right_dual.e2 * right_complement.e4315) + (right_dual.e3 * right_complement.e4125) + (right_dual.e4 * right_complement.e3215), 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(-(right_dual.e41 * right_complement.e3215) - (right_dual.e31 * right_complement.e4125), -(right_dual.e42 * right_complement.e3215) - (right_dual.e12 * right_complement.e4235), -(right_dual.e43 * right_complement.e3215) - (right_dual.e23 * right_complement.e4315), (right_dual.e42 * right_complement.e4315) + (right_dual.e43 * right_complement.e4125)) + (right_complement.group0.yzxx * float4(right_dual.group5.xyz.zxy, right_dual.e41)), 
            /* e5, 0, 0, 0 */
            float4(-(right_dual.e15 * right_complement.e4235) - (right_dual.e25 * right_complement.e4315) - (right_dual.e35 * right_complement.e4125) - (right_dual.e45 * right_complement.e3215), 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4((right_dual.e415 * right_complement.e3215) + (right_dual.e315 * right_complement.e4125), (right_dual.e425 * right_complement.e3215) + (right_dual.e125 * right_complement.e4235), (right_dual.e435 * right_complement.e3215) + (right_dual.e235 * right_complement.e4315), -(right_dual.e425 * right_complement.e4315) - (right_dual.e435 * right_complement.e4125)) - (right_complement.group0.yzxx * float4(right_dual.group8.xyz.zxy, right_dual.e415)), 
            /* e41, e42, e43, 0 */
            float4((right_dual.group7.xyz.zxy * right_complement.group0.yzx) - (right_dual.group7.xyz.yzx * right_complement.group0.zxy), 0.0), 
            /* e23, e31, e12, 0 */
            float4((float3(right_complement.e3215) * right_dual.group7.xyz) - (float3(right_dual.e321) * right_complement.group0.xyz), 0.0), 
            /* e415, e425, e435, e321 */
            float4(((right_dual.group9.yzx * right_complement.group0.zxy) - (right_dual.group9.zxy * right_complement.group0.yzx)), right_dual.e1234 * right_complement.e3215 * -1.0), 
            /* e423, e431, e412, 0 */
            float4(float3(right_dual.e1234) * right_complement.group0.xyz * float3(-1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4((float3(right_dual.e3215) * right_complement.group0.xyz) - (float3(right_complement.e3215) * right_dual.group9.xyz), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.e12345) * right_complement.group0, 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension Plane {
    // Fancy infix trick
    func operator/(_rhs: AntiSupportPrefixOrPostfix) -> Scalar {
        return this.anti_support();
    }
}
extension Plane: AntiSupport {
    associatedtype Output = Scalar;
    func anti_support() -> Output {
        Plane right_complement = Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), 1.0)
        );
        AntiPlane right_dual = AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((right_dual.e1 * right_complement.e4235) + (right_dual.e2 * right_complement.e4315) + (right_dual.e3 * right_complement.e4125), 0.0, 0.0, 0.0)
        );
    }
}
extension RoundPoint {
    // Fancy infix trick
    func operator/(_rhs: AntiSupportPrefixOrPostfix) -> Circle {
        return this.anti_support();
    }
}
extension RoundPoint: AntiSupport {
    associatedtype Output = Circle;
    func anti_support() -> Output {
        Plane right_complement = Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), 1.0)
        );
        Sphere right_dual = Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(this.group0.xyz, this.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234, 0, 0, 0 */
            float4(this.e4 * -1.0, 0.0, 0.0, 0.0)
        );
        return Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(right_dual.e1234) * right_complement.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            float4(((right_complement.group0.zxy * right_dual.group0.yzx) - (right_complement.group0.yzx * right_dual.group0.zxy)), right_complement.e3215 * right_dual.e1234 * -1.0), 
            /* e235, e315, e125, 0 */
            float4((float3(right_dual.e3215) * right_complement.group0.xyz) - (float3(right_complement.e3215) * right_dual.group0.xyz), 0.0)
        );
    }
}
extension Scalar {
    // Fancy infix trick
    func operator/(_rhs: AntiSupportPrefixOrPostfix) -> Plane {
        return this.anti_support();
    }
}
extension Scalar: AntiSupport {
    associatedtype Output = Plane;
    func anti_support() -> Output {
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * float4(float3(0.0), 1.0)
        );
    }
}
extension Sphere {
    // Fancy infix trick
    func operator/(_rhs: AntiSupportPrefixOrPostfix) -> Scalar {
        return this.anti_support();
    }
}
extension Sphere: AntiSupport {
    associatedtype Output = Scalar;
    func anti_support() -> Output {
        Plane right_complement = Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), 1.0)
        );
        RoundPoint right_dual = RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.group0.xyz, this.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5, 0, 0, 0 */
            float4(this.e3215, 0.0, 0.0, 0.0)
        );
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4((right_complement.e4235 * right_dual.e1) + (right_complement.e4315 * right_dual.e2) + (right_complement.e4125 * right_dual.e3) + (right_complement.e3215 * right_dual.e4), 0.0, 0.0, 0.0)
        );
    }
}
extension VersorEven {
    // Fancy infix trick
    func operator/(_rhs: AntiSupportPrefixOrPostfix) -> AntiDipoleInversion {
        return this.anti_support();
    }
}
extension VersorEven: AntiSupport {
    associatedtype Output = AntiDipoleInversion;
    func anti_support() -> Output {
        Plane right_complement = Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), 1.0)
        );
        VersorOdd right_dual = VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            this.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e23, e31, e12, e45 */
            this.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            float4(this.group2.xyz, this.e4) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.group3.xyz, this.e5) * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(right_dual.e1234) * right_complement.group0.xyz * float3(-1.0), 0.0), 
            /* e415, e425, e435, e321 */
            float4(((right_complement.group0.zxy * right_dual.group3.yzx) - (right_complement.group0.yzx * right_dual.group3.zxy)), right_complement.e3215 * right_dual.e1234 * -1.0), 
            /* e235, e315, e125, e4 */
            float4(right_complement.e3215 * right_dual.e4235 * -1.0, right_complement.e3215 * right_dual.e4315 * -1.0, right_complement.e3215 * right_dual.e4125 * -1.0, (right_complement.e4315 * right_dual.e42) + (right_complement.e4125 * right_dual.e43)) + (right_complement.group0.xyzx * float4(right_dual.group3.www, right_dual.e41)), 
            /* e1, e2, e3, e5 */
            float4(right_complement.e4315 * right_dual.e12, right_complement.e4125 * right_dual.e23, right_complement.e4235 * right_dual.e31, -(right_complement.e4125 * right_dual.e35) - (right_complement.e3215 * right_dual.e45)) - (right_complement.group0.zxyx * float4(right_dual.group1.yzx, right_dual.e15)) - (right_complement.group0.wwwy * float4(right_dual.group0.xyz, right_dual.e25))
        );
    }
}
extension VersorOdd {
    // Fancy infix trick
    func operator/(_rhs: AntiSupportPrefixOrPostfix) -> VersorOdd {
        return this.anti_support();
    }
    func operator/=(const AntiSupportPrefixOrPostfix& _rhs) -> VersorOdd& {
        this = this.anti_support();
        return *this;
    }
}
extension VersorOdd: AntiSupport {
    associatedtype Output = VersorOdd;
    func anti_support() -> Output {
        Plane right_complement = Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), 1.0)
        );
        VersorEven right_dual = VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e415, e425, e435, e321 */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            float4(this.group2.xyz, this.e3215) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(this.group3.xyz, this.e1234) * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            float4(right_complement.e4125 * right_dual.e431 * -1.0, right_complement.e4235 * right_dual.e412 * -1.0, right_complement.e4315 * right_dual.e423 * -1.0, (right_complement.e4315 * right_dual.e2) + (right_complement.e4125 * right_dual.e3) + (right_complement.e3215 * right_dual.e4)) + (right_complement.group0.yzxx * float4(right_dual.group0.zxy, right_dual.e1)), 
            /* e23, e31, e12, e45 */
            float4(right_complement.e3215 * right_dual.e423, right_complement.e3215 * right_dual.e431, right_complement.e3215 * right_dual.e412, -(right_complement.e4315 * right_dual.e425) - (right_complement.e4125 * right_dual.e435)) - (right_complement.group0.xyzx * right_dual.group1.wwwx), 
            /* e15, e25, e35, e1234 */
            float4((float3(right_complement.e3215) * right_dual.group1.xyz), 0.0) + float4((right_complement.group0.zxy * right_dual.group2.yzx), 0.0) - float4((right_complement.group0.yzx * right_dual.group2.zxy), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.e12345) * right_complement.group0
        );
    }
}
