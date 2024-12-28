public extension DualNum {
    // Fancy postfix trick
    func operator/(_rhs: InversePrefixOrPostfix) -> DualNum {
        return this.inverse();
    }
    // Fancy postfix self-assign
    public func operator/=(const InversePrefixOrPostfix& _rhs) -> DualNum& {
        this = this.inverse();
        return *this;
    }
}
public extension InversePrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: DualNum) -> DualNum.Output {
        return rhs.inverse();
    }
}
public extension DualNum: Inverse {
    public typedef DualNum InverseOutput;
    public func inverse() -> DualNum {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(pow(this.scalar, -2)) * this.group0.xy
        );
    }
}
public extension Flector {
    // Fancy postfix trick
    func operator/(_rhs: InversePrefixOrPostfix) -> Flector {
        return this.inverse();
    }
    // Fancy postfix self-assign
    public func operator/=(const InversePrefixOrPostfix& _rhs) -> Flector& {
        this = this.inverse();
        return *this;
    }
}
public extension InversePrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Flector) -> Flector.Output {
        return rhs.inverse();
    }
}
public extension Flector: Inverse {
    public typedef Flector InverseOutput;
    public func inverse() -> Flector {
        Flector reverse = Flector.from_groups(
            /* e1, e2, e3, e4 */
            this.group0, 
            /* e423, e431, e412, e321 */
            this.group1 * float4(-1.0)
        );
        Scalar other = Scalar.from_groups(
            /* scalar */
            pow(this.e1, 2) + pow(this.e2, 2) + pow(this.e3, 2) + pow(this.e321, 2)
        );
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(other.scalar) * reverse.group0, 
            /* e423, e431, e412, e321 */
            float4(other.scalar) * reverse.group1
        );
    }
}
public extension Horizon {
    // Fancy postfix trick
    func operator/(_rhs: InversePrefixOrPostfix) -> Horizon {
        return this.inverse();
    }
    // Fancy postfix self-assign
    public func operator/=(const InversePrefixOrPostfix& _rhs) -> Horizon& {
        this = this.inverse();
        return *this;
    }
}
public extension InversePrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Horizon) -> Horizon.Output {
        return rhs.inverse();
    }
}
public extension Horizon: Inverse {
    public typedef Horizon InverseOutput;
    public func inverse() -> Horizon {
        return Horizon.from_groups(
            /* e321 */
            1.0/this.e321 * -1.0
        );
    }
}
public extension Line {
    // Fancy postfix trick
    func operator/(_rhs: InversePrefixOrPostfix) -> Line {
        return this.inverse();
    }
    // Fancy postfix self-assign
    public func operator/=(const InversePrefixOrPostfix& _rhs) -> Line& {
        this = this.inverse();
        return *this;
    }
}
public extension InversePrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Line) -> Line.Output {
        return rhs.inverse();
    }
}
public extension Line: Inverse {
    public typedef Line InverseOutput;
    public func inverse() -> Line {
        Line reverse = Line.from_groups(
            /* e41, e42, e43 */
            this.group0.xyz * float3(-1.0), 
            /* e23, e31, e12 */
            this.group1.xyz * float3(-1.0)
        );
        Scalar other = Scalar.from_groups(
            /* scalar */
            pow(this.e23, 2) + pow(this.e31, 2) + pow(this.e12, 2)
        );
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(other.scalar) * reverse.group0.xyz, 
            /* e23, e31, e12 */
            float3(other.scalar) * reverse.group1.xyz
        );
    }
}
public extension Motor {
    // Fancy postfix trick
    func operator/(_rhs: InversePrefixOrPostfix) -> Motor {
        return this.inverse();
    }
    // Fancy postfix self-assign
    public func operator/=(const InversePrefixOrPostfix& _rhs) -> Motor& {
        this = this.inverse();
        return *this;
    }
}
public extension InversePrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Motor) -> Motor.Output {
        return rhs.inverse();
    }
}
public extension Motor: Inverse {
    public typedef Motor InverseOutput;
    public func inverse() -> Motor {
        Motor reverse = Motor.from_groups(
            /* e41, e42, e43, e1234 */
            this.group0 * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            this.group1 * float4(-1.0, -1.0, -1.0, 1.0)
        );
        Scalar other = Scalar.from_groups(
            /* scalar */
            pow(this.e23, 2) + pow(this.e31, 2) + pow(this.e12, 2) + pow(this.scalar, 2)
        );
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(other.scalar) * reverse.group0, 
            /* e23, e31, e12, scalar */
            float4(other.scalar) * reverse.group1
        );
    }
}
public extension MultiVector {
    // Fancy postfix trick
    func operator/(_rhs: InversePrefixOrPostfix) -> MultiVector {
        return this.inverse();
    }
    // Fancy postfix self-assign
    public func operator/=(const InversePrefixOrPostfix& _rhs) -> MultiVector& {
        this = this.inverse();
        return *this;
    }
}
public extension InversePrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: MultiVector) -> MultiVector.Output {
        return rhs.inverse();
    }
}
public extension MultiVector: Inverse {
    public typedef MultiVector InverseOutput;
    public func inverse() -> MultiVector {
        MultiVector reverse = MultiVector.from_groups(
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
        Scalar other = Scalar.from_groups(
            /* scalar */
            pow(this.scalar, 2) + pow(this.e1, 2) + pow(this.e2, 2) + pow(this.e3, 2) + pow(this.e23, 2) + pow(this.e31, 2) + pow(this.e12, 2) + pow(this.e321, 2)
        );
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(other.scalar) * reverse.group0.xy, 
            /* e1, e2, e3, e4 */
            float4(other.scalar) * reverse.group1, 
            /* e41, e42, e43 */
            float3(other.scalar) * reverse.group2.xyz, 
            /* e23, e31, e12 */
            float3(other.scalar) * reverse.group3.xyz, 
            /* e423, e431, e412, e321 */
            float4(other.scalar) * reverse.group4
        );
    }
}
public extension Plane {
    // Fancy postfix trick
    func operator/(_rhs: InversePrefixOrPostfix) -> Plane {
        return this.inverse();
    }
    // Fancy postfix self-assign
    public func operator/=(const InversePrefixOrPostfix& _rhs) -> Plane& {
        this = this.inverse();
        return *this;
    }
}
public extension InversePrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Plane) -> Plane.Output {
        return rhs.inverse();
    }
}
public extension Plane: Inverse {
    public typedef Plane InverseOutput;
    public func inverse() -> Plane {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(pow(this.e321, -2)) * float4(this.e423 * -1.0, this.e431 * -1.0, this.e412 * -1.0, this.e321 * -1.0)
        );
    }
}
public extension Point {
    // Fancy postfix trick
    func operator/(_rhs: InversePrefixOrPostfix) -> Point {
        return this.inverse();
    }
    // Fancy postfix self-assign
    public func operator/=(const InversePrefixOrPostfix& _rhs) -> Point& {
        this = this.inverse();
        return *this;
    }
}
public extension InversePrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Point) -> Point.Output {
        return rhs.inverse();
    }
}
public extension Point: Inverse {
    public typedef Point InverseOutput;
    public func inverse() -> Point {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(pow(this.e1, 2) + pow(this.e2, 2) + pow(this.e3, 2)) * this.group0
        );
    }
}
public extension Scalar {
    // Fancy postfix trick
    func operator/(_rhs: InversePrefixOrPostfix) -> Scalar {
        return this.inverse();
    }
    // Fancy postfix self-assign
    public func operator/=(const InversePrefixOrPostfix& _rhs) -> Scalar& {
        this = this.inverse();
        return *this;
    }
}
public extension InversePrefixOrPostfix {
    // Fancy prefix trick
    public func operator/(rhs: Scalar) -> Scalar.Output {
        return rhs.inverse();
    }
}
public extension Scalar: Inverse {
    public typedef Scalar InverseOutput;
    public func inverse() -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            1.0/this.scalar
        );
    }
}
