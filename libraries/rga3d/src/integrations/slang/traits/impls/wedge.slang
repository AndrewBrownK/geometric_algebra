extension AntiScalar {
    func div(_rhs: wedge) -> wedge_partial<AntiScalar> {
        wedge_partial(this)
    }
}
extension AntiScalar: Wedge<DualNum> {
    func wedge(self, other: DualNum) -> AntiScalar {
        return AntiScalar::from_groups(
            /* e1234 */
            self[e1234] * other[scalar]
        );
    }
}
extension AntiScalar: Wedge<Motor> {
    func wedge(self, other: Motor) -> AntiScalar {
        return AntiScalar::from_groups(
            /* e1234 */
            self[e1234] * other[scalar]
        );
    }
}
extension AntiScalar: Wedge<MultiVector> {
    func wedge(self, other: MultiVector) -> AntiScalar {
        return AntiScalar::from_groups(
            /* e1234 */
            self[e1234] * other[scalar]
        );
    }
}
extension AntiScalar: Wedge<Scalar> {
    func wedge(self, other: Scalar) -> AntiScalar {
        return AntiScalar::from_groups(
            /* e1234 */
            self[e1234] * other[scalar]
        );
    }
}
extension DualNum {
    func div(_rhs: wedge) -> wedge_partial<DualNum> {
        wedge_partial(this)
    }
}
extension DualNum: Wedge<AntiScalar> {
    func wedge(self, other: AntiScalar) -> AntiScalar {
        return AntiScalar::from_groups(
            /* e1234 */
            other[e1234] * self[scalar]
        );
    }
}
extension DualNum: Wedge<DualNum> {
    func wedge(self, other: DualNum) -> DualNum {
        return DualNum::from_groups(
            /* scalar, e1234 */
            float2(other[scalar] * self[scalar], (other[scalar] * self[e1234]) + (other[e1234] * self[scalar]))
        );
    }
}
extension DualNum: Wedge<Flector> {
    func wedge(self, other: Flector) -> Flector {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(self[scalar]) * other.group0(), 
            /* e423, e431, e412, e321 */
            float4(self[scalar]) * other.group1()
        );
    }
}
extension DualNum: Wedge<Horizon> {
    func wedge(self, other: Horizon) -> Horizon {
        return Horizon::from_groups(
            /* e321 */
            self[scalar] * other[e321]
        );
    }
}
extension DualNum: Wedge<Line> {
    func wedge(self, other: Line) -> Line {
        return Line::from_groups(
            /* e41, e42, e43 */
            float3(self[scalar]) * other.group0(), 
            /* e23, e31, e12 */
            float3(self[scalar]) * other.group1()
        );
    }
}
extension DualNum: Wedge<Motor> {
    func wedge(self, other: Motor) -> Motor {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(other[e41], other[e42], other[e43], 1.0) * float4(self.group0().xx, self[scalar], (self[scalar] * other[e1234]) + (self[e1234] * other[scalar])), 
            /* e23, e31, e12, scalar */
            float4(self[scalar]) * other.group1()
        );
    }
}
extension DualNum: Wedge<MultiVector> {
    func wedge(self, other: MultiVector) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(self[scalar] * other[scalar], (self[scalar] * other[e1234]) + (self[e1234] * other[scalar])), 
            /* e1, e2, e3, e4 */
            float4(self[scalar]) * other.group1(), 
            /* e41, e42, e43 */
            float3(self[scalar]) * other.group2(), 
            /* e23, e31, e12 */
            float3(self[scalar]) * other.group3(), 
            /* e423, e431, e412, e321 */
            float4(self[scalar]) * other.group4()
        );
    }
}
extension DualNum: Wedge<Origin> {
    func wedge(self, other: Origin) -> Origin {
        return Origin::from_groups(
            /* e4 */
            self[scalar] * other[e4]
        );
    }
}
extension DualNum: Wedge<Plane> {
    func wedge(self, other: Plane) -> Plane {
        return Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4(self[scalar]) * other.group0()
        );
    }
}
extension DualNum: Wedge<Point> {
    func wedge(self, other: Point) -> Point {
        return Point::from_groups(
            /* e1, e2, e3, e4 */
            float4(self[scalar]) * other.group0()
        );
    }
}
extension DualNum: Wedge<Scalar> {
    func wedge(self, other: Scalar) -> DualNum {
        return DualNum::from_groups(
            /* scalar, e1234 */
            float2(other[scalar]) * self.group0()
        );
    }
}
extension Flector {
    func div(_rhs: wedge) -> wedge_partial<Flector> {
        wedge_partial(this)
    }
}
extension Flector: Wedge<DualNum> {
    func wedge(self, other: DualNum) -> Flector {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(other[scalar]) * self.group0(), 
            /* e423, e431, e412, e321 */
            float4(other[scalar]) * self.group1()
        );
    }
}
extension Flector: Wedge<Flector> {
    func wedge(self, other: Flector) -> Motor {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            (self.group0().wwwx * float4(other.group0().xyz, other[e423])) + float4(float3(0.0), (other[e431] * self[e2]) + (other[e412] * self[e3]) + (other[e321] * self[e4]) - (other[e2] * self[e431]) - (other[e3] * self[e412]) - (other[e4] * self[e321])) - (other.group0().wwwx * float4(self.group0().xyz, self[e423])), 
            /* e23, e31, e12, scalar */
            float4(((other.group0().zxy * self.group0().yzx) - (other.group0().yzx * self.group0().zxy)), 0.0)
        );
    }
}
extension Flector: Wedge<Horizon> {
    func wedge(self, other: Horizon) -> AntiScalar {
        return AntiScalar::from_groups(
            /* e1234 */
            self[e4] * other[e321]
        );
    }
}
extension Flector: Wedge<Line> {
    func wedge(self, other: Line) -> Plane {
        return Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4((self[e3] * other[e42]) + (self[e4] * other[e23]), (self[e1] * other[e43]) + (self[e4] * other[e31]), (self[e2] * other[e41]) + (self[e4] * other[e12]), -(self[e2] * other[e31]) - (self[e3] * other[e12])) - (self.group0().yzxx * float4(other.group0().zxy, other[e23]))
        );
    }
}
extension Flector: Wedge<Motor> {
    func wedge(self, other: Motor) -> Flector {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(other[scalar]) * self.group0(), 
            /* e423, e431, e412, e321 */
            float4((self[e4] * other[e23]) + (self[e423] * other[scalar]), (self[e4] * other[e31]) + (self[e431] * other[scalar]), (self[e4] * other[e12]) + (self[e412] * other[scalar]), -(self[e2] * other[e31]) - (self[e3] * other[e12])) + float4((self.group0().zxy * other.group0().yzx), self[e321] * other[scalar]) - (self.group0().yzxx * float4(other.group0().zxy, other[e23]))
        );
    }
}
extension Flector: Wedge<MultiVector> {
    func wedge(self, other: MultiVector) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(0.0, (self[e1] * other[e423]) + (self[e2] * other[e431]) + (self[e3] * other[e412]) + (self[e4] * other[e321]) - (self[e423] * other[e1]) - (self[e431] * other[e2]) - (self[e412] * other[e3]) - (self[e321] * other[e4])), 
            /* e1, e2, e3, e4 */
            float4(other[scalar]) * self.group0(), 
            /* e41, e42, e43 */
            (float3(self[e4]) * other.group1().xyz) - (float3(other[e4]) * self.group0().xyz), 
            /* e23, e31, e12 */
            (self.group0().yzx * other.group1().zxy) - (self.group0().zxy * other.group1().yzx), 
            /* e423, e431, e412, e321 */
            float4((self[e3] * other[e42]) + (self[e4] * other[e23]), (self[e1] * other[e43]) + (self[e4] * other[e31]), (self[e2] * other[e41]) + (self[e4] * other[e12]), -(self[e2] * other[e31]) - (self[e3] * other[e12])) + (float4(other[scalar]) * self.group1()) - (self.group0().yzxx * float4(other.group2().zxy, other[e23]))
        );
    }
}
extension Flector: Wedge<Origin> {
    func wedge(self, other: Origin) -> Motor {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(other[e4]) * float4(self.group0().xyz, self[e321]) * float4(-1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
extension Flector: Wedge<Plane> {
    func wedge(self, other: Plane) -> AntiScalar {
        return AntiScalar::from_groups(
            /* e1234 */
            (self[e1] * other[e423]) + (self[e2] * other[e431]) + (self[e3] * other[e412]) + (self[e4] * other[e321])
        );
    }
}
extension Flector: Wedge<Point> {
    func wedge(self, other: Point) -> Motor {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(self[e4] * other[e1], self[e4] * other[e2], self[e4] * other[e3], -(self[e431] * other[e2]) - (self[e412] * other[e3]) - (self[e321] * other[e4])) - (other.group0().wwwx * float4(self.group0().xyz, self[e423])), 
            /* e23, e31, e12, scalar */
            float4(((self.group0().yzx * other.group0().zxy) - (self.group0().zxy * other.group0().yzx)), 0.0)
        );
    }
}
extension Flector: Wedge<Scalar> {
    func wedge(self, other: Scalar) -> Flector {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(other[scalar]) * self.group0(), 
            /* e423, e431, e412, e321 */
            float4(other[scalar]) * self.group1()
        );
    }
}
extension Horizon {
    func div(_rhs: wedge) -> wedge_partial<Horizon> {
        wedge_partial(this)
    }
}
extension Horizon: Wedge<DualNum> {
    func wedge(self, other: DualNum) -> Horizon {
        return Horizon::from_groups(
            /* e321 */
            other[scalar] * self[e321]
        );
    }
}
extension Horizon: Wedge<Flector> {
    func wedge(self, other: Flector) -> AntiScalar {
        return AntiScalar::from_groups(
            /* e1234 */
            other[e4] * self[e321] * -1.0
        );
    }
}
extension Horizon: Wedge<Motor> {
    func wedge(self, other: Motor) -> Horizon {
        return Horizon::from_groups(
            /* e321 */
            self[e321] * other[scalar]
        );
    }
}
extension Horizon: Wedge<MultiVector> {
    func wedge(self, other: MultiVector) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(1.0, self[e321] * other[e4]) * float2(0.0, -1.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), self[e321] * other[scalar])
        );
    }
}
extension Horizon: Wedge<Origin> {
    func wedge(self, other: Origin) -> AntiScalar {
        return AntiScalar::from_groups(
            /* e1234 */
            self[e321] * other[e4] * -1.0
        );
    }
}
extension Horizon: Wedge<Point> {
    func wedge(self, other: Point) -> AntiScalar {
        return AntiScalar::from_groups(
            /* e1234 */
            self[e321] * other[e4] * -1.0
        );
    }
}
extension Horizon: Wedge<Scalar> {
    func wedge(self, other: Scalar) -> Horizon {
        return Horizon::from_groups(
            /* e321 */
            self[e321] * other[scalar]
        );
    }
}
extension Line {
    func div(_rhs: wedge) -> wedge_partial<Line> {
        wedge_partial(this)
    }
}
extension Line: Wedge<DualNum> {
    func wedge(self, other: DualNum) -> Line {
        return Line::from_groups(
            /* e41, e42, e43 */
            float3(other[scalar]) * self.group0(), 
            /* e23, e31, e12 */
            float3(other[scalar]) * self.group1()
        );
    }
}
extension Line: Wedge<Flector> {
    func wedge(self, other: Flector) -> Plane {
        return Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4((other[e3] * self[e42]) + (other[e4] * self[e23]), (other[e1] * self[e43]) + (other[e4] * self[e31]), (other[e2] * self[e41]) + (other[e4] * self[e12]), -(other[e2] * self[e31]) - (other[e3] * self[e12])) - (other.group0().yzxx * float4(self.group0().zxy, self[e23]))
        );
    }
}
extension Line: Wedge<Line> {
    func wedge(self, other: Line) -> AntiScalar {
        return AntiScalar::from_groups(
            /* e1234 */
            -(other[e41] * self[e23]) - (other[e42] * self[e31]) - (other[e43] * self[e12]) - (other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43])
        );
    }
}
extension Line: Wedge<Motor> {
    func wedge(self, other: Motor) -> Motor {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(other[scalar], other[scalar], other[scalar], 1.0) * float4(self.group0(), -(self[e41] * other[e23]) - (self[e42] * other[e31]) - (self[e43] * other[e12]) - (self[e23] * other[e41]) - (self[e31] * other[e42]) - (self[e12] * other[e43])), 
            /* e23, e31, e12, scalar */
            float4(float3(1.0), 0.0) * float4(self.group1(), 0.0) * float4(other.group1().www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Line: Wedge<MultiVector> {
    func wedge(self, other: MultiVector) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(0.0, -(self[e41] * other[e23]) - (self[e42] * other[e31]) - (self[e43] * other[e12]) - (self[e23] * other[e41]) - (self[e31] * other[e42]) - (self[e12] * other[e43])), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(other[scalar]) * self.group0(), 
            /* e23, e31, e12 */
            float3(other[scalar]) * self.group1(), 
            /* e423, e431, e412, e321 */
            float4((self[e42] * other[e3]) + (self[e23] * other[e4]), (self[e43] * other[e1]) + (self[e31] * other[e4]), (self[e41] * other[e2]) + (self[e12] * other[e4]), -(self[e31] * other[e2]) - (self[e12] * other[e3])) - (other.group1().yzxx * float4(self.group0().zxy, self[e23]))
        );
    }
}
extension Line: Wedge<Origin> {
    func wedge(self, other: Origin) -> Plane {
        return Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4(other[e4], other[e4], other[e4], 0.0) * float4(self.group1(), 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Line: Wedge<Point> {
    func wedge(self, other: Point) -> Plane {
        return Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4((self[e42] * other[e3]) + (self[e23] * other[e4]), (self[e43] * other[e1]) + (self[e31] * other[e4]), (self[e41] * other[e2]) + (self[e12] * other[e4]), -(self[e31] * other[e2]) - (self[e12] * other[e3])) - (other.group0().yzxx * float4(self.group0().zxy, self[e23]))
        );
    }
}
extension Line: Wedge<Scalar> {
    func wedge(self, other: Scalar) -> Line {
        return Line::from_groups(
            /* e41, e42, e43 */
            float3(other[scalar]) * self.group0(), 
            /* e23, e31, e12 */
            float3(other[scalar]) * self.group1()
        );
    }
}
extension Motor {
    func div(_rhs: wedge) -> wedge_partial<Motor> {
        wedge_partial(this)
    }
}
extension Motor: Wedge<AntiScalar> {
    func wedge(self, other: AntiScalar) -> AntiScalar {
        return AntiScalar::from_groups(
            /* e1234 */
            other[e1234] * self[scalar]
        );
    }
}
extension Motor: Wedge<DualNum> {
    func wedge(self, other: DualNum) -> Motor {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(self[e41], self[e42], self[e43], 1.0) * float4(other.group0().xx, other[scalar], (other[scalar] * self[e1234]) + (other[e1234] * self[scalar])), 
            /* e23, e31, e12, scalar */
            float4(other[scalar]) * self.group1()
        );
    }
}
extension Motor: Wedge<Flector> {
    func wedge(self, other: Flector) -> Flector {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(self[scalar]) * other.group0(), 
            /* e423, e431, e412, e321 */
            float4((other[e4] * self[e23]) + (other[e423] * self[scalar]), (other[e4] * self[e31]) + (other[e431] * self[scalar]), (other[e4] * self[e12]) + (other[e412] * self[scalar]), -(other[e2] * self[e31]) - (other[e3] * self[e12])) + float4((other.group0().zxy * self.group0().yzx), other[e321] * self[scalar]) - (other.group0().yzxx * float4(self.group0().zxy, self[e23]))
        );
    }
}
extension Motor: Wedge<Horizon> {
    func wedge(self, other: Horizon) -> Horizon {
        return Horizon::from_groups(
            /* e321 */
            other[e321] * self[scalar]
        );
    }
}
extension Motor: Wedge<Line> {
    func wedge(self, other: Line) -> Motor {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(self[scalar], self[scalar], self[scalar], 1.0) * float4(other.group0(), -(other[e41] * self[e23]) - (other[e42] * self[e31]) - (other[e43] * self[e12]) - (other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43])), 
            /* e23, e31, e12, scalar */
            float4(float3(1.0), 0.0) * float4(other.group1(), 0.0) * float4(self.group1().www, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Motor: Wedge<Motor> {
    func wedge(self, other: Motor) -> Motor {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            (float4(other[scalar]) * self.group0()) + (float4(self[scalar]) * other.group0()) + float4(float3(0.0), -(other[e41] * self[e23]) - (other[e42] * self[e31]) - (other[e43] * self[e12]) - (other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43])), 
            /* e23, e31, e12, scalar */
            float4(((float3(other[scalar]) * self.group1().xyz) + (float3(self[scalar]) * other.group1().xyz)), other[scalar] * self[scalar])
        );
    }
}
extension Motor: Wedge<MultiVector> {
    func wedge(self, other: MultiVector) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(self[scalar] * other[scalar], (self[e1234] * other[scalar]) + (self[scalar] * other[e1234]) - (self[e41] * other[e23]) - (self[e42] * other[e31]) - (self[e43] * other[e12]) - (self[e23] * other[e41]) - (self[e31] * other[e42]) - (self[e12] * other[e43])), 
            /* e1, e2, e3, e4 */
            float4(self[scalar]) * other.group1(), 
            /* e41, e42, e43 */
            (float3(self[scalar]) * other.group2()) + (float3(other[scalar]) * self.group0().xyz), 
            /* e23, e31, e12 */
            (float3(self[scalar]) * other.group3()) + (float3(other[scalar]) * self.group1().xyz), 
            /* e423, e431, e412, e321 */
            float4((self[e23] * other[e4]) + (self[scalar] * other[e423]), (self[e31] * other[e4]) + (self[scalar] * other[e431]), (self[e12] * other[e4]) + (self[scalar] * other[e412]), -(self[e31] * other[e2]) - (self[e12] * other[e3])) + float4((self.group0().yzx * other.group1().zxy), self[scalar] * other[e321]) - (other.group1().yzxx * float4(self.group0().zxy, self[e23]))
        );
    }
}
extension Motor: Wedge<Origin> {
    func wedge(self, other: Origin) -> Flector {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), self[scalar] * other[e4]), 
            /* e423, e431, e412, e321 */
            float4(other[e4], other[e4], other[e4], 0.0) * float4(self.group1().xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Motor: Wedge<Plane> {
    func wedge(self, other: Plane) -> Plane {
        return Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4(self[scalar]) * other.group0()
        );
    }
}
extension Motor: Wedge<Point> {
    func wedge(self, other: Point) -> Flector {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(self[scalar]) * other.group0(), 
            /* e423, e431, e412, e321 */
            float4((self[e42] * other[e3]) + (self[e23] * other[e4]), (self[e43] * other[e1]) + (self[e31] * other[e4]), (self[e41] * other[e2]) + (self[e12] * other[e4]), -(self[e31] * other[e2]) - (self[e12] * other[e3])) - (other.group0().yzxx * float4(self.group0().zxy, self[e23]))
        );
    }
}
extension Motor: Wedge<Scalar> {
    func wedge(self, other: Scalar) -> Motor {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(other[scalar]) * self.group0(), 
            /* e23, e31, e12, scalar */
            float4(other[scalar]) * self.group1()
        );
    }
}
extension MultiVector {
    func div(_rhs: wedge) -> wedge_partial<MultiVector> {
        wedge_partial(this)
    }
}
extension MultiVector: Wedge<AntiScalar> {
    func wedge(self, other: AntiScalar) -> AntiScalar {
        return AntiScalar::from_groups(
            /* e1234 */
            other[e1234] * self[scalar]
        );
    }
}
extension MultiVector: Wedge<DualNum> {
    func wedge(self, other: DualNum) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(other[scalar] * self[scalar], (other[scalar] * self[e1234]) + (other[e1234] * self[scalar])), 
            /* e1, e2, e3, e4 */
            float4(other[scalar]) * self.group1(), 
            /* e41, e42, e43 */
            float3(other[scalar]) * self.group2(), 
            /* e23, e31, e12 */
            float3(other[scalar]) * self.group3(), 
            /* e423, e431, e412, e321 */
            float4(other[scalar]) * self.group4()
        );
    }
}
extension MultiVector: Wedge<Flector> {
    func wedge(self, other: Flector) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(0.0, (other[e423] * self[e1]) + (other[e431] * self[e2]) + (other[e412] * self[e3]) + (other[e321] * self[e4]) - (other[e1] * self[e423]) - (other[e2] * self[e431]) - (other[e3] * self[e412]) - (other[e4] * self[e321])), 
            /* e1, e2, e3, e4 */
            float4(self[scalar]) * other.group0(), 
            /* e41, e42, e43 */
            (float3(self[e4]) * other.group0().xyz) - (float3(other[e4]) * self.group1().xyz), 
            /* e23, e31, e12 */
            (other.group0().zxy * self.group1().yzx) - (other.group0().yzx * self.group1().zxy), 
            /* e423, e431, e412, e321 */
            float4((other[e3] * self[e42]) + (other[e4] * self[e23]), (other[e1] * self[e43]) + (other[e4] * self[e31]), (other[e2] * self[e41]) + (other[e4] * self[e12]), -(other[e2] * self[e31]) - (other[e3] * self[e12])) + (float4(self[scalar]) * other.group1()) - (other.group0().yzxx * float4(self.group2().zxy, self[e23]))
        );
    }
}
extension MultiVector: Wedge<Horizon> {
    func wedge(self, other: Horizon) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(1.0, other[e321] * self[e4]) * float2(0.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other[e321] * self[scalar])
        );
    }
}
extension MultiVector: Wedge<Line> {
    func wedge(self, other: Line) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(0.0, -(other[e41] * self[e23]) - (other[e42] * self[e31]) - (other[e43] * self[e12]) - (other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43])), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(self[scalar]) * other.group0(), 
            /* e23, e31, e12 */
            float3(self[scalar]) * other.group1(), 
            /* e423, e431, e412, e321 */
            float4((other[e42] * self[e3]) + (other[e23] * self[e4]), (other[e43] * self[e1]) + (other[e31] * self[e4]), (other[e41] * self[e2]) + (other[e12] * self[e4]), -(other[e31] * self[e2]) - (other[e12] * self[e3])) - (self.group1().yzxx * float4(other.group0().zxy, other[e23]))
        );
    }
}
extension MultiVector: Wedge<Motor> {
    func wedge(self, other: Motor) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(other[scalar] * self[scalar], (other[e1234] * self[scalar]) + (other[scalar] * self[e1234]) - (other[e41] * self[e23]) - (other[e42] * self[e31]) - (other[e43] * self[e12]) - (other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43])), 
            /* e1, e2, e3, e4 */
            float4(other[scalar]) * self.group1(), 
            /* e41, e42, e43 */
            (float3(other[scalar]) * self.group2()) + (float3(self[scalar]) * other.group0().xyz), 
            /* e23, e31, e12 */
            (float3(other[scalar]) * self.group3()) + (float3(self[scalar]) * other.group1().xyz), 
            /* e423, e431, e412, e321 */
            float4((other[e23] * self[e4]) + (other[scalar] * self[e423]), (other[e31] * self[e4]) + (other[scalar] * self[e431]), (other[e12] * self[e4]) + (other[scalar] * self[e412]), -(other[e31] * self[e2]) - (other[e12] * self[e3])) + float4((other.group0().yzx * self.group1().zxy), other[scalar] * self[e321]) - (self.group1().yzxx * float4(other.group0().zxy, other[e23]))
        );
    }
}
extension MultiVector: Wedge<MultiVector> {
    func wedge(self, other: MultiVector) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(other[scalar] * self[scalar], (other[scalar] * self[e1234]) + (other[e1234] * self[scalar]) + (other[e423] * self[e1]) + (other[e431] * self[e2]) + (other[e412] * self[e3]) + (other[e321] * self[e4]) - (other[e1] * self[e423]) - (other[e2] * self[e431]) - (other[e3] * self[e412]) - (other[e4] * self[e321]) - (other[e41] * self[e23]) - (other[e42] * self[e31]) - (other[e43] * self[e12]) - (other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43])), 
            /* e1, e2, e3, e4 */
            (float4(other[scalar]) * self.group1()) + (float4(self[scalar]) * other.group1()), 
            /* e41, e42, e43 */
            (float3(other[scalar]) * self.group2()) + (float3(self[scalar]) * other.group2()) + (float3(self[e4]) * other.group1().xyz) - (float3(other[e4]) * self.group1().xyz), 
            /* e23, e31, e12 */
            (float3(other[scalar]) * self.group3()) + (float3(self[scalar]) * other.group3()) + (other.group1().zxy * self.group1().yzx) - (other.group1().yzx * self.group1().zxy), 
            /* e423, e431, e412, e321 */
            float4((other[e3] * self[e42]) + (other[e4] * self[e23]) + (other[e42] * self[e3]) + (other[e23] * self[e4]), (other[e1] * self[e43]) + (other[e4] * self[e31]) + (other[e43] * self[e1]) + (other[e31] * self[e4]), (other[e2] * self[e41]) + (other[e4] * self[e12]) + (other[e41] * self[e2]) + (other[e12] * self[e4]), -(other[e1] * self[e23]) - (other[e2] * self[e31]) - (other[e3] * self[e12]) - (other[e12] * self[e3])) + (float4(other[scalar]) * self.group4()) + (float4(self[scalar]) * other.group4()) - (self.group1().yzxx * float4(other.group2().zxy, other[e23])) - float4((self.group2().zxy * other.group1().yzx), other[e31] * self[e2])
        );
    }
}
extension MultiVector: Wedge<Origin> {
    func wedge(self, other: Origin) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(1.0, self[e321] * other[e4]) * float2(0.0, -1.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), self[scalar] * other[e4]), 
            /* e41, e42, e43 */
            float3(other[e4]) * self.group1().xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(other[e4], other[e4], other[e4], 0.0) * float4(self.group3(), 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension MultiVector: Wedge<Plane> {
    func wedge(self, other: Plane) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(0.0, (self[e1] * other[e423]) + (self[e2] * other[e431]) + (self[e3] * other[e412]) + (self[e4] * other[e321])), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(self[scalar]) * other.group0()
        );
    }
}
extension MultiVector: Wedge<Point> {
    func wedge(self, other: Point) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(0.0, -(self[e423] * other[e1]) - (self[e431] * other[e2]) - (self[e412] * other[e3]) - (self[e321] * other[e4])), 
            /* e1, e2, e3, e4 */
            float4(self[scalar]) * other.group0(), 
            /* e41, e42, e43 */
            (float3(self[e4]) * other.group0().xyz) - (float3(other[e4]) * self.group1().xyz), 
            /* e23, e31, e12 */
            (self.group1().yzx * other.group0().zxy) - (self.group1().zxy * other.group0().yzx), 
            /* e423, e431, e412, e321 */
            float4((self[e42] * other[e3]) + (self[e23] * other[e4]), (self[e43] * other[e1]) + (self[e31] * other[e4]), (self[e41] * other[e2]) + (self[e12] * other[e4]), -(self[e31] * other[e2]) - (self[e12] * other[e3])) - (other.group0().yzxx * float4(self.group2().zxy, self[e23]))
        );
    }
}
extension MultiVector: Wedge<Scalar> {
    func wedge(self, other: Scalar) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(other[scalar]) * self.group0(), 
            /* e1, e2, e3, e4 */
            float4(other[scalar]) * self.group1(), 
            /* e41, e42, e43 */
            float3(other[scalar]) * self.group2(), 
            /* e23, e31, e12 */
            float3(other[scalar]) * self.group3(), 
            /* e423, e431, e412, e321 */
            float4(other[scalar]) * self.group4()
        );
    }
}
extension Origin {
    func div(_rhs: wedge) -> wedge_partial<Origin> {
        wedge_partial(this)
    }
}
extension Origin: Wedge<DualNum> {
    func wedge(self, other: DualNum) -> Origin {
        return Origin::from_groups(
            /* e4 */
            other[scalar] * self[e4]
        );
    }
}
extension Origin: Wedge<Flector> {
    func wedge(self, other: Flector) -> Motor {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(self[e4]) * float4(other.group0().xyz, other[e321]), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
extension Origin: Wedge<Horizon> {
    func wedge(self, other: Horizon) -> AntiScalar {
        return AntiScalar::from_groups(
            /* e1234 */
            other[e321] * self[e4]
        );
    }
}
extension Origin: Wedge<Line> {
    func wedge(self, other: Line) -> Plane {
        return Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4(self[e4], self[e4], self[e4], 0.0) * float4(other.group1(), 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Origin: Wedge<Motor> {
    func wedge(self, other: Motor) -> Flector {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other[scalar] * self[e4]), 
            /* e423, e431, e412, e321 */
            float4(self[e4], self[e4], self[e4], 0.0) * float4(other.group1().xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Origin: Wedge<MultiVector> {
    func wedge(self, other: MultiVector) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(1.0, other[e321] * self[e4]) * float2(0.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other[scalar] * self[e4]), 
            /* e41, e42, e43 */
            float3(self[e4]) * other.group1().xyz, 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(self[e4], self[e4], self[e4], 0.0) * float4(other.group3(), 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Origin: Wedge<Plane> {
    func wedge(self, other: Plane) -> AntiScalar {
        return AntiScalar::from_groups(
            /* e1234 */
            self[e4] * other[e321]
        );
    }
}
extension Origin: Wedge<Point> {
    func wedge(self, other: Point) -> Line {
        return Line::from_groups(
            /* e41, e42, e43 */
            float3(self[e4]) * other.group0().xyz, 
            /* e23, e31, e12 */
            float3(0.0)
        );
    }
}
extension Origin: Wedge<Scalar> {
    func wedge(self, other: Scalar) -> Origin {
        return Origin::from_groups(
            /* e4 */
            self[e4] * other[scalar]
        );
    }
}
extension Plane {
    func div(_rhs: wedge) -> wedge_partial<Plane> {
        wedge_partial(this)
    }
}
extension Plane: Wedge<DualNum> {
    func wedge(self, other: DualNum) -> Plane {
        return Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4(other[scalar]) * self.group0()
        );
    }
}
extension Plane: Wedge<Flector> {
    func wedge(self, other: Flector) -> AntiScalar {
        return AntiScalar::from_groups(
            /* e1234 */
            -(other[e1] * self[e423]) - (other[e2] * self[e431]) - (other[e3] * self[e412]) - (other[e4] * self[e321])
        );
    }
}
extension Plane: Wedge<Motor> {
    func wedge(self, other: Motor) -> Plane {
        return Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4(other[scalar]) * self.group0()
        );
    }
}
extension Plane: Wedge<MultiVector> {
    func wedge(self, other: MultiVector) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(0.0, -(other[e1] * self[e423]) - (other[e2] * self[e431]) - (other[e3] * self[e412]) - (other[e4] * self[e321])), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(other[scalar]) * self.group0()
        );
    }
}
extension Plane: Wedge<Origin> {
    func wedge(self, other: Origin) -> AntiScalar {
        return AntiScalar::from_groups(
            /* e1234 */
            other[e4] * self[e321] * -1.0
        );
    }
}
extension Plane: Wedge<Point> {
    func wedge(self, other: Point) -> AntiScalar {
        return AntiScalar::from_groups(
            /* e1234 */
            -(self[e423] * other[e1]) - (self[e431] * other[e2]) - (self[e412] * other[e3]) - (self[e321] * other[e4])
        );
    }
}
extension Plane: Wedge<Scalar> {
    func wedge(self, other: Scalar) -> Plane {
        return Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4(other[scalar]) * self.group0()
        );
    }
}
extension Point {
    func div(_rhs: wedge) -> wedge_partial<Point> {
        wedge_partial(this)
    }
}
extension Point: Wedge<DualNum> {
    func wedge(self, other: DualNum) -> Point {
        return Point::from_groups(
            /* e1, e2, e3, e4 */
            float4(other[scalar]) * self.group0()
        );
    }
}
extension Point: Wedge<Flector> {
    func wedge(self, other: Flector) -> Motor {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(other[e4] * self[e1] * -1.0, other[e4] * self[e2] * -1.0, other[e4] * self[e3] * -1.0, (other[e431] * self[e2]) + (other[e412] * self[e3]) + (other[e321] * self[e4])) + (self.group0().wwwx * float4(other.group0().xyz, other[e423])), 
            /* e23, e31, e12, scalar */
            float4(((other.group0().zxy * self.group0().yzx) - (other.group0().yzx * self.group0().zxy)), 0.0)
        );
    }
}
extension Point: Wedge<Horizon> {
    func wedge(self, other: Horizon) -> AntiScalar {
        return AntiScalar::from_groups(
            /* e1234 */
            other[e321] * self[e4]
        );
    }
}
extension Point: Wedge<Line> {
    func wedge(self, other: Line) -> Plane {
        return Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4((other[e42] * self[e3]) + (other[e23] * self[e4]), (other[e43] * self[e1]) + (other[e31] * self[e4]), (other[e41] * self[e2]) + (other[e12] * self[e4]), -(other[e31] * self[e2]) - (other[e12] * self[e3])) - (self.group0().yzxx * float4(other.group0().zxy, other[e23]))
        );
    }
}
extension Point: Wedge<Motor> {
    func wedge(self, other: Motor) -> Flector {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(other[scalar]) * self.group0(), 
            /* e423, e431, e412, e321 */
            float4((other[e42] * self[e3]) + (other[e23] * self[e4]), (other[e43] * self[e1]) + (other[e31] * self[e4]), (other[e41] * self[e2]) + (other[e12] * self[e4]), -(other[e31] * self[e2]) - (other[e12] * self[e3])) - (self.group0().yzxx * float4(other.group0().zxy, other[e23]))
        );
    }
}
extension Point: Wedge<MultiVector> {
    func wedge(self, other: MultiVector) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(0.0, (other[e423] * self[e1]) + (other[e431] * self[e2]) + (other[e412] * self[e3]) + (other[e321] * self[e4])), 
            /* e1, e2, e3, e4 */
            float4(other[scalar]) * self.group0(), 
            /* e41, e42, e43 */
            (float3(self[e4]) * other.group1().xyz) - (float3(other[e4]) * self.group0().xyz), 
            /* e23, e31, e12 */
            (other.group1().zxy * self.group0().yzx) - (other.group1().yzx * self.group0().zxy), 
            /* e423, e431, e412, e321 */
            float4((other[e42] * self[e3]) + (other[e23] * self[e4]), (other[e43] * self[e1]) + (other[e31] * self[e4]), (other[e41] * self[e2]) + (other[e12] * self[e4]), -(other[e31] * self[e2]) - (other[e12] * self[e3])) - (self.group0().yzxx * float4(other.group2().zxy, other[e23]))
        );
    }
}
extension Point: Wedge<Origin> {
    func wedge(self, other: Origin) -> Line {
        return Line::from_groups(
            /* e41, e42, e43 */
            float3(other[e4]) * self.group0().xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
    }
}
extension Point: Wedge<Plane> {
    func wedge(self, other: Plane) -> AntiScalar {
        return AntiScalar::from_groups(
            /* e1234 */
            (other[e423] * self[e1]) + (other[e431] * self[e2]) + (other[e412] * self[e3]) + (other[e321] * self[e4])
        );
    }
}
extension Point: Wedge<Point> {
    func wedge(self, other: Point) -> Line {
        return Line::from_groups(
            /* e41, e42, e43 */
            (float3(self[e4]) * other.group0().xyz) - (float3(other[e4]) * self.group0().xyz), 
            /* e23, e31, e12 */
            (other.group0().zxy * self.group0().yzx) - (other.group0().yzx * self.group0().zxy)
        );
    }
}
extension Point: Wedge<Scalar> {
    func wedge(self, other: Scalar) -> Point {
        return Point::from_groups(
            /* e1, e2, e3, e4 */
            float4(other[scalar]) * self.group0()
        );
    }
}
extension Scalar {
    func div(_rhs: wedge) -> wedge_partial<Scalar> {
        wedge_partial(this)
    }
}
extension Scalar: Wedge<AntiScalar> {
    func wedge(self, other: AntiScalar) -> AntiScalar {
        return AntiScalar::from_groups(
            /* e1234 */
            other[e1234] * self[scalar]
        );
    }
}
extension Scalar: Wedge<DualNum> {
    func wedge(self, other: DualNum) -> DualNum {
        return DualNum::from_groups(
            /* scalar, e1234 */
            float2(self[scalar]) * other.group0()
        );
    }
}
extension Scalar: Wedge<Flector> {
    func wedge(self, other: Flector) -> Flector {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(self[scalar]) * other.group0(), 
            /* e423, e431, e412, e321 */
            float4(self[scalar]) * other.group1()
        );
    }
}
extension Scalar: Wedge<Horizon> {
    func wedge(self, other: Horizon) -> Horizon {
        return Horizon::from_groups(
            /* e321 */
            other[e321] * self[scalar]
        );
    }
}
extension Scalar: Wedge<Line> {
    func wedge(self, other: Line) -> Line {
        return Line::from_groups(
            /* e41, e42, e43 */
            float3(self[scalar]) * other.group0(), 
            /* e23, e31, e12 */
            float3(self[scalar]) * other.group1()
        );
    }
}
extension Scalar: Wedge<Motor> {
    func wedge(self, other: Motor) -> Motor {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(self[scalar]) * other.group0(), 
            /* e23, e31, e12, scalar */
            float4(self[scalar]) * other.group1()
        );
    }
}
extension Scalar: Wedge<MultiVector> {
    func wedge(self, other: MultiVector) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(self[scalar]) * other.group0(), 
            /* e1, e2, e3, e4 */
            float4(self[scalar]) * other.group1(), 
            /* e41, e42, e43 */
            float3(self[scalar]) * other.group2(), 
            /* e23, e31, e12 */
            float3(self[scalar]) * other.group3(), 
            /* e423, e431, e412, e321 */
            float4(self[scalar]) * other.group4()
        );
    }
}
extension Scalar: Wedge<Origin> {
    func wedge(self, other: Origin) -> Origin {
        return Origin::from_groups(
            /* e4 */
            other[e4] * self[scalar]
        );
    }
}
extension Scalar: Wedge<Plane> {
    func wedge(self, other: Plane) -> Plane {
        return Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4(self[scalar]) * other.group0()
        );
    }
}
extension Scalar: Wedge<Point> {
    func wedge(self, other: Point) -> Point {
        return Point::from_groups(
            /* e1, e2, e3, e4 */
            float4(self[scalar]) * other.group0()
        );
    }
}
extension Scalar: Wedge<Scalar> {
    func wedge(self, other: Scalar) -> Scalar {
        return Scalar::from_groups(
            /* scalar */
            other[scalar] * self[scalar]
        );
    }
}
