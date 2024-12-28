public extension AntiScalar {
    // Fancy postfix trick
    func operator/(_rhs: RightComplementPrefixOrPostfix) -> Scalar {
        return this.right_complement();
    }
}
public extension RightComplementPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: AntiScalar) -> AntiScalar.Output {
        return rhs.right_complement();
    }
}
public extension AntiScalar: RightComplement {
    public typedef Scalar RightComplementOutput;
    public func right_complement() -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            this.e1234
        );
    }
}
public extension DualNum {
    // Fancy postfix trick
    func operator/(_rhs: RightComplementPrefixOrPostfix) -> DualNum {
        return this.right_complement();
    }
    // Fancy postfix self-assign
    public func operator/=(const RightComplementPrefixOrPostfix& _rhs) -> DualNum& {
        this = this.right_complement();
        return *this;
    }
}
public extension RightComplementPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: DualNum) -> DualNum.Output {
        return rhs.right_complement();
    }
}
public extension DualNum: RightComplement {
    public typedef DualNum RightComplementOutput;
    public func right_complement() -> DualNum {
        return DualNum.from_groups(
            /* scalar, e1234 */
            this.group0.xy.yx
        );
    }
}
public extension Flector {
    // Fancy postfix trick
    func operator/(_rhs: RightComplementPrefixOrPostfix) -> Flector {
        return this.right_complement();
    }
    // Fancy postfix self-assign
    public func operator/=(const RightComplementPrefixOrPostfix& _rhs) -> Flector& {
        this = this.right_complement();
        return *this;
    }
}
public extension RightComplementPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Flector) -> Flector.Output {
        return rhs.right_complement();
    }
}
public extension Flector: RightComplement {
    public typedef Flector RightComplementOutput;
    public func right_complement() -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            this.group1 * float4(-1.0), 
            /* e423, e431, e412, e321 */
            this.group0
        );
    }
}
public extension Horizon {
    // Fancy postfix trick
    func operator/(_rhs: RightComplementPrefixOrPostfix) -> Origin {
        return this.right_complement();
    }
}
public extension RightComplementPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Horizon) -> Horizon.Output {
        return rhs.right_complement();
    }
}
public extension Horizon: RightComplement {
    public typedef Origin RightComplementOutput;
    public func right_complement() -> Origin {
        return Origin.from_groups(
            /* e4 */
            this.e321 * -1.0
        );
    }
}
public extension Line {
    // Fancy postfix trick
    func operator/(_rhs: RightComplementPrefixOrPostfix) -> Line {
        return this.right_complement();
    }
    // Fancy postfix self-assign
    public func operator/=(const RightComplementPrefixOrPostfix& _rhs) -> Line& {
        this = this.right_complement();
        return *this;
    }
}
public extension RightComplementPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Line) -> Line.Output {
        return rhs.right_complement();
    }
}
public extension Line: RightComplement {
    public typedef Line RightComplementOutput;
    public func right_complement() -> Line {
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
    func operator/(_rhs: RightComplementPrefixOrPostfix) -> Motor {
        return this.right_complement();
    }
    // Fancy postfix self-assign
    public func operator/=(const RightComplementPrefixOrPostfix& _rhs) -> Motor& {
        this = this.right_complement();
        return *this;
    }
}
public extension RightComplementPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Motor) -> Motor.Output {
        return rhs.right_complement();
    }
}
public extension Motor: RightComplement {
    public typedef Motor RightComplementOutput;
    public func right_complement() -> Motor {
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
    func operator/(_rhs: RightComplementPrefixOrPostfix) -> MultiVector {
        return this.right_complement();
    }
    // Fancy postfix self-assign
    public func operator/=(const RightComplementPrefixOrPostfix& _rhs) -> MultiVector& {
        this = this.right_complement();
        return *this;
    }
}
public extension RightComplementPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: MultiVector) -> MultiVector.Output {
        return rhs.right_complement();
    }
}
public extension MultiVector: RightComplement {
    public typedef MultiVector RightComplementOutput;
    public func right_complement() -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            this.group0.xy.yx, 
            /* e1, e2, e3, e4 */
            this.group4 * float4(-1.0), 
            /* e41, e42, e43 */
            this.group3.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            this.group2.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            this.group1
        );
    }
}
public extension Origin {
    // Fancy postfix trick
    func operator/(_rhs: RightComplementPrefixOrPostfix) -> Horizon {
        return this.right_complement();
    }
}
public extension RightComplementPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Origin) -> Origin.Output {
        return rhs.right_complement();
    }
}
public extension Origin: RightComplement {
    public typedef Horizon RightComplementOutput;
    public func right_complement() -> Horizon {
        return Horizon.from_groups(
            /* e321 */
            this.e4
        );
    }
}
public extension Plane {
    // Fancy postfix trick
    func operator/(_rhs: RightComplementPrefixOrPostfix) -> Point {
        return this.right_complement();
    }
}
public extension RightComplementPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Plane) -> Plane.Output {
        return rhs.right_complement();
    }
}
public extension Plane: RightComplement {
    public typedef Point RightComplementOutput;
    public func right_complement() -> Point {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            this.group0 * float4(-1.0)
        );
    }
}
public extension Point {
    // Fancy postfix trick
    func operator/(_rhs: RightComplementPrefixOrPostfix) -> Plane {
        return this.right_complement();
    }
}
public extension RightComplementPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Point) -> Point.Output {
        return rhs.right_complement();
    }
}
public extension Point: RightComplement {
    public typedef Plane RightComplementOutput;
    public func right_complement() -> Plane {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            this.group0
        );
    }
}
public extension Scalar {
    // Fancy postfix trick
    func operator/(_rhs: RightComplementPrefixOrPostfix) -> AntiScalar {
        return this.right_complement();
    }
}
public extension RightComplementPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Scalar) -> Scalar.Output {
        return rhs.right_complement();
    }
}
public extension Scalar: RightComplement {
    public typedef AntiScalar RightComplementOutput;
    public func right_complement() -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            this.scalar
        );
    }
}
