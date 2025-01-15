implementing cga3d;
using traits;
// Fancy postfix trick
public DipoleInversion operator /(AntiCircleRotor lhs, AntiSupportPrefixOrPostfix rhs)  {
    return lhs.anti_support();
    }
// Fancy prefix trick
public DipoleInversion operator /(AntiSupportPrefixOrPostfix lhs, AntiCircleRotor rhs) {
    return rhs.anti_support();
}
public extension AntiCircleRotor {
    public DipoleInversion anti_support() {
        Plane right_complement = Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), 1.0)
        );
        CircleRotor right_dual = CircleRotor(
            /* e423, e431, e412 */
            this.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e12345 */
            this.group2 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return DipoleInversion(
            /* e41, e42, e43 */
            (right_dual.group0.xyz.zxy * right_complement.group0.yzx) - (right_dual.group0.xyz.yzx * right_complement.group0.zxy), 
            /* e23, e31, e12, e45 */
            float4(right_dual.e423 * right_complement.e3215, right_dual.e431 * right_complement.e3215, right_dual.e412 * right_complement.e3215, -(right_dual.e425 * right_complement.e4315) - (right_dual.e435 * right_complement.e4125)) - (right_dual.group1.wwwx * right_complement.group0.xyzx), 
            /* e15, e25, e35, e1234 */
            float4((float3(right_complement.e3215) * right_dual.group1.xyz), 0.0) + float4((right_dual.group2.yzx * right_complement.group0.zxy), 0.0) - float4((right_dual.group2.zxy * right_complement.group0.yzx), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.e12345) * right_complement.group0
        );
    }
}
// Fancy postfix trick
public AntiDipoleInversion operator /(AntiDipoleInversion lhs, AntiSupportPrefixOrPostfix rhs)  {
    return lhs.anti_support();
    }
// Fancy prefix trick
public AntiDipoleInversion operator /(AntiSupportPrefixOrPostfix lhs, AntiDipoleInversion rhs) {
    return rhs.anti_support();
}
public extension AntiDipoleInversion {
    public AntiDipoleInversion anti_support() {
        Plane right_complement = Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), 1.0)
        );
        DipoleInversion right_dual = DipoleInversion(
            /* e41, e42, e43 */
            this.group0.xyz, 
            /* e23, e31, e12, e45 */
            this.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            this.group2 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group3 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(right_dual.e1234) * right_complement.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            float4(((right_dual.group3.yzx * right_complement.group0.zxy) - (right_dual.group3.zxy * right_complement.group0.yzx)), right_dual.e1234 * right_complement.e3215 * -1.0), 
            /* e235, e315, e125, e4 */
            float4(right_dual.e4235 * right_complement.e3215 * -1.0, right_dual.e4315 * right_complement.e3215 * -1.0, right_dual.e4125 * right_complement.e3215 * -1.0, (right_dual.e42 * right_complement.e4315) + (right_dual.e43 * right_complement.e4125)) + (right_complement.group0.xyzx * float4(right_dual.group3.www, right_dual.e41)), 
            /* e1, e2, e3, e5 */
            float4(right_dual.e12 * right_complement.e4315, right_dual.e23 * right_complement.e4125, right_dual.e31 * right_complement.e4235, -(right_dual.e25 * right_complement.e4315) - (right_dual.e35 * right_complement.e4125)) - (float4(right_complement.e3215) * float4(right_dual.group0.xyz, right_dual.e45)) - (right_complement.group0.zxyx * float4(right_dual.group1.yzx, right_dual.e15))
        );
    }
}
// Fancy postfix trick
public Plane operator /(AntiDualNum lhs, AntiSupportPrefixOrPostfix rhs)  {
    return lhs.anti_support();
    }
// Fancy prefix trick
public Plane operator /(AntiSupportPrefixOrPostfix lhs, AntiDualNum rhs) {
    return rhs.anti_support();
}
public extension AntiDualNum {
    public Plane anti_support() {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * float4(float3(0.0), 1.0)
        );
    }
}
// Fancy postfix trick
public DualNum operator /(AntiFlatPoint lhs, AntiSupportPrefixOrPostfix rhs)  {
    return lhs.anti_support();
    }
// Fancy prefix trick
public DualNum operator /(AntiSupportPrefixOrPostfix lhs, AntiFlatPoint rhs) {
    return rhs.anti_support();
}
public extension AntiFlatPoint {
    public DualNum anti_support() {
        Plane right_complement = Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), 1.0)
        );
        FlatPoint right_dual = FlatPoint(
            /* e15, e25, e35, e45 */
            this.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return DualNum(
            /* e5, e12345 */
            float2(-(right_dual.e15 * right_complement.e4235) - (right_dual.e25 * right_complement.e4315) - (right_dual.e35 * right_complement.e4125) - (right_dual.e45 * right_complement.e3215), 0.0)
        );
    }
}
// Fancy postfix trick
public Motor operator /(AntiFlector lhs, AntiSupportPrefixOrPostfix rhs)  {
    return lhs.anti_support();
    }
// Fancy prefix trick
public Motor operator /(AntiSupportPrefixOrPostfix lhs, AntiFlector rhs) {
    return rhs.anti_support();
}
public extension AntiFlector {
    public Motor anti_support() {
        Plane right_complement = Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), 1.0)
        );
        Flector right_dual = Flector(
            /* e15, e25, e35, e45 */
            this.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            this.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Motor(
            /* e415, e425, e435, e12345 */
            float4(((right_dual.group1.yzx * right_complement.group0.zxy) - (right_dual.group1.zxy * right_complement.group0.yzx)), 0.0), 
            /* e235, e315, e125, e5 */
            float4(right_dual.e3215 * right_complement.e4235, right_dual.e3215 * right_complement.e4315, right_dual.e3215 * right_complement.e4125, -(right_dual.e25 * right_complement.e4315) - (right_dual.e35 * right_complement.e4125) - (right_dual.e45 * right_complement.e3215)) - (right_complement.group0.wwwx * float4(right_dual.group1.xyz, right_dual.e15))
        );
    }
}
// Fancy postfix trick
public FlatPoint operator /(AntiLine lhs, AntiSupportPrefixOrPostfix rhs)  {
    return lhs.anti_support();
    }
// Fancy prefix trick
public FlatPoint operator /(AntiSupportPrefixOrPostfix lhs, AntiLine rhs) {
    return rhs.anti_support();
}
public extension AntiLine {
    public FlatPoint anti_support() {
        Plane right_complement = Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), 1.0)
        );
        Line right_dual = Line(
            /* e415, e425, e435 */
            this.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            this.group1.xyz * float3(-1.0)
        );
        return FlatPoint(
            /* e15, e25, e35, e45 */
            float4((right_dual.e415 * right_complement.e3215) + (right_dual.e315 * right_complement.e4125), (right_dual.e425 * right_complement.e3215) + (right_dual.e125 * right_complement.e4235), (right_dual.e435 * right_complement.e3215) + (right_dual.e235 * right_complement.e4315), -(right_dual.e425 * right_complement.e4315) - (right_dual.e435 * right_complement.e4125)) - (right_complement.group0.yzxx * float4(right_dual.group1.xyz.zxy, right_dual.e415))
        );
    }
}
// Fancy postfix trick
public Flector operator /(AntiMotor lhs, AntiSupportPrefixOrPostfix rhs)  {
    return lhs.anti_support();
    }
// Fancy prefix trick
public Flector operator /(AntiSupportPrefixOrPostfix lhs, AntiMotor rhs) {
    return rhs.anti_support();
}
public extension AntiMotor {
    public Flector anti_support() {
        Plane right_complement = Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), 1.0)
        );
        Motor right_dual = Motor(
            /* e415, e425, e435, e12345 */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Flector(
            /* e15, e25, e35, e45 */
            float4((right_dual.e415 * right_complement.e3215) + (right_dual.e315 * right_complement.e4125), (right_dual.e425 * right_complement.e3215) + (right_dual.e125 * right_complement.e4235), (right_dual.e435 * right_complement.e3215) + (right_dual.e235 * right_complement.e4315), -(right_dual.e425 * right_complement.e4315) - (right_dual.e435 * right_complement.e4125)) - (right_complement.group0.yzxx * float4(right_dual.group1.zxy, right_dual.e415)), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.e12345) * right_complement.group0
        );
    }
}
// Fancy postfix trick
public Line operator /(AntiPlane lhs, AntiSupportPrefixOrPostfix rhs)  {
    return lhs.anti_support();
    }
// Fancy prefix trick
public Line operator /(AntiSupportPrefixOrPostfix lhs, AntiPlane rhs) {
    return rhs.anti_support();
}
public extension AntiPlane {
    public Line anti_support() {
        Plane right_complement = Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), 1.0)
        );
        Plane right_dual = Plane(
            /* e4235, e4315, e4125, e3215 */
            this.group0 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return Line(
            /* e415, e425, e435 */
            (right_complement.group0.zxy * right_dual.group0.yzx) - (right_complement.group0.yzx * right_dual.group0.zxy), 
            /* e235, e315, e125 */
            (float3(right_dual.e3215) * right_complement.group0.xyz) - (float3(right_complement.e3215) * right_dual.group0.xyz)
        );
    }
}
// Fancy postfix trick
public RoundPoint operator /(Circle lhs, AntiSupportPrefixOrPostfix rhs)  {
    return lhs.anti_support();
    }
// Fancy prefix trick
public RoundPoint operator /(AntiSupportPrefixOrPostfix lhs, Circle rhs) {
    return rhs.anti_support();
}
public extension Circle {
    public RoundPoint anti_support() {
        Plane right_complement = Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), 1.0)
        );
        Dipole right_dual = Dipole(
            /* e41, e42, e43 */
            this.group0.xyz, 
            /* e23, e31, e12, e45 */
            this.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35 */
            this.group2.xyz
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(-(right_dual.e41 * right_complement.e3215) - (right_dual.e31 * right_complement.e4125), -(right_dual.e42 * right_complement.e3215) - (right_dual.e12 * right_complement.e4235), -(right_dual.e43 * right_complement.e3215) - (right_dual.e23 * right_complement.e4315), (right_dual.e42 * right_complement.e4315) + (right_dual.e43 * right_complement.e4125)) + (right_complement.group0.yzxx * float4(right_dual.group1.zxy, right_dual.e41)), 
            /* e5 */
            -(right_dual.e45 * right_complement.e3215) - (right_dual.e15 * right_complement.e4235) - (right_dual.e25 * right_complement.e4315) - (right_dual.e35 * right_complement.e4125)
        );
    }
}
// Fancy postfix trick
public RoundPoint operator /(CircleRotor lhs, AntiSupportPrefixOrPostfix rhs)  {
    return lhs.anti_support();
    }
// Fancy prefix trick
public RoundPoint operator /(AntiSupportPrefixOrPostfix lhs, CircleRotor rhs) {
    return rhs.anti_support();
}
public extension CircleRotor {
    public RoundPoint anti_support() {
        Plane right_complement = Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), 1.0)
        );
        AntiCircleRotor right_dual = AntiCircleRotor(
            /* e41, e42, e43 */
            this.group0.xyz, 
            /* e23, e31, e12, e45 */
            this.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, scalar */
            this.group2 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return RoundPoint(
            /* e1, e2, e3, e4 */
            float4(-(right_dual.e41 * right_complement.e3215) - (right_dual.e31 * right_complement.e4125), -(right_dual.e42 * right_complement.e3215) - (right_dual.e12 * right_complement.e4235), -(right_dual.e43 * right_complement.e3215) - (right_dual.e23 * right_complement.e4315), (right_dual.e42 * right_complement.e4315) + (right_dual.e43 * right_complement.e4125)) + (right_complement.group0.yzxx * float4(right_dual.group1.zxy, right_dual.e41)), 
            /* e5 */
            -(right_dual.e45 * right_complement.e3215) - (right_dual.e15 * right_complement.e4235) - (right_dual.e25 * right_complement.e4315) - (right_dual.e35 * right_complement.e4125)
        );
    }
}
// Fancy postfix trick
public Dipole operator /(Dipole lhs, AntiSupportPrefixOrPostfix rhs)  {
    return lhs.anti_support();
    }
// Fancy prefix trick
public Dipole operator /(AntiSupportPrefixOrPostfix lhs, Dipole rhs) {
    return rhs.anti_support();
}
public extension Dipole {
    public Dipole anti_support() {
        Plane right_complement = Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), 1.0)
        );
        Circle right_dual = Circle(
            /* e423, e431, e412 */
            this.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125 */
            this.group2.xyz * float3(-1.0)
        );
        return Dipole(
            /* e41, e42, e43 */
            (right_dual.group0.xyz.zxy * right_complement.group0.yzx) - (right_dual.group0.xyz.yzx * right_complement.group0.zxy), 
            /* e23, e31, e12, e45 */
            float4(right_dual.e423 * right_complement.e3215, right_dual.e431 * right_complement.e3215, right_dual.e412 * right_complement.e3215, -(right_dual.e425 * right_complement.e4315) - (right_dual.e435 * right_complement.e4125)) - (right_dual.group1.wwwx * right_complement.group0.xyzx), 
            /* e15, e25, e35 */
            (float3(right_complement.e3215) * right_dual.group1.xyz) + (right_dual.group2.xyz.yzx * right_complement.group0.zxy) - (right_dual.group2.xyz.zxy * right_complement.group0.yzx)
        );
    }
}
// Fancy postfix trick
public AntiCircleRotor operator /(DipoleInversion lhs, AntiSupportPrefixOrPostfix rhs)  {
    return lhs.anti_support();
    }
// Fancy prefix trick
public AntiCircleRotor operator /(AntiSupportPrefixOrPostfix lhs, DipoleInversion rhs) {
    return rhs.anti_support();
}
public extension DipoleInversion {
    public AntiCircleRotor anti_support() {
        Plane right_complement = Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), 1.0)
        );
        AntiDipoleInversion right_dual = AntiDipoleInversion(
            /* e423, e431, e412 */
            this.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e4 */
            this.group2 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            this.group3 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiCircleRotor(
            /* e41, e42, e43 */
            (right_dual.group0.xyz.zxy * right_complement.group0.yzx) - (right_dual.group0.xyz.yzx * right_complement.group0.zxy), 
            /* e23, e31, e12, e45 */
            float4(right_dual.e423 * right_complement.e3215, right_dual.e431 * right_complement.e3215, right_dual.e412 * right_complement.e3215, -(right_dual.e425 * right_complement.e4315) - (right_dual.e435 * right_complement.e4125)) - (right_dual.group1.wwwx * right_complement.group0.xyzx), 
            /* e15, e25, e35, scalar */
            float4(right_dual.e125 * right_complement.e4315 * -1.0, right_dual.e235 * right_complement.e4125 * -1.0, right_dual.e315 * right_complement.e4235 * -1.0, (right_dual.e2 * right_complement.e4315) + (right_dual.e3 * right_complement.e4125)) + (float4(right_complement.e3215) * float4(right_dual.group1.xyz, right_dual.e4)) + (right_complement.group0.zxyx * float4(right_dual.group2.yzx, right_dual.e1))
        );
    }
}
// Fancy postfix trick
public AntiFlatPoint operator /(DualNum lhs, AntiSupportPrefixOrPostfix rhs)  {
    return lhs.anti_support();
    }
// Fancy prefix trick
public AntiFlatPoint operator /(AntiSupportPrefixOrPostfix lhs, DualNum rhs) {
    return rhs.anti_support();
}
public extension DualNum {
    public AntiFlatPoint anti_support() {
        AntiDualNum right_dual = AntiDualNum(
            /* e3215, scalar */
            this.group0.xy * float2(-1.0)
        );
        return AntiFlatPoint(
            /* e235, e315, e125, e321 */
            float4(right_dual.group0.xy.xx, right_dual.e3215, 0.0) * float4(float3(0.0), 0.0) * float4(float3(1.0), 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
// Fancy postfix trick
public AntiLine operator /(FlatPoint lhs, AntiSupportPrefixOrPostfix rhs)  {
    return lhs.anti_support();
    }
// Fancy prefix trick
public AntiLine operator /(AntiSupportPrefixOrPostfix lhs, FlatPoint rhs) {
    return rhs.anti_support();
}
public extension FlatPoint {
    public AntiLine anti_support() {
        Plane right_complement = Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), 1.0)
        );
        AntiFlatPoint right_dual = AntiFlatPoint(
            /* e235, e315, e125, e321 */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiLine(
            /* e23, e31, e12 */
            float3(right_dual.e321) * right_complement.group0.xyz * float3(-1.0), 
            /* e15, e25, e35 */
            (right_dual.group0.yzx * right_complement.group0.zxy) - (right_dual.group0.zxy * right_complement.group0.yzx)
        );
    }
}
// Fancy postfix trick
public AntiMotor operator /(Flector lhs, AntiSupportPrefixOrPostfix rhs)  {
    return lhs.anti_support();
    }
// Fancy prefix trick
public AntiMotor operator /(AntiSupportPrefixOrPostfix lhs, Flector rhs) {
    return rhs.anti_support();
}
public extension Flector {
    public AntiMotor anti_support() {
        Plane right_complement = Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), 1.0)
        );
        AntiFlector right_dual = AntiFlector(
            /* e235, e315, e125, e321 */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e5 */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return AntiMotor(
            /* e23, e31, e12, scalar */
            float4((right_dual.group0.www * right_complement.group0.xyz * float3(-1.0)), (right_dual.e1 * right_complement.e4235) + (right_dual.e2 * right_complement.e4315) + (right_dual.e3 * right_complement.e4125)), 
            /* e15, e25, e35, e3215 */
            float4(((right_dual.group0.yzx * right_complement.group0.zxy) - (right_dual.group0.zxy * right_complement.group0.yzx)), 0.0)
        );
    }
}
// Fancy postfix trick
public AntiPlane operator /(Line lhs, AntiSupportPrefixOrPostfix rhs)  {
    return lhs.anti_support();
    }
// Fancy prefix trick
public AntiPlane operator /(AntiSupportPrefixOrPostfix lhs, Line rhs) {
    return rhs.anti_support();
}
public extension Line {
    public AntiPlane anti_support() {
        Plane right_complement = Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), 1.0)
        );
        AntiLine right_dual = AntiLine(
            /* e23, e31, e12 */
            this.group0.xyz, 
            /* e15, e25, e35 */
            this.group1.xyz
        );
        return AntiPlane(
            /* e1, e2, e3, e5 */
            float4(right_dual.e12 * right_complement.e4315, right_dual.e23 * right_complement.e4125, right_dual.e31 * right_complement.e4235, -(right_dual.e25 * right_complement.e4315) - (right_dual.e35 * right_complement.e4125)) - (right_complement.group0.zxyx * float4(right_dual.group0.xyz.yzx, right_dual.e15))
        );
    }
}
// Fancy postfix trick
public AntiFlector operator /(Motor lhs, AntiSupportPrefixOrPostfix rhs)  {
    return lhs.anti_support();
    }
// Fancy prefix trick
public AntiFlector operator /(AntiSupportPrefixOrPostfix lhs, Motor rhs) {
    return rhs.anti_support();
}
public extension Motor {
    public AntiFlector anti_support() {
        Plane right_complement = Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), 1.0)
        );
        AntiMotor right_dual = AntiMotor(
            /* e23, e31, e12, scalar */
            this.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e3215 */
            this.group1 * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiFlector(
            /* e235, e315, e125, e321 */
            float4(float3(1.0), 0.0) * float4(right_dual.group1.www, 0.0) * float4(right_complement.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e1, e2, e3, e5 */
            float4(right_dual.e12 * right_complement.e4315, right_dual.e23 * right_complement.e4125, right_dual.e31 * right_complement.e4235, -(right_dual.e25 * right_complement.e4315) - (right_dual.e35 * right_complement.e4125)) - (right_complement.group0.zxyx * float4(right_dual.group0.yzx, right_dual.e15))
        );
    }
}
// Fancy postfix trick
public MultiVector operator /(MultiVector lhs, AntiSupportPrefixOrPostfix rhs)  {
    return lhs.anti_support();
    }
// Fancy prefix trick
public MultiVector operator /(AntiSupportPrefixOrPostfix lhs, MultiVector rhs) {
    return rhs.anti_support();
}
public extension MultiVector {
    public MultiVector anti_support() {
        Plane right_complement = Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), 1.0)
        );
        MultiVector right_dual = MultiVector(
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
        return MultiVector(
            /* scalar, e12345 */
            float2((right_dual.e1 * right_complement.e4235) + (right_dual.e2 * right_complement.e4315) + (right_dual.e3 * right_complement.e4125) + (right_dual.e4 * right_complement.e3215), 0.0), 
            /* e1, e2, e3, e4 */
            float4(-(right_dual.e41 * right_complement.e3215) - (right_dual.e31 * right_complement.e4125), -(right_dual.e42 * right_complement.e3215) - (right_dual.e12 * right_complement.e4235), -(right_dual.e43 * right_complement.e3215) - (right_dual.e23 * right_complement.e4315), (right_dual.e42 * right_complement.e4315) + (right_dual.e43 * right_complement.e4125)) + (right_complement.group0.yzxx * float4(right_dual.group5.xyz.zxy, right_dual.e41)), 
            /* e5 */
            -(right_dual.e15 * right_complement.e4235) - (right_dual.e25 * right_complement.e4315) - (right_dual.e35 * right_complement.e4125) - (right_dual.e45 * right_complement.e3215), 
            /* e15, e25, e35, e45 */
            float4((right_dual.e415 * right_complement.e3215) + (right_dual.e315 * right_complement.e4125), (right_dual.e425 * right_complement.e3215) + (right_dual.e125 * right_complement.e4235), (right_dual.e435 * right_complement.e3215) + (right_dual.e235 * right_complement.e4315), -(right_dual.e425 * right_complement.e4315) - (right_dual.e435 * right_complement.e4125)) - (right_complement.group0.yzxx * float4(right_dual.group8.xyz.zxy, right_dual.e415)), 
            /* e41, e42, e43 */
            (right_dual.group7.xyz.zxy * right_complement.group0.yzx) - (right_dual.group7.xyz.yzx * right_complement.group0.zxy), 
            /* e23, e31, e12 */
            (float3(right_complement.e3215) * right_dual.group7.xyz) - (float3(right_dual.e321) * right_complement.group0.xyz), 
            /* e415, e425, e435, e321 */
            float4(((right_dual.group9.yzx * right_complement.group0.zxy) - (right_dual.group9.zxy * right_complement.group0.yzx)), right_dual.e1234 * right_complement.e3215 * -1.0), 
            /* e423, e431, e412 */
            float3(right_dual.e1234) * right_complement.group0.xyz * float3(-1.0), 
            /* e235, e315, e125 */
            (float3(right_dual.e3215) * right_complement.group0.xyz) - (float3(right_complement.e3215) * right_dual.group9.xyz), 
            /* e4235, e4315, e4125, e3215 */
            float4(right_dual.e12345) * right_complement.group0, 
            /* e1234 */
            0.0
        );
    }
}
// Fancy postfix trick
public Scalar operator /(Plane lhs, AntiSupportPrefixOrPostfix rhs)  {
    return lhs.anti_support();
    }
// Fancy prefix trick
public Scalar operator /(AntiSupportPrefixOrPostfix lhs, Plane rhs) {
    return rhs.anti_support();
}
public extension Plane {
    public Scalar anti_support() {
        Plane right_complement = Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), 1.0)
        );
        AntiPlane right_dual = AntiPlane(
            /* e1, e2, e3, e5 */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return Scalar(
            /* scalar */
            (right_dual.e1 * right_complement.e4235) + (right_dual.e2 * right_complement.e4315) + (right_dual.e3 * right_complement.e4125)
        );
    }
}
// Fancy postfix trick
public Circle operator /(RoundPoint lhs, AntiSupportPrefixOrPostfix rhs)  {
    return lhs.anti_support();
    }
// Fancy prefix trick
public Circle operator /(AntiSupportPrefixOrPostfix lhs, RoundPoint rhs) {
    return rhs.anti_support();
}
public extension RoundPoint {
    public Circle anti_support() {
        Plane right_complement = Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), 1.0)
        );
        Sphere right_dual = Sphere(
            /* e4235, e4315, e4125, e3215 */
            float4(this.group0.xyz, this.e5) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e1234 */
            this.e4 * -1.0
        );
        return Circle(
            /* e423, e431, e412 */
            float3(right_dual.e1234) * right_complement.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            float4(((right_complement.group0.zxy * right_dual.group0.yzx) - (right_complement.group0.yzx * right_dual.group0.zxy)), right_complement.e3215 * right_dual.e1234 * -1.0), 
            /* e235, e315, e125 */
            (float3(right_dual.e3215) * right_complement.group0.xyz) - (float3(right_complement.e3215) * right_dual.group0.xyz)
        );
    }
}
// Fancy postfix trick
public Plane operator /(Scalar lhs, AntiSupportPrefixOrPostfix rhs)  {
    return lhs.anti_support();
    }
// Fancy prefix trick
public Plane operator /(AntiSupportPrefixOrPostfix lhs, Scalar rhs) {
    return rhs.anti_support();
}
public extension Scalar {
    public Plane anti_support() {
        return Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(this.scalar) * float4(float3(0.0), 1.0)
        );
    }
}
// Fancy postfix trick
public Scalar operator /(Sphere lhs, AntiSupportPrefixOrPostfix rhs)  {
    return lhs.anti_support();
    }
// Fancy prefix trick
public Scalar operator /(AntiSupportPrefixOrPostfix lhs, Sphere rhs) {
    return rhs.anti_support();
}
public extension Sphere {
    public Scalar anti_support() {
        Plane right_complement = Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), 1.0)
        );
        RoundPoint right_dual = RoundPoint(
            /* e1, e2, e3, e4 */
            float4(this.group0.xyz, this.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e5 */
            this.e3215
        );
        return Scalar(
            /* scalar */
            (right_complement.e4235 * right_dual.e1) + (right_complement.e4315 * right_dual.e2) + (right_complement.e4125 * right_dual.e3) + (right_complement.e3215 * right_dual.e4)
        );
    }
}
// Fancy postfix trick
public AntiDipoleInversion operator /(VersorEven lhs, AntiSupportPrefixOrPostfix rhs)  {
    return lhs.anti_support();
    }
// Fancy prefix trick
public AntiDipoleInversion operator /(AntiSupportPrefixOrPostfix lhs, VersorEven rhs) {
    return rhs.anti_support();
}
public extension VersorEven {
    public AntiDipoleInversion anti_support() {
        Plane right_complement = Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), 1.0)
        );
        VersorOdd right_dual = VersorOdd(
            /* e41, e42, e43, scalar */
            this.group0 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e23, e31, e12, e45 */
            this.group1 * float4(1.0, 1.0, 1.0, -1.0), 
            /* e15, e25, e35, e1234 */
            float4(this.group2.xyz, this.e4) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(this.group3.xyz, this.e5) * float4(1.0, 1.0, 1.0, -1.0)
        );
        return AntiDipoleInversion(
            /* e423, e431, e412 */
            float3(right_dual.e1234) * right_complement.group0.xyz * float3(-1.0), 
            /* e415, e425, e435, e321 */
            float4(((right_complement.group0.zxy * right_dual.group3.yzx) - (right_complement.group0.yzx * right_dual.group3.zxy)), right_complement.e3215 * right_dual.e1234 * -1.0), 
            /* e235, e315, e125, e4 */
            float4(right_complement.e3215 * right_dual.e4235 * -1.0, right_complement.e3215 * right_dual.e4315 * -1.0, right_complement.e3215 * right_dual.e4125 * -1.0, (right_complement.e4315 * right_dual.e42) + (right_complement.e4125 * right_dual.e43)) + (right_complement.group0.xyzx * float4(right_dual.group3.www, right_dual.e41)), 
            /* e1, e2, e3, e5 */
            float4(right_complement.e4315 * right_dual.e12, right_complement.e4125 * right_dual.e23, right_complement.e4235 * right_dual.e31, -(right_complement.e4125 * right_dual.e35) - (right_complement.e3215 * right_dual.e45)) - (right_complement.group0.zxyx * float4(right_dual.group1.yzx, right_dual.e15)) - (right_complement.group0.wwwy * float4(right_dual.group0.xyz, right_dual.e25))
        );
    }
}
// Fancy postfix trick
public VersorOdd operator /(VersorOdd lhs, AntiSupportPrefixOrPostfix rhs)  {
    return lhs.anti_support();
    }
// Fancy prefix trick
public VersorOdd operator /(AntiSupportPrefixOrPostfix lhs, VersorOdd rhs) {
    return rhs.anti_support();
}
public extension VersorOdd {
    public VersorOdd anti_support() {
        Plane right_complement = Plane(
            /* e4235, e4315, e4125, e3215 */
            float4(float3(0.0), 1.0)
        );
        VersorEven right_dual = VersorEven(
            /* e423, e431, e412, e12345 */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e415, e425, e435, e321 */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e235, e315, e125, e5 */
            float4(this.group2.xyz, this.e3215) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(this.group3.xyz, this.e1234) * float4(-1.0, -1.0, -1.0, 1.0)
        );
        return VersorOdd(
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
