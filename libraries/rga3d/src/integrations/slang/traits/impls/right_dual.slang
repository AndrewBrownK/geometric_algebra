public extension DualNum {
    // Fancy postfix trick
    func operator/(_rhs: RightDualPrefixOrPostfix) -> AntiScalar {
        return this.right_dual();
    }
}
public extension RightDualPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: DualNum) -> DualNum.Output {
        return rhs.right_dual();
    }
}
public extension DualNum: RightDual {
    public typedef AntiScalar RightDualOutput;
    public func right_dual() -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            this.scalar
        );
    }
}
public extension Flector {
    // Fancy postfix trick
    func operator/(_rhs: RightDualPrefixOrPostfix) -> Flector {
        return this.right_dual();
    }
    // Fancy postfix self-assign
    public func operator/=(const RightDualPrefixOrPostfix& _rhs) -> Flector& {
        this = this.right_dual();
        return *this;
    }
}
public extension RightDualPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Flector) -> Flector.Output {
        return rhs.right_dual();
    }
}
public extension Flector: RightDual {
    public typedef Flector RightDualOutput;
    public func right_dual() -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e321 * -1.0), 
            /* e423, e431, e412, e321 */
            float4(this.group0.xyz, 0.0)
        );
    }
}
public extension Horizon {
    // Fancy postfix trick
    func operator/(_rhs: RightDualPrefixOrPostfix) -> Origin {
        return this.right_dual();
    }
}
public extension RightDualPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Horizon) -> Horizon.Output {
        return rhs.right_dual();
    }
}
public extension Horizon: RightDual {
    public typedef Origin RightDualOutput;
    public func right_dual() -> Origin {
        return Origin.from_groups(
            /* e4 */
            this.e321 * -1.0
        );
    }
}
public extension Line {
    // Fancy postfix trick
    func operator/(_rhs: RightDualPrefixOrPostfix) -> Line {
        return this.right_dual();
    }
    // Fancy postfix self-assign
    public func operator/=(const RightDualPrefixOrPostfix& _rhs) -> Line& {
        this = this.right_dual();
        return *this;
    }
}
public extension RightDualPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Line) -> Line.Output {
        return rhs.right_dual();
    }
}
public extension Line: RightDual {
    public typedef Line RightDualOutput;
    public func right_dual() -> Line {
        return Line.from_groups(
            /* e41, e42, e43 */
            this.group1.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
    }
}
public extension Motor {
    // Fancy postfix trick
    func operator/(_rhs: RightDualPrefixOrPostfix) -> Motor {
        return this.right_dual();
    }
    // Fancy postfix self-assign
    public func operator/=(const RightDualPrefixOrPostfix& _rhs) -> Motor& {
        this = this.right_dual();
        return *this;
    }
}
public extension RightDualPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Motor) -> Motor.Output {
        return rhs.right_dual();
    }
}
public extension Motor: RightDual {
    public typedef Motor RightDualOutput;
    public func right_dual() -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
public extension MultiVector {
    // Fancy postfix trick
    func operator/(_rhs: RightDualPrefixOrPostfix) -> MultiVector {
        return this.right_dual();
    }
    // Fancy postfix self-assign
    public func operator/=(const RightDualPrefixOrPostfix& _rhs) -> MultiVector& {
        this = this.right_dual();
        return *this;
    }
}
public extension RightDualPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: MultiVector) -> MultiVector.Output {
        return rhs.right_dual();
    }
}
public extension MultiVector: RightDual {
    public typedef MultiVector RightDualOutput;
    public func right_dual() -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, this.scalar), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), this.e321 * -1.0), 
            /* e41, e42, e43 */
            this.group3.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(this.group1.xyz, 0.0)
        );
    }
}
public extension Plane {
    // Fancy postfix trick
    func operator/(_rhs: RightDualPrefixOrPostfix) -> Origin {
        return this.right_dual();
    }
}
public extension RightDualPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Plane) -> Plane.Output {
        return rhs.right_dual();
    }
}
public extension Plane: RightDual {
    public typedef Origin RightDualOutput;
    public func right_dual() -> Origin {
        return Origin.from_groups(
            /* e4 */
            this.e321 * -1.0
        );
    }
}
public extension Point {
    // Fancy postfix trick
    func operator/(_rhs: RightDualPrefixOrPostfix) -> Plane {
        return this.right_dual();
    }
}
public extension RightDualPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Point) -> Point.Output {
        return rhs.right_dual();
    }
}
public extension Point: RightDual {
    public typedef Plane RightDualOutput;
    public func right_dual() -> Plane {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(this.group0.xyz, 0.0)
        );
    }
}
public extension Scalar {
    // Fancy postfix trick
    func operator/(_rhs: RightDualPrefixOrPostfix) -> AntiScalar {
        return this.right_dual();
    }
}
public extension RightDualPrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Scalar) -> Scalar.Output {
        return rhs.right_dual();
    }
}
public extension Scalar: RightDual {
    public typedef AntiScalar RightDualOutput;
    public func right_dual() -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            this.scalar
        );
    }
}
