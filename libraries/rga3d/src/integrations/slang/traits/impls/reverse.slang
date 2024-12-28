public extension AntiScalar {
    // Fancy postfix trick
    func operator/(_rhs: ReversePrefixOrPostfix) -> AntiScalar {
        return this.reverse();
    }
    // Fancy postfix self-assign
    public func operator/=(const ReversePrefixOrPostfix& _rhs) -> AntiScalar& {
        this = this.reverse();
        return *this;
    }
}
public extension ReversePrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: AntiScalar) -> AntiScalar.Output {
        return rhs.reverse();
    }
}
public extension AntiScalar: Reverse {
    public typedef AntiScalar ReverseOutput;
    public func reverse() -> AntiScalar {
        return this;
    }
}
public extension DualNum {
    // Fancy postfix trick
    func operator/(_rhs: ReversePrefixOrPostfix) -> DualNum {
        return this.reverse();
    }
    // Fancy postfix self-assign
    public func operator/=(const ReversePrefixOrPostfix& _rhs) -> DualNum& {
        this = this.reverse();
        return *this;
    }
}
public extension ReversePrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: DualNum) -> DualNum.Output {
        return rhs.reverse();
    }
}
public extension DualNum: Reverse {
    public typedef DualNum ReverseOutput;
    public func reverse() -> DualNum {
        return this;
    }
}
public extension Flector {
    // Fancy postfix trick
    func operator/(_rhs: ReversePrefixOrPostfix) -> Flector {
        return this.reverse();
    }
    // Fancy postfix self-assign
    public func operator/=(const ReversePrefixOrPostfix& _rhs) -> Flector& {
        this = this.reverse();
        return *this;
    }
}
public extension ReversePrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Flector) -> Flector.Output {
        return rhs.reverse();
    }
}
public extension Flector: Reverse {
    public typedef Flector ReverseOutput;
    public func reverse() -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            this.group0, 
            /* e423, e431, e412, e321 */
            this.group1 * float4(-1.0)
        );
    }
}
public extension Horizon {
    // Fancy postfix trick
    func operator/(_rhs: ReversePrefixOrPostfix) -> Horizon {
        return this.reverse();
    }
    // Fancy postfix self-assign
    public func operator/=(const ReversePrefixOrPostfix& _rhs) -> Horizon& {
        this = this.reverse();
        return *this;
    }
}
public extension ReversePrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Horizon) -> Horizon.Output {
        return rhs.reverse();
    }
}
public extension Horizon: Reverse {
    public typedef Horizon ReverseOutput;
    public func reverse() -> Horizon {
        return Horizon.from_groups(
            /* e321 */
            this.e321 * -1.0
        );
    }
}
public extension Line {
    // Fancy postfix trick
    func operator/(_rhs: ReversePrefixOrPostfix) -> Line {
        return this.reverse();
    }
    // Fancy postfix self-assign
    public func operator/=(const ReversePrefixOrPostfix& _rhs) -> Line& {
        this = this.reverse();
        return *this;
    }
}
public extension ReversePrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Line) -> Line.Output {
        return rhs.reverse();
    }
}
public extension Line: Reverse {
    public typedef Line ReverseOutput;
    public func reverse() -> Line {
        return Line.from_groups(
            /* e41, e42, e43 */
            this.group0.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            this.group1.xyz * float3(-1.0)
        );
    }
}
public extension Motor {
    // Fancy postfix trick
    func operator/(_rhs: ReversePrefixOrPostfix) -> Motor {
        return this.reverse();
    }
    // Fancy postfix self-assign
    public func operator/=(const ReversePrefixOrPostfix& _rhs) -> Motor& {
        this = this.reverse();
        return *this;
    }
}
public extension ReversePrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Motor) -> Motor.Output {
        return rhs.reverse();
    }
}
public extension Motor: Reverse {
    public typedef Motor ReverseOutput;
    public func reverse() -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
public extension MultiVector {
    // Fancy postfix trick
    func operator/(_rhs: ReversePrefixOrPostfix) -> MultiVector {
        return this.reverse();
    }
    // Fancy postfix self-assign
    public func operator/=(const ReversePrefixOrPostfix& _rhs) -> MultiVector& {
        this = this.reverse();
        return *this;
    }
}
public extension ReversePrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: MultiVector) -> MultiVector.Output {
        return rhs.reverse();
    }
}
public extension MultiVector: Reverse {
    public typedef MultiVector ReverseOutput;
    public func reverse() -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            this.group0.xy, 
            /* e1, e2, e3, e4 */
            this.group1, 
            /* e41, e42, e43 */
            this.group2.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            this.group3.xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            this.group4 * float4(-1.0)
        );
    }
}
public extension Origin {
    // Fancy postfix trick
    func operator/(_rhs: ReversePrefixOrPostfix) -> Origin {
        return this.reverse();
    }
    // Fancy postfix self-assign
    public func operator/=(const ReversePrefixOrPostfix& _rhs) -> Origin& {
        this = this.reverse();
        return *this;
    }
}
public extension ReversePrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Origin) -> Origin.Output {
        return rhs.reverse();
    }
}
public extension Origin: Reverse {
    public typedef Origin ReverseOutput;
    public func reverse() -> Origin {
        return this;
    }
}
public extension Plane {
    // Fancy postfix trick
    func operator/(_rhs: ReversePrefixOrPostfix) -> Plane {
        return this.reverse();
    }
    // Fancy postfix self-assign
    public func operator/=(const ReversePrefixOrPostfix& _rhs) -> Plane& {
        this = this.reverse();
        return *this;
    }
}
public extension ReversePrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Plane) -> Plane.Output {
        return rhs.reverse();
    }
}
public extension Plane: Reverse {
    public typedef Plane ReverseOutput;
    public func reverse() -> Plane {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            this.group0 * float4(-1.0)
        );
    }
}
public extension Point {
    // Fancy postfix trick
    func operator/(_rhs: ReversePrefixOrPostfix) -> Point {
        return this.reverse();
    }
    // Fancy postfix self-assign
    public func operator/=(const ReversePrefixOrPostfix& _rhs) -> Point& {
        this = this.reverse();
        return *this;
    }
}
public extension ReversePrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Point) -> Point.Output {
        return rhs.reverse();
    }
}
public extension Point: Reverse {
    public typedef Point ReverseOutput;
    public func reverse() -> Point {
        return this;
    }
}
public extension Scalar {
    // Fancy postfix trick
    func operator/(_rhs: ReversePrefixOrPostfix) -> Scalar {
        return this.reverse();
    }
    // Fancy postfix self-assign
    public func operator/=(const ReversePrefixOrPostfix& _rhs) -> Scalar& {
        this = this.reverse();
        return *this;
    }
}
public extension ReversePrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Scalar) -> Scalar.Output {
        return rhs.reverse();
    }
}
public extension Scalar: Reverse {
    public typedef Scalar ReverseOutput;
    public func reverse() -> Scalar {
        return this;
    }
}
