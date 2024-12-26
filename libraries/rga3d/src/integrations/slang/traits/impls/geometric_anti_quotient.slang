extension AntiScalar {
    func div(_rhs: geometric_anti_quotient) -> geometric_anti_quotient_partial<AntiScalar> {
        geometric_anti_quotient_partial(this)
    }
}
extension AntiScalar: GeometricAntiQuotient<AntiScalar> {
    func geometric_anti_quotient(self, other: AntiScalar) -> AntiScalar {
        return AntiScalar::from_groups(
            /* e1234 */
            self[e1234] / (other[e1234])
        );
    }
}
extension AntiScalar: GeometricAntiQuotient<DualNum> {
    func geometric_anti_quotient(self, other: DualNum) -> DualNum {
AntiScalar other_2 = AntiScalar::from_groups(
            /* e1234 */
            powi(other[e1234], -2)
        );
        return DualNum::from_groups(
            /* scalar, e1234 */
            float2(self[e1234]) * float2(other_2[e1234] * other[scalar], other_2[e1234] * other[e1234])
        );
    }
}
extension AntiScalar: GeometricAntiQuotient<Flector> {
    func geometric_anti_quotient(self, other: Flector) -> Flector {
Flector anti_reverse = Flector::from_groups(
            /* e1, e2, e3, e4 */
            other.group0() * float4(-1.0), 
            /* e423, e431, e412, e321 */
            other.group1()
        );
AntiScalar other_2 = AntiScalar::from_groups(
            /* e1234 */
            powi(other[e4], 2) + powi(other[e423], 2) + powi(other[e431], 2) + powi(other[e412], 2)
        );
Flector geometric_anti_product = Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(other_2[e1234]) * anti_reverse.group0(), 
            /* e423, e431, e412, e321 */
            float4(other_2[e1234]) * anti_reverse.group1()
        );
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(self[e1234]) * geometric_anti_product.group0(), 
            /* e423, e431, e412, e321 */
            float4(self[e1234]) * geometric_anti_product.group1()
        );
    }
}
extension AntiScalar: GeometricAntiQuotient<Line> {
    func geometric_anti_quotient(self, other: Line) -> Line {
Line anti_reverse = Line::from_groups(
            /* e41, e42, e43 */
            other.group0() * float3(-1.0), 
            /* e23, e31, e12 */
            other.group1() * float3(-1.0)
        );
AntiScalar other_2 = AntiScalar::from_groups(
            /* e1234 */
            powi(other[e41], 2) + powi(other[e42], 2) + powi(other[e43], 2)
        );
Line geometric_anti_product = Line::from_groups(
            /* e41, e42, e43 */
            float3(other_2[e1234]) * anti_reverse.group0(), 
            /* e23, e31, e12 */
            float3(other_2[e1234]) * anti_reverse.group1()
        );
        return Line::from_groups(
            /* e41, e42, e43 */
            float3(self[e1234]) * geometric_anti_product.group0(), 
            /* e23, e31, e12 */
            float3(self[e1234]) * geometric_anti_product.group1()
        );
    }
}
extension AntiScalar: GeometricAntiQuotient<Motor> {
    func geometric_anti_quotient(self, other: Motor) -> Motor {
Motor anti_reverse = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            other.group0() * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            other.group1() * float4(-1.0, -1.0, -1.0, 1.0)
        );
AntiScalar other_2 = AntiScalar::from_groups(
            /* e1234 */
            powi(other[e41], 2) + powi(other[e42], 2) + powi(other[e43], 2) + powi(other[e1234], 2)
        );
Motor geometric_anti_product = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(other_2[e1234]) * anti_reverse.group0(), 
            /* e23, e31, e12, scalar */
            float4(other_2[e1234]) * anti_reverse.group1()
        );
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(self[e1234]) * geometric_anti_product.group0(), 
            /* e23, e31, e12, scalar */
            float4(self[e1234]) * geometric_anti_product.group1()
        );
    }
}
extension AntiScalar: GeometricAntiQuotient<MultiVector> {
    func geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
MultiVector anti_reverse = MultiVector::from_groups(
            /* scalar, e1234 */
            other.group0(), 
            /* e1, e2, e3, e4 */
            other.group1() * float4(-1.0), 
            /* e41, e42, e43 */
            other.group2() * float3(-1.0), 
            /* e23, e31, e12 */
            other.group3() * float3(-1.0), 
            /* e423, e431, e412, e321 */
            other.group4()
        );
AntiScalar other_2 = AntiScalar::from_groups(
            /* e1234 */
            powi(other[e1234], 2) + powi(other[e4], 2) + powi(other[e41], 2) + powi(other[e42], 2) + powi(other[e43], 2) + powi(other[e423], 2) + powi(other[e431], 2) + powi(other[e412], 2)
        );
MultiVector geometric_anti_product = MultiVector::from_groups(
            /* scalar, e1234 */
            float2(other_2[e1234]) * anti_reverse.group0(), 
            /* e1, e2, e3, e4 */
            float4(other_2[e1234]) * anti_reverse.group1(), 
            /* e41, e42, e43 */
            float3(other_2[e1234]) * anti_reverse.group2(), 
            /* e23, e31, e12 */
            float3(other_2[e1234]) * anti_reverse.group3(), 
            /* e423, e431, e412, e321 */
            float4(other_2[e1234]) * anti_reverse.group4()
        );
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(self[e1234]) * geometric_anti_product.group0(), 
            /* e1, e2, e3, e4 */
            float4(self[e1234]) * geometric_anti_product.group1(), 
            /* e41, e42, e43 */
            float3(self[e1234]) * geometric_anti_product.group2(), 
            /* e23, e31, e12 */
            float3(self[e1234]) * geometric_anti_product.group3(), 
            /* e423, e431, e412, e321 */
            float4(self[e1234]) * geometric_anti_product.group4()
        );
    }
}
extension AntiScalar: GeometricAntiQuotient<Origin> {
    func geometric_anti_quotient(self, other: Origin) -> Origin {
        return Origin::from_groups(
            /* e4 */
            self[e1234] / (other[e4]) * -1.0
        );
    }
}
extension AntiScalar: GeometricAntiQuotient<Plane> {
    func geometric_anti_quotient(self, other: Plane) -> Plane {
AntiScalar other_2 = AntiScalar::from_groups(
            /* e1234 */
            powi(other[e423], 2) + powi(other[e431], 2) + powi(other[e412], 2)
        );
        return Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4(self[e1234]) * float4(other_2[e1234] * other[e423], other_2[e1234] * other[e431], other_2[e1234] * other[e412], other_2[e1234] * other[e321])
        );
    }
}
extension AntiScalar: GeometricAntiQuotient<Point> {
    func geometric_anti_quotient(self, other: Point) -> Point {
Point anti_reverse = Point::from_groups(
            /* e1, e2, e3, e4 */
            other.group0() * float4(-1.0)
        );
AntiScalar other_2 = AntiScalar::from_groups(
            /* e1234 */
            powi(other[e4], -2)
        );
        return Point::from_groups(
            /* e1, e2, e3, e4 */
            float4(self[e1234]) * float4(other_2[e1234] * anti_reverse[e1], other_2[e1234] * anti_reverse[e2], other_2[e1234] * anti_reverse[e3], other_2[e1234] * anti_reverse[e4])
        );
    }
}
extension DualNum {
    func div(_rhs: geometric_anti_quotient) -> geometric_anti_quotient_partial<DualNum> {
        geometric_anti_quotient_partial(this)
    }
}
extension DualNum: GeometricAntiQuotient<AntiScalar> {
    func geometric_anti_quotient(self, other: AntiScalar) -> DualNum {
        return DualNum::from_groups(
            /* scalar, e1234 */
            float2(1.0/other[e1234]) * self.group0()
        );
    }
}
extension DualNum: GeometricAntiQuotient<DualNum> {
    func geometric_anti_quotient(self, other: DualNum) -> DualNum {
DualNum geometric_anti_product = DualNum::from_groups(
            /* scalar, e1234 */
            float2(powi(other[e1234], -2)) * other.group0()
        );
        return DualNum::from_groups(
            /* scalar, e1234 */
            float2((geometric_anti_product[scalar] * self[e1234]) + (geometric_anti_product[e1234] * self[scalar]), geometric_anti_product[e1234] * self[e1234])
        );
    }
}
extension DualNum: GeometricAntiQuotient<Flector> {
    func geometric_anti_quotient(self, other: Flector) -> Flector {
Flector anti_reverse = Flector::from_groups(
            /* e1, e2, e3, e4 */
            other.group0() * float4(-1.0), 
            /* e423, e431, e412, e321 */
            other.group1()
        );
AntiScalar other_2 = AntiScalar::from_groups(
            /* e1234 */
            powi(other[e4], 2) + powi(other[e423], 2) + powi(other[e431], 2) + powi(other[e412], 2)
        );
Flector geometric_anti_product = Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(other_2[e1234]) * anti_reverse.group0(), 
            /* e423, e431, e412, e321 */
            float4(other_2[e1234]) * anti_reverse.group1()
        );
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(((float3(self[scalar]) * geometric_anti_product.group1().xyz) + (float3(self[e1234]) * geometric_anti_product.group0().xyz)), self[e1234] * geometric_anti_product[e4]), 
            /* e423, e431, e412, e321 */
            float4(geometric_anti_product[e423], geometric_anti_product[e431], geometric_anti_product[e412], 1.0) * float4(self.group0().yy, self[e1234], (self[scalar] * geometric_anti_product[e4]) + (self[e1234] * geometric_anti_product[e321]))
        );
    }
}
extension DualNum: GeometricAntiQuotient<Line> {
    func geometric_anti_quotient(self, other: Line) -> Line {
Line anti_reverse = Line::from_groups(
            /* e41, e42, e43 */
            other.group0() * float3(-1.0), 
            /* e23, e31, e12 */
            other.group1() * float3(-1.0)
        );
AntiScalar other_2 = AntiScalar::from_groups(
            /* e1234 */
            powi(other[e41], 2) + powi(other[e42], 2) + powi(other[e43], 2)
        );
Line geometric_anti_product = Line::from_groups(
            /* e41, e42, e43 */
            float3(other_2[e1234]) * anti_reverse.group0(), 
            /* e23, e31, e12 */
            float3(other_2[e1234]) * anti_reverse.group1()
        );
        return Line::from_groups(
            /* e41, e42, e43 */
            float3(self[e1234]) * geometric_anti_product.group0(), 
            /* e23, e31, e12 */
            (float3(self[scalar]) * geometric_anti_product.group0()) + (float3(self[e1234]) * geometric_anti_product.group1())
        );
    }
}
extension DualNum: GeometricAntiQuotient<Motor> {
    func geometric_anti_quotient(self, other: Motor) -> Motor {
Motor anti_reverse = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            other.group0() * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            other.group1() * float4(-1.0, -1.0, -1.0, 1.0)
        );
AntiScalar other_2 = AntiScalar::from_groups(
            /* e1234 */
            powi(other[e41], 2) + powi(other[e42], 2) + powi(other[e43], 2) + powi(other[e1234], 2)
        );
Motor geometric_anti_product = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(other_2[e1234]) * anti_reverse.group0(), 
            /* e23, e31, e12, scalar */
            float4(other_2[e1234]) * anti_reverse.group1()
        );
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(self[e1234]) * geometric_anti_product.group0(), 
            /* e23, e31, e12, scalar */
            (float4(self[scalar]) * geometric_anti_product.group0()) + (float4(self[e1234]) * geometric_anti_product.group1())
        );
    }
}
extension DualNum: GeometricAntiQuotient<MultiVector> {
    func geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
MultiVector anti_reverse = MultiVector::from_groups(
            /* scalar, e1234 */
            other.group0(), 
            /* e1, e2, e3, e4 */
            other.group1() * float4(-1.0), 
            /* e41, e42, e43 */
            other.group2() * float3(-1.0), 
            /* e23, e31, e12 */
            other.group3() * float3(-1.0), 
            /* e423, e431, e412, e321 */
            other.group4()
        );
AntiScalar other_2 = AntiScalar::from_groups(
            /* e1234 */
            powi(other[e1234], 2) + powi(other[e4], 2) + powi(other[e41], 2) + powi(other[e42], 2) + powi(other[e43], 2) + powi(other[e423], 2) + powi(other[e431], 2) + powi(other[e412], 2)
        );
MultiVector geometric_anti_product = MultiVector::from_groups(
            /* scalar, e1234 */
            float2(other_2[e1234]) * anti_reverse.group0(), 
            /* e1, e2, e3, e4 */
            float4(other_2[e1234]) * anti_reverse.group1(), 
            /* e41, e42, e43 */
            float3(other_2[e1234]) * anti_reverse.group2(), 
            /* e23, e31, e12 */
            float3(other_2[e1234]) * anti_reverse.group3(), 
            /* e423, e431, e412, e321 */
            float4(other_2[e1234]) * anti_reverse.group4()
        );
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2((self[scalar] * geometric_anti_product[e1234]) + (self[e1234] * geometric_anti_product[scalar]), self[e1234] * geometric_anti_product[e1234]), 
            /* e1, e2, e3, e4 */
            float4(((float3(self[scalar]) * geometric_anti_product.group4().xyz) + (float3(self[e1234]) * geometric_anti_product.group1().xyz)), self[e1234] * geometric_anti_product[e4]), 
            /* e41, e42, e43 */
            float3(self[e1234]) * geometric_anti_product.group2(), 
            /* e23, e31, e12 */
            (float3(self[scalar]) * geometric_anti_product.group2()) + (float3(self[e1234]) * geometric_anti_product.group3()), 
            /* e423, e431, e412, e321 */
            float4(geometric_anti_product[e423], geometric_anti_product[e431], geometric_anti_product[e412], 1.0) * float4(self.group0().yy, self[e1234], (self[scalar] * geometric_anti_product[e4]) + (self[e1234] * geometric_anti_product[e321]))
        );
    }
}
extension DualNum: GeometricAntiQuotient<Origin> {
    func geometric_anti_quotient(self, other: Origin) -> Flector {
Origin geometric_anti_product = Origin::from_groups(
            /* e4 */
            1.0/other[e4] * -1.0
        );
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), self[e1234] * geometric_anti_product[e4]), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), self[scalar] * geometric_anti_product[e4])
        );
    }
}
extension DualNum: GeometricAntiQuotient<Plane> {
    func geometric_anti_quotient(self, other: Plane) -> Flector {
Plane geometric_anti_product = Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4(powi(other[e423], 2) + powi(other[e431], 2) + powi(other[e412], 2)) * other.group0()
        );
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(self.group0().xx, self[scalar], 0.0) * float4(float3(1.0), 0.0) * float4(geometric_anti_product.group0().xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e423, e431, e412, e321 */
            float4(self[e1234]) * geometric_anti_product.group0()
        );
    }
}
extension DualNum: GeometricAntiQuotient<Point> {
    func geometric_anti_quotient(self, other: Point) -> Flector {
Point geometric_anti_product = Point::from_groups(
            /* e1, e2, e3, e4 */
            float4(powi(other[e4], -2)) * float4(other[e1] * -1.0, other[e2] * -1.0, other[e3] * -1.0, other[e4] * -1.0)
        );
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(self[e1234]) * geometric_anti_product.group0(), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), self[scalar] * geometric_anti_product[e4])
        );
    }
}
extension Flector {
    func div(_rhs: geometric_anti_quotient) -> geometric_anti_quotient_partial<Flector> {
        geometric_anti_quotient_partial(this)
    }
}
extension Flector: GeometricAntiQuotient<AntiScalar> {
    func geometric_anti_quotient(self, other: AntiScalar) -> Flector {
AntiScalar geometric_anti_product = AntiScalar::from_groups(
            /* e1234 */
            1.0/other[e1234]
        );
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(geometric_anti_product[e1234]) * self.group0(), 
            /* e423, e431, e412, e321 */
            float4(geometric_anti_product[e1234]) * self.group1()
        );
    }
}
extension Flector: GeometricAntiQuotient<DualNum> {
    func geometric_anti_quotient(self, other: DualNum) -> Flector {
DualNum geometric_anti_product = DualNum::from_groups(
            /* scalar, e1234 */
            float2(powi(other[e1234], -2)) * other.group0()
        );
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(((float3(geometric_anti_product[e1234]) * self.group0().xyz) - (float3(geometric_anti_product[scalar]) * self.group1().xyz)), geometric_anti_product[e1234] * self[e4]), 
            /* e423, e431, e412, e321 */
            float4(self[e423], self[e431], self[e412], 1.0) * float4(geometric_anti_product.group0().yy, geometric_anti_product[e1234], (geometric_anti_product[e1234] * self[e321]) - (geometric_anti_product[scalar] * self[e4]))
        );
    }
}
extension Flector: GeometricAntiQuotient<Flector> {
    func geometric_anti_quotient(self, other: Flector) -> Motor {
Flector anti_reverse = Flector::from_groups(
            /* e1, e2, e3, e4 */
            other.group0() * float4(-1.0), 
            /* e423, e431, e412, e321 */
            other.group1()
        );
AntiScalar other_2 = AntiScalar::from_groups(
            /* e1234 */
            powi(other[e4], 2) + powi(other[e423], 2) + powi(other[e431], 2) + powi(other[e412], 2)
        );
Flector geometric_anti_product = Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(other_2[e1234]) * anti_reverse.group0(), 
            /* e423, e431, e412, e321 */
            float4(other_2[e1234]) * anti_reverse.group1()
        );
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(-(geometric_anti_product[e423] * self[e4]) - (geometric_anti_product[e412] * self[e431]), -(geometric_anti_product[e423] * self[e412]) - (geometric_anti_product[e431] * self[e4]), -(geometric_anti_product[e431] * self[e423]) - (geometric_anti_product[e412] * self[e4]), (geometric_anti_product[e431] * self[e431]) + (geometric_anti_product[e412] * self[e412])) + (geometric_anti_product.group1().yzxx * self.group1().zxyx) - (float4(geometric_anti_product[e4]) * float4(self.group1().xyz, self[e4])), 
            /* e23, e31, e12, scalar */
            (float4(geometric_anti_product[e321]) * float4(self.group1().xyz, self[e4])) + (float4(self[e4], self[e412], self[e423], self[e1]) * float4(geometric_anti_product.group0().xxy, geometric_anti_product[e423])) + (float4(self[e431], self[e4], self[e4], self[e2]) * float4(geometric_anti_product.group0().zyz, geometric_anti_product[e431])) + (geometric_anti_product.group1().yzxz * self.group0().zxyz) - (float4(self[e2], self[e321], self[e321], self[e321]) * float4(geometric_anti_product.group1().zyz, geometric_anti_product[e4])) - (float4(self[e321], self[e3], self[e1], self[e412]) * float4(geometric_anti_product.group1().xxy, geometric_anti_product[e3])) - (geometric_anti_product.group0().yzxx * self.group1().zxyx) - (geometric_anti_product.group0().wwwy * float4(self.group0().xyz, self[e431]))
        );
    }
}
extension Flector: GeometricAntiQuotient<Line> {
    func geometric_anti_quotient(self, other: Line) -> Flector {
Line anti_reverse = Line::from_groups(
            /* e41, e42, e43 */
            other.group0() * float3(-1.0), 
            /* e23, e31, e12 */
            other.group1() * float3(-1.0)
        );
AntiScalar other_2 = AntiScalar::from_groups(
            /* e1234 */
            powi(other[e41], 2) + powi(other[e42], 2) + powi(other[e43], 2)
        );
Line geometric_anti_product = Line::from_groups(
            /* e41, e42, e43 */
            float3(other_2[e1234]) * anti_reverse.group0(), 
            /* e23, e31, e12 */
            float3(other_2[e1234]) * anti_reverse.group1()
        );
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4((self[e2] * geometric_anti_product[e43]) + (self[e412] * geometric_anti_product[e31]) + (self[e321] * geometric_anti_product[e41]), (self[e3] * geometric_anti_product[e41]) + (self[e423] * geometric_anti_product[e12]) + (self[e321] * geometric_anti_product[e42]), (self[e1] * geometric_anti_product[e42]) + (self[e431] * geometric_anti_product[e23]) + (self[e321] * geometric_anti_product[e43]), 0.0) - (float4(self[e4], self[e412], self[e423], self[e431]) * float4(geometric_anti_product.group1().xxy, geometric_anti_product[e42])) - (float4(self[e431], self[e4], self[e4], self[e412]) * float4(geometric_anti_product.group1().zyz, geometric_anti_product[e43])) - float4((geometric_anti_product.group0().yzx * self.group0().zxy), self[e423] * geometric_anti_product[e41]), 
            /* e423, e431, e412, e321 */
            (float4(self[e4], self[e412], self[e423], self[e423]) * float4(geometric_anti_product.group0().xxy, geometric_anti_product[e23])) + (float4(self[e431], self[e4], self[e4], self[e431]) * float4(geometric_anti_product.group0().zyz, geometric_anti_product[e31])) + float4(float3(0.0), (self[e412] * geometric_anti_product[e12]) - (self[e2] * geometric_anti_product[e42]) - (self[e3] * geometric_anti_product[e43])) - float4((geometric_anti_product.group0().yzx * self.group1().zxy), self[e1] * geometric_anti_product[e41])
        );
    }
}
extension Flector: GeometricAntiQuotient<Motor> {
    func geometric_anti_quotient(self, other: Motor) -> Flector {
Motor anti_reverse = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            other.group0() * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            other.group1() * float4(-1.0, -1.0, -1.0, 1.0)
        );
AntiScalar other_2 = AntiScalar::from_groups(
            /* e1234 */
            powi(other[e41], 2) + powi(other[e42], 2) + powi(other[e43], 2) + powi(other[e1234], 2)
        );
Motor geometric_anti_product = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(other_2[e1234]) * anti_reverse.group0(), 
            /* e23, e31, e12, scalar */
            float4(other_2[e1234]) * anti_reverse.group1()
        );
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4((self[e2] * geometric_anti_product[e43]) + (self[e412] * geometric_anti_product[e31]) + (self[e321] * geometric_anti_product[e41]) - (self[e431] * geometric_anti_product[e12]), (self[e3] * geometric_anti_product[e41]) + (self[e423] * geometric_anti_product[e12]) + (self[e321] * geometric_anti_product[e42]) - (self[e412] * geometric_anti_product[e23]), (self[e3] * geometric_anti_product[e1234]) + (self[e431] * geometric_anti_product[e23]) + (self[e321] * geometric_anti_product[e43]) - (self[e412] * geometric_anti_product[scalar]), 0.0) + (self.group0().xyxw * geometric_anti_product.group0().wwyw) - (self.group1().xyxz * float4(geometric_anti_product.group1().wwy, geometric_anti_product[e43])) - (geometric_anti_product.group0().yzxx * float4(self.group0().zxy, self[e423])) - float4((self.group0().www * geometric_anti_product.group1().xyz), self[e431] * geometric_anti_product[e42]), 
            /* e423, e431, e412, e321 */
            (self.group1().xyxy * float4(geometric_anti_product.group0().wwy, geometric_anti_product[e31])) + (self.group1().yzzz * float4(geometric_anti_product.group0().zxw, geometric_anti_product[e12])) + float4(float3(0.0), (self[e321] * geometric_anti_product[e1234]) - (self[e2] * geometric_anti_product[e42]) - (self[e3] * geometric_anti_product[e43]) - (self[e4] * geometric_anti_product[scalar])) + float4((self.group0().www * geometric_anti_product.group0().xyz), self[e423] * geometric_anti_product[e23]) - (geometric_anti_product.group0().yzxx * float4(self.group1().zxy, self[e1]))
        );
    }
}
extension Flector: GeometricAntiQuotient<MultiVector> {
    func geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
MultiVector anti_reverse = MultiVector::from_groups(
            /* scalar, e1234 */
            other.group0(), 
            /* e1, e2, e3, e4 */
            other.group1() * float4(-1.0), 
            /* e41, e42, e43 */
            other.group2() * float3(-1.0), 
            /* e23, e31, e12 */
            other.group3() * float3(-1.0), 
            /* e423, e431, e412, e321 */
            other.group4()
        );
AntiScalar other_2 = AntiScalar::from_groups(
            /* e1234 */
            powi(other[e1234], 2) + powi(other[e4], 2) + powi(other[e41], 2) + powi(other[e42], 2) + powi(other[e43], 2) + powi(other[e423], 2) + powi(other[e431], 2) + powi(other[e412], 2)
        );
MultiVector geometric_anti_product = MultiVector::from_groups(
            /* scalar, e1234 */
            float2(other_2[e1234]) * anti_reverse.group0(), 
            /* e1, e2, e3, e4 */
            float4(other_2[e1234]) * anti_reverse.group1(), 
            /* e41, e42, e43 */
            float3(other_2[e1234]) * anti_reverse.group2(), 
            /* e23, e31, e12 */
            float3(other_2[e1234]) * anti_reverse.group3(), 
            /* e423, e431, e412, e321 */
            float4(other_2[e1234]) * anti_reverse.group4()
        );
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2((self[e4] * geometric_anti_product[e321]) - (self[e431] * geometric_anti_product[e2]) - (self[e412] * geometric_anti_product[e3]) - (self[e321] * geometric_anti_product[e4]), 0.0) + (float2(geometric_anti_product[e423]) * float2(self[e1], self[e423])) + (float2(geometric_anti_product[e431]) * float2(self[e2], self[e431])) + (float2(geometric_anti_product[e412]) * float2(self[e3], self[e412])) - (float2(self[e423], self[e4]) * geometric_anti_product.group1().xw), 
            /* e1, e2, e3, e4 */
            float4((self[e2] * geometric_anti_product[e43]) + (self[e412] * geometric_anti_product[e31]) + (self[e321] * geometric_anti_product[e41]) - (self[e431] * geometric_anti_product[e12]), (self[e3] * geometric_anti_product[e41]) + (self[e423] * geometric_anti_product[e12]) + (self[e321] * geometric_anti_product[e42]) - (self[e4] * geometric_anti_product[e31]), (self[e1] * geometric_anti_product[e42]) + (self[e431] * geometric_anti_product[e23]) + (self[e321] * geometric_anti_product[e43]) - (self[e4] * geometric_anti_product[e12]), 0.0) + (float4(geometric_anti_product[e1234]) * self.group0()) - (float4(self[e4], self[e412], self[e423], self[e412]) * float4(geometric_anti_product.group3().xxy, geometric_anti_product[e43])) - (self.group1().xyzx * float4(geometric_anti_product.group0().xx, geometric_anti_product[scalar], geometric_anti_product[e41])) - float4((geometric_anti_product.group2().yzx * self.group0().zxy), self[e431] * geometric_anti_product[e42]), 
            /* e41, e42, e43 */
            (self.group1().zxy * geometric_anti_product.group4().yzx) - (float3(self[e4]) * geometric_anti_product.group4().xyz) - (float3(geometric_anti_product[e4], geometric_anti_product[e4], geometric_anti_product[e431]) * self.group1().xyx) - (float3(geometric_anti_product[e412], geometric_anti_product[e423], geometric_anti_product[e4]) * self.group1().yzz), 
            /* e23, e31, e12 */
            (float3(self[e4]) * geometric_anti_product.group1().xyz) + (float3(geometric_anti_product[e3], geometric_anti_product[e1], geometric_anti_product[e321]) * self.group1().yzz) + (float3(geometric_anti_product[e321], geometric_anti_product[e321], geometric_anti_product[e2]) * self.group1().xyx) + (self.group0().zxy * geometric_anti_product.group4().yzx) - (float3(self[e321]) * geometric_anti_product.group4().xyz) - (float3(geometric_anti_product[e4], geometric_anti_product[e4], geometric_anti_product[e431]) * self.group0().xyx) - (float3(geometric_anti_product[e412], geometric_anti_product[e423], geometric_anti_product[e4]) * self.group0().yzz) - (self.group1().zxy * geometric_anti_product.group1().yzx), 
            /* e423, e431, e412, e321 */
            (float4(geometric_anti_product[e1234]) * self.group1()) + (float4(self[e4], self[e412], self[e423], self[e423]) * float4(geometric_anti_product.group2().xxy, geometric_anti_product[e23])) + (float4(self[e431], self[e4], self[e4], self[e431]) * float4(geometric_anti_product.group2().zyz, geometric_anti_product[e31])) + float4(float3(0.0), (self[e412] * geometric_anti_product[e12]) - (self[e1] * geometric_anti_product[e41]) - (self[e2] * geometric_anti_product[e42]) - (self[e3] * geometric_anti_product[e43])) - float4((geometric_anti_product.group2().yzx * self.group1().zxy), self[e4] * geometric_anti_product[scalar])
        );
    }
}
extension Flector: GeometricAntiQuotient<Origin> {
    func geometric_anti_quotient(self, other: Origin) -> Motor {
Origin geometric_anti_product = Origin::from_groups(
            /* e4 */
            1.0/other[e4] * -1.0
        );
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(geometric_anti_product[e4]) * float4(self.group1().xyz, self[e4]) * float4(-1.0), 
            /* e23, e31, e12, scalar */
            float4(geometric_anti_product[e4]) * float4(self.group0().xyz, self[e321]) * float4(-1.0)
        );
    }
}
extension Flector: GeometricAntiQuotient<Plane> {
    func geometric_anti_quotient(self, other: Plane) -> Motor {
Plane geometric_anti_product = Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4(powi(other[e423], 2) + powi(other[e431], 2) + powi(other[e412], 2)) * other.group0()
        );
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(-(self[e4] * geometric_anti_product[e423]) - (self[e431] * geometric_anti_product[e412]), -(self[e4] * geometric_anti_product[e431]) - (self[e412] * geometric_anti_product[e423]), -(self[e4] * geometric_anti_product[e412]) - (self[e423] * geometric_anti_product[e431]), (self[e431] * geometric_anti_product[e431]) + (self[e412] * geometric_anti_product[e412])) + (self.group1().zxyx * geometric_anti_product.group0().yzxx), 
            /* e23, e31, e12, scalar */
            float4(-(self[e2] * geometric_anti_product[e412]) - (self[e321] * geometric_anti_product[e423]), -(self[e3] * geometric_anti_product[e423]) - (self[e321] * geometric_anti_product[e431]), -(self[e1] * geometric_anti_product[e431]) - (self[e321] * geometric_anti_product[e412]), (self[e3] * geometric_anti_product[e412]) + (self[e4] * geometric_anti_product[e321])) + (self.group0().zxyx * geometric_anti_product.group0().yzxx) + (geometric_anti_product.group0().wwwy * float4(self.group1().xyz, self[e2]))
        );
    }
}
extension Flector: GeometricAntiQuotient<Point> {
    func geometric_anti_quotient(self, other: Point) -> Motor {
Point geometric_anti_product = Point::from_groups(
            /* e1, e2, e3, e4 */
            float4(powi(other[e4], -2)) * float4(other[e1] * -1.0, other[e2] * -1.0, other[e3] * -1.0, other[e4] * -1.0)
        );
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(geometric_anti_product[e4]) * float4(self.group1().xyz, self[e4]) * float4(-1.0), 
            /* e23, e31, e12, scalar */
            float4((self[e4] * geometric_anti_product[e1]) + (self[e431] * geometric_anti_product[e3]), (self[e4] * geometric_anti_product[e2]) + (self[e412] * geometric_anti_product[e1]), (self[e4] * geometric_anti_product[e3]) + (self[e423] * geometric_anti_product[e2]), -(self[e412] * geometric_anti_product[e3]) - (self[e321] * geometric_anti_product[e4])) - (self.group1().zxyy * geometric_anti_product.group0().yzxy) - (geometric_anti_product.group0().wwwx * float4(self.group0().xyz, self[e423]))
        );
    }
}
extension Horizon {
    func div(_rhs: geometric_anti_quotient) -> geometric_anti_quotient_partial<Horizon> {
        geometric_anti_quotient_partial(this)
    }
}
extension Horizon: GeometricAntiQuotient<AntiScalar> {
    func geometric_anti_quotient(self, other: AntiScalar) -> Horizon {
        return Horizon::from_groups(
            /* e321 */
            self[e321] / (other[e1234])
        );
    }
}
extension Horizon: GeometricAntiQuotient<DualNum> {
    func geometric_anti_quotient(self, other: DualNum) -> Horizon {
        return Horizon::from_groups(
            /* e321 */
            self[e321] / (other[e1234])
        );
    }
}
extension Horizon: GeometricAntiQuotient<Flector> {
    func geometric_anti_quotient(self, other: Flector) -> Motor {
Flector anti_reverse = Flector::from_groups(
            /* e1, e2, e3, e4 */
            other.group0() * float4(-1.0), 
            /* e423, e431, e412, e321 */
            other.group1()
        );
AntiScalar other_2 = AntiScalar::from_groups(
            /* e1234 */
            powi(other[e4], 2) + powi(other[e423], 2) + powi(other[e431], 2) + powi(other[e412], 2)
        );
Flector geometric_anti_product = Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(other_2[e1234]) * anti_reverse.group0(), 
            /* e423, e431, e412, e321 */
            float4(other_2[e1234]) * anti_reverse.group1()
        );
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(0.0), 
            /* e23, e31, e12, scalar */
            float4(self[e321]) * float4(geometric_anti_product.group1().xyz, geometric_anti_product[e4]) * float4(-1.0)
        );
    }
}
extension Horizon: GeometricAntiQuotient<Line> {
    func geometric_anti_quotient(self, other: Line) -> Point {
Line anti_reverse = Line::from_groups(
            /* e41, e42, e43 */
            other.group0() * float3(-1.0), 
            /* e23, e31, e12 */
            other.group1() * float3(-1.0)
        );
AntiScalar other_2 = AntiScalar::from_groups(
            /* e1234 */
            powi(other[e41], 2) + powi(other[e42], 2) + powi(other[e43], 2)
        );
        return Point::from_groups(
            /* e1, e2, e3, e4 */
            float4(self[e321], self[e321], self[e321], 0.0) * float4(float3(other_2[e1234] * anti_reverse[e41], other_2[e1234] * anti_reverse[e42], other_2[e1234] * anti_reverse[e43]), 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Horizon: GeometricAntiQuotient<Motor> {
    func geometric_anti_quotient(self, other: Motor) -> Flector {
Motor anti_reverse = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            other.group0() * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            other.group1() * float4(-1.0, -1.0, -1.0, 1.0)
        );
AntiScalar other_2 = AntiScalar::from_groups(
            /* e1234 */
            powi(other[e41], 2) + powi(other[e42], 2) + powi(other[e43], 2) + powi(other[e1234], 2)
        );
Motor geometric_anti_product = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(other_2[e1234]) * anti_reverse.group0(), 
            /* e23, e31, e12, scalar */
            float4(other_2[e1234]) * anti_reverse.group1()
        );
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(self[e321], self[e321], self[e321], 0.0) * float4(geometric_anti_product.group0().xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), self[e321] * geometric_anti_product[e1234])
        );
    }
}
extension Horizon: GeometricAntiQuotient<MultiVector> {
    func geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
MultiVector anti_reverse = MultiVector::from_groups(
            /* scalar, e1234 */
            other.group0(), 
            /* e1, e2, e3, e4 */
            other.group1() * float4(-1.0), 
            /* e41, e42, e43 */
            other.group2() * float3(-1.0), 
            /* e23, e31, e12 */
            other.group3() * float3(-1.0), 
            /* e423, e431, e412, e321 */
            other.group4()
        );
AntiScalar other_2 = AntiScalar::from_groups(
            /* e1234 */
            powi(other[e1234], 2) + powi(other[e4], 2) + powi(other[e41], 2) + powi(other[e42], 2) + powi(other[e43], 2) + powi(other[e423], 2) + powi(other[e431], 2) + powi(other[e412], 2)
        );
MultiVector geometric_anti_product = MultiVector::from_groups(
            /* scalar, e1234 */
            float2(other_2[e1234]) * anti_reverse.group0(), 
            /* e1, e2, e3, e4 */
            float4(other_2[e1234]) * anti_reverse.group1(), 
            /* e41, e42, e43 */
            float3(other_2[e1234]) * anti_reverse.group2(), 
            /* e23, e31, e12 */
            float3(other_2[e1234]) * anti_reverse.group3(), 
            /* e423, e431, e412, e321 */
            float4(other_2[e1234]) * anti_reverse.group4()
        );
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(self[e321] * geometric_anti_product[e4], 1.0) * float2(-1.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(self[e321], self[e321], self[e321], 0.0) * float4(geometric_anti_product.group2(), 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(self[e321]) * geometric_anti_product.group4().xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), self[e321] * geometric_anti_product[e1234])
        );
    }
}
extension Horizon: GeometricAntiQuotient<Origin> {
    func geometric_anti_quotient(self, other: Origin) -> Scalar {
        return Scalar::from_groups(
            /* scalar */
            self[e321] / (other[e4])
        );
    }
}
extension Horizon: GeometricAntiQuotient<Plane> {
    func geometric_anti_quotient(self, other: Plane) -> Line {
AntiScalar other_2 = AntiScalar::from_groups(
            /* e1234 */
            powi(other[e423], 2) + powi(other[e431], 2) + powi(other[e412], 2)
        );
        return Line::from_groups(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(self[e321]) * float4(other_2[e1234] * other[e423], other_2[e1234] * other[e431], other_2[e1234] * other[e412], other_2[e1234] * other[e321]).xyz * float3(-1.0)
        );
    }
}
extension Horizon: GeometricAntiQuotient<Point> {
    func geometric_anti_quotient(self, other: Point) -> Scalar {
        return Scalar::from_groups(
            /* scalar */
            self[e321] / (other[e4])
        );
    }
}
extension Line {
    func div(_rhs: geometric_anti_quotient) -> geometric_anti_quotient_partial<Line> {
        geometric_anti_quotient_partial(this)
    }
}
extension Line: GeometricAntiQuotient<AntiScalar> {
    func geometric_anti_quotient(self, other: AntiScalar) -> Line {
AntiScalar geometric_anti_product = AntiScalar::from_groups(
            /* e1234 */
            1.0/other[e1234]
        );
        return Line::from_groups(
            /* e41, e42, e43 */
            float3(geometric_anti_product[e1234]) * self.group0(), 
            /* e23, e31, e12 */
            float3(geometric_anti_product[e1234]) * self.group1()
        );
    }
}
extension Line: GeometricAntiQuotient<DualNum> {
    func geometric_anti_quotient(self, other: DualNum) -> Line {
DualNum geometric_anti_product = DualNum::from_groups(
            /* scalar, e1234 */
            float2(powi(other[e1234], -2)) * other.group0()
        );
        return Line::from_groups(
            /* e41, e42, e43 */
            float3(geometric_anti_product[e1234]) * self.group0(), 
            /* e23, e31, e12 */
            (float3(geometric_anti_product[scalar]) * self.group0()) + (float3(geometric_anti_product[e1234]) * self.group1())
        );
    }
}
extension Line: GeometricAntiQuotient<Flector> {
    func geometric_anti_quotient(self, other: Flector) -> Flector {
Flector anti_reverse = Flector::from_groups(
            /* e1, e2, e3, e4 */
            other.group0() * float4(-1.0), 
            /* e423, e431, e412, e321 */
            other.group1()
        );
AntiScalar other_2 = AntiScalar::from_groups(
            /* e1234 */
            powi(other[e4], 2) + powi(other[e423], 2) + powi(other[e431], 2) + powi(other[e412], 2)
        );
Flector geometric_anti_product = Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(other_2[e1234]) * anti_reverse.group0(), 
            /* e423, e431, e412, e321 */
            float4(other_2[e1234]) * anti_reverse.group1()
        );
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4((geometric_anti_product[e3] * self[e42]) + (geometric_anti_product[e4] * self[e23]) + (geometric_anti_product[e412] * self[e31]) + (geometric_anti_product[e321] * self[e41]), (geometric_anti_product[e1] * self[e43]) + (geometric_anti_product[e4] * self[e31]) + (geometric_anti_product[e423] * self[e12]) + (geometric_anti_product[e321] * self[e42]), (geometric_anti_product[e2] * self[e41]) + (geometric_anti_product[e4] * self[e12]) + (geometric_anti_product[e431] * self[e23]) + (geometric_anti_product[e321] * self[e43]), geometric_anti_product[e412] * self[e43] * -1.0) - (geometric_anti_product.group1().yzxy * float4(self.group1().zxy, self[e42])) - float4((self.group0().zxy * geometric_anti_product.group0().yzx), geometric_anti_product[e423] * self[e41]), 
            /* e423, e431, e412, e321 */
            float4((geometric_anti_product[e4] * self[e41]) + (geometric_anti_product[e412] * self[e42]), (geometric_anti_product[e4] * self[e42]) + (geometric_anti_product[e423] * self[e43]), (geometric_anti_product[e4] * self[e43]) + (geometric_anti_product[e431] * self[e41]), -(geometric_anti_product[e2] * self[e42]) - (geometric_anti_product[e3] * self[e43]) - (geometric_anti_product[e423] * self[e23]) - (geometric_anti_product[e431] * self[e31]) - (geometric_anti_product[e412] * self[e12])) - float4((self.group0().zxy * geometric_anti_product.group1().yzx), geometric_anti_product[e1] * self[e41])
        );
    }
}
extension Line: GeometricAntiQuotient<Line> {
    func geometric_anti_quotient(self, other: Line) -> Motor {
Line anti_reverse = Line::from_groups(
            /* e41, e42, e43 */
            other.group0() * float3(-1.0), 
            /* e23, e31, e12 */
            other.group1() * float3(-1.0)
        );
AntiScalar other_2 = AntiScalar::from_groups(
            /* e1234 */
            powi(other[e41], 2) + powi(other[e42], 2) + powi(other[e43], 2)
        );
Line geometric_anti_product = Line::from_groups(
            /* e41, e42, e43 */
            float3(other_2[e1234]) * anti_reverse.group0(), 
            /* e23, e31, e12 */
            float3(other_2[e1234]) * anti_reverse.group1()
        );
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(geometric_anti_product[e43] * self[e42], geometric_anti_product[e41] * self[e43], geometric_anti_product[e42] * self[e41], -(geometric_anti_product[e42] * self[e42]) - (geometric_anti_product[e43] * self[e43])) - float4((geometric_anti_product.group0().yzx * self.group0().zxy), geometric_anti_product[e41] * self[e41]), 
            /* e23, e31, e12, scalar */
            float4((geometric_anti_product[e43] * self[e31]) + (geometric_anti_product[e12] * self[e42]), (geometric_anti_product[e41] * self[e12]) + (geometric_anti_product[e23] * self[e43]), (geometric_anti_product[e42] * self[e23]) + (geometric_anti_product[e31] * self[e41]), -(geometric_anti_product[e43] * self[e12]) - (geometric_anti_product[e23] * self[e41]) - (geometric_anti_product[e31] * self[e42]) - (geometric_anti_product[e12] * self[e43])) - float4((geometric_anti_product.group0().yzx * self.group1().zxy), geometric_anti_product[e41] * self[e23]) - float4((geometric_anti_product.group1().yzx * self.group0().zxy), geometric_anti_product[e42] * self[e31])
        );
    }
}
extension Line: GeometricAntiQuotient<Motor> {
    func geometric_anti_quotient(self, other: Motor) -> Motor {
Motor anti_reverse = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            other.group0() * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            other.group1() * float4(-1.0, -1.0, -1.0, 1.0)
        );
AntiScalar other_2 = AntiScalar::from_groups(
            /* e1234 */
            powi(other[e41], 2) + powi(other[e42], 2) + powi(other[e43], 2) + powi(other[e1234], 2)
        );
Motor geometric_anti_product = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(other_2[e1234]) * anti_reverse.group0(), 
            /* e23, e31, e12, scalar */
            float4(other_2[e1234]) * anti_reverse.group1()
        );
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4((self[e41] * geometric_anti_product[e1234]) + (self[e42] * geometric_anti_product[e43]), (self[e42] * geometric_anti_product[e1234]) + (self[e43] * geometric_anti_product[e41]), (self[e41] * geometric_anti_product[e42]) + (self[e43] * geometric_anti_product[e1234]), -(self[e42] * geometric_anti_product[e42]) - (self[e43] * geometric_anti_product[e43])) - (geometric_anti_product.group0().yzxx * float4(self.group0().zxy, self[e41])), 
            /* e23, e31, e12, scalar */
            float4((self[e41] * geometric_anti_product[scalar]) + (self[e42] * geometric_anti_product[e12]) + (self[e23] * geometric_anti_product[e1234]) + (self[e31] * geometric_anti_product[e43]), (self[e42] * geometric_anti_product[scalar]) + (self[e43] * geometric_anti_product[e23]) + (self[e31] * geometric_anti_product[e1234]) + (self[e12] * geometric_anti_product[e41]), (self[e41] * geometric_anti_product[e31]) + (self[e43] * geometric_anti_product[scalar]) + (self[e23] * geometric_anti_product[e42]) + (self[e12] * geometric_anti_product[e1234]), -(self[e43] * geometric_anti_product[e12]) - (self[e23] * geometric_anti_product[e41]) - (self[e31] * geometric_anti_product[e42]) - (self[e12] * geometric_anti_product[e43])) - (geometric_anti_product.group1().yzxx * float4(self.group0().zxy, self[e41])) - float4((self.group1().zxy * geometric_anti_product.group0().yzx), self[e42] * geometric_anti_product[e31])
        );
    }
}
extension Line: GeometricAntiQuotient<MultiVector> {
    func geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
MultiVector anti_reverse = MultiVector::from_groups(
            /* scalar, e1234 */
            other.group0(), 
            /* e1, e2, e3, e4 */
            other.group1() * float4(-1.0), 
            /* e41, e42, e43 */
            other.group2() * float3(-1.0), 
            /* e23, e31, e12 */
            other.group3() * float3(-1.0), 
            /* e423, e431, e412, e321 */
            other.group4()
        );
AntiScalar other_2 = AntiScalar::from_groups(
            /* e1234 */
            powi(other[e1234], 2) + powi(other[e4], 2) + powi(other[e41], 2) + powi(other[e42], 2) + powi(other[e43], 2) + powi(other[e423], 2) + powi(other[e431], 2) + powi(other[e412], 2)
        );
MultiVector geometric_anti_product = MultiVector::from_groups(
            /* scalar, e1234 */
            float2(other_2[e1234]) * anti_reverse.group0(), 
            /* e1, e2, e3, e4 */
            float4(other_2[e1234]) * anti_reverse.group1(), 
            /* e41, e42, e43 */
            float3(other_2[e1234]) * anti_reverse.group2(), 
            /* e23, e31, e12 */
            float3(other_2[e1234]) * anti_reverse.group3(), 
            /* e423, e431, e412, e321 */
            float4(other_2[e1234]) * anti_reverse.group4()
        );
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(-(self[e23] * geometric_anti_product[e41]) - (self[e31] * geometric_anti_product[e42]) - (self[e12] * geometric_anti_product[e43]), 0.0) - (float2(self[e41]) * float2(geometric_anti_product[e23], geometric_anti_product[e41])) - (float2(self[e42]) * float2(geometric_anti_product[e31], geometric_anti_product[e42])) - (float2(self[e43]) * float2(geometric_anti_product[e12], geometric_anti_product[e43])), 
            /* e1, e2, e3, e4 */
            float4((self[e41] * geometric_anti_product[e321]) + (self[e42] * geometric_anti_product[e3]) + (self[e23] * geometric_anti_product[e4]) + (self[e31] * geometric_anti_product[e412]), (self[e42] * geometric_anti_product[e321]) + (self[e43] * geometric_anti_product[e1]) + (self[e31] * geometric_anti_product[e4]) + (self[e12] * geometric_anti_product[e423]), (self[e41] * geometric_anti_product[e2]) + (self[e43] * geometric_anti_product[e321]) + (self[e23] * geometric_anti_product[e431]) + (self[e12] * geometric_anti_product[e4]), self[e43] * geometric_anti_product[e412] * -1.0) - (geometric_anti_product.group4().yzxy * float4(self.group1().zxy, self[e42])) - float4((self.group0().zxy * geometric_anti_product.group1().yzx), self[e41] * geometric_anti_product[e423]), 
            /* e41, e42, e43 */
            (float3(geometric_anti_product[e1234]) * self.group0()) + (self.group0().yzx * geometric_anti_product.group2().zxy) - (self.group0().zxy * geometric_anti_product.group2().yzx), 
            /* e23, e31, e12 */
            (float3(geometric_anti_product[scalar]) * self.group0()) + (float3(geometric_anti_product[e1234]) * self.group1()) + (self.group0().yzx * geometric_anti_product.group3().zxy) + (self.group1().yzx * geometric_anti_product.group2().zxy) - (self.group0().zxy * geometric_anti_product.group3().yzx) - (self.group1().zxy * geometric_anti_product.group2().yzx), 
            /* e423, e431, e412, e321 */
            float4((self[e41] * geometric_anti_product[e4]) + (self[e42] * geometric_anti_product[e412]), (self[e42] * geometric_anti_product[e4]) + (self[e43] * geometric_anti_product[e423]), (self[e41] * geometric_anti_product[e431]) + (self[e43] * geometric_anti_product[e4]), -(self[e42] * geometric_anti_product[e2]) - (self[e43] * geometric_anti_product[e3]) - (self[e23] * geometric_anti_product[e423]) - (self[e31] * geometric_anti_product[e431]) - (self[e12] * geometric_anti_product[e412])) - float4((self.group0().zxy * geometric_anti_product.group4().yzx), self[e41] * geometric_anti_product[e1])
        );
    }
}
extension Line: GeometricAntiQuotient<Origin> {
    func geometric_anti_quotient(self, other: Origin) -> Flector {
Origin geometric_anti_product = Origin::from_groups(
            /* e4 */
            1.0/other[e4] * -1.0
        );
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(geometric_anti_product[e4], geometric_anti_product[e4], geometric_anti_product[e4], 0.0) * float4(self.group1(), 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e423, e431, e412, e321 */
            float4(geometric_anti_product[e4], geometric_anti_product[e4], geometric_anti_product[e4], 0.0) * float4(self.group0(), 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Line: GeometricAntiQuotient<Plane> {
    func geometric_anti_quotient(self, other: Plane) -> Flector {
Plane geometric_anti_product = Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4(powi(other[e423], 2) + powi(other[e431], 2) + powi(other[e412], 2)) * other.group0()
        );
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4((self[e41] * geometric_anti_product[e321]) + (self[e31] * geometric_anti_product[e412]), (self[e42] * geometric_anti_product[e321]) + (self[e12] * geometric_anti_product[e423]), (self[e43] * geometric_anti_product[e321]) + (self[e23] * geometric_anti_product[e431]), -(self[e42] * geometric_anti_product[e431]) - (self[e43] * geometric_anti_product[e412])) - (geometric_anti_product.group0().yzxx * float4(self.group1().zxy, self[e41])), 
            /* e423, e431, e412, e321 */
            float4(self[e42] * geometric_anti_product[e412], self[e43] * geometric_anti_product[e423], self[e41] * geometric_anti_product[e431], -(self[e31] * geometric_anti_product[e431]) - (self[e12] * geometric_anti_product[e412])) - (geometric_anti_product.group0().yzxx * float4(self.group0().zxy, self[e23]))
        );
    }
}
extension Line: GeometricAntiQuotient<Point> {
    func geometric_anti_quotient(self, other: Point) -> Flector {
Point geometric_anti_product = Point::from_groups(
            /* e1, e2, e3, e4 */
            float4(powi(other[e4], -2)) * float4(other[e1] * -1.0, other[e2] * -1.0, other[e3] * -1.0, other[e4] * -1.0)
        );
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4((float3(geometric_anti_product[e4]) * self.group1()), 0.0) + float4((self.group0().yzx * geometric_anti_product.group0().zxy), 0.0) - float4((self.group0().zxy * geometric_anti_product.group0().yzx), 0.0), 
            /* e423, e431, e412, e321 */
            float4(geometric_anti_product[e4], geometric_anti_product[e4], geometric_anti_product[e4], 1.0) * float4(self.group0(), -(self[e41] * geometric_anti_product[e1]) - (self[e42] * geometric_anti_product[e2]) - (self[e43] * geometric_anti_product[e3]))
        );
    }
}
extension Motor {
    func div(_rhs: geometric_anti_quotient) -> geometric_anti_quotient_partial<Motor> {
        geometric_anti_quotient_partial(this)
    }
}
extension Motor: GeometricAntiQuotient<AntiScalar> {
    func geometric_anti_quotient(self, other: AntiScalar) -> Motor {
AntiScalar geometric_anti_product = AntiScalar::from_groups(
            /* e1234 */
            1.0/other[e1234]
        );
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(geometric_anti_product[e1234]) * self.group0(), 
            /* e23, e31, e12, scalar */
            float4(geometric_anti_product[e1234]) * self.group1()
        );
    }
}
extension Motor: GeometricAntiQuotient<DualNum> {
    func geometric_anti_quotient(self, other: DualNum) -> Motor {
DualNum geometric_anti_product = DualNum::from_groups(
            /* scalar, e1234 */
            float2(powi(other[e1234], -2)) * other.group0()
        );
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(geometric_anti_product[e1234]) * self.group0(), 
            /* e23, e31, e12, scalar */
            (float4(geometric_anti_product[scalar]) * self.group0()) + (float4(geometric_anti_product[e1234]) * self.group1())
        );
    }
}
extension Motor: GeometricAntiQuotient<Flector> {
    func geometric_anti_quotient(self, other: Flector) -> Flector {
Flector anti_reverse = Flector::from_groups(
            /* e1, e2, e3, e4 */
            other.group0() * float4(-1.0), 
            /* e423, e431, e412, e321 */
            other.group1()
        );
AntiScalar other_2 = AntiScalar::from_groups(
            /* e1234 */
            powi(other[e4], 2) + powi(other[e423], 2) + powi(other[e431], 2) + powi(other[e412], 2)
        );
Flector geometric_anti_product = Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(other_2[e1234]) * anti_reverse.group0(), 
            /* e423, e431, e412, e321 */
            float4(other_2[e1234]) * anti_reverse.group1()
        );
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4((geometric_anti_product[e3] * self[e42]) + (geometric_anti_product[e4] * self[e23]) + (geometric_anti_product[e423] * self[scalar]) + (geometric_anti_product[e412] * self[e31]) + (geometric_anti_product[e321] * self[e41]), (geometric_anti_product[e2] * self[e1234]) + (geometric_anti_product[e4] * self[e31]) + (geometric_anti_product[e423] * self[e12]) + (geometric_anti_product[e431] * self[scalar]) + (geometric_anti_product[e321] * self[e42]), (geometric_anti_product[e3] * self[e1234]) + (geometric_anti_product[e4] * self[e12]) + (geometric_anti_product[e431] * self[e23]) + (geometric_anti_product[e412] * self[scalar]) + (geometric_anti_product[e321] * self[e43]), geometric_anti_product[e412] * self[e43] * -1.0) + (geometric_anti_product.group0().xxyw * self.group0().wzxw) - (geometric_anti_product.group1().yzxy * float4(self.group1().zxy, self[e42])) - (self.group0().zxyx * float4(geometric_anti_product.group0().yzx, geometric_anti_product[e423])), 
            /* e423, e431, e412, e321 */
            float4(geometric_anti_product[e412] * self[e42], geometric_anti_product[e431] * self[e1234], geometric_anti_product[e412] * self[e1234], -(geometric_anti_product[e2] * self[e42]) - (geometric_anti_product[e3] * self[e43]) - (geometric_anti_product[e423] * self[e23]) - (geometric_anti_product[e431] * self[e31]) - (geometric_anti_product[e412] * self[e12])) + (float4(geometric_anti_product[e4]) * float4(self.group0().xyz, self[scalar])) + (geometric_anti_product.group1().xxyw * self.group0().wzxw) - (self.group0().zxyx * float4(geometric_anti_product.group1().yzx, geometric_anti_product[e1]))
        );
    }
}
extension Motor: GeometricAntiQuotient<Line> {
    func geometric_anti_quotient(self, other: Line) -> Motor {
Line anti_reverse = Line::from_groups(
            /* e41, e42, e43 */
            other.group0() * float3(-1.0), 
            /* e23, e31, e12 */
            other.group1() * float3(-1.0)
        );
AntiScalar other_2 = AntiScalar::from_groups(
            /* e1234 */
            powi(other[e41], 2) + powi(other[e42], 2) + powi(other[e43], 2)
        );
Line geometric_anti_product = Line::from_groups(
            /* e41, e42, e43 */
            float3(other_2[e1234]) * anti_reverse.group0(), 
            /* e23, e31, e12 */
            float3(other_2[e1234]) * anti_reverse.group1()
        );
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4((geometric_anti_product[e41] * self[e1234]) + (geometric_anti_product[e43] * self[e42]), (geometric_anti_product[e41] * self[e43]) + (geometric_anti_product[e42] * self[e1234]), (geometric_anti_product[e42] * self[e41]) + (geometric_anti_product[e43] * self[e1234]), -(geometric_anti_product[e42] * self[e42]) - (geometric_anti_product[e43] * self[e43])) - (self.group0().zxyx * float4(geometric_anti_product.group0().yzx, geometric_anti_product[e41])), 
            /* e23, e31, e12, scalar */
            float4((geometric_anti_product[e41] * self[scalar]) + (geometric_anti_product[e43] * self[e31]) + (geometric_anti_product[e23] * self[e1234]) + (geometric_anti_product[e12] * self[e42]), (geometric_anti_product[e41] * self[e12]) + (geometric_anti_product[e42] * self[scalar]) + (geometric_anti_product[e23] * self[e43]) + (geometric_anti_product[e31] * self[e1234]), (geometric_anti_product[e42] * self[e23]) + (geometric_anti_product[e43] * self[scalar]) + (geometric_anti_product[e31] * self[e41]) + (geometric_anti_product[e12] * self[e1234]), -(geometric_anti_product[e43] * self[e12]) - (geometric_anti_product[e23] * self[e41]) - (geometric_anti_product[e31] * self[e42]) - (geometric_anti_product[e12] * self[e43])) - (self.group1().zxyx * float4(geometric_anti_product.group0().yzx, geometric_anti_product[e41])) - float4((geometric_anti_product.group1().yzx * self.group0().zxy), geometric_anti_product[e42] * self[e31])
        );
    }
}
extension Motor: GeometricAntiQuotient<Motor> {
    func geometric_anti_quotient(self, other: Motor) -> Motor {
Motor anti_reverse = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            other.group0() * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            other.group1() * float4(-1.0, -1.0, -1.0, 1.0)
        );
AntiScalar other_2 = AntiScalar::from_groups(
            /* e1234 */
            powi(other[e41], 2) + powi(other[e42], 2) + powi(other[e43], 2) + powi(other[e1234], 2)
        );
Motor geometric_anti_product = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(other_2[e1234]) * anti_reverse.group0(), 
            /* e23, e31, e12, scalar */
            float4(other_2[e1234]) * anti_reverse.group1()
        );
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4((geometric_anti_product[e43] * self[e42]) + (geometric_anti_product[e1234] * self[e41]), (geometric_anti_product[e42] * self[e1234]) + (geometric_anti_product[e1234] * self[e42]), (geometric_anti_product[e43] * self[e1234]) + (geometric_anti_product[e1234] * self[e43]), -(geometric_anti_product[e42] * self[e42]) - (geometric_anti_product[e43] * self[e43])) + (geometric_anti_product.group0().xxyw * self.group0().wzxw) - (geometric_anti_product.group0().yzxx * self.group0().zxyx), 
            /* e23, e31, e12, scalar */
            float4((geometric_anti_product[e1234] * self[e23]) + (geometric_anti_product[e23] * self[e1234]) + (geometric_anti_product[e12] * self[e42]) + (geometric_anti_product[scalar] * self[e41]), (geometric_anti_product[e1234] * self[e31]) + (geometric_anti_product[e23] * self[e43]) + (geometric_anti_product[e31] * self[e1234]) + (geometric_anti_product[scalar] * self[e42]), (geometric_anti_product[e1234] * self[e12]) + (geometric_anti_product[e31] * self[e41]) + (geometric_anti_product[e12] * self[e1234]) + (geometric_anti_product[scalar] * self[e43]), -(geometric_anti_product[e43] * self[e12]) - (geometric_anti_product[e23] * self[e41]) - (geometric_anti_product[e31] * self[e42]) - (geometric_anti_product[e12] * self[e43])) + (geometric_anti_product.group0().xxyw * self.group1().wzxw) + float4((geometric_anti_product.group0().zyz * self.group1().yww), geometric_anti_product[scalar] * self[e1234]) - (geometric_anti_product.group0().yzxx * self.group1().zxyx) - float4((geometric_anti_product.group1().yzx * self.group0().zxy), geometric_anti_product[e42] * self[e31])
        );
    }
}
extension Motor: GeometricAntiQuotient<MultiVector> {
    func geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
MultiVector anti_reverse = MultiVector::from_groups(
            /* scalar, e1234 */
            other.group0(), 
            /* e1, e2, e3, e4 */
            other.group1() * float4(-1.0), 
            /* e41, e42, e43 */
            other.group2() * float3(-1.0), 
            /* e23, e31, e12 */
            other.group3() * float3(-1.0), 
            /* e423, e431, e412, e321 */
            other.group4()
        );
AntiScalar other_2 = AntiScalar::from_groups(
            /* e1234 */
            powi(other[e1234], 2) + powi(other[e4], 2) + powi(other[e41], 2) + powi(other[e42], 2) + powi(other[e43], 2) + powi(other[e423], 2) + powi(other[e431], 2) + powi(other[e412], 2)
        );
MultiVector geometric_anti_product = MultiVector::from_groups(
            /* scalar, e1234 */
            float2(other_2[e1234]) * anti_reverse.group0(), 
            /* e1, e2, e3, e4 */
            float4(other_2[e1234]) * anti_reverse.group1(), 
            /* e41, e42, e43 */
            float3(other_2[e1234]) * anti_reverse.group2(), 
            /* e23, e31, e12 */
            float3(other_2[e1234]) * anti_reverse.group3(), 
            /* e423, e431, e412, e321 */
            float4(other_2[e1234]) * anti_reverse.group4()
        );
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2((self[scalar] * geometric_anti_product[e1234]) - (self[e41] * geometric_anti_product[e23]) - (self[e42] * geometric_anti_product[e31]) - (self[e43] * geometric_anti_product[e12]), 0.0) + (float2(self[e1234]) * geometric_anti_product.group0()) - (float2(geometric_anti_product[e41]) * float2(self[e23], self[e41])) - (float2(geometric_anti_product[e42]) * float2(self[e31], self[e42])) - (float2(geometric_anti_product[e43]) * float2(self[e12], self[e43])), 
            /* e1, e2, e3, e4 */
            float4((self[e42] * geometric_anti_product[e3]) + (self[e1234] * geometric_anti_product[e1]) + (self[e23] * geometric_anti_product[e4]) + (self[e31] * geometric_anti_product[e412]) + (self[scalar] * geometric_anti_product[e423]), (self[e43] * geometric_anti_product[e1]) + (self[e1234] * geometric_anti_product[e2]) + (self[e31] * geometric_anti_product[e4]) + (self[e12] * geometric_anti_product[e423]) + (self[scalar] * geometric_anti_product[e431]), (self[e43] * geometric_anti_product[e321]) + (self[e1234] * geometric_anti_product[e3]) + (self[e23] * geometric_anti_product[e431]) + (self[e12] * geometric_anti_product[e4]) + (self[scalar] * geometric_anti_product[e412]), self[e43] * geometric_anti_product[e412] * -1.0) + (self.group0().xyxw * float4(geometric_anti_product.group4().ww, geometric_anti_product[e2], geometric_anti_product[e4])) - (self.group0().zxyx * float4(geometric_anti_product.group1().yzx, geometric_anti_product[e423])) - (geometric_anti_product.group4().yzxy * float4(self.group1().zxy, self[e42])), 
            /* e41, e42, e43 */
            (float3(geometric_anti_product[e1234]) * self.group0().xyz) + (geometric_anti_product.group2().xxy * self.group0().wzx) + (geometric_anti_product.group2().zyz * self.group0().yww) - (geometric_anti_product.group2().yzx * self.group0().zxy), 
            /* e23, e31, e12 */
            (float3(geometric_anti_product[scalar]) * self.group0().xyz) + (float3(geometric_anti_product[e1234]) * self.group1().xyz) + (geometric_anti_product.group2().xxy * self.group1().wzx) + (geometric_anti_product.group2().zyz * self.group1().yww) + (geometric_anti_product.group3().xxy * self.group0().wzx) + (geometric_anti_product.group3().zyz * self.group0().yww) - (geometric_anti_product.group2().yzx * self.group1().zxy) - (geometric_anti_product.group3().yzx * self.group0().zxy), 
            /* e423, e431, e412, e321 */
            float4(self[e1234] * geometric_anti_product[e423], self[e1234] * geometric_anti_product[e431], self[e1234] * geometric_anti_product[e412], -(self[e42] * geometric_anti_product[e2]) - (self[e43] * geometric_anti_product[e3]) - (self[e23] * geometric_anti_product[e423]) - (self[e31] * geometric_anti_product[e431]) - (self[e12] * geometric_anti_product[e412])) + (self.group0().xyxw * float4(geometric_anti_product.group1().ww, geometric_anti_product[e431], geometric_anti_product[e321])) + (float4(geometric_anti_product.group4().zx, geometric_anti_product[e4], geometric_anti_product[e4]) * float4(self.group0().yzz, self[scalar])) - (self.group0().zxyx * float4(geometric_anti_product.group4().yzx, geometric_anti_product[e1]))
        );
    }
}
extension Motor: GeometricAntiQuotient<Origin> {
    func geometric_anti_quotient(self, other: Origin) -> Flector {
Origin geometric_anti_product = Origin::from_groups(
            /* e4 */
            1.0/other[e4] * -1.0
        );
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(geometric_anti_product[e4]) * float4(self.group1().xyz, self[e1234]), 
            /* e423, e431, e412, e321 */
            float4(geometric_anti_product[e4]) * float4(self.group0().xyz, self[scalar])
        );
    }
}
extension Motor: GeometricAntiQuotient<Plane> {
    func geometric_anti_quotient(self, other: Plane) -> Flector {
Plane geometric_anti_product = Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4(powi(other[e423], 2) + powi(other[e431], 2) + powi(other[e412], 2)) * other.group0()
        );
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4((self[e41] * geometric_anti_product[e321]) + (self[e31] * geometric_anti_product[e412]) + (self[scalar] * geometric_anti_product[e423]), (self[e42] * geometric_anti_product[e321]) + (self[e12] * geometric_anti_product[e423]) + (self[scalar] * geometric_anti_product[e431]), (self[e43] * geometric_anti_product[e321]) + (self[e23] * geometric_anti_product[e431]) + (self[scalar] * geometric_anti_product[e412]), -(self[e42] * geometric_anti_product[e431]) - (self[e43] * geometric_anti_product[e412])) - (geometric_anti_product.group0().yzxx * float4(self.group1().zxy, self[e41])), 
            /* e423, e431, e412, e321 */
            float4(self[e1234] * geometric_anti_product[e423], self[e1234] * geometric_anti_product[e431], self[e1234] * geometric_anti_product[e412], -(self[e31] * geometric_anti_product[e431]) - (self[e12] * geometric_anti_product[e412])) + (self.group0().yzxw * geometric_anti_product.group0().zxyw) - (geometric_anti_product.group0().yzxx * float4(self.group0().zxy, self[e23]))
        );
    }
}
extension Motor: GeometricAntiQuotient<Point> {
    func geometric_anti_quotient(self, other: Point) -> Flector {
Point geometric_anti_product = Point::from_groups(
            /* e1, e2, e3, e4 */
            float4(powi(other[e4], -2)) * float4(other[e1] * -1.0, other[e2] * -1.0, other[e3] * -1.0, other[e4] * -1.0)
        );
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(((float3(self[e1234]) * geometric_anti_product.group0().xyz) + (float3(geometric_anti_product[e4]) * self.group1().xyz) + (self.group0().yzx * geometric_anti_product.group0().zxy) - (self.group0().zxy * geometric_anti_product.group0().yzx)), self[e1234] * geometric_anti_product[e4]), 
            /* e423, e431, e412, e321 */
            float4(geometric_anti_product[e4], geometric_anti_product[e4], geometric_anti_product[e4], 1.0) * float4(self.group0().xyz, (self[scalar] * geometric_anti_product[e4]) - (self[e41] * geometric_anti_product[e1]) - (self[e42] * geometric_anti_product[e2]) - (self[e43] * geometric_anti_product[e3]))
        );
    }
}
extension MultiVector {
    func div(_rhs: geometric_anti_quotient) -> geometric_anti_quotient_partial<MultiVector> {
        geometric_anti_quotient_partial(this)
    }
}
extension MultiVector: GeometricAntiQuotient<AntiScalar> {
    func geometric_anti_quotient(self, other: AntiScalar) -> MultiVector {
AntiScalar geometric_anti_product = AntiScalar::from_groups(
            /* e1234 */
            1.0/other[e1234]
        );
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(geometric_anti_product[e1234]) * self.group0(), 
            /* e1, e2, e3, e4 */
            float4(geometric_anti_product[e1234]) * self.group1(), 
            /* e41, e42, e43 */
            float3(geometric_anti_product[e1234]) * self.group2(), 
            /* e23, e31, e12 */
            float3(geometric_anti_product[e1234]) * self.group3(), 
            /* e423, e431, e412, e321 */
            float4(geometric_anti_product[e1234]) * self.group4()
        );
    }
}
extension MultiVector: GeometricAntiQuotient<DualNum> {
    func geometric_anti_quotient(self, other: DualNum) -> MultiVector {
DualNum geometric_anti_product = DualNum::from_groups(
            /* scalar, e1234 */
            float2(powi(other[e1234], -2)) * other.group0()
        );
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2((geometric_anti_product[scalar] * self[e1234]) + (geometric_anti_product[e1234] * self[scalar]), geometric_anti_product[e1234] * self[e1234]), 
            /* e1, e2, e3, e4 */
            float4(((float3(geometric_anti_product[e1234]) * self.group1().xyz) - (float3(geometric_anti_product[scalar]) * self.group4().xyz)), geometric_anti_product[e1234] * self[e4]), 
            /* e41, e42, e43 */
            float3(geometric_anti_product[e1234]) * self.group2(), 
            /* e23, e31, e12 */
            (float3(geometric_anti_product[scalar]) * self.group2()) + (float3(geometric_anti_product[e1234]) * self.group3()), 
            /* e423, e431, e412, e321 */
            float4(self[e423], self[e431], self[e412], 1.0) * float4(geometric_anti_product.group0().yy, geometric_anti_product[e1234], (geometric_anti_product[e1234] * self[e321]) - (geometric_anti_product[scalar] * self[e4]))
        );
    }
}
extension MultiVector: GeometricAntiQuotient<Flector> {
    func geometric_anti_quotient(self, other: Flector) -> MultiVector {
Flector anti_reverse = Flector::from_groups(
            /* e1, e2, e3, e4 */
            other.group0() * float4(-1.0), 
            /* e423, e431, e412, e321 */
            other.group1()
        );
AntiScalar other_2 = AntiScalar::from_groups(
            /* e1234 */
            powi(other[e4], 2) + powi(other[e423], 2) + powi(other[e431], 2) + powi(other[e412], 2)
        );
Flector geometric_anti_product = Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(other_2[e1234]) * anti_reverse.group0(), 
            /* e423, e431, e412, e321 */
            float4(other_2[e1234]) * anti_reverse.group1()
        );
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2((geometric_anti_product[e321] * self[e4]) - (geometric_anti_product[e2] * self[e431]) - (geometric_anti_product[e3] * self[e412]) - (geometric_anti_product[e4] * self[e321]), 0.0) + (float2(geometric_anti_product[e423]) * float2(self[e1], self[e423])) + (float2(geometric_anti_product[e431]) * float2(self[e2], self[e431])) + (float2(geometric_anti_product[e412]) * float2(self[e3], self[e412])) - (float2(self[e423], self[e4]) * geometric_anti_product.group0().xw), 
            /* e1, e2, e3, e4 */
            float4((geometric_anti_product[e1] * self[e1234]) + (geometric_anti_product[e3] * self[e42]) + (geometric_anti_product[e4] * self[e23]) + (geometric_anti_product[e412] * self[e31]) + (geometric_anti_product[e321] * self[e41]), (geometric_anti_product[e1] * self[e43]) + (geometric_anti_product[e2] * self[e1234]) + (geometric_anti_product[e4] * self[e31]) + (geometric_anti_product[e423] * self[e12]) + (geometric_anti_product[e321] * self[e42]), (geometric_anti_product[e2] * self[e41]) + (geometric_anti_product[e3] * self[e1234]) + (geometric_anti_product[e4] * self[e12]) + (geometric_anti_product[e431] * self[e23]) + (geometric_anti_product[e321] * self[e43]), geometric_anti_product[e412] * self[e43] * -1.0) + (float4(self.group0().xx, self[scalar], self[e1234]) * float4(geometric_anti_product.group1().xyz, geometric_anti_product[e4])) - (geometric_anti_product.group1().yzxy * float4(self.group3().zxy, self[e42])) - float4((self.group2().zxy * geometric_anti_product.group0().yzx), geometric_anti_product[e423] * self[e41]), 
            /* e41, e42, e43 */
            (geometric_anti_product.group1().yzx * self.group4().zxy) - (float3(geometric_anti_product[e4]) * self.group4().xyz) - (float3(self[e4], self[e412], self[e423]) * geometric_anti_product.group1().xxy) - (float3(self[e431], self[e4], self[e4]) * geometric_anti_product.group1().zyz), 
            /* e23, e31, e12 */
            (float3(geometric_anti_product[e321]) * self.group4().xyz) + (float3(self[e4], self[e412], self[e423]) * geometric_anti_product.group0().xxy) + (float3(self[e431], self[e4], self[e4]) * geometric_anti_product.group0().zyz) + (geometric_anti_product.group1().yzx * self.group1().zxy) - (float3(geometric_anti_product[e4]) * self.group1().xyz) - (float3(self[e2], self[e321], self[e321]) * geometric_anti_product.group1().zyz) - (float3(self[e321], self[e3], self[e1]) * geometric_anti_product.group1().xxy) - (geometric_anti_product.group0().yzx * self.group4().zxy), 
            /* e423, e431, e412, e321 */
            float4(geometric_anti_product[e412] * self[e42], geometric_anti_product[e423] * self[e43], geometric_anti_product[e4] * self[e43], -(geometric_anti_product[e2] * self[e42]) - (geometric_anti_product[e3] * self[e43]) - (geometric_anti_product[e423] * self[e23]) - (geometric_anti_product[e431] * self[e31]) - (geometric_anti_product[e412] * self[e12])) + (float4(self.group0().yy, self[e1234], self[scalar]) * float4(geometric_anti_product.group1().xyz, geometric_anti_product[e4])) + (float4(geometric_anti_product.group0().ww, geometric_anti_product[e431], geometric_anti_product[e321]) * float4(self.group2().xyx, self[e1234])) - float4((self.group2().zxy * geometric_anti_product.group1().yzx), geometric_anti_product[e1] * self[e41])
        );
    }
}
extension MultiVector: GeometricAntiQuotient<Line> {
    func geometric_anti_quotient(self, other: Line) -> MultiVector {
Line anti_reverse = Line::from_groups(
            /* e41, e42, e43 */
            other.group0() * float3(-1.0), 
            /* e23, e31, e12 */
            other.group1() * float3(-1.0)
        );
AntiScalar other_2 = AntiScalar::from_groups(
            /* e1234 */
            powi(other[e41], 2) + powi(other[e42], 2) + powi(other[e43], 2)
        );
Line geometric_anti_product = Line::from_groups(
            /* e41, e42, e43 */
            float3(other_2[e1234]) * anti_reverse.group0(), 
            /* e23, e31, e12 */
            float3(other_2[e1234]) * anti_reverse.group1()
        );
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(-(geometric_anti_product[e23] * self[e41]) - (geometric_anti_product[e31] * self[e42]) - (geometric_anti_product[e12] * self[e43]), 0.0) - (float2(geometric_anti_product[e41]) * float2(self[e23], self[e41])) - (float2(geometric_anti_product[e42]) * float2(self[e31], self[e42])) - (float2(geometric_anti_product[e43]) * float2(self[e12], self[e43])), 
            /* e1, e2, e3, e4 */
            float4((geometric_anti_product[e41] * self[e321]) + (geometric_anti_product[e43] * self[e2]) + (geometric_anti_product[e31] * self[e412]), (geometric_anti_product[e41] * self[e3]) + (geometric_anti_product[e42] * self[e321]) + (geometric_anti_product[e12] * self[e423]), (geometric_anti_product[e42] * self[e1]) + (geometric_anti_product[e43] * self[e321]) + (geometric_anti_product[e23] * self[e431]), 0.0) - (float4(self[e4], self[e412], self[e423], self[e431]) * float4(geometric_anti_product.group1().xxy, geometric_anti_product[e42])) - (float4(self[e431], self[e4], self[e4], self[e412]) * float4(geometric_anti_product.group1().zyz, geometric_anti_product[e43])) - float4((geometric_anti_product.group0().yzx * self.group1().zxy), geometric_anti_product[e41] * self[e423]), 
            /* e41, e42, e43 */
            (float3(self[e1234]) * geometric_anti_product.group0()) + (geometric_anti_product.group0().zxy * self.group2().yzx) - (geometric_anti_product.group0().yzx * self.group2().zxy), 
            /* e23, e31, e12 */
            (float3(self[scalar]) * geometric_anti_product.group0()) + (float3(self[e1234]) * geometric_anti_product.group1()) + (geometric_anti_product.group0().zxy * self.group3().yzx) + (geometric_anti_product.group1().zxy * self.group2().yzx) - (geometric_anti_product.group0().yzx * self.group3().zxy) - (geometric_anti_product.group1().yzx * self.group2().zxy), 
            /* e423, e431, e412, e321 */
            (float4(self[e4], self[e412], self[e423], self[e423]) * float4(geometric_anti_product.group0().xxy, geometric_anti_product[e23])) + (float4(self[e431], self[e4], self[e4], self[e431]) * float4(geometric_anti_product.group0().zyz, geometric_anti_product[e31])) + float4(float3(0.0), (geometric_anti_product[e12] * self[e412]) - (geometric_anti_product[e42] * self[e2]) - (geometric_anti_product[e43] * self[e3])) - float4((geometric_anti_product.group0().yzx * self.group4().zxy), geometric_anti_product[e41] * self[e1])
        );
    }
}
extension MultiVector: GeometricAntiQuotient<Motor> {
    func geometric_anti_quotient(self, other: Motor) -> MultiVector {
Motor anti_reverse = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            other.group0() * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            other.group1() * float4(-1.0, -1.0, -1.0, 1.0)
        );
AntiScalar other_2 = AntiScalar::from_groups(
            /* e1234 */
            powi(other[e41], 2) + powi(other[e42], 2) + powi(other[e43], 2) + powi(other[e1234], 2)
        );
Motor geometric_anti_product = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(other_2[e1234]) * anti_reverse.group0(), 
            /* e23, e31, e12, scalar */
            float4(other_2[e1234]) * anti_reverse.group1()
        );
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2((geometric_anti_product[scalar] * self[e1234]) - (geometric_anti_product[e41] * self[e23]) - (geometric_anti_product[e42] * self[e31]) - (geometric_anti_product[e43] * self[e12]), 0.0) + (float2(geometric_anti_product[e1234]) * self.group0()) - (float2(self[e41]) * float2(geometric_anti_product[e23], geometric_anti_product[e41])) - (float2(self[e42]) * float2(geometric_anti_product[e31], geometric_anti_product[e42])) - (float2(self[e43]) * float2(geometric_anti_product[e12], geometric_anti_product[e43])), 
            /* e1, e2, e3, e4 */
            float4((geometric_anti_product[e43] * self[e2]) + (geometric_anti_product[e1234] * self[e1]) + (geometric_anti_product[e31] * self[e412]) - (geometric_anti_product[scalar] * self[e423]), (geometric_anti_product[e42] * self[e321]) + (geometric_anti_product[e1234] * self[e2]) + (geometric_anti_product[e12] * self[e423]) - (geometric_anti_product[scalar] * self[e431]), (geometric_anti_product[e43] * self[e321]) + (geometric_anti_product[e1234] * self[e3]) + (geometric_anti_product[e23] * self[e431]) - (geometric_anti_product[scalar] * self[e412]), 0.0) + (float4(self[e321], self[e3], self[e1], self[e4]) * geometric_anti_product.group0().xxyw) - (float4(self[e4], self[e412], self[e423], self[e431]) * float4(geometric_anti_product.group1().xxy, geometric_anti_product[e42])) - (float4(self[e431], self[e4], self[e4], self[e412]) * float4(geometric_anti_product.group1().zyz, geometric_anti_product[e43])) - (geometric_anti_product.group0().yzxx * float4(self.group1().zxy, self[e423])), 
            /* e41, e42, e43 */
            (float3(self[e1234]) * geometric_anti_product.group0().xyz) + (self.group2().xyx * geometric_anti_product.group0().wwy) + (self.group2().yzz * geometric_anti_product.group0().zxw) - (self.group2().zxy * geometric_anti_product.group0().yzx), 
            /* e23, e31, e12 */
            (float3(self[scalar]) * geometric_anti_product.group0().xyz) + (float3(self[e1234]) * geometric_anti_product.group1().xyz) + (self.group2().xyx * geometric_anti_product.group1().wwy) + (self.group2().yzz * geometric_anti_product.group1().zxw) + (self.group3().xyx * geometric_anti_product.group0().wwy) + (self.group3().yzz * geometric_anti_product.group0().zxw) - (self.group2().zxy * geometric_anti_product.group1().yzx) - (self.group3().zxy * geometric_anti_product.group0().yzx), 
            /* e423, e431, e412, e321 */
            (float4(self[e4], self[e412], self[e423], self[e321]) * geometric_anti_product.group0().xxyw) + (float4(self[e431], self[e4], self[e4], self[e423]) * float4(geometric_anti_product.group0().zyz, geometric_anti_product[e23])) + (self.group4().xyzy * float4(geometric_anti_product.group0().www, geometric_anti_product[e31])) + float4(float3(0.0), (geometric_anti_product[e12] * self[e412]) - (geometric_anti_product[e42] * self[e2]) - (geometric_anti_product[e43] * self[e3]) - (geometric_anti_product[scalar] * self[e4])) - (geometric_anti_product.group0().yzxx * float4(self.group4().zxy, self[e1]))
        );
    }
}
extension MultiVector: GeometricAntiQuotient<MultiVector> {
    func geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
MultiVector anti_reverse = MultiVector::from_groups(
            /* scalar, e1234 */
            other.group0(), 
            /* e1, e2, e3, e4 */
            other.group1() * float4(-1.0), 
            /* e41, e42, e43 */
            other.group2() * float3(-1.0), 
            /* e23, e31, e12 */
            other.group3() * float3(-1.0), 
            /* e423, e431, e412, e321 */
            other.group4()
        );
AntiScalar other_2 = AntiScalar::from_groups(
            /* e1234 */
            powi(other[e1234], 2) + powi(other[e4], 2) + powi(other[e41], 2) + powi(other[e42], 2) + powi(other[e43], 2) + powi(other[e423], 2) + powi(other[e431], 2) + powi(other[e412], 2)
        );
MultiVector geometric_anti_product = MultiVector::from_groups(
            /* scalar, e1234 */
            float2(other_2[e1234]) * anti_reverse.group0(), 
            /* e1, e2, e3, e4 */
            float4(other_2[e1234]) * anti_reverse.group1(), 
            /* e41, e42, e43 */
            float3(other_2[e1234]) * anti_reverse.group2(), 
            /* e23, e31, e12 */
            float3(other_2[e1234]) * anti_reverse.group3(), 
            /* e423, e431, e412, e321 */
            float4(other_2[e1234]) * anti_reverse.group4()
        );
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2((geometric_anti_product[e1234] * self[scalar]) + (geometric_anti_product[e321] * self[e4]) - (geometric_anti_product[e2] * self[e431]) - (geometric_anti_product[e3] * self[e412]) - (geometric_anti_product[e4] * self[e321]) - (geometric_anti_product[e23] * self[e41]) - (geometric_anti_product[e31] * self[e42]) - (geometric_anti_product[e12] * self[e43]), 0.0) + (float2(geometric_anti_product[e423]) * float2(self[e1], self[e423])) + (float2(geometric_anti_product[e431]) * float2(self[e2], self[e431])) + (float2(geometric_anti_product[e412]) * float2(self[e3], self[e412])) + (float2(self[e1234]) * geometric_anti_product.group0()) - (float2(geometric_anti_product[e41]) * float2(self[e23], self[e41])) - (float2(geometric_anti_product[e42]) * float2(self[e31], self[e42])) - (float2(geometric_anti_product[e43]) * float2(self[e12], self[e43])) - (float2(self[e423], self[e4]) * geometric_anti_product.group1().xw), 
            /* e1, e2, e3, e4 */
            float4((geometric_anti_product[e1] * self[e1234]) + (geometric_anti_product[e3] * self[e42]) + (geometric_anti_product[e4] * self[e23]) + (geometric_anti_product[e41] * self[e321]) + (geometric_anti_product[e43] * self[e2]) + (geometric_anti_product[e31] * self[e412]) + (geometric_anti_product[e412] * self[e31]) + (geometric_anti_product[e321] * self[e41]), (geometric_anti_product[e1] * self[e43]) + (geometric_anti_product[e2] * self[e1234]) + (geometric_anti_product[e4] * self[e31]) + (geometric_anti_product[e41] * self[e3]) + (geometric_anti_product[e42] * self[e321]) + (geometric_anti_product[e12] * self[e423]) + (geometric_anti_product[e423] * self[e12]) + (geometric_anti_product[e321] * self[e42]), (geometric_anti_product[e2] * self[e41]) + (geometric_anti_product[e3] * self[e1234]) + (geometric_anti_product[e4] * self[e12]) + (geometric_anti_product[e42] * self[e1]) + (geometric_anti_product[e43] * self[e321]) + (geometric_anti_product[e23] * self[e431]) + (geometric_anti_product[e431] * self[e23]) + (geometric_anti_product[e321] * self[e43]), 0.0) + (float4(geometric_anti_product[e1234]) * self.group1()) + (float4(self.group0().xx, self[scalar], self[e1234]) * float4(geometric_anti_product.group4().xyz, geometric_anti_product[e4])) - (float4(self[e4], self[e412], self[e423], self[e412]) * float4(geometric_anti_product.group3().xxy, geometric_anti_product[e43])) - (float4(self[e431], self[e4], self[e4], geometric_anti_product[e423]) * float4(geometric_anti_product.group3().zyz, self[e41])) - (geometric_anti_product.group4().yzxz * float4(self.group3().zxy, self[e43])) - (self.group4().xyzx * float4(geometric_anti_product.group0().xx, geometric_anti_product[scalar], geometric_anti_product[e41])) - float4((geometric_anti_product.group2().yzx * self.group1().zxy), geometric_anti_product[e42] * self[e431]) - float4((self.group2().zxy * geometric_anti_product.group1().yzx), geometric_anti_product[e431] * self[e42]), 
            /* e41, e42, e43 */
            (float3(geometric_anti_product[e1234]) * self.group2()) + (float3(self[e1234]) * geometric_anti_product.group2()) + (geometric_anti_product.group2().zxy * self.group2().yzx) + (geometric_anti_product.group4().yzx * self.group4().zxy) - (float3(geometric_anti_product[e4]) * self.group4().xyz) - (float3(self[e4], self[e412], self[e423]) * geometric_anti_product.group4().xxy) - (float3(self[e431], self[e4], self[e4]) * geometric_anti_product.group4().zyz) - (geometric_anti_product.group2().yzx * self.group2().zxy), 
            /* e23, e31, e12 */
            (float3(geometric_anti_product[scalar]) * self.group2()) + (float3(geometric_anti_product[e1234]) * self.group3()) + (float3(geometric_anti_product[e321]) * self.group4().xyz) + (float3(self[scalar]) * geometric_anti_product.group2()) + (float3(self[e1234]) * geometric_anti_product.group3()) + (float3(self[e4], self[e412], self[e423]) * geometric_anti_product.group1().xxy) + (float3(self[e431], self[e4], self[e4]) * geometric_anti_product.group1().zyz) + (geometric_anti_product.group2().zxy * self.group3().yzx) + (geometric_anti_product.group3().zxy * self.group2().yzx) + (geometric_anti_product.group4().yzx * self.group1().zxy) - (float3(geometric_anti_product[e4]) * self.group1().xyz) - (float3(self[e2], self[e321], self[e321]) * geometric_anti_product.group4().zyz) - (float3(self[e321], self[e3], self[e1]) * geometric_anti_product.group4().xxy) - (geometric_anti_product.group2().yzx * self.group3().zxy) - (geometric_anti_product.group3().yzx * self.group2().zxy) - (geometric_anti_product.group1().yzx * self.group4().zxy), 
            /* e423, e431, e412, e321 */
            (float4(geometric_anti_product[e1234]) * self.group4()) + (float4(self[e4], self[e412], self[e423], geometric_anti_product[e321]) * float4(geometric_anti_product.group2().xxy, self[e1234])) + (float4(self[e431], self[e4], self[e4], self[e423]) * float4(geometric_anti_product.group2().zyz, geometric_anti_product[e23])) + (float4(self.group0().yy, self[e1234], self[scalar]) * float4(geometric_anti_product.group4().xyz, geometric_anti_product[e4])) + (float4(geometric_anti_product.group1().ww, geometric_anti_product[e431], self[e431]) * float4(self.group2().xyx, geometric_anti_product[e31])) + (float4(geometric_anti_product.group4().zx, geometric_anti_product[e4], self[e412]) * float4(self.group2().yzz, geometric_anti_product[e12])) + float4(float3(0.0), -(geometric_anti_product[e1] * self[e41]) - (geometric_anti_product[e2] * self[e42]) - (geometric_anti_product[e3] * self[e43]) - (geometric_anti_product[e42] * self[e2]) - (geometric_anti_product[e43] * self[e3]) - (geometric_anti_product[e423] * self[e23]) - (geometric_anti_product[e431] * self[e31]) - (geometric_anti_product[e412] * self[e12])) - float4((geometric_anti_product.group2().yzx * self.group4().zxy), geometric_anti_product[scalar] * self[e4]) - float4((self.group2().zxy * geometric_anti_product.group4().yzx), geometric_anti_product[e41] * self[e1])
        );
    }
}
extension MultiVector: GeometricAntiQuotient<Origin> {
    func geometric_anti_quotient(self, other: Origin) -> MultiVector {
Origin geometric_anti_product = Origin::from_groups(
            /* e4 */
            1.0/other[e4] * -1.0
        );
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(geometric_anti_product[e4]) * float2(self[e321], self[e4]) * float2(-1.0), 
            /* e1, e2, e3, e4 */
            float4(geometric_anti_product[e4]) * float4(self.group3(), self[e1234]), 
            /* e41, e42, e43 */
            float3(geometric_anti_product[e4]) * self.group4().xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(geometric_anti_product[e4]) * self.group1().xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            float4(geometric_anti_product[e4]) * float4(self.group2(), self[scalar])
        );
    }
}
extension MultiVector: GeometricAntiQuotient<Plane> {
    func geometric_anti_quotient(self, other: Plane) -> MultiVector {
Plane geometric_anti_product = Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4(powi(other[e423], 2) + powi(other[e431], 2) + powi(other[e412], 2)) * other.group0()
        );
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(self[e4] * geometric_anti_product[e321], 0.0) + (float2(geometric_anti_product[e423]) * float2(self[e1], self[e423])) + (float2(geometric_anti_product[e431]) * float2(self[e2], self[e431])) + (float2(geometric_anti_product[e412]) * float2(self[e3], self[e412])), 
            /* e1, e2, e3, e4 */
            float4((self[scalar] * geometric_anti_product[e423]) + (self[e41] * geometric_anti_product[e321]) + (self[e31] * geometric_anti_product[e412]), (self[scalar] * geometric_anti_product[e431]) + (self[e42] * geometric_anti_product[e321]) + (self[e12] * geometric_anti_product[e423]), (self[scalar] * geometric_anti_product[e412]) + (self[e43] * geometric_anti_product[e321]) + (self[e23] * geometric_anti_product[e431]), -(self[e42] * geometric_anti_product[e431]) - (self[e43] * geometric_anti_product[e412])) - (geometric_anti_product.group0().yzxx * float4(self.group3().zxy, self[e41])), 
            /* e41, e42, e43 */
            (self.group4().zxy * geometric_anti_product.group0().yzx) - (float3(self[e4]) * geometric_anti_product.group0().xyz) - (self.group4().yzx * geometric_anti_product.group0().zxy), 
            /* e23, e31, e12 */
            (float3(geometric_anti_product[e321]) * self.group4().xyz) + (self.group1().zxy * geometric_anti_product.group0().yzx) - (float3(self[e321]) * geometric_anti_product.group0().xyz) - (self.group1().yzx * geometric_anti_product.group0().zxy), 
            /* e423, e431, e412, e321 */
            float4(self[e42] * geometric_anti_product[e412], self[e43] * geometric_anti_product[e423], self[e41] * geometric_anti_product[e431], -(self[e31] * geometric_anti_product[e431]) - (self[e12] * geometric_anti_product[e412])) + (float4(self[e1234]) * geometric_anti_product.group0()) - (geometric_anti_product.group0().yzxx * float4(self.group2().zxy, self[e23]))
        );
    }
}
extension MultiVector: GeometricAntiQuotient<Point> {
    func geometric_anti_quotient(self, other: Point) -> MultiVector {
Point geometric_anti_product = Point::from_groups(
            /* e1, e2, e3, e4 */
            float4(powi(other[e4], -2)) * float4(other[e1] * -1.0, other[e2] * -1.0, other[e3] * -1.0, other[e4] * -1.0)
        );
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(-(self[e423] * geometric_anti_product[e1]) - (self[e431] * geometric_anti_product[e2]) - (self[e412] * geometric_anti_product[e3]) - (self[e321] * geometric_anti_product[e4]), self[e4] * geometric_anti_product[e4]) * float2(1.0, -1.0), 
            /* e1, e2, e3, e4 */
            float4(((float3(self[e1234]) * geometric_anti_product.group0().xyz) + (float3(geometric_anti_product[e4]) * self.group3()) + (self.group2().yzx * geometric_anti_product.group0().zxy) - (self.group2().zxy * geometric_anti_product.group0().yzx)), self[e1234] * geometric_anti_product[e4]), 
            /* e41, e42, e43 */
            float3(geometric_anti_product[e4]) * self.group4().xyz * float3(-1.0), 
            /* e23, e31, e12 */
            (float3(self[e4]) * geometric_anti_product.group0().xyz) + (self.group4().yzx * geometric_anti_product.group0().zxy) - (float3(geometric_anti_product[e4]) * self.group1().xyz) - (self.group4().zxy * geometric_anti_product.group0().yzx), 
            /* e423, e431, e412, e321 */
            float4(geometric_anti_product[e4], geometric_anti_product[e4], geometric_anti_product[e4], 1.0) * float4(self.group2(), (self[scalar] * geometric_anti_product[e4]) - (self[e41] * geometric_anti_product[e1]) - (self[e42] * geometric_anti_product[e2]) - (self[e43] * geometric_anti_product[e3]))
        );
    }
}
extension Origin {
    func div(_rhs: geometric_anti_quotient) -> geometric_anti_quotient_partial<Origin> {
        geometric_anti_quotient_partial(this)
    }
}
extension Origin: GeometricAntiQuotient<AntiScalar> {
    func geometric_anti_quotient(self, other: AntiScalar) -> Origin {
        return Origin::from_groups(
            /* e4 */
            self[e4] / (other[e1234])
        );
    }
}
extension Origin: GeometricAntiQuotient<DualNum> {
    func geometric_anti_quotient(self, other: DualNum) -> Flector {
DualNum geometric_anti_product = DualNum::from_groups(
            /* scalar, e1234 */
            float2(powi(other[e1234], -2)) * other.group0()
        );
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), geometric_anti_product[e1234] * self[e4]), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), geometric_anti_product[scalar] * self[e4] * -1.0)
        );
    }
}
extension Origin: GeometricAntiQuotient<Flector> {
    func geometric_anti_quotient(self, other: Flector) -> Motor {
Flector anti_reverse = Flector::from_groups(
            /* e1, e2, e3, e4 */
            other.group0() * float4(-1.0), 
            /* e423, e431, e412, e321 */
            other.group1()
        );
AntiScalar other_2 = AntiScalar::from_groups(
            /* e1234 */
            powi(other[e4], 2) + powi(other[e423], 2) + powi(other[e431], 2) + powi(other[e412], 2)
        );
Flector geometric_anti_product = Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(other_2[e1234]) * anti_reverse.group0(), 
            /* e423, e431, e412, e321 */
            float4(other_2[e1234]) * anti_reverse.group1()
        );
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(self[e4]) * float4(geometric_anti_product.group1().xyz, geometric_anti_product[e4]) * float4(-1.0), 
            /* e23, e31, e12, scalar */
            float4(self[e4]) * float4(geometric_anti_product.group0().xyz, geometric_anti_product[e321])
        );
    }
}
extension Origin: GeometricAntiQuotient<Line> {
    func geometric_anti_quotient(self, other: Line) -> Flector {
Line anti_reverse = Line::from_groups(
            /* e41, e42, e43 */
            other.group0() * float3(-1.0), 
            /* e23, e31, e12 */
            other.group1() * float3(-1.0)
        );
AntiScalar other_2 = AntiScalar::from_groups(
            /* e1234 */
            powi(other[e41], 2) + powi(other[e42], 2) + powi(other[e43], 2)
        );
Line geometric_anti_product = Line::from_groups(
            /* e41, e42, e43 */
            float3(other_2[e1234]) * anti_reverse.group0(), 
            /* e23, e31, e12 */
            float3(other_2[e1234]) * anti_reverse.group1()
        );
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(self[e4], self[e4], self[e4], 0.0) * float4(geometric_anti_product.group1(), 0.0) * float4(-1.0, -1.0, -1.0, 0.0), 
            /* e423, e431, e412, e321 */
            float4(self[e4], self[e4], self[e4], 0.0) * float4(geometric_anti_product.group0(), 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Origin: GeometricAntiQuotient<Motor> {
    func geometric_anti_quotient(self, other: Motor) -> Flector {
Motor anti_reverse = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            other.group0() * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            other.group1() * float4(-1.0, -1.0, -1.0, 1.0)
        );
AntiScalar other_2 = AntiScalar::from_groups(
            /* e1234 */
            powi(other[e41], 2) + powi(other[e42], 2) + powi(other[e43], 2) + powi(other[e1234], 2)
        );
Motor geometric_anti_product = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(other_2[e1234]) * anti_reverse.group0(), 
            /* e23, e31, e12, scalar */
            float4(other_2[e1234]) * anti_reverse.group1()
        );
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(self[e4]) * float4(geometric_anti_product.group1().xyz, geometric_anti_product[e1234]) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e423, e431, e412, e321 */
            float4(self[e4]) * float4(geometric_anti_product.group0().xyz, geometric_anti_product[scalar]) * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
extension Origin: GeometricAntiQuotient<MultiVector> {
    func geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
MultiVector anti_reverse = MultiVector::from_groups(
            /* scalar, e1234 */
            other.group0(), 
            /* e1, e2, e3, e4 */
            other.group1() * float4(-1.0), 
            /* e41, e42, e43 */
            other.group2() * float3(-1.0), 
            /* e23, e31, e12 */
            other.group3() * float3(-1.0), 
            /* e423, e431, e412, e321 */
            other.group4()
        );
AntiScalar other_2 = AntiScalar::from_groups(
            /* e1234 */
            powi(other[e1234], 2) + powi(other[e4], 2) + powi(other[e41], 2) + powi(other[e42], 2) + powi(other[e43], 2) + powi(other[e423], 2) + powi(other[e431], 2) + powi(other[e412], 2)
        );
MultiVector geometric_anti_product = MultiVector::from_groups(
            /* scalar, e1234 */
            float2(other_2[e1234]) * anti_reverse.group0(), 
            /* e1, e2, e3, e4 */
            float4(other_2[e1234]) * anti_reverse.group1(), 
            /* e41, e42, e43 */
            float3(other_2[e1234]) * anti_reverse.group2(), 
            /* e23, e31, e12 */
            float3(other_2[e1234]) * anti_reverse.group3(), 
            /* e423, e431, e412, e321 */
            float4(other_2[e1234]) * anti_reverse.group4()
        );
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(self[e4]) * float2(geometric_anti_product[e321], geometric_anti_product[e4]) * float2(1.0, -1.0), 
            /* e1, e2, e3, e4 */
            float4(self[e4]) * float4(geometric_anti_product.group3(), geometric_anti_product[e1234]) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e41, e42, e43 */
            float3(self[e4]) * geometric_anti_product.group4().xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(self[e4]) * geometric_anti_product.group1().xyz, 
            /* e423, e431, e412, e321 */
            float4(self[e4]) * float4(geometric_anti_product.group2(), geometric_anti_product[scalar]) * float4(1.0, 1.0, 1.0, -1.0)
        );
    }
}
extension Origin: GeometricAntiQuotient<Origin> {
    func geometric_anti_quotient(self, other: Origin) -> AntiScalar {
        return AntiScalar::from_groups(
            /* e1234 */
            self[e4] / (other[e4])
        );
    }
}
extension Origin: GeometricAntiQuotient<Plane> {
    func geometric_anti_quotient(self, other: Plane) -> Motor {
Plane geometric_anti_product = Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4(powi(other[e423], 2) + powi(other[e431], 2) + powi(other[e412], 2)) * other.group0()
        );
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(self[e4], self[e4], self[e4], 0.0) * float4(geometric_anti_product.group0().xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0), 
            /* e23, e31, e12, scalar */
            float4(float3(0.0), self[e4] * geometric_anti_product[e321])
        );
    }
}
extension Origin: GeometricAntiQuotient<Point> {
    func geometric_anti_quotient(self, other: Point) -> Motor {
Point geometric_anti_product = Point::from_groups(
            /* e1, e2, e3, e4 */
            float4(powi(other[e4], -2)) * float4(other[e1] * -1.0, other[e2] * -1.0, other[e3] * -1.0, other[e4] * -1.0)
        );
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(float3(0.0), self[e4] * geometric_anti_product[e4] * -1.0), 
            /* e23, e31, e12, scalar */
            float4(self[e4], self[e4], self[e4], 0.0) * float4(geometric_anti_product.group0().xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Plane {
    func div(_rhs: geometric_anti_quotient) -> geometric_anti_quotient_partial<Plane> {
        geometric_anti_quotient_partial(this)
    }
}
extension Plane: GeometricAntiQuotient<AntiScalar> {
    func geometric_anti_quotient(self, other: AntiScalar) -> Plane {
        return Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4(1.0/other[e1234]) * self.group0()
        );
    }
}
extension Plane: GeometricAntiQuotient<DualNum> {
    func geometric_anti_quotient(self, other: DualNum) -> Flector {
DualNum geometric_anti_product = DualNum::from_groups(
            /* scalar, e1234 */
            float2(powi(other[e1234], -2)) * other.group0()
        );
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(geometric_anti_product.group0().xx, geometric_anti_product[scalar], 0.0) * float4(float3(1.0), 0.0) * float4(self.group0().xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0), 
            /* e423, e431, e412, e321 */
            float4(geometric_anti_product[e1234]) * self.group0()
        );
    }
}
extension Plane: GeometricAntiQuotient<Flector> {
    func geometric_anti_quotient(self, other: Flector) -> Motor {
Flector anti_reverse = Flector::from_groups(
            /* e1, e2, e3, e4 */
            other.group0() * float4(-1.0), 
            /* e423, e431, e412, e321 */
            other.group1()
        );
AntiScalar other_2 = AntiScalar::from_groups(
            /* e1234 */
            powi(other[e4], 2) + powi(other[e423], 2) + powi(other[e431], 2) + powi(other[e412], 2)
        );
Flector geometric_anti_product = Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(other_2[e1234]) * anti_reverse.group0(), 
            /* e423, e431, e412, e321 */
            float4(other_2[e1234]) * anti_reverse.group1()
        );
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(-(geometric_anti_product[e4] * self[e423]) - (geometric_anti_product[e412] * self[e431]), -(geometric_anti_product[e4] * self[e431]) - (geometric_anti_product[e423] * self[e412]), -(geometric_anti_product[e4] * self[e412]) - (geometric_anti_product[e431] * self[e423]), (geometric_anti_product[e431] * self[e431]) + (geometric_anti_product[e412] * self[e412])) + (geometric_anti_product.group1().yzxx * self.group0().zxyx), 
            /* e23, e31, e12, scalar */
            float4((geometric_anti_product[e3] * self[e431]) + (geometric_anti_product[e321] * self[e423]), (geometric_anti_product[e1] * self[e412]) + (geometric_anti_product[e321] * self[e431]), (geometric_anti_product[e2] * self[e423]) + (geometric_anti_product[e321] * self[e412]), -(geometric_anti_product[e3] * self[e412]) - (geometric_anti_product[e4] * self[e321])) - (geometric_anti_product.group0().yzxx * self.group0().zxyx) - (self.group0().wwwy * float4(geometric_anti_product.group1().xyz, geometric_anti_product[e2]))
        );
    }
}
extension Plane: GeometricAntiQuotient<Line> {
    func geometric_anti_quotient(self, other: Line) -> Flector {
Line anti_reverse = Line::from_groups(
            /* e41, e42, e43 */
            other.group0() * float3(-1.0), 
            /* e23, e31, e12 */
            other.group1() * float3(-1.0)
        );
AntiScalar other_2 = AntiScalar::from_groups(
            /* e1234 */
            powi(other[e41], 2) + powi(other[e42], 2) + powi(other[e43], 2)
        );
Line geometric_anti_product = Line::from_groups(
            /* e41, e42, e43 */
            float3(other_2[e1234]) * anti_reverse.group0(), 
            /* e23, e31, e12 */
            float3(other_2[e1234]) * anti_reverse.group1()
        );
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4((geometric_anti_product[e41] * self[e321]) + (geometric_anti_product[e31] * self[e412]), (geometric_anti_product[e42] * self[e321]) + (geometric_anti_product[e12] * self[e423]), (geometric_anti_product[e43] * self[e321]) + (geometric_anti_product[e23] * self[e431]), -(geometric_anti_product[e42] * self[e431]) - (geometric_anti_product[e43] * self[e412])) - (self.group0().yzxx * float4(geometric_anti_product.group1().zxy, geometric_anti_product[e41])), 
            /* e423, e431, e412, e321 */
            float4(geometric_anti_product[e42] * self[e412] * -1.0, geometric_anti_product[e43] * self[e423] * -1.0, geometric_anti_product[e41] * self[e431] * -1.0, (geometric_anti_product[e31] * self[e431]) + (geometric_anti_product[e12] * self[e412])) + (self.group0().yzxx * float4(geometric_anti_product.group0().zxy, geometric_anti_product[e23]))
        );
    }
}
extension Plane: GeometricAntiQuotient<Motor> {
    func geometric_anti_quotient(self, other: Motor) -> Flector {
Motor anti_reverse = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            other.group0() * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            other.group1() * float4(-1.0, -1.0, -1.0, 1.0)
        );
AntiScalar other_2 = AntiScalar::from_groups(
            /* e1234 */
            powi(other[e41], 2) + powi(other[e42], 2) + powi(other[e43], 2) + powi(other[e1234], 2)
        );
Motor geometric_anti_product = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(other_2[e1234]) * anti_reverse.group0(), 
            /* e23, e31, e12, scalar */
            float4(other_2[e1234]) * anti_reverse.group1()
        );
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4((geometric_anti_product[e41] * self[e321]) + (geometric_anti_product[e31] * self[e412]), (geometric_anti_product[e42] * self[e321]) + (geometric_anti_product[e12] * self[e423]), (geometric_anti_product[e43] * self[e321]) + (geometric_anti_product[e23] * self[e431]), geometric_anti_product[e43] * self[e412] * -1.0) - (self.group0().xyzy * float4(geometric_anti_product.group1().www, geometric_anti_product[e42])) - (self.group0().yzxx * float4(geometric_anti_product.group1().zxy, geometric_anti_product[e41])), 
            /* e423, e431, e412, e321 */
            float4(geometric_anti_product[e42] * self[e412] * -1.0, geometric_anti_product[e43] * self[e423] * -1.0, geometric_anti_product[e41] * self[e431] * -1.0, (geometric_anti_product[e31] * self[e431]) + (geometric_anti_product[e12] * self[e412])) + (geometric_anti_product.group0().zxyw * self.group0().yzxw) + (self.group0().xyzx * float4(geometric_anti_product.group0().www, geometric_anti_product[e23]))
        );
    }
}
extension Plane: GeometricAntiQuotient<MultiVector> {
    func geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
MultiVector anti_reverse = MultiVector::from_groups(
            /* scalar, e1234 */
            other.group0(), 
            /* e1, e2, e3, e4 */
            other.group1() * float4(-1.0), 
            /* e41, e42, e43 */
            other.group2() * float3(-1.0), 
            /* e23, e31, e12 */
            other.group3() * float3(-1.0), 
            /* e423, e431, e412, e321 */
            other.group4()
        );
AntiScalar other_2 = AntiScalar::from_groups(
            /* e1234 */
            powi(other[e1234], 2) + powi(other[e4], 2) + powi(other[e41], 2) + powi(other[e42], 2) + powi(other[e43], 2) + powi(other[e423], 2) + powi(other[e431], 2) + powi(other[e412], 2)
        );
MultiVector geometric_anti_product = MultiVector::from_groups(
            /* scalar, e1234 */
            float2(other_2[e1234]) * anti_reverse.group0(), 
            /* e1, e2, e3, e4 */
            float4(other_2[e1234]) * anti_reverse.group1(), 
            /* e41, e42, e43 */
            float3(other_2[e1234]) * anti_reverse.group2(), 
            /* e23, e31, e12 */
            float3(other_2[e1234]) * anti_reverse.group3(), 
            /* e423, e431, e412, e321 */
            float4(other_2[e1234]) * anti_reverse.group4()
        );
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(-(geometric_anti_product[e1] * self[e423]) - (geometric_anti_product[e2] * self[e431]) - (geometric_anti_product[e3] * self[e412]) - (geometric_anti_product[e4] * self[e321]), (geometric_anti_product[e423] * self[e423]) + (geometric_anti_product[e431] * self[e431]) + (geometric_anti_product[e412] * self[e412])), 
            /* e1, e2, e3, e4 */
            float4((geometric_anti_product[e41] * self[e321]) + (geometric_anti_product[e31] * self[e412]), (geometric_anti_product[e42] * self[e321]) + (geometric_anti_product[e12] * self[e423]), (geometric_anti_product[e43] * self[e321]) + (geometric_anti_product[e23] * self[e431]), geometric_anti_product[e43] * self[e412] * -1.0) - (self.group0().xyzx * float4(geometric_anti_product.group0().xx, geometric_anti_product[scalar], geometric_anti_product[e41])) - (self.group0().yzxy * float4(geometric_anti_product.group3().zxy, geometric_anti_product[e42])), 
            /* e41, e42, e43 */
            (geometric_anti_product.group4().yzx * self.group0().zxy) - (float3(geometric_anti_product[e4]) * self.group0().xyz) - (geometric_anti_product.group4().zxy * self.group0().yzx), 
            /* e23, e31, e12 */
            (float3(geometric_anti_product[e321]) * self.group0().xyz) + (geometric_anti_product.group1().zxy * self.group0().yzx) - (float3(self[e321]) * geometric_anti_product.group4().xyz) - (geometric_anti_product.group1().yzx * self.group0().zxy), 
            /* e423, e431, e412, e321 */
            float4(geometric_anti_product[e42] * self[e412] * -1.0, geometric_anti_product[e43] * self[e423] * -1.0, geometric_anti_product[e41] * self[e431] * -1.0, (geometric_anti_product[e31] * self[e431]) + (geometric_anti_product[e12] * self[e412])) + (float4(geometric_anti_product[e1234]) * self.group0()) + (self.group0().yzxx * float4(geometric_anti_product.group2().zxy, geometric_anti_product[e23]))
        );
    }
}
extension Plane: GeometricAntiQuotient<Origin> {
    func geometric_anti_quotient(self, other: Origin) -> Motor {
Origin geometric_anti_product = Origin::from_groups(
            /* e4 */
            1.0/other[e4] * -1.0
        );
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(geometric_anti_product[e4], geometric_anti_product[e4], geometric_anti_product[e4], 0.0) * float4(self.group0().xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0), 
            /* e23, e31, e12, scalar */
            float4(float3(0.0), geometric_anti_product[e4] * self[e321] * -1.0)
        );
    }
}
extension Plane: GeometricAntiQuotient<Plane> {
    func geometric_anti_quotient(self, other: Plane) -> Motor {
Plane geometric_anti_product = Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4(powi(other[e423], 2) + powi(other[e431], 2) + powi(other[e412], 2)) * other.group0()
        );
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(geometric_anti_product[e412] * self[e431] * -1.0, geometric_anti_product[e423] * self[e412] * -1.0, geometric_anti_product[e431] * self[e423] * -1.0, (geometric_anti_product[e431] * self[e431]) + (geometric_anti_product[e412] * self[e412])) + (geometric_anti_product.group0().yzxx * self.group0().zxyx), 
            /* e23, e31, e12, scalar */
            float4(((float3(geometric_anti_product[e321]) * self.group0().xyz) - (float3(self[e321]) * geometric_anti_product.group0().xyz)), 0.0)
        );
    }
}
extension Plane: GeometricAntiQuotient<Point> {
    func geometric_anti_quotient(self, other: Point) -> Motor {
Point geometric_anti_product = Point::from_groups(
            /* e1, e2, e3, e4 */
            float4(powi(other[e4], -2)) * float4(other[e1] * -1.0, other[e2] * -1.0, other[e3] * -1.0, other[e4] * -1.0)
        );
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(float3(1.0), 0.0) * float4(geometric_anti_product.group0().www, 0.0) * float4(self.group0().xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0), 
            /* e23, e31, e12, scalar */
            float4(self[e431] * geometric_anti_product[e3], self[e412] * geometric_anti_product[e1], self[e423] * geometric_anti_product[e2], -(self[e431] * geometric_anti_product[e2]) - (self[e412] * geometric_anti_product[e3]) - (self[e321] * geometric_anti_product[e4])) - (self.group0().zxyx * geometric_anti_product.group0().yzxx)
        );
    }
}
extension Point {
    func div(_rhs: geometric_anti_quotient) -> geometric_anti_quotient_partial<Point> {
        geometric_anti_quotient_partial(this)
    }
}
extension Point: GeometricAntiQuotient<AntiScalar> {
    func geometric_anti_quotient(self, other: AntiScalar) -> Point {
        return Point::from_groups(
            /* e1, e2, e3, e4 */
            float4(1.0/other[e1234]) * self.group0()
        );
    }
}
extension Point: GeometricAntiQuotient<DualNum> {
    func geometric_anti_quotient(self, other: DualNum) -> Flector {
DualNum geometric_anti_product = DualNum::from_groups(
            /* scalar, e1234 */
            float2(powi(other[e1234], -2)) * other.group0()
        );
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(geometric_anti_product[e1234]) * self.group0(), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), geometric_anti_product[scalar] * self[e4] * -1.0)
        );
    }
}
extension Point: GeometricAntiQuotient<Flector> {
    func geometric_anti_quotient(self, other: Flector) -> Motor {
Flector anti_reverse = Flector::from_groups(
            /* e1, e2, e3, e4 */
            other.group0() * float4(-1.0), 
            /* e423, e431, e412, e321 */
            other.group1()
        );
AntiScalar other_2 = AntiScalar::from_groups(
            /* e1234 */
            powi(other[e4], 2) + powi(other[e423], 2) + powi(other[e431], 2) + powi(other[e412], 2)
        );
Flector geometric_anti_product = Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(other_2[e1234]) * anti_reverse.group0(), 
            /* e423, e431, e412, e321 */
            float4(other_2[e1234]) * anti_reverse.group1()
        );
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(self[e4]) * float4(geometric_anti_product.group1().xyz, geometric_anti_product[e4]) * float4(-1.0), 
            /* e23, e31, e12, scalar */
            float4(-(geometric_anti_product[e4] * self[e1]) - (geometric_anti_product[e412] * self[e2]), -(geometric_anti_product[e4] * self[e2]) - (geometric_anti_product[e423] * self[e3]), -(geometric_anti_product[e4] * self[e3]) - (geometric_anti_product[e431] * self[e1]), (geometric_anti_product[e412] * self[e3]) + (geometric_anti_product[e321] * self[e4])) + (geometric_anti_product.group1().yzxy * self.group0().zxyy) + (self.group0().wwwx * float4(geometric_anti_product.group0().xyz, geometric_anti_product[e423]))
        );
    }
}
extension Point: GeometricAntiQuotient<Line> {
    func geometric_anti_quotient(self, other: Line) -> Flector {
Line anti_reverse = Line::from_groups(
            /* e41, e42, e43 */
            other.group0() * float3(-1.0), 
            /* e23, e31, e12 */
            other.group1() * float3(-1.0)
        );
AntiScalar other_2 = AntiScalar::from_groups(
            /* e1234 */
            powi(other[e41], 2) + powi(other[e42], 2) + powi(other[e43], 2)
        );
Line geometric_anti_product = Line::from_groups(
            /* e41, e42, e43 */
            float3(other_2[e1234]) * anti_reverse.group0(), 
            /* e23, e31, e12 */
            float3(other_2[e1234]) * anti_reverse.group1()
        );
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4((geometric_anti_product.group0().zxy * self.group0().yzx), 0.0) - float4((float3(self[e4]) * geometric_anti_product.group1()), 0.0) - float4((geometric_anti_product.group0().yzx * self.group0().zxy), 0.0), 
            /* e423, e431, e412, e321 */
            float4(self[e4], self[e4], self[e4], 1.0) * float4(geometric_anti_product.group0(), -(geometric_anti_product[e41] * self[e1]) - (geometric_anti_product[e42] * self[e2]) - (geometric_anti_product[e43] * self[e3]))
        );
    }
}
extension Point: GeometricAntiQuotient<Motor> {
    func geometric_anti_quotient(self, other: Motor) -> Flector {
Motor anti_reverse = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            other.group0() * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            other.group1() * float4(-1.0, -1.0, -1.0, 1.0)
        );
AntiScalar other_2 = AntiScalar::from_groups(
            /* e1234 */
            powi(other[e41], 2) + powi(other[e42], 2) + powi(other[e43], 2) + powi(other[e1234], 2)
        );
Motor geometric_anti_product = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(other_2[e1234]) * anti_reverse.group0(), 
            /* e23, e31, e12, scalar */
            float4(other_2[e1234]) * anti_reverse.group1()
        );
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(((float3(geometric_anti_product[e1234]) * self.group0().xyz) + (geometric_anti_product.group0().zxy * self.group0().yzx) - (float3(self[e4]) * geometric_anti_product.group1().xyz) - (geometric_anti_product.group0().yzx * self.group0().zxy)), geometric_anti_product[e1234] * self[e4]), 
            /* e423, e431, e412, e321 */
            float4(self[e4], self[e4], self[e4], 1.0) * float4(geometric_anti_product.group0().xyz, -(geometric_anti_product[e41] * self[e1]) - (geometric_anti_product[e42] * self[e2]) - (geometric_anti_product[e43] * self[e3]) - (geometric_anti_product[scalar] * self[e4]))
        );
    }
}
extension Point: GeometricAntiQuotient<MultiVector> {
    func geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
MultiVector anti_reverse = MultiVector::from_groups(
            /* scalar, e1234 */
            other.group0(), 
            /* e1, e2, e3, e4 */
            other.group1() * float4(-1.0), 
            /* e41, e42, e43 */
            other.group2() * float3(-1.0), 
            /* e23, e31, e12 */
            other.group3() * float3(-1.0), 
            /* e423, e431, e412, e321 */
            other.group4()
        );
AntiScalar other_2 = AntiScalar::from_groups(
            /* e1234 */
            powi(other[e1234], 2) + powi(other[e4], 2) + powi(other[e41], 2) + powi(other[e42], 2) + powi(other[e43], 2) + powi(other[e423], 2) + powi(other[e431], 2) + powi(other[e412], 2)
        );
MultiVector geometric_anti_product = MultiVector::from_groups(
            /* scalar, e1234 */
            float2(other_2[e1234]) * anti_reverse.group0(), 
            /* e1, e2, e3, e4 */
            float4(other_2[e1234]) * anti_reverse.group1(), 
            /* e41, e42, e43 */
            float3(other_2[e1234]) * anti_reverse.group2(), 
            /* e23, e31, e12 */
            float3(other_2[e1234]) * anti_reverse.group3(), 
            /* e423, e431, e412, e321 */
            float4(other_2[e1234]) * anti_reverse.group4()
        );
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2((geometric_anti_product[e423] * self[e1]) + (geometric_anti_product[e431] * self[e2]) + (geometric_anti_product[e412] * self[e3]) + (geometric_anti_product[e321] * self[e4]), geometric_anti_product[e4] * self[e4]) * float2(1.0, -1.0), 
            /* e1, e2, e3, e4 */
            float4(((float3(geometric_anti_product[e1234]) * self.group0().xyz) + (geometric_anti_product.group2().zxy * self.group0().yzx) - (float3(self[e4]) * geometric_anti_product.group3()) - (geometric_anti_product.group2().yzx * self.group0().zxy)), geometric_anti_product[e1234] * self[e4]), 
            /* e41, e42, e43 */
            float3(self[e4]) * geometric_anti_product.group4().xyz * float3(-1.0), 
            /* e23, e31, e12 */
            (float3(self[e4]) * geometric_anti_product.group1().xyz) + (geometric_anti_product.group4().yzx * self.group0().zxy) - (float3(geometric_anti_product[e4]) * self.group0().xyz) - (geometric_anti_product.group4().zxy * self.group0().yzx), 
            /* e423, e431, e412, e321 */
            float4(self[e4], self[e4], self[e4], 1.0) * float4(geometric_anti_product.group2(), -(geometric_anti_product[scalar] * self[e4]) - (geometric_anti_product[e41] * self[e1]) - (geometric_anti_product[e42] * self[e2]) - (geometric_anti_product[e43] * self[e3]))
        );
    }
}
extension Point: GeometricAntiQuotient<Origin> {
    func geometric_anti_quotient(self, other: Origin) -> Motor {
Origin geometric_anti_product = Origin::from_groups(
            /* e4 */
            1.0/other[e4] * -1.0
        );
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(float3(0.0), geometric_anti_product[e4] * self[e4] * -1.0), 
            /* e23, e31, e12, scalar */
            float4(geometric_anti_product[e4], geometric_anti_product[e4], geometric_anti_product[e4], 0.0) * float4(self.group0().xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0)
        );
    }
}
extension Point: GeometricAntiQuotient<Plane> {
    func geometric_anti_quotient(self, other: Plane) -> Motor {
Plane geometric_anti_product = Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4(powi(other[e423], 2) + powi(other[e431], 2) + powi(other[e412], 2)) * other.group0()
        );
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(float3(1.0), 0.0) * float4(self.group0().www, 0.0) * float4(geometric_anti_product.group0().xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0), 
            /* e23, e31, e12, scalar */
            float4(geometric_anti_product[e412] * self[e2] * -1.0, geometric_anti_product[e423] * self[e3] * -1.0, geometric_anti_product[e431] * self[e1] * -1.0, (geometric_anti_product[e431] * self[e2]) + (geometric_anti_product[e412] * self[e3]) + (geometric_anti_product[e321] * self[e4])) + (geometric_anti_product.group0().yzxx * self.group0().zxyx)
        );
    }
}
extension Point: GeometricAntiQuotient<Point> {
    func geometric_anti_quotient(self, other: Point) -> Motor {
Point geometric_anti_product = Point::from_groups(
            /* e1, e2, e3, e4 */
            float4(powi(other[e4], -2)) * float4(other[e1] * -1.0, other[e2] * -1.0, other[e3] * -1.0, other[e4] * -1.0)
        );
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(float3(0.0), geometric_anti_product[e4] * self[e4] * -1.0), 
            /* e23, e31, e12, scalar */
            float4(((float3(self[e4]) * geometric_anti_product.group0().xyz) - (float3(geometric_anti_product[e4]) * self.group0().xyz)), 0.0)
        );
    }
}
extension Scalar {
    func div(_rhs: geometric_anti_quotient) -> geometric_anti_quotient_partial<Scalar> {
        geometric_anti_quotient_partial(this)
    }
}
extension Scalar: GeometricAntiQuotient<AntiScalar> {
    func geometric_anti_quotient(self, other: AntiScalar) -> Scalar {
        return Scalar::from_groups(
            /* scalar */
            self[scalar] / (other[e1234])
        );
    }
}
extension Scalar: GeometricAntiQuotient<DualNum> {
    func geometric_anti_quotient(self, other: DualNum) -> Scalar {
        return Scalar::from_groups(
            /* scalar */
            self[scalar] / (other[e1234])
        );
    }
}
extension Scalar: GeometricAntiQuotient<Flector> {
    func geometric_anti_quotient(self, other: Flector) -> Flector {
Flector anti_reverse = Flector::from_groups(
            /* e1, e2, e3, e4 */
            other.group0() * float4(-1.0), 
            /* e423, e431, e412, e321 */
            other.group1()
        );
AntiScalar other_2 = AntiScalar::from_groups(
            /* e1234 */
            powi(other[e4], 2) + powi(other[e423], 2) + powi(other[e431], 2) + powi(other[e412], 2)
        );
Flector geometric_anti_product = Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(other_2[e1234]) * anti_reverse.group0(), 
            /* e423, e431, e412, e321 */
            float4(other_2[e1234]) * anti_reverse.group1()
        );
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(self[scalar], self[scalar], self[scalar], 0.0) * float4(geometric_anti_product.group1().xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), geometric_anti_product[e4] * self[scalar])
        );
    }
}
extension Scalar: GeometricAntiQuotient<Line> {
    func geometric_anti_quotient(self, other: Line) -> Line {
Line anti_reverse = Line::from_groups(
            /* e41, e42, e43 */
            other.group0() * float3(-1.0), 
            /* e23, e31, e12 */
            other.group1() * float3(-1.0)
        );
AntiScalar other_2 = AntiScalar::from_groups(
            /* e1234 */
            powi(other[e41], 2) + powi(other[e42], 2) + powi(other[e43], 2)
        );
        return Line::from_groups(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(self[scalar]) * float3(other_2[e1234] * anti_reverse[e41], other_2[e1234] * anti_reverse[e42], other_2[e1234] * anti_reverse[e43])
        );
    }
}
extension Scalar: GeometricAntiQuotient<Motor> {
    func geometric_anti_quotient(self, other: Motor) -> Motor {
Motor anti_reverse = Motor::from_groups(
            /* e41, e42, e43, e1234 */
            other.group0() * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e23, e31, e12, scalar */
            other.group1() * float4(-1.0, -1.0, -1.0, 1.0)
        );
AntiScalar other_2 = AntiScalar::from_groups(
            /* e1234 */
            powi(other[e41], 2) + powi(other[e42], 2) + powi(other[e43], 2) + powi(other[e1234], 2)
        );
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(0.0), 
            /* e23, e31, e12, scalar */
            float4(self[scalar]) * float4(other_2[e1234] * anti_reverse[e41], other_2[e1234] * anti_reverse[e42], other_2[e1234] * anti_reverse[e43], other_2[e1234] * anti_reverse[e1234])
        );
    }
}
extension Scalar: GeometricAntiQuotient<MultiVector> {
    func geometric_anti_quotient(self, other: MultiVector) -> MultiVector {
MultiVector anti_reverse = MultiVector::from_groups(
            /* scalar, e1234 */
            other.group0(), 
            /* e1, e2, e3, e4 */
            other.group1() * float4(-1.0), 
            /* e41, e42, e43 */
            other.group2() * float3(-1.0), 
            /* e23, e31, e12 */
            other.group3() * float3(-1.0), 
            /* e423, e431, e412, e321 */
            other.group4()
        );
AntiScalar other_2 = AntiScalar::from_groups(
            /* e1234 */
            powi(other[e1234], 2) + powi(other[e4], 2) + powi(other[e41], 2) + powi(other[e42], 2) + powi(other[e43], 2) + powi(other[e423], 2) + powi(other[e431], 2) + powi(other[e412], 2)
        );
MultiVector geometric_anti_product = MultiVector::from_groups(
            /* scalar, e1234 */
            float2(other_2[e1234]) * anti_reverse.group0(), 
            /* e1, e2, e3, e4 */
            float4(other_2[e1234]) * anti_reverse.group1(), 
            /* e41, e42, e43 */
            float3(other_2[e1234]) * anti_reverse.group2(), 
            /* e23, e31, e12 */
            float3(other_2[e1234]) * anti_reverse.group3(), 
            /* e423, e431, e412, e321 */
            float4(other_2[e1234]) * anti_reverse.group4()
        );
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(geometric_anti_product[e1234] * self[scalar], 1.0) * float2(1.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(self[scalar], self[scalar], self[scalar], 0.0) * float4(geometric_anti_product.group4().xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(self[scalar]) * geometric_anti_product.group2(), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), geometric_anti_product[e4] * self[scalar])
        );
    }
}
extension Scalar: GeometricAntiQuotient<Origin> {
    func geometric_anti_quotient(self, other: Origin) -> Horizon {
        return Horizon::from_groups(
            /* e321 */
            self[scalar] / (other[e4]) * -1.0
        );
    }
}
extension Scalar: GeometricAntiQuotient<Plane> {
    func geometric_anti_quotient(self, other: Plane) -> Point {
AntiScalar other_2 = AntiScalar::from_groups(
            /* e1234 */
            powi(other[e423], 2) + powi(other[e431], 2) + powi(other[e412], 2)
        );
        return Point::from_groups(
            /* e1, e2, e3, e4 */
            float4(self[scalar], self[scalar], self[scalar], 0.0) * float4(float4(other_2[e1234] * other[e423], other_2[e1234] * other[e431], other_2[e1234] * other[e412], other_2[e1234] * other[e321]).xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Scalar: GeometricAntiQuotient<Point> {
    func geometric_anti_quotient(self, other: Point) -> Horizon {
        return Horizon::from_groups(
            /* e321 */
            self[scalar] / (other[e4]) * -1.0
        );
    }
}
