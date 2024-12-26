extension AntiScalar {
    func div(_rhs: geometric_quotient) -> geometric_quotient_partial<AntiScalar> {
        geometric_quotient_partial(this)
    }
}
extension AntiScalar: GeometricQuotient<DualNum> {
    func geometric_quotient(self, other: DualNum) -> AntiScalar {
        return AntiScalar::from_groups(
            /* e1234 */
            self[e1234] / (other[scalar])
        );
    }
}
extension AntiScalar: GeometricQuotient<Flector> {
    func geometric_quotient(self, other: Flector) -> Flector {
Flector reverse = Flector::from_groups(
            /* e1, e2, e3, e4 */
            other.group0(), 
            /* e423, e431, e412, e321 */
            other.group1() * float4(-1.0)
        );
Scalar other_2 = Scalar::from_groups(
            /* scalar */
            powi(other[e1], 2) + powi(other[e2], 2) + powi(other[e3], 2) + powi(other[e321], 2)
        );
Flector geometric_product = Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(other_2[scalar]) * reverse.group0(), 
            /* e423, e431, e412, e321 */
            float4(other_2[scalar]) * reverse.group1()
        );
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), self[e1234] * geometric_product[e321] * -1.0), 
            /* e423, e431, e412, e321 */
            float4(self[e1234], self[e1234], self[e1234], 0.0) * float4(geometric_product.group0().xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0)
        );
    }
}
extension AntiScalar: GeometricQuotient<Horizon> {
    func geometric_quotient(self, other: Horizon) -> Origin {
        return Origin::from_groups(
            /* e4 */
            self[e1234] / (other[e321])
        );
    }
}
extension AntiScalar: GeometricQuotient<Line> {
    func geometric_quotient(self, other: Line) -> Line {
Line reverse = Line::from_groups(
            /* e41, e42, e43 */
            other.group0() * float3(-1.0), 
            /* e23, e31, e12 */
            other.group1() * float3(-1.0)
        );
Scalar other_2 = Scalar::from_groups(
            /* scalar */
            powi(other[e23], 2) + powi(other[e31], 2) + powi(other[e12], 2)
        );
        return Line::from_groups(
            /* e41, e42, e43 */
            float3(self[e1234]) * float3(reverse[e23] * other_2[scalar], reverse[e31] * other_2[scalar], reverse[e12] * other_2[scalar]), 
            /* e23, e31, e12 */
            float3(0.0)
        );
    }
}
extension AntiScalar: GeometricQuotient<Motor> {
    func geometric_quotient(self, other: Motor) -> Motor {
Motor reverse = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            other.group0() * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            other.group1() * float4(-1.0, -1.0, -1.0, 1.0)
        );
Scalar other_2 = Scalar::from_groups(
            /* scalar */
            powi(other[e23], 2) + powi(other[e31], 2) + powi(other[e12], 2) + powi(other[scalar], 2)
        );
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(self[e1234]) * float4(reverse[e23] * other_2[scalar], reverse[e31] * other_2[scalar], reverse[e12] * other_2[scalar], reverse[scalar] * other_2[scalar]), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
extension AntiScalar: GeometricQuotient<MultiVector> {
    func geometric_quotient(self, other: MultiVector) -> MultiVector {
MultiVector reverse = MultiVector::from_groups(
            /* scalar, e1234 */
            other.group0(), 
            /* e1, e2, e3, e4 */
            other.group1(), 
            /* e41, e42, e43 */
            other.group2() * float3(-1.0), 
            /* e23, e31, e12 */
            other.group3() * float3(-1.0), 
            /* e423, e431, e412, e321 */
            other.group4() * float4(-1.0)
        );
Scalar other_2 = Scalar::from_groups(
            /* scalar */
            powi(other[scalar], 2) + powi(other[e1], 2) + powi(other[e2], 2) + powi(other[e3], 2) + powi(other[e23], 2) + powi(other[e31], 2) + powi(other[e12], 2) + powi(other[e321], 2)
        );
MultiVector geometric_product = MultiVector::from_groups(
            /* scalar, e1234 */
            float2(other_2[scalar]) * reverse.group0(), 
            /* e1, e2, e3, e4 */
            float4(other_2[scalar]) * reverse.group1(), 
            /* e41, e42, e43 */
            float3(other_2[scalar]) * reverse.group2(), 
            /* e23, e31, e12 */
            float3(other_2[scalar]) * reverse.group3(), 
            /* e423, e431, e412, e321 */
            float4(other_2[scalar]) * reverse.group4()
        );
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(1.0, self[e1234] * geometric_product[scalar]) * float2(0.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), self[e1234] * geometric_product[e321] * -1.0), 
            /* e41, e42, e43 */
            float3(self[e1234]) * geometric_product.group3(), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(self[e1234], self[e1234], self[e1234], 0.0) * float4(geometric_product.group1().xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0)
        );
    }
}
extension AntiScalar: GeometricQuotient<Plane> {
    func geometric_quotient(self, other: Plane) -> Origin {
        return Origin::from_groups(
            /* e4 */
            self[e1234] / (other[e321])
        );
    }
}
extension AntiScalar: GeometricQuotient<Point> {
    func geometric_quotient(self, other: Point) -> Plane {
Scalar other_2 = Scalar::from_groups(
            /* scalar */
            powi(other[e1], 2) + powi(other[e2], 2) + powi(other[e3], 2)
        );
        return Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4(self[e1234], self[e1234], self[e1234], 0.0) * float4(float4(other[e1] * other_2[scalar], other[e2] * other_2[scalar], other[e3] * other_2[scalar], other[e4] * other_2[scalar]).xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0)
        );
    }
}
extension AntiScalar: GeometricQuotient<Scalar> {
    func geometric_quotient(self, other: Scalar) -> AntiScalar {
        return AntiScalar::from_groups(
            /* e1234 */
            self[e1234] / (other[scalar])
        );
    }
}
extension DualNum {
    func div(_rhs: geometric_quotient) -> geometric_quotient_partial<DualNum> {
        geometric_quotient_partial(this)
    }
}
extension DualNum: GeometricQuotient<DualNum> {
    func geometric_quotient(self, other: DualNum) -> DualNum {
DualNum geometric_product = DualNum::from_groups(
            /* scalar, e1234 */
            float2(powi(other[scalar], -2)) * other.group0()
        );
        return DualNum::from_groups(
            /* scalar, e1234 */
            float2(geometric_product[scalar] * self[scalar], (geometric_product[scalar] * self[e1234]) + (geometric_product[e1234] * self[scalar]))
        );
    }
}
extension DualNum: GeometricQuotient<Flector> {
    func geometric_quotient(self, other: Flector) -> Flector {
Flector reverse = Flector::from_groups(
            /* e1, e2, e3, e4 */
            other.group0(), 
            /* e423, e431, e412, e321 */
            other.group1() * float4(-1.0)
        );
Scalar other_2 = Scalar::from_groups(
            /* scalar */
            powi(other[e1], 2) + powi(other[e2], 2) + powi(other[e3], 2) + powi(other[e321], 2)
        );
Flector geometric_product = Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(other_2[scalar]) * reverse.group0(), 
            /* e423, e431, e412, e321 */
            float4(other_2[scalar]) * reverse.group1()
        );
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(geometric_product[e1], geometric_product[e2], geometric_product[e3], 1.0) * float4(self.group0().xx, self[scalar], (self[scalar] * geometric_product[e4]) - (self[e1234] * geometric_product[e321])), 
            /* e423, e431, e412, e321 */
            float4(((float3(self[scalar]) * geometric_product.group1().xyz) - (float3(self[e1234]) * geometric_product.group0().xyz)), self[scalar] * geometric_product[e321])
        );
    }
}
extension DualNum: GeometricQuotient<Horizon> {
    func geometric_quotient(self, other: Horizon) -> Flector {
Horizon geometric_product = Horizon::from_groups(
            /* e321 */
            1.0/other[e321] * -1.0
        );
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), self[e1234] * geometric_product[e321] * -1.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), self[scalar] * geometric_product[e321])
        );
    }
}
extension DualNum: GeometricQuotient<Line> {
    func geometric_quotient(self, other: Line) -> Line {
Line reverse = Line::from_groups(
            /* e41, e42, e43 */
            other.group0() * float3(-1.0), 
            /* e23, e31, e12 */
            other.group1() * float3(-1.0)
        );
Scalar other_2 = Scalar::from_groups(
            /* scalar */
            powi(other[e23], 2) + powi(other[e31], 2) + powi(other[e12], 2)
        );
Line geometric_product = Line::from_groups(
            /* e41, e42, e43 */
            float3(other_2[scalar]) * reverse.group0(), 
            /* e23, e31, e12 */
            float3(other_2[scalar]) * reverse.group1()
        );
        return Line::from_groups(
            /* e41, e42, e43 */
            (float3(self[scalar]) * geometric_product.group0()) + (float3(self[e1234]) * geometric_product.group1()), 
            /* e23, e31, e12 */
            float3(self[scalar]) * geometric_product.group1()
        );
    }
}
extension DualNum: GeometricQuotient<Motor> {
    func geometric_quotient(self, other: Motor) -> Motor {
Motor reverse = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            other.group0() * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            other.group1() * float4(-1.0, -1.0, -1.0, 1.0)
        );
Scalar other_2 = Scalar::from_groups(
            /* scalar */
            powi(other[e23], 2) + powi(other[e31], 2) + powi(other[e12], 2) + powi(other[scalar], 2)
        );
Motor geometric_product = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(other_2[scalar]) * reverse.group0(), 
            /* e23, e31, e12, scalar */
            float4(other_2[scalar]) * reverse.group1()
        );
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            (float4(self[scalar]) * geometric_product.group0()) + (float4(self[e1234]) * geometric_product.group1()), 
            /* e23, e31, e12, scalar */
            float4(self[scalar]) * geometric_product.group1()
        );
    }
}
extension DualNum: GeometricQuotient<MultiVector> {
    func geometric_quotient(self, other: MultiVector) -> MultiVector {
MultiVector reverse = MultiVector::from_groups(
            /* scalar, e1234 */
            other.group0(), 
            /* e1, e2, e3, e4 */
            other.group1(), 
            /* e41, e42, e43 */
            other.group2() * float3(-1.0), 
            /* e23, e31, e12 */
            other.group3() * float3(-1.0), 
            /* e423, e431, e412, e321 */
            other.group4() * float4(-1.0)
        );
Scalar other_2 = Scalar::from_groups(
            /* scalar */
            powi(other[scalar], 2) + powi(other[e1], 2) + powi(other[e2], 2) + powi(other[e3], 2) + powi(other[e23], 2) + powi(other[e31], 2) + powi(other[e12], 2) + powi(other[e321], 2)
        );
MultiVector geometric_product = MultiVector::from_groups(
            /* scalar, e1234 */
            float2(other_2[scalar]) * reverse.group0(), 
            /* e1, e2, e3, e4 */
            float4(other_2[scalar]) * reverse.group1(), 
            /* e41, e42, e43 */
            float3(other_2[scalar]) * reverse.group2(), 
            /* e23, e31, e12 */
            float3(other_2[scalar]) * reverse.group3(), 
            /* e423, e431, e412, e321 */
            float4(other_2[scalar]) * reverse.group4()
        );
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(self[scalar] * geometric_product[scalar], (self[scalar] * geometric_product[e1234]) + (self[e1234] * geometric_product[scalar])), 
            /* e1, e2, e3, e4 */
            float4(geometric_product[e1], geometric_product[e2], geometric_product[e3], 1.0) * float4(self.group0().xx, self[scalar], (self[scalar] * geometric_product[e4]) - (self[e1234] * geometric_product[e321])), 
            /* e41, e42, e43 */
            (float3(self[scalar]) * geometric_product.group2()) + (float3(self[e1234]) * geometric_product.group3()), 
            /* e23, e31, e12 */
            float3(self[scalar]) * geometric_product.group3(), 
            /* e423, e431, e412, e321 */
            float4(((float3(self[scalar]) * geometric_product.group4().xyz) - (float3(self[e1234]) * geometric_product.group1().xyz)), self[scalar] * geometric_product[e321])
        );
    }
}
extension DualNum: GeometricQuotient<Plane> {
    func geometric_quotient(self, other: Plane) -> Flector {
Plane geometric_product = Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4(powi(other[e321], -2)) * float4(other[e423] * -1.0, other[e431] * -1.0, other[e412] * -1.0, other[e321] * -1.0)
        );
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), self[e1234] * geometric_product[e321] * -1.0), 
            /* e423, e431, e412, e321 */
            float4(self[scalar]) * geometric_product.group0()
        );
    }
}
extension DualNum: GeometricQuotient<Point> {
    func geometric_quotient(self, other: Point) -> Flector {
Point geometric_product = Point::from_groups(
            /* e1, e2, e3, e4 */
            float4(powi(other[e1], 2) + powi(other[e2], 2) + powi(other[e3], 2)) * other.group0()
        );
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(self[scalar]) * geometric_product.group0(), 
            /* e423, e431, e412, e321 */
            float4(self.group0().yy, self[e1234], 0.0) * float4(float3(1.0), 0.0) * float4(geometric_product.group0().xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0)
        );
    }
}
extension DualNum: GeometricQuotient<Scalar> {
    func geometric_quotient(self, other: Scalar) -> DualNum {
        return DualNum::from_groups(
            /* scalar, e1234 */
            float2(1.0/other[scalar]) * self.group0()
        );
    }
}
extension Flector {
    func div(_rhs: geometric_quotient) -> geometric_quotient_partial<Flector> {
        geometric_quotient_partial(this)
    }
}
extension Flector: GeometricQuotient<DualNum> {
    func geometric_quotient(self, other: DualNum) -> Flector {
DualNum geometric_product = DualNum::from_groups(
            /* scalar, e1234 */
            float2(powi(other[scalar], -2)) * other.group0()
        );
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(self[e1], self[e2], self[e3], 1.0) * float4(geometric_product.group0().xx, geometric_product[scalar], (geometric_product[scalar] * self[e4]) + (geometric_product[e1234] * self[e321])), 
            /* e423, e431, e412, e321 */
            float4(((float3(geometric_product[scalar]) * self.group1().xyz) + (float3(geometric_product[e1234]) * self.group0().xyz)), geometric_product[scalar] * self[e321])
        );
    }
}
extension Flector: GeometricQuotient<Flector> {
    func geometric_quotient(self, other: Flector) -> Motor {
Flector reverse = Flector::from_groups(
            /* e1, e2, e3, e4 */
            other.group0(), 
            /* e423, e431, e412, e321 */
            other.group1() * float4(-1.0)
        );
Scalar other_2 = Scalar::from_groups(
            /* scalar */
            powi(other[e1], 2) + powi(other[e2], 2) + powi(other[e3], 2) + powi(other[e321], 2)
        );
Flector geometric_product = Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(other_2[scalar]) * reverse.group0(), 
            /* e423, e431, e412, e321 */
            float4(other_2[scalar]) * reverse.group1()
        );
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            (float4(geometric_product[e321]) * float4(self.group1().xyz, self[e4])) + (geometric_product.group1().zxyz * self.group0().yzxz) + (float4(self.group0().ww, self[e431], self[e1]) * float4(geometric_product.group0().xyx, geometric_product[e423])) + (float4(self.group1().zx, self[e4], self[e2]) * float4(geometric_product.group0().yzz, geometric_product[e431])) - (geometric_product.group0().zxyx * self.group1().yzxx) - (geometric_product.group0().wwwy * float4(self.group0().xyz, self[e431])) - (float4(self.group0().zx, self[e321], self[e321]) * float4(geometric_product.group1().yzz, geometric_product[e4])) - (float4(self.group1().ww, self[e2], self[e412]) * float4(geometric_product.group1().xyx, geometric_product[e3])), 
            /* e23, e31, e12, scalar */
            float4(-(geometric_product[e2] * self[e3]) - (geometric_product[e321] * self[e1]), -(geometric_product[e3] * self[e1]) - (geometric_product[e321] * self[e2]), -(geometric_product[e3] * self[e321]) - (geometric_product[e321] * self[e3]), (geometric_product[e2] * self[e2]) + (geometric_product[e3] * self[e3])) + (geometric_product.group0().zxyx * self.group0().yzxx) - (float4(self.group1().ww, self[e2], self[e321]) * float4(geometric_product.group0().xyx, geometric_product[e321]))
        );
    }
}
extension Flector: GeometricQuotient<Horizon> {
    func geometric_quotient(self, other: Horizon) -> Motor {
Horizon geometric_product = Horizon::from_groups(
            /* e321 */
            1.0/other[e321] * -1.0
        );
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(geometric_product[e321]) * float4(self.group1().xyz, self[e4]), 
            /* e23, e31, e12, scalar */
            float4(geometric_product[e321]) * float4(self.group0().xyz, self[e321]) * float4(-1.0)
        );
    }
}
extension Flector: GeometricQuotient<Line> {
    func geometric_quotient(self, other: Line) -> Flector {
Line reverse = Line::from_groups(
            /* e41, e42, e43 */
            other.group0() * float3(-1.0), 
            /* e23, e31, e12 */
            other.group1() * float3(-1.0)
        );
Scalar other_2 = Scalar::from_groups(
            /* scalar */
            powi(other[e23], 2) + powi(other[e31], 2) + powi(other[e12], 2)
        );
Line geometric_product = Line::from_groups(
            /* e41, e42, e43 */
            float3(other_2[scalar]) * reverse.group0(), 
            /* e23, e31, e12 */
            float3(other_2[scalar]) * reverse.group1()
        );
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4((self[e3] * geometric_product[e31]) + (self[e321] * geometric_product[e23]), (self[e1] * geometric_product[e12]) + (self[e321] * geometric_product[e31]), (self[e2] * geometric_product[e23]) + (self[e321] * geometric_product[e12]), -(self[e2] * geometric_product[e42]) - (self[e3] * geometric_product[e43]) - (self[e423] * geometric_product[e23]) - (self[e431] * geometric_product[e31]) - (self[e412] * geometric_product[e12])) - (self.group0().yzxx * float4(geometric_product.group1().zxy, geometric_product[e41])), 
            /* e423, e431, e412, e321 */
            float4((self[e3] * geometric_product[e42]) + (self[e4] * geometric_product[e23]) + (self[e412] * geometric_product[e31]) + (self[e321] * geometric_product[e41]), (self[e1] * geometric_product[e43]) + (self[e4] * geometric_product[e31]) + (self[e423] * geometric_product[e12]) + (self[e321] * geometric_product[e42]), (self[e2] * geometric_product[e41]) + (self[e4] * geometric_product[e12]) + (self[e431] * geometric_product[e23]) + (self[e321] * geometric_product[e43]), self[e3] * geometric_product[e12] * -1.0) - (self.group0().yzxx * float4(geometric_product.group0().zxy, geometric_product[e23])) - float4((geometric_product.group1().zxy * self.group1().yzx), self[e2] * geometric_product[e31])
        );
    }
}
extension Flector: GeometricQuotient<Motor> {
    func geometric_quotient(self, other: Motor) -> Flector {
Motor reverse = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            other.group0() * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            other.group1() * float4(-1.0, -1.0, -1.0, 1.0)
        );
Scalar other_2 = Scalar::from_groups(
            /* scalar */
            powi(other[e23], 2) + powi(other[e31], 2) + powi(other[e12], 2) + powi(other[scalar], 2)
        );
Motor geometric_product = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(other_2[scalar]) * reverse.group0(), 
            /* e23, e31, e12, scalar */
            float4(other_2[scalar]) * reverse.group1()
        );
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(self[e321] * geometric_product[e23], self[e321] * geometric_product[e31], self[e321] * geometric_product[e12], -(self[e2] * geometric_product[e42]) - (self[e3] * geometric_product[e43]) - (self[e423] * geometric_product[e23]) - (self[e431] * geometric_product[e31]) - (self[e412] * geometric_product[e12])) + (self.group0().xxyw * geometric_product.group1().wzxw) + float4((self.group0().zyz * geometric_product.group1().yww), self[e321] * geometric_product[e1234]) - (self.group0().yzxx * float4(geometric_product.group1().zxy, geometric_product[e41])), 
            /* e423, e431, e412, e321 */
            float4((self[e3] * geometric_product[e42]) + (self[e4] * geometric_product[e23]) + (self[e423] * geometric_product[scalar]) + (self[e412] * geometric_product[e31]) + (self[e321] * geometric_product[e41]), (self[e2] * geometric_product[e1234]) + (self[e4] * geometric_product[e31]) + (self[e423] * geometric_product[e12]) + (self[e431] * geometric_product[scalar]) + (self[e321] * geometric_product[e42]), (self[e3] * geometric_product[e1234]) + (self[e4] * geometric_product[e12]) + (self[e431] * geometric_product[e23]) + (self[e412] * geometric_product[scalar]) + (self[e321] * geometric_product[e43]), self[e3] * geometric_product[e12] * -1.0) + float4((self.group0().xxy * geometric_product.group0().wzx), self[e321] * geometric_product[scalar]) - (self.group0().yzxx * float4(geometric_product.group0().zxy, geometric_product[e23])) - (geometric_product.group1().zxyy * float4(self.group1().yzx, self[e2]))
        );
    }
}
extension Flector: GeometricQuotient<MultiVector> {
    func geometric_quotient(self, other: MultiVector) -> MultiVector {
MultiVector reverse = MultiVector::from_groups(
            /* scalar, e1234 */
            other.group0(), 
            /* e1, e2, e3, e4 */
            other.group1(), 
            /* e41, e42, e43 */
            other.group2() * float3(-1.0), 
            /* e23, e31, e12 */
            other.group3() * float3(-1.0), 
            /* e423, e431, e412, e321 */
            other.group4() * float4(-1.0)
        );
Scalar other_2 = Scalar::from_groups(
            /* scalar */
            powi(other[scalar], 2) + powi(other[e1], 2) + powi(other[e2], 2) + powi(other[e3], 2) + powi(other[e23], 2) + powi(other[e31], 2) + powi(other[e12], 2) + powi(other[e321], 2)
        );
MultiVector geometric_product = MultiVector::from_groups(
            /* scalar, e1234 */
            float2(other_2[scalar]) * reverse.group0(), 
            /* e1, e2, e3, e4 */
            float4(other_2[scalar]) * reverse.group1(), 
            /* e41, e42, e43 */
            float3(other_2[scalar]) * reverse.group2(), 
            /* e23, e31, e12 */
            float3(other_2[scalar]) * reverse.group3(), 
            /* e423, e431, e412, e321 */
            float4(other_2[scalar]) * reverse.group4()
        );
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(0.0, (self[e4] * geometric_product[e321]) - (self[e431] * geometric_product[e2]) - (self[e412] * geometric_product[e3]) - (self[e321] * geometric_product[e4])) + (float2(self[e1]) * float2(geometric_product[e1], geometric_product[e423])) + (float2(self[e2]) * float2(geometric_product[e2], geometric_product[e431])) + (float2(self[e3]) * float2(geometric_product[e3], geometric_product[e412])) - (float2(geometric_product[e321], geometric_product[e1]) * self.group1().wx), 
            /* e1, e2, e3, e4 */
            float4(self[e3] * geometric_product[e31], self[e1] * geometric_product[e12], self[e321] * geometric_product[e12], -(self[e2] * geometric_product[e42]) - (self[e3] * geometric_product[e43]) - (self[e423] * geometric_product[e23]) - (self[e431] * geometric_product[e31]) - (self[e412] * geometric_product[e12])) + (float4(geometric_product[scalar]) * self.group0()) + (float4(self.group1().ww, self[e2], self[e321]) * float4(geometric_product.group3().xyx, geometric_product[e1234])) - (self.group0().yzxx * float4(geometric_product.group3().zxy, geometric_product[e41])), 
            /* e41, e42, e43 */
            (float3(self[e4]) * geometric_product.group1().xyz) + (float3(geometric_product[e2], geometric_product[e321], geometric_product[e321]) * self.group1().zyz) + (float3(geometric_product[e321], geometric_product[e3], geometric_product[e1]) * self.group1().xxy) + (self.group0().yzx * geometric_product.group4().zxy) - (float3(self[e321]) * geometric_product.group4().xyz) - (float3(geometric_product[e4], geometric_product[e412], geometric_product[e423]) * self.group0().xxy) - (float3(geometric_product[e431], geometric_product[e4], geometric_product[e4]) * self.group0().zyz) - (self.group1().yzx * geometric_product.group1().zxy), 
            /* e23, e31, e12 */
            (self.group0().yzx * geometric_product.group1().zxy) - (float3(self[e321]) * geometric_product.group1().xyz) - (float3(geometric_product[e2], geometric_product[e321], geometric_product[e321]) * self.group0().zyz) - (float3(geometric_product[e321], geometric_product[e3], geometric_product[e1]) * self.group0().xxy), 
            /* e423, e431, e412, e321 */
            float4((self[e1] * geometric_product[e1234]) + (self[e3] * geometric_product[e42]) + (self[e4] * geometric_product[e23]) + (self[e412] * geometric_product[e31]) + (self[e321] * geometric_product[e41]), (self[e1] * geometric_product[e43]) + (self[e2] * geometric_product[e1234]) + (self[e4] * geometric_product[e31]) + (self[e423] * geometric_product[e12]) + (self[e321] * geometric_product[e42]), (self[e2] * geometric_product[e41]) + (self[e3] * geometric_product[e1234]) + (self[e4] * geometric_product[e12]) + (self[e431] * geometric_product[e23]) + (self[e321] * geometric_product[e43]), self[e3] * geometric_product[e12] * -1.0) + (float4(geometric_product[scalar]) * self.group1()) - (self.group0().yzxx * float4(geometric_product.group2().zxy, geometric_product[e23])) - float4((geometric_product.group3().zxy * self.group1().yzx), self[e2] * geometric_product[e31])
        );
    }
}
extension Flector: GeometricQuotient<Plane> {
    func geometric_quotient(self, other: Plane) -> Motor {
Plane geometric_product = Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4(powi(other[e321], -2)) * float4(other[e423] * -1.0, other[e431] * -1.0, other[e412] * -1.0, other[e321] * -1.0)
        );
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(-(self[e3] * geometric_product[e431]) - (self[e321] * geometric_product[e423]), -(self[e1] * geometric_product[e412]) - (self[e321] * geometric_product[e431]), -(self[e2] * geometric_product[e423]) - (self[e321] * geometric_product[e412]), (self[e3] * geometric_product[e412]) + (self[e4] * geometric_product[e321])) + (self.group0().yzxx * geometric_product.group0().zxyx) + (geometric_product.group0().wwwy * float4(self.group1().xyz, self[e2])), 
            /* e23, e31, e12, scalar */
            float4(geometric_product[e321]) * float4(self.group0().xyz, self[e321]) * float4(-1.0)
        );
    }
}
extension Flector: GeometricQuotient<Point> {
    func geometric_quotient(self, other: Point) -> Motor {
Point geometric_product = Point::from_groups(
            /* e1, e2, e3, e4 */
            float4(powi(other[e1], 2) + powi(other[e2], 2) + powi(other[e3], 2)) * other.group0()
        );
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4((self[e4] * geometric_product[e1]) + (self[e412] * geometric_product[e2]), (self[e4] * geometric_product[e2]) + (self[e423] * geometric_product[e3]), (self[e4] * geometric_product[e3]) + (self[e431] * geometric_product[e1]), -(self[e412] * geometric_product[e3]) - (self[e321] * geometric_product[e4])) - (self.group1().yzxy * geometric_product.group0().zxyy) - (geometric_product.group0().wwwx * float4(self.group0().xyz, self[e423])), 
            /* e23, e31, e12, scalar */
            float4(-(self[e3] * geometric_product[e2]) - (self[e321] * geometric_product[e1]), -(self[e1] * geometric_product[e3]) - (self[e321] * geometric_product[e2]), -(self[e2] * geometric_product[e1]) - (self[e321] * geometric_product[e3]), (self[e2] * geometric_product[e2]) + (self[e3] * geometric_product[e3])) + (self.group0().yzxx * geometric_product.group0().zxyx)
        );
    }
}
extension Flector: GeometricQuotient<Scalar> {
    func geometric_quotient(self, other: Scalar) -> Flector {
Scalar geometric_product = Scalar::from_groups(
            /* scalar */
            1.0/other[scalar]
        );
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(geometric_product[scalar]) * self.group0(), 
            /* e423, e431, e412, e321 */
            float4(geometric_product[scalar]) * self.group1()
        );
    }
}
extension Horizon {
    func div(_rhs: geometric_quotient) -> geometric_quotient_partial<Horizon> {
        geometric_quotient_partial(this)
    }
}
extension Horizon: GeometricQuotient<DualNum> {
    func geometric_quotient(self, other: DualNum) -> Flector {
DualNum geometric_product = DualNum::from_groups(
            /* scalar, e1234 */
            float2(powi(other[scalar], -2)) * other.group0()
        );
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), geometric_product[e1234] * self[e321]), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), geometric_product[scalar] * self[e321])
        );
    }
}
extension Horizon: GeometricQuotient<Flector> {
    func geometric_quotient(self, other: Flector) -> Motor {
Flector reverse = Flector::from_groups(
            /* e1, e2, e3, e4 */
            other.group0(), 
            /* e423, e431, e412, e321 */
            other.group1() * float4(-1.0)
        );
Scalar other_2 = Scalar::from_groups(
            /* scalar */
            powi(other[e1], 2) + powi(other[e2], 2) + powi(other[e3], 2) + powi(other[e321], 2)
        );
Flector geometric_product = Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(other_2[scalar]) * reverse.group0(), 
            /* e423, e431, e412, e321 */
            float4(other_2[scalar]) * reverse.group1()
        );
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(self[e321]) * float4(geometric_product.group1().xyz, geometric_product[e4]) * float4(-1.0), 
            /* e23, e31, e12, scalar */
            float4(self[e321]) * float4(geometric_product.group0().xyz, geometric_product[e321]) * float4(-1.0)
        );
    }
}
extension Horizon: GeometricQuotient<Horizon> {
    func geometric_quotient(self, other: Horizon) -> Scalar {
        return Scalar::from_groups(
            /* scalar */
            self[e321] / (other[e321])
        );
    }
}
extension Horizon: GeometricQuotient<Line> {
    func geometric_quotient(self, other: Line) -> Flector {
Line reverse = Line::from_groups(
            /* e41, e42, e43 */
            other.group0() * float3(-1.0), 
            /* e23, e31, e12 */
            other.group1() * float3(-1.0)
        );
Scalar other_2 = Scalar::from_groups(
            /* scalar */
            powi(other[e23], 2) + powi(other[e31], 2) + powi(other[e12], 2)
        );
Line geometric_product = Line::from_groups(
            /* e41, e42, e43 */
            float3(other_2[scalar]) * reverse.group0(), 
            /* e23, e31, e12 */
            float3(other_2[scalar]) * reverse.group1()
        );
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(self[e321], self[e321], self[e321], 0.0) * float4(geometric_product.group1(), 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e423, e431, e412, e321 */
            float4(self[e321], self[e321], self[e321], 0.0) * float4(geometric_product.group0(), 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Horizon: GeometricQuotient<Motor> {
    func geometric_quotient(self, other: Motor) -> Flector {
Motor reverse = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            other.group0() * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            other.group1() * float4(-1.0, -1.0, -1.0, 1.0)
        );
Scalar other_2 = Scalar::from_groups(
            /* scalar */
            powi(other[e23], 2) + powi(other[e31], 2) + powi(other[e12], 2) + powi(other[scalar], 2)
        );
Motor geometric_product = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(other_2[scalar]) * reverse.group0(), 
            /* e23, e31, e12, scalar */
            float4(other_2[scalar]) * reverse.group1()
        );
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(self[e321]) * float4(geometric_product.group1().xyz, geometric_product[e1234]), 
            /* e423, e431, e412, e321 */
            float4(self[e321]) * float4(geometric_product.group0().xyz, geometric_product[scalar])
        );
    }
}
extension Horizon: GeometricQuotient<MultiVector> {
    func geometric_quotient(self, other: MultiVector) -> MultiVector {
MultiVector reverse = MultiVector::from_groups(
            /* scalar, e1234 */
            other.group0(), 
            /* e1, e2, e3, e4 */
            other.group1(), 
            /* e41, e42, e43 */
            other.group2() * float3(-1.0), 
            /* e23, e31, e12 */
            other.group3() * float3(-1.0), 
            /* e423, e431, e412, e321 */
            other.group4() * float4(-1.0)
        );
Scalar other_2 = Scalar::from_groups(
            /* scalar */
            powi(other[scalar], 2) + powi(other[e1], 2) + powi(other[e2], 2) + powi(other[e3], 2) + powi(other[e23], 2) + powi(other[e31], 2) + powi(other[e12], 2) + powi(other[e321], 2)
        );
MultiVector geometric_product = MultiVector::from_groups(
            /* scalar, e1234 */
            float2(other_2[scalar]) * reverse.group0(), 
            /* e1, e2, e3, e4 */
            float4(other_2[scalar]) * reverse.group1(), 
            /* e41, e42, e43 */
            float3(other_2[scalar]) * reverse.group2(), 
            /* e23, e31, e12 */
            float3(other_2[scalar]) * reverse.group3(), 
            /* e423, e431, e412, e321 */
            float4(other_2[scalar]) * reverse.group4()
        );
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(self[e321]) * float2(geometric_product[e321], geometric_product[e4]) * float2(-1.0), 
            /* e1, e2, e3, e4 */
            float4(self[e321]) * float4(geometric_product.group3(), geometric_product[e1234]), 
            /* e41, e42, e43 */
            float3(self[e321]) * geometric_product.group4().xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(self[e321]) * geometric_product.group1().xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            float4(self[e321]) * float4(geometric_product.group2(), geometric_product[scalar])
        );
    }
}
extension Horizon: GeometricQuotient<Plane> {
    func geometric_quotient(self, other: Plane) -> Motor {
Plane geometric_product = Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4(powi(other[e321], -2)) * float4(other[e423] * -1.0, other[e431] * -1.0, other[e412] * -1.0, other[e321] * -1.0)
        );
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(self[e321], self[e321], self[e321], 0.0) * float4(geometric_product.group0().xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0), 
            /* e23, e31, e12, scalar */
            float4(float3(0.0), self[e321] * geometric_product[e321] * -1.0)
        );
    }
}
extension Horizon: GeometricQuotient<Point> {
    func geometric_quotient(self, other: Point) -> Motor {
Point geometric_product = Point::from_groups(
            /* e1, e2, e3, e4 */
            float4(powi(other[e1], 2) + powi(other[e2], 2) + powi(other[e3], 2)) * other.group0()
        );
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(float3(0.0), self[e321] * geometric_product[e4] * -1.0), 
            /* e23, e31, e12, scalar */
            float4(self[e321], self[e321], self[e321], 0.0) * float4(geometric_product.group0().xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0)
        );
    }
}
extension Horizon: GeometricQuotient<Scalar> {
    func geometric_quotient(self, other: Scalar) -> Horizon {
        return Horizon::from_groups(
            /* e321 */
            self[e321] / (other[scalar])
        );
    }
}
extension Line {
    func div(_rhs: geometric_quotient) -> geometric_quotient_partial<Line> {
        geometric_quotient_partial(this)
    }
}
extension Line: GeometricQuotient<DualNum> {
    func geometric_quotient(self, other: DualNum) -> Line {
DualNum geometric_product = DualNum::from_groups(
            /* scalar, e1234 */
            float2(powi(other[scalar], -2)) * other.group0()
        );
        return Line::from_groups(
            /* e41, e42, e43 */
            (float3(geometric_product[scalar]) * self.group0()) + (float3(geometric_product[e1234]) * self.group1()), 
            /* e23, e31, e12 */
            float3(geometric_product[scalar]) * self.group1()
        );
    }
}
extension Line: GeometricQuotient<Flector> {
    func geometric_quotient(self, other: Flector) -> Flector {
Flector reverse = Flector::from_groups(
            /* e1, e2, e3, e4 */
            other.group0(), 
            /* e423, e431, e412, e321 */
            other.group1() * float4(-1.0)
        );
Scalar other_2 = Scalar::from_groups(
            /* scalar */
            powi(other[e1], 2) + powi(other[e2], 2) + powi(other[e3], 2) + powi(other[e321], 2)
        );
Flector geometric_product = Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(other_2[scalar]) * reverse.group0(), 
            /* e423, e431, e412, e321 */
            float4(other_2[scalar]) * reverse.group1()
        );
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            (float4(geometric_product[e2], geometric_product[e321], geometric_product[e321], geometric_product[e2]) * float4(self.group1().zyz, self[e42])) + (float4(geometric_product[e321], geometric_product[e3], geometric_product[e1], geometric_product[e1]) * float4(self.group1().xxy, self[e41])) + float4(float3(0.0), (geometric_product[e3] * self[e43]) - (geometric_product[e431] * self[e31]) - (geometric_product[e412] * self[e12])) - float4((self.group1().yzx * geometric_product.group0().zxy), geometric_product[e423] * self[e23]), 
            /* e423, e431, e412, e321 */
            float4((geometric_product[e3] * self[e42]) + (geometric_product[e4] * self[e23]) + (geometric_product[e431] * self[e12]), (geometric_product[e1] * self[e43]) + (geometric_product[e4] * self[e31]) + (geometric_product[e412] * self[e23]), (geometric_product[e2] * self[e41]) + (geometric_product[e4] * self[e12]) + (geometric_product[e423] * self[e31]), 0.0) - (float4(geometric_product[e2], geometric_product[e321], geometric_product[e321], geometric_product[e2]) * float4(self.group0().zyz, self[e31])) - (float4(geometric_product[e321], geometric_product[e3], geometric_product[e1], geometric_product[e1]) * float4(self.group0().xxy, self[e23])) - float4((self.group1().yzx * geometric_product.group1().zxy), geometric_product[e3] * self[e12])
        );
    }
}
extension Line: GeometricQuotient<Horizon> {
    func geometric_quotient(self, other: Horizon) -> Flector {
Horizon geometric_product = Horizon::from_groups(
            /* e321 */
            1.0/other[e321] * -1.0
        );
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(geometric_product[e321], geometric_product[e321], geometric_product[e321], 0.0) * float4(self.group1(), 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e423, e431, e412, e321 */
            float4(geometric_product[e321], geometric_product[e321], geometric_product[e321], 0.0) * float4(self.group0(), 0.0) * float4(-1.0, -1.0, -1.0, 0.0)
        );
    }
}
extension Line: GeometricQuotient<Line> {
    func geometric_quotient(self, other: Line) -> Motor {
Line reverse = Line::from_groups(
            /* e41, e42, e43 */
            other.group0() * float3(-1.0), 
            /* e23, e31, e12 */
            other.group1() * float3(-1.0)
        );
Scalar other_2 = Scalar::from_groups(
            /* scalar */
            powi(other[e23], 2) + powi(other[e31], 2) + powi(other[e12], 2)
        );
Line geometric_product = Line::from_groups(
            /* e41, e42, e43 */
            float3(other_2[scalar]) * reverse.group0(), 
            /* e23, e31, e12 */
            float3(other_2[scalar]) * reverse.group1()
        );
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4((geometric_product[e42] * self[e12]) + (geometric_product[e31] * self[e43]), (geometric_product[e43] * self[e23]) + (geometric_product[e12] * self[e41]), (geometric_product[e41] * self[e31]) + (geometric_product[e23] * self[e42]), -(geometric_product[e43] * self[e12]) - (geometric_product[e23] * self[e41]) - (geometric_product[e31] * self[e42]) - (geometric_product[e12] * self[e43])) - float4((geometric_product.group0().zxy * self.group1().yzx), geometric_product[e41] * self[e23]) - float4((geometric_product.group1().zxy * self.group0().yzx), geometric_product[e42] * self[e31]), 
            /* e23, e31, e12, scalar */
            float4(geometric_product[e31] * self[e12], geometric_product[e12] * self[e23], geometric_product[e23] * self[e31], -(geometric_product[e31] * self[e31]) - (geometric_product[e12] * self[e12])) - float4((geometric_product.group1().zxy * self.group1().yzx), geometric_product[e23] * self[e23])
        );
    }
}
extension Line: GeometricQuotient<Motor> {
    func geometric_quotient(self, other: Motor) -> Motor {
Motor reverse = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            other.group0() * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            other.group1() * float4(-1.0, -1.0, -1.0, 1.0)
        );
Scalar other_2 = Scalar::from_groups(
            /* scalar */
            powi(other[e23], 2) + powi(other[e31], 2) + powi(other[e12], 2) + powi(other[scalar], 2)
        );
Motor geometric_product = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(other_2[scalar]) * reverse.group0(), 
            /* e23, e31, e12, scalar */
            float4(other_2[scalar]) * reverse.group1()
        );
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4((self[e41] * geometric_product[scalar]) + (self[e43] * geometric_product[e31]) + (self[e23] * geometric_product[e1234]) + (self[e12] * geometric_product[e42]), (self[e41] * geometric_product[e12]) + (self[e42] * geometric_product[scalar]) + (self[e23] * geometric_product[e43]) + (self[e31] * geometric_product[e1234]), (self[e42] * geometric_product[e23]) + (self[e43] * geometric_product[scalar]) + (self[e31] * geometric_product[e41]) + (self[e12] * geometric_product[e1234]), -(self[e43] * geometric_product[e12]) - (self[e23] * geometric_product[e41]) - (self[e31] * geometric_product[e42]) - (self[e12] * geometric_product[e43])) - (geometric_product.group1().zxyx * float4(self.group0().yzx, self[e41])) - float4((self.group1().yzx * geometric_product.group0().zxy), self[e42] * geometric_product[e31]), 
            /* e23, e31, e12, scalar */
            float4((self[e23] * geometric_product[scalar]) + (self[e12] * geometric_product[e31]), (self[e23] * geometric_product[e12]) + (self[e31] * geometric_product[scalar]), (self[e31] * geometric_product[e23]) + (self[e12] * geometric_product[scalar]), -(self[e31] * geometric_product[e31]) - (self[e12] * geometric_product[e12])) - (geometric_product.group1().zxyx * float4(self.group1().yzx, self[e23]))
        );
    }
}
extension Line: GeometricQuotient<MultiVector> {
    func geometric_quotient(self, other: MultiVector) -> MultiVector {
MultiVector reverse = MultiVector::from_groups(
            /* scalar, e1234 */
            other.group0(), 
            /* e1, e2, e3, e4 */
            other.group1(), 
            /* e41, e42, e43 */
            other.group2() * float3(-1.0), 
            /* e23, e31, e12 */
            other.group3() * float3(-1.0), 
            /* e423, e431, e412, e321 */
            other.group4() * float4(-1.0)
        );
Scalar other_2 = Scalar::from_groups(
            /* scalar */
            powi(other[scalar], 2) + powi(other[e1], 2) + powi(other[e2], 2) + powi(other[e3], 2) + powi(other[e23], 2) + powi(other[e31], 2) + powi(other[e12], 2) + powi(other[e321], 2)
        );
MultiVector geometric_product = MultiVector::from_groups(
            /* scalar, e1234 */
            float2(other_2[scalar]) * reverse.group0(), 
            /* e1, e2, e3, e4 */
            float4(other_2[scalar]) * reverse.group1(), 
            /* e41, e42, e43 */
            float3(other_2[scalar]) * reverse.group2(), 
            /* e23, e31, e12 */
            float3(other_2[scalar]) * reverse.group3(), 
            /* e423, e431, e412, e321 */
            float4(other_2[scalar]) * reverse.group4()
        );
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(0.0, -(self[e23] * geometric_product[e41]) - (self[e31] * geometric_product[e42]) - (self[e12] * geometric_product[e43])) - (float2(geometric_product[e23]) * float2(self[e23], self[e41])) - (float2(geometric_product[e31]) * float2(self[e31], self[e42])) - (float2(geometric_product[e12]) * float2(self[e12], self[e43])), 
            /* e1, e2, e3, e4 */
            (float4(geometric_product[e2], geometric_product[e321], geometric_product[e321], geometric_product[e2]) * float4(self.group1().zyz, self[e42])) + (float4(geometric_product[e321], geometric_product[e3], geometric_product[e1], geometric_product[e1]) * float4(self.group1().xxy, self[e41])) + float4(float3(0.0), (self[e43] * geometric_product[e3]) - (self[e31] * geometric_product[e431]) - (self[e12] * geometric_product[e412])) - float4((self.group1().yzx * geometric_product.group1().zxy), self[e23] * geometric_product[e423]), 
            /* e41, e42, e43 */
            (float3(geometric_product[scalar]) * self.group0()) + (float3(geometric_product[e1234]) * self.group1()) + (self.group0().zxy * geometric_product.group3().yzx) + (self.group1().zxy * geometric_product.group2().yzx) - (self.group0().yzx * geometric_product.group3().zxy) - (self.group1().yzx * geometric_product.group2().zxy), 
            /* e23, e31, e12 */
            (float3(geometric_product[scalar]) * self.group1()) + (self.group1().zxy * geometric_product.group3().yzx) - (self.group1().yzx * geometric_product.group3().zxy), 
            /* e423, e431, e412, e321 */
            float4((self[e42] * geometric_product[e3]) + (self[e23] * geometric_product[e4]) + (self[e12] * geometric_product[e431]), (self[e43] * geometric_product[e1]) + (self[e23] * geometric_product[e412]) + (self[e31] * geometric_product[e4]), (self[e41] * geometric_product[e2]) + (self[e31] * geometric_product[e423]) + (self[e12] * geometric_product[e4]), 0.0) - (float4(geometric_product[e2], geometric_product[e321], geometric_product[e321], geometric_product[e2]) * float4(self.group0().zyz, self[e31])) - (float4(geometric_product[e321], geometric_product[e3], geometric_product[e1], geometric_product[e1]) * float4(self.group0().xxy, self[e23])) - float4((self.group1().yzx * geometric_product.group4().zxy), self[e12] * geometric_product[e3])
        );
    }
}
extension Line: GeometricQuotient<Plane> {
    func geometric_quotient(self, other: Plane) -> Flector {
Plane geometric_product = Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4(powi(other[e321], -2)) * float4(other[e423] * -1.0, other[e431] * -1.0, other[e412] * -1.0, other[e321] * -1.0)
        );
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(geometric_product[e321], geometric_product[e321], geometric_product[e321], 1.0) * float4(self.group1(), -(self[e23] * geometric_product[e423]) - (self[e31] * geometric_product[e431]) - (self[e12] * geometric_product[e412])), 
            /* e423, e431, e412, e321 */
            float4((self.group1().zxy * geometric_product.group0().yzx), 0.0) - float4((float3(geometric_product[e321]) * self.group0()), 0.0) - float4((self.group1().yzx * geometric_product.group0().zxy), 0.0)
        );
    }
}
extension Line: GeometricQuotient<Point> {
    func geometric_quotient(self, other: Point) -> Flector {
Point geometric_product = Point::from_groups(
            /* e1, e2, e3, e4 */
            float4(powi(other[e1], 2) + powi(other[e2], 2) + powi(other[e3], 2)) * other.group0()
        );
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(self[e31] * geometric_product[e3] * -1.0, self[e12] * geometric_product[e1] * -1.0, self[e23] * geometric_product[e2] * -1.0, (self[e42] * geometric_product[e2]) + (self[e43] * geometric_product[e3])) + (geometric_product.group0().yzxx * float4(self.group1().zxy, self[e41])), 
            /* e423, e431, e412, e321 */
            float4((self[e42] * geometric_product[e3]) + (self[e23] * geometric_product[e4]), (self[e43] * geometric_product[e1]) + (self[e31] * geometric_product[e4]), (self[e41] * geometric_product[e2]) + (self[e12] * geometric_product[e4]), -(self[e31] * geometric_product[e2]) - (self[e12] * geometric_product[e3])) - (geometric_product.group0().yzxx * float4(self.group0().zxy, self[e23]))
        );
    }
}
extension Line: GeometricQuotient<Scalar> {
    func geometric_quotient(self, other: Scalar) -> Line {
Scalar geometric_product = Scalar::from_groups(
            /* scalar */
            1.0/other[scalar]
        );
        return Line::from_groups(
            /* e41, e42, e43 */
            float3(geometric_product[scalar]) * self.group0(), 
            /* e23, e31, e12 */
            float3(geometric_product[scalar]) * self.group1()
        );
    }
}
extension Motor {
    func div(_rhs: geometric_quotient) -> geometric_quotient_partial<Motor> {
        geometric_quotient_partial(this)
    }
}
extension Motor: GeometricQuotient<DualNum> {
    func geometric_quotient(self, other: DualNum) -> Motor {
DualNum geometric_product = DualNum::from_groups(
            /* scalar, e1234 */
            float2(powi(other[scalar], -2)) * other.group0()
        );
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            (float4(geometric_product[scalar]) * self.group0()) + (float4(geometric_product[e1234]) * self.group1()), 
            /* e23, e31, e12, scalar */
            float4(geometric_product[scalar]) * self.group1()
        );
    }
}
extension Motor: GeometricQuotient<Flector> {
    func geometric_quotient(self, other: Flector) -> Flector {
Flector reverse = Flector::from_groups(
            /* e1, e2, e3, e4 */
            other.group0(), 
            /* e423, e431, e412, e321 */
            other.group1() * float4(-1.0)
        );
Scalar other_2 = Scalar::from_groups(
            /* scalar */
            powi(other[e1], 2) + powi(other[e2], 2) + powi(other[e3], 2) + powi(other[e321], 2)
        );
Flector geometric_product = Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(other_2[scalar]) * reverse.group0(), 
            /* e423, e431, e412, e321 */
            float4(other_2[scalar]) * reverse.group1()
        );
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            (geometric_product.group0().xyxx * float4(self.group1().wwy, self[e41])) + (geometric_product.group0().yzzy * float4(self.group1().zxw, self[e42])) + float4(float3(0.0), (geometric_product[e4] * self[scalar]) - (geometric_product[e431] * self[e31]) - (geometric_product[e412] * self[e12]) - (geometric_product[e321] * self[e1234])) + float4((geometric_product.group1().www * self.group1().xyz), geometric_product[e3] * self[e43]) - (self.group1().yzxx * float4(geometric_product.group0().zxy, geometric_product[e423])), 
            /* e423, e431, e412, e321 */
            float4((geometric_product[e4] * self[e23]) + (geometric_product[e423] * self[scalar]) + (geometric_product[e431] * self[e12]) - (geometric_product[e321] * self[e41]), (geometric_product[e4] * self[e31]) + (geometric_product[e431] * self[scalar]) + (geometric_product[e412] * self[e23]) - (geometric_product[e321] * self[e42]), (geometric_product[e4] * self[e12]) + (geometric_product[e423] * self[e31]) + (geometric_product[e412] * self[scalar]) - (geometric_product[e321] * self[e43]), 0.0) + float4((geometric_product.group0().zxy * self.group0().yzx), geometric_product[e321] * self[scalar]) - (geometric_product.group0().xyxx * float4(self.group0().wwy, self[e23])) - (geometric_product.group0().yzzy * float4(self.group0().zxw, self[e31])) - (self.group1().yzxz * float4(geometric_product.group1().zxy, geometric_product[e3]))
        );
    }
}
extension Motor: GeometricQuotient<Horizon> {
    func geometric_quotient(self, other: Horizon) -> Flector {
Horizon geometric_product = Horizon::from_groups(
            /* e321 */
            1.0/other[e321] * -1.0
        );
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(geometric_product[e321]) * float4(self.group1().xyz, self[e1234]) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e423, e431, e412, e321 */
            float4(geometric_product[e321]) * float4(self.group0().xyz, self[scalar]) * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
extension Motor: GeometricQuotient<Line> {
    func geometric_quotient(self, other: Line) -> Motor {
Line reverse = Line::from_groups(
            /* e41, e42, e43 */
            other.group0() * float3(-1.0), 
            /* e23, e31, e12 */
            other.group1() * float3(-1.0)
        );
Scalar other_2 = Scalar::from_groups(
            /* scalar */
            powi(other[e23], 2) + powi(other[e31], 2) + powi(other[e12], 2)
        );
Line geometric_product = Line::from_groups(
            /* e41, e42, e43 */
            float3(other_2[scalar]) * reverse.group0(), 
            /* e23, e31, e12 */
            float3(other_2[scalar]) * reverse.group1()
        );
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4((geometric_product[e41] * self[scalar]) + (geometric_product[e42] * self[e12]) + (geometric_product[e23] * self[e1234]) + (geometric_product[e31] * self[e43]), (geometric_product[e42] * self[scalar]) + (geometric_product[e43] * self[e23]) + (geometric_product[e31] * self[e1234]) + (geometric_product[e12] * self[e41]), (geometric_product[e41] * self[e31]) + (geometric_product[e43] * self[scalar]) + (geometric_product[e23] * self[e42]) + (geometric_product[e12] * self[e1234]), -(geometric_product[e43] * self[e12]) - (geometric_product[e23] * self[e41]) - (geometric_product[e31] * self[e42]) - (geometric_product[e12] * self[e43])) - (self.group1().yzxx * float4(geometric_product.group0().zxy, geometric_product[e41])) - float4((geometric_product.group1().zxy * self.group0().yzx), geometric_product[e42] * self[e31]), 
            /* e23, e31, e12, scalar */
            float4((geometric_product[e23] * self[scalar]) + (geometric_product[e31] * self[e12]), (geometric_product[e31] * self[scalar]) + (geometric_product[e12] * self[e23]), (geometric_product[e23] * self[e31]) + (geometric_product[e12] * self[scalar]), -(geometric_product[e31] * self[e31]) - (geometric_product[e12] * self[e12])) - (self.group1().yzxx * float4(geometric_product.group1().zxy, geometric_product[e23]))
        );
    }
}
extension Motor: GeometricQuotient<Motor> {
    func geometric_quotient(self, other: Motor) -> Motor {
Motor reverse = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            other.group0() * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            other.group1() * float4(-1.0, -1.0, -1.0, 1.0)
        );
Scalar other_2 = Scalar::from_groups(
            /* scalar */
            powi(other[e23], 2) + powi(other[e31], 2) + powi(other[e12], 2) + powi(other[scalar], 2)
        );
Motor geometric_product = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(other_2[scalar]) * reverse.group0(), 
            /* e23, e31, e12, scalar */
            float4(other_2[scalar]) * reverse.group1()
        );
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4((geometric_product[e1234] * self[e23]) + (geometric_product[e23] * self[e1234]) + (geometric_product[e31] * self[e43]) + (geometric_product[scalar] * self[e41]), (geometric_product[e1234] * self[e31]) + (geometric_product[e31] * self[e1234]) + (geometric_product[e12] * self[e41]) + (geometric_product[scalar] * self[e42]), (geometric_product[e1234] * self[e12]) + (geometric_product[e23] * self[e42]) + (geometric_product[e12] * self[e1234]) + (geometric_product[scalar] * self[e43]), -(geometric_product[e43] * self[e12]) - (geometric_product[e23] * self[e41]) - (geometric_product[e31] * self[e42]) - (geometric_product[e12] * self[e43])) + (geometric_product.group0().xyxw * self.group1().wwyw) + float4((geometric_product.group0().yzz * self.group1().zxw), geometric_product[scalar] * self[e1234]) - (geometric_product.group0().zxyx * self.group1().yzxx) - float4((geometric_product.group1().zxy * self.group0().yzx), geometric_product[e42] * self[e31]), 
            /* e23, e31, e12, scalar */
            float4((geometric_product[e31] * self[e12]) + (geometric_product[scalar] * self[e23]), (geometric_product[e12] * self[e23]) + (geometric_product[scalar] * self[e31]), (geometric_product[e12] * self[scalar]) + (geometric_product[scalar] * self[e12]), -(geometric_product[e31] * self[e31]) - (geometric_product[e12] * self[e12])) + (geometric_product.group1().xyxw * self.group1().wwyw) - (geometric_product.group1().zxyx * self.group1().yzxx)
        );
    }
}
extension Motor: GeometricQuotient<MultiVector> {
    func geometric_quotient(self, other: MultiVector) -> MultiVector {
MultiVector reverse = MultiVector::from_groups(
            /* scalar, e1234 */
            other.group0(), 
            /* e1, e2, e3, e4 */
            other.group1(), 
            /* e41, e42, e43 */
            other.group2() * float3(-1.0), 
            /* e23, e31, e12 */
            other.group3() * float3(-1.0), 
            /* e423, e431, e412, e321 */
            other.group4() * float4(-1.0)
        );
Scalar other_2 = Scalar::from_groups(
            /* scalar */
            powi(other[scalar], 2) + powi(other[e1], 2) + powi(other[e2], 2) + powi(other[e3], 2) + powi(other[e23], 2) + powi(other[e31], 2) + powi(other[e12], 2) + powi(other[e321], 2)
        );
MultiVector geometric_product = MultiVector::from_groups(
            /* scalar, e1234 */
            float2(other_2[scalar]) * reverse.group0(), 
            /* e1, e2, e3, e4 */
            float4(other_2[scalar]) * reverse.group1(), 
            /* e41, e42, e43 */
            float3(other_2[scalar]) * reverse.group2(), 
            /* e23, e31, e12 */
            float3(other_2[scalar]) * reverse.group3(), 
            /* e423, e431, e412, e321 */
            float4(other_2[scalar]) * reverse.group4()
        );
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(0.0, (self[scalar] * geometric_product[e1234]) - (self[e41] * geometric_product[e23]) - (self[e42] * geometric_product[e31]) - (self[e43] * geometric_product[e12])) + (float2(geometric_product[scalar]) * float2(self[scalar], self[e1234])) - (float2(self[e23]) * float2(geometric_product[e23], geometric_product[e41])) - (float2(self[e31]) * float2(geometric_product[e31], geometric_product[e42])) - (float2(self[e12]) * float2(geometric_product[e12], geometric_product[e43])), 
            /* e1, e2, e3, e4 */
            (float4(geometric_product[e2], geometric_product[e321], geometric_product[e321], geometric_product[e2]) * float4(self.group1().zyz, self[e42])) + (float4(geometric_product[e321], geometric_product[e3], geometric_product[e1], geometric_product[e1]) * float4(self.group1().xxy, self[e41])) + (geometric_product.group1().xyzz * float4(self.group1().www, self[e43])) + float4(float3(0.0), (self[scalar] * geometric_product[e4]) - (self[e23] * geometric_product[e423]) - (self[e31] * geometric_product[e431]) - (self[e12] * geometric_product[e412])) - float4((self.group1().yzx * geometric_product.group1().zxy), self[e1234] * geometric_product[e321]), 
            /* e41, e42, e43 */
            (float3(geometric_product[scalar]) * self.group0().xyz) + (float3(geometric_product[e1234]) * self.group1().xyz) + (geometric_product.group2().xyx * self.group1().wwy) + (geometric_product.group2().yzz * self.group1().zxw) + (geometric_product.group3().xyx * self.group0().wwy) + (geometric_product.group3().yzz * self.group0().zxw) - (geometric_product.group2().zxy * self.group1().yzx) - (geometric_product.group3().zxy * self.group0().yzx), 
            /* e23, e31, e12 */
            (float3(geometric_product[scalar]) * self.group1().xyz) + (geometric_product.group3().xyx * self.group1().wwy) + (geometric_product.group3().yzz * self.group1().zxw) - (geometric_product.group3().zxy * self.group1().yzx), 
            /* e423, e431, e412, e321 */
            float4((self[e23] * geometric_product[e4]) + (self[e12] * geometric_product[e431]) + (self[scalar] * geometric_product[e423]) - (self[e31] * geometric_product[e412]), (self[e23] * geometric_product[e412]) + (self[e31] * geometric_product[e4]) + (self[scalar] * geometric_product[e431]) - (self[e12] * geometric_product[e423]), (self[e31] * geometric_product[e423]) + (self[e12] * geometric_product[e4]) + (self[scalar] * geometric_product[e412]) - (self[e23] * geometric_product[e431]), 0.0) + float4((self.group0().yzx * geometric_product.group1().zxy), self[scalar] * geometric_product[e321]) - (float4(geometric_product[e2], geometric_product[e321], geometric_product[e321], geometric_product[e2]) * float4(self.group0().zyz, self[e31])) - (float4(geometric_product[e321], geometric_product[e3], geometric_product[e1], geometric_product[e1]) * float4(self.group0().xxy, self[e23])) - (geometric_product.group1().xyzz * float4(self.group0().www, self[e12]))
        );
    }
}
extension Motor: GeometricQuotient<Plane> {
    func geometric_quotient(self, other: Plane) -> Flector {
Plane geometric_product = Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4(powi(other[e321], -2)) * float4(other[e423] * -1.0, other[e431] * -1.0, other[e412] * -1.0, other[e321] * -1.0)
        );
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(geometric_product[e321], geometric_product[e321], geometric_product[e321], 1.0) * float4(self.group1().xyz, -(self[e1234] * geometric_product[e321]) - (self[e23] * geometric_product[e423]) - (self[e31] * geometric_product[e431]) - (self[e12] * geometric_product[e412])), 
            /* e423, e431, e412, e321 */
            float4(((float3(self[scalar]) * geometric_product.group0().xyz) + (self.group1().zxy * geometric_product.group0().yzx) - (float3(geometric_product[e321]) * self.group0().xyz) - (self.group1().yzx * geometric_product.group0().zxy)), self[scalar] * geometric_product[e321])
        );
    }
}
extension Motor: GeometricQuotient<Point> {
    func geometric_quotient(self, other: Point) -> Flector {
Point geometric_product = Point::from_groups(
            /* e1, e2, e3, e4 */
            float4(powi(other[e1], 2) + powi(other[e2], 2) + powi(other[e3], 2)) * other.group0()
        );
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(self[e31] * geometric_product[e3] * -1.0, self[e12] * geometric_product[e1] * -1.0, self[e23] * geometric_product[e2] * -1.0, (self[e43] * geometric_product[e3]) + (self[scalar] * geometric_product[e4])) + (geometric_product.group0().xyzy * float4(self.group1().www, self[e42])) + (geometric_product.group0().yzxx * float4(self.group1().zxy, self[e41])), 
            /* e423, e431, e412, e321 */
            float4((self[e42] * geometric_product[e3]) + (self[e23] * geometric_product[e4]), (self[e43] * geometric_product[e1]) + (self[e31] * geometric_product[e4]), (self[e41] * geometric_product[e2]) + (self[e12] * geometric_product[e4]), self[e12] * geometric_product[e3] * -1.0) - (geometric_product.group0().xyzy * float4(self.group0().www, self[e31])) - (geometric_product.group0().yzxx * float4(self.group0().zxy, self[e23]))
        );
    }
}
extension Motor: GeometricQuotient<Scalar> {
    func geometric_quotient(self, other: Scalar) -> Motor {
Scalar geometric_product = Scalar::from_groups(
            /* scalar */
            1.0/other[scalar]
        );
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(geometric_product[scalar]) * self.group0(), 
            /* e23, e31, e12, scalar */
            float4(geometric_product[scalar]) * self.group1()
        );
    }
}
extension MultiVector {
    func div(_rhs: geometric_quotient) -> geometric_quotient_partial<MultiVector> {
        geometric_quotient_partial(this)
    }
}
extension MultiVector: GeometricQuotient<DualNum> {
    func geometric_quotient(self, other: DualNum) -> MultiVector {
DualNum geometric_product = DualNum::from_groups(
            /* scalar, e1234 */
            float2(powi(other[scalar], -2)) * other.group0()
        );
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(geometric_product[scalar] * self[scalar], (geometric_product[scalar] * self[e1234]) + (geometric_product[e1234] * self[scalar])), 
            /* e1, e2, e3, e4 */
            float4(self[e1], self[e2], self[e3], 1.0) * float4(geometric_product.group0().xx, geometric_product[scalar], (geometric_product[scalar] * self[e4]) + (geometric_product[e1234] * self[e321])), 
            /* e41, e42, e43 */
            (float3(geometric_product[scalar]) * self.group2()) + (float3(geometric_product[e1234]) * self.group3()), 
            /* e23, e31, e12 */
            float3(geometric_product[scalar]) * self.group3(), 
            /* e423, e431, e412, e321 */
            float4(((float3(geometric_product[scalar]) * self.group4().xyz) + (float3(geometric_product[e1234]) * self.group1().xyz)), geometric_product[scalar] * self[e321])
        );
    }
}
extension MultiVector: GeometricQuotient<Flector> {
    func geometric_quotient(self, other: Flector) -> MultiVector {
Flector reverse = Flector::from_groups(
            /* e1, e2, e3, e4 */
            other.group0(), 
            /* e423, e431, e412, e321 */
            other.group1() * float4(-1.0)
        );
Scalar other_2 = Scalar::from_groups(
            /* scalar */
            powi(other[e1], 2) + powi(other[e2], 2) + powi(other[e3], 2) + powi(other[e321], 2)
        );
Flector geometric_product = Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(other_2[scalar]) * reverse.group0(), 
            /* e423, e431, e412, e321 */
            float4(other_2[scalar]) * reverse.group1()
        );
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(0.0, (geometric_product[e321] * self[e4]) - (geometric_product[e2] * self[e431]) - (geometric_product[e3] * self[e412]) - (geometric_product[e4] * self[e321])) + (float2(self[e1]) * float2(geometric_product[e1], geometric_product[e423])) + (float2(self[e2]) * float2(geometric_product[e2], geometric_product[e431])) + (float2(self[e3]) * float2(geometric_product[e3], geometric_product[e412])) - (float2(geometric_product[e321], geometric_product[e1]) * self.group4().wx), 
            /* e1, e2, e3, e4 */
            (float4(self[scalar]) * geometric_product.group0()) + (float4(geometric_product[e2], geometric_product[e321], geometric_product[e321], geometric_product[e2]) * float4(self.group3().zyz, self[e42])) + (float4(geometric_product[e321], geometric_product[e3], geometric_product[e1], geometric_product[e1]) * float4(self.group3().xxy, self[e41])) + float4(float3(0.0), (geometric_product[e3] * self[e43]) - (geometric_product[e423] * self[e23]) - (geometric_product[e431] * self[e31]) - (geometric_product[e412] * self[e12])) - float4((self.group3().yzx * geometric_product.group0().zxy), geometric_product[e321] * self[e1234]), 
            /* e41, e42, e43 */
            (float3(geometric_product[e321]) * self.group4().xyz) + (float3(self[e4], self[e4], self[e431]) * geometric_product.group0().xyx) + (float3(self[e412], self[e423], self[e4]) * geometric_product.group0().yzz) + (geometric_product.group1().zxy * self.group1().yzx) - (float3(geometric_product[e4]) * self.group1().xyz) - (float3(self[e3], self[e1], self[e321]) * geometric_product.group1().yzz) - (float3(self[e321], self[e321], self[e2]) * geometric_product.group1().xyx) - (geometric_product.group0().zxy * self.group4().yzx), 
            /* e23, e31, e12 */
            (geometric_product.group0().zxy * self.group1().yzx) - (float3(geometric_product[e321]) * self.group1().xyz) - (float3(self[e3], self[e1], self[e321]) * geometric_product.group0().yzz) - (float3(self[e321], self[e321], self[e2]) * geometric_product.group0().xyx), 
            /* e423, e431, e412, e321 */
            float4((geometric_product[e3] * self[e42]) + (geometric_product[e4] * self[e23]) + (geometric_product[e431] * self[e12]) - (geometric_product[e412] * self[e31]), (geometric_product[e1] * self[e43]) + (geometric_product[e4] * self[e31]) + (geometric_product[e412] * self[e23]) - (geometric_product[e423] * self[e12]), (geometric_product[e2] * self[e41]) + (geometric_product[e4] * self[e12]) + (geometric_product[e423] * self[e31]) - (geometric_product[e431] * self[e23]), 0.0) + (float4(self[scalar]) * geometric_product.group1()) - (float4(geometric_product[e2], geometric_product[e321], geometric_product[e321], geometric_product[e3]) * float4(self.group2().zyz, self[e12])) - (float4(geometric_product[e321], geometric_product[e3], geometric_product[e1], geometric_product[e2]) * float4(self.group2().xxy, self[e31])) - (geometric_product.group0().xyzx * float4(self.group0().yy, self[e1234], self[e23]))
        );
    }
}
extension MultiVector: GeometricQuotient<Horizon> {
    func geometric_quotient(self, other: Horizon) -> MultiVector {
Horizon geometric_product = Horizon::from_groups(
            /* e321 */
            1.0/other[e321] * -1.0
        );
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(geometric_product[e321]) * float2(self[e321], self[e4]) * float2(-1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(geometric_product[e321]) * float4(self.group3(), self[e1234]) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e41, e42, e43 */
            float3(geometric_product[e321]) * self.group4().xyz, 
            /* e23, e31, e12 */
            float3(geometric_product[e321]) * self.group1().xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            float4(geometric_product[e321]) * float4(self.group2(), self[scalar]) * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
extension MultiVector: GeometricQuotient<Line> {
    func geometric_quotient(self, other: Line) -> MultiVector {
Line reverse = Line::from_groups(
            /* e41, e42, e43 */
            other.group0() * float3(-1.0), 
            /* e23, e31, e12 */
            other.group1() * float3(-1.0)
        );
Scalar other_2 = Scalar::from_groups(
            /* scalar */
            powi(other[e23], 2) + powi(other[e31], 2) + powi(other[e12], 2)
        );
Line geometric_product = Line::from_groups(
            /* e41, e42, e43 */
            float3(other_2[scalar]) * reverse.group0(), 
            /* e23, e31, e12 */
            float3(other_2[scalar]) * reverse.group1()
        );
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(0.0, -(geometric_product[e23] * self[e41]) - (geometric_product[e31] * self[e42]) - (geometric_product[e12] * self[e43])) - (float2(self[e23]) * float2(geometric_product[e23], geometric_product[e41])) - (float2(self[e31]) * float2(geometric_product[e31], geometric_product[e42])) - (float2(self[e12]) * float2(geometric_product[e12], geometric_product[e43])), 
            /* e1, e2, e3, e4 */
            float4((geometric_product[e23] * self[e321]) + (geometric_product[e31] * self[e3]), (geometric_product[e31] * self[e321]) + (geometric_product[e12] * self[e1]), (geometric_product[e23] * self[e2]) + (geometric_product[e12] * self[e321]), -(geometric_product[e42] * self[e2]) - (geometric_product[e43] * self[e3]) - (geometric_product[e23] * self[e423]) - (geometric_product[e31] * self[e431]) - (geometric_product[e12] * self[e412])) - (self.group1().yzxx * float4(geometric_product.group1().zxy, geometric_product[e41])), 
            /* e41, e42, e43 */
            (float3(self[scalar]) * geometric_product.group0()) + (float3(self[e1234]) * geometric_product.group1()) + (geometric_product.group0().yzx * self.group3().zxy) + (geometric_product.group1().yzx * self.group2().zxy) - (geometric_product.group0().zxy * self.group3().yzx) - (geometric_product.group1().zxy * self.group2().yzx), 
            /* e23, e31, e12 */
            (float3(self[scalar]) * geometric_product.group1()) + (geometric_product.group1().yzx * self.group3().zxy) - (geometric_product.group1().zxy * self.group3().yzx), 
            /* e423, e431, e412, e321 */
            float4((geometric_product[e41] * self[e321]) + (geometric_product[e42] * self[e3]) + (geometric_product[e23] * self[e4]) + (geometric_product[e31] * self[e412]), (geometric_product[e42] * self[e321]) + (geometric_product[e43] * self[e1]) + (geometric_product[e31] * self[e4]) + (geometric_product[e12] * self[e423]), (geometric_product[e41] * self[e2]) + (geometric_product[e43] * self[e321]) + (geometric_product[e23] * self[e431]) + (geometric_product[e12] * self[e4]), geometric_product[e12] * self[e3] * -1.0) - (self.group1().yzxx * float4(geometric_product.group0().zxy, geometric_product[e23])) - float4((geometric_product.group1().zxy * self.group4().yzx), geometric_product[e31] * self[e2])
        );
    }
}
extension MultiVector: GeometricQuotient<Motor> {
    func geometric_quotient(self, other: Motor) -> MultiVector {
Motor reverse = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            other.group0() * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            other.group1() * float4(-1.0, -1.0, -1.0, 1.0)
        );
Scalar other_2 = Scalar::from_groups(
            /* scalar */
            powi(other[e23], 2) + powi(other[e31], 2) + powi(other[e12], 2) + powi(other[scalar], 2)
        );
Motor geometric_product = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(other_2[scalar]) * reverse.group0(), 
            /* e23, e31, e12, scalar */
            float4(other_2[scalar]) * reverse.group1()
        );
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(0.0, (geometric_product[scalar] * self[e1234]) - (geometric_product[e41] * self[e23]) - (geometric_product[e42] * self[e31]) - (geometric_product[e43] * self[e12])) + (float2(self[scalar]) * float2(geometric_product[scalar], geometric_product[e1234])) - (float2(geometric_product[e23]) * float2(self[e23], self[e41])) - (float2(geometric_product[e31]) * float2(self[e31], self[e42])) - (float2(geometric_product[e12]) * float2(self[e12], self[e43])), 
            /* e1, e2, e3, e4 */
            float4(geometric_product[scalar] * self[e1], geometric_product[scalar] * self[e2], geometric_product[scalar] * self[e3], -(geometric_product[e42] * self[e2]) - (geometric_product[e43] * self[e3]) - (geometric_product[e23] * self[e423]) - (geometric_product[e31] * self[e431]) - (geometric_product[e12] * self[e412])) + (geometric_product.group1().yzzw * float4(self.group1().zx, self[e321], self[e4])) + (float4(self.group4().ww, self[e2], self[e321]) * float4(geometric_product.group1().xyx, geometric_product[e1234])) - (self.group1().yzxx * float4(geometric_product.group1().zxy, geometric_product[e41])), 
            /* e41, e42, e43 */
            (float3(self[scalar]) * geometric_product.group0().xyz) + (float3(self[e1234]) * geometric_product.group1().xyz) + (self.group2().xxy * geometric_product.group1().wzx) + (self.group2().zyz * geometric_product.group1().yww) + (self.group3().xxy * geometric_product.group0().wzx) + (self.group3().zyz * geometric_product.group0().yww) - (self.group2().yzx * geometric_product.group1().zxy) - (self.group3().yzx * geometric_product.group0().zxy), 
            /* e23, e31, e12 */
            (float3(self[scalar]) * geometric_product.group1().xyz) + (self.group3().xxy * geometric_product.group1().wzx) + (self.group3().zyz * geometric_product.group1().yww) - (self.group3().yzx * geometric_product.group1().zxy), 
            /* e423, e431, e412, e321 */
            float4((geometric_product[e42] * self[e3]) + (geometric_product[e1234] * self[e1]) + (geometric_product[e23] * self[e4]) + (geometric_product[e31] * self[e412]) + (geometric_product[scalar] * self[e423]), (geometric_product[e43] * self[e1]) + (geometric_product[e1234] * self[e2]) + (geometric_product[e31] * self[e4]) + (geometric_product[e12] * self[e423]) + (geometric_product[scalar] * self[e431]), (geometric_product[e43] * self[e321]) + (geometric_product[e1234] * self[e3]) + (geometric_product[e23] * self[e431]) + (geometric_product[e12] * self[e4]) + (geometric_product[scalar] * self[e412]), geometric_product[e12] * self[e3] * -1.0) + (float4(self.group4().ww, self[e2], self[e321]) * float4(geometric_product.group0().xyx, geometric_product[scalar])) - (geometric_product.group1().zxyy * float4(self.group4().yzx, self[e2])) - (self.group1().yzxx * float4(geometric_product.group0().zxy, geometric_product[e23]))
        );
    }
}
extension MultiVector: GeometricQuotient<MultiVector> {
    func geometric_quotient(self, other: MultiVector) -> MultiVector {
MultiVector reverse = MultiVector::from_groups(
            /* scalar, e1234 */
            other.group0(), 
            /* e1, e2, e3, e4 */
            other.group1(), 
            /* e41, e42, e43 */
            other.group2() * float3(-1.0), 
            /* e23, e31, e12 */
            other.group3() * float3(-1.0), 
            /* e423, e431, e412, e321 */
            other.group4() * float4(-1.0)
        );
Scalar other_2 = Scalar::from_groups(
            /* scalar */
            powi(other[scalar], 2) + powi(other[e1], 2) + powi(other[e2], 2) + powi(other[e3], 2) + powi(other[e23], 2) + powi(other[e31], 2) + powi(other[e12], 2) + powi(other[e321], 2)
        );
MultiVector geometric_product = MultiVector::from_groups(
            /* scalar, e1234 */
            float2(other_2[scalar]) * reverse.group0(), 
            /* e1, e2, e3, e4 */
            float4(other_2[scalar]) * reverse.group1(), 
            /* e41, e42, e43 */
            float3(other_2[scalar]) * reverse.group2(), 
            /* e23, e31, e12 */
            float3(other_2[scalar]) * reverse.group3(), 
            /* e423, e431, e412, e321 */
            float4(other_2[scalar]) * reverse.group4()
        );
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(0.0, (geometric_product[e1234] * self[scalar]) + (geometric_product[e321] * self[e4]) - (geometric_product[e2] * self[e431]) - (geometric_product[e3] * self[e412]) - (geometric_product[e4] * self[e321]) - (geometric_product[e23] * self[e41]) - (geometric_product[e31] * self[e42]) - (geometric_product[e12] * self[e43])) + (float2(geometric_product[scalar]) * self.group0()) + (float2(self[e1]) * float2(geometric_product[e1], geometric_product[e423])) + (float2(self[e2]) * float2(geometric_product[e2], geometric_product[e431])) + (float2(self[e3]) * float2(geometric_product[e3], geometric_product[e412])) - (float2(self[e23]) * float2(geometric_product[e23], geometric_product[e41])) - (float2(self[e31]) * float2(geometric_product[e31], geometric_product[e42])) - (float2(self[e12]) * float2(geometric_product[e12], geometric_product[e43])) - (float2(geometric_product[e321], geometric_product[e1]) * self.group4().wx), 
            /* e1, e2, e3, e4 */
            (float4(geometric_product[scalar]) * self.group1()) + (float4(geometric_product[e2], geometric_product[e321], geometric_product[e321], geometric_product[e3]) * float4(self.group3().zyz, self[e43])) + (float4(geometric_product[e321], geometric_product[e3], geometric_product[e1], geometric_product[e2]) * float4(self.group3().xxy, self[e42])) + (float4(self.group0().xx, self[scalar], geometric_product[e1234]) * float4(geometric_product.group1().xyz, self[e321])) + (float4(self.group1().zx, self[e321], geometric_product[e1]) * float4(geometric_product.group3().yzz, self[e41])) + (float4(self.group4().ww, self[e2], geometric_product[e4]) * float4(geometric_product.group3().xyx, self[scalar])) + float4(float3(0.0), -(geometric_product[e42] * self[e2]) - (geometric_product[e43] * self[e3]) - (geometric_product[e23] * self[e423]) - (geometric_product[e31] * self[e431]) - (geometric_product[e12] * self[e412]) - (geometric_product[e423] * self[e23]) - (geometric_product[e431] * self[e31]) - (geometric_product[e412] * self[e12])) - float4((geometric_product.group3().zxy * self.group1().yzx), geometric_product[e321] * self[e1234]) - float4((self.group3().yzx * geometric_product.group1().zxy), geometric_product[e41] * self[e1]), 
            /* e41, e42, e43 */
            (float3(geometric_product[scalar]) * self.group2()) + (float3(geometric_product[e1234]) * self.group3()) + (float3(geometric_product[e321]) * self.group4().xyz) + (float3(self[scalar]) * geometric_product.group2()) + (float3(self[e1234]) * geometric_product.group3()) + (float3(self[e4], self[e4], self[e431]) * geometric_product.group1().xyx) + (float3(self[e412], self[e423], self[e4]) * geometric_product.group1().yzz) + (geometric_product.group2().yzx * self.group3().zxy) + (geometric_product.group3().yzx * self.group2().zxy) + (geometric_product.group4().zxy * self.group1().yzx) - (float3(geometric_product[e4]) * self.group1().xyz) - (float3(self[e3], self[e1], self[e321]) * geometric_product.group4().yzz) - (float3(self[e321], self[e321], self[e2]) * geometric_product.group4().xyx) - (geometric_product.group2().zxy * self.group3().yzx) - (geometric_product.group3().zxy * self.group2().yzx) - (geometric_product.group1().zxy * self.group4().yzx), 
            /* e23, e31, e12 */
            (float3(geometric_product[scalar]) * self.group3()) + (float3(self[scalar]) * geometric_product.group3()) + (geometric_product.group3().yzx * self.group3().zxy) + (geometric_product.group1().zxy * self.group1().yzx) - (float3(geometric_product[e321]) * self.group1().xyz) - (float3(self[e3], self[e1], self[e321]) * geometric_product.group1().yzz) - (float3(self[e321], self[e321], self[e2]) * geometric_product.group1().xyx) - (geometric_product.group3().zxy * self.group3().yzx), 
            /* e423, e431, e412, e321 */
            float4((geometric_product[e3] * self[e42]) + (geometric_product[e4] * self[e23]) + (geometric_product[e41] * self[e321]) + (geometric_product[e42] * self[e3]) + (geometric_product[e23] * self[e4]) + (geometric_product[e31] * self[e412]) + (geometric_product[e423] * self[scalar]) + (geometric_product[e431] * self[e12]), (geometric_product[e1] * self[e43]) + (geometric_product[e4] * self[e31]) + (geometric_product[e42] * self[e321]) + (geometric_product[e43] * self[e1]) + (geometric_product[e31] * self[e4]) + (geometric_product[e12] * self[e423]) + (geometric_product[e431] * self[scalar]) + (geometric_product[e412] * self[e23]), (geometric_product[e2] * self[e41]) + (geometric_product[e4] * self[e12]) + (geometric_product[e41] * self[e2]) + (geometric_product[e43] * self[e321]) + (geometric_product[e23] * self[e431]) + (geometric_product[e12] * self[e4]) + (geometric_product[e423] * self[e31]) + (geometric_product[e412] * self[scalar]), 0.0) + (float4(geometric_product[scalar]) * self.group4()) + (float4(geometric_product.group0().yy, geometric_product[e1234], self[scalar]) * float4(self.group1().xyz, geometric_product[e321])) - (float4(geometric_product[e2], geometric_product[e321], geometric_product[e321], geometric_product[e2]) * float4(self.group2().zyz, self[e31])) - (float4(geometric_product[e321], geometric_product[e3], geometric_product[e1], geometric_product[e1]) * float4(self.group2().xxy, self[e23])) - (self.group1().yzxy * float4(geometric_product.group2().zxy, geometric_product[e31])) - (float4(self.group0().yy, self[e1234], geometric_product[e23]) * float4(geometric_product.group1().xyz, self[e1])) - float4((geometric_product.group3().zxy * self.group4().yzx), geometric_product[e12] * self[e3]) - float4((self.group3().yzx * geometric_product.group4().zxy), geometric_product[e3] * self[e12])
        );
    }
}
extension MultiVector: GeometricQuotient<Plane> {
    func geometric_quotient(self, other: Plane) -> MultiVector {
Plane geometric_product = Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4(powi(other[e321], -2)) * float4(other[e423] * -1.0, other[e431] * -1.0, other[e412] * -1.0, other[e321] * -1.0)
        );
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(self[e321] * geometric_product[e321], (self[e1] * geometric_product[e423]) + (self[e2] * geometric_product[e431]) + (self[e3] * geometric_product[e412]) + (self[e4] * geometric_product[e321])) * float2(-1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(geometric_product[e321], geometric_product[e321], geometric_product[e321], 1.0) * float4(self.group3(), -(self[e1234] * geometric_product[e321]) - (self[e23] * geometric_product[e423]) - (self[e31] * geometric_product[e431]) - (self[e12] * geometric_product[e412])), 
            /* e41, e42, e43 */
            (float3(geometric_product[e321]) * self.group4().xyz) + (self.group1().yzx * geometric_product.group0().zxy) - (float3(self[e321]) * geometric_product.group0().xyz) - (self.group1().zxy * geometric_product.group0().yzx), 
            /* e23, e31, e12 */
            float3(geometric_product[e321]) * self.group1().xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            float4(((float3(self[scalar]) * geometric_product.group0().xyz) + (self.group3().zxy * geometric_product.group0().yzx) - (float3(geometric_product[e321]) * self.group2()) - (self.group3().yzx * geometric_product.group0().zxy)), self[scalar] * geometric_product[e321])
        );
    }
}
extension MultiVector: GeometricQuotient<Point> {
    func geometric_quotient(self, other: Point) -> MultiVector {
Point geometric_product = Point::from_groups(
            /* e1, e2, e3, e4 */
            float4(powi(other[e1], 2) + powi(other[e2], 2) + powi(other[e3], 2)) * other.group0()
        );
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2((self[e1] * geometric_product[e1]) + (self[e2] * geometric_product[e2]) + (self[e3] * geometric_product[e3]), -(self[e423] * geometric_product[e1]) - (self[e431] * geometric_product[e2]) - (self[e412] * geometric_product[e3]) - (self[e321] * geometric_product[e4])), 
            /* e1, e2, e3, e4 */
            float4(self[e31] * geometric_product[e3] * -1.0, self[e12] * geometric_product[e1] * -1.0, self[e23] * geometric_product[e2] * -1.0, (self[e42] * geometric_product[e2]) + (self[e43] * geometric_product[e3])) + (float4(self[scalar]) * geometric_product.group0()) + (geometric_product.group0().yzxx * float4(self.group3().zxy, self[e41])), 
            /* e41, e42, e43 */
            (float3(self[e4]) * geometric_product.group0().xyz) + (self.group4().zxy * geometric_product.group0().yzx) - (float3(geometric_product[e4]) * self.group1().xyz) - (self.group4().yzx * geometric_product.group0().zxy), 
            /* e23, e31, e12 */
            (self.group1().yzx * geometric_product.group0().zxy) - (float3(self[e321]) * geometric_product.group0().xyz) - (self.group1().zxy * geometric_product.group0().yzx), 
            /* e423, e431, e412, e321 */
            float4((self[e42] * geometric_product[e3]) + (self[e23] * geometric_product[e4]), (self[e43] * geometric_product[e1]) + (self[e31] * geometric_product[e4]), (self[e41] * geometric_product[e2]) + (self[e12] * geometric_product[e4]), self[e12] * geometric_product[e3] * -1.0) - (geometric_product.group0().xyzx * float4(self.group0().yy, self[e1234], self[e23])) - (geometric_product.group0().yzxy * float4(self.group2().zxy, self[e31]))
        );
    }
}
extension MultiVector: GeometricQuotient<Scalar> {
    func geometric_quotient(self, other: Scalar) -> MultiVector {
Scalar geometric_product = Scalar::from_groups(
            /* scalar */
            1.0/other[scalar]
        );
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(geometric_product[scalar]) * self.group0(), 
            /* e1, e2, e3, e4 */
            float4(geometric_product[scalar]) * self.group1(), 
            /* e41, e42, e43 */
            float3(geometric_product[scalar]) * self.group2(), 
            /* e23, e31, e12 */
            float3(geometric_product[scalar]) * self.group3(), 
            /* e423, e431, e412, e321 */
            float4(geometric_product[scalar]) * self.group4()
        );
    }
}
extension Origin {
    func div(_rhs: geometric_quotient) -> geometric_quotient_partial<Origin> {
        geometric_quotient_partial(this)
    }
}
extension Origin: GeometricQuotient<DualNum> {
    func geometric_quotient(self, other: DualNum) -> Origin {
        return Origin::from_groups(
            /* e4 */
            self[e4] / (other[scalar])
        );
    }
}
extension Origin: GeometricQuotient<Flector> {
    func geometric_quotient(self, other: Flector) -> Motor {
Flector reverse = Flector::from_groups(
            /* e1, e2, e3, e4 */
            other.group0(), 
            /* e423, e431, e412, e321 */
            other.group1() * float4(-1.0)
        );
Scalar other_2 = Scalar::from_groups(
            /* scalar */
            powi(other[e1], 2) + powi(other[e2], 2) + powi(other[e3], 2) + powi(other[e321], 2)
        );
Flector geometric_product = Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(other_2[scalar]) * reverse.group0(), 
            /* e423, e431, e412, e321 */
            float4(other_2[scalar]) * reverse.group1()
        );
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(self[e4]) * float4(geometric_product.group0().xyz, geometric_product[e321]), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
extension Origin: GeometricQuotient<Horizon> {
    func geometric_quotient(self, other: Horizon) -> AntiScalar {
        return AntiScalar::from_groups(
            /* e1234 */
            self[e4] / (other[e321]) * -1.0
        );
    }
}
extension Origin: GeometricQuotient<Line> {
    func geometric_quotient(self, other: Line) -> Plane {
Line reverse = Line::from_groups(
            /* e41, e42, e43 */
            other.group0() * float3(-1.0), 
            /* e23, e31, e12 */
            other.group1() * float3(-1.0)
        );
Scalar other_2 = Scalar::from_groups(
            /* scalar */
            powi(other[e23], 2) + powi(other[e31], 2) + powi(other[e12], 2)
        );
        return Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4(self[e4], self[e4], self[e4], 0.0) * float4(float3(reverse[e23] * other_2[scalar], reverse[e31] * other_2[scalar], reverse[e12] * other_2[scalar]), 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Origin: GeometricQuotient<Motor> {
    func geometric_quotient(self, other: Motor) -> Flector {
Motor reverse = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            other.group0() * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            other.group1() * float4(-1.0, -1.0, -1.0, 1.0)
        );
Scalar other_2 = Scalar::from_groups(
            /* scalar */
            powi(other[e23], 2) + powi(other[e31], 2) + powi(other[e12], 2) + powi(other[scalar], 2)
        );
Motor geometric_product = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(other_2[scalar]) * reverse.group0(), 
            /* e23, e31, e12, scalar */
            float4(other_2[scalar]) * reverse.group1()
        );
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), geometric_product[scalar] * self[e4]), 
            /* e423, e431, e412, e321 */
            float4(self[e4], self[e4], self[e4], 0.0) * float4(geometric_product.group1().xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Origin: GeometricQuotient<MultiVector> {
    func geometric_quotient(self, other: MultiVector) -> MultiVector {
MultiVector reverse = MultiVector::from_groups(
            /* scalar, e1234 */
            other.group0(), 
            /* e1, e2, e3, e4 */
            other.group1(), 
            /* e41, e42, e43 */
            other.group2() * float3(-1.0), 
            /* e23, e31, e12 */
            other.group3() * float3(-1.0), 
            /* e423, e431, e412, e321 */
            other.group4() * float4(-1.0)
        );
Scalar other_2 = Scalar::from_groups(
            /* scalar */
            powi(other[scalar], 2) + powi(other[e1], 2) + powi(other[e2], 2) + powi(other[e3], 2) + powi(other[e23], 2) + powi(other[e31], 2) + powi(other[e12], 2) + powi(other[e321], 2)
        );
MultiVector geometric_product = MultiVector::from_groups(
            /* scalar, e1234 */
            float2(other_2[scalar]) * reverse.group0(), 
            /* e1, e2, e3, e4 */
            float4(other_2[scalar]) * reverse.group1(), 
            /* e41, e42, e43 */
            float3(other_2[scalar]) * reverse.group2(), 
            /* e23, e31, e12 */
            float3(other_2[scalar]) * reverse.group3(), 
            /* e423, e431, e412, e321 */
            float4(other_2[scalar]) * reverse.group4()
        );
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(1.0, geometric_product[e321] * self[e4]) * float2(0.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), geometric_product[scalar] * self[e4]), 
            /* e41, e42, e43 */
            float3(self[e4]) * geometric_product.group1().xyz, 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(self[e4], self[e4], self[e4], 0.0) * float4(geometric_product.group3(), 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Origin: GeometricQuotient<Plane> {
    func geometric_quotient(self, other: Plane) -> AntiScalar {
        return AntiScalar::from_groups(
            /* e1234 */
            self[e4] / (other[e321]) * -1.0
        );
    }
}
extension Origin: GeometricQuotient<Point> {
    func geometric_quotient(self, other: Point) -> Line {
Scalar other_2 = Scalar::from_groups(
            /* scalar */
            powi(other[e1], 2) + powi(other[e2], 2) + powi(other[e3], 2)
        );
        return Line::from_groups(
            /* e41, e42, e43 */
            float3(self[e4]) * float4(other[e1] * other_2[scalar], other[e2] * other_2[scalar], other[e3] * other_2[scalar], other[e4] * other_2[scalar]).xyz, 
            /* e23, e31, e12 */
            float3(0.0)
        );
    }
}
extension Origin: GeometricQuotient<Scalar> {
    func geometric_quotient(self, other: Scalar) -> Origin {
        return Origin::from_groups(
            /* e4 */
            self[e4] / (other[scalar])
        );
    }
}
extension Plane {
    func div(_rhs: geometric_quotient) -> geometric_quotient_partial<Plane> {
        geometric_quotient_partial(this)
    }
}
extension Plane: GeometricQuotient<DualNum> {
    func geometric_quotient(self, other: DualNum) -> Flector {
DualNum geometric_product = DualNum::from_groups(
            /* scalar, e1234 */
            float2(powi(other[scalar], -2)) * other.group0()
        );
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), geometric_product[e1234] * self[e321]), 
            /* e423, e431, e412, e321 */
            float4(geometric_product[scalar]) * self.group0()
        );
    }
}
extension Plane: GeometricQuotient<Flector> {
    func geometric_quotient(self, other: Flector) -> Motor {
Flector reverse = Flector::from_groups(
            /* e1, e2, e3, e4 */
            other.group0(), 
            /* e423, e431, e412, e321 */
            other.group1() * float4(-1.0)
        );
Scalar other_2 = Scalar::from_groups(
            /* scalar */
            powi(other[e1], 2) + powi(other[e2], 2) + powi(other[e3], 2) + powi(other[e321], 2)
        );
Flector geometric_product = Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(other_2[scalar]) * reverse.group0(), 
            /* e423, e431, e412, e321 */
            float4(other_2[scalar]) * reverse.group1()
        );
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4((geometric_product[e2] * self[e412]) + (geometric_product[e321] * self[e423]), (geometric_product[e3] * self[e423]) + (geometric_product[e321] * self[e431]), (geometric_product[e1] * self[e431]) + (geometric_product[e321] * self[e412]), -(geometric_product[e3] * self[e412]) - (geometric_product[e4] * self[e321])) - (geometric_product.group0().zxyx * self.group0().yzxx) - (self.group0().wwwy * float4(geometric_product.group1().xyz, geometric_product[e2])), 
            /* e23, e31, e12, scalar */
            float4(self[e321]) * float4(geometric_product.group0().xyz, geometric_product[e321]) * float4(-1.0)
        );
    }
}
extension Plane: GeometricQuotient<Horizon> {
    func geometric_quotient(self, other: Horizon) -> Motor {
Horizon geometric_product = Horizon::from_groups(
            /* e321 */
            1.0/other[e321] * -1.0
        );
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(geometric_product[e321], geometric_product[e321], geometric_product[e321], 0.0) * float4(self.group0().xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e23, e31, e12, scalar */
            float4(float3(0.0), geometric_product[e321] * self[e321] * -1.0)
        );
    }
}
extension Plane: GeometricQuotient<Line> {
    func geometric_quotient(self, other: Line) -> Flector {
Line reverse = Line::from_groups(
            /* e41, e42, e43 */
            other.group0() * float3(-1.0), 
            /* e23, e31, e12 */
            other.group1() * float3(-1.0)
        );
Scalar other_2 = Scalar::from_groups(
            /* scalar */
            powi(other[e23], 2) + powi(other[e31], 2) + powi(other[e12], 2)
        );
Line geometric_product = Line::from_groups(
            /* e41, e42, e43 */
            float3(other_2[scalar]) * reverse.group0(), 
            /* e23, e31, e12 */
            float3(other_2[scalar]) * reverse.group1()
        );
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(self[e321], self[e321], self[e321], 1.0) * float4(geometric_product.group1(), -(geometric_product[e23] * self[e423]) - (geometric_product[e31] * self[e431]) - (geometric_product[e12] * self[e412])), 
            /* e423, e431, e412, e321 */
            float4((float3(self[e321]) * geometric_product.group0()), 0.0) + float4((geometric_product.group1().yzx * self.group0().zxy), 0.0) - float4((geometric_product.group1().zxy * self.group0().yzx), 0.0)
        );
    }
}
extension Plane: GeometricQuotient<Motor> {
    func geometric_quotient(self, other: Motor) -> Flector {
Motor reverse = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            other.group0() * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            other.group1() * float4(-1.0, -1.0, -1.0, 1.0)
        );
Scalar other_2 = Scalar::from_groups(
            /* scalar */
            powi(other[e23], 2) + powi(other[e31], 2) + powi(other[e12], 2) + powi(other[scalar], 2)
        );
Motor geometric_product = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(other_2[scalar]) * reverse.group0(), 
            /* e23, e31, e12, scalar */
            float4(other_2[scalar]) * reverse.group1()
        );
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(self[e321], self[e321], self[e321], 1.0) * float4(geometric_product.group1().xyz, (geometric_product[e1234] * self[e321]) - (geometric_product[e23] * self[e423]) - (geometric_product[e31] * self[e431]) - (geometric_product[e12] * self[e412])), 
            /* e423, e431, e412, e321 */
            float4(((float3(geometric_product[scalar]) * self.group0().xyz) + (float3(self[e321]) * geometric_product.group0().xyz) + (geometric_product.group1().yzx * self.group0().zxy) - (geometric_product.group1().zxy * self.group0().yzx)), geometric_product[scalar] * self[e321])
        );
    }
}
extension Plane: GeometricQuotient<MultiVector> {
    func geometric_quotient(self, other: MultiVector) -> MultiVector {
MultiVector reverse = MultiVector::from_groups(
            /* scalar, e1234 */
            other.group0(), 
            /* e1, e2, e3, e4 */
            other.group1(), 
            /* e41, e42, e43 */
            other.group2() * float3(-1.0), 
            /* e23, e31, e12 */
            other.group3() * float3(-1.0), 
            /* e423, e431, e412, e321 */
            other.group4() * float4(-1.0)
        );
Scalar other_2 = Scalar::from_groups(
            /* scalar */
            powi(other[scalar], 2) + powi(other[e1], 2) + powi(other[e2], 2) + powi(other[e3], 2) + powi(other[e23], 2) + powi(other[e31], 2) + powi(other[e12], 2) + powi(other[e321], 2)
        );
MultiVector geometric_product = MultiVector::from_groups(
            /* scalar, e1234 */
            float2(other_2[scalar]) * reverse.group0(), 
            /* e1, e2, e3, e4 */
            float4(other_2[scalar]) * reverse.group1(), 
            /* e41, e42, e43 */
            float3(other_2[scalar]) * reverse.group2(), 
            /* e23, e31, e12 */
            float3(other_2[scalar]) * reverse.group3(), 
            /* e423, e431, e412, e321 */
            float4(other_2[scalar]) * reverse.group4()
        );
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(geometric_product[e321] * self[e321], -(geometric_product[e1] * self[e423]) - (geometric_product[e2] * self[e431]) - (geometric_product[e3] * self[e412]) - (geometric_product[e4] * self[e321])) * float2(-1.0, 1.0), 
            /* e1, e2, e3, e4 */
            float4(self[e321], self[e321], self[e321], 1.0) * float4(geometric_product.group3(), (geometric_product[e1234] * self[e321]) - (geometric_product[e23] * self[e423]) - (geometric_product[e31] * self[e431]) - (geometric_product[e12] * self[e412])), 
            /* e41, e42, e43 */
            (float3(geometric_product[e321]) * self.group0().xyz) + (geometric_product.group1().yzx * self.group0().zxy) - (float3(self[e321]) * geometric_product.group4().xyz) - (geometric_product.group1().zxy * self.group0().yzx), 
            /* e23, e31, e12 */
            float3(self[e321]) * geometric_product.group1().xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            float4(((float3(geometric_product[scalar]) * self.group0().xyz) + (float3(self[e321]) * geometric_product.group2()) + (geometric_product.group3().yzx * self.group0().zxy) - (geometric_product.group3().zxy * self.group0().yzx)), geometric_product[scalar] * self[e321])
        );
    }
}
extension Plane: GeometricQuotient<Plane> {
    func geometric_quotient(self, other: Plane) -> Motor {
Plane geometric_product = Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4(powi(other[e321], -2)) * float4(other[e423] * -1.0, other[e431] * -1.0, other[e412] * -1.0, other[e321] * -1.0)
        );
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(((float3(geometric_product[e321]) * self.group0().xyz) - (float3(self[e321]) * geometric_product.group0().xyz)), 0.0), 
            /* e23, e31, e12, scalar */
            float4(float3(0.0), geometric_product[e321] * self[e321] * -1.0)
        );
    }
}
extension Plane: GeometricQuotient<Point> {
    func geometric_quotient(self, other: Point) -> Motor {
Point geometric_product = Point::from_groups(
            /* e1, e2, e3, e4 */
            float4(powi(other[e1], 2) + powi(other[e2], 2) + powi(other[e3], 2)) * other.group0()
        );
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(self[e412] * geometric_product[e2], self[e423] * geometric_product[e3], self[e431] * geometric_product[e1], -(self[e431] * geometric_product[e2]) - (self[e412] * geometric_product[e3]) - (self[e321] * geometric_product[e4])) - (self.group0().yzxx * geometric_product.group0().zxyx), 
            /* e23, e31, e12, scalar */
            float4(float3(1.0), 0.0) * float4(self.group0().www, 0.0) * float4(geometric_product.group0().xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0)
        );
    }
}
extension Plane: GeometricQuotient<Scalar> {
    func geometric_quotient(self, other: Scalar) -> Plane {
        return Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4(1.0/other[scalar]) * self.group0()
        );
    }
}
extension Point {
    func div(_rhs: geometric_quotient) -> geometric_quotient_partial<Point> {
        geometric_quotient_partial(this)
    }
}
extension Point: GeometricQuotient<DualNum> {
    func geometric_quotient(self, other: DualNum) -> Flector {
DualNum geometric_product = DualNum::from_groups(
            /* scalar, e1234 */
            float2(powi(other[scalar], -2)) * other.group0()
        );
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(geometric_product[scalar]) * self.group0(), 
            /* e423, e431, e412, e321 */
            float4(geometric_product.group0().yy, geometric_product[e1234], 0.0) * float4(float3(1.0), 0.0) * float4(self.group0().xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Point: GeometricQuotient<Flector> {
    func geometric_quotient(self, other: Flector) -> Motor {
Flector reverse = Flector::from_groups(
            /* e1, e2, e3, e4 */
            other.group0(), 
            /* e423, e431, e412, e321 */
            other.group1() * float4(-1.0)
        );
Scalar other_2 = Scalar::from_groups(
            /* scalar */
            powi(other[e1], 2) + powi(other[e2], 2) + powi(other[e3], 2) + powi(other[e321], 2)
        );
Flector geometric_product = Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(other_2[scalar]) * reverse.group0(), 
            /* e423, e431, e412, e321 */
            float4(other_2[scalar]) * reverse.group1()
        );
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(-(geometric_product[e4] * self[e1]) - (geometric_product[e431] * self[e3]), -(geometric_product[e4] * self[e2]) - (geometric_product[e412] * self[e1]), -(geometric_product[e4] * self[e3]) - (geometric_product[e423] * self[e2]), (geometric_product[e412] * self[e3]) + (geometric_product[e321] * self[e4])) + (geometric_product.group1().zxyy * self.group0().yzxy) + (self.group0().wwwx * float4(geometric_product.group0().xyz, geometric_product[e423])), 
            /* e23, e31, e12, scalar */
            float4(-(geometric_product[e2] * self[e3]) - (geometric_product[e321] * self[e1]), -(geometric_product[e3] * self[e1]) - (geometric_product[e321] * self[e2]), -(geometric_product[e1] * self[e2]) - (geometric_product[e321] * self[e3]), (geometric_product[e2] * self[e2]) + (geometric_product[e3] * self[e3])) + (geometric_product.group0().zxyx * self.group0().yzxx)
        );
    }
}
extension Point: GeometricQuotient<Horizon> {
    func geometric_quotient(self, other: Horizon) -> Motor {
Horizon geometric_product = Horizon::from_groups(
            /* e321 */
            1.0/other[e321] * -1.0
        );
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(float3(0.0), geometric_product[e321] * self[e4]), 
            /* e23, e31, e12, scalar */
            float4(geometric_product[e321], geometric_product[e321], geometric_product[e321], 0.0) * float4(self.group0().xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0)
        );
    }
}
extension Point: GeometricQuotient<Line> {
    func geometric_quotient(self, other: Line) -> Flector {
Line reverse = Line::from_groups(
            /* e41, e42, e43 */
            other.group0() * float3(-1.0), 
            /* e23, e31, e12 */
            other.group1() * float3(-1.0)
        );
Scalar other_2 = Scalar::from_groups(
            /* scalar */
            powi(other[e23], 2) + powi(other[e31], 2) + powi(other[e12], 2)
        );
Line geometric_product = Line::from_groups(
            /* e41, e42, e43 */
            float3(other_2[scalar]) * reverse.group0(), 
            /* e23, e31, e12 */
            float3(other_2[scalar]) * reverse.group1()
        );
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(geometric_product[e31] * self[e3], geometric_product[e12] * self[e1], geometric_product[e23] * self[e2], -(geometric_product[e42] * self[e2]) - (geometric_product[e43] * self[e3])) - (self.group0().yzxx * float4(geometric_product.group1().zxy, geometric_product[e41])), 
            /* e423, e431, e412, e321 */
            float4((geometric_product[e42] * self[e3]) + (geometric_product[e23] * self[e4]), (geometric_product[e43] * self[e1]) + (geometric_product[e31] * self[e4]), (geometric_product[e41] * self[e2]) + (geometric_product[e12] * self[e4]), -(geometric_product[e31] * self[e2]) - (geometric_product[e12] * self[e3])) - (self.group0().yzxx * float4(geometric_product.group0().zxy, geometric_product[e23]))
        );
    }
}
extension Point: GeometricQuotient<Motor> {
    func geometric_quotient(self, other: Motor) -> Flector {
Motor reverse = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            other.group0() * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            other.group1() * float4(-1.0, -1.0, -1.0, 1.0)
        );
Scalar other_2 = Scalar::from_groups(
            /* scalar */
            powi(other[e23], 2) + powi(other[e31], 2) + powi(other[e12], 2) + powi(other[scalar], 2)
        );
Motor geometric_product = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(other_2[scalar]) * reverse.group0(), 
            /* e23, e31, e12, scalar */
            float4(other_2[scalar]) * reverse.group1()
        );
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(geometric_product[scalar] * self[e1], geometric_product[scalar] * self[e2], geometric_product[scalar] * self[e3], -(geometric_product[e42] * self[e2]) - (geometric_product[e43] * self[e3])) + (geometric_product.group1().yzxw * self.group0().zxyw) - (self.group0().yzxx * float4(geometric_product.group1().zxy, geometric_product[e41])), 
            /* e423, e431, e412, e321 */
            float4((geometric_product[e42] * self[e3]) + (geometric_product[e1234] * self[e1]) + (geometric_product[e23] * self[e4]), (geometric_product[e43] * self[e1]) + (geometric_product[e1234] * self[e2]) + (geometric_product[e31] * self[e4]), (geometric_product[e41] * self[e2]) + (geometric_product[e1234] * self[e3]) + (geometric_product[e12] * self[e4]), -(geometric_product[e31] * self[e2]) - (geometric_product[e12] * self[e3])) - (self.group0().yzxx * float4(geometric_product.group0().zxy, geometric_product[e23]))
        );
    }
}
extension Point: GeometricQuotient<MultiVector> {
    func geometric_quotient(self, other: MultiVector) -> MultiVector {
MultiVector reverse = MultiVector::from_groups(
            /* scalar, e1234 */
            other.group0(), 
            /* e1, e2, e3, e4 */
            other.group1(), 
            /* e41, e42, e43 */
            other.group2() * float3(-1.0), 
            /* e23, e31, e12 */
            other.group3() * float3(-1.0), 
            /* e423, e431, e412, e321 */
            other.group4() * float4(-1.0)
        );
Scalar other_2 = Scalar::from_groups(
            /* scalar */
            powi(other[scalar], 2) + powi(other[e1], 2) + powi(other[e2], 2) + powi(other[e3], 2) + powi(other[e23], 2) + powi(other[e31], 2) + powi(other[e12], 2) + powi(other[e321], 2)
        );
MultiVector geometric_product = MultiVector::from_groups(
            /* scalar, e1234 */
            float2(other_2[scalar]) * reverse.group0(), 
            /* e1, e2, e3, e4 */
            float4(other_2[scalar]) * reverse.group1(), 
            /* e41, e42, e43 */
            float3(other_2[scalar]) * reverse.group2(), 
            /* e23, e31, e12 */
            float3(other_2[scalar]) * reverse.group3(), 
            /* e423, e431, e412, e321 */
            float4(other_2[scalar]) * reverse.group4()
        );
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(0.0, geometric_product[e321] * self[e4]) + (float2(self[e1]) * float2(geometric_product[e1], geometric_product[e423])) + (float2(self[e2]) * float2(geometric_product[e2], geometric_product[e431])) + (float2(self[e3]) * float2(geometric_product[e3], geometric_product[e412])), 
            /* e1, e2, e3, e4 */
            float4(geometric_product[e31] * self[e3], geometric_product[e12] * self[e1], geometric_product[e23] * self[e2], -(geometric_product[e42] * self[e2]) - (geometric_product[e43] * self[e3])) + (float4(geometric_product[scalar]) * self.group0()) - (self.group0().yzxx * float4(geometric_product.group3().zxy, geometric_product[e41])), 
            /* e41, e42, e43 */
            (float3(self[e4]) * geometric_product.group1().xyz) + (geometric_product.group4().zxy * self.group0().yzx) - (float3(geometric_product[e4]) * self.group0().xyz) - (geometric_product.group4().yzx * self.group0().zxy), 
            /* e23, e31, e12 */
            (geometric_product.group1().zxy * self.group0().yzx) - (float3(geometric_product[e321]) * self.group0().xyz) - (geometric_product.group1().yzx * self.group0().zxy), 
            /* e423, e431, e412, e321 */
            float4((geometric_product[e1234] * self[e1]) + (geometric_product[e42] * self[e3]) + (geometric_product[e23] * self[e4]), (geometric_product[e1234] * self[e2]) + (geometric_product[e43] * self[e1]) + (geometric_product[e31] * self[e4]), (geometric_product[e1234] * self[e3]) + (geometric_product[e41] * self[e2]) + (geometric_product[e12] * self[e4]), -(geometric_product[e31] * self[e2]) - (geometric_product[e12] * self[e3])) - (self.group0().yzxx * float4(geometric_product.group2().zxy, geometric_product[e23]))
        );
    }
}
extension Point: GeometricQuotient<Plane> {
    func geometric_quotient(self, other: Plane) -> Motor {
Plane geometric_product = Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4(powi(other[e321], -2)) * float4(other[e423] * -1.0, other[e431] * -1.0, other[e412] * -1.0, other[e321] * -1.0)
        );
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(geometric_product[e431] * self[e3] * -1.0, geometric_product[e412] * self[e1] * -1.0, geometric_product[e423] * self[e2] * -1.0, (geometric_product[e431] * self[e2]) + (geometric_product[e412] * self[e3]) + (geometric_product[e321] * self[e4])) + (geometric_product.group0().zxyx * self.group0().yzxx), 
            /* e23, e31, e12, scalar */
            float4(float3(1.0), 0.0) * float4(geometric_product.group0().www, 0.0) * float4(self.group0().xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0)
        );
    }
}
extension Point: GeometricQuotient<Point> {
    func geometric_quotient(self, other: Point) -> Motor {
Point geometric_product = Point::from_groups(
            /* e1, e2, e3, e4 */
            float4(powi(other[e1], 2) + powi(other[e2], 2) + powi(other[e3], 2)) * other.group0()
        );
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(((float3(self[e4]) * geometric_product.group0().xyz) - (float3(geometric_product[e4]) * self.group0().xyz)), 0.0), 
            /* e23, e31, e12, scalar */
            float4(geometric_product[e2] * self[e3] * -1.0, geometric_product[e3] * self[e1] * -1.0, geometric_product[e1] * self[e2] * -1.0, (geometric_product[e2] * self[e2]) + (geometric_product[e3] * self[e3])) + (geometric_product.group0().zxyx * self.group0().yzxx)
        );
    }
}
extension Point: GeometricQuotient<Scalar> {
    func geometric_quotient(self, other: Scalar) -> Point {
        return Point::from_groups(
            /* e1, e2, e3, e4 */
            float4(1.0/other[scalar]) * self.group0()
        );
    }
}
extension Scalar {
    func div(_rhs: geometric_quotient) -> geometric_quotient_partial<Scalar> {
        geometric_quotient_partial(this)
    }
}
extension Scalar: GeometricQuotient<DualNum> {
    func geometric_quotient(self, other: DualNum) -> DualNum {
Scalar other_2 = Scalar::from_groups(
            /* scalar */
            powi(other[scalar], -2)
        );
        return DualNum::from_groups(
            /* scalar, e1234 */
            float2(self[scalar]) * float2(other[scalar] * other_2[scalar], other[e1234] * other_2[scalar])
        );
    }
}
extension Scalar: GeometricQuotient<Flector> {
    func geometric_quotient(self, other: Flector) -> Flector {
Flector reverse = Flector::from_groups(
            /* e1, e2, e3, e4 */
            other.group0(), 
            /* e423, e431, e412, e321 */
            other.group1() * float4(-1.0)
        );
Scalar other_2 = Scalar::from_groups(
            /* scalar */
            powi(other[e1], 2) + powi(other[e2], 2) + powi(other[e3], 2) + powi(other[e321], 2)
        );
Flector geometric_product = Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(other_2[scalar]) * reverse.group0(), 
            /* e423, e431, e412, e321 */
            float4(other_2[scalar]) * reverse.group1()
        );
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(self[scalar]) * geometric_product.group0(), 
            /* e423, e431, e412, e321 */
            float4(self[scalar]) * geometric_product.group1()
        );
    }
}
extension Scalar: GeometricQuotient<Horizon> {
    func geometric_quotient(self, other: Horizon) -> Horizon {
        return Horizon::from_groups(
            /* e321 */
            self[scalar] / (other[e321]) * -1.0
        );
    }
}
extension Scalar: GeometricQuotient<Line> {
    func geometric_quotient(self, other: Line) -> Line {
Line reverse = Line::from_groups(
            /* e41, e42, e43 */
            other.group0() * float3(-1.0), 
            /* e23, e31, e12 */
            other.group1() * float3(-1.0)
        );
Scalar other_2 = Scalar::from_groups(
            /* scalar */
            powi(other[e23], 2) + powi(other[e31], 2) + powi(other[e12], 2)
        );
Line geometric_product = Line::from_groups(
            /* e41, e42, e43 */
            float3(other_2[scalar]) * reverse.group0(), 
            /* e23, e31, e12 */
            float3(other_2[scalar]) * reverse.group1()
        );
        return Line::from_groups(
            /* e41, e42, e43 */
            float3(self[scalar]) * geometric_product.group0(), 
            /* e23, e31, e12 */
            float3(self[scalar]) * geometric_product.group1()
        );
    }
}
extension Scalar: GeometricQuotient<Motor> {
    func geometric_quotient(self, other: Motor) -> Motor {
Motor reverse = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            other.group0() * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            other.group1() * float4(-1.0, -1.0, -1.0, 1.0)
        );
Scalar other_2 = Scalar::from_groups(
            /* scalar */
            powi(other[e23], 2) + powi(other[e31], 2) + powi(other[e12], 2) + powi(other[scalar], 2)
        );
Motor geometric_product = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(other_2[scalar]) * reverse.group0(), 
            /* e23, e31, e12, scalar */
            float4(other_2[scalar]) * reverse.group1()
        );
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(self[scalar]) * geometric_product.group0(), 
            /* e23, e31, e12, scalar */
            float4(self[scalar]) * geometric_product.group1()
        );
    }
}
extension Scalar: GeometricQuotient<MultiVector> {
    func geometric_quotient(self, other: MultiVector) -> MultiVector {
MultiVector reverse = MultiVector::from_groups(
            /* scalar, e1234 */
            other.group0(), 
            /* e1, e2, e3, e4 */
            other.group1(), 
            /* e41, e42, e43 */
            other.group2() * float3(-1.0), 
            /* e23, e31, e12 */
            other.group3() * float3(-1.0), 
            /* e423, e431, e412, e321 */
            other.group4() * float4(-1.0)
        );
Scalar other_2 = Scalar::from_groups(
            /* scalar */
            powi(other[scalar], 2) + powi(other[e1], 2) + powi(other[e2], 2) + powi(other[e3], 2) + powi(other[e23], 2) + powi(other[e31], 2) + powi(other[e12], 2) + powi(other[e321], 2)
        );
MultiVector geometric_product = MultiVector::from_groups(
            /* scalar, e1234 */
            float2(other_2[scalar]) * reverse.group0(), 
            /* e1, e2, e3, e4 */
            float4(other_2[scalar]) * reverse.group1(), 
            /* e41, e42, e43 */
            float3(other_2[scalar]) * reverse.group2(), 
            /* e23, e31, e12 */
            float3(other_2[scalar]) * reverse.group3(), 
            /* e423, e431, e412, e321 */
            float4(other_2[scalar]) * reverse.group4()
        );
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(self[scalar]) * geometric_product.group0(), 
            /* e1, e2, e3, e4 */
            float4(self[scalar]) * geometric_product.group1(), 
            /* e41, e42, e43 */
            float3(self[scalar]) * geometric_product.group2(), 
            /* e23, e31, e12 */
            float3(self[scalar]) * geometric_product.group3(), 
            /* e423, e431, e412, e321 */
            float4(self[scalar]) * geometric_product.group4()
        );
    }
}
extension Scalar: GeometricQuotient<Plane> {
    func geometric_quotient(self, other: Plane) -> Plane {
Plane reverse = Plane::from_groups(
            /* e423, e431, e412, e321 */
            other.group0() * float4(-1.0)
        );
Scalar other_2 = Scalar::from_groups(
            /* scalar */
            powi(other[e321], -2)
        );
        return Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4(self[scalar]) * float4(reverse[e423] * other_2[scalar], reverse[e431] * other_2[scalar], reverse[e412] * other_2[scalar], reverse[e321] * other_2[scalar])
        );
    }
}
extension Scalar: GeometricQuotient<Point> {
    func geometric_quotient(self, other: Point) -> Point {
Scalar other_2 = Scalar::from_groups(
            /* scalar */
            powi(other[e1], 2) + powi(other[e2], 2) + powi(other[e3], 2)
        );
        return Point::from_groups(
            /* e1, e2, e3, e4 */
            float4(self[scalar]) * float4(other[e1] * other_2[scalar], other[e2] * other_2[scalar], other[e3] * other_2[scalar], other[e4] * other_2[scalar])
        );
    }
}
extension Scalar: GeometricQuotient<Scalar> {
    func geometric_quotient(self, other: Scalar) -> Scalar {
        return Scalar::from_groups(
            /* scalar */
            self[scalar] / (other[scalar])
        );
    }
}
