implementing rga3d;
using traits;
public extension AntiScalar {
    // Fancy postfix trick
    public AntiScalar operator /(AntiInversePrefixOrPostfix rhs)  {
        return this.anti_inverse();
    }
    // Fancy postfix self-assign
    public func operator /=(const AntiInversePrefixOrPostfix& rhs) -> AntiScalar& {
        this = this.anti_inverse();
        return *this;
    }
}
public extension AntiInversePrefixOrPostfix {
    // Fancy prefix trick
    public AntiScalar operator /(AntiScalar rhs) {
        return rhs.anti_inverse();
    }
}
public extension AntiScalar: AntiInverse {
    public typedef AntiScalar AntiInverseOutput;
    public func anti_inverse() -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            1.0/this.e1234
        );
    }
}
public extension DualNum {
    // Fancy postfix trick
    public DualNum operator /(AntiInversePrefixOrPostfix rhs)  {
        return this.anti_inverse();
    }
    // Fancy postfix self-assign
    public func operator /=(const AntiInversePrefixOrPostfix& rhs) -> DualNum& {
        this = this.anti_inverse();
        return *this;
    }
}
public extension AntiInversePrefixOrPostfix {
    // Fancy prefix trick
    public DualNum operator /(DualNum rhs) {
        return rhs.anti_inverse();
    }
}
public extension DualNum: AntiInverse {
    public typedef DualNum AntiInverseOutput;
    public func anti_inverse() -> DualNum {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(pow(this.e1234, -2)) * this.group0.xy
        );
    }
}
public extension Flector {
    // Fancy postfix trick
    public Flector operator /(AntiInversePrefixOrPostfix rhs)  {
        return this.anti_inverse();
    }
    // Fancy postfix self-assign
    public func operator /=(const AntiInversePrefixOrPostfix& rhs) -> Flector& {
        this = this.anti_inverse();
        return *this;
    }
}
public extension AntiInversePrefixOrPostfix {
    // Fancy prefix trick
    public Flector operator /(Flector rhs) {
        return rhs.anti_inverse();
    }
}
public extension Flector: AntiInverse {
    public typedef Flector AntiInverseOutput;
    public func anti_inverse() -> Flector {
        Flector anti_reverse = Flector.from_groups(
            /* e1, e2, e3, e4 */
            this.group0 * float4(-1.0), 
            /* e423, e431, e412, e321 */
            this.group1
        );
        AntiScalar other = AntiScalar.from_groups(
            /* e1234 */
            pow(this.e4, 2) + pow(this.e423, 2) + pow(this.e431, 2) + pow(this.e412, 2)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.e1234) * anti_reverse.group0, 
            /* e423, e431, e412, e321 */
            float4(other.e1234) * anti_reverse.group1
        );
    }
}
public extension Line {
    // Fancy postfix trick
    public Line operator /(AntiInversePrefixOrPostfix rhs)  {
        return this.anti_inverse();
    }
    // Fancy postfix self-assign
    public func operator /=(const AntiInversePrefixOrPostfix& rhs) -> Line& {
        this = this.anti_inverse();
        return *this;
    }
}
public extension AntiInversePrefixOrPostfix {
    // Fancy prefix trick
    public Line operator /(Line rhs) {
        return rhs.anti_inverse();
    }
}
public extension Line: AntiInverse {
    public typedef Line AntiInverseOutput;
    public func anti_inverse() -> Line {
        Line anti_reverse = Line.from_groups(
            /* e41, e42, e43 */
            this.group0.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            this.group1.xyz * float3(-1.0)
        );
        AntiScalar other = AntiScalar.from_groups(
            /* e1234 */
            pow(this.e41, 2) + pow(this.e42, 2) + pow(this.e43, 2)
        );
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(other.e1234) * anti_reverse.group0.xyz, 
            /* e23, e31, e12 */
            float3(other.e1234) * anti_reverse.group1.xyz
        );
    }
}
public extension Motor {
    // Fancy postfix trick
    public Motor operator /(AntiInversePrefixOrPostfix rhs)  {
        return this.anti_inverse();
    }
    // Fancy postfix self-assign
    public func operator /=(const AntiInversePrefixOrPostfix& rhs) -> Motor& {
        this = this.anti_inverse();
        return *this;
    }
}
public extension AntiInversePrefixOrPostfix {
    // Fancy prefix trick
    public Motor operator /(Motor rhs) {
        return rhs.anti_inverse();
    }
}
public extension Motor: AntiInverse {
    public typedef Motor AntiInverseOutput;
    public func anti_inverse() -> Motor {
        Motor anti_reverse = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        AntiScalar other = AntiScalar.from_groups(
            /* e1234 */
            pow(this.e41, 2) + pow(this.e42, 2) + pow(this.e43, 2) + pow(this.e1234, 2)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.e1234) * anti_reverse.group0, 
            /* e23, e31, e12, scalar */
            float4(other.e1234) * anti_reverse.group1
        );
    }
}
public extension MultiVector {
    // Fancy postfix trick
    public MultiVector operator /(AntiInversePrefixOrPostfix rhs)  {
        return this.anti_inverse();
    }
    // Fancy postfix self-assign
    public func operator /=(const AntiInversePrefixOrPostfix& rhs) -> MultiVector& {
        this = this.anti_inverse();
        return *this;
    }
}
public extension AntiInversePrefixOrPostfix {
    // Fancy prefix trick
    public MultiVector operator /(MultiVector rhs) {
        return rhs.anti_inverse();
    }
}
public extension MultiVector: AntiInverse {
    public typedef MultiVector AntiInverseOutput;
    public func anti_inverse() -> MultiVector {
        MultiVector anti_reverse = MultiVector.from_groups(
            /* scalar, e1234 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1 * float4(-1.0), 
            /* e41, e42, e43 */
            this.group2.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            this.group3.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            this.group4
        );
        AntiScalar other = AntiScalar.from_groups(
            /* e1234 */
            pow(this.e1234, 2) + pow(this.e4, 2) + pow(this.e41, 2) + pow(this.e42, 2) + pow(this.e43, 2) + pow(this.e423, 2) + pow(this.e431, 2) + pow(this.e412, 2)
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other.e1234) * anti_reverse.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(other.e1234) * anti_reverse.group1, 
            /* e41, e42, e43 */
            float3(other.e1234) * anti_reverse.group2.xyz, 
            /* e23, e31, e12 */
            float3(other.e1234) * anti_reverse.group3.xyz, 
            /* e423, e431, e412, e321 */
            float4(other.e1234) * anti_reverse.group4
        );
    }
}
public extension Origin {
    // Fancy postfix trick
    public Origin operator /(AntiInversePrefixOrPostfix rhs)  {
        return this.anti_inverse();
    }
    // Fancy postfix self-assign
    public func operator /=(const AntiInversePrefixOrPostfix& rhs) -> Origin& {
        this = this.anti_inverse();
        return *this;
    }
}
public extension AntiInversePrefixOrPostfix {
    // Fancy prefix trick
    public Origin operator /(Origin rhs) {
        return rhs.anti_inverse();
    }
}
public extension Origin: AntiInverse {
    public typedef Origin AntiInverseOutput;
    public func anti_inverse() -> Origin {
        return Origin.from_groups(
            /* e4 */
            1.0/this.e4 * -1.0
        );
    }
}
public extension Plane {
    // Fancy postfix trick
    public Plane operator /(AntiInversePrefixOrPostfix rhs)  {
        return this.anti_inverse();
    }
    // Fancy postfix self-assign
    public func operator /=(const AntiInversePrefixOrPostfix& rhs) -> Plane& {
        this = this.anti_inverse();
        return *this;
    }
}
public extension AntiInversePrefixOrPostfix {
    // Fancy prefix trick
    public Plane operator /(Plane rhs) {
        return rhs.anti_inverse();
    }
}
public extension Plane: AntiInverse {
    public typedef Plane AntiInverseOutput;
    public func anti_inverse() -> Plane {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(pow(this.e423, 2) + pow(this.e431, 2) + pow(this.e412, 2)) * this.group0
        );
    }
}
public extension Point {
    // Fancy postfix trick
    public Point operator /(AntiInversePrefixOrPostfix rhs)  {
        return this.anti_inverse();
    }
    // Fancy postfix self-assign
    public func operator /=(const AntiInversePrefixOrPostfix& rhs) -> Point& {
        this = this.anti_inverse();
        return *this;
    }
}
public extension AntiInversePrefixOrPostfix {
    // Fancy prefix trick
    public Point operator /(Point rhs) {
        return rhs.anti_inverse();
    }
}
public extension Point: AntiInverse {
    public typedef Point AntiInverseOutput;
    public func anti_inverse() -> Point {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(pow(this.e4, -2)) * float4(this.e1 * -1.0, this.e2 * -1.0, this.e3 * -1.0, this.e4 * -1.0)
        );
    }
}
