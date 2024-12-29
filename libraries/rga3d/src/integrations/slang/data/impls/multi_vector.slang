implementing rga3d;
using traits;
public extension MultiVector {
    public typedef MultiVector AddOutput;
    public func operator +(other: AntiScalar) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, other.e1234) + this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43 */
            this.group2.xyz, 
            /* e23, e31, e12 */
            this.group3.xyz, 
            /* e423, e431, e412, e321 */
            this.group4
        );
    }
}
public extension MultiVector {
    public func operator +=(const AntiScalar& other) {
        *this = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, other.e1234) + this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43 */
            this.group2.xyz, 
            /* e23, e31, e12 */
            this.group3.xyz, 
            /* e423, e431, e412, e321 */
            this.group4
        );
    }
}
public extension MultiVector {
    public typedef MultiVector AddOutput;
    public func operator +(other: DualNum) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            other.group0.xy + this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43 */
            this.group2.xyz, 
            /* e23, e31, e12 */
            this.group3.xyz, 
            /* e423, e431, e412, e321 */
            this.group4
        );
    }
}
public extension MultiVector {
    public func operator +=(const DualNum& other) {
        *this = MultiVector.from_groups(
            /* scalar, e1234 */
            other.group0.xy + this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43 */
            this.group2.xyz, 
            /* e23, e31, e12 */
            this.group3.xyz, 
            /* e423, e431, e412, e321 */
            this.group4
        );
    }
}
public extension MultiVector {
    public typedef MultiVector AddOutput;
    public func operator +(other: Flector) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            other.group0 + this.group1, 
            /* e41, e42, e43 */
            this.group2.xyz, 
            /* e23, e31, e12 */
            this.group3.xyz, 
            /* e423, e431, e412, e321 */
            other.group1 + this.group4
        );
    }
}
public extension MultiVector {
    public func operator +=(const Flector& other) {
        *this = MultiVector.from_groups(
            /* scalar, e1234 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            other.group0 + this.group1, 
            /* e41, e42, e43 */
            this.group2.xyz, 
            /* e23, e31, e12 */
            this.group3.xyz, 
            /* e423, e431, e412, e321 */
            other.group1 + this.group4
        );
    }
}
public extension MultiVector {
    public typedef MultiVector AddOutput;
    public func operator +(other: Horizon) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43 */
            this.group2.xyz, 
            /* e23, e31, e12 */
            this.group3.xyz, 
            /* e423, e431, e412, e321 */
            this.group4 + float4(float3(0.0), other.e321)
        );
    }
}
public extension MultiVector {
    public func operator +=(const Horizon& other) {
        *this = MultiVector.from_groups(
            /* scalar, e1234 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43 */
            this.group2.xyz, 
            /* e23, e31, e12 */
            this.group3.xyz, 
            /* e423, e431, e412, e321 */
            this.group4 + float4(float3(0.0), other.e321)
        );
    }
}
public extension MultiVector {
    public typedef MultiVector AddOutput;
    public func operator +(other: Line) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43 */
            other.group0.xyz + this.group2.xyz, 
            /* e23, e31, e12 */
            other.group1.xyz + this.group3.xyz, 
            /* e423, e431, e412, e321 */
            this.group4
        );
    }
}
public extension MultiVector {
    public func operator +=(const Line& other) {
        *this = MultiVector.from_groups(
            /* scalar, e1234 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43 */
            other.group0.xyz + this.group2.xyz, 
            /* e23, e31, e12 */
            other.group1.xyz + this.group3.xyz, 
            /* e423, e431, e412, e321 */
            this.group4
        );
    }
}
public extension MultiVector {
    public typedef MultiVector AddOutput;
    public func operator +(other: Motor) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other.scalar, other.e1234) + this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43 */
            this.group2.xyz + other.group0.xyz, 
            /* e23, e31, e12 */
            this.group3.xyz + other.group1.xyz, 
            /* e423, e431, e412, e321 */
            this.group4
        );
    }
}
public extension MultiVector {
    public func operator +=(const Motor& other) {
        *this = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other.scalar, other.e1234) + this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43 */
            this.group2.xyz + other.group0.xyz, 
            /* e23, e31, e12 */
            this.group3.xyz + other.group1.xyz, 
            /* e423, e431, e412, e321 */
            this.group4
        );
    }
}
public extension MultiVector {
    public typedef MultiVector AddOutput;
    public func operator +(other: MultiVector) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            other.group0.xy + this.group0.xy, 
            /* e1, e2, e3, e4 */
            other.group1 + this.group1, 
            /* e41, e42, e43 */
            other.group2.xyz + this.group2.xyz, 
            /* e23, e31, e12 */
            other.group3.xyz + this.group3.xyz, 
            /* e423, e431, e412, e321 */
            other.group4 + this.group4
        );
    }
}
public extension MultiVector {
    public func operator +=(const MultiVector& other) {
        *this = MultiVector.from_groups(
            /* scalar, e1234 */
            other.group0.xy + this.group0.xy, 
            /* e1, e2, e3, e4 */
            other.group1 + this.group1, 
            /* e41, e42, e43 */
            other.group2.xyz + this.group2.xyz, 
            /* e23, e31, e12 */
            other.group3.xyz + this.group3.xyz, 
            /* e423, e431, e412, e321 */
            other.group4 + this.group4
        );
    }
}
public extension MultiVector {
    public typedef MultiVector AddOutput;
    public func operator +(other: Origin) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1 + float4(float3(0.0), other.e4), 
            /* e41, e42, e43 */
            this.group2.xyz, 
            /* e23, e31, e12 */
            this.group3.xyz, 
            /* e423, e431, e412, e321 */
            this.group4
        );
    }
}
public extension MultiVector {
    public func operator +=(const Origin& other) {
        *this = MultiVector.from_groups(
            /* scalar, e1234 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1 + float4(float3(0.0), other.e4), 
            /* e41, e42, e43 */
            this.group2.xyz, 
            /* e23, e31, e12 */
            this.group3.xyz, 
            /* e423, e431, e412, e321 */
            this.group4
        );
    }
}
public extension MultiVector {
    public typedef MultiVector AddOutput;
    public func operator +(other: Plane) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43 */
            this.group2.xyz, 
            /* e23, e31, e12 */
            this.group3.xyz, 
            /* e423, e431, e412, e321 */
            this.group4 + other.group0
        );
    }
}
public extension MultiVector {
    public func operator +=(const Plane& other) {
        *this = MultiVector.from_groups(
            /* scalar, e1234 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43 */
            this.group2.xyz, 
            /* e23, e31, e12 */
            this.group3.xyz, 
            /* e423, e431, e412, e321 */
            this.group4 + other.group0
        );
    }
}
public extension MultiVector {
    public typedef MultiVector AddOutput;
    public func operator +(other: Point) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1 + other.group0, 
            /* e41, e42, e43 */
            this.group2.xyz, 
            /* e23, e31, e12 */
            this.group3.xyz, 
            /* e423, e431, e412, e321 */
            this.group4
        );
    }
}
public extension MultiVector {
    public func operator +=(const Point& other) {
        *this = MultiVector.from_groups(
            /* scalar, e1234 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1 + other.group0, 
            /* e41, e42, e43 */
            this.group2.xyz, 
            /* e23, e31, e12 */
            this.group3.xyz, 
            /* e423, e431, e412, e321 */
            this.group4
        );
    }
}
public extension MultiVector {
    public typedef MultiVector AddOutput;
    public func operator +(other: Scalar) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other.scalar, 0.0) + this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43 */
            this.group2.xyz, 
            /* e23, e31, e12 */
            this.group3.xyz, 
            /* e423, e431, e412, e321 */
            this.group4
        );
    }
}
public extension MultiVector {
    public func operator +=(const Scalar& other) {
        *this = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other.scalar, 0.0) + this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43 */
            this.group2.xyz, 
            /* e23, e31, e12 */
            this.group3.xyz, 
            /* e423, e431, e412, e321 */
            this.group4
        );
    }
}
public extension MultiVector {
    public typedef MultiVector MulOutput;
    public func operator *(other: AntiScalar) -> MultiVector {
        return this.geometric_product(other);
    }
}
public extension MultiVector {
    public func operator *=(const AntiScalar& other) {
        *this = this.geometric_product(other);
    }
}
public extension MultiVector {
    public typedef MultiVector MulOutput;
    public func operator *(other: DualNum) -> MultiVector {
        return this.geometric_product(other);
    }
}
public extension MultiVector {
    public func operator *=(const DualNum& other) {
        *this = this.geometric_product(other);
    }
}
public extension MultiVector {
    public typedef MultiVector MulOutput;
    public func operator *(other: Flector) -> MultiVector {
        return this.geometric_product(other);
    }
}
public extension MultiVector {
    public func operator *=(const Flector& other) {
        *this = this.geometric_product(other);
    }
}
public extension MultiVector {
    public typedef MultiVector MulOutput;
    public func operator *(other: Horizon) -> MultiVector {
        return this.geometric_product(other);
    }
}
public extension MultiVector {
    public func operator *=(const Horizon& other) {
        *this = this.geometric_product(other);
    }
}
public extension MultiVector {
    public typedef MultiVector MulOutput;
    public func operator *(other: Line) -> MultiVector {
        return this.geometric_product(other);
    }
}
public extension MultiVector {
    public func operator *=(const Line& other) {
        *this = this.geometric_product(other);
    }
}
public extension MultiVector {
    public typedef MultiVector MulOutput;
    public func operator *(other: Motor) -> MultiVector {
        return this.geometric_product(other);
    }
}
public extension MultiVector {
    public func operator *=(const Motor& other) {
        *this = this.geometric_product(other);
    }
}
public extension MultiVector {
    public typedef MultiVector MulOutput;
    public func operator *(other: MultiVector) -> MultiVector {
        return this.geometric_product(other);
    }
}
public extension MultiVector {
    public func operator *=(const MultiVector& other) {
        *this = this.geometric_product(other);
    }
}
public extension MultiVector {
    public typedef MultiVector MulOutput;
    public func operator *(other: Origin) -> MultiVector {
        return this.geometric_product(other);
    }
}
public extension MultiVector {
    public func operator *=(const Origin& other) {
        *this = this.geometric_product(other);
    }
}
public extension MultiVector {
    public typedef MultiVector MulOutput;
    public func operator *(other: Plane) -> MultiVector {
        return this.geometric_product(other);
    }
}
public extension MultiVector {
    public func operator *=(const Plane& other) {
        *this = this.geometric_product(other);
    }
}
public extension MultiVector {
    public typedef MultiVector MulOutput;
    public func operator *(other: Point) -> MultiVector {
        return this.geometric_product(other);
    }
}
public extension MultiVector {
    public func operator *=(const Point& other) {
        *this = this.geometric_product(other);
    }
}
public extension MultiVector {
    public typedef MultiVector MulOutput;
    public func operator *(other: Scalar) -> MultiVector {
        return this.geometric_product(other);
    }
}
public extension MultiVector {
    public func operator *=(const Scalar& other) {
        *this = this.geometric_product(other);
    }
}
public extension MultiVector {
    public typedef MultiVector NegOutput;
    public func neg() -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            this.group0.xy * float2(-1.0), 
            /* e1, e2, e3, e4 */
            this.group1 * float4(-1.0), 
            /* e41, e42, e43 */
            this.group2.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            this.group3.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            this.group4 * float4(-1.0)
        );
    }
}
public extension MultiVector {
    public typedef MultiVector NotOutput;
    public func not() -> MultiVector {
        return this.right_dual();
    }
}
public extension MultiVector {
    public typedef MultiVector SubOutput;
    public func operator -(other: AntiScalar) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, other.e1234 * -1.0) + this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43 */
            this.group2.xyz, 
            /* e23, e31, e12 */
            this.group3.xyz, 
            /* e423, e431, e412, e321 */
            this.group4
        );
    }
}
public extension MultiVector {
    public func operator -=(const AntiScalar& other) {
        *this = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, other.e1234 * -1.0) + this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43 */
            this.group2.xyz, 
            /* e23, e31, e12 */
            this.group3.xyz, 
            /* e423, e431, e412, e321 */
            this.group4
        );
    }
}
public extension MultiVector {
    public typedef MultiVector SubOutput;
    public func operator -(other: DualNum) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            this.group0.xy - other.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43 */
            this.group2.xyz, 
            /* e23, e31, e12 */
            this.group3.xyz, 
            /* e423, e431, e412, e321 */
            this.group4
        );
    }
}
public extension MultiVector {
    public func operator -=(const DualNum& other) {
        *this = MultiVector.from_groups(
            /* scalar, e1234 */
            this.group0.xy - other.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43 */
            this.group2.xyz, 
            /* e23, e31, e12 */
            this.group3.xyz, 
            /* e423, e431, e412, e321 */
            this.group4
        );
    }
}
public extension MultiVector {
    public typedef MultiVector SubOutput;
    public func operator -(other: Flector) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1 - other.group0, 
            /* e41, e42, e43 */
            this.group2.xyz, 
            /* e23, e31, e12 */
            this.group3.xyz, 
            /* e423, e431, e412, e321 */
            this.group4 - other.group1
        );
    }
}
public extension MultiVector {
    public func operator -=(const Flector& other) {
        *this = MultiVector.from_groups(
            /* scalar, e1234 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1 - other.group0, 
            /* e41, e42, e43 */
            this.group2.xyz, 
            /* e23, e31, e12 */
            this.group3.xyz, 
            /* e423, e431, e412, e321 */
            this.group4 - other.group1
        );
    }
}
public extension MultiVector {
    public typedef MultiVector SubOutput;
    public func operator -(other: Horizon) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43 */
            this.group2.xyz, 
            /* e23, e31, e12 */
            this.group3.xyz, 
            /* e423, e431, e412, e321 */
            this.group4 + float4(float3(0.0), other.e321 * -1.0)
        );
    }
}
public extension MultiVector {
    public func operator -=(const Horizon& other) {
        *this = MultiVector.from_groups(
            /* scalar, e1234 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43 */
            this.group2.xyz, 
            /* e23, e31, e12 */
            this.group3.xyz, 
            /* e423, e431, e412, e321 */
            this.group4 + float4(float3(0.0), other.e321 * -1.0)
        );
    }
}
public extension MultiVector {
    public typedef MultiVector SubOutput;
    public func operator -(other: Line) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43 */
            this.group2.xyz - other.group0.xyz, 
            /* e23, e31, e12 */
            this.group3.xyz - other.group1.xyz, 
            /* e423, e431, e412, e321 */
            this.group4
        );
    }
}
public extension MultiVector {
    public func operator -=(const Line& other) {
        *this = MultiVector.from_groups(
            /* scalar, e1234 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43 */
            this.group2.xyz - other.group0.xyz, 
            /* e23, e31, e12 */
            this.group3.xyz - other.group1.xyz, 
            /* e423, e431, e412, e321 */
            this.group4
        );
    }
}
public extension MultiVector {
    public typedef MultiVector SubOutput;
    public func operator -(other: Motor) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other.scalar * -1.0, other.e1234 * -1.0) + this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43 */
            this.group2.xyz - other.group0.xyz, 
            /* e23, e31, e12 */
            this.group3.xyz - other.group1.xyz, 
            /* e423, e431, e412, e321 */
            this.group4
        );
    }
}
public extension MultiVector {
    public func operator -=(const Motor& other) {
        *this = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other.scalar * -1.0, other.e1234 * -1.0) + this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43 */
            this.group2.xyz - other.group0.xyz, 
            /* e23, e31, e12 */
            this.group3.xyz - other.group1.xyz, 
            /* e423, e431, e412, e321 */
            this.group4
        );
    }
}
public extension MultiVector {
    public typedef MultiVector SubOutput;
    public func operator -(other: MultiVector) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            this.group0.xy - other.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1 - other.group1, 
            /* e41, e42, e43 */
            this.group2.xyz - other.group2.xyz, 
            /* e23, e31, e12 */
            this.group3.xyz - other.group3.xyz, 
            /* e423, e431, e412, e321 */
            this.group4 - other.group4
        );
    }
}
public extension MultiVector {
    public func operator -=(const MultiVector& other) {
        *this = MultiVector.from_groups(
            /* scalar, e1234 */
            this.group0.xy - other.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1 - other.group1, 
            /* e41, e42, e43 */
            this.group2.xyz - other.group2.xyz, 
            /* e23, e31, e12 */
            this.group3.xyz - other.group3.xyz, 
            /* e423, e431, e412, e321 */
            this.group4 - other.group4
        );
    }
}
public extension MultiVector {
    public typedef MultiVector SubOutput;
    public func operator -(other: Origin) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1 + float4(float3(0.0), other.e4 * -1.0), 
            /* e41, e42, e43 */
            this.group2.xyz, 
            /* e23, e31, e12 */
            this.group3.xyz, 
            /* e423, e431, e412, e321 */
            this.group4
        );
    }
}
public extension MultiVector {
    public func operator -=(const Origin& other) {
        *this = MultiVector.from_groups(
            /* scalar, e1234 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1 + float4(float3(0.0), other.e4 * -1.0), 
            /* e41, e42, e43 */
            this.group2.xyz, 
            /* e23, e31, e12 */
            this.group3.xyz, 
            /* e423, e431, e412, e321 */
            this.group4
        );
    }
}
public extension MultiVector {
    public typedef MultiVector SubOutput;
    public func operator -(other: Plane) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43 */
            this.group2.xyz, 
            /* e23, e31, e12 */
            this.group3.xyz, 
            /* e423, e431, e412, e321 */
            this.group4 - other.group0
        );
    }
}
public extension MultiVector {
    public func operator -=(const Plane& other) {
        *this = MultiVector.from_groups(
            /* scalar, e1234 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43 */
            this.group2.xyz, 
            /* e23, e31, e12 */
            this.group3.xyz, 
            /* e423, e431, e412, e321 */
            this.group4 - other.group0
        );
    }
}
public extension MultiVector {
    public typedef MultiVector SubOutput;
    public func operator -(other: Point) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1 - other.group0, 
            /* e41, e42, e43 */
            this.group2.xyz, 
            /* e23, e31, e12 */
            this.group3.xyz, 
            /* e423, e431, e412, e321 */
            this.group4
        );
    }
}
public extension MultiVector {
    public func operator -=(const Point& other) {
        *this = MultiVector.from_groups(
            /* scalar, e1234 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1 - other.group0, 
            /* e41, e42, e43 */
            this.group2.xyz, 
            /* e23, e31, e12 */
            this.group3.xyz, 
            /* e423, e431, e412, e321 */
            this.group4
        );
    }
}
public extension MultiVector {
    public typedef MultiVector SubOutput;
    public func operator -(other: Scalar) -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other.scalar * -1.0, 0.0) + this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43 */
            this.group2.xyz, 
            /* e23, e31, e12 */
            this.group3.xyz, 
            /* e423, e431, e412, e321 */
            this.group4
        );
    }
}
public extension MultiVector {
    public func operator -=(const Scalar& other) {
        *this = MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other.scalar * -1.0, 0.0) + this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43 */
            this.group2.xyz, 
            /* e23, e31, e12 */
            this.group3.xyz, 
            /* e423, e431, e412, e321 */
            this.group4
        );
    }
}
