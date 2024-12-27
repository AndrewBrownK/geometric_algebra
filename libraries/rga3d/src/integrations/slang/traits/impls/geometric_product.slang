extension AntiScalar {
    func div(_rhs: geometric_product) -> geometric_product_partial<AntiScalar> {
        geometric_product_partial(this)
    }
}
extension AntiScalar: GeometricProduct<DualNum> {
    func geometric_product(other: DualNum) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            this.e1234 * other.scalar
        );
    }
}
extension AntiScalar: GeometricProduct<Flector> {
    func geometric_product(other: Flector) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e1234 * other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(this.e1234, this.e1234, this.e1234, 0.0) * float4(other.group0.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0)
        );
    }
}
extension AntiScalar: GeometricProduct<Horizon> {
    func geometric_product(other: Horizon) -> Origin {
        return Origin.from_groups(
            /* e4 */
            this.e1234 * other.e321 * -1.0
        );
    }
}
extension AntiScalar: GeometricProduct<Line> {
    func geometric_product(other: Line) -> Line {
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(this.e1234) * other.group1, 
            /* e23, e31, e12 */
            float3(0.0)
        );
    }
}
extension AntiScalar: GeometricProduct<Motor> {
    func geometric_product(other: Motor) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.e1234) * other.group1, 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
extension AntiScalar: GeometricProduct<MultiVector> {
    func geometric_product(other: MultiVector) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(1.0, this.e1234 * other.scalar) * float2(0.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e1234 * other.e321 * -1.0), 
            /* e41, e42, e43 */
            float3(this.e1234) * other.group3, 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(this.e1234, this.e1234, this.e1234, 0.0) * float4(other.group1.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0)
        );
    }
}
extension AntiScalar: GeometricProduct<Plane> {
    func geometric_product(other: Plane) -> Origin {
        return Origin.from_groups(
            /* e4 */
            this.e1234 * other.e321 * -1.0
        );
    }
}
extension AntiScalar: GeometricProduct<Point> {
    func geometric_product(other: Point) -> Plane {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(this.e1234, this.e1234, this.e1234, 0.0) * float4(other.group0.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0)
        );
    }
}
extension AntiScalar: GeometricProduct<Scalar> {
    func geometric_product(other: Scalar) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            this.e1234 * other.scalar
        );
    }
}
extension DualNum {
    func div(_rhs: geometric_product) -> geometric_product_partial<DualNum> {
        geometric_product_partial(this)
    }
}
extension DualNum: GeometricProduct<AntiScalar> {
    func geometric_product(other: AntiScalar) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            other.e1234 * this.scalar
        );
    }
}
extension DualNum: GeometricProduct<DualNum> {
    func geometric_product(other: DualNum) -> DualNum {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(other.scalar * this.scalar, (other.scalar * this.e1234) + (other.e1234 * this.scalar))
        );
    }
}
extension DualNum: GeometricProduct<Flector> {
    func geometric_product(other: Flector) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.e1, other.e2, other.e3, 1.0) * float4(this.group0.xx, this.scalar, (this.scalar * other.e4) - (this.e1234 * other.e321)), 
            /* e423, e431, e412, e321 */
            float4(((float3(this.scalar) * other.group1.xyz) - (float3(this.e1234) * other.group0.xyz)), this.scalar * other.e321)
        );
    }
}
extension DualNum: GeometricProduct<Horizon> {
    func geometric_product(other: Horizon) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e1234 * other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), this.scalar * other.e321)
        );
    }
}
extension DualNum: GeometricProduct<Line> {
    func geometric_product(other: Line) -> Line {
        return Line.from_groups(
            /* e41, e42, e43 */
            (float3(this.scalar) * other.group0) + (float3(this.e1234) * other.group1), 
            /* e23, e31, e12 */
            float3(this.scalar) * other.group1
        );
    }
}
extension DualNum: GeometricProduct<Motor> {
    func geometric_product(other: Motor) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            (float4(this.scalar) * other.group0) + (float4(this.e1234) * other.group1), 
            /* e23, e31, e12, scalar */
            float4(this.scalar) * other.group1
        );
    }
}
extension DualNum: GeometricProduct<MultiVector> {
    func geometric_product(other: MultiVector) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(this.scalar * other.scalar, (this.scalar * other.e1234) + (this.e1234 * other.scalar)), 
            /* e1, e2, e3, e4 */
            float4(other.e1, other.e2, other.e3, 1.0) * float4(this.group0.xx, this.scalar, (this.scalar * other.e4) - (this.e1234 * other.e321)), 
            /* e41, e42, e43 */
            (float3(this.scalar) * other.group2) + (float3(this.e1234) * other.group3), 
            /* e23, e31, e12 */
            float3(this.scalar) * other.group3, 
            /* e423, e431, e412, e321 */
            float4(((float3(this.scalar) * other.group4.xyz) - (float3(this.e1234) * other.group1.xyz)), this.scalar * other.e321)
        );
    }
}
extension DualNum: GeometricProduct<Origin> {
    func geometric_product(other: Origin) -> Origin {
        return Origin.from_groups(
            /* e4 */
            this.scalar * other.e4
        );
    }
}
extension DualNum: GeometricProduct<Plane> {
    func geometric_product(other: Plane) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e1234 * other.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(this.scalar) * other.group0
        );
    }
}
extension DualNum: GeometricProduct<Point> {
    func geometric_product(other: Point) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group0, 
            /* e423, e431, e412, e321 */
            float4(this.group0.yy, this.e1234, 0.0) * float4(float3(1.0), 0.0) * float4(other.group0.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0)
        );
    }
}
extension DualNum: GeometricProduct<Scalar> {
    func geometric_product(other: Scalar) -> DualNum {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(other.scalar) * this.group0
        );
    }
}
extension Flector {
    func div(_rhs: geometric_product) -> geometric_product_partial<Flector> {
        geometric_product_partial(this)
    }
}
extension Flector: GeometricProduct<AntiScalar> {
    func geometric_product(other: AntiScalar) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e1234 * this.e321), 
            /* e423, e431, e412, e321 */
            float4(other.e1234, other.e1234, other.e1234, 0.0) * float4(this.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Flector: GeometricProduct<DualNum> {
    func geometric_product(other: DualNum) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e1, this.e2, this.e3, 1.0) * float4(other.group0.xx, other.scalar, (other.scalar * this.e4) + (other.e1234 * this.e321)), 
            /* e423, e431, e412, e321 */
            float4(((float3(other.scalar) * this.group1.xyz) + (float3(other.e1234) * this.group0.xyz)), other.scalar * this.e321)
        );
    }
}
extension Flector: GeometricProduct<Flector> {
    func geometric_product(other: Flector) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            (float4(other.e321) * float4(this.group1.xyz, this.e4)) + (other.group1.zxyz * this.group0.yzxz) + (float4(this.group0.ww, this.e431, this.e1) * float4(other.group0.xyx, other.e423)) + (float4(this.group1.zx, this.e4, this.e2) * float4(other.group0.yzz, other.e431)) - (other.group0.zxyx * this.group1.yzxx) - (other.group0.wwwy * float4(this.group0.xyz, this.e431)) - (float4(this.group0.zx, this.e321, this.e321) * float4(other.group1.yzz, other.e4)) - (float4(this.group1.ww, this.e2, this.e412) * float4(other.group1.xyx, other.e3)), 
            /* e23, e31, e12, scalar */
            float4(-(other.e2 * this.e3) - (other.e321 * this.e1), -(other.e3 * this.e1) - (other.e321 * this.e2), -(other.e3 * this.e321) - (other.e321 * this.e3), (other.e2 * this.e2) + (other.e3 * this.e3)) + (other.group0.zxyx * this.group0.yzxx) - (float4(this.group1.ww, this.e2, this.e321) * float4(other.group0.xyx, other.e321))
        );
    }
}
extension Flector: GeometricProduct<Horizon> {
    func geometric_product(other: Horizon) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.e321) * float4(this.group1.xyz, this.e4), 
            /* e23, e31, e12, scalar */
            float4(other.e321) * float4(this.group0.xyz, this.e321) * float4(-1.0)
        );
    }
}
extension Flector: GeometricProduct<Line> {
    func geometric_product(other: Line) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4((this.e3 * other.e31) + (this.e321 * other.e23), (this.e1 * other.e12) + (this.e321 * other.e31), (this.e2 * other.e23) + (this.e321 * other.e12), -(this.e2 * other.e42) - (this.e3 * other.e43) - (this.e423 * other.e23) - (this.e431 * other.e31) - (this.e412 * other.e12)) - (this.group0.yzxx * float4(other.group1.zxy, other.e41)), 
            /* e423, e431, e412, e321 */
            float4((this.e3 * other.e42) + (this.e4 * other.e23) + (this.e412 * other.e31) + (this.e321 * other.e41), (this.e1 * other.e43) + (this.e4 * other.e31) + (this.e423 * other.e12) + (this.e321 * other.e42), (this.e2 * other.e41) + (this.e4 * other.e12) + (this.e431 * other.e23) + (this.e321 * other.e43), this.e3 * other.e12 * -1.0) - (this.group0.yzxx * float4(other.group0.zxy, other.e23)) - float4((other.group1.zxy * this.group1.yzx), this.e2 * other.e31)
        );
    }
}
extension Flector: GeometricProduct<Motor> {
    func geometric_product(other: Motor) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e321 * other.e23, this.e321 * other.e31, this.e321 * other.e12, -(this.e2 * other.e42) - (this.e3 * other.e43) - (this.e423 * other.e23) - (this.e431 * other.e31) - (this.e412 * other.e12)) + (this.group0.xxyw * other.group1.wzxw) + float4((this.group0.zyz * other.group1.yww), this.e321 * other.e1234) - (this.group0.yzxx * float4(other.group1.zxy, other.e41)), 
            /* e423, e431, e412, e321 */
            float4((this.e3 * other.e42) + (this.e4 * other.e23) + (this.e423 * other.scalar) + (this.e412 * other.e31) + (this.e321 * other.e41), (this.e2 * other.e1234) + (this.e4 * other.e31) + (this.e423 * other.e12) + (this.e431 * other.scalar) + (this.e321 * other.e42), (this.e3 * other.e1234) + (this.e4 * other.e12) + (this.e431 * other.e23) + (this.e412 * other.scalar) + (this.e321 * other.e43), this.e3 * other.e12 * -1.0) + float4((this.group0.xxy * other.group0.wzx), this.e321 * other.scalar) - (this.group0.yzxx * float4(other.group0.zxy, other.e23)) - (other.group1.zxyy * float4(this.group1.yzx, this.e2))
        );
    }
}
extension Flector: GeometricProduct<MultiVector> {
    func geometric_product(other: MultiVector) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, (this.e4 * other.e321) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4)) + (float2(this.e1) * float2(other.e1, other.e423)) + (float2(this.e2) * float2(other.e2, other.e431)) + (float2(this.e3) * float2(other.e3, other.e412)) - (float2(other.e321, other.e1) * this.group1.wx), 
            /* e1, e2, e3, e4 */
            float4(this.e3 * other.e31, this.e1 * other.e12, this.e321 * other.e12, -(this.e2 * other.e42) - (this.e3 * other.e43) - (this.e423 * other.e23) - (this.e431 * other.e31) - (this.e412 * other.e12)) + (float4(other.scalar) * this.group0) + (float4(this.group1.ww, this.e2, this.e321) * float4(other.group3.xyx, other.e1234)) - (this.group0.yzxx * float4(other.group3.zxy, other.e41)), 
            /* e41, e42, e43 */
            (float3(this.e4) * other.group1.xyz) + (float3(other.e2, other.e321, other.e321) * this.group1.zyz) + (float3(other.e321, other.e3, other.e1) * this.group1.xxy) + (this.group0.yzx * other.group4.zxy) - (float3(this.e321) * other.group4.xyz) - (float3(other.e4, other.e412, other.e423) * this.group0.xxy) - (float3(other.e431, other.e4, other.e4) * this.group0.zyz) - (this.group1.yzx * other.group1.zxy), 
            /* e23, e31, e12 */
            (this.group0.yzx * other.group1.zxy) - (float3(this.e321) * other.group1.xyz) - (float3(other.e2, other.e321, other.e321) * this.group0.zyz) - (float3(other.e321, other.e3, other.e1) * this.group0.xxy), 
            /* e423, e431, e412, e321 */
            float4((this.e1 * other.e1234) + (this.e3 * other.e42) + (this.e4 * other.e23) + (this.e412 * other.e31) + (this.e321 * other.e41), (this.e1 * other.e43) + (this.e2 * other.e1234) + (this.e4 * other.e31) + (this.e423 * other.e12) + (this.e321 * other.e42), (this.e2 * other.e41) + (this.e3 * other.e1234) + (this.e4 * other.e12) + (this.e431 * other.e23) + (this.e321 * other.e43), this.e3 * other.e12 * -1.0) + (float4(other.scalar) * this.group1) - (this.group0.yzxx * float4(other.group2.zxy, other.e23)) - float4((other.group3.zxy * this.group1.yzx), this.e2 * other.e31)
        );
    }
}
extension Flector: GeometricProduct<Origin> {
    func geometric_product(other: Origin) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.e4) * float4(this.group0.xyz, this.e321) * float4(-1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
extension Flector: GeometricProduct<Plane> {
    func geometric_product(other: Plane) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(-(this.e3 * other.e431) - (this.e321 * other.e423), -(this.e1 * other.e412) - (this.e321 * other.e431), -(this.e2 * other.e423) - (this.e321 * other.e412), (this.e3 * other.e412) + (this.e4 * other.e321)) + (this.group0.yzxx * other.group0.zxyx) + (other.group0.wwwy * float4(this.group1.xyz, this.e2)), 
            /* e23, e31, e12, scalar */
            float4(other.e321) * float4(this.group0.xyz, this.e321) * float4(-1.0)
        );
    }
}
extension Flector: GeometricProduct<Point> {
    func geometric_product(other: Point) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4((this.e4 * other.e1) + (this.e412 * other.e2), (this.e4 * other.e2) + (this.e423 * other.e3), (this.e4 * other.e3) + (this.e431 * other.e1), -(this.e412 * other.e3) - (this.e321 * other.e4)) - (this.group1.yzxy * other.group0.zxyy) - (other.group0.wwwx * float4(this.group0.xyz, this.e423)), 
            /* e23, e31, e12, scalar */
            float4(-(this.e3 * other.e2) - (this.e321 * other.e1), -(this.e1 * other.e3) - (this.e321 * other.e2), -(this.e2 * other.e1) - (this.e321 * other.e3), (this.e2 * other.e2) + (this.e3 * other.e3)) + (this.group0.yzxx * other.group0.zxyx)
        );
    }
}
extension Flector: GeometricProduct<Scalar> {
    func geometric_product(other: Scalar) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group0, 
            /* e423, e431, e412, e321 */
            float4(other.scalar) * this.group1
        );
    }
}
extension Horizon {
    func div(_rhs: geometric_product) -> geometric_product_partial<Horizon> {
        geometric_product_partial(this)
    }
}
extension Horizon: GeometricProduct<AntiScalar> {
    func geometric_product(other: AntiScalar) -> Origin {
        return Origin.from_groups(
            /* e4 */
            other.e1234 * this.e321
        );
    }
}
extension Horizon: GeometricProduct<DualNum> {
    func geometric_product(other: DualNum) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e1234 * this.e321), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other.scalar * this.e321)
        );
    }
}
extension Horizon: GeometricProduct<Flector> {
    func geometric_product(other: Flector) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.e321) * float4(other.group1.xyz, other.e4) * float4(-1.0), 
            /* e23, e31, e12, scalar */
            float4(this.e321) * float4(other.group0.xyz, other.e321) * float4(-1.0)
        );
    }
}
extension Horizon: GeometricProduct<Horizon> {
    func geometric_product(other: Horizon) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            other.e321 * this.e321 * -1.0
        );
    }
}
extension Horizon: GeometricProduct<Line> {
    func geometric_product(other: Line) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e321, this.e321, this.e321, 0.0) * float4(other.group1, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e423, e431, e412, e321 */
            float4(this.e321, this.e321, this.e321, 0.0) * float4(other.group0, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Horizon: GeometricProduct<Motor> {
    func geometric_product(other: Motor) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e321) * float4(other.group1.xyz, other.e1234), 
            /* e423, e431, e412, e321 */
            float4(this.e321) * float4(other.group0.xyz, other.scalar)
        );
    }
}
extension Horizon: GeometricProduct<MultiVector> {
    func geometric_product(other: MultiVector) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(this.e321) * float2(other.e321, other.e4) * float2(-1.0), 
            /* e1, e2, e3, e4 */
            float4(this.e321) * float4(other.group3, other.e1234), 
            /* e41, e42, e43 */
            float3(this.e321) * other.group4.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(this.e321) * other.group1.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            float4(this.e321) * float4(other.group2, other.scalar)
        );
    }
}
extension Horizon: GeometricProduct<Origin> {
    func geometric_product(other: Origin) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            this.e321 * other.e4 * -1.0
        );
    }
}
extension Horizon: GeometricProduct<Plane> {
    func geometric_product(other: Plane) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.e321, this.e321, this.e321, 0.0) * float4(other.group0.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0), 
            /* e23, e31, e12, scalar */
            float4(float3(0.0), this.e321 * other.e321 * -1.0)
        );
    }
}
extension Horizon: GeometricProduct<Point> {
    func geometric_product(other: Point) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(float3(0.0), this.e321 * other.e4 * -1.0), 
            /* e23, e31, e12, scalar */
            float4(this.e321, this.e321, this.e321, 0.0) * float4(other.group0.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0)
        );
    }
}
extension Horizon: GeometricProduct<Scalar> {
    func geometric_product(other: Scalar) -> Horizon {
        return Horizon.from_groups(
            /* e321 */
            this.e321 * other.scalar
        );
    }
}
extension Line {
    func div(_rhs: geometric_product) -> geometric_product_partial<Line> {
        geometric_product_partial(this)
    }
}
extension Line: GeometricProduct<AntiScalar> {
    func geometric_product(other: AntiScalar) -> Line {
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(other.e1234) * this.group1, 
            /* e23, e31, e12 */
            float3(0.0)
        );
    }
}
extension Line: GeometricProduct<DualNum> {
    func geometric_product(other: DualNum) -> Line {
        return Line.from_groups(
            /* e41, e42, e43 */
            (float3(other.scalar) * this.group0) + (float3(other.e1234) * this.group1), 
            /* e23, e31, e12 */
            float3(other.scalar) * this.group1
        );
    }
}
extension Line: GeometricProduct<Flector> {
    func geometric_product(other: Flector) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            (float4(other.e2, other.e321, other.e321, other.e2) * float4(this.group1.zyz, this.e42)) + (float4(other.e321, other.e3, other.e1, other.e1) * float4(this.group1.xxy, this.e41)) + float4(float3(0.0), (other.e3 * this.e43) - (other.e431 * this.e31) - (other.e412 * this.e12)) - float4((this.group1.yzx * other.group0.zxy), other.e423 * this.e23), 
            /* e423, e431, e412, e321 */
            float4((other.e3 * this.e42) + (other.e4 * this.e23) + (other.e431 * this.e12), (other.e1 * this.e43) + (other.e4 * this.e31) + (other.e412 * this.e23), (other.e2 * this.e41) + (other.e4 * this.e12) + (other.e423 * this.e31), 0.0) - (float4(other.e2, other.e321, other.e321, other.e2) * float4(this.group0.zyz, this.e31)) - (float4(other.e321, other.e3, other.e1, other.e1) * float4(this.group0.xxy, this.e23)) - float4((this.group1.yzx * other.group1.zxy), other.e3 * this.e12)
        );
    }
}
extension Line: GeometricProduct<Horizon> {
    func geometric_product(other: Horizon) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.e321, other.e321, other.e321, 0.0) * float4(this.group1, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e423, e431, e412, e321 */
            float4(other.e321, other.e321, other.e321, 0.0) * float4(this.group0, 0.0) * float4(-1.0, -1.0, -1.0, 0.0)
        );
    }
}
extension Line: GeometricProduct<Line> {
    func geometric_product(other: Line) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4((other.e42 * this.e12) + (other.e31 * this.e43), (other.e43 * this.e23) + (other.e12 * this.e41), (other.e41 * this.e31) + (other.e23 * this.e42), -(other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)) - float4((other.group0.zxy * this.group1.yzx), other.e41 * this.e23) - float4((other.group1.zxy * this.group0.yzx), other.e42 * this.e31), 
            /* e23, e31, e12, scalar */
            float4(other.e31 * this.e12, other.e12 * this.e23, other.e23 * this.e31, -(other.e31 * this.e31) - (other.e12 * this.e12)) - float4((other.group1.zxy * this.group1.yzx), other.e23 * this.e23)
        );
    }
}
extension Line: GeometricProduct<Motor> {
    func geometric_product(other: Motor) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4((this.e41 * other.scalar) + (this.e43 * other.e31) + (this.e23 * other.e1234) + (this.e12 * other.e42), (this.e41 * other.e12) + (this.e42 * other.scalar) + (this.e23 * other.e43) + (this.e31 * other.e1234), (this.e42 * other.e23) + (this.e43 * other.scalar) + (this.e31 * other.e41) + (this.e12 * other.e1234), -(this.e43 * other.e12) - (this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43)) - (other.group1.zxyx * float4(this.group0.yzx, this.e41)) - float4((this.group1.yzx * other.group0.zxy), this.e42 * other.e31), 
            /* e23, e31, e12, scalar */
            float4((this.e23 * other.scalar) + (this.e12 * other.e31), (this.e23 * other.e12) + (this.e31 * other.scalar), (this.e31 * other.e23) + (this.e12 * other.scalar), -(this.e31 * other.e31) - (this.e12 * other.e12)) - (other.group1.zxyx * float4(this.group1.yzx, this.e23))
        );
    }
}
extension Line: GeometricProduct<MultiVector> {
    func geometric_product(other: MultiVector) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, -(this.e23 * other.e41) - (this.e31 * other.e42) - (this.e12 * other.e43)) - (float2(other.e23) * float2(this.e23, this.e41)) - (float2(other.e31) * float2(this.e31, this.e42)) - (float2(other.e12) * float2(this.e12, this.e43)), 
            /* e1, e2, e3, e4 */
            (float4(other.e2, other.e321, other.e321, other.e2) * float4(this.group1.zyz, this.e42)) + (float4(other.e321, other.e3, other.e1, other.e1) * float4(this.group1.xxy, this.e41)) + float4(float3(0.0), (this.e43 * other.e3) - (this.e31 * other.e431) - (this.e12 * other.e412)) - float4((this.group1.yzx * other.group1.zxy), this.e23 * other.e423), 
            /* e41, e42, e43 */
            (float3(other.scalar) * this.group0) + (float3(other.e1234) * this.group1) + (this.group0.zxy * other.group3.yzx) + (this.group1.zxy * other.group2.yzx) - (this.group0.yzx * other.group3.zxy) - (this.group1.yzx * other.group2.zxy), 
            /* e23, e31, e12 */
            (float3(other.scalar) * this.group1) + (this.group1.zxy * other.group3.yzx) - (this.group1.yzx * other.group3.zxy), 
            /* e423, e431, e412, e321 */
            float4((this.e42 * other.e3) + (this.e23 * other.e4) + (this.e12 * other.e431), (this.e43 * other.e1) + (this.e23 * other.e412) + (this.e31 * other.e4), (this.e41 * other.e2) + (this.e31 * other.e423) + (this.e12 * other.e4), 0.0) - (float4(other.e2, other.e321, other.e321, other.e2) * float4(this.group0.zyz, this.e31)) - (float4(other.e321, other.e3, other.e1, other.e1) * float4(this.group0.xxy, this.e23)) - float4((this.group1.yzx * other.group4.zxy), this.e12 * other.e3)
        );
    }
}
extension Line: GeometricProduct<Origin> {
    func geometric_product(other: Origin) -> Plane {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.e4, other.e4, other.e4, 0.0) * float4(this.group1, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Line: GeometricProduct<Plane> {
    func geometric_product(other: Plane) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.e321, other.e321, other.e321, 1.0) * float4(this.group1, -(this.e23 * other.e423) - (this.e31 * other.e431) - (this.e12 * other.e412)), 
            /* e423, e431, e412, e321 */
            float4((this.group1.zxy * other.group0.yzx), 0.0) - float4((float3(other.e321) * this.group0), 0.0) - float4((this.group1.yzx * other.group0.zxy), 0.0)
        );
    }
}
extension Line: GeometricProduct<Point> {
    func geometric_product(other: Point) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e31 * other.e3 * -1.0, this.e12 * other.e1 * -1.0, this.e23 * other.e2 * -1.0, (this.e42 * other.e2) + (this.e43 * other.e3)) + (other.group0.yzxx * float4(this.group1.zxy, this.e41)), 
            /* e423, e431, e412, e321 */
            float4((this.e42 * other.e3) + (this.e23 * other.e4), (this.e43 * other.e1) + (this.e31 * other.e4), (this.e41 * other.e2) + (this.e12 * other.e4), -(this.e31 * other.e2) - (this.e12 * other.e3)) - (other.group0.yzxx * float4(this.group0.zxy, this.e23))
        );
    }
}
extension Line: GeometricProduct<Scalar> {
    func geometric_product(other: Scalar) -> Line {
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(other.scalar) * this.group0, 
            /* e23, e31, e12 */
            float3(other.scalar) * this.group1
        );
    }
}
extension Motor {
    func div(_rhs: geometric_product) -> geometric_product_partial<Motor> {
        geometric_product_partial(this)
    }
}
extension Motor: GeometricProduct<AntiScalar> {
    func geometric_product(other: AntiScalar) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.e1234) * this.group1, 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
extension Motor: GeometricProduct<DualNum> {
    func geometric_product(other: DualNum) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            (float4(other.scalar) * this.group0) + (float4(other.e1234) * this.group1), 
            /* e23, e31, e12, scalar */
            float4(other.scalar) * this.group1
        );
    }
}
extension Motor: GeometricProduct<Flector> {
    func geometric_product(other: Flector) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            (other.group0.xyxx * float4(this.group1.wwy, this.e41)) + (other.group0.yzzy * float4(this.group1.zxw, this.e42)) + float4(float3(0.0), (other.e4 * this.scalar) - (other.e431 * this.e31) - (other.e412 * this.e12) - (other.e321 * this.e1234)) + float4((other.group1.www * this.group1.xyz), other.e3 * this.e43) - (this.group1.yzxx * float4(other.group0.zxy, other.e423)), 
            /* e423, e431, e412, e321 */
            float4((other.e4 * this.e23) + (other.e423 * this.scalar) + (other.e431 * this.e12) - (other.e321 * this.e41), (other.e4 * this.e31) + (other.e431 * this.scalar) + (other.e412 * this.e23) - (other.e321 * this.e42), (other.e4 * this.e12) + (other.e423 * this.e31) + (other.e412 * this.scalar) - (other.e321 * this.e43), 0.0) + float4((other.group0.zxy * this.group0.yzx), other.e321 * this.scalar) - (other.group0.xyxx * float4(this.group0.wwy, this.e23)) - (other.group0.yzzy * float4(this.group0.zxw, this.e31)) - (this.group1.yzxz * float4(other.group1.zxy, other.e3))
        );
    }
}
extension Motor: GeometricProduct<Horizon> {
    func geometric_product(other: Horizon) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.e321) * float4(this.group1.xyz, this.e1234) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e423, e431, e412, e321 */
            float4(other.e321) * float4(this.group0.xyz, this.scalar) * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
extension Motor: GeometricProduct<Line> {
    func geometric_product(other: Line) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4((other.e41 * this.scalar) + (other.e42 * this.e12) + (other.e23 * this.e1234) + (other.e31 * this.e43), (other.e42 * this.scalar) + (other.e43 * this.e23) + (other.e31 * this.e1234) + (other.e12 * this.e41), (other.e41 * this.e31) + (other.e43 * this.scalar) + (other.e23 * this.e42) + (other.e12 * this.e1234), -(other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)) - (this.group1.yzxx * float4(other.group0.zxy, other.e41)) - float4((other.group1.zxy * this.group0.yzx), other.e42 * this.e31), 
            /* e23, e31, e12, scalar */
            float4((other.e23 * this.scalar) + (other.e31 * this.e12), (other.e31 * this.scalar) + (other.e12 * this.e23), (other.e23 * this.e31) + (other.e12 * this.scalar), -(other.e31 * this.e31) - (other.e12 * this.e12)) - (this.group1.yzxx * float4(other.group1.zxy, other.e23))
        );
    }
}
extension Motor: GeometricProduct<Motor> {
    func geometric_product(other: Motor) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4((other.e1234 * this.e23) + (other.e23 * this.e1234) + (other.e31 * this.e43) + (other.scalar * this.e41), (other.e1234 * this.e31) + (other.e31 * this.e1234) + (other.e12 * this.e41) + (other.scalar * this.e42), (other.e1234 * this.e12) + (other.e23 * this.e42) + (other.e12 * this.e1234) + (other.scalar * this.e43), -(other.e43 * this.e12) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)) + (other.group0.xyxw * this.group1.wwyw) + float4((other.group0.yzz * this.group1.zxw), other.scalar * this.e1234) - (other.group0.zxyx * this.group1.yzxx) - float4((other.group1.zxy * this.group0.yzx), other.e42 * this.e31), 
            /* e23, e31, e12, scalar */
            float4((other.e31 * this.e12) + (other.scalar * this.e23), (other.e12 * this.e23) + (other.scalar * this.e31), (other.e12 * this.scalar) + (other.scalar * this.e12), -(other.e31 * this.e31) - (other.e12 * this.e12)) + (other.group1.xyxw * this.group1.wwyw) - (other.group1.zxyx * this.group1.yzxx)
        );
    }
}
extension Motor: GeometricProduct<MultiVector> {
    func geometric_product(other: MultiVector) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, (this.scalar * other.e1234) - (this.e41 * other.e23) - (this.e42 * other.e31) - (this.e43 * other.e12)) + (float2(other.scalar) * float2(this.scalar, this.e1234)) - (float2(this.e23) * float2(other.e23, other.e41)) - (float2(this.e31) * float2(other.e31, other.e42)) - (float2(this.e12) * float2(other.e12, other.e43)), 
            /* e1, e2, e3, e4 */
            (float4(other.e2, other.e321, other.e321, other.e2) * float4(this.group1.zyz, this.e42)) + (float4(other.e321, other.e3, other.e1, other.e1) * float4(this.group1.xxy, this.e41)) + (other.group1.xyzz * float4(this.group1.www, this.e43)) + float4(float3(0.0), (this.scalar * other.e4) - (this.e23 * other.e423) - (this.e31 * other.e431) - (this.e12 * other.e412)) - float4((this.group1.yzx * other.group1.zxy), this.e1234 * other.e321), 
            /* e41, e42, e43 */
            (float3(other.scalar) * this.group0.xyz) + (float3(other.e1234) * this.group1.xyz) + (other.group2.xyx * this.group1.wwy) + (other.group2.yzz * this.group1.zxw) + (other.group3.xyx * this.group0.wwy) + (other.group3.yzz * this.group0.zxw) - (other.group2.zxy * this.group1.yzx) - (other.group3.zxy * this.group0.yzx), 
            /* e23, e31, e12 */
            (float3(other.scalar) * this.group1.xyz) + (other.group3.xyx * this.group1.wwy) + (other.group3.yzz * this.group1.zxw) - (other.group3.zxy * this.group1.yzx), 
            /* e423, e431, e412, e321 */
            float4((this.e23 * other.e4) + (this.e12 * other.e431) + (this.scalar * other.e423) - (this.e31 * other.e412), (this.e23 * other.e412) + (this.e31 * other.e4) + (this.scalar * other.e431) - (this.e12 * other.e423), (this.e31 * other.e423) + (this.e12 * other.e4) + (this.scalar * other.e412) - (this.e23 * other.e431), 0.0) + float4((this.group0.yzx * other.group1.zxy), this.scalar * other.e321) - (float4(other.e2, other.e321, other.e321, other.e2) * float4(this.group0.zyz, this.e31)) - (float4(other.e321, other.e3, other.e1, other.e1) * float4(this.group0.xxy, this.e23)) - (other.group1.xyzz * float4(this.group0.www, this.e12))
        );
    }
}
extension Motor: GeometricProduct<Origin> {
    func geometric_product(other: Origin) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.scalar * other.e4), 
            /* e423, e431, e412, e321 */
            float4(other.e4, other.e4, other.e4, 0.0) * float4(this.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Motor: GeometricProduct<Plane> {
    func geometric_product(other: Plane) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.e321, other.e321, other.e321, 1.0) * float4(this.group1.xyz, -(this.e1234 * other.e321) - (this.e23 * other.e423) - (this.e31 * other.e431) - (this.e12 * other.e412)), 
            /* e423, e431, e412, e321 */
            float4(((float3(this.scalar) * other.group0.xyz) + (this.group1.zxy * other.group0.yzx) - (float3(other.e321) * this.group0.xyz) - (this.group1.yzx * other.group0.zxy)), this.scalar * other.e321)
        );
    }
}
extension Motor: GeometricProduct<Point> {
    func geometric_product(other: Point) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e31 * other.e3 * -1.0, this.e12 * other.e1 * -1.0, this.e23 * other.e2 * -1.0, (this.e43 * other.e3) + (this.scalar * other.e4)) + (other.group0.xyzy * float4(this.group1.www, this.e42)) + (other.group0.yzxx * float4(this.group1.zxy, this.e41)), 
            /* e423, e431, e412, e321 */
            float4((this.e42 * other.e3) + (this.e23 * other.e4), (this.e43 * other.e1) + (this.e31 * other.e4), (this.e41 * other.e2) + (this.e12 * other.e4), this.e12 * other.e3 * -1.0) - (other.group0.xyzy * float4(this.group0.www, this.e31)) - (other.group0.yzxx * float4(this.group0.zxy, this.e23))
        );
    }
}
extension Motor: GeometricProduct<Scalar> {
    func geometric_product(other: Scalar) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.scalar) * this.group0, 
            /* e23, e31, e12, scalar */
            float4(other.scalar) * this.group1
        );
    }
}
extension MultiVector {
    func div(_rhs: geometric_product) -> geometric_product_partial<MultiVector> {
        geometric_product_partial(this)
    }
}
extension MultiVector: GeometricProduct<AntiScalar> {
    func geometric_product(other: AntiScalar) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(1.0, other.e1234 * this.scalar) * float2(0.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e1234 * this.e321), 
            /* e41, e42, e43 */
            float3(other.e1234) * this.group3, 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(other.e1234, other.e1234, other.e1234, 0.0) * float4(this.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension MultiVector: GeometricProduct<DualNum> {
    func geometric_product(other: DualNum) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other.scalar * this.scalar, (other.scalar * this.e1234) + (other.e1234 * this.scalar)), 
            /* e1, e2, e3, e4 */
            float4(this.e1, this.e2, this.e3, 1.0) * float4(other.group0.xx, other.scalar, (other.scalar * this.e4) + (other.e1234 * this.e321)), 
            /* e41, e42, e43 */
            (float3(other.scalar) * this.group2) + (float3(other.e1234) * this.group3), 
            /* e23, e31, e12 */
            float3(other.scalar) * this.group3, 
            /* e423, e431, e412, e321 */
            float4(((float3(other.scalar) * this.group4.xyz) + (float3(other.e1234) * this.group1.xyz)), other.scalar * this.e321)
        );
    }
}
extension MultiVector: GeometricProduct<Flector> {
    func geometric_product(other: Flector) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, (other.e321 * this.e4) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321)) + (float2(this.e1) * float2(other.e1, other.e423)) + (float2(this.e2) * float2(other.e2, other.e431)) + (float2(this.e3) * float2(other.e3, other.e412)) - (float2(other.e321, other.e1) * this.group4.wx), 
            /* e1, e2, e3, e4 */
            (float4(this.scalar) * other.group0) + (float4(other.e2, other.e321, other.e321, other.e2) * float4(this.group3.zyz, this.e42)) + (float4(other.e321, other.e3, other.e1, other.e1) * float4(this.group3.xxy, this.e41)) + float4(float3(0.0), (other.e3 * this.e43) - (other.e423 * this.e23) - (other.e431 * this.e31) - (other.e412 * this.e12)) - float4((this.group3.yzx * other.group0.zxy), other.e321 * this.e1234), 
            /* e41, e42, e43 */
            (float3(other.e321) * this.group4.xyz) + (float3(this.e4, this.e4, this.e431) * other.group0.xyx) + (float3(this.e412, this.e423, this.e4) * other.group0.yzz) + (other.group1.zxy * this.group1.yzx) - (float3(other.e4) * this.group1.xyz) - (float3(this.e3, this.e1, this.e321) * other.group1.yzz) - (float3(this.e321, this.e321, this.e2) * other.group1.xyx) - (other.group0.zxy * this.group4.yzx), 
            /* e23, e31, e12 */
            (other.group0.zxy * this.group1.yzx) - (float3(other.e321) * this.group1.xyz) - (float3(this.e3, this.e1, this.e321) * other.group0.yzz) - (float3(this.e321, this.e321, this.e2) * other.group0.xyx), 
            /* e423, e431, e412, e321 */
            float4((other.e3 * this.e42) + (other.e4 * this.e23) + (other.e431 * this.e12) - (other.e412 * this.e31), (other.e1 * this.e43) + (other.e4 * this.e31) + (other.e412 * this.e23) - (other.e423 * this.e12), (other.e2 * this.e41) + (other.e4 * this.e12) + (other.e423 * this.e31) - (other.e431 * this.e23), 0.0) + (float4(this.scalar) * other.group1) - (float4(other.e2, other.e321, other.e321, other.e3) * float4(this.group2.zyz, this.e12)) - (float4(other.e321, other.e3, other.e1, other.e2) * float4(this.group2.xxy, this.e31)) - (other.group0.xyzx * float4(this.group0.yy, this.e1234, this.e23))
        );
    }
}
extension MultiVector: GeometricProduct<Horizon> {
    func geometric_product(other: Horizon) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other.e321) * float2(this.e321, this.e4) * float2(-1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(other.e321) * float4(this.group3, this.e1234) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e41, e42, e43 */
            float3(other.e321) * this.group4.xyz, 
            /* e23, e31, e12 */
            float3(other.e321) * this.group1.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            float4(other.e321) * float4(this.group2, this.scalar) * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
extension MultiVector: GeometricProduct<Line> {
    func geometric_product(other: Line) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, -(other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)) - (float2(this.e23) * float2(other.e23, other.e41)) - (float2(this.e31) * float2(other.e31, other.e42)) - (float2(this.e12) * float2(other.e12, other.e43)), 
            /* e1, e2, e3, e4 */
            float4((other.e23 * this.e321) + (other.e31 * this.e3), (other.e31 * this.e321) + (other.e12 * this.e1), (other.e23 * this.e2) + (other.e12 * this.e321), -(other.e42 * this.e2) - (other.e43 * this.e3) - (other.e23 * this.e423) - (other.e31 * this.e431) - (other.e12 * this.e412)) - (this.group1.yzxx * float4(other.group1.zxy, other.e41)), 
            /* e41, e42, e43 */
            (float3(this.scalar) * other.group0) + (float3(this.e1234) * other.group1) + (other.group0.yzx * this.group3.zxy) + (other.group1.yzx * this.group2.zxy) - (other.group0.zxy * this.group3.yzx) - (other.group1.zxy * this.group2.yzx), 
            /* e23, e31, e12 */
            (float3(this.scalar) * other.group1) + (other.group1.yzx * this.group3.zxy) - (other.group1.zxy * this.group3.yzx), 
            /* e423, e431, e412, e321 */
            float4((other.e41 * this.e321) + (other.e42 * this.e3) + (other.e23 * this.e4) + (other.e31 * this.e412), (other.e42 * this.e321) + (other.e43 * this.e1) + (other.e31 * this.e4) + (other.e12 * this.e423), (other.e41 * this.e2) + (other.e43 * this.e321) + (other.e23 * this.e431) + (other.e12 * this.e4), other.e12 * this.e3 * -1.0) - (this.group1.yzxx * float4(other.group0.zxy, other.e23)) - float4((other.group1.zxy * this.group4.yzx), other.e31 * this.e2)
        );
    }
}
extension MultiVector: GeometricProduct<Motor> {
    func geometric_product(other: Motor) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, (other.scalar * this.e1234) - (other.e41 * this.e23) - (other.e42 * this.e31) - (other.e43 * this.e12)) + (float2(this.scalar) * float2(other.scalar, other.e1234)) - (float2(other.e23) * float2(this.e23, this.e41)) - (float2(other.e31) * float2(this.e31, this.e42)) - (float2(other.e12) * float2(this.e12, this.e43)), 
            /* e1, e2, e3, e4 */
            float4(other.scalar * this.e1, other.scalar * this.e2, other.scalar * this.e3, -(other.e42 * this.e2) - (other.e43 * this.e3) - (other.e23 * this.e423) - (other.e31 * this.e431) - (other.e12 * this.e412)) + (other.group1.yzzw * float4(this.group1.zx, this.e321, this.e4)) + (float4(this.group4.ww, this.e2, this.e321) * float4(other.group1.xyx, other.e1234)) - (this.group1.yzxx * float4(other.group1.zxy, other.e41)), 
            /* e41, e42, e43 */
            (float3(this.scalar) * other.group0.xyz) + (float3(this.e1234) * other.group1.xyz) + (this.group2.xxy * other.group1.wzx) + (this.group2.zyz * other.group1.yww) + (this.group3.xxy * other.group0.wzx) + (this.group3.zyz * other.group0.yww) - (this.group2.yzx * other.group1.zxy) - (this.group3.yzx * other.group0.zxy), 
            /* e23, e31, e12 */
            (float3(this.scalar) * other.group1.xyz) + (this.group3.xxy * other.group1.wzx) + (this.group3.zyz * other.group1.yww) - (this.group3.yzx * other.group1.zxy), 
            /* e423, e431, e412, e321 */
            float4((other.e42 * this.e3) + (other.e1234 * this.e1) + (other.e23 * this.e4) + (other.e31 * this.e412) + (other.scalar * this.e423), (other.e43 * this.e1) + (other.e1234 * this.e2) + (other.e31 * this.e4) + (other.e12 * this.e423) + (other.scalar * this.e431), (other.e43 * this.e321) + (other.e1234 * this.e3) + (other.e23 * this.e431) + (other.e12 * this.e4) + (other.scalar * this.e412), other.e12 * this.e3 * -1.0) + (float4(this.group4.ww, this.e2, this.e321) * float4(other.group0.xyx, other.scalar)) - (other.group1.zxyy * float4(this.group4.yzx, this.e2)) - (this.group1.yzxx * float4(other.group0.zxy, other.e23))
        );
    }
}
extension MultiVector: GeometricProduct<MultiVector> {
    func geometric_product(other: MultiVector) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, (other.e1234 * this.scalar) + (other.e321 * this.e4) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321) - (other.e23 * this.e41) - (other.e31 * this.e42) - (other.e12 * this.e43)) + (float2(other.scalar) * this.group0) + (float2(this.e1) * float2(other.e1, other.e423)) + (float2(this.e2) * float2(other.e2, other.e431)) + (float2(this.e3) * float2(other.e3, other.e412)) - (float2(this.e23) * float2(other.e23, other.e41)) - (float2(this.e31) * float2(other.e31, other.e42)) - (float2(this.e12) * float2(other.e12, other.e43)) - (float2(other.e321, other.e1) * this.group4.wx), 
            /* e1, e2, e3, e4 */
            (float4(other.scalar) * this.group1) + (float4(other.e2, other.e321, other.e321, other.e3) * float4(this.group3.zyz, this.e43)) + (float4(other.e321, other.e3, other.e1, other.e2) * float4(this.group3.xxy, this.e42)) + (float4(this.group0.xx, this.scalar, other.e1234) * float4(other.group1.xyz, this.e321)) + (float4(this.group1.zx, this.e321, other.e1) * float4(other.group3.yzz, this.e41)) + (float4(this.group4.ww, this.e2, other.e4) * float4(other.group3.xyx, this.scalar)) + float4(float3(0.0), -(other.e42 * this.e2) - (other.e43 * this.e3) - (other.e23 * this.e423) - (other.e31 * this.e431) - (other.e12 * this.e412) - (other.e423 * this.e23) - (other.e431 * this.e31) - (other.e412 * this.e12)) - float4((other.group3.zxy * this.group1.yzx), other.e321 * this.e1234) - float4((this.group3.yzx * other.group1.zxy), other.e41 * this.e1), 
            /* e41, e42, e43 */
            (float3(other.scalar) * this.group2) + (float3(other.e1234) * this.group3) + (float3(other.e321) * this.group4.xyz) + (float3(this.scalar) * other.group2) + (float3(this.e1234) * other.group3) + (float3(this.e4, this.e4, this.e431) * other.group1.xyx) + (float3(this.e412, this.e423, this.e4) * other.group1.yzz) + (other.group2.yzx * this.group3.zxy) + (other.group3.yzx * this.group2.zxy) + (other.group4.zxy * this.group1.yzx) - (float3(other.e4) * this.group1.xyz) - (float3(this.e3, this.e1, this.e321) * other.group4.yzz) - (float3(this.e321, this.e321, this.e2) * other.group4.xyx) - (other.group2.zxy * this.group3.yzx) - (other.group3.zxy * this.group2.yzx) - (other.group1.zxy * this.group4.yzx), 
            /* e23, e31, e12 */
            (float3(other.scalar) * this.group3) + (float3(this.scalar) * other.group3) + (other.group3.yzx * this.group3.zxy) + (other.group1.zxy * this.group1.yzx) - (float3(other.e321) * this.group1.xyz) - (float3(this.e3, this.e1, this.e321) * other.group1.yzz) - (float3(this.e321, this.e321, this.e2) * other.group1.xyx) - (other.group3.zxy * this.group3.yzx), 
            /* e423, e431, e412, e321 */
            float4((other.e3 * this.e42) + (other.e4 * this.e23) + (other.e41 * this.e321) + (other.e42 * this.e3) + (other.e23 * this.e4) + (other.e31 * this.e412) + (other.e423 * this.scalar) + (other.e431 * this.e12), (other.e1 * this.e43) + (other.e4 * this.e31) + (other.e42 * this.e321) + (other.e43 * this.e1) + (other.e31 * this.e4) + (other.e12 * this.e423) + (other.e431 * this.scalar) + (other.e412 * this.e23), (other.e2 * this.e41) + (other.e4 * this.e12) + (other.e41 * this.e2) + (other.e43 * this.e321) + (other.e23 * this.e431) + (other.e12 * this.e4) + (other.e423 * this.e31) + (other.e412 * this.scalar), 0.0) + (float4(other.scalar) * this.group4) + (float4(other.group0.yy, other.e1234, this.scalar) * float4(this.group1.xyz, other.e321)) - (float4(other.e2, other.e321, other.e321, other.e2) * float4(this.group2.zyz, this.e31)) - (float4(other.e321, other.e3, other.e1, other.e1) * float4(this.group2.xxy, this.e23)) - (this.group1.yzxy * float4(other.group2.zxy, other.e31)) - (float4(this.group0.yy, this.e1234, other.e23) * float4(other.group1.xyz, this.e1)) - float4((other.group3.zxy * this.group4.yzx), other.e12 * this.e3) - float4((this.group3.yzx * other.group4.zxy), other.e3 * this.e12)
        );
    }
}
extension MultiVector: GeometricProduct<Origin> {
    func geometric_product(other: Origin) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(1.0, this.e321 * other.e4) * float2(0.0, -1.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.scalar * other.e4), 
            /* e41, e42, e43 */
            float3(other.e4) * this.group1.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(other.e4, other.e4, other.e4, 0.0) * float4(this.group3, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension MultiVector: GeometricProduct<Plane> {
    func geometric_product(other: Plane) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(this.e321 * other.e321, (this.e1 * other.e423) + (this.e2 * other.e431) + (this.e3 * other.e412) + (this.e4 * other.e321)) * float2(-1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(other.e321, other.e321, other.e321, 1.0) * float4(this.group3, -(this.e1234 * other.e321) - (this.e23 * other.e423) - (this.e31 * other.e431) - (this.e12 * other.e412)), 
            /* e41, e42, e43 */
            (float3(other.e321) * this.group4.xyz) + (this.group1.yzx * other.group0.zxy) - (float3(this.e321) * other.group0.xyz) - (this.group1.zxy * other.group0.yzx), 
            /* e23, e31, e12 */
            float3(other.e321) * this.group1.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            float4(((float3(this.scalar) * other.group0.xyz) + (this.group3.zxy * other.group0.yzx) - (float3(other.e321) * this.group2) - (this.group3.yzx * other.group0.zxy)), this.scalar * other.e321)
        );
    }
}
extension MultiVector: GeometricProduct<Point> {
    func geometric_product(other: Point) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2((this.e1 * other.e1) + (this.e2 * other.e2) + (this.e3 * other.e3), -(this.e423 * other.e1) - (this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4)), 
            /* e1, e2, e3, e4 */
            float4(this.e31 * other.e3 * -1.0, this.e12 * other.e1 * -1.0, this.e23 * other.e2 * -1.0, (this.e42 * other.e2) + (this.e43 * other.e3)) + (float4(this.scalar) * other.group0) + (other.group0.yzxx * float4(this.group3.zxy, this.e41)), 
            /* e41, e42, e43 */
            (float3(this.e4) * other.group0.xyz) + (this.group4.zxy * other.group0.yzx) - (float3(other.e4) * this.group1.xyz) - (this.group4.yzx * other.group0.zxy), 
            /* e23, e31, e12 */
            (this.group1.yzx * other.group0.zxy) - (float3(this.e321) * other.group0.xyz) - (this.group1.zxy * other.group0.yzx), 
            /* e423, e431, e412, e321 */
            float4((this.e42 * other.e3) + (this.e23 * other.e4), (this.e43 * other.e1) + (this.e31 * other.e4), (this.e41 * other.e2) + (this.e12 * other.e4), this.e12 * other.e3 * -1.0) - (other.group0.xyzx * float4(this.group0.yy, this.e1234, this.e23)) - (other.group0.yzxy * float4(this.group2.zxy, this.e31))
        );
    }
}
extension MultiVector: GeometricProduct<Scalar> {
    func geometric_product(other: Scalar) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other.scalar) * this.group0, 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group1, 
            /* e41, e42, e43 */
            float3(other.scalar) * this.group2, 
            /* e23, e31, e12 */
            float3(other.scalar) * this.group3, 
            /* e423, e431, e412, e321 */
            float4(other.scalar) * this.group4
        );
    }
}
extension Origin {
    func div(_rhs: geometric_product) -> geometric_product_partial<Origin> {
        geometric_product_partial(this)
    }
}
extension Origin: GeometricProduct<DualNum> {
    func geometric_product(other: DualNum) -> Origin {
        return Origin.from_groups(
            /* e4 */
            other.scalar * this.e4
        );
    }
}
extension Origin: GeometricProduct<Flector> {
    func geometric_product(other: Flector) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.e4) * float4(other.group0.xyz, other.e321), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
extension Origin: GeometricProduct<Horizon> {
    func geometric_product(other: Horizon) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            other.e321 * this.e4
        );
    }
}
extension Origin: GeometricProduct<Line> {
    func geometric_product(other: Line) -> Plane {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(this.e4, this.e4, this.e4, 0.0) * float4(other.group1, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Origin: GeometricProduct<Motor> {
    func geometric_product(other: Motor) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.scalar * this.e4), 
            /* e423, e431, e412, e321 */
            float4(this.e4, this.e4, this.e4, 0.0) * float4(other.group1.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Origin: GeometricProduct<MultiVector> {
    func geometric_product(other: MultiVector) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(1.0, other.e321 * this.e4) * float2(0.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.scalar * this.e4), 
            /* e41, e42, e43 */
            float3(this.e4) * other.group1.xyz, 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(this.e4, this.e4, this.e4, 0.0) * float4(other.group3, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Origin: GeometricProduct<Plane> {
    func geometric_product(other: Plane) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            this.e4 * other.e321
        );
    }
}
extension Origin: GeometricProduct<Point> {
    func geometric_product(other: Point) -> Line {
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(this.e4) * other.group0.xyz, 
            /* e23, e31, e12 */
            float3(0.0)
        );
    }
}
extension Origin: GeometricProduct<Scalar> {
    func geometric_product(other: Scalar) -> Origin {
        return Origin.from_groups(
            /* e4 */
            this.e4 * other.scalar
        );
    }
}
extension Plane {
    func div(_rhs: geometric_product) -> geometric_product_partial<Plane> {
        geometric_product_partial(this)
    }
}
extension Plane: GeometricProduct<AntiScalar> {
    func geometric_product(other: AntiScalar) -> Origin {
        return Origin.from_groups(
            /* e4 */
            other.e1234 * this.e321
        );
    }
}
extension Plane: GeometricProduct<DualNum> {
    func geometric_product(other: DualNum) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e1234 * this.e321), 
            /* e423, e431, e412, e321 */
            float4(other.scalar) * this.group0
        );
    }
}
extension Plane: GeometricProduct<Flector> {
    func geometric_product(other: Flector) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4((other.e2 * this.e412) + (other.e321 * this.e423), (other.e3 * this.e423) + (other.e321 * this.e431), (other.e1 * this.e431) + (other.e321 * this.e412), -(other.e3 * this.e412) - (other.e4 * this.e321)) - (other.group0.zxyx * this.group0.yzxx) - (this.group0.wwwy * float4(other.group1.xyz, other.e2)), 
            /* e23, e31, e12, scalar */
            float4(this.e321) * float4(other.group0.xyz, other.e321) * float4(-1.0)
        );
    }
}
extension Plane: GeometricProduct<Horizon> {
    func geometric_product(other: Horizon) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.e321, other.e321, other.e321, 0.0) * float4(this.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e23, e31, e12, scalar */
            float4(float3(0.0), other.e321 * this.e321 * -1.0)
        );
    }
}
extension Plane: GeometricProduct<Line> {
    func geometric_product(other: Line) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e321, this.e321, this.e321, 1.0) * float4(other.group1, -(other.e23 * this.e423) - (other.e31 * this.e431) - (other.e12 * this.e412)), 
            /* e423, e431, e412, e321 */
            float4((float3(this.e321) * other.group0), 0.0) + float4((other.group1.yzx * this.group0.zxy), 0.0) - float4((other.group1.zxy * this.group0.yzx), 0.0)
        );
    }
}
extension Plane: GeometricProduct<Motor> {
    func geometric_product(other: Motor) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.e321, this.e321, this.e321, 1.0) * float4(other.group1.xyz, (other.e1234 * this.e321) - (other.e23 * this.e423) - (other.e31 * this.e431) - (other.e12 * this.e412)), 
            /* e423, e431, e412, e321 */
            float4(((float3(other.scalar) * this.group0.xyz) + (float3(this.e321) * other.group0.xyz) + (other.group1.yzx * this.group0.zxy) - (other.group1.zxy * this.group0.yzx)), other.scalar * this.e321)
        );
    }
}
extension Plane: GeometricProduct<MultiVector> {
    func geometric_product(other: MultiVector) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other.e321 * this.e321, -(other.e1 * this.e423) - (other.e2 * this.e431) - (other.e3 * this.e412) - (other.e4 * this.e321)) * float2(-1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(this.e321, this.e321, this.e321, 1.0) * float4(other.group3, (other.e1234 * this.e321) - (other.e23 * this.e423) - (other.e31 * this.e431) - (other.e12 * this.e412)), 
            /* e41, e42, e43 */
            (float3(other.e321) * this.group0.xyz) + (other.group1.yzx * this.group0.zxy) - (float3(this.e321) * other.group4.xyz) - (other.group1.zxy * this.group0.yzx), 
            /* e23, e31, e12 */
            float3(this.e321) * other.group1.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            float4(((float3(other.scalar) * this.group0.xyz) + (float3(this.e321) * other.group2) + (other.group3.yzx * this.group0.zxy) - (other.group3.zxy * this.group0.yzx)), other.scalar * this.e321)
        );
    }
}
extension Plane: GeometricProduct<Origin> {
    func geometric_product(other: Origin) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            other.e4 * this.e321 * -1.0
        );
    }
}
extension Plane: GeometricProduct<Plane> {
    func geometric_product(other: Plane) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(((float3(other.e321) * this.group0.xyz) - (float3(this.e321) * other.group0.xyz)), 0.0), 
            /* e23, e31, e12, scalar */
            float4(float3(0.0), other.e321 * this.e321 * -1.0)
        );
    }
}
extension Plane: GeometricProduct<Point> {
    func geometric_product(other: Point) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.e412 * other.e2, this.e423 * other.e3, this.e431 * other.e1, -(this.e431 * other.e2) - (this.e412 * other.e3) - (this.e321 * other.e4)) - (this.group0.yzxx * other.group0.zxyx), 
            /* e23, e31, e12, scalar */
            float4(float3(1.0), 0.0) * float4(this.group0.www, 0.0) * float4(other.group0.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0)
        );
    }
}
extension Plane: GeometricProduct<Scalar> {
    func geometric_product(other: Scalar) -> Plane {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.scalar) * this.group0
        );
    }
}
extension Point {
    func div(_rhs: geometric_product) -> geometric_product_partial<Point> {
        geometric_product_partial(this)
    }
}
extension Point: GeometricProduct<AntiScalar> {
    func geometric_product(other: AntiScalar) -> Plane {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(other.e1234, other.e1234, other.e1234, 0.0) * float4(this.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Point: GeometricProduct<DualNum> {
    func geometric_product(other: DualNum) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group0, 
            /* e423, e431, e412, e321 */
            float4(other.group0.yy, other.e1234, 0.0) * float4(float3(1.0), 0.0) * float4(this.group0.xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Point: GeometricProduct<Flector> {
    func geometric_product(other: Flector) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(-(other.e4 * this.e1) - (other.e431 * this.e3), -(other.e4 * this.e2) - (other.e412 * this.e1), -(other.e4 * this.e3) - (other.e423 * this.e2), (other.e412 * this.e3) + (other.e321 * this.e4)) + (other.group1.zxyy * this.group0.yzxy) + (this.group0.wwwx * float4(other.group0.xyz, other.e423)), 
            /* e23, e31, e12, scalar */
            float4(-(other.e2 * this.e3) - (other.e321 * this.e1), -(other.e3 * this.e1) - (other.e321 * this.e2), -(other.e1 * this.e2) - (other.e321 * this.e3), (other.e2 * this.e2) + (other.e3 * this.e3)) + (other.group0.zxyx * this.group0.yzxx)
        );
    }
}
extension Point: GeometricProduct<Horizon> {
    func geometric_product(other: Horizon) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(float3(0.0), other.e321 * this.e4), 
            /* e23, e31, e12, scalar */
            float4(other.e321, other.e321, other.e321, 0.0) * float4(this.group0.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0)
        );
    }
}
extension Point: GeometricProduct<Line> {
    func geometric_product(other: Line) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.e31 * this.e3, other.e12 * this.e1, other.e23 * this.e2, -(other.e42 * this.e2) - (other.e43 * this.e3)) - (this.group0.yzxx * float4(other.group1.zxy, other.e41)), 
            /* e423, e431, e412, e321 */
            float4((other.e42 * this.e3) + (other.e23 * this.e4), (other.e43 * this.e1) + (other.e31 * this.e4), (other.e41 * this.e2) + (other.e12 * this.e4), -(other.e31 * this.e2) - (other.e12 * this.e3)) - (this.group0.yzxx * float4(other.group0.zxy, other.e23))
        );
    }
}
extension Point: GeometricProduct<Motor> {
    func geometric_product(other: Motor) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.scalar * this.e1, other.scalar * this.e2, other.scalar * this.e3, -(other.e42 * this.e2) - (other.e43 * this.e3)) + (other.group1.yzxw * this.group0.zxyw) - (this.group0.yzxx * float4(other.group1.zxy, other.e41)), 
            /* e423, e431, e412, e321 */
            float4((other.e42 * this.e3) + (other.e1234 * this.e1) + (other.e23 * this.e4), (other.e43 * this.e1) + (other.e1234 * this.e2) + (other.e31 * this.e4), (other.e41 * this.e2) + (other.e1234 * this.e3) + (other.e12 * this.e4), -(other.e31 * this.e2) - (other.e12 * this.e3)) - (this.group0.yzxx * float4(other.group0.zxy, other.e23))
        );
    }
}
extension Point: GeometricProduct<MultiVector> {
    func geometric_product(other: MultiVector) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, other.e321 * this.e4) + (float2(this.e1) * float2(other.e1, other.e423)) + (float2(this.e2) * float2(other.e2, other.e431)) + (float2(this.e3) * float2(other.e3, other.e412)), 
            /* e1, e2, e3, e4 */
            float4(other.e31 * this.e3, other.e12 * this.e1, other.e23 * this.e2, -(other.e42 * this.e2) - (other.e43 * this.e3)) + (float4(other.scalar) * this.group0) - (this.group0.yzxx * float4(other.group3.zxy, other.e41)), 
            /* e41, e42, e43 */
            (float3(this.e4) * other.group1.xyz) + (other.group4.zxy * this.group0.yzx) - (float3(other.e4) * this.group0.xyz) - (other.group4.yzx * this.group0.zxy), 
            /* e23, e31, e12 */
            (other.group1.zxy * this.group0.yzx) - (float3(other.e321) * this.group0.xyz) - (other.group1.yzx * this.group0.zxy), 
            /* e423, e431, e412, e321 */
            float4((other.e1234 * this.e1) + (other.e42 * this.e3) + (other.e23 * this.e4), (other.e1234 * this.e2) + (other.e43 * this.e1) + (other.e31 * this.e4), (other.e1234 * this.e3) + (other.e41 * this.e2) + (other.e12 * this.e4), -(other.e31 * this.e2) - (other.e12 * this.e3)) - (this.group0.yzxx * float4(other.group2.zxy, other.e23))
        );
    }
}
extension Point: GeometricProduct<Origin> {
    func geometric_product(other: Origin) -> Line {
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(other.e4) * this.group0.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
    }
}
extension Point: GeometricProduct<Plane> {
    func geometric_product(other: Plane) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.e431 * this.e3 * -1.0, other.e412 * this.e1 * -1.0, other.e423 * this.e2 * -1.0, (other.e431 * this.e2) + (other.e412 * this.e3) + (other.e321 * this.e4)) + (other.group0.zxyx * this.group0.yzxx), 
            /* e23, e31, e12, scalar */
            float4(float3(1.0), 0.0) * float4(other.group0.www, 0.0) * float4(this.group0.xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0)
        );
    }
}
extension Point: GeometricProduct<Point> {
    func geometric_product(other: Point) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(((float3(this.e4) * other.group0.xyz) - (float3(other.e4) * this.group0.xyz)), 0.0), 
            /* e23, e31, e12, scalar */
            float4(other.e2 * this.e3 * -1.0, other.e3 * this.e1 * -1.0, other.e1 * this.e2 * -1.0, (other.e2 * this.e2) + (other.e3 * this.e3)) + (other.group0.zxyx * this.group0.yzxx)
        );
    }
}
extension Point: GeometricProduct<Scalar> {
    func geometric_product(other: Scalar) -> Point {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * this.group0
        );
    }
}
extension Scalar {
    func div(_rhs: geometric_product) -> geometric_product_partial<Scalar> {
        geometric_product_partial(this)
    }
}
extension Scalar: GeometricProduct<AntiScalar> {
    func geometric_product(other: AntiScalar) -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            other.e1234 * this.scalar
        );
    }
}
extension Scalar: GeometricProduct<DualNum> {
    func geometric_product(other: DualNum) -> DualNum {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(this.scalar) * other.group0
        );
    }
}
extension Scalar: GeometricProduct<Flector> {
    func geometric_product(other: Flector) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group0, 
            /* e423, e431, e412, e321 */
            float4(this.scalar) * other.group1
        );
    }
}
extension Scalar: GeometricProduct<Horizon> {
    func geometric_product(other: Horizon) -> Horizon {
        return Horizon.from_groups(
            /* e321 */
            other.e321 * this.scalar
        );
    }
}
extension Scalar: GeometricProduct<Line> {
    func geometric_product(other: Line) -> Line {
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(this.scalar) * other.group0, 
            /* e23, e31, e12 */
            float3(this.scalar) * other.group1
        );
    }
}
extension Scalar: GeometricProduct<Motor> {
    func geometric_product(other: Motor) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(this.scalar) * other.group0, 
            /* e23, e31, e12, scalar */
            float4(this.scalar) * other.group1
        );
    }
}
extension Scalar: GeometricProduct<MultiVector> {
    func geometric_product(other: MultiVector) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(this.scalar) * other.group0, 
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group1, 
            /* e41, e42, e43 */
            float3(this.scalar) * other.group2, 
            /* e23, e31, e12 */
            float3(this.scalar) * other.group3, 
            /* e423, e431, e412, e321 */
            float4(this.scalar) * other.group4
        );
    }
}
extension Scalar: GeometricProduct<Origin> {
    func geometric_product(other: Origin) -> Origin {
        return Origin.from_groups(
            /* e4 */
            other.e4 * this.scalar
        );
    }
}
extension Scalar: GeometricProduct<Plane> {
    func geometric_product(other: Plane) -> Plane {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(this.scalar) * other.group0
        );
    }
}
extension Scalar: GeometricProduct<Point> {
    func geometric_product(other: Point) -> Point {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(this.scalar) * other.group0
        );
    }
}
extension Scalar: GeometricProduct<Scalar> {
    func geometric_product(other: Scalar) -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            other.scalar * this.scalar
        );
    }
}
