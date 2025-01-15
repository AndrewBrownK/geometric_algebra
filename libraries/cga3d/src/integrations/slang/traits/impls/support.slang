implementing cga3d;
using traits;
// Fancy postfix trick
public Sphere operator /(AntiCircleRotor lhs, SupportPrefixOrPostfix rhs)  {
    return lhs.support();
    }
// Fancy prefix trick
public Sphere operator /(SupportPrefixOrPostfix lhs, AntiCircleRotor rhs) {
    return rhs.support();
}
public extension AntiCircleRotor {
    public Sphere support() {
        CircleRotor right_anti_dual = CircleRotor(
            /* e423, e431, e412 */
            this.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            this.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        RoundPoint self_2 = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5 */
            0.0
        );
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e425 * self_2.e3) + (right_anti_dual.e235 * self_2.e4), (right_anti_dual.e435 * self_2.e1) + (right_anti_dual.e315 * self_2.e4), (right_anti_dual.e415 * self_2.e2) + (right_anti_dual.e125 * self_2.e4), -(right_anti_dual.e315 * self_2.e2) - (right_anti_dual.e125 * self_2.e3)) - (float4(self_2.e5) * float4(right_anti_dual.group0.xyz, right_anti_dual.e321)) - (self_2.group0.yzxx * float4(right_anti_dual.group1.zxy, right_anti_dual.e235)), 
            /* e1234 */
            (right_anti_dual.e423 * self_2.e1) + (right_anti_dual.e431 * self_2.e2) + (right_anti_dual.e412 * self_2.e3) + (right_anti_dual.e321 * self_2.e4)
        );
    }
}
// Fancy postfix trick
public CircleRotor operator /(AntiDipoleInversion lhs, SupportPrefixOrPostfix rhs)  {
    return lhs.support();
    }
// Fancy prefix trick
public CircleRotor operator /(SupportPrefixOrPostfix lhs, AntiDipoleInversion rhs) {
    return rhs.support();
}
public extension AntiDipoleInversion {
    public CircleRotor support() {
        DipoleInversion right_anti_dual = DipoleInversion(
            /* e41, e42, e43 */
            this.group0.xyz, 
            /* e23, e31, e12, e45 */
            this.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            this.group2 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group3 * float4(1.0, 1.0, 1.0, -1.0)
        );
        RoundPoint self_2 = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5 */
            0.0
        );
        return CircleRotor(
            /* e423, e431, e412 */
            (float3(self_2.e4) * right_anti_dual.group1.xyz) + (right_anti_dual.group0.xyz.yzx * self_2.group0.zxy) - (right_anti_dual.group0.xyz.zxy * self_2.group0.yzx), 
            /* e415, e425, e435, e321 */
            float4((right_anti_dual.e41 * self_2.e5) + (right_anti_dual.e15 * self_2.e4), (right_anti_dual.e42 * self_2.e5) + (right_anti_dual.e25 * self_2.e4), (right_anti_dual.e43 * self_2.e5) + (right_anti_dual.e35 * self_2.e4), -(right_anti_dual.e31 * self_2.e2) - (right_anti_dual.e12 * self_2.e3)) - (right_anti_dual.group1.wwwx * self_2.group0.xyzx), 
            /* e235, e315, e125, e12345 */
            float4(right_anti_dual.e25 * self_2.e3 * -1.0, right_anti_dual.e35 * self_2.e1 * -1.0, right_anti_dual.e15 * self_2.e2 * -1.0, (right_anti_dual.e4315 * self_2.e2) + (right_anti_dual.e4125 * self_2.e3) + (right_anti_dual.e3215 * self_2.e4)) + (float4(self_2.e5) * float4(right_anti_dual.group1.xyz, right_anti_dual.e1234)) + (self_2.group0.yzxx * float4(right_anti_dual.group2.zxy, right_anti_dual.e4235))
        );
    }
}
// Fancy postfix trick
public FlatPoint operator /(AntiDualNum lhs, SupportPrefixOrPostfix rhs)  {
    return lhs.support();
    }
// Fancy prefix trick
public FlatPoint operator /(SupportPrefixOrPostfix lhs, AntiDualNum rhs) {
    return rhs.support();
}
public extension AntiDualNum {
    public FlatPoint support() {
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4(this.e3215) * float4(float3(0.0), 1.0)
        );
    }
}
// Fancy postfix trick
public Line operator /(AntiFlatPoint lhs, SupportPrefixOrPostfix rhs)  {
    return lhs.support();
    }
// Fancy prefix trick
public Line operator /(SupportPrefixOrPostfix lhs, AntiFlatPoint rhs) {
    return rhs.support();
}
public extension AntiFlatPoint {
    public Line support() {
        FlatPoint right_anti_dual = FlatPoint(
            /* e15, e25, e35, e45 */
            this.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        RoundPoint self_2 = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5 */
            0.0
        );
        return Line(
            /* e415, e425, e435 */
            (float3(self_2.e4) * right_anti_dual.group0.xyz) - (float3(right_anti_dual.e45) * self_2.group0.xyz), 
            /* e235, e315, e125 */
            (right_anti_dual.group0.zxy * self_2.group0.yzx) - (right_anti_dual.group0.yzx * self_2.group0.zxy)
        );
    }
}
// Fancy postfix trick
public Motor operator /(AntiFlector lhs, SupportPrefixOrPostfix rhs)  {
    return lhs.support();
    }
// Fancy prefix trick
public Motor operator /(SupportPrefixOrPostfix lhs, AntiFlector rhs) {
    return rhs.support();
}
public extension AntiFlector {
    public Motor support() {
        Flector right_anti_dual = Flector(
            /* e15, e25, e35, e45 */
            this.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        RoundPoint self_2 = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5 */
            0.0
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(right_anti_dual.e45 * self_2.e1 * -1.0, right_anti_dual.e45 * self_2.e2 * -1.0, right_anti_dual.e45 * self_2.e3 * -1.0, (right_anti_dual.e4315 * self_2.e2) + (right_anti_dual.e4125 * self_2.e3) + (right_anti_dual.e3215 * self_2.e4)) + (self_2.group0.wwwx * float4(right_anti_dual.group0.xyz, right_anti_dual.e4235)), 
            /* e235, e315, e125, e5 */
            float4(((right_anti_dual.group0.zxy * self_2.group0.yzx) - (right_anti_dual.group0.yzx * self_2.group0.zxy)), 0.0)
        );
    }
}
// Fancy postfix trick
public Plane operator /(AntiLine lhs, SupportPrefixOrPostfix rhs)  {
    return lhs.support();
    }
// Fancy prefix trick
public Plane operator /(SupportPrefixOrPostfix lhs, AntiLine rhs) {
    return rhs.support();
}
public extension AntiLine {
    public Plane support() {
        Line right_anti_dual = Line(
            /* e415, e425, e435 */
            this.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            this.group1.xyz * float3(-1.0)
        );
        RoundPoint self_2 = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5 */
            0.0
        );
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e425 * self_2.e3) + (right_anti_dual.e235 * self_2.e4), (right_anti_dual.e435 * self_2.e1) + (right_anti_dual.e315 * self_2.e4), (right_anti_dual.e415 * self_2.e2) + (right_anti_dual.e125 * self_2.e4), -(right_anti_dual.e315 * self_2.e2) - (right_anti_dual.e125 * self_2.e3)) - (self_2.group0.yzxx * float4(right_anti_dual.group0.xyz.zxy, right_anti_dual.e235))
        );
    }
}
// Fancy postfix trick
public Flector operator /(AntiMotor lhs, SupportPrefixOrPostfix rhs)  {
    return lhs.support();
    }
// Fancy prefix trick
public Flector operator /(SupportPrefixOrPostfix lhs, AntiMotor rhs) {
    return rhs.support();
}
public extension AntiMotor {
    public Flector support() {
        Motor right_anti_dual = Motor(
            /* e415, e425, e435, e12345 */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        RoundPoint self_2 = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5 */
            0.0
        );
        return Flector(
            /* e15, e25, e35, e45 */
            float4(right_anti_dual.e5) * self_2.group0, 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e425 * self_2.e3) + (right_anti_dual.e235 * self_2.e4), (right_anti_dual.e435 * self_2.e1) + (right_anti_dual.e315 * self_2.e4), (right_anti_dual.e415 * self_2.e2) + (right_anti_dual.e125 * self_2.e4), -(right_anti_dual.e315 * self_2.e2) - (right_anti_dual.e125 * self_2.e3)) - (self_2.group0.yzxx * float4(right_anti_dual.group0.zxy, right_anti_dual.e235))
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(AntiPlane lhs, SupportPrefixOrPostfix rhs)  {
    return lhs.support();
    }
// Fancy prefix trick
public AntiScalar operator /(SupportPrefixOrPostfix lhs, AntiPlane rhs) {
    return rhs.support();
}
public extension AntiPlane {
    public AntiScalar support() {
        Plane right_anti_dual = Plane(
            /* e4235, e4315, e4125, e3215 */
            this.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        RoundPoint self_2 = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5 */
            0.0
        );
        return AntiScalar(
            /* e12345 */
            (right_anti_dual.e4235 * self_2.e1) + (right_anti_dual.e4315 * self_2.e2) + (right_anti_dual.e4125 * self_2.e3) + (right_anti_dual.e3215 * self_2.e4)
        );
    }
}
// Fancy postfix trick
public RoundPoint operator /(AntiScalar lhs, SupportPrefixOrPostfix rhs)  {
    return lhs.support();
    }
// Fancy prefix trick
public RoundPoint operator /(SupportPrefixOrPostfix lhs, AntiScalar rhs) {
    return rhs.support();
}
public extension AntiScalar {
    public RoundPoint support() {
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(this.e12345 * -1.0) * float4(float3(0.0), 1.0), 
            /* e5 */
            0.0
        );
    }
}
// Fancy postfix trick
public Circle operator /(Circle lhs, SupportPrefixOrPostfix rhs)  {
    return lhs.support();
    }
// Fancy prefix trick
public Circle operator /(SupportPrefixOrPostfix lhs, Circle rhs) {
    return rhs.support();
}
public extension Circle {
    public Circle support() {
        Dipole right_anti_dual = Dipole(
            /* e41, e42, e43 */
            this.group0.xyz, 
            /* e23, e31, e12, e45 */
            this.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            this.group2.xyz
        );
        RoundPoint self_2 = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5 */
            0.0
        );
        return Circle(
            /* e423, e431, e412 */
            (float3(self_2.e4) * right_anti_dual.group1.xyz) + (right_anti_dual.group0.xyz.yzx * self_2.group0.zxy) - (right_anti_dual.group0.xyz.zxy * self_2.group0.yzx), 
            /* e415, e425, e435, e321 */
            float4((right_anti_dual.e41 * self_2.e5) + (right_anti_dual.e15 * self_2.e4), (right_anti_dual.e42 * self_2.e5) + (right_anti_dual.e25 * self_2.e4), (right_anti_dual.e43 * self_2.e5) + (right_anti_dual.e35 * self_2.e4), -(right_anti_dual.e31 * self_2.e2) - (right_anti_dual.e12 * self_2.e3)) - (right_anti_dual.group1.wwwx * self_2.group0.xyzx), 
            /* e235, e315, e125 */
            (float3(self_2.e5) * right_anti_dual.group1.xyz) + (right_anti_dual.group2.xyz.zxy * self_2.group0.yzx) - (right_anti_dual.group2.xyz.yzx * self_2.group0.zxy)
        );
    }
}
// Fancy postfix trick
public AntiDipoleInversion operator /(CircleRotor lhs, SupportPrefixOrPostfix rhs)  {
    return lhs.support();
    }
// Fancy prefix trick
public AntiDipoleInversion operator /(SupportPrefixOrPostfix lhs, CircleRotor rhs) {
    return rhs.support();
}
public extension CircleRotor {
    public AntiDipoleInversion support() {
        AntiCircleRotor right_anti_dual = AntiCircleRotor(
            /* e41, e42, e43 */
            this.group0.xyz, 
            /* e23, e31, e12, e45 */
            this.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            this.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        RoundPoint self_2 = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5 */
            0.0
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            (float3(self_2.e4) * right_anti_dual.group1.xyz) + (right_anti_dual.group0.xyz.yzx * self_2.group0.zxy) - (right_anti_dual.group0.xyz.zxy * self_2.group0.yzx), 
            /* e415, e425, e435, e321 */
            float4((right_anti_dual.e41 * self_2.e5) + (right_anti_dual.e15 * self_2.e4), (right_anti_dual.e42 * self_2.e5) + (right_anti_dual.e25 * self_2.e4), (right_anti_dual.e43 * self_2.e5) + (right_anti_dual.e35 * self_2.e4), -(right_anti_dual.e31 * self_2.e2) - (right_anti_dual.e12 * self_2.e3)) - (right_anti_dual.group1.wwwx * self_2.group0.xyzx), 
            /* e235, e315, e125, e4 */
            float4(((float3(self_2.e5) * right_anti_dual.group1.xyz) + (right_anti_dual.group2.zxy * self_2.group0.yzx) - (right_anti_dual.group2.yzx * self_2.group0.zxy)), right_anti_dual.scalar * self_2.e4), 
            /* e1, e2, e3, e5 */
            float4(float3(1.0), 0.0) * float4(right_anti_dual.group2.www, 0.0) * float4(self_2.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
// Fancy postfix trick
public Sphere operator /(Dipole lhs, SupportPrefixOrPostfix rhs)  {
    return lhs.support();
    }
// Fancy prefix trick
public Sphere operator /(SupportPrefixOrPostfix lhs, Dipole rhs) {
    return rhs.support();
}
public extension Dipole {
    public Sphere support() {
        Circle right_anti_dual = Circle(
            /* e423, e431, e412 */
            this.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            this.group2.xyz * float3(-1.0)
        );
        RoundPoint self_2 = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5 */
            0.0
        );
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e425 * self_2.e3) + (right_anti_dual.e235 * self_2.e4), (right_anti_dual.e435 * self_2.e1) + (right_anti_dual.e315 * self_2.e4), (right_anti_dual.e415 * self_2.e2) + (right_anti_dual.e125 * self_2.e4), -(right_anti_dual.e321 * self_2.e5) - (right_anti_dual.e125 * self_2.e3)) - (self_2.group0.yzxy * float4(right_anti_dual.group1.zxy, right_anti_dual.e315)) - float4((float3(self_2.e5) * right_anti_dual.group0.xyz), right_anti_dual.e235 * self_2.e1), 
            /* e1234 */
            (right_anti_dual.e423 * self_2.e1) + (right_anti_dual.e431 * self_2.e2) + (right_anti_dual.e412 * self_2.e3) + (right_anti_dual.e321 * self_2.e4)
        );
    }
}
// Fancy postfix trick
public DipoleInversion operator /(DipoleInversion lhs, SupportPrefixOrPostfix rhs)  {
    return lhs.support();
    }
// Fancy prefix trick
public DipoleInversion operator /(SupportPrefixOrPostfix lhs, DipoleInversion rhs) {
    return rhs.support();
}
public extension DipoleInversion {
    public DipoleInversion support() {
        AntiDipoleInversion right_anti_dual = AntiDipoleInversion(
            /* e423, e431, e412 */
            this.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e4 */
            this.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            this.group3 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        RoundPoint self_2 = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5 */
            0.0
        );
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(self_2.e4) * right_anti_dual.group3.xyz) - (float3(right_anti_dual.e4) * self_2.group0.xyz), 
            /* e23, e31, e12, e45 */
            (right_anti_dual.group3.zxyw * self_2.group0.yzxw) - float4((right_anti_dual.group3.yzx * self_2.group0.zxy), right_anti_dual.e4 * self_2.e5), 
            /* e15, e25, e35, e1234 */
            float4(right_anti_dual.e1 * self_2.e5 * -1.0, right_anti_dual.e2 * self_2.e5 * -1.0, right_anti_dual.e3 * self_2.e5 * -1.0, (right_anti_dual.e431 * self_2.e2) + (right_anti_dual.e412 * self_2.e3) + (right_anti_dual.e321 * self_2.e4)) + (self_2.group0.xyzx * float4(right_anti_dual.group3.www, right_anti_dual.e423)), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e425 * self_2.e3) + (right_anti_dual.e235 * self_2.e4), (right_anti_dual.e435 * self_2.e1) + (right_anti_dual.e315 * self_2.e4), (right_anti_dual.e415 * self_2.e2) + (right_anti_dual.e125 * self_2.e4), -(right_anti_dual.e315 * self_2.e2) - (right_anti_dual.e125 * self_2.e3)) - (float4(self_2.e5) * float4(right_anti_dual.group0.xyz, right_anti_dual.e321)) - (self_2.group0.yzxx * float4(right_anti_dual.group1.zxy, right_anti_dual.e235))
        );
    }
}
// Fancy postfix trick
public VersorEven operator /(DualNum lhs, SupportPrefixOrPostfix rhs)  {
    return lhs.support();
    }
// Fancy prefix trick
public VersorEven operator /(SupportPrefixOrPostfix lhs, DualNum rhs) {
    return rhs.support();
}
public extension DualNum {
    public VersorEven support() {
        AntiDualNum right_anti_dual = AntiDualNum(
            /* e3215, scalar */
            this.group0.xy * float2(-1.0)
        );
        RoundPoint self_2 = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5 */
            0.0
        );
        return VersorEven(
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
// Fancy postfix trick
public Sphere operator /(FlatPoint lhs, SupportPrefixOrPostfix rhs)  {
    return lhs.support();
    }
// Fancy prefix trick
public Sphere operator /(SupportPrefixOrPostfix lhs, FlatPoint rhs) {
    return rhs.support();
}
public extension FlatPoint {
    public Sphere support() {
        AntiFlatPoint right_anti_dual = AntiFlatPoint(
            /* e235, e315, e125, e321 */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        RoundPoint self_2 = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5 */
            0.0
        );
        return Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4((float3(self_2.e4) * right_anti_dual.group0.xyz), -(right_anti_dual.e235 * self_2.e1) - (right_anti_dual.e315 * self_2.e2) - (right_anti_dual.e125 * self_2.e3) - (right_anti_dual.e321 * self_2.e5)), 
            /* e1234 */
            right_anti_dual.e321 * self_2.e4
        );
    }
}
// Fancy postfix trick
public DipoleInversion operator /(Flector lhs, SupportPrefixOrPostfix rhs)  {
    return lhs.support();
    }
// Fancy prefix trick
public DipoleInversion operator /(SupportPrefixOrPostfix lhs, Flector rhs) {
    return rhs.support();
}
public extension Flector {
    public DipoleInversion support() {
        AntiFlector right_anti_dual = AntiFlector(
            /* e235, e315, e125, e321 */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        RoundPoint self_2 = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5 */
            0.0
        );
        return DipoleInversion(
            /* e41, e42, e43 */
            float3(self_2.e4) * right_anti_dual.group1.xyz, 
            /* e23, e31, e12, e45 */
            float4(((right_anti_dual.group1.zxy * self_2.group0.yzx) - (right_anti_dual.group1.yzx * self_2.group0.zxy)), right_anti_dual.e5 * self_2.e4), 
            /* e15, e25, e35, e1234 */
            float4(((float3(right_anti_dual.e5) * self_2.group0.xyz) - (float3(self_2.e5) * right_anti_dual.group1.xyz)), right_anti_dual.e321 * self_2.e4), 
            /* e4235, e4315, e4125, e3215 */
            float4((float3(self_2.e4) * right_anti_dual.group0.xyz), -(right_anti_dual.e235 * self_2.e1) - (right_anti_dual.e315 * self_2.e2) - (right_anti_dual.e125 * self_2.e3) - (right_anti_dual.e321 * self_2.e5))
        );
    }
}
// Fancy postfix trick
public Circle operator /(Line lhs, SupportPrefixOrPostfix rhs)  {
    return lhs.support();
    }
// Fancy prefix trick
public Circle operator /(SupportPrefixOrPostfix lhs, Line rhs) {
    return rhs.support();
}
public extension Line {
    public Circle support() {
        AntiLine right_anti_dual = AntiLine(
            /* e23, e31, e12 */
            this.group0.xyz, 
            /* e15, e25, e35 */
            this.group1.xyz
        );
        RoundPoint self_2 = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5 */
            0.0
        );
        return Circle(
            /* e423, e431, e412 */
            float3(self_2.e4) * right_anti_dual.group0.xyz, 
            /* e415, e425, e435, e321 */
            float4((float3(self_2.e4) * right_anti_dual.group1.xyz), -(right_anti_dual.e23 * self_2.e1) - (right_anti_dual.e31 * self_2.e2) - (right_anti_dual.e12 * self_2.e3)), 
            /* e235, e315, e125 */
            (float3(self_2.e5) * right_anti_dual.group0.xyz) + (right_anti_dual.group1.xyz.zxy * self_2.group0.yzx) - (right_anti_dual.group1.xyz.yzx * self_2.group0.zxy)
        );
    }
}
// Fancy postfix trick
public VersorEven operator /(Motor lhs, SupportPrefixOrPostfix rhs)  {
    return lhs.support();
    }
// Fancy prefix trick
public VersorEven operator /(SupportPrefixOrPostfix lhs, Motor rhs) {
    return rhs.support();
}
public extension Motor {
    public VersorEven support() {
        AntiMotor right_anti_dual = AntiMotor(
            /* e23, e31, e12, scalar */
            this.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            this.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        RoundPoint self_2 = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5 */
            0.0
        );
        return VersorEven(
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
// Fancy postfix trick
public MultiVector operator /(MultiVector lhs, SupportPrefixOrPostfix rhs)  {
    return lhs.support();
    }
// Fancy prefix trick
public MultiVector operator /(SupportPrefixOrPostfix lhs, MultiVector rhs) {
    return rhs.support();
}
public extension MultiVector {
    public MultiVector support() {
        MultiVector right_anti_dual = MultiVector(
            /* scalar, e12345 */
            this.group0.xy.yx * float2(-1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(this.group9.xyz, this.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            this.e3215, 
            /* e15, e25, e35, e45 */
            float4(this.group8.xyz, this.e321 * -1.0), 
            /* e41, e42, e43 */
            this.group7.xyz, 
            /* e23, e31, e12 */
            this.group6.xyz, 
            /* e415, e425, e435, e321 */
            float4(this.group5.xyz, this.e45) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e423, e431, e412 */
            this.group4.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            this.group3.xyz * float3(-1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.group1.xyz, this.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            this.e4 * -1.0
        );
        RoundPoint self_2 = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5 */
            0.0
        );
        return MultiVector(
            /* scalar, e12345 */
            float2(0.0, (right_anti_dual.e4235 * self_2.e1) + (right_anti_dual.e4315 * self_2.e2) + (right_anti_dual.e4125 * self_2.e3) + (right_anti_dual.e3215 * self_2.e4) + (right_anti_dual.e1234 * self_2.e5)), 
            /* e1, e2, e3, e4 */
            float4(right_anti_dual.scalar) * self_2.group0, 
            /* e5 */
            0.0, 
            /* e15, e25, e35, e45 */
            (float4(right_anti_dual.e5) * self_2.group0) - (float4(self_2.e5) * right_anti_dual.group1), 
            /* e41, e42, e43 */
            (float3(self_2.e4) * right_anti_dual.group1.xyz) - (float3(right_anti_dual.e4) * self_2.group0.xyz), 
            /* e23, e31, e12 */
            (right_anti_dual.group1.zxy * self_2.group0.yzx) - (right_anti_dual.group1.yzx * self_2.group0.zxy), 
            /* e415, e425, e435, e321 */
            float4((right_anti_dual.e15 * self_2.e4) + (right_anti_dual.e41 * self_2.e5), (right_anti_dual.e25 * self_2.e4) + (right_anti_dual.e42 * self_2.e5), (right_anti_dual.e35 * self_2.e4) + (right_anti_dual.e43 * self_2.e5), -(right_anti_dual.e31 * self_2.e2) - (right_anti_dual.e12 * self_2.e3)) - (self_2.group0.xyzx * float4(right_anti_dual.group3.www, right_anti_dual.e23)), 
            /* e423, e431, e412 */
            (float3(self_2.e4) * right_anti_dual.group5.xyz) + (right_anti_dual.group4.xyz.yzx * self_2.group0.zxy) - (right_anti_dual.group4.xyz.zxy * self_2.group0.yzx), 
            /* e235, e315, e125 */
            (float3(self_2.e5) * right_anti_dual.group5.xyz) + (right_anti_dual.group3.zxy * self_2.group0.yzx) - (right_anti_dual.group3.yzx * self_2.group0.zxy), 
            /* e4235, e4315, e4125, e3215 */
            float4((right_anti_dual.e425 * self_2.e3) + (right_anti_dual.e235 * self_2.e4), (right_anti_dual.e435 * self_2.e1) + (right_anti_dual.e315 * self_2.e4), (right_anti_dual.e415 * self_2.e2) + (right_anti_dual.e125 * self_2.e4), -(right_anti_dual.e321 * self_2.e5) - (right_anti_dual.e125 * self_2.e3)) - (self_2.group0.yzxy * float4(right_anti_dual.group6.zxy, right_anti_dual.e315)) - float4((float3(self_2.e5) * right_anti_dual.group7.xyz), right_anti_dual.e235 * self_2.e1), 
            /* e1234 */
            (right_anti_dual.e321 * self_2.e4) + (right_anti_dual.e423 * self_2.e1) + (right_anti_dual.e431 * self_2.e2) + (right_anti_dual.e412 * self_2.e3)
        );
    }
}
// Fancy postfix trick
public Dipole operator /(Plane lhs, SupportPrefixOrPostfix rhs)  {
    return lhs.support();
    }
// Fancy prefix trick
public Dipole operator /(SupportPrefixOrPostfix lhs, Plane rhs) {
    return rhs.support();
}
public extension Plane {
    public Dipole support() {
        AntiPlane right_anti_dual = AntiPlane(
            /* e1, e2, e3, e5 */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        RoundPoint self_2 = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5 */
            0.0
        );
        return Dipole(
            /* e41, e42, e43 */
            float3(self_2.e4) * right_anti_dual.group0.xyz, 
            /* e23, e31, e12, e45 */
            float4(((right_anti_dual.group0.zxy * self_2.group0.yzx) - (right_anti_dual.group0.yzx * self_2.group0.zxy)), right_anti_dual.e5 * self_2.e4), 
            /* e15, e25, e35 */
            (float3(right_anti_dual.e5) * self_2.group0.xyz) - (float3(self_2.e5) * right_anti_dual.group0.xyz)
        );
    }
}
// Fancy postfix trick
public AntiScalar operator /(RoundPoint lhs, SupportPrefixOrPostfix rhs)  {
    return lhs.support();
    }
// Fancy prefix trick
public AntiScalar operator /(SupportPrefixOrPostfix lhs, RoundPoint rhs) {
    return rhs.support();
}
public extension RoundPoint {
    public AntiScalar support() {
        Sphere right_anti_dual = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(this.group0.xyz, this.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            this.e4 * -1.0
        );
        RoundPoint self_2 = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5 */
            0.0
        );
        return AntiScalar(
            /* e12345 */
            (self_2.e1 * right_anti_dual.e4235) + (self_2.e2 * right_anti_dual.e4315) + (self_2.e3 * right_anti_dual.e4125) + (self_2.e4 * right_anti_dual.e3215) + (self_2.e5 * right_anti_dual.e1234)
        );
    }
}
// Fancy postfix trick
public Dipole operator /(Sphere lhs, SupportPrefixOrPostfix rhs)  {
    return lhs.support();
    }
// Fancy prefix trick
public Dipole operator /(SupportPrefixOrPostfix lhs, Sphere rhs) {
    return rhs.support();
}
public extension Sphere {
    public Dipole support() {
        RoundPoint right_anti_dual = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(this.group0.xyz, this.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            this.e3215
        );
        RoundPoint self_2 = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5 */
            0.0
        );
        return Dipole(
            /* e41, e42, e43 */
            (float3(self_2.e4) * right_anti_dual.group0.xyz) - (float3(right_anti_dual.e4) * self_2.group0.xyz), 
            /* e23, e31, e12, e45 */
            float4((right_anti_dual.group0.zxy * self_2.group0.yzx), right_anti_dual.e5 * self_2.e4) - (right_anti_dual.group0.yzxw * float4(self_2.group0.zxy, self_2.e5)), 
            /* e15, e25, e35 */
            (float3(right_anti_dual.e5) * self_2.group0.xyz) - (float3(self_2.e5) * right_anti_dual.group0.xyz)
        );
    }
}
// Fancy postfix trick
public VersorEven operator /(VersorEven lhs, SupportPrefixOrPostfix rhs)  {
    return lhs.support();
    }
// Fancy prefix trick
public VersorEven operator /(SupportPrefixOrPostfix lhs, VersorEven rhs) {
    return rhs.support();
}
public extension VersorEven {
    public VersorEven support() {
        VersorOdd right_anti_dual = VersorOdd(
            /* e41, e42, e43, scalar */
            this.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e23, e31, e12, e45 */
            this.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            float4(this.group2.xyz, this.e4) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.group3.xyz, this.e5) * float4(1.0, 1.0, 1.0, -1.0)
        );
        RoundPoint self_2 = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5 */
            0.0
        );
        return VersorEven(
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
// Fancy postfix trick
public DipoleInversion operator /(VersorOdd lhs, SupportPrefixOrPostfix rhs)  {
    return lhs.support();
    }
// Fancy prefix trick
public DipoleInversion operator /(SupportPrefixOrPostfix lhs, VersorOdd rhs) {
    return rhs.support();
}
public extension VersorOdd {
    public DipoleInversion support() {
        VersorEven right_anti_dual = VersorEven(
            /* e423, e431, e412, e12345 */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e415, e425, e435, e321 */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            float4(this.group2.xyz, this.e3215) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(this.group3.xyz, this.e1234) * float4(-1.0, -1.0, -1.0, 1.0)
        );
        RoundPoint self_2 = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5 */
            0.0
        );
        return DipoleInversion(
            /* e41, e42, e43 */
            (float3(self_2.e4) * right_anti_dual.group3.xyz) - (float3(right_anti_dual.e4) * self_2.group0.xyz), 
            /* e23, e31, e12, e45 */
            (self_2.group0.yzxw * float4(right_anti_dual.group3.zxy, right_anti_dual.e5)) - float4((self_2.group0.zxy * right_anti_dual.group3.yzx), self_2.e5 * right_anti_dual.e4), 
            /* e15, e25, e35, e1234 */
            float4(self_2.e5 * right_anti_dual.e1 * -1.0, self_2.e5 * right_anti_dual.e2 * -1.0, self_2.e5 * right_anti_dual.e3 * -1.0, (self_2.e2 * right_anti_dual.e431) + (self_2.e3 * right_anti_dual.e412) + (self_2.e4 * right_anti_dual.e321)) + (self_2.group0.xyzx * float4(right_anti_dual.group2.www, right_anti_dual.e423)), 
            /* e4235, e4315, e4125, e3215 */
            float4((self_2.e3 * right_anti_dual.e425) + (self_2.e4 * right_anti_dual.e235), (self_2.e1 * right_anti_dual.e435) + (self_2.e4 * right_anti_dual.e315), (self_2.e2 * right_anti_dual.e415) + (self_2.e4 * right_anti_dual.e125), -(self_2.e3 * right_anti_dual.e125) - (self_2.e5 * right_anti_dual.e321)) - (self_2.group0.yzxx * float4(right_anti_dual.group1.zxy, right_anti_dual.e235)) - float4((float3(self_2.e5) * right_anti_dual.group0.xyz), self_2.e2 * right_anti_dual.e315)
        );
    }
}
