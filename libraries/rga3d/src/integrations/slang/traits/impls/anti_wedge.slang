extension AntiScalar {
    func div(_rhs: anti_wedge) -> anti_wedge_partial<AntiScalar> {
        anti_wedge_partial(this)
    }
}
extension AntiScalar: AntiWedge<AntiScalar> {
    func anti_wedge(self, other: AntiScalar) -> AntiScalar {
        return AntiScalar::from_groups(
            /* e1234 */
            other[e1234] * self[e1234]
        );
    }
}
extension AntiScalar: AntiWedge<DualNum> {
    func anti_wedge(self, other: DualNum) -> DualNum {
        return DualNum::from_groups(
            /* scalar, e1234 */
            float2(self[e1234]) * other.group0()
        );
    }
}
extension AntiScalar: AntiWedge<Flector> {
    func anti_wedge(self, other: Flector) -> Flector {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(self[e1234]) * other.group0(), 
            /* e423, e431, e412, e321 */
            float4(self[e1234]) * other.group1()
        );
    }
}
extension AntiScalar: AntiWedge<Horizon> {
    func anti_wedge(self, other: Horizon) -> Horizon {
        return Horizon::from_groups(
            /* e321 */
            self[e1234] * other[e321]
        );
    }
}
extension AntiScalar: AntiWedge<Line> {
    func anti_wedge(self, other: Line) -> Line {
        return Line::from_groups(
            /* e41, e42, e43 */
            float3(self[e1234]) * other.group0(), 
            /* e23, e31, e12 */
            float3(self[e1234]) * other.group1()
        );
    }
}
extension AntiScalar: AntiWedge<Motor> {
    func anti_wedge(self, other: Motor) -> Motor {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(self[e1234]) * other.group0(), 
            /* e23, e31, e12, scalar */
            float4(self[e1234]) * other.group1()
        );
    }
}
extension AntiScalar: AntiWedge<MultiVector> {
    func anti_wedge(self, other: MultiVector) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(self[e1234]) * other.group0(), 
            /* e1, e2, e3, e4 */
            float4(self[e1234]) * other.group1(), 
            /* e41, e42, e43 */
            float3(self[e1234]) * other.group2(), 
            /* e23, e31, e12 */
            float3(self[e1234]) * other.group3(), 
            /* e423, e431, e412, e321 */
            float4(self[e1234]) * other.group4()
        );
    }
}
extension AntiScalar: AntiWedge<Origin> {
    func anti_wedge(self, other: Origin) -> Origin {
        return Origin::from_groups(
            /* e4 */
            self[e1234] * other[e4]
        );
    }
}
extension AntiScalar: AntiWedge<Plane> {
    func anti_wedge(self, other: Plane) -> Plane {
        return Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4(self[e1234]) * other.group0()
        );
    }
}
extension AntiScalar: AntiWedge<Point> {
    func anti_wedge(self, other: Point) -> Point {
        return Point::from_groups(
            /* e1, e2, e3, e4 */
            float4(self[e1234]) * other.group0()
        );
    }
}
extension AntiScalar: AntiWedge<Scalar> {
    func anti_wedge(self, other: Scalar) -> Scalar {
        return Scalar::from_groups(
            /* scalar */
            self[e1234] * other[scalar]
        );
    }
}
extension DualNum {
    func div(_rhs: anti_wedge) -> anti_wedge_partial<DualNum> {
        anti_wedge_partial(this)
    }
}
extension DualNum: AntiWedge<AntiScalar> {
    func anti_wedge(self, other: AntiScalar) -> DualNum {
        return DualNum::from_groups(
            /* scalar, e1234 */
            float2(other[e1234]) * self.group0()
        );
    }
}
extension DualNum: AntiWedge<DualNum> {
    func anti_wedge(self, other: DualNum) -> DualNum {
        return DualNum::from_groups(
            /* scalar, e1234 */
            float2((other[scalar] * self[e1234]) + (other[e1234] * self[scalar]), other[e1234] * self[e1234])
        );
    }
}
extension DualNum: AntiWedge<Flector> {
    func anti_wedge(self, other: Flector) -> Flector {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(self[e1234]) * other.group0(), 
            /* e423, e431, e412, e321 */
            float4(self[e1234]) * other.group1()
        );
    }
}
extension DualNum: AntiWedge<Horizon> {
    func anti_wedge(self, other: Horizon) -> Horizon {
        return Horizon::from_groups(
            /* e321 */
            self[e1234] * other[e321]
        );
    }
}
extension DualNum: AntiWedge<Line> {
    func anti_wedge(self, other: Line) -> Line {
        return Line::from_groups(
            /* e41, e42, e43 */
            float3(self[e1234]) * other.group0(), 
            /* e23, e31, e12 */
            float3(self[e1234]) * other.group1()
        );
    }
}
extension DualNum: AntiWedge<Motor> {
    func anti_wedge(self, other: Motor) -> Motor {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(self[e1234]) * other.group0(), 
            /* e23, e31, e12, scalar */
            float4(other[e23], other[e31], other[e12], 1.0) * float4(self.group0().yy, self[e1234], (self[scalar] * other[e1234]) + (self[e1234] * other[scalar]))
        );
    }
}
extension DualNum: AntiWedge<MultiVector> {
    func anti_wedge(self, other: MultiVector) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2((self[scalar] * other[e1234]) + (self[e1234] * other[scalar]), self[e1234] * other[e1234]), 
            /* e1, e2, e3, e4 */
            float4(self[e1234]) * other.group1(), 
            /* e41, e42, e43 */
            float3(self[e1234]) * other.group2(), 
            /* e23, e31, e12 */
            float3(self[e1234]) * other.group3(), 
            /* e423, e431, e412, e321 */
            float4(self[e1234]) * other.group4()
        );
    }
}
extension DualNum: AntiWedge<Origin> {
    func anti_wedge(self, other: Origin) -> Origin {
        return Origin::from_groups(
            /* e4 */
            self[e1234] * other[e4]
        );
    }
}
extension DualNum: AntiWedge<Plane> {
    func anti_wedge(self, other: Plane) -> Plane {
        return Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4(self[e1234]) * other.group0()
        );
    }
}
extension DualNum: AntiWedge<Point> {
    func anti_wedge(self, other: Point) -> Point {
        return Point::from_groups(
            /* e1, e2, e3, e4 */
            float4(self[e1234]) * other.group0()
        );
    }
}
extension DualNum: AntiWedge<Scalar> {
    func anti_wedge(self, other: Scalar) -> Scalar {
        return Scalar::from_groups(
            /* scalar */
            self[e1234] * other[scalar]
        );
    }
}
extension Flector {
    func div(_rhs: anti_wedge) -> anti_wedge_partial<Flector> {
        anti_wedge_partial(this)
    }
}
extension Flector: AntiWedge<AntiScalar> {
    func anti_wedge(self, other: AntiScalar) -> Flector {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(other[e1234]) * self.group0(), 
            /* e423, e431, e412, e321 */
            float4(other[e1234]) * self.group1()
        );
    }
}
extension Flector: AntiWedge<DualNum> {
    func anti_wedge(self, other: DualNum) -> Flector {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(other[e1234]) * self.group0(), 
            /* e423, e431, e412, e321 */
            float4(other[e1234]) * self.group1()
        );
    }
}
extension Flector: AntiWedge<Flector> {
    func anti_wedge(self, other: Flector) -> Motor {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(((other.group1().yzx * self.group1().zxy) - (other.group1().zxy * self.group1().yzx)), 0.0), 
            /* e23, e31, e12, scalar */
            (other.group1().wwwx * float4(self.group1().xyz, self[e1])) + float4(float3(0.0), (other[e431] * self[e2]) + (other[e412] * self[e3]) + (other[e321] * self[e4]) - (other[e2] * self[e431]) - (other[e3] * self[e412]) - (other[e4] * self[e321])) - (self.group1().wwwx * float4(other.group1().xyz, other[e1]))
        );
    }
}
extension Flector: AntiWedge<Horizon> {
    func anti_wedge(self, other: Horizon) -> Motor {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(0.0), 
            /* e23, e31, e12, scalar */
            float4(other[e321]) * float4(self.group1().xyz, self[e4])
        );
    }
}
extension Flector: AntiWedge<Line> {
    func anti_wedge(self, other: Line) -> Point {
        return Point::from_groups(
            /* e1, e2, e3, e4 */
            float4((self[e412] * other[e31]) + (self[e321] * other[e41]), (self[e423] * other[e12]) + (self[e321] * other[e42]), (self[e431] * other[e23]) + (self[e321] * other[e43]), -(self[e431] * other[e42]) - (self[e412] * other[e43])) - (self.group1().yzxx * float4(other.group1().zxy, other[e41]))
        );
    }
}
extension Flector: AntiWedge<Motor> {
    func anti_wedge(self, other: Motor) -> Flector {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4((self[e412] * other[e31]) + (self[e321] * other[e41]), (self[e423] * other[e12]) + (self[e321] * other[e42]), (self[e431] * other[e23]) + (self[e321] * other[e43]), -(self[e431] * other[e42]) - (self[e412] * other[e43])) + (float4(other[e1234]) * self.group0()) - (self.group1().yzxx * float4(other.group1().zxy, other[e41])), 
            /* e423, e431, e412, e321 */
            float4(other[e1234]) * self.group1()
        );
    }
}
extension Flector: AntiWedge<MultiVector> {
    func anti_wedge(self, other: MultiVector) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2((self[e1] * other[e423]) + (self[e2] * other[e431]) + (self[e3] * other[e412]) + (self[e4] * other[e321]) - (self[e423] * other[e1]) - (self[e431] * other[e2]) - (self[e412] * other[e3]) - (self[e321] * other[e4]), 0.0), 
            /* e1, e2, e3, e4 */
            float4((self[e412] * other[e31]) + (self[e321] * other[e41]), (self[e423] * other[e12]) + (self[e321] * other[e42]), (self[e431] * other[e23]) + (self[e321] * other[e43]), -(self[e431] * other[e42]) - (self[e412] * other[e43])) + (float4(other[e1234]) * self.group0()) - (self.group1().yzxx * float4(other.group3().zxy, other[e41])), 
            /* e41, e42, e43 */
            (self.group1().zxy * other.group4().yzx) - (self.group1().yzx * other.group4().zxy), 
            /* e23, e31, e12 */
            (float3(other[e321]) * self.group1().xyz) - (float3(self[e321]) * other.group4().xyz), 
            /* e423, e431, e412, e321 */
            float4(other[e1234]) * self.group1()
        );
    }
}
extension Flector: AntiWedge<Origin> {
    func anti_wedge(self, other: Origin) -> Scalar {
        return Scalar::from_groups(
            /* scalar */
            self[e321] * other[e4] * -1.0
        );
    }
}
extension Flector: AntiWedge<Plane> {
    func anti_wedge(self, other: Plane) -> Motor {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(((self.group1().zxy * other.group0().yzx) - (self.group1().yzx * other.group0().zxy)), 0.0), 
            /* e23, e31, e12, scalar */
            float4(self[e321] * other[e423] * -1.0, self[e321] * other[e431] * -1.0, self[e321] * other[e412] * -1.0, (self[e2] * other[e431]) + (self[e3] * other[e412]) + (self[e4] * other[e321])) + (other.group0().wwwx * float4(self.group1().xyz, self[e1]))
        );
    }
}
extension Flector: AntiWedge<Point> {
    func anti_wedge(self, other: Point) -> Scalar {
        return Scalar::from_groups(
            /* scalar */
            -(self[e423] * other[e1]) - (self[e431] * other[e2]) - (self[e412] * other[e3]) - (self[e321] * other[e4])
        );
    }
}
extension Horizon {
    func div(_rhs: anti_wedge) -> anti_wedge_partial<Horizon> {
        anti_wedge_partial(this)
    }
}
extension Horizon: AntiWedge<AntiScalar> {
    func anti_wedge(self, other: AntiScalar) -> Horizon {
        return Horizon::from_groups(
            /* e321 */
            other[e1234] * self[e321]
        );
    }
}
extension Horizon: AntiWedge<DualNum> {
    func anti_wedge(self, other: DualNum) -> Horizon {
        return Horizon::from_groups(
            /* e321 */
            other[e1234] * self[e321]
        );
    }
}
extension Horizon: AntiWedge<Flector> {
    func anti_wedge(self, other: Flector) -> Motor {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(0.0), 
            /* e23, e31, e12, scalar */
            float4(self[e321]) * float4(other.group1().xyz, other[e4]) * float4(-1.0)
        );
    }
}
extension Horizon: AntiWedge<Line> {
    func anti_wedge(self, other: Line) -> Point {
        return Point::from_groups(
            /* e1, e2, e3, e4 */
            float4(self[e321], self[e321], self[e321], 0.0) * float4(other.group0(), 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Horizon: AntiWedge<Motor> {
    func anti_wedge(self, other: Motor) -> Flector {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(self[e321], self[e321], self[e321], 0.0) * float4(other.group0().xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), self[e321] * other[e1234])
        );
    }
}
extension Horizon: AntiWedge<MultiVector> {
    func anti_wedge(self, other: MultiVector) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(self[e321] * other[e4], 1.0) * float2(-1.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(self[e321], self[e321], self[e321], 0.0) * float4(other.group2(), 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(self[e321]) * other.group4().xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), self[e321] * other[e1234])
        );
    }
}
extension Horizon: AntiWedge<Origin> {
    func anti_wedge(self, other: Origin) -> Scalar {
        return Scalar::from_groups(
            /* scalar */
            self[e321] * other[e4] * -1.0
        );
    }
}
extension Horizon: AntiWedge<Plane> {
    func anti_wedge(self, other: Plane) -> Line {
        return Line::from_groups(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(self[e321]) * other.group0().xyz * float3(-1.0)
        );
    }
}
extension Horizon: AntiWedge<Point> {
    func anti_wedge(self, other: Point) -> Scalar {
        return Scalar::from_groups(
            /* scalar */
            self[e321] * other[e4] * -1.0
        );
    }
}
extension Line {
    func div(_rhs: anti_wedge) -> anti_wedge_partial<Line> {
        anti_wedge_partial(this)
    }
}
extension Line: AntiWedge<AntiScalar> {
    func anti_wedge(self, other: AntiScalar) -> Line {
        return Line::from_groups(
            /* e41, e42, e43 */
            float3(other[e1234]) * self.group0(), 
            /* e23, e31, e12 */
            float3(other[e1234]) * self.group1()
        );
    }
}
extension Line: AntiWedge<DualNum> {
    func anti_wedge(self, other: DualNum) -> Line {
        return Line::from_groups(
            /* e41, e42, e43 */
            float3(other[e1234]) * self.group0(), 
            /* e23, e31, e12 */
            float3(other[e1234]) * self.group1()
        );
    }
}
extension Line: AntiWedge<Flector> {
    func anti_wedge(self, other: Flector) -> Point {
        return Point::from_groups(
            /* e1, e2, e3, e4 */
            float4((other[e412] * self[e31]) + (other[e321] * self[e41]), (other[e423] * self[e12]) + (other[e321] * self[e42]), (other[e431] * self[e23]) + (other[e321] * self[e43]), -(other[e431] * self[e42]) - (other[e412] * self[e43])) - (other.group1().yzxx * float4(self.group1().zxy, self[e41]))
        );
    }
}
extension Line: AntiWedge<Horizon> {
    func anti_wedge(self, other: Horizon) -> Point {
        return Point::from_groups(
            /* e1, e2, e3, e4 */
            float4(other[e321], other[e321], other[e321], 0.0) * float4(self.group0(), 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Line: AntiWedge<Line> {
    func anti_wedge(self, other: Line) -> Scalar {
        return Scalar::from_groups(
            /* scalar */
            -(other[e41] * self[e23]) - (other[e42] * self[e31]) - (other[e43] * self[e12]) - (other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43])
        );
    }
}
extension Line: AntiWedge<Motor> {
    func anti_wedge(self, other: Motor) -> Motor {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(float3(1.0), 0.0) * float4(self.group0(), 0.0) * float4(other.group0().www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e23, e31, e12, scalar */
            float4(other[e1234], other[e1234], other[e1234], 1.0) * float4(self.group1(), -(self[e41] * other[e23]) - (self[e42] * other[e31]) - (self[e43] * other[e12]) - (self[e23] * other[e41]) - (self[e31] * other[e42]) - (self[e12] * other[e43]))
        );
    }
}
extension Line: AntiWedge<MultiVector> {
    func anti_wedge(self, other: MultiVector) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(-(self[e41] * other[e23]) - (self[e42] * other[e31]) - (self[e43] * other[e12]) - (self[e23] * other[e41]) - (self[e31] * other[e42]) - (self[e12] * other[e43]), 0.0), 
            /* e1, e2, e3, e4 */
            float4((self[e41] * other[e321]) + (self[e31] * other[e412]), (self[e42] * other[e321]) + (self[e12] * other[e423]), (self[e43] * other[e321]) + (self[e23] * other[e431]), -(self[e42] * other[e431]) - (self[e43] * other[e412])) - (other.group4().yzxx * float4(self.group1().zxy, self[e41])), 
            /* e41, e42, e43 */
            float3(other[e1234]) * self.group0(), 
            /* e23, e31, e12 */
            float3(other[e1234]) * self.group1(), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
extension Line: AntiWedge<Plane> {
    func anti_wedge(self, other: Plane) -> Point {
        return Point::from_groups(
            /* e1, e2, e3, e4 */
            float4((self[e41] * other[e321]) + (self[e31] * other[e412]), (self[e42] * other[e321]) + (self[e12] * other[e423]), (self[e43] * other[e321]) + (self[e23] * other[e431]), -(self[e42] * other[e431]) - (self[e43] * other[e412])) - (other.group0().yzxx * float4(self.group1().zxy, self[e41]))
        );
    }
}
extension Motor {
    func div(_rhs: anti_wedge) -> anti_wedge_partial<Motor> {
        anti_wedge_partial(this)
    }
}
extension Motor: AntiWedge<AntiScalar> {
    func anti_wedge(self, other: AntiScalar) -> Motor {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(other[e1234]) * self.group0(), 
            /* e23, e31, e12, scalar */
            float4(other[e1234]) * self.group1()
        );
    }
}
extension Motor: AntiWedge<DualNum> {
    func anti_wedge(self, other: DualNum) -> Motor {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(other[e1234]) * self.group0(), 
            /* e23, e31, e12, scalar */
            float4(self[e23], self[e31], self[e12], 1.0) * float4(other.group0().yy, other[e1234], (other[scalar] * self[e1234]) + (other[e1234] * self[scalar]))
        );
    }
}
extension Motor: AntiWedge<Flector> {
    func anti_wedge(self, other: Flector) -> Flector {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4((other[e412] * self[e31]) + (other[e321] * self[e41]), (other[e423] * self[e12]) + (other[e321] * self[e42]), (other[e431] * self[e23]) + (other[e321] * self[e43]), -(other[e431] * self[e42]) - (other[e412] * self[e43])) + (float4(self[e1234]) * other.group0()) - (other.group1().yzxx * float4(self.group1().zxy, self[e41])), 
            /* e423, e431, e412, e321 */
            float4(self[e1234]) * other.group1()
        );
    }
}
extension Motor: AntiWedge<Horizon> {
    func anti_wedge(self, other: Horizon) -> Flector {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(other[e321], other[e321], other[e321], 0.0) * float4(self.group0().xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other[e321] * self[e1234])
        );
    }
}
extension Motor: AntiWedge<Line> {
    func anti_wedge(self, other: Line) -> Motor {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(float3(1.0), 0.0) * float4(other.group0(), 0.0) * float4(self.group0().www, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e23, e31, e12, scalar */
            float4(self[e1234], self[e1234], self[e1234], 1.0) * float4(other.group1(), -(other[e41] * self[e23]) - (other[e42] * self[e31]) - (other[e43] * self[e12]) - (other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43]))
        );
    }
}
extension Motor: AntiWedge<Motor> {
    func anti_wedge(self, other: Motor) -> Motor {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(((float3(other[e1234]) * self.group0().xyz) + (float3(self[e1234]) * other.group0().xyz)), other[e1234] * self[e1234]), 
            /* e23, e31, e12, scalar */
            (float4(other[e1234]) * self.group1()) + (float4(self[e1234]) * other.group1()) + float4(float3(0.0), -(other[e41] * self[e23]) - (other[e42] * self[e31]) - (other[e43] * self[e12]) - (other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43]))
        );
    }
}
extension Motor: AntiWedge<MultiVector> {
    func anti_wedge(self, other: MultiVector) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2((self[e1234] * other[scalar]) + (self[scalar] * other[e1234]) - (self[e41] * other[e23]) - (self[e42] * other[e31]) - (self[e43] * other[e12]) - (self[e23] * other[e41]) - (self[e31] * other[e42]) - (self[e12] * other[e43]), self[e1234] * other[e1234]), 
            /* e1, e2, e3, e4 */
            float4((self[e1234] * other[e1]) + (self[e31] * other[e412]), (self[e1234] * other[e2]) + (self[e12] * other[e423]), (self[e1234] * other[e3]) + (self[e23] * other[e431]), -(self[e42] * other[e431]) - (self[e43] * other[e412])) + (self.group0() * float4(other.group4().www, other[e4])) - (other.group4().yzxx * float4(self.group1().zxy, self[e41])), 
            /* e41, e42, e43 */
            (float3(self[e1234]) * other.group2()) + (float3(other[e1234]) * self.group0().xyz), 
            /* e23, e31, e12 */
            (float3(self[e1234]) * other.group3()) + (float3(other[e1234]) * self.group1().xyz), 
            /* e423, e431, e412, e321 */
            float4(self[e1234]) * other.group4()
        );
    }
}
extension Motor: AntiWedge<Origin> {
    func anti_wedge(self, other: Origin) -> Origin {
        return Origin::from_groups(
            /* e4 */
            self[e1234] * other[e4]
        );
    }
}
extension Motor: AntiWedge<Plane> {
    func anti_wedge(self, other: Plane) -> Flector {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4((self[e41] * other[e321]) + (self[e31] * other[e412]), (self[e42] * other[e321]) + (self[e12] * other[e423]), (self[e43] * other[e321]) + (self[e23] * other[e431]), -(self[e42] * other[e431]) - (self[e43] * other[e412])) - (other.group0().yzxx * float4(self.group1().zxy, self[e41])), 
            /* e423, e431, e412, e321 */
            float4(self[e1234]) * other.group0()
        );
    }
}
extension Motor: AntiWedge<Point> {
    func anti_wedge(self, other: Point) -> Point {
        return Point::from_groups(
            /* e1, e2, e3, e4 */
            float4(self[e1234]) * other.group0()
        );
    }
}
extension Motor: AntiWedge<Scalar> {
    func anti_wedge(self, other: Scalar) -> Scalar {
        return Scalar::from_groups(
            /* scalar */
            self[e1234] * other[scalar]
        );
    }
}
extension MultiVector {
    func div(_rhs: anti_wedge) -> anti_wedge_partial<MultiVector> {
        anti_wedge_partial(this)
    }
}
extension MultiVector: AntiWedge<AntiScalar> {
    func anti_wedge(self, other: AntiScalar) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(other[e1234]) * self.group0(), 
            /* e1, e2, e3, e4 */
            float4(other[e1234]) * self.group1(), 
            /* e41, e42, e43 */
            float3(other[e1234]) * self.group2(), 
            /* e23, e31, e12 */
            float3(other[e1234]) * self.group3(), 
            /* e423, e431, e412, e321 */
            float4(other[e1234]) * self.group4()
        );
    }
}
extension MultiVector: AntiWedge<DualNum> {
    func anti_wedge(self, other: DualNum) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2((other[scalar] * self[e1234]) + (other[e1234] * self[scalar]), other[e1234] * self[e1234]), 
            /* e1, e2, e3, e4 */
            float4(other[e1234]) * self.group1(), 
            /* e41, e42, e43 */
            float3(other[e1234]) * self.group2(), 
            /* e23, e31, e12 */
            float3(other[e1234]) * self.group3(), 
            /* e423, e431, e412, e321 */
            float4(other[e1234]) * self.group4()
        );
    }
}
extension MultiVector: AntiWedge<Flector> {
    func anti_wedge(self, other: Flector) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2((other[e423] * self[e1]) + (other[e431] * self[e2]) + (other[e412] * self[e3]) + (other[e321] * self[e4]) - (other[e1] * self[e423]) - (other[e2] * self[e431]) - (other[e3] * self[e412]) - (other[e4] * self[e321]), 0.0), 
            /* e1, e2, e3, e4 */
            float4((other[e412] * self[e31]) + (other[e321] * self[e41]), (other[e423] * self[e12]) + (other[e321] * self[e42]), (other[e431] * self[e23]) + (other[e321] * self[e43]), -(other[e431] * self[e42]) - (other[e412] * self[e43])) + (float4(self[e1234]) * other.group0()) - (other.group1().yzxx * float4(self.group3().zxy, self[e41])), 
            /* e41, e42, e43 */
            (other.group1().yzx * self.group4().zxy) - (other.group1().zxy * self.group4().yzx), 
            /* e23, e31, e12 */
            (float3(other[e321]) * self.group4().xyz) - (float3(self[e321]) * other.group1().xyz), 
            /* e423, e431, e412, e321 */
            float4(self[e1234]) * other.group1()
        );
    }
}
extension MultiVector: AntiWedge<Horizon> {
    func anti_wedge(self, other: Horizon) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(other[e321] * self[e4], 1.0) * float2(1.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(other[e321], other[e321], other[e321], 0.0) * float4(self.group2(), 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(other[e321]) * self.group4().xyz, 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other[e321] * self[e1234])
        );
    }
}
extension MultiVector: AntiWedge<Line> {
    func anti_wedge(self, other: Line) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(-(other[e41] * self[e23]) - (other[e42] * self[e31]) - (other[e43] * self[e12]) - (other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43]), 0.0), 
            /* e1, e2, e3, e4 */
            float4((other[e41] * self[e321]) + (other[e31] * self[e412]), (other[e42] * self[e321]) + (other[e12] * self[e423]), (other[e43] * self[e321]) + (other[e23] * self[e431]), -(other[e42] * self[e431]) - (other[e43] * self[e412])) - (self.group4().yzxx * float4(other.group1().zxy, other[e41])), 
            /* e41, e42, e43 */
            float3(self[e1234]) * other.group0(), 
            /* e23, e31, e12 */
            float3(self[e1234]) * other.group1(), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
extension MultiVector: AntiWedge<Motor> {
    func anti_wedge(self, other: Motor) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2((other[e1234] * self[scalar]) + (other[scalar] * self[e1234]) - (other[e41] * self[e23]) - (other[e42] * self[e31]) - (other[e43] * self[e12]) - (other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43]), other[e1234] * self[e1234]), 
            /* e1, e2, e3, e4 */
            float4((other[e1234] * self[e1]) + (other[e31] * self[e412]), (other[e1234] * self[e2]) + (other[e12] * self[e423]), (other[e1234] * self[e3]) + (other[e23] * self[e431]), -(other[e42] * self[e431]) - (other[e43] * self[e412])) + (other.group0() * float4(self.group4().www, self[e4])) - (self.group4().yzxx * float4(other.group1().zxy, other[e41])), 
            /* e41, e42, e43 */
            (float3(other[e1234]) * self.group2()) + (float3(self[e1234]) * other.group0().xyz), 
            /* e23, e31, e12 */
            (float3(other[e1234]) * self.group3()) + (float3(self[e1234]) * other.group1().xyz), 
            /* e423, e431, e412, e321 */
            float4(other[e1234]) * self.group4()
        );
    }
}
extension MultiVector: AntiWedge<MultiVector> {
    func anti_wedge(self, other: MultiVector) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2((other[scalar] * self[e1234]) + (other[e1234] * self[scalar]) + (other[e423] * self[e1]) + (other[e431] * self[e2]) + (other[e412] * self[e3]) + (other[e321] * self[e4]) - (other[e1] * self[e423]) - (other[e2] * self[e431]) - (other[e3] * self[e412]) - (other[e4] * self[e321]) - (other[e41] * self[e23]) - (other[e42] * self[e31]) - (other[e43] * self[e12]) - (other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43]), other[e1234] * self[e1234]), 
            /* e1, e2, e3, e4 */
            float4((other[e41] * self[e321]) + (other[e31] * self[e412]) + (other[e412] * self[e31]) + (other[e321] * self[e41]), (other[e42] * self[e321]) + (other[e12] * self[e423]) + (other[e423] * self[e12]) + (other[e321] * self[e42]), (other[e43] * self[e321]) + (other[e23] * self[e431]) + (other[e431] * self[e23]) + (other[e321] * self[e43]), -(other[e43] * self[e412]) - (other[e423] * self[e41]) - (other[e431] * self[e42]) - (other[e412] * self[e43])) + (float4(other[e1234]) * self.group1()) + (float4(self[e1234]) * other.group1()) - (self.group4().yzxx * float4(other.group3().zxy, other[e41])) - float4((self.group3().zxy * other.group4().yzx), other[e42] * self[e431]), 
            /* e41, e42, e43 */
            (float3(other[e1234]) * self.group2()) + (float3(self[e1234]) * other.group2()) + (other.group4().yzx * self.group4().zxy) - (other.group4().zxy * self.group4().yzx), 
            /* e23, e31, e12 */
            (float3(other[e1234]) * self.group3()) + (float3(other[e321]) * self.group4().xyz) + (float3(self[e1234]) * other.group3()) - (float3(self[e321]) * other.group4().xyz), 
            /* e423, e431, e412, e321 */
            (float4(other[e1234]) * self.group4()) + (float4(self[e1234]) * other.group4())
        );
    }
}
extension MultiVector: AntiWedge<Origin> {
    func anti_wedge(self, other: Origin) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(self[e321] * other[e4], 1.0) * float2(-1.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), self[e1234] * other[e4]), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
extension MultiVector: AntiWedge<Plane> {
    func anti_wedge(self, other: Plane) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2((self[e1] * other[e423]) + (self[e2] * other[e431]) + (self[e3] * other[e412]) + (self[e4] * other[e321]), 0.0), 
            /* e1, e2, e3, e4 */
            float4((self[e41] * other[e321]) + (self[e31] * other[e412]), (self[e42] * other[e321]) + (self[e12] * other[e423]), (self[e43] * other[e321]) + (self[e23] * other[e431]), -(self[e42] * other[e431]) - (self[e43] * other[e412])) - (other.group0().yzxx * float4(self.group3().zxy, self[e41])), 
            /* e41, e42, e43 */
            (self.group4().zxy * other.group0().yzx) - (self.group4().yzx * other.group0().zxy), 
            /* e23, e31, e12 */
            (float3(other[e321]) * self.group4().xyz) - (float3(self[e321]) * other.group0().xyz), 
            /* e423, e431, e412, e321 */
            float4(self[e1234]) * other.group0()
        );
    }
}
extension MultiVector: AntiWedge<Point> {
    func anti_wedge(self, other: Point) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(-(self[e423] * other[e1]) - (self[e431] * other[e2]) - (self[e412] * other[e3]) - (self[e321] * other[e4]), 0.0), 
            /* e1, e2, e3, e4 */
            float4(self[e1234]) * other.group0(), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
extension MultiVector: AntiWedge<Scalar> {
    func anti_wedge(self, other: Scalar) -> Scalar {
        return Scalar::from_groups(
            /* scalar */
            self[e1234] * other[scalar]
        );
    }
}
extension Origin {
    func div(_rhs: anti_wedge) -> anti_wedge_partial<Origin> {
        anti_wedge_partial(this)
    }
}
extension Origin: AntiWedge<AntiScalar> {
    func anti_wedge(self, other: AntiScalar) -> Origin {
        return Origin::from_groups(
            /* e4 */
            other[e1234] * self[e4]
        );
    }
}
extension Origin: AntiWedge<DualNum> {
    func anti_wedge(self, other: DualNum) -> Origin {
        return Origin::from_groups(
            /* e4 */
            other[e1234] * self[e4]
        );
    }
}
extension Origin: AntiWedge<Flector> {
    func anti_wedge(self, other: Flector) -> Scalar {
        return Scalar::from_groups(
            /* scalar */
            other[e321] * self[e4]
        );
    }
}
extension Origin: AntiWedge<Horizon> {
    func anti_wedge(self, other: Horizon) -> Scalar {
        return Scalar::from_groups(
            /* scalar */
            other[e321] * self[e4]
        );
    }
}
extension Origin: AntiWedge<Motor> {
    func anti_wedge(self, other: Motor) -> Origin {
        return Origin::from_groups(
            /* e4 */
            other[e1234] * self[e4]
        );
    }
}
extension Origin: AntiWedge<MultiVector> {
    func anti_wedge(self, other: MultiVector) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(other[e321] * self[e4], 1.0) * float2(1.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other[e1234] * self[e4]), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
extension Origin: AntiWedge<Plane> {
    func anti_wedge(self, other: Plane) -> Scalar {
        return Scalar::from_groups(
            /* scalar */
            self[e4] * other[e321]
        );
    }
}
extension Plane {
    func div(_rhs: anti_wedge) -> anti_wedge_partial<Plane> {
        anti_wedge_partial(this)
    }
}
extension Plane: AntiWedge<AntiScalar> {
    func anti_wedge(self, other: AntiScalar) -> Plane {
        return Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4(other[e1234]) * self.group0()
        );
    }
}
extension Plane: AntiWedge<DualNum> {
    func anti_wedge(self, other: DualNum) -> Plane {
        return Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4(other[e1234]) * self.group0()
        );
    }
}
extension Plane: AntiWedge<Flector> {
    func anti_wedge(self, other: Flector) -> Motor {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(((other.group1().yzx * self.group0().zxy) - (other.group1().zxy * self.group0().yzx)), 0.0), 
            /* e23, e31, e12, scalar */
            float4(other[e321] * self[e423], other[e321] * self[e431], other[e321] * self[e412], -(other[e2] * self[e431]) - (other[e3] * self[e412]) - (other[e4] * self[e321])) - (self.group0().wwwx * float4(other.group1().xyz, other[e1]))
        );
    }
}
extension Plane: AntiWedge<Horizon> {
    func anti_wedge(self, other: Horizon) -> Line {
        return Line::from_groups(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(other[e321]) * self.group0().xyz
        );
    }
}
extension Plane: AntiWedge<Line> {
    func anti_wedge(self, other: Line) -> Point {
        return Point::from_groups(
            /* e1, e2, e3, e4 */
            float4((other[e41] * self[e321]) + (other[e31] * self[e412]), (other[e42] * self[e321]) + (other[e12] * self[e423]), (other[e43] * self[e321]) + (other[e23] * self[e431]), -(other[e42] * self[e431]) - (other[e43] * self[e412])) - (self.group0().yzxx * float4(other.group1().zxy, other[e41]))
        );
    }
}
extension Plane: AntiWedge<Motor> {
    func anti_wedge(self, other: Motor) -> Flector {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4((other[e41] * self[e321]) + (other[e31] * self[e412]), (other[e42] * self[e321]) + (other[e12] * self[e423]), (other[e43] * self[e321]) + (other[e23] * self[e431]), -(other[e42] * self[e431]) - (other[e43] * self[e412])) - (self.group0().yzxx * float4(other.group1().zxy, other[e41])), 
            /* e423, e431, e412, e321 */
            float4(other[e1234]) * self.group0()
        );
    }
}
extension Plane: AntiWedge<MultiVector> {
    func anti_wedge(self, other: MultiVector) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(-(other[e1] * self[e423]) - (other[e2] * self[e431]) - (other[e3] * self[e412]) - (other[e4] * self[e321]), 0.0), 
            /* e1, e2, e3, e4 */
            float4((other[e41] * self[e321]) + (other[e31] * self[e412]), (other[e42] * self[e321]) + (other[e12] * self[e423]), (other[e43] * self[e321]) + (other[e23] * self[e431]), -(other[e42] * self[e431]) - (other[e43] * self[e412])) - (self.group0().yzxx * float4(other.group3().zxy, other[e41])), 
            /* e41, e42, e43 */
            (other.group4().yzx * self.group0().zxy) - (other.group4().zxy * self.group0().yzx), 
            /* e23, e31, e12 */
            (float3(other[e321]) * self.group0().xyz) - (float3(self[e321]) * other.group4().xyz), 
            /* e423, e431, e412, e321 */
            float4(other[e1234]) * self.group0()
        );
    }
}
extension Plane: AntiWedge<Origin> {
    func anti_wedge(self, other: Origin) -> Scalar {
        return Scalar::from_groups(
            /* scalar */
            other[e4] * self[e321] * -1.0
        );
    }
}
extension Plane: AntiWedge<Plane> {
    func anti_wedge(self, other: Plane) -> Line {
        return Line::from_groups(
            /* e41, e42, e43 */
            (other.group0().yzx * self.group0().zxy) - (other.group0().zxy * self.group0().yzx), 
            /* e23, e31, e12 */
            (float3(other[e321]) * self.group0().xyz) - (float3(self[e321]) * other.group0().xyz)
        );
    }
}
extension Plane: AntiWedge<Point> {
    func anti_wedge(self, other: Point) -> Scalar {
        return Scalar::from_groups(
            /* scalar */
            -(self[e423] * other[e1]) - (self[e431] * other[e2]) - (self[e412] * other[e3]) - (self[e321] * other[e4])
        );
    }
}
extension Point {
    func div(_rhs: anti_wedge) -> anti_wedge_partial<Point> {
        anti_wedge_partial(this)
    }
}
extension Point: AntiWedge<AntiScalar> {
    func anti_wedge(self, other: AntiScalar) -> Point {
        return Point::from_groups(
            /* e1, e2, e3, e4 */
            float4(other[e1234]) * self.group0()
        );
    }
}
extension Point: AntiWedge<DualNum> {
    func anti_wedge(self, other: DualNum) -> Point {
        return Point::from_groups(
            /* e1, e2, e3, e4 */
            float4(other[e1234]) * self.group0()
        );
    }
}
extension Point: AntiWedge<Flector> {
    func anti_wedge(self, other: Flector) -> Scalar {
        return Scalar::from_groups(
            /* scalar */
            (other[e423] * self[e1]) + (other[e431] * self[e2]) + (other[e412] * self[e3]) + (other[e321] * self[e4])
        );
    }
}
extension Point: AntiWedge<Horizon> {
    func anti_wedge(self, other: Horizon) -> Scalar {
        return Scalar::from_groups(
            /* scalar */
            other[e321] * self[e4]
        );
    }
}
extension Point: AntiWedge<Motor> {
    func anti_wedge(self, other: Motor) -> Point {
        return Point::from_groups(
            /* e1, e2, e3, e4 */
            float4(other[e1234]) * self.group0()
        );
    }
}
extension Point: AntiWedge<MultiVector> {
    func anti_wedge(self, other: MultiVector) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2((other[e423] * self[e1]) + (other[e431] * self[e2]) + (other[e412] * self[e3]) + (other[e321] * self[e4]), 0.0), 
            /* e1, e2, e3, e4 */
            float4(other[e1234]) * self.group0(), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
extension Point: AntiWedge<Plane> {
    func anti_wedge(self, other: Plane) -> Scalar {
        return Scalar::from_groups(
            /* scalar */
            (other[e423] * self[e1]) + (other[e431] * self[e2]) + (other[e412] * self[e3]) + (other[e321] * self[e4])
        );
    }
}
extension Scalar {
    func div(_rhs: anti_wedge) -> anti_wedge_partial<Scalar> {
        anti_wedge_partial(this)
    }
}
extension Scalar: AntiWedge<AntiScalar> {
    func anti_wedge(self, other: AntiScalar) -> Scalar {
        return Scalar::from_groups(
            /* scalar */
            other[e1234] * self[scalar]
        );
    }
}
extension Scalar: AntiWedge<DualNum> {
    func anti_wedge(self, other: DualNum) -> Scalar {
        return Scalar::from_groups(
            /* scalar */
            other[e1234] * self[scalar]
        );
    }
}
extension Scalar: AntiWedge<Motor> {
    func anti_wedge(self, other: Motor) -> Scalar {
        return Scalar::from_groups(
            /* scalar */
            other[e1234] * self[scalar]
        );
    }
}
extension Scalar: AntiWedge<MultiVector> {
    func anti_wedge(self, other: MultiVector) -> Scalar {
        return Scalar::from_groups(
            /* scalar */
            other[e1234] * self[scalar]
        );
    }
}
