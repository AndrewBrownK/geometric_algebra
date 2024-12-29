implementing rga3d;
using traits;
public extension Point {
    public typedef MultiVector AddOutput;
    public func operator +(other: AntiScalar) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, other.e1234), 
            /* e1, e2, e3, e4 */
            this.group0, 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
public extension Point {
    public typedef MultiVector AddOutput;
    public func operator +(other: DualNum) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            other.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group0, 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
public extension Point {
    public typedef Flector AddOutput;
    public func operator +(other: Flector) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            other.group0 + this.group0, 
            /* e423, e431, e412, e321 */
            other.group1
        );
    }
}
public extension Point {
    public typedef Flector AddOutput;
    public func operator +(other: Horizon) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            this.group0, 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other.e321)
        );
    }
}
public extension Point {
    public typedef MultiVector AddOutput;
    public func operator +(other: Line) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            this.group0, 
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12 */
            other.group1.xyz, 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
public extension Point {
    public typedef MultiVector AddOutput;
    public func operator +(other: Motor) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other.scalar, other.e1234), 
            /* e1, e2, e3, e4 */
            this.group0, 
            /* e41, e42, e43 */
            other.group0.xyz, 
            /* e23, e31, e12 */
            other.group1.xyz, 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
public extension Point {
    public typedef MultiVector AddOutput;
    public func operator +(other: MultiVector) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            other.group0.xy, 
            /* e1, e2, e3, e4 */
            other.group1 + this.group0, 
            /* e41, e42, e43 */
            other.group2.xyz, 
            /* e23, e31, e12 */
            other.group3.xyz, 
            /* e423, e431, e412, e321 */
            other.group4
        );
    }
}
public extension Point {
    public typedef Point AddOutput;
    public func operator +(other: Origin) -> Point {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            this.group0 + float4(float3(0.0), other.e4)
        );
    }
}
public extension Point {
    public func operator +=(const Origin& other) {
        *this = Point.from_groups(
            /* e1, e2, e3, e4 */
            this.group0 + float4(float3(0.0), other.e4)
        );
    }
}
public extension Point {
    public typedef Flector AddOutput;
    public func operator +(other: Plane) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            this.group0, 
            /* e423, e431, e412, e321 */
            other.group0
        );
    }
}
public extension Point {
    public typedef Point AddOutput;
    public func operator +(other: Point) -> Point {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            other.group0 + this.group0
        );
    }
}
public extension Point {
    public func operator +=(const Point& other) {
        *this = Point.from_groups(
            /* e1, e2, e3, e4 */
            other.group0 + this.group0
        );
    }
}
public extension Point {
    public typedef MultiVector AddOutput;
    public func operator +(other: Scalar) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other.scalar, 0.0), 
            /* e1, e2, e3, e4 */
            this.group0, 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
public extension Point {
    public typedef Plane MulOutput;
    public func operator *(other: AntiScalar) -> Plane {
        return this.geometric_product(other);
    }
}
public extension Point {
    public typedef Flector MulOutput;
    public func operator *(other: DualNum) -> Flector {
        return this.geometric_product(other);
    }
}
public extension Point {
    public typedef Motor MulOutput;
    public func operator *(other: Flector) -> Motor {
        return this.geometric_product(other);
    }
}
public extension Point {
    public typedef Motor MulOutput;
    public func operator *(other: Horizon) -> Motor {
        return this.geometric_product(other);
    }
}
public extension Point {
    public typedef Flector MulOutput;
    public func operator *(other: Line) -> Flector {
        return this.geometric_product(other);
    }
}
public extension Point {
    public typedef Flector MulOutput;
    public func operator *(other: Motor) -> Flector {
        return this.geometric_product(other);
    }
}
public extension Point {
    public typedef MultiVector MulOutput;
    public func operator *(other: MultiVector) -> MultiVector {
        return this.geometric_product(other);
    }
}
public extension Point {
    public typedef Line MulOutput;
    public func operator *(other: Origin) -> Line {
        return this.geometric_product(other);
    }
}
public extension Point {
    public typedef Motor MulOutput;
    public func operator *(other: Plane) -> Motor {
        return this.geometric_product(other);
    }
}
public extension Point {
    public typedef Motor MulOutput;
    public func operator *(other: Point) -> Motor {
        return this.geometric_product(other);
    }
}
public extension Point {
    public typedef Point MulOutput;
    public func operator *(other: Scalar) -> Point {
        return this.geometric_product(other);
    }
}
public extension Point {
    public func operator *=(const Scalar& other) {
        *this = this.geometric_product(other);
    }
}
public extension Point {
    public typedef Point NegOutput;
    public func neg() -> Point {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            this.group0 * float4(-1.0)
        );
    }
}
public extension Point {
    public typedef Plane NotOutput;
    public func not() -> Plane {
        return this.right_dual();
    }
}
public extension Point {
    public typedef MultiVector SubOutput;
    public func operator -(other: AntiScalar) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(1.0, other.e1234) * float2(0.0, -1.0), 
            /* e1, e2, e3, e4 */
            this.group0, 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
public extension Point {
    public typedef MultiVector SubOutput;
    public func operator -(other: DualNum) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            other.group0.xy * float2(-1.0), 
            /* e1, e2, e3, e4 */
            this.group0, 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
public extension Point {
    public typedef Flector SubOutput;
    public func operator -(other: Flector) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            this.group0 - other.group0, 
            /* e423, e431, e412, e321 */
            other.group1 * float4(-1.0)
        );
    }
}
public extension Point {
    public typedef Flector SubOutput;
    public func operator -(other: Horizon) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            this.group0, 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other.e321 * -1.0)
        );
    }
}
public extension Point {
    public typedef MultiVector SubOutput;
    public func operator -(other: Line) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            this.group0, 
            /* e41, e42, e43 */
            other.group0.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            other.group1.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
public extension Point {
    public typedef MultiVector SubOutput;
    public func operator -(other: Motor) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other.scalar, other.e1234) * float2(-1.0), 
            /* e1, e2, e3, e4 */
            this.group0, 
            /* e41, e42, e43 */
            other.group0.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            other.group1.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
public extension Point {
    public typedef MultiVector SubOutput;
    public func operator -(other: MultiVector) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            other.group0.xy * float2(-1.0), 
            /* e1, e2, e3, e4 */
            this.group0 - other.group1, 
            /* e41, e42, e43 */
            other.group2.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            other.group3.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            other.group4 * float4(-1.0)
        );
    }
}
public extension Point {
    public typedef Point SubOutput;
    public func operator -(other: Origin) -> Point {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            this.group0 + float4(float3(0.0), other.e4 * -1.0)
        );
    }
}
public extension Point {
    public func operator -=(const Origin& other) {
        *this = Point.from_groups(
            /* e1, e2, e3, e4 */
            this.group0 + float4(float3(0.0), other.e4 * -1.0)
        );
    }
}
public extension Point {
    public typedef Flector SubOutput;
    public func operator -(other: Plane) -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            this.group0, 
            /* e423, e431, e412, e321 */
            other.group0 * float4(-1.0)
        );
    }
}
public extension Point {
    public typedef Point SubOutput;
    public func operator -(other: Point) -> Point {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            this.group0 - other.group0
        );
    }
}
public extension Point {
    public func operator -=(const Point& other) {
        *this = Point.from_groups(
            /* e1, e2, e3, e4 */
            this.group0 - other.group0
        );
    }
}
public extension Point {
    public typedef MultiVector SubOutput;
    public func operator -(other: Scalar) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other.scalar, 1.0) * float2(-1.0, 0.0), 
            /* e1, e2, e3, e4 */
            this.group0, 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
