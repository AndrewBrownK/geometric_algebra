public extension AntiScalar {
    // Fancy postfix trick
    func operator/(_rhs: WeightPrefixOrPostfix) -> AntiScalar {
        return this.weight();
    }
    // Fancy postfix self-assign
    public func operator/=(const WeightPrefixOrPostfix& _rhs) -> AntiScalar& {
        this = this.weight();
        return *this;
    }
}
public extension WeightPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: AntiScalar) -> AntiScalar.Output {
        return rhs.weight();
    }
}
public extension AntiScalar: Weight {
    public typedef AntiScalar WeightOutput;
    public func weight() -> AntiScalar {
        return this;
    }
}
public extension DualNum {
    // Fancy postfix trick
    func operator/(_rhs: WeightPrefixOrPostfix) -> AntiScalar {
        return this.weight();
    }
}
public extension WeightPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: DualNum) -> DualNum.Output {
        return rhs.weight();
    }
}
public extension DualNum: Weight {
    public typedef AntiScalar WeightOutput;
    public func weight() -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            this.e1234
        );
    }
}
public extension Flector {
    // Fancy postfix trick
    func operator/(_rhs: WeightPrefixOrPostfix) -> Flector {
        return this.weight();
    }
    // Fancy postfix self-assign
    public func operator/=(const WeightPrefixOrPostfix& _rhs) -> Flector& {
        this = this.weight();
        return *this;
    }
}
public extension WeightPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Flector) -> Flector.Output {
        return rhs.weight();
    }
}
public extension Flector: Weight {
    public typedef Flector WeightOutput;
    public func weight() -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e4), 
            /* e423, e431, e412, e321 */
            float4(this.group1.xyz, 0.0)
        );
    }
}
public extension Line {
    // Fancy postfix trick
    func operator/(_rhs: WeightPrefixOrPostfix) -> Line {
        return this.weight();
    }
    // Fancy postfix self-assign
    public func operator/=(const WeightPrefixOrPostfix& _rhs) -> Line& {
        this = this.weight();
        return *this;
    }
}
public extension WeightPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Line) -> Line.Output {
        return rhs.weight();
    }
}
public extension Line: Weight {
    public typedef Line WeightOutput;
    public func weight() -> Line {
        return Line.from_groups(
            /* e41, e42, e43 */
            this.group0.xyz, 
            /* e23, e31, e12 */
            float3(0.0)
        );
    }
}
public extension Motor {
    // Fancy postfix trick
    func operator/(_rhs: WeightPrefixOrPostfix) -> Motor {
        return this.weight();
    }
    // Fancy postfix self-assign
    public func operator/=(const WeightPrefixOrPostfix& _rhs) -> Motor& {
        this = this.weight();
        return *this;
    }
}
public extension WeightPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Motor) -> Motor.Output {
        return rhs.weight();
    }
}
public extension Motor: Weight {
    public typedef Motor WeightOutput;
    public func weight() -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            this.group0, 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
public extension MultiVector {
    // Fancy postfix trick
    func operator/(_rhs: WeightPrefixOrPostfix) -> MultiVector {
        return this.weight();
    }
    // Fancy postfix self-assign
    public func operator/=(const WeightPrefixOrPostfix& _rhs) -> MultiVector& {
        this = this.weight();
        return *this;
    }
}
public extension WeightPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: MultiVector) -> MultiVector.Output {
        return rhs.weight();
    }
}
public extension MultiVector: Weight {
    public typedef MultiVector WeightOutput;
    public func weight() -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, this.e1234), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e4), 
            /* e41, e42, e43 */
            this.group2.xyz, 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(this.group4.xyz, 0.0)
        );
    }
}
public extension Origin {
    // Fancy postfix trick
    func operator/(_rhs: WeightPrefixOrPostfix) -> Origin {
        return this.weight();
    }
    // Fancy postfix self-assign
    public func operator/=(const WeightPrefixOrPostfix& _rhs) -> Origin& {
        this = this.weight();
        return *this;
    }
}
public extension WeightPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Origin) -> Origin.Output {
        return rhs.weight();
    }
}
public extension Origin: Weight {
    public typedef Origin WeightOutput;
    public func weight() -> Origin {
        return this;
    }
}
public extension Plane {
    // Fancy postfix trick
    func operator/(_rhs: WeightPrefixOrPostfix) -> Plane {
        return this.weight();
    }
    // Fancy postfix self-assign
    public func operator/=(const WeightPrefixOrPostfix& _rhs) -> Plane& {
        this = this.weight();
        return *this;
    }
}
public extension WeightPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Plane) -> Plane.Output {
        return rhs.weight();
    }
}
public extension Plane: Weight {
    public typedef Plane WeightOutput;
    public func weight() -> Plane {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(this.group0.xyz, 0.0)
        );
    }
}
public extension Point {
    // Fancy postfix trick
    func operator/(_rhs: WeightPrefixOrPostfix) -> Origin {
        return this.weight();
    }
}
public extension WeightPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Point) -> Point.Output {
        return rhs.weight();
    }
}
public extension Point: Weight {
    public typedef Origin WeightOutput;
    public func weight() -> Origin {
        return Origin.from_groups(
            /* e4 */
            this.e4
        );
    }
}
