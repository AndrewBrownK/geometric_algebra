implementing rga3d;
using traits;
public extension Origin {
    public typedef MultiVector AddOutput;
    public func operator +(other: AntiScalar) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, other.e1234), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e4), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
public extension Origin {
    public typedef MultiVector AddOutput;
    public func operator +(other: DualNum) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            other.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e4), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
public extension Origin {
    public typedef Flector AddOutput;
    public func operator +(other: Flector) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            other.group0 + float4(float3(0.0), this.e4), 
            /* e423, e431, e412, e321 */
            other.group1
        );
    }
}
public extension Origin {
    public typedef Flector AddOutput;
    public func operator +(other: Horizon) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e4), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other.e321)
        );
    }
}
public extension Origin {
    public typedef MultiVector AddOutput;
    public func operator +(other: Line) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e4), 
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12 */
            other.group1.xyz, 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
public extension Origin {
    public typedef MultiVector AddOutput;
    public func operator +(other: Motor) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other.scalar, other.e1234), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e4), 
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12 */
            other.group1.xyz, 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
public extension Origin {
    public typedef MultiVector AddOutput;
    public func operator +(other: MultiVector) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            other.group0.xy, 
            /* e1, e2, e3, e4 */
            other.group1 + float4(float3(0.0), this.e4), 
            /* e41, e42, e43 */
            other.group2.xyz, 
            /* e23, e31, e12 */
            other.group3.xyz, 
            /* e423, e431, e412, e321 */
            other.group4
        );
    }
}
public extension Origin {
    public typedef Origin AddOutput;
    public func operator +(other: Origin) -> Origin {
        return Origin.from_groups(
            /* e4 */
            other.e4 + this.e4
        );
    }
}
public extension Origin {
    public func operator +=(const Origin& other) {
        *this = Origin.from_groups(
            /* e4 */
            other.e4 + this.e4
        );
    }
}
public extension Origin {
    public typedef Flector AddOutput;
    public func operator +(other: Plane) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e4), 
            /* e423, e431, e412, e321 */
            other.group0
        );
    }
}
public extension Origin {
    public typedef Point AddOutput;
    public func operator +(other: Point) -> Point {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            other.group0 + float4(float3(0.0), this.e4)
        );
    }
}
public extension Origin {
    public typedef MultiVector AddOutput;
    public func operator +(other: Scalar) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other.scalar, 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e4), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
public extension Origin {
    public typedef Origin MulOutput;
    public func operator *(other: DualNum) -> Origin {
        return this.geometric_product(other);
    }
}
public extension Origin {
    public func operator *=(const DualNum& other) {
        *this = this.geometric_product(other);
    }
}
public extension Origin {
    public typedef Motor MulOutput;
    public func operator *(other: Flector) -> Motor {
        return this.geometric_product(other);
    }
}
public extension Origin {
    public typedef AntiScalar MulOutput;
    public func operator *(other: Horizon) -> AntiScalar {
        return this.geometric_product(other);
    }
}
public extension Origin {
    public typedef Plane MulOutput;
    public func operator *(other: Line) -> Plane {
        return this.geometric_product(other);
    }
}
public extension Origin {
    public typedef Flector MulOutput;
    public func operator *(other: Motor) -> Flector {
        return this.geometric_product(other);
    }
}
public extension Origin {
    public typedef MultiVector MulOutput;
    public func operator *(other: MultiVector) -> MultiVector {
        return this.geometric_product(other);
    }
}
public extension Origin {
    public typedef AntiScalar MulOutput;
    public func operator *(other: Plane) -> AntiScalar {
        return this.geometric_product(other);
    }
}
public extension Origin {
    public typedef Line MulOutput;
    public func operator *(other: Point) -> Line {
        return this.geometric_product(other);
    }
}
public extension Origin {
    public typedef Origin MulOutput;
    public func operator *(other: Scalar) -> Origin {
        return this.geometric_product(other);
    }
}
public extension Origin {
    public func operator *=(const Scalar& other) {
        *this = this.geometric_product(other);
    }
}
public extension Origin {
    public typedef Origin NegOutput;
    public func neg() -> Origin {
        return Origin.from_groups(
            /* e4 */
            this.e4 * -1.0
        );
    }
}
public extension Origin {
    public typedef MultiVector SubOutput;
    public func operator -(other: AntiScalar) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(1.0, other.e1234) * float2(0.0, -1.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e4), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
public extension Origin {
    public typedef MultiVector SubOutput;
    public func operator -(other: DualNum) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            other.group0.xy * float2(-1.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e4), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
public extension Origin {
    public typedef Flector SubOutput;
    public func operator -(other: Flector) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, this.e4 - other.e4) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e423, e431, e412, e321 */
            other.group1 * float4(-1.0)
        );
    }
}
public extension Origin {
    public typedef Flector SubOutput;
    public func operator -(other: Horizon) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e4), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other.e321 * -1.0)
        );
    }
}
public extension Origin {
    public typedef MultiVector SubOutput;
    public func operator -(other: Line) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e4), 
            /* e41, e42, e43 */
            other.group0.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            other.group1.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
public extension Origin {
    public typedef MultiVector SubOutput;
    public func operator -(other: Motor) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other.scalar, other.e1234) * float2(-1.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e4), 
            /* e41, e42, e43 */
            other.group0.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            other.group1.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
public extension Origin {
    public typedef MultiVector SubOutput;
    public func operator -(other: MultiVector) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            other.group0.xy * float2(-1.0), 
            /* e1, e2, e3, e4 */
            float4(other.group1.xyz, this.e4 - other.e4) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e41, e42, e43 */
            other.group2.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            other.group3.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            other.group4 * float4(-1.0)
        );
    }
}
public extension Origin {
    public typedef Origin SubOutput;
    public func operator -(other: Origin) -> Origin {
        return Origin.from_groups(
            /* e4 */
            this.e4 - other.e4
        );
    }
}
public extension Origin {
    public func operator -=(const Origin& other) {
        *this = Origin.from_groups(
            /* e4 */
            this.e4 - other.e4
        );
    }
}
public extension Origin {
    public typedef Flector SubOutput;
    public func operator -(other: Plane) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e4), 
            /* e423, e431, e412, e321 */
            other.group0 * float4(-1.0)
        );
    }
}
public extension Origin {
    public typedef Point SubOutput;
    public func operator -(other: Point) -> Point {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.group0.xyz, this.e4 - other.e4) * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
public extension Origin {
    public typedef MultiVector SubOutput;
    public func operator -(other: Scalar) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other.scalar, 1.0) * float2(-1.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e4), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
