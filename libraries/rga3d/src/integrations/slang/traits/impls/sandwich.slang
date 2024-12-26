using traits::GeometricProduct;
using traits::Reverse;
extension AntiScalar {
    func div(_rhs: sandwich) -> sandwich_partial<AntiScalar> {
        sandwich_partial(this)
    }
}
extension AntiScalar: Sandwich<Flector> {
    func sandwich(self, other: Flector) -> Flector {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), self[e1234] * other[e321] * -1.0), 
            /* e423, e431, e412, e321 */
            float4(self[e1234], self[e1234], self[e1234], 0.0) * float4(other.group0().xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0)
        ).geometric_product(self.reverse());
    }
}
extension AntiScalar: Sandwich<Line> {
    func sandwich(self, other: Line) -> Line {
        return Line::from_groups(
            /* e41, e42, e43 */
            float3(self[e1234]) * other.group1(), 
            /* e23, e31, e12 */
            float3(0.0)
        ).geometric_product(self.reverse());
    }
}
extension AntiScalar: Sandwich<Motor> {
    func sandwich(self, other: Motor) -> Motor {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(self[e1234]) * other.group1(), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        ).geometric_product(self.reverse());
    }
}
extension AntiScalar: Sandwich<MultiVector> {
    func sandwich(self, other: MultiVector) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(1.0, self[e1234] * other[scalar]) * float2(0.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), self[e1234] * other[e321] * -1.0), 
            /* e41, e42, e43 */
            float3(self[e1234]) * other.group3(), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(self[e1234], self[e1234], self[e1234], 0.0) * float4(other.group1().xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0)
        ).geometric_product(self.reverse());
    }
}
extension AntiScalar: Sandwich<Point> {
    func sandwich(self, other: Point) -> Origin {
        return Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4(self[e1234], self[e1234], self[e1234], 0.0) * float4(other.group0().xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0)
        ).geometric_product(self.reverse());
    }
}
extension DualNum {
    func div(_rhs: sandwich) -> sandwich_partial<DualNum> {
        sandwich_partial(this)
    }
}
extension DualNum: Sandwich<AntiScalar> {
    func sandwich(self, other: AntiScalar) -> AntiScalar {
        return AntiScalar::from_groups(
            /* e1234 */
            other[e1234] * self[scalar]
        ).geometric_product(self.reverse());
    }
}
extension DualNum: Sandwich<DualNum> {
    func sandwich(self, other: DualNum) -> DualNum {
        return DualNum::from_groups(
            /* scalar, e1234 */
            float2(other[scalar] * self[scalar], (other[scalar] * self[e1234]) + (other[e1234] * self[scalar]))
        ).geometric_product(self.reverse());
    }
}
extension DualNum: Sandwich<Flector> {
    func sandwich(self, other: Flector) -> Flector {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(other[e1], other[e2], other[e3], 1.0) * float4(self.group0().xx, self[scalar], (self[scalar] * other[e4]) - (self[e1234] * other[e321])), 
            /* e423, e431, e412, e321 */
            float4(((float3(self[scalar]) * other.group1().xyz) - (float3(self[e1234]) * other.group0().xyz)), self[scalar] * other[e321])
        ).geometric_product(self.reverse());
    }
}
extension DualNum: Sandwich<Horizon> {
    func sandwich(self, other: Horizon) -> Flector {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), self[e1234] * other[e321] * -1.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), self[scalar] * other[e321])
        ).geometric_product(self.reverse());
    }
}
extension DualNum: Sandwich<Line> {
    func sandwich(self, other: Line) -> Line {
        return Line::from_groups(
            /* e41, e42, e43 */
            (float3(self[scalar]) * other.group0()) + (float3(self[e1234]) * other.group1()), 
            /* e23, e31, e12 */
            float3(self[scalar]) * other.group1()
        ).geometric_product(self.reverse());
    }
}
extension DualNum: Sandwich<Motor> {
    func sandwich(self, other: Motor) -> Motor {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            (float4(self[scalar]) * other.group0()) + (float4(self[e1234]) * other.group1()), 
            /* e23, e31, e12, scalar */
            float4(self[scalar]) * other.group1()
        ).geometric_product(self.reverse());
    }
}
extension DualNum: Sandwich<MultiVector> {
    func sandwich(self, other: MultiVector) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(self[scalar] * other[scalar], (self[scalar] * other[e1234]) + (self[e1234] * other[scalar])), 
            /* e1, e2, e3, e4 */
            float4(other[e1], other[e2], other[e3], 1.0) * float4(self.group0().xx, self[scalar], (self[scalar] * other[e4]) - (self[e1234] * other[e321])), 
            /* e41, e42, e43 */
            (float3(self[scalar]) * other.group2()) + (float3(self[e1234]) * other.group3()), 
            /* e23, e31, e12 */
            float3(self[scalar]) * other.group3(), 
            /* e423, e431, e412, e321 */
            float4(((float3(self[scalar]) * other.group4().xyz) - (float3(self[e1234]) * other.group1().xyz)), self[scalar] * other[e321])
        ).geometric_product(self.reverse());
    }
}
extension DualNum: Sandwich<Origin> {
    func sandwich(self, other: Origin) -> Origin {
        return Origin::from_groups(
            /* e4 */
            self[scalar] * other[e4]
        ).geometric_product(self.reverse());
    }
}
extension DualNum: Sandwich<Plane> {
    func sandwich(self, other: Plane) -> Flector {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), self[e1234] * other[e321] * -1.0), 
            /* e423, e431, e412, e321 */
            float4(self[scalar]) * other.group0()
        ).geometric_product(self.reverse());
    }
}
extension DualNum: Sandwich<Point> {
    func sandwich(self, other: Point) -> Flector {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(self[scalar]) * other.group0(), 
            /* e423, e431, e412, e321 */
            float4(self.group0().yy, self[e1234], 0.0) * float4(float3(1.0), 0.0) * float4(other.group0().xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0)
        ).geometric_product(self.reverse());
    }
}
extension DualNum: Sandwich<Scalar> {
    func sandwich(self, other: Scalar) -> DualNum {
        return DualNum::from_groups(
            /* scalar, e1234 */
            float2(other[scalar]) * self.group0()
        ).geometric_product(self.reverse());
    }
}
extension Flector {
    func div(_rhs: sandwich) -> sandwich_partial<Flector> {
        sandwich_partial(this)
    }
}
extension Flector: Sandwich<AntiScalar> {
    func sandwich(self, other: AntiScalar) -> Motor {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other[e1234] * self[e321]), 
            /* e423, e431, e412, e321 */
            float4(other[e1234], other[e1234], other[e1234], 0.0) * float4(self.group0().xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        ).geometric_product(self.reverse());
    }
}
extension Flector: Sandwich<DualNum> {
    func sandwich(self, other: DualNum) -> Motor {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(self[e1], self[e2], self[e3], 1.0) * float4(other.group0().xx, other[scalar], (other[scalar] * self[e4]) + (other[e1234] * self[e321])), 
            /* e423, e431, e412, e321 */
            float4(((float3(other[scalar]) * self.group1().xyz) + (float3(other[e1234]) * self.group0().xyz)), other[scalar] * self[e321])
        ).geometric_product(self.reverse());
    }
}
extension Flector: Sandwich<Flector> {
    func sandwich(self, other: Flector) -> Flector {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            (float4(other[e321]) * float4(self.group1().xyz, self[e4])) + (other.group1().zxyz * self.group0().yzxz) + (float4(self.group0().ww, self[e431], self[e1]) * float4(other.group0().xyx, other[e423])) + (float4(self.group1().zx, self[e4], self[e2]) * float4(other.group0().yzz, other[e431])) - (other.group0().zxyx * self.group1().yzxx) - (other.group0().wwwy * float4(self.group0().xyz, self[e431])) - (float4(self.group0().zx, self[e321], self[e321]) * float4(other.group1().yzz, other[e4])) - (float4(self.group1().ww, self[e2], self[e412]) * float4(other.group1().xyx, other[e3])), 
            /* e23, e31, e12, scalar */
            float4(-(other[e2] * self[e3]) - (other[e321] * self[e1]), -(other[e3] * self[e1]) - (other[e321] * self[e2]), -(other[e3] * self[e321]) - (other[e321] * self[e3]), (other[e2] * self[e2]) + (other[e3] * self[e3])) + (other.group0().zxyx * self.group0().yzxx) - (float4(self.group1().ww, self[e2], self[e321]) * float4(other.group0().xyx, other[e321]))
        ).geometric_product(self.reverse());
    }
}
extension Flector: Sandwich<Horizon> {
    func sandwich(self, other: Horizon) -> Flector {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(other[e321]) * float4(self.group1().xyz, self[e4]), 
            /* e23, e31, e12, scalar */
            float4(other[e321]) * float4(self.group0().xyz, self[e321]) * float4(-1.0)
        ).geometric_product(self.reverse());
    }
}
extension Flector: Sandwich<Line> {
    func sandwich(self, other: Line) -> Motor {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4((self[e3] * other[e31]) + (self[e321] * other[e23]), (self[e1] * other[e12]) + (self[e321] * other[e31]), (self[e2] * other[e23]) + (self[e321] * other[e12]), -(self[e2] * other[e42]) - (self[e3] * other[e43]) - (self[e423] * other[e23]) - (self[e431] * other[e31]) - (self[e412] * other[e12])) - (self.group0().yzxx * float4(other.group1().zxy, other[e41])), 
            /* e423, e431, e412, e321 */
            float4((self[e3] * other[e42]) + (self[e4] * other[e23]) + (self[e412] * other[e31]) + (self[e321] * other[e41]), (self[e1] * other[e43]) + (self[e4] * other[e31]) + (self[e423] * other[e12]) + (self[e321] * other[e42]), (self[e2] * other[e41]) + (self[e4] * other[e12]) + (self[e431] * other[e23]) + (self[e321] * other[e43]), self[e3] * other[e12] * -1.0) - (self.group0().yzxx * float4(other.group0().zxy, other[e23])) - float4((other.group1().zxy * self.group1().yzx), self[e2] * other[e31])
        ).geometric_product(self.reverse());
    }
}
extension Flector: Sandwich<Motor> {
    func sandwich(self, other: Motor) -> Motor {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(self[e321] * other[e23], self[e321] * other[e31], self[e321] * other[e12], -(self[e2] * other[e42]) - (self[e3] * other[e43]) - (self[e423] * other[e23]) - (self[e431] * other[e31]) - (self[e412] * other[e12])) + (self.group0().xxyw * other.group1().wzxw) + float4((self.group0().zyz * other.group1().yww), self[e321] * other[e1234]) - (self.group0().yzxx * float4(other.group1().zxy, other[e41])), 
            /* e423, e431, e412, e321 */
            float4((self[e3] * other[e42]) + (self[e4] * other[e23]) + (self[e423] * other[scalar]) + (self[e412] * other[e31]) + (self[e321] * other[e41]), (self[e2] * other[e1234]) + (self[e4] * other[e31]) + (self[e423] * other[e12]) + (self[e431] * other[scalar]) + (self[e321] * other[e42]), (self[e3] * other[e1234]) + (self[e4] * other[e12]) + (self[e431] * other[e23]) + (self[e412] * other[scalar]) + (self[e321] * other[e43]), self[e3] * other[e12] * -1.0) + float4((self.group0().xxy * other.group0().wzx), self[e321] * other[scalar]) - (self.group0().yzxx * float4(other.group0().zxy, other[e23])) - (other.group1().zxyy * float4(self.group1().yzx, self[e2]))
        ).geometric_product(self.reverse());
    }
}
extension Flector: Sandwich<MultiVector> {
    func sandwich(self, other: MultiVector) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(0.0, (self[e4] * other[e321]) - (self[e431] * other[e2]) - (self[e412] * other[e3]) - (self[e321] * other[e4])) + (float2(self[e1]) * float2(other[e1], other[e423])) + (float2(self[e2]) * float2(other[e2], other[e431])) + (float2(self[e3]) * float2(other[e3], other[e412])) - (float2(other[e321], other[e1]) * self.group1().wx), 
            /* e1, e2, e3, e4 */
            float4(self[e3] * other[e31], self[e1] * other[e12], self[e321] * other[e12], -(self[e2] * other[e42]) - (self[e3] * other[e43]) - (self[e423] * other[e23]) - (self[e431] * other[e31]) - (self[e412] * other[e12])) + (float4(other[scalar]) * self.group0()) + (float4(self.group1().ww, self[e2], self[e321]) * float4(other.group3().xyx, other[e1234])) - (self.group0().yzxx * float4(other.group3().zxy, other[e41])), 
            /* e41, e42, e43 */
            (float3(self[e4]) * other.group1().xyz) + (float3(other[e2], other[e321], other[e321]) * self.group1().zyz) + (float3(other[e321], other[e3], other[e1]) * self.group1().xxy) + (self.group0().yzx * other.group4().zxy) - (float3(self[e321]) * other.group4().xyz) - (float3(other[e4], other[e412], other[e423]) * self.group0().xxy) - (float3(other[e431], other[e4], other[e4]) * self.group0().zyz) - (self.group1().yzx * other.group1().zxy), 
            /* e23, e31, e12 */
            (self.group0().yzx * other.group1().zxy) - (float3(self[e321]) * other.group1().xyz) - (float3(other[e2], other[e321], other[e321]) * self.group0().zyz) - (float3(other[e321], other[e3], other[e1]) * self.group0().xxy), 
            /* e423, e431, e412, e321 */
            float4((self[e1] * other[e1234]) + (self[e3] * other[e42]) + (self[e4] * other[e23]) + (self[e412] * other[e31]) + (self[e321] * other[e41]), (self[e1] * other[e43]) + (self[e2] * other[e1234]) + (self[e4] * other[e31]) + (self[e423] * other[e12]) + (self[e321] * other[e42]), (self[e2] * other[e41]) + (self[e3] * other[e1234]) + (self[e4] * other[e12]) + (self[e431] * other[e23]) + (self[e321] * other[e43]), self[e3] * other[e12] * -1.0) + (float4(other[scalar]) * self.group1()) - (self.group0().yzxx * float4(other.group2().zxy, other[e23])) - float4((other.group3().zxy * self.group1().yzx), self[e2] * other[e31])
        ).geometric_product(self.reverse());
    }
}
extension Flector: Sandwich<Origin> {
    func sandwich(self, other: Origin) -> Flector {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(other[e4]) * float4(self.group0().xyz, self[e321]) * float4(-1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        ).geometric_product(self.reverse());
    }
}
extension Flector: Sandwich<Plane> {
    func sandwich(self, other: Plane) -> Flector {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(-(self[e3] * other[e431]) - (self[e321] * other[e423]), -(self[e1] * other[e412]) - (self[e321] * other[e431]), -(self[e2] * other[e423]) - (self[e321] * other[e412]), (self[e3] * other[e412]) + (self[e4] * other[e321])) + (self.group0().yzxx * other.group0().zxyx) + (other.group0().wwwy * float4(self.group1().xyz, self[e2])), 
            /* e23, e31, e12, scalar */
            float4(other[e321]) * float4(self.group0().xyz, self[e321]) * float4(-1.0)
        ).geometric_product(self.reverse());
    }
}
extension Flector: Sandwich<Point> {
    func sandwich(self, other: Point) -> Flector {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4((self[e4] * other[e1]) + (self[e412] * other[e2]), (self[e4] * other[e2]) + (self[e423] * other[e3]), (self[e4] * other[e3]) + (self[e431] * other[e1]), -(self[e412] * other[e3]) - (self[e321] * other[e4])) - (self.group1().yzxy * other.group0().zxyy) - (other.group0().wwwx * float4(self.group0().xyz, self[e423])), 
            /* e23, e31, e12, scalar */
            float4(-(self[e3] * other[e2]) - (self[e321] * other[e1]), -(self[e1] * other[e3]) - (self[e321] * other[e2]), -(self[e2] * other[e1]) - (self[e321] * other[e3]), (self[e2] * other[e2]) + (self[e3] * other[e3])) + (self.group0().yzxx * other.group0().zxyx)
        ).geometric_product(self.reverse());
    }
}
extension Flector: Sandwich<Scalar> {
    func sandwich(self, other: Scalar) -> Motor {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(other[scalar]) * self.group0(), 
            /* e423, e431, e412, e321 */
            float4(other[scalar]) * self.group1()
        ).geometric_product(self.reverse());
    }
}
extension Horizon {
    func div(_rhs: sandwich) -> sandwich_partial<Horizon> {
        sandwich_partial(this)
    }
}
extension Horizon: Sandwich<AntiScalar> {
    func sandwich(self, other: AntiScalar) -> AntiScalar {
        return Origin::from_groups(
            /* e4 */
            other[e1234] * self[e321]
        ).geometric_product(self.reverse());
    }
}
extension Horizon: Sandwich<DualNum> {
    func sandwich(self, other: DualNum) -> Motor {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other[e1234] * self[e321]), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other[scalar] * self[e321])
        ).geometric_product(self.reverse());
    }
}
extension Horizon: Sandwich<Flector> {
    func sandwich(self, other: Flector) -> Flector {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(self[e321]) * float4(other.group1().xyz, other[e4]) * float4(-1.0), 
            /* e23, e31, e12, scalar */
            float4(self[e321]) * float4(other.group0().xyz, other[e321]) * float4(-1.0)
        ).geometric_product(self.reverse());
    }
}
extension Horizon: Sandwich<Horizon> {
    func sandwich(self, other: Horizon) -> Horizon {
        return Scalar::from_groups(
            /* scalar */
            other[e321] * self[e321] * -1.0
        ).geometric_product(self.reverse());
    }
}
extension Horizon: Sandwich<Line> {
    func sandwich(self, other: Line) -> Motor {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(self[e321], self[e321], self[e321], 0.0) * float4(other.group1(), 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e423, e431, e412, e321 */
            float4(self[e321], self[e321], self[e321], 0.0) * float4(other.group0(), 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        ).geometric_product(self.reverse());
    }
}
extension Horizon: Sandwich<Motor> {
    func sandwich(self, other: Motor) -> Motor {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(self[e321]) * float4(other.group1().xyz, other[e1234]), 
            /* e423, e431, e412, e321 */
            float4(self[e321]) * float4(other.group0().xyz, other[scalar])
        ).geometric_product(self.reverse());
    }
}
extension Horizon: Sandwich<MultiVector> {
    func sandwich(self, other: MultiVector) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(self[e321]) * float2(other[e321], other[e4]) * float2(-1.0), 
            /* e1, e2, e3, e4 */
            float4(self[e321]) * float4(other.group3(), other[e1234]), 
            /* e41, e42, e43 */
            float3(self[e321]) * other.group4().xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(self[e321]) * other.group1().xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            float4(self[e321]) * float4(other.group2(), other[scalar])
        ).geometric_product(self.reverse());
    }
}
extension Horizon: Sandwich<Origin> {
    func sandwich(self, other: Origin) -> Origin {
        return AntiScalar::from_groups(
            /* e1234 */
            self[e321] * other[e4] * -1.0
        ).geometric_product(self.reverse());
    }
}
extension Horizon: Sandwich<Plane> {
    func sandwich(self, other: Plane) -> Flector {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(self[e321], self[e321], self[e321], 0.0) * float4(other.group0().xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0), 
            /* e23, e31, e12, scalar */
            float4(float3(0.0), self[e321] * other[e321] * -1.0)
        ).geometric_product(self.reverse());
    }
}
extension Horizon: Sandwich<Point> {
    func sandwich(self, other: Point) -> Flector {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(float3(0.0), self[e321] * other[e4] * -1.0), 
            /* e23, e31, e12, scalar */
            float4(self[e321], self[e321], self[e321], 0.0) * float4(other.group0().xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0)
        ).geometric_product(self.reverse());
    }
}
extension Horizon: Sandwich<Scalar> {
    func sandwich(self, other: Scalar) -> Scalar {
        return Horizon::from_groups(
            /* e321 */
            self[e321] * other[scalar]
        ).geometric_product(self.reverse());
    }
}
extension Line {
    func div(_rhs: sandwich) -> sandwich_partial<Line> {
        sandwich_partial(this)
    }
}
extension Line: Sandwich<AntiScalar> {
    func sandwich(self, other: AntiScalar) -> Motor {
        return Line::from_groups(
            /* e41, e42, e43 */
            float3(other[e1234]) * self.group1(), 
            /* e23, e31, e12 */
            float3(0.0)
        ).geometric_product(self.reverse());
    }
}
extension Line: Sandwich<DualNum> {
    func sandwich(self, other: DualNum) -> Motor {
        return Line::from_groups(
            /* e41, e42, e43 */
            (float3(other[scalar]) * self.group0()) + (float3(other[e1234]) * self.group1()), 
            /* e23, e31, e12 */
            float3(other[scalar]) * self.group1()
        ).geometric_product(self.reverse());
    }
}
extension Line: Sandwich<Flector> {
    func sandwich(self, other: Flector) -> Flector {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            (float4(other[e2], other[e321], other[e321], other[e2]) * float4(self.group1().zyz, self[e42])) + (float4(other[e321], other[e3], other[e1], other[e1]) * float4(self.group1().xxy, self[e41])) + float4(float3(0.0), (other[e3] * self[e43]) - (other[e431] * self[e31]) - (other[e412] * self[e12])) - float4((self.group1().yzx * other.group0().zxy), other[e423] * self[e23]), 
            /* e423, e431, e412, e321 */
            float4((other[e3] * self[e42]) + (other[e4] * self[e23]) + (other[e431] * self[e12]), (other[e1] * self[e43]) + (other[e4] * self[e31]) + (other[e412] * self[e23]), (other[e2] * self[e41]) + (other[e4] * self[e12]) + (other[e423] * self[e31]), 0.0) - (float4(other[e2], other[e321], other[e321], other[e2]) * float4(self.group0().zyz, self[e31])) - (float4(other[e321], other[e3], other[e1], other[e1]) * float4(self.group0().xxy, self[e23])) - float4((self.group1().yzx * other.group1().zxy), other[e3] * self[e12])
        ).geometric_product(self.reverse());
    }
}
extension Line: Sandwich<Horizon> {
    func sandwich(self, other: Horizon) -> Flector {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(other[e321], other[e321], other[e321], 0.0) * float4(self.group1(), 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e423, e431, e412, e321 */
            float4(other[e321], other[e321], other[e321], 0.0) * float4(self.group0(), 0.0) * float4(-1.0, -1.0, -1.0, 0.0)
        ).geometric_product(self.reverse());
    }
}
extension Line: Sandwich<Line> {
    func sandwich(self, other: Line) -> Motor {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4((other[e42] * self[e12]) + (other[e31] * self[e43]), (other[e43] * self[e23]) + (other[e12] * self[e41]), (other[e41] * self[e31]) + (other[e23] * self[e42]), -(other[e43] * self[e12]) - (other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43])) - float4((other.group0().zxy * self.group1().yzx), other[e41] * self[e23]) - float4((other.group1().zxy * self.group0().yzx), other[e42] * self[e31]), 
            /* e23, e31, e12, scalar */
            float4(other[e31] * self[e12], other[e12] * self[e23], other[e23] * self[e31], -(other[e31] * self[e31]) - (other[e12] * self[e12])) - float4((other.group1().zxy * self.group1().yzx), other[e23] * self[e23])
        ).geometric_product(self.reverse());
    }
}
extension Line: Sandwich<Motor> {
    func sandwich(self, other: Motor) -> Motor {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4((self[e41] * other[scalar]) + (self[e43] * other[e31]) + (self[e23] * other[e1234]) + (self[e12] * other[e42]), (self[e41] * other[e12]) + (self[e42] * other[scalar]) + (self[e23] * other[e43]) + (self[e31] * other[e1234]), (self[e42] * other[e23]) + (self[e43] * other[scalar]) + (self[e31] * other[e41]) + (self[e12] * other[e1234]), -(self[e43] * other[e12]) - (self[e23] * other[e41]) - (self[e31] * other[e42]) - (self[e12] * other[e43])) - (other.group1().zxyx * float4(self.group0().yzx, self[e41])) - float4((self.group1().yzx * other.group0().zxy), self[e42] * other[e31]), 
            /* e23, e31, e12, scalar */
            float4((self[e23] * other[scalar]) + (self[e12] * other[e31]), (self[e23] * other[e12]) + (self[e31] * other[scalar]), (self[e31] * other[e23]) + (self[e12] * other[scalar]), -(self[e31] * other[e31]) - (self[e12] * other[e12])) - (other.group1().zxyx * float4(self.group1().yzx, self[e23]))
        ).geometric_product(self.reverse());
    }
}
extension Line: Sandwich<MultiVector> {
    func sandwich(self, other: MultiVector) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(0.0, -(self[e23] * other[e41]) - (self[e31] * other[e42]) - (self[e12] * other[e43])) - (float2(other[e23]) * float2(self[e23], self[e41])) - (float2(other[e31]) * float2(self[e31], self[e42])) - (float2(other[e12]) * float2(self[e12], self[e43])), 
            /* e1, e2, e3, e4 */
            (float4(other[e2], other[e321], other[e321], other[e2]) * float4(self.group1().zyz, self[e42])) + (float4(other[e321], other[e3], other[e1], other[e1]) * float4(self.group1().xxy, self[e41])) + float4(float3(0.0), (self[e43] * other[e3]) - (self[e31] * other[e431]) - (self[e12] * other[e412])) - float4((self.group1().yzx * other.group1().zxy), self[e23] * other[e423]), 
            /* e41, e42, e43 */
            (float3(other[scalar]) * self.group0()) + (float3(other[e1234]) * self.group1()) + (self.group0().zxy * other.group3().yzx) + (self.group1().zxy * other.group2().yzx) - (self.group0().yzx * other.group3().zxy) - (self.group1().yzx * other.group2().zxy), 
            /* e23, e31, e12 */
            (float3(other[scalar]) * self.group1()) + (self.group1().zxy * other.group3().yzx) - (self.group1().yzx * other.group3().zxy), 
            /* e423, e431, e412, e321 */
            float4((self[e42] * other[e3]) + (self[e23] * other[e4]) + (self[e12] * other[e431]), (self[e43] * other[e1]) + (self[e23] * other[e412]) + (self[e31] * other[e4]), (self[e41] * other[e2]) + (self[e31] * other[e423]) + (self[e12] * other[e4]), 0.0) - (float4(other[e2], other[e321], other[e321], other[e2]) * float4(self.group0().zyz, self[e31])) - (float4(other[e321], other[e3], other[e1], other[e1]) * float4(self.group0().xxy, self[e23])) - float4((self.group1().yzx * other.group4().zxy), self[e12] * other[e3])
        ).geometric_product(self.reverse());
    }
}
extension Line: Sandwich<Origin> {
    func sandwich(self, other: Origin) -> Flector {
        return Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4(other[e4], other[e4], other[e4], 0.0) * float4(self.group1(), 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        ).geometric_product(self.reverse());
    }
}
extension Line: Sandwich<Plane> {
    func sandwich(self, other: Plane) -> Flector {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(other[e321], other[e321], other[e321], 1.0) * float4(self.group1(), -(self[e23] * other[e423]) - (self[e31] * other[e431]) - (self[e12] * other[e412])), 
            /* e423, e431, e412, e321 */
            float4((self.group1().zxy * other.group0().yzx), 0.0) - float4((float3(other[e321]) * self.group0()), 0.0) - float4((self.group1().yzx * other.group0().zxy), 0.0)
        ).geometric_product(self.reverse());
    }
}
extension Line: Sandwich<Point> {
    func sandwich(self, other: Point) -> Flector {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(self[e31] * other[e3] * -1.0, self[e12] * other[e1] * -1.0, self[e23] * other[e2] * -1.0, (self[e42] * other[e2]) + (self[e43] * other[e3])) + (other.group0().yzxx * float4(self.group1().zxy, self[e41])), 
            /* e423, e431, e412, e321 */
            float4((self[e42] * other[e3]) + (self[e23] * other[e4]), (self[e43] * other[e1]) + (self[e31] * other[e4]), (self[e41] * other[e2]) + (self[e12] * other[e4]), -(self[e31] * other[e2]) - (self[e12] * other[e3])) - (other.group0().yzxx * float4(self.group0().zxy, self[e23]))
        ).geometric_product(self.reverse());
    }
}
extension Line: Sandwich<Scalar> {
    func sandwich(self, other: Scalar) -> Motor {
        return Line::from_groups(
            /* e41, e42, e43 */
            float3(other[scalar]) * self.group0(), 
            /* e23, e31, e12 */
            float3(other[scalar]) * self.group1()
        ).geometric_product(self.reverse());
    }
}
extension Motor {
    func div(_rhs: sandwich) -> sandwich_partial<Motor> {
        sandwich_partial(this)
    }
}
extension Motor: Sandwich<AntiScalar> {
    func sandwich(self, other: AntiScalar) -> Motor {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(other[e1234]) * self.group1(), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        ).geometric_product(self.reverse());
    }
}
extension Motor: Sandwich<DualNum> {
    func sandwich(self, other: DualNum) -> Motor {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            (float4(other[scalar]) * self.group0()) + (float4(other[e1234]) * self.group1()), 
            /* e23, e31, e12, scalar */
            float4(other[scalar]) * self.group1()
        ).geometric_product(self.reverse());
    }
}
extension Motor: Sandwich<Flector> {
    func sandwich(self, other: Flector) -> Flector {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            (other.group0().xyxx * float4(self.group1().wwy, self[e41])) + (other.group0().yzzy * float4(self.group1().zxw, self[e42])) + float4(float3(0.0), (other[e4] * self[scalar]) - (other[e431] * self[e31]) - (other[e412] * self[e12]) - (other[e321] * self[e1234])) + float4((other.group1().www * self.group1().xyz), other[e3] * self[e43]) - (self.group1().yzxx * float4(other.group0().zxy, other[e423])), 
            /* e423, e431, e412, e321 */
            float4((other[e4] * self[e23]) + (other[e423] * self[scalar]) + (other[e431] * self[e12]) - (other[e321] * self[e41]), (other[e4] * self[e31]) + (other[e431] * self[scalar]) + (other[e412] * self[e23]) - (other[e321] * self[e42]), (other[e4] * self[e12]) + (other[e423] * self[e31]) + (other[e412] * self[scalar]) - (other[e321] * self[e43]), 0.0) + float4((other.group0().zxy * self.group0().yzx), other[e321] * self[scalar]) - (other.group0().xyxx * float4(self.group0().wwy, self[e23])) - (other.group0().yzzy * float4(self.group0().zxw, self[e31])) - (self.group1().yzxz * float4(other.group1().zxy, other[e3]))
        ).geometric_product(self.reverse());
    }
}
extension Motor: Sandwich<Horizon> {
    func sandwich(self, other: Horizon) -> Flector {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(other[e321]) * float4(self.group1().xyz, self[e1234]) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e423, e431, e412, e321 */
            float4(other[e321]) * float4(self.group0().xyz, self[scalar]) * float4(-1.0, -1.0, -1.0, 1.0)
        ).geometric_product(self.reverse());
    }
}
extension Motor: Sandwich<Line> {
    func sandwich(self, other: Line) -> Motor {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4((other[e41] * self[scalar]) + (other[e42] * self[e12]) + (other[e23] * self[e1234]) + (other[e31] * self[e43]), (other[e42] * self[scalar]) + (other[e43] * self[e23]) + (other[e31] * self[e1234]) + (other[e12] * self[e41]), (other[e41] * self[e31]) + (other[e43] * self[scalar]) + (other[e23] * self[e42]) + (other[e12] * self[e1234]), -(other[e43] * self[e12]) - (other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43])) - (self.group1().yzxx * float4(other.group0().zxy, other[e41])) - float4((other.group1().zxy * self.group0().yzx), other[e42] * self[e31]), 
            /* e23, e31, e12, scalar */
            float4((other[e23] * self[scalar]) + (other[e31] * self[e12]), (other[e31] * self[scalar]) + (other[e12] * self[e23]), (other[e23] * self[e31]) + (other[e12] * self[scalar]), -(other[e31] * self[e31]) - (other[e12] * self[e12])) - (self.group1().yzxx * float4(other.group1().zxy, other[e23]))
        ).geometric_product(self.reverse());
    }
}
extension Motor: Sandwich<Motor> {
    func sandwich(self, other: Motor) -> Motor {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4((other[e1234] * self[e23]) + (other[e23] * self[e1234]) + (other[e31] * self[e43]) + (other[scalar] * self[e41]), (other[e1234] * self[e31]) + (other[e31] * self[e1234]) + (other[e12] * self[e41]) + (other[scalar] * self[e42]), (other[e1234] * self[e12]) + (other[e23] * self[e42]) + (other[e12] * self[e1234]) + (other[scalar] * self[e43]), -(other[e43] * self[e12]) - (other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43])) + (other.group0().xyxw * self.group1().wwyw) + float4((other.group0().yzz * self.group1().zxw), other[scalar] * self[e1234]) - (other.group0().zxyx * self.group1().yzxx) - float4((other.group1().zxy * self.group0().yzx), other[e42] * self[e31]), 
            /* e23, e31, e12, scalar */
            float4((other[e31] * self[e12]) + (other[scalar] * self[e23]), (other[e12] * self[e23]) + (other[scalar] * self[e31]), (other[e12] * self[scalar]) + (other[scalar] * self[e12]), -(other[e31] * self[e31]) - (other[e12] * self[e12])) + (other.group1().xyxw * self.group1().wwyw) - (other.group1().zxyx * self.group1().yzxx)
        ).geometric_product(self.reverse());
    }
}
extension Motor: Sandwich<MultiVector> {
    func sandwich(self, other: MultiVector) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(0.0, (self[scalar] * other[e1234]) - (self[e41] * other[e23]) - (self[e42] * other[e31]) - (self[e43] * other[e12])) + (float2(other[scalar]) * float2(self[scalar], self[e1234])) - (float2(self[e23]) * float2(other[e23], other[e41])) - (float2(self[e31]) * float2(other[e31], other[e42])) - (float2(self[e12]) * float2(other[e12], other[e43])), 
            /* e1, e2, e3, e4 */
            (float4(other[e2], other[e321], other[e321], other[e2]) * float4(self.group1().zyz, self[e42])) + (float4(other[e321], other[e3], other[e1], other[e1]) * float4(self.group1().xxy, self[e41])) + (other.group1().xyzz * float4(self.group1().www, self[e43])) + float4(float3(0.0), (self[scalar] * other[e4]) - (self[e23] * other[e423]) - (self[e31] * other[e431]) - (self[e12] * other[e412])) - float4((self.group1().yzx * other.group1().zxy), self[e1234] * other[e321]), 
            /* e41, e42, e43 */
            (float3(other[scalar]) * self.group0().xyz) + (float3(other[e1234]) * self.group1().xyz) + (other.group2().xyx * self.group1().wwy) + (other.group2().yzz * self.group1().zxw) + (other.group3().xyx * self.group0().wwy) + (other.group3().yzz * self.group0().zxw) - (other.group2().zxy * self.group1().yzx) - (other.group3().zxy * self.group0().yzx), 
            /* e23, e31, e12 */
            (float3(other[scalar]) * self.group1().xyz) + (other.group3().xyx * self.group1().wwy) + (other.group3().yzz * self.group1().zxw) - (other.group3().zxy * self.group1().yzx), 
            /* e423, e431, e412, e321 */
            float4((self[e23] * other[e4]) + (self[e12] * other[e431]) + (self[scalar] * other[e423]) - (self[e31] * other[e412]), (self[e23] * other[e412]) + (self[e31] * other[e4]) + (self[scalar] * other[e431]) - (self[e12] * other[e423]), (self[e31] * other[e423]) + (self[e12] * other[e4]) + (self[scalar] * other[e412]) - (self[e23] * other[e431]), 0.0) + float4((self.group0().yzx * other.group1().zxy), self[scalar] * other[e321]) - (float4(other[e2], other[e321], other[e321], other[e2]) * float4(self.group0().zyz, self[e31])) - (float4(other[e321], other[e3], other[e1], other[e1]) * float4(self.group0().xxy, self[e23])) - (other.group1().xyzz * float4(self.group0().www, self[e12]))
        ).geometric_product(self.reverse());
    }
}
extension Motor: Sandwich<Origin> {
    func sandwich(self, other: Origin) -> Flector {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), self[scalar] * other[e4]), 
            /* e423, e431, e412, e321 */
            float4(other[e4], other[e4], other[e4], 0.0) * float4(self.group1().xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        ).geometric_product(self.reverse());
    }
}
extension Motor: Sandwich<Plane> {
    func sandwich(self, other: Plane) -> Flector {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(other[e321], other[e321], other[e321], 1.0) * float4(self.group1().xyz, -(self[e1234] * other[e321]) - (self[e23] * other[e423]) - (self[e31] * other[e431]) - (self[e12] * other[e412])), 
            /* e423, e431, e412, e321 */
            float4(((float3(self[scalar]) * other.group0().xyz) + (self.group1().zxy * other.group0().yzx) - (float3(other[e321]) * self.group0().xyz) - (self.group1().yzx * other.group0().zxy)), self[scalar] * other[e321])
        ).geometric_product(self.reverse());
    }
}
extension Motor: Sandwich<Point> {
    func sandwich(self, other: Point) -> Flector {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(self[e31] * other[e3] * -1.0, self[e12] * other[e1] * -1.0, self[e23] * other[e2] * -1.0, (self[e43] * other[e3]) + (self[scalar] * other[e4])) + (other.group0().xyzy * float4(self.group1().www, self[e42])) + (other.group0().yzxx * float4(self.group1().zxy, self[e41])), 
            /* e423, e431, e412, e321 */
            float4((self[e42] * other[e3]) + (self[e23] * other[e4]), (self[e43] * other[e1]) + (self[e31] * other[e4]), (self[e41] * other[e2]) + (self[e12] * other[e4]), self[e12] * other[e3] * -1.0) - (other.group0().xyzy * float4(self.group0().www, self[e31])) - (other.group0().yzxx * float4(self.group0().zxy, self[e23]))
        ).geometric_product(self.reverse());
    }
}
extension Motor: Sandwich<Scalar> {
    func sandwich(self, other: Scalar) -> Motor {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(other[scalar]) * self.group0(), 
            /* e23, e31, e12, scalar */
            float4(other[scalar]) * self.group1()
        ).geometric_product(self.reverse());
    }
}
extension MultiVector {
    func div(_rhs: sandwich) -> sandwich_partial<MultiVector> {
        sandwich_partial(this)
    }
}
extension MultiVector: Sandwich<AntiScalar> {
    func sandwich(self, other: AntiScalar) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(1.0, other[e1234] * self[scalar]) * float2(0.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other[e1234] * self[e321]), 
            /* e41, e42, e43 */
            float3(other[e1234]) * self.group3(), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(other[e1234], other[e1234], other[e1234], 0.0) * float4(self.group1().xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        ).geometric_product(self.reverse());
    }
}
extension MultiVector: Sandwich<DualNum> {
    func sandwich(self, other: DualNum) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(other[scalar] * self[scalar], (other[scalar] * self[e1234]) + (other[e1234] * self[scalar])), 
            /* e1, e2, e3, e4 */
            float4(self[e1], self[e2], self[e3], 1.0) * float4(other.group0().xx, other[scalar], (other[scalar] * self[e4]) + (other[e1234] * self[e321])), 
            /* e41, e42, e43 */
            (float3(other[scalar]) * self.group2()) + (float3(other[e1234]) * self.group3()), 
            /* e23, e31, e12 */
            float3(other[scalar]) * self.group3(), 
            /* e423, e431, e412, e321 */
            float4(((float3(other[scalar]) * self.group4().xyz) + (float3(other[e1234]) * self.group1().xyz)), other[scalar] * self[e321])
        ).geometric_product(self.reverse());
    }
}
extension MultiVector: Sandwich<Flector> {
    func sandwich(self, other: Flector) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(0.0, (other[e321] * self[e4]) - (other[e2] * self[e431]) - (other[e3] * self[e412]) - (other[e4] * self[e321])) + (float2(self[e1]) * float2(other[e1], other[e423])) + (float2(self[e2]) * float2(other[e2], other[e431])) + (float2(self[e3]) * float2(other[e3], other[e412])) - (float2(other[e321], other[e1]) * self.group4().wx), 
            /* e1, e2, e3, e4 */
            (float4(self[scalar]) * other.group0()) + (float4(other[e2], other[e321], other[e321], other[e2]) * float4(self.group3().zyz, self[e42])) + (float4(other[e321], other[e3], other[e1], other[e1]) * float4(self.group3().xxy, self[e41])) + float4(float3(0.0), (other[e3] * self[e43]) - (other[e423] * self[e23]) - (other[e431] * self[e31]) - (other[e412] * self[e12])) - float4((self.group3().yzx * other.group0().zxy), other[e321] * self[e1234]), 
            /* e41, e42, e43 */
            (float3(other[e321]) * self.group4().xyz) + (float3(self[e4], self[e4], self[e431]) * other.group0().xyx) + (float3(self[e412], self[e423], self[e4]) * other.group0().yzz) + (other.group1().zxy * self.group1().yzx) - (float3(other[e4]) * self.group1().xyz) - (float3(self[e3], self[e1], self[e321]) * other.group1().yzz) - (float3(self[e321], self[e321], self[e2]) * other.group1().xyx) - (other.group0().zxy * self.group4().yzx), 
            /* e23, e31, e12 */
            (other.group0().zxy * self.group1().yzx) - (float3(other[e321]) * self.group1().xyz) - (float3(self[e3], self[e1], self[e321]) * other.group0().yzz) - (float3(self[e321], self[e321], self[e2]) * other.group0().xyx), 
            /* e423, e431, e412, e321 */
            float4((other[e3] * self[e42]) + (other[e4] * self[e23]) + (other[e431] * self[e12]) - (other[e412] * self[e31]), (other[e1] * self[e43]) + (other[e4] * self[e31]) + (other[e412] * self[e23]) - (other[e423] * self[e12]), (other[e2] * self[e41]) + (other[e4] * self[e12]) + (other[e423] * self[e31]) - (other[e431] * self[e23]), 0.0) + (float4(self[scalar]) * other.group1()) - (float4(other[e2], other[e321], other[e321], other[e3]) * float4(self.group2().zyz, self[e12])) - (float4(other[e321], other[e3], other[e1], other[e2]) * float4(self.group2().xxy, self[e31])) - (other.group0().xyzx * float4(self.group0().yy, self[e1234], self[e23]))
        ).geometric_product(self.reverse());
    }
}
extension MultiVector: Sandwich<Horizon> {
    func sandwich(self, other: Horizon) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(other[e321]) * float2(self[e321], self[e4]) * float2(-1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(other[e321]) * float4(self.group3(), self[e1234]) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e41, e42, e43 */
            float3(other[e321]) * self.group4().xyz, 
            /* e23, e31, e12 */
            float3(other[e321]) * self.group1().xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            float4(other[e321]) * float4(self.group2(), self[scalar]) * float4(-1.0, -1.0, -1.0, 1.0)
        ).geometric_product(self.reverse());
    }
}
extension MultiVector: Sandwich<Line> {
    func sandwich(self, other: Line) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(0.0, -(other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43])) - (float2(self[e23]) * float2(other[e23], other[e41])) - (float2(self[e31]) * float2(other[e31], other[e42])) - (float2(self[e12]) * float2(other[e12], other[e43])), 
            /* e1, e2, e3, e4 */
            float4((other[e23] * self[e321]) + (other[e31] * self[e3]), (other[e31] * self[e321]) + (other[e12] * self[e1]), (other[e23] * self[e2]) + (other[e12] * self[e321]), -(other[e42] * self[e2]) - (other[e43] * self[e3]) - (other[e23] * self[e423]) - (other[e31] * self[e431]) - (other[e12] * self[e412])) - (self.group1().yzxx * float4(other.group1().zxy, other[e41])), 
            /* e41, e42, e43 */
            (float3(self[scalar]) * other.group0()) + (float3(self[e1234]) * other.group1()) + (other.group0().yzx * self.group3().zxy) + (other.group1().yzx * self.group2().zxy) - (other.group0().zxy * self.group3().yzx) - (other.group1().zxy * self.group2().yzx), 
            /* e23, e31, e12 */
            (float3(self[scalar]) * other.group1()) + (other.group1().yzx * self.group3().zxy) - (other.group1().zxy * self.group3().yzx), 
            /* e423, e431, e412, e321 */
            float4((other[e41] * self[e321]) + (other[e42] * self[e3]) + (other[e23] * self[e4]) + (other[e31] * self[e412]), (other[e42] * self[e321]) + (other[e43] * self[e1]) + (other[e31] * self[e4]) + (other[e12] * self[e423]), (other[e41] * self[e2]) + (other[e43] * self[e321]) + (other[e23] * self[e431]) + (other[e12] * self[e4]), other[e12] * self[e3] * -1.0) - (self.group1().yzxx * float4(other.group0().zxy, other[e23])) - float4((other.group1().zxy * self.group4().yzx), other[e31] * self[e2])
        ).geometric_product(self.reverse());
    }
}
extension MultiVector: Sandwich<Motor> {
    func sandwich(self, other: Motor) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(0.0, (other[scalar] * self[e1234]) - (other[e41] * self[e23]) - (other[e42] * self[e31]) - (other[e43] * self[e12])) + (float2(self[scalar]) * float2(other[scalar], other[e1234])) - (float2(other[e23]) * float2(self[e23], self[e41])) - (float2(other[e31]) * float2(self[e31], self[e42])) - (float2(other[e12]) * float2(self[e12], self[e43])), 
            /* e1, e2, e3, e4 */
            float4(other[scalar] * self[e1], other[scalar] * self[e2], other[scalar] * self[e3], -(other[e42] * self[e2]) - (other[e43] * self[e3]) - (other[e23] * self[e423]) - (other[e31] * self[e431]) - (other[e12] * self[e412])) + (other.group1().yzzw * float4(self.group1().zx, self[e321], self[e4])) + (float4(self.group4().ww, self[e2], self[e321]) * float4(other.group1().xyx, other[e1234])) - (self.group1().yzxx * float4(other.group1().zxy, other[e41])), 
            /* e41, e42, e43 */
            (float3(self[scalar]) * other.group0().xyz) + (float3(self[e1234]) * other.group1().xyz) + (self.group2().xxy * other.group1().wzx) + (self.group2().zyz * other.group1().yww) + (self.group3().xxy * other.group0().wzx) + (self.group3().zyz * other.group0().yww) - (self.group2().yzx * other.group1().zxy) - (self.group3().yzx * other.group0().zxy), 
            /* e23, e31, e12 */
            (float3(self[scalar]) * other.group1().xyz) + (self.group3().xxy * other.group1().wzx) + (self.group3().zyz * other.group1().yww) - (self.group3().yzx * other.group1().zxy), 
            /* e423, e431, e412, e321 */
            float4((other[e42] * self[e3]) + (other[e1234] * self[e1]) + (other[e23] * self[e4]) + (other[e31] * self[e412]) + (other[scalar] * self[e423]), (other[e43] * self[e1]) + (other[e1234] * self[e2]) + (other[e31] * self[e4]) + (other[e12] * self[e423]) + (other[scalar] * self[e431]), (other[e43] * self[e321]) + (other[e1234] * self[e3]) + (other[e23] * self[e431]) + (other[e12] * self[e4]) + (other[scalar] * self[e412]), other[e12] * self[e3] * -1.0) + (float4(self.group4().ww, self[e2], self[e321]) * float4(other.group0().xyx, other[scalar])) - (other.group1().zxyy * float4(self.group4().yzx, self[e2])) - (self.group1().yzxx * float4(other.group0().zxy, other[e23]))
        ).geometric_product(self.reverse());
    }
}
extension MultiVector: Sandwich<MultiVector> {
    func sandwich(self, other: MultiVector) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(0.0, (other[e1234] * self[scalar]) + (other[e321] * self[e4]) - (other[e2] * self[e431]) - (other[e3] * self[e412]) - (other[e4] * self[e321]) - (other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43])) + (float2(other[scalar]) * self.group0()) + (float2(self[e1]) * float2(other[e1], other[e423])) + (float2(self[e2]) * float2(other[e2], other[e431])) + (float2(self[e3]) * float2(other[e3], other[e412])) - (float2(self[e23]) * float2(other[e23], other[e41])) - (float2(self[e31]) * float2(other[e31], other[e42])) - (float2(self[e12]) * float2(other[e12], other[e43])) - (float2(other[e321], other[e1]) * self.group4().wx), 
            /* e1, e2, e3, e4 */
            (float4(other[scalar]) * self.group1()) + (float4(other[e2], other[e321], other[e321], other[e3]) * float4(self.group3().zyz, self[e43])) + (float4(other[e321], other[e3], other[e1], other[e2]) * float4(self.group3().xxy, self[e42])) + (float4(self.group0().xx, self[scalar], other[e1234]) * float4(other.group1().xyz, self[e321])) + (float4(self.group1().zx, self[e321], other[e1]) * float4(other.group3().yzz, self[e41])) + (float4(self.group4().ww, self[e2], other[e4]) * float4(other.group3().xyx, self[scalar])) + float4(float3(0.0), -(other[e42] * self[e2]) - (other[e43] * self[e3]) - (other[e23] * self[e423]) - (other[e31] * self[e431]) - (other[e12] * self[e412]) - (other[e423] * self[e23]) - (other[e431] * self[e31]) - (other[e412] * self[e12])) - float4((other.group3().zxy * self.group1().yzx), other[e321] * self[e1234]) - float4((self.group3().yzx * other.group1().zxy), other[e41] * self[e1]), 
            /* e41, e42, e43 */
            (float3(other[scalar]) * self.group2()) + (float3(other[e1234]) * self.group3()) + (float3(other[e321]) * self.group4().xyz) + (float3(self[scalar]) * other.group2()) + (float3(self[e1234]) * other.group3()) + (float3(self[e4], self[e4], self[e431]) * other.group1().xyx) + (float3(self[e412], self[e423], self[e4]) * other.group1().yzz) + (other.group2().yzx * self.group3().zxy) + (other.group3().yzx * self.group2().zxy) + (other.group4().zxy * self.group1().yzx) - (float3(other[e4]) * self.group1().xyz) - (float3(self[e3], self[e1], self[e321]) * other.group4().yzz) - (float3(self[e321], self[e321], self[e2]) * other.group4().xyx) - (other.group2().zxy * self.group3().yzx) - (other.group3().zxy * self.group2().yzx) - (other.group1().zxy * self.group4().yzx), 
            /* e23, e31, e12 */
            (float3(other[scalar]) * self.group3()) + (float3(self[scalar]) * other.group3()) + (other.group3().yzx * self.group3().zxy) + (other.group1().zxy * self.group1().yzx) - (float3(other[e321]) * self.group1().xyz) - (float3(self[e3], self[e1], self[e321]) * other.group1().yzz) - (float3(self[e321], self[e321], self[e2]) * other.group1().xyx) - (other.group3().zxy * self.group3().yzx), 
            /* e423, e431, e412, e321 */
            float4((other[e3] * self[e42]) + (other[e4] * self[e23]) + (other[e41] * self[e321]) + (other[e42] * self[e3]) + (other[e23] * self[e4]) + (other[e31] * self[e412]) + (other[e423] * self[scalar]) + (other[e431] * self[e12]), (other[e1] * self[e43]) + (other[e4] * self[e31]) + (other[e42] * self[e321]) + (other[e43] * self[e1]) + (other[e31] * self[e4]) + (other[e12] * self[e423]) + (other[e431] * self[scalar]) + (other[e412] * self[e23]), (other[e2] * self[e41]) + (other[e4] * self[e12]) + (other[e41] * self[e2]) + (other[e43] * self[e321]) + (other[e23] * self[e431]) + (other[e12] * self[e4]) + (other[e423] * self[e31]) + (other[e412] * self[scalar]), 0.0) + (float4(other[scalar]) * self.group4()) + (float4(other.group0().yy, other[e1234], self[scalar]) * float4(self.group1().xyz, other[e321])) - (float4(other[e2], other[e321], other[e321], other[e2]) * float4(self.group2().zyz, self[e31])) - (float4(other[e321], other[e3], other[e1], other[e1]) * float4(self.group2().xxy, self[e23])) - (self.group1().yzxy * float4(other.group2().zxy, other[e31])) - (float4(self.group0().yy, self[e1234], other[e23]) * float4(other.group1().xyz, self[e1])) - float4((other.group3().zxy * self.group4().yzx), other[e12] * self[e3]) - float4((self.group3().yzx * other.group4().zxy), other[e3] * self[e12])
        ).geometric_product(self.reverse());
    }
}
extension MultiVector: Sandwich<Origin> {
    func sandwich(self, other: Origin) -> MultiVector {
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
        ).geometric_product(self.reverse());
    }
}
extension MultiVector: Sandwich<Plane> {
    func sandwich(self, other: Plane) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(self[e321] * other[e321], (self[e1] * other[e423]) + (self[e2] * other[e431]) + (self[e3] * other[e412]) + (self[e4] * other[e321])) * float2(-1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(other[e321], other[e321], other[e321], 1.0) * float4(self.group3(), -(self[e1234] * other[e321]) - (self[e23] * other[e423]) - (self[e31] * other[e431]) - (self[e12] * other[e412])), 
            /* e41, e42, e43 */
            (float3(other[e321]) * self.group4().xyz) + (self.group1().yzx * other.group0().zxy) - (float3(self[e321]) * other.group0().xyz) - (self.group1().zxy * other.group0().yzx), 
            /* e23, e31, e12 */
            float3(other[e321]) * self.group1().xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            float4(((float3(self[scalar]) * other.group0().xyz) + (self.group3().zxy * other.group0().yzx) - (float3(other[e321]) * self.group2()) - (self.group3().yzx * other.group0().zxy)), self[scalar] * other[e321])
        ).geometric_product(self.reverse());
    }
}
extension MultiVector: Sandwich<Point> {
    func sandwich(self, other: Point) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2((self[e1] * other[e1]) + (self[e2] * other[e2]) + (self[e3] * other[e3]), -(self[e423] * other[e1]) - (self[e431] * other[e2]) - (self[e412] * other[e3]) - (self[e321] * other[e4])), 
            /* e1, e2, e3, e4 */
            float4(self[e31] * other[e3] * -1.0, self[e12] * other[e1] * -1.0, self[e23] * other[e2] * -1.0, (self[e42] * other[e2]) + (self[e43] * other[e3])) + (float4(self[scalar]) * other.group0()) + (other.group0().yzxx * float4(self.group3().zxy, self[e41])), 
            /* e41, e42, e43 */
            (float3(self[e4]) * other.group0().xyz) + (self.group4().zxy * other.group0().yzx) - (float3(other[e4]) * self.group1().xyz) - (self.group4().yzx * other.group0().zxy), 
            /* e23, e31, e12 */
            (self.group1().yzx * other.group0().zxy) - (float3(self[e321]) * other.group0().xyz) - (self.group1().zxy * other.group0().yzx), 
            /* e423, e431, e412, e321 */
            float4((self[e42] * other[e3]) + (self[e23] * other[e4]), (self[e43] * other[e1]) + (self[e31] * other[e4]), (self[e41] * other[e2]) + (self[e12] * other[e4]), self[e12] * other[e3] * -1.0) - (other.group0().xyzx * float4(self.group0().yy, self[e1234], self[e23])) - (other.group0().yzxy * float4(self.group2().zxy, self[e31]))
        ).geometric_product(self.reverse());
    }
}
extension MultiVector: Sandwich<Scalar> {
    func sandwich(self, other: Scalar) -> MultiVector {
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
        ).geometric_product(self.reverse());
    }
}
extension Origin {
    func div(_rhs: sandwich) -> sandwich_partial<Origin> {
        sandwich_partial(this)
    }
}
extension Origin: Sandwich<Flector> {
    func sandwich(self, other: Flector) -> Flector {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(self[e4]) * float4(other.group0().xyz, other[e321]), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        ).geometric_product(self.reverse());
    }
}
extension Origin: Sandwich<Line> {
    func sandwich(self, other: Line) -> AntiScalar {
        return Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4(self[e4], self[e4], self[e4], 0.0) * float4(other.group1(), 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        ).geometric_product(self.reverse());
    }
}
extension Origin: Sandwich<Motor> {
    func sandwich(self, other: Motor) -> Motor {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other[scalar] * self[e4]), 
            /* e423, e431, e412, e321 */
            float4(self[e4], self[e4], self[e4], 0.0) * float4(other.group1().xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        ).geometric_product(self.reverse());
    }
}
extension Origin: Sandwich<MultiVector> {
    func sandwich(self, other: MultiVector) -> MultiVector {
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
        ).geometric_product(self.reverse());
    }
}
extension Origin: Sandwich<Point> {
    func sandwich(self, other: Point) -> Plane {
        return Line::from_groups(
            /* e41, e42, e43 */
            float3(self[e4]) * other.group0().xyz, 
            /* e23, e31, e12 */
            float3(0.0)
        ).geometric_product(self.reverse());
    }
}
extension Plane {
    func div(_rhs: sandwich) -> sandwich_partial<Plane> {
        sandwich_partial(this)
    }
}
extension Plane: Sandwich<AntiScalar> {
    func sandwich(self, other: AntiScalar) -> AntiScalar {
        return Origin::from_groups(
            /* e4 */
            other[e1234] * self[e321]
        ).geometric_product(self.reverse());
    }
}
extension Plane: Sandwich<DualNum> {
    func sandwich(self, other: DualNum) -> Motor {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other[e1234] * self[e321]), 
            /* e423, e431, e412, e321 */
            float4(other[scalar]) * self.group0()
        ).geometric_product(self.reverse());
    }
}
extension Plane: Sandwich<Flector> {
    func sandwich(self, other: Flector) -> Flector {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4((other[e2] * self[e412]) + (other[e321] * self[e423]), (other[e3] * self[e423]) + (other[e321] * self[e431]), (other[e1] * self[e431]) + (other[e321] * self[e412]), -(other[e3] * self[e412]) - (other[e4] * self[e321])) - (other.group0().zxyx * self.group0().yzxx) - (self.group0().wwwy * float4(other.group1().xyz, other[e2])), 
            /* e23, e31, e12, scalar */
            float4(self[e321]) * float4(other.group0().xyz, other[e321]) * float4(-1.0)
        ).geometric_product(self.reverse());
    }
}
extension Plane: Sandwich<Horizon> {
    func sandwich(self, other: Horizon) -> Flector {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(other[e321], other[e321], other[e321], 0.0) * float4(self.group0().xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e23, e31, e12, scalar */
            float4(float3(0.0), other[e321] * self[e321] * -1.0)
        ).geometric_product(self.reverse());
    }
}
extension Plane: Sandwich<Line> {
    func sandwich(self, other: Line) -> Motor {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(self[e321], self[e321], self[e321], 1.0) * float4(other.group1(), -(other[e23] * self[e423]) - (other[e31] * self[e431]) - (other[e12] * self[e412])), 
            /* e423, e431, e412, e321 */
            float4((float3(self[e321]) * other.group0()), 0.0) + float4((other.group1().yzx * self.group0().zxy), 0.0) - float4((other.group1().zxy * self.group0().yzx), 0.0)
        ).geometric_product(self.reverse());
    }
}
extension Plane: Sandwich<Motor> {
    func sandwich(self, other: Motor) -> Motor {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(self[e321], self[e321], self[e321], 1.0) * float4(other.group1().xyz, (other[e1234] * self[e321]) - (other[e23] * self[e423]) - (other[e31] * self[e431]) - (other[e12] * self[e412])), 
            /* e423, e431, e412, e321 */
            float4(((float3(other[scalar]) * self.group0().xyz) + (float3(self[e321]) * other.group0().xyz) + (other.group1().yzx * self.group0().zxy) - (other.group1().zxy * self.group0().yzx)), other[scalar] * self[e321])
        ).geometric_product(self.reverse());
    }
}
extension Plane: Sandwich<MultiVector> {
    func sandwich(self, other: MultiVector) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(other[e321] * self[e321], -(other[e1] * self[e423]) - (other[e2] * self[e431]) - (other[e3] * self[e412]) - (other[e4] * self[e321])) * float2(-1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(self[e321], self[e321], self[e321], 1.0) * float4(other.group3(), (other[e1234] * self[e321]) - (other[e23] * self[e423]) - (other[e31] * self[e431]) - (other[e12] * self[e412])), 
            /* e41, e42, e43 */
            (float3(other[e321]) * self.group0().xyz) + (other.group1().yzx * self.group0().zxy) - (float3(self[e321]) * other.group4().xyz) - (other.group1().zxy * self.group0().yzx), 
            /* e23, e31, e12 */
            float3(self[e321]) * other.group1().xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            float4(((float3(other[scalar]) * self.group0().xyz) + (float3(self[e321]) * other.group2()) + (other.group3().yzx * self.group0().zxy) - (other.group3().zxy * self.group0().yzx)), other[scalar] * self[e321])
        ).geometric_product(self.reverse());
    }
}
extension Plane: Sandwich<Origin> {
    func sandwich(self, other: Origin) -> Origin {
        return AntiScalar::from_groups(
            /* e1234 */
            other[e4] * self[e321] * -1.0
        ).geometric_product(self.reverse());
    }
}
extension Plane: Sandwich<Plane> {
    func sandwich(self, other: Plane) -> Flector {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(((float3(other[e321]) * self.group0().xyz) - (float3(self[e321]) * other.group0().xyz)), 0.0), 
            /* e23, e31, e12, scalar */
            float4(float3(0.0), other[e321] * self[e321] * -1.0)
        ).geometric_product(self.reverse());
    }
}
extension Plane: Sandwich<Point> {
    func sandwich(self, other: Point) -> Flector {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(self[e412] * other[e2], self[e423] * other[e3], self[e431] * other[e1], -(self[e431] * other[e2]) - (self[e412] * other[e3]) - (self[e321] * other[e4])) - (self.group0().yzxx * other.group0().zxyx), 
            /* e23, e31, e12, scalar */
            float4(float3(1.0), 0.0) * float4(self.group0().www, 0.0) * float4(other.group0().xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0)
        ).geometric_product(self.reverse());
    }
}
extension Plane: Sandwich<Scalar> {
    func sandwich(self, other: Scalar) -> Motor {
        return Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4(other[scalar]) * self.group0()
        ).geometric_product(self.reverse());
    }
}
extension Point {
    func div(_rhs: sandwich) -> sandwich_partial<Point> {
        sandwich_partial(this)
    }
}
extension Point: Sandwich<AntiScalar> {
    func sandwich(self, other: AntiScalar) -> Motor {
        return Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4(other[e1234], other[e1234], other[e1234], 0.0) * float4(self.group0().xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        ).geometric_product(self.reverse());
    }
}
extension Point: Sandwich<DualNum> {
    func sandwich(self, other: DualNum) -> Motor {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(other[scalar]) * self.group0(), 
            /* e423, e431, e412, e321 */
            float4(other.group0().yy, other[e1234], 0.0) * float4(float3(1.0), 0.0) * float4(self.group0().xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        ).geometric_product(self.reverse());
    }
}
extension Point: Sandwich<Flector> {
    func sandwich(self, other: Flector) -> Flector {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(-(other[e4] * self[e1]) - (other[e431] * self[e3]), -(other[e4] * self[e2]) - (other[e412] * self[e1]), -(other[e4] * self[e3]) - (other[e423] * self[e2]), (other[e412] * self[e3]) + (other[e321] * self[e4])) + (other.group1().zxyy * self.group0().yzxy) + (self.group0().wwwx * float4(other.group0().xyz, other[e423])), 
            /* e23, e31, e12, scalar */
            float4(-(other[e2] * self[e3]) - (other[e321] * self[e1]), -(other[e3] * self[e1]) - (other[e321] * self[e2]), -(other[e1] * self[e2]) - (other[e321] * self[e3]), (other[e2] * self[e2]) + (other[e3] * self[e3])) + (other.group0().zxyx * self.group0().yzxx)
        ).geometric_product(self.reverse());
    }
}
extension Point: Sandwich<Horizon> {
    func sandwich(self, other: Horizon) -> Flector {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(float3(0.0), other[e321] * self[e4]), 
            /* e23, e31, e12, scalar */
            float4(other[e321], other[e321], other[e321], 0.0) * float4(self.group0().xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0)
        ).geometric_product(self.reverse());
    }
}
extension Point: Sandwich<Line> {
    func sandwich(self, other: Line) -> Motor {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(other[e31] * self[e3], other[e12] * self[e1], other[e23] * self[e2], -(other[e42] * self[e2]) - (other[e43] * self[e3])) - (self.group0().yzxx * float4(other.group1().zxy, other[e41])), 
            /* e423, e431, e412, e321 */
            float4((other[e42] * self[e3]) + (other[e23] * self[e4]), (other[e43] * self[e1]) + (other[e31] * self[e4]), (other[e41] * self[e2]) + (other[e12] * self[e4]), -(other[e31] * self[e2]) - (other[e12] * self[e3])) - (self.group0().yzxx * float4(other.group0().zxy, other[e23]))
        ).geometric_product(self.reverse());
    }
}
extension Point: Sandwich<Motor> {
    func sandwich(self, other: Motor) -> Motor {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(other[scalar] * self[e1], other[scalar] * self[e2], other[scalar] * self[e3], -(other[e42] * self[e2]) - (other[e43] * self[e3])) + (other.group1().yzxw * self.group0().zxyw) - (self.group0().yzxx * float4(other.group1().zxy, other[e41])), 
            /* e423, e431, e412, e321 */
            float4((other[e42] * self[e3]) + (other[e1234] * self[e1]) + (other[e23] * self[e4]), (other[e43] * self[e1]) + (other[e1234] * self[e2]) + (other[e31] * self[e4]), (other[e41] * self[e2]) + (other[e1234] * self[e3]) + (other[e12] * self[e4]), -(other[e31] * self[e2]) - (other[e12] * self[e3])) - (self.group0().yzxx * float4(other.group0().zxy, other[e23]))
        ).geometric_product(self.reverse());
    }
}
extension Point: Sandwich<MultiVector> {
    func sandwich(self, other: MultiVector) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(0.0, other[e321] * self[e4]) + (float2(self[e1]) * float2(other[e1], other[e423])) + (float2(self[e2]) * float2(other[e2], other[e431])) + (float2(self[e3]) * float2(other[e3], other[e412])), 
            /* e1, e2, e3, e4 */
            float4(other[e31] * self[e3], other[e12] * self[e1], other[e23] * self[e2], -(other[e42] * self[e2]) - (other[e43] * self[e3])) + (float4(other[scalar]) * self.group0()) - (self.group0().yzxx * float4(other.group3().zxy, other[e41])), 
            /* e41, e42, e43 */
            (float3(self[e4]) * other.group1().xyz) + (other.group4().zxy * self.group0().yzx) - (float3(other[e4]) * self.group0().xyz) - (other.group4().yzx * self.group0().zxy), 
            /* e23, e31, e12 */
            (other.group1().zxy * self.group0().yzx) - (float3(other[e321]) * self.group0().xyz) - (other.group1().yzx * self.group0().zxy), 
            /* e423, e431, e412, e321 */
            float4((other[e1234] * self[e1]) + (other[e42] * self[e3]) + (other[e23] * self[e4]), (other[e1234] * self[e2]) + (other[e43] * self[e1]) + (other[e31] * self[e4]), (other[e1234] * self[e3]) + (other[e41] * self[e2]) + (other[e12] * self[e4]), -(other[e31] * self[e2]) - (other[e12] * self[e3])) - (self.group0().yzxx * float4(other.group2().zxy, other[e23]))
        ).geometric_product(self.reverse());
    }
}
extension Point: Sandwich<Origin> {
    func sandwich(self, other: Origin) -> Flector {
        return Line::from_groups(
            /* e41, e42, e43 */
            float3(other[e4]) * self.group0().xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        ).geometric_product(self.reverse());
    }
}
extension Point: Sandwich<Plane> {
    func sandwich(self, other: Plane) -> Flector {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(other[e431] * self[e3] * -1.0, other[e412] * self[e1] * -1.0, other[e423] * self[e2] * -1.0, (other[e431] * self[e2]) + (other[e412] * self[e3]) + (other[e321] * self[e4])) + (other.group0().zxyx * self.group0().yzxx), 
            /* e23, e31, e12, scalar */
            float4(float3(1.0), 0.0) * float4(other.group0().www, 0.0) * float4(self.group0().xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0)
        ).geometric_product(self.reverse());
    }
}
extension Point: Sandwich<Point> {
    func sandwich(self, other: Point) -> Flector {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(((float3(self[e4]) * other.group0().xyz) - (float3(other[e4]) * self.group0().xyz)), 0.0), 
            /* e23, e31, e12, scalar */
            float4(other[e2] * self[e3] * -1.0, other[e3] * self[e1] * -1.0, other[e1] * self[e2] * -1.0, (other[e2] * self[e2]) + (other[e3] * self[e3])) + (other.group0().zxyx * self.group0().yzxx)
        ).geometric_product(self.reverse());
    }
}
extension Point: Sandwich<Scalar> {
    func sandwich(self, other: Scalar) -> Motor {
        return Point::from_groups(
            /* e1, e2, e3, e4 */
            float4(other[scalar]) * self.group0()
        ).geometric_product(self.reverse());
    }
}
extension Scalar {
    func div(_rhs: sandwich) -> sandwich_partial<Scalar> {
        sandwich_partial(this)
    }
}
extension Scalar: Sandwich<AntiScalar> {
    func sandwich(self, other: AntiScalar) -> AntiScalar {
        return AntiScalar::from_groups(
            /* e1234 */
            other[e1234] * self[scalar]
        ).geometric_product(self.reverse());
    }
}
extension Scalar: Sandwich<DualNum> {
    func sandwich(self, other: DualNum) -> DualNum {
        return DualNum::from_groups(
            /* scalar, e1234 */
            float2(self[scalar]) * other.group0()
        ).geometric_product(self.reverse());
    }
}
extension Scalar: Sandwich<Flector> {
    func sandwich(self, other: Flector) -> Flector {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(self[scalar]) * other.group0(), 
            /* e423, e431, e412, e321 */
            float4(self[scalar]) * other.group1()
        ).geometric_product(self.reverse());
    }
}
extension Scalar: Sandwich<Horizon> {
    func sandwich(self, other: Horizon) -> Horizon {
        return Horizon::from_groups(
            /* e321 */
            other[e321] * self[scalar]
        ).geometric_product(self.reverse());
    }
}
extension Scalar: Sandwich<Line> {
    func sandwich(self, other: Line) -> Line {
        return Line::from_groups(
            /* e41, e42, e43 */
            float3(self[scalar]) * other.group0(), 
            /* e23, e31, e12 */
            float3(self[scalar]) * other.group1()
        ).geometric_product(self.reverse());
    }
}
extension Scalar: Sandwich<Motor> {
    func sandwich(self, other: Motor) -> Motor {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(self[scalar]) * other.group0(), 
            /* e23, e31, e12, scalar */
            float4(self[scalar]) * other.group1()
        ).geometric_product(self.reverse());
    }
}
extension Scalar: Sandwich<MultiVector> {
    func sandwich(self, other: MultiVector) -> MultiVector {
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
        ).geometric_product(self.reverse());
    }
}
extension Scalar: Sandwich<Origin> {
    func sandwich(self, other: Origin) -> Origin {
        return Origin::from_groups(
            /* e4 */
            other[e4] * self[scalar]
        ).geometric_product(self.reverse());
    }
}
extension Scalar: Sandwich<Plane> {
    func sandwich(self, other: Plane) -> Plane {
        return Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4(self[scalar]) * other.group0()
        ).geometric_product(self.reverse());
    }
}
extension Scalar: Sandwich<Point> {
    func sandwich(self, other: Point) -> Point {
        return Point::from_groups(
            /* e1, e2, e3, e4 */
            float4(self[scalar]) * other.group0()
        ).geometric_product(self.reverse());
    }
}
extension Scalar: Sandwich<Scalar> {
    func sandwich(self, other: Scalar) -> Scalar {
        return Scalar::from_groups(
            /* scalar */
            other[scalar] * self[scalar]
        ).geometric_product(self.reverse());
    }
}
