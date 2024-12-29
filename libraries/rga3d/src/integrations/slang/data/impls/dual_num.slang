implementing rga3d;
using traits;
public extension DualNum {
    public typedef DualNum AddOutput;
    public func operator +(other: AntiScalar) -> DualNum {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(0.0, other.e1234) + this.group0.xy
        );
    }
}
public extension DualNum {
    public func operator +=(const AntiScalar& other) {
        *this = DualNum.from_groups(
            /* scalar, e1234 */
            float2(0.0, other.e1234) + this.group0.xy
        );
    }
}
public extension DualNum {
    public typedef DualNum AddOutput;
    public func operator +(other: DualNum) -> DualNum {
        return DualNum.from_groups(
            /* scalar, e1234 */
            other.group0.xy + this.group0.xy
        );
    }
}
public extension DualNum {
    public func operator +=(const DualNum& other) {
        *this = DualNum.from_groups(
            /* scalar, e1234 */
            other.group0.xy + this.group0.xy
        );
    }
}
public extension DualNum {
    public typedef MultiVector AddOutput;
    public func operator +(other: Flector) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            other.group0, 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            other.group1
        );
    }
}
public extension DualNum {
    public typedef MultiVector AddOutput;
    public func operator +(other: Horizon) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other.e321)
        );
    }
}
public extension DualNum {
    public typedef Motor AddOutput;
    public func operator +(other: Line) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.group0.xyz, this.e1234), 
            /* e23, e31, e12, scalar */
            float4(other.group1.xyz, this.scalar)
        );
    }
}
public extension DualNum {
    public typedef Motor AddOutput;
    public func operator +(other: Motor) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.group0.xyz, this.e1234 + other.e1234), 
            /* e23, e31, e12, scalar */
            float4(other.group1.xyz, this.scalar + other.scalar)
        );
    }
}
public extension DualNum {
    public typedef MultiVector AddOutput;
    public func operator +(other: MultiVector) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            this.group0.xy + other.group0.xy, 
            /* e1, e2, e3, e4 */
            other.group1, 
            /* e41, e42, e43 */
            other.group2.xyz, 
            /* e23, e31, e12 */
            other.group3.xyz, 
            /* e423, e431, e412, e321 */
            other.group4
        );
    }
}
public extension DualNum {
    public typedef MultiVector AddOutput;
    public func operator +(other: Origin) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e4), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
public extension DualNum {
    public typedef MultiVector AddOutput;
    public func operator +(other: Plane) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            other.group0
        );
    }
}
public extension DualNum {
    public typedef MultiVector AddOutput;
    public func operator +(other: Point) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            other.group0, 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
public extension DualNum {
    public typedef DualNum AddOutput;
    public func operator +(other: Scalar) -> DualNum {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(other.scalar, 0.0) + this.group0.xy
        );
    }
}
public extension DualNum {
    public func operator +=(const Scalar& other) {
        *this = DualNum.from_groups(
            /* scalar, e1234 */
            float2(other.scalar, 0.0) + this.group0.xy
        );
    }
}
public extension DualNum {
    public typedef AntiScalar MulOutput;
    public func operator *(other: AntiScalar) -> AntiScalar {
        return this.geometric_product(other);
    }
}
public extension DualNum {
    public typedef DualNum MulOutput;
    public func operator *(other: DualNum) -> DualNum {
        return this.geometric_product(other);
    }
}
public extension DualNum {
    public func operator *=(const DualNum& other) {
        *this = this.geometric_product(other);
    }
}
public extension DualNum {
    public typedef Flector MulOutput;
    public func operator *(other: Flector) -> Flector {
        return this.geometric_product(other);
    }
}
public extension DualNum {
    public typedef Flector MulOutput;
    public func operator *(other: Horizon) -> Flector {
        return this.geometric_product(other);
    }
}
public extension DualNum {
    public typedef Line MulOutput;
    public func operator *(other: Line) -> Line {
        return this.geometric_product(other);
    }
}
public extension DualNum {
    public typedef Motor MulOutput;
    public func operator *(other: Motor) -> Motor {
        return this.geometric_product(other);
    }
}
public extension DualNum {
    public typedef MultiVector MulOutput;
    public func operator *(other: MultiVector) -> MultiVector {
        return this.geometric_product(other);
    }
}
public extension DualNum {
    public typedef Origin MulOutput;
    public func operator *(other: Origin) -> Origin {
        return this.geometric_product(other);
    }
}
public extension DualNum {
    public typedef Flector MulOutput;
    public func operator *(other: Plane) -> Flector {
        return this.geometric_product(other);
    }
}
public extension DualNum {
    public typedef Flector MulOutput;
    public func operator *(other: Point) -> Flector {
        return this.geometric_product(other);
    }
}
public extension DualNum {
    public typedef DualNum MulOutput;
    public func operator *(other: Scalar) -> DualNum {
        return this.geometric_product(other);
    }
}
public extension DualNum {
    public func operator *=(const Scalar& other) {
        *this = this.geometric_product(other);
    }
}
public extension DualNum {
    public typedef DualNum NegOutput;
    public func neg() -> DualNum {
        return DualNum.from_groups(
            /* scalar, e1234 */
            this.group0.xy * float2(-1.0)
        );
    }
}
public extension DualNum {
    public typedef AntiScalar NotOutput;
    public func not() -> AntiScalar {
        return this.right_dual();
    }
}
public extension DualNum {
    public typedef DualNum SubOutput;
    public func operator -(other: AntiScalar) -> DualNum {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(0.0, other.e1234 * -1.0) + this.group0.xy
        );
    }
}
public extension DualNum {
    public func operator -=(const AntiScalar& other) {
        *this = DualNum.from_groups(
            /* scalar, e1234 */
            float2(0.0, other.e1234 * -1.0) + this.group0.xy
        );
    }
}
public extension DualNum {
    public typedef DualNum SubOutput;
    public func operator -(other: DualNum) -> DualNum {
        return DualNum.from_groups(
            /* scalar, e1234 */
            this.group0.xy - other.group0.xy
        );
    }
}
public extension DualNum {
    public func operator -=(const DualNum& other) {
        *this = DualNum.from_groups(
            /* scalar, e1234 */
            this.group0.xy - other.group0.xy
        );
    }
}
public extension DualNum {
    public typedef MultiVector SubOutput;
    public func operator -(other: Flector) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            other.group0 * float4(-1.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            other.group1 * float4(-1.0)
        );
    }
}
public extension DualNum {
    public typedef MultiVector SubOutput;
    public func operator -(other: Horizon) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other.e321 * -1.0)
        );
    }
}
public extension DualNum {
    public typedef Motor SubOutput;
    public func operator -(other: Line) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.group0.xyz, this.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(other.group1.xyz, this.scalar) * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
public extension DualNum {
    public typedef Motor SubOutput;
    public func operator -(other: Motor) -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.group0.xyz, this.e1234 - other.e1234) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(other.group1.xyz, this.scalar - other.scalar) * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
public extension DualNum {
    public typedef MultiVector SubOutput;
    public func operator -(other: MultiVector) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            this.group0.xy - other.group0.xy, 
            /* e1, e2, e3, e4 */
            other.group1 * float4(-1.0), 
            /* e41, e42, e43 */
            other.group2.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            other.group3.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            other.group4 * float4(-1.0)
        );
    }
}
public extension DualNum {
    public typedef MultiVector SubOutput;
    public func operator -(other: Origin) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other.e4 * -1.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
public extension DualNum {
    public typedef MultiVector SubOutput;
    public func operator -(other: Plane) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            other.group0 * float4(-1.0)
        );
    }
}
public extension DualNum {
    public typedef MultiVector SubOutput;
    public func operator -(other: Point) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            other.group0 * float4(-1.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
public extension DualNum {
    public typedef DualNum SubOutput;
    public func operator -(other: Scalar) -> DualNum {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(other.scalar * -1.0, 0.0) + this.group0.xy
        );
    }
}
public extension DualNum {
    public func operator -=(const Scalar& other) {
        *this = DualNum.from_groups(
            /* scalar, e1234 */
            float2(other.scalar * -1.0, 0.0) + this.group0.xy
        );
    }
}
