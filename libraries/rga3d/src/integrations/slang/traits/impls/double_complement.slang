public extension AntiScalar {
    // Fancy postfix trick
    func operator/(_rhs: DoubleComplementPrefixOrPostfix) -> AntiScalar {
        return this.double_complement();
    }
    // Fancy postfix self-assign
    public func operator/=(const DoubleComplementPrefixOrPostfix& _rhs) -> AntiScalar& {
        this = this.double_complement();
        return *this;
    }
}
public extension DoubleComplementPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: AntiScalar) -> AntiScalar.Output {
        return rhs.double_complement();
    }
}
public extension AntiScalar: DoubleComplement {
    public typedef AntiScalar DoubleComplementOutput;
    public func double_complement() -> AntiScalar {
        return this;
    }
}
public extension DualNum {
    // Fancy postfix trick
    func operator/(_rhs: DoubleComplementPrefixOrPostfix) -> DualNum {
        return this.double_complement();
    }
    // Fancy postfix self-assign
    public func operator/=(const DoubleComplementPrefixOrPostfix& _rhs) -> DualNum& {
        this = this.double_complement();
        return *this;
    }
}
public extension DoubleComplementPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: DualNum) -> DualNum.Output {
        return rhs.double_complement();
    }
}
public extension DualNum: DoubleComplement {
    public typedef DualNum DoubleComplementOutput;
    public func double_complement() -> DualNum {
        return this;
    }
}
public extension Flector {
    // Fancy postfix trick
    func operator/(_rhs: DoubleComplementPrefixOrPostfix) -> Flector {
        return this.double_complement();
    }
    // Fancy postfix self-assign
    public func operator/=(const DoubleComplementPrefixOrPostfix& _rhs) -> Flector& {
        this = this.double_complement();
        return *this;
    }
}
public extension DoubleComplementPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Flector) -> Flector.Output {
        return rhs.double_complement();
    }
}
public extension Flector: DoubleComplement {
    public typedef Flector DoubleComplementOutput;
    public func double_complement() -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            this.group0 * float4(-1.0), 
            /* e423, e431, e412, e321 */
            this.group1 * float4(-1.0)
        );
    }
}
public extension Horizon {
    // Fancy postfix trick
    func operator/(_rhs: DoubleComplementPrefixOrPostfix) -> Horizon {
        return this.double_complement();
    }
    // Fancy postfix self-assign
    public func operator/=(const DoubleComplementPrefixOrPostfix& _rhs) -> Horizon& {
        this = this.double_complement();
        return *this;
    }
}
public extension DoubleComplementPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Horizon) -> Horizon.Output {
        return rhs.double_complement();
    }
}
public extension Horizon: DoubleComplement {
    public typedef Horizon DoubleComplementOutput;
    public func double_complement() -> Horizon {
        return Horizon.from_groups(
            /* e321 */
            this.e321 * -1.0
        );
    }
}
public extension Line {
    // Fancy postfix trick
    func operator/(_rhs: DoubleComplementPrefixOrPostfix) -> Line {
        return this.double_complement();
    }
    // Fancy postfix self-assign
    public func operator/=(const DoubleComplementPrefixOrPostfix& _rhs) -> Line& {
        this = this.double_complement();
        return *this;
    }
}
public extension DoubleComplementPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Line) -> Line.Output {
        return rhs.double_complement();
    }
}
public extension Line: DoubleComplement {
    public typedef Line DoubleComplementOutput;
    public func double_complement() -> Line {
        return this;
    }
}
public extension Motor {
    // Fancy postfix trick
    func operator/(_rhs: DoubleComplementPrefixOrPostfix) -> Motor {
        return this.double_complement();
    }
    // Fancy postfix self-assign
    public func operator/=(const DoubleComplementPrefixOrPostfix& _rhs) -> Motor& {
        this = this.double_complement();
        return *this;
    }
}
public extension DoubleComplementPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Motor) -> Motor.Output {
        return rhs.double_complement();
    }
}
public extension Motor: DoubleComplement {
    public typedef Motor DoubleComplementOutput;
    public func double_complement() -> Motor {
        return this;
    }
}
public extension MultiVector {
    // Fancy postfix trick
    func operator/(_rhs: DoubleComplementPrefixOrPostfix) -> MultiVector {
        return this.double_complement();
    }
    // Fancy postfix self-assign
    public func operator/=(const DoubleComplementPrefixOrPostfix& _rhs) -> MultiVector& {
        this = this.double_complement();
        return *this;
    }
}
public extension DoubleComplementPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: MultiVector) -> MultiVector.Output {
        return rhs.double_complement();
    }
}
public extension MultiVector: DoubleComplement {
    public typedef MultiVector DoubleComplementOutput;
    public func double_complement() -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1 * float4(-1.0), 
            /* e41, e42, e43 */
            this.group2.xyz, 
            /* e23, e31, e12 */
            this.group3.xyz, 
            /* e423, e431, e412, e321 */
            this.group4 * float4(-1.0)
        );
    }
}
public extension Origin {
    // Fancy postfix trick
    func operator/(_rhs: DoubleComplementPrefixOrPostfix) -> Origin {
        return this.double_complement();
    }
    // Fancy postfix self-assign
    public func operator/=(const DoubleComplementPrefixOrPostfix& _rhs) -> Origin& {
        this = this.double_complement();
        return *this;
    }
}
public extension DoubleComplementPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Origin) -> Origin.Output {
        return rhs.double_complement();
    }
}
public extension Origin: DoubleComplement {
    public typedef Origin DoubleComplementOutput;
    public func double_complement() -> Origin {
        return Origin.from_groups(
            /* e4 */
            this.e4 * -1.0
        );
    }
}
public extension Plane {
    // Fancy postfix trick
    func operator/(_rhs: DoubleComplementPrefixOrPostfix) -> Plane {
        return this.double_complement();
    }
    // Fancy postfix self-assign
    public func operator/=(const DoubleComplementPrefixOrPostfix& _rhs) -> Plane& {
        this = this.double_complement();
        return *this;
    }
}
public extension DoubleComplementPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Plane) -> Plane.Output {
        return rhs.double_complement();
    }
}
public extension Plane: DoubleComplement {
    public typedef Plane DoubleComplementOutput;
    public func double_complement() -> Plane {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            this.group0 * float4(-1.0)
        );
    }
}
public extension Point {
    // Fancy postfix trick
    func operator/(_rhs: DoubleComplementPrefixOrPostfix) -> Point {
        return this.double_complement();
    }
    // Fancy postfix self-assign
    public func operator/=(const DoubleComplementPrefixOrPostfix& _rhs) -> Point& {
        this = this.double_complement();
        return *this;
    }
}
public extension DoubleComplementPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Point) -> Point.Output {
        return rhs.double_complement();
    }
}
public extension Point: DoubleComplement {
    public typedef Point DoubleComplementOutput;
    public func double_complement() -> Point {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            this.group0 * float4(-1.0)
        );
    }
}
public extension Scalar {
    // Fancy postfix trick
    func operator/(_rhs: DoubleComplementPrefixOrPostfix) -> Scalar {
        return this.double_complement();
    }
    // Fancy postfix self-assign
    public func operator/=(const DoubleComplementPrefixOrPostfix& _rhs) -> Scalar& {
        this = this.double_complement();
        return *this;
    }
}
public extension DoubleComplementPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Scalar) -> Scalar.Output {
        return rhs.double_complement();
    }
}
public extension Scalar: DoubleComplement {
    public typedef Scalar DoubleComplementOutput;
    public func double_complement() -> Scalar {
        return this;
    }
}
