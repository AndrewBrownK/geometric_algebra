public extension AntiScalar {
    // Fancy postfix trick
    func operator/(_rhs: LeftComplementPrefixOrPostfix) -> Scalar {
        return this.left_complement();
    }
}
public extension LeftComplementPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: AntiScalar) -> AntiScalar.Output {
        return rhs.left_complement();
    }
}
public extension AntiScalar: LeftComplement {
    public typedef Scalar LeftComplementOutput;
    public func left_complement() -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            this.e1234
        );
    }
}
public extension DualNum {
    // Fancy postfix trick
    func operator/(_rhs: LeftComplementPrefixOrPostfix) -> DualNum {
        return this.left_complement();
    }
    // Fancy postfix self-assign
    public func operator/=(const LeftComplementPrefixOrPostfix& _rhs) -> DualNum& {
        this = this.left_complement();
        return *this;
    }
}
public extension LeftComplementPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: DualNum) -> DualNum.Output {
        return rhs.left_complement();
    }
}
public extension DualNum: LeftComplement {
    public typedef DualNum LeftComplementOutput;
    public func left_complement() -> DualNum {
        return DualNum.from_groups(
            /* scalar, e1234 */
            this.group0.xy.yx
        );
    }
}
public extension Flector {
    // Fancy postfix trick
    func operator/(_rhs: LeftComplementPrefixOrPostfix) -> Flector {
        return this.left_complement();
    }
    // Fancy postfix self-assign
    public func operator/=(const LeftComplementPrefixOrPostfix& _rhs) -> Flector& {
        this = this.left_complement();
        return *this;
    }
}
public extension LeftComplementPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Flector) -> Flector.Output {
        return rhs.left_complement();
    }
}
public extension Flector: LeftComplement {
    public typedef Flector LeftComplementOutput;
    public func left_complement() -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e423, e431, e412, e321 */
            this.group0 * float4(-1.0)
        );
    }
}
public extension Horizon {
    // Fancy postfix trick
    func operator/(_rhs: LeftComplementPrefixOrPostfix) -> Origin {
        return this.left_complement();
    }
}
public extension LeftComplementPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Horizon) -> Horizon.Output {
        return rhs.left_complement();
    }
}
public extension Horizon: LeftComplement {
    public typedef Origin LeftComplementOutput;
    public func left_complement() -> Origin {
        return Origin.from_groups(
            /* e4 */
            this.e321
        );
    }
}
public extension Line {
    // Fancy postfix trick
    func operator/(_rhs: LeftComplementPrefixOrPostfix) -> Line {
        return this.left_complement();
    }
    // Fancy postfix self-assign
    public func operator/=(const LeftComplementPrefixOrPostfix& _rhs) -> Line& {
        this = this.left_complement();
        return *this;
    }
}
public extension LeftComplementPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Line) -> Line.Output {
        return rhs.left_complement();
    }
}
public extension Line: LeftComplement {
    public typedef Line LeftComplementOutput;
    public func left_complement() -> Line {
        return Line.from_groups(
            /* e41, e42, e43 */
            this.group1.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            this.group0.xyz * float3(-1.0)
        );
    }
}
public extension Motor {
    // Fancy postfix trick
    func operator/(_rhs: LeftComplementPrefixOrPostfix) -> Motor {
        return this.left_complement();
    }
    // Fancy postfix self-assign
    public func operator/=(const LeftComplementPrefixOrPostfix& _rhs) -> Motor& {
        this = this.left_complement();
        return *this;
    }
}
public extension LeftComplementPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Motor) -> Motor.Output {
        return rhs.left_complement();
    }
}
public extension Motor: LeftComplement {
    public typedef Motor LeftComplementOutput;
    public func left_complement() -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
public extension MultiVector {
    // Fancy postfix trick
    func operator/(_rhs: LeftComplementPrefixOrPostfix) -> MultiVector {
        return this.left_complement();
    }
    // Fancy postfix self-assign
    public func operator/=(const LeftComplementPrefixOrPostfix& _rhs) -> MultiVector& {
        this = this.left_complement();
        return *this;
    }
}
public extension LeftComplementPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: MultiVector) -> MultiVector.Output {
        return rhs.left_complement();
    }
}
public extension MultiVector: LeftComplement {
    public typedef MultiVector LeftComplementOutput;
    public func left_complement() -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            this.group0.xy.yx, 
            /* e1, e2, e3, e4 */
            this.group4, 
            /* e41, e42, e43 */
            this.group3.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            this.group2.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            this.group1 * float4(-1.0)
        );
    }
}
public extension Origin {
    // Fancy postfix trick
    func operator/(_rhs: LeftComplementPrefixOrPostfix) -> Horizon {
        return this.left_complement();
    }
}
public extension LeftComplementPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Origin) -> Origin.Output {
        return rhs.left_complement();
    }
}
public extension Origin: LeftComplement {
    public typedef Horizon LeftComplementOutput;
    public func left_complement() -> Horizon {
        return Horizon.from_groups(
            /* e321 */
            this.e4 * -1.0
        );
    }
}
public extension Plane {
    // Fancy postfix trick
    func operator/(_rhs: LeftComplementPrefixOrPostfix) -> Point {
        return this.left_complement();
    }
}
public extension LeftComplementPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Plane) -> Plane.Output {
        return rhs.left_complement();
    }
}
public extension Plane: LeftComplement {
    public typedef Point LeftComplementOutput;
    public func left_complement() -> Point {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            this.group0
        );
    }
}
public extension Point {
    // Fancy postfix trick
    func operator/(_rhs: LeftComplementPrefixOrPostfix) -> Plane {
        return this.left_complement();
    }
}
public extension LeftComplementPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Point) -> Point.Output {
        return rhs.left_complement();
    }
}
public extension Point: LeftComplement {
    public typedef Plane LeftComplementOutput;
    public func left_complement() -> Plane {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            this.group0 * float4(-1.0)
        );
    }
}
public extension Scalar {
    // Fancy postfix trick
    func operator/(_rhs: LeftComplementPrefixOrPostfix) -> AntiScalar {
        return this.left_complement();
    }
}
public extension LeftComplementPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Scalar) -> Scalar.Output {
        return rhs.left_complement();
    }
}
public extension Scalar: LeftComplement {
    public typedef AntiScalar LeftComplementOutput;
    public func left_complement() -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            this.scalar
        );
    }
}
