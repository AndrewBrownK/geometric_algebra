using traits::AntiReverse;
using traits::GeometricAntiProduct;
extension AntiScalar {
    func div(_rhs: anti_sandwich) -> anti_sandwich_partial<AntiScalar> {
        anti_sandwich_partial(this)
    }
}
extension AntiScalar: AntiSandwich<AntiScalar> {
    func anti_sandwich(self, other: AntiScalar) -> AntiScalar {
        return AntiScalar::from_groups(
            /* e1234 */
            other[e1234] * self[e1234]
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension AntiScalar: AntiSandwich<DualNum> {
    func anti_sandwich(self, other: DualNum) -> DualNum {
        return DualNum::from_groups(
            /* scalar, e1234 */
            float2(self[e1234]) * other.group0()
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension AntiScalar: AntiSandwich<Flector> {
    func anti_sandwich(self, other: Flector) -> Flector {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(self[e1234]) * other.group0(), 
            /* e423, e431, e412, e321 */
            float4(self[e1234]) * other.group1()
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension AntiScalar: AntiSandwich<Horizon> {
    func anti_sandwich(self, other: Horizon) -> Horizon {
        return Horizon::from_groups(
            /* e321 */
            self[e1234] * other[e321]
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension AntiScalar: AntiSandwich<Line> {
    func anti_sandwich(self, other: Line) -> Line {
        return Line::from_groups(
            /* e41, e42, e43 */
            float3(self[e1234]) * other.group0(), 
            /* e23, e31, e12 */
            float3(self[e1234]) * other.group1()
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension AntiScalar: AntiSandwich<Motor> {
    func anti_sandwich(self, other: Motor) -> Motor {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(self[e1234]) * other.group0(), 
            /* e23, e31, e12, scalar */
            float4(self[e1234]) * other.group1()
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension AntiScalar: AntiSandwich<MultiVector> {
    func anti_sandwich(self, other: MultiVector) -> MultiVector {
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
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension AntiScalar: AntiSandwich<Origin> {
    func anti_sandwich(self, other: Origin) -> Origin {
        return Origin::from_groups(
            /* e4 */
            self[e1234] * other[e4]
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension AntiScalar: AntiSandwich<Plane> {
    func anti_sandwich(self, other: Plane) -> Plane {
        return Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4(self[e1234]) * other.group0()
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension AntiScalar: AntiSandwich<Point> {
    func anti_sandwich(self, other: Point) -> Point {
        return Point::from_groups(
            /* e1, e2, e3, e4 */
            float4(self[e1234]) * other.group0()
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension AntiScalar: AntiSandwich<Scalar> {
    func anti_sandwich(self, other: Scalar) -> Scalar {
        return Scalar::from_groups(
            /* scalar */
            self[e1234] * other[scalar]
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension DualNum {
    func div(_rhs: anti_sandwich) -> anti_sandwich_partial<DualNum> {
        anti_sandwich_partial(this)
    }
}
extension DualNum: AntiSandwich<AntiScalar> {
    func anti_sandwich(self, other: AntiScalar) -> DualNum {
        return DualNum::from_groups(
            /* scalar, e1234 */
            float2(other[e1234]) * self.group0()
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension DualNum: AntiSandwich<DualNum> {
    func anti_sandwich(self, other: DualNum) -> DualNum {
        return DualNum::from_groups(
            /* scalar, e1234 */
            float2((other[scalar] * self[e1234]) + (other[e1234] * self[scalar]), other[e1234] * self[e1234])
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension DualNum: AntiSandwich<Flector> {
    func anti_sandwich(self, other: Flector) -> Flector {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(((float3(self[scalar]) * other.group1().xyz) + (float3(self[e1234]) * other.group0().xyz)), self[e1234] * other[e4]), 
            /* e423, e431, e412, e321 */
            float4(other[e423], other[e431], other[e412], 1.0) * float4(self.group0().yy, self[e1234], (self[scalar] * other[e4]) + (self[e1234] * other[e321]))
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension DualNum: AntiSandwich<Horizon> {
    func anti_sandwich(self, other: Horizon) -> Horizon {
        return Horizon::from_groups(
            /* e321 */
            self[e1234] * other[e321]
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension DualNum: AntiSandwich<Line> {
    func anti_sandwich(self, other: Line) -> Line {
        return Line::from_groups(
            /* e41, e42, e43 */
            float3(self[e1234]) * other.group0(), 
            /* e23, e31, e12 */
            (float3(self[scalar]) * other.group0()) + (float3(self[e1234]) * other.group1())
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension DualNum: AntiSandwich<Motor> {
    func anti_sandwich(self, other: Motor) -> Motor {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(self[e1234]) * other.group0(), 
            /* e23, e31, e12, scalar */
            (float4(self[scalar]) * other.group0()) + (float4(self[e1234]) * other.group1())
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension DualNum: AntiSandwich<MultiVector> {
    func anti_sandwich(self, other: MultiVector) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2((self[scalar] * other[e1234]) + (self[e1234] * other[scalar]), self[e1234] * other[e1234]), 
            /* e1, e2, e3, e4 */
            float4(((float3(self[scalar]) * other.group4().xyz) + (float3(self[e1234]) * other.group1().xyz)), self[e1234] * other[e4]), 
            /* e41, e42, e43 */
            float3(self[e1234]) * other.group2(), 
            /* e23, e31, e12 */
            (float3(self[scalar]) * other.group2()) + (float3(self[e1234]) * other.group3()), 
            /* e423, e431, e412, e321 */
            float4(other[e423], other[e431], other[e412], 1.0) * float4(self.group0().yy, self[e1234], (self[scalar] * other[e4]) + (self[e1234] * other[e321]))
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension DualNum: AntiSandwich<Origin> {
    func anti_sandwich(self, other: Origin) -> Flector {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), self[e1234] * other[e4]), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), self[scalar] * other[e4])
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension DualNum: AntiSandwich<Plane> {
    func anti_sandwich(self, other: Plane) -> Flector {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(self.group0().xx, self[scalar], 0.0) * float4(float3(1.0), 0.0) * float4(other.group0().xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e423, e431, e412, e321 */
            float4(self[e1234]) * other.group0()
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension DualNum: AntiSandwich<Point> {
    func anti_sandwich(self, other: Point) -> Flector {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(self[e1234]) * other.group0(), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), self[scalar] * other[e4])
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension DualNum: AntiSandwich<Scalar> {
    func anti_sandwich(self, other: Scalar) -> Scalar {
        return Scalar::from_groups(
            /* scalar */
            self[e1234] * other[scalar]
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension Flector {
    func div(_rhs: anti_sandwich) -> anti_sandwich_partial<Flector> {
        anti_sandwich_partial(this)
    }
}
extension Flector: AntiSandwich<AntiScalar> {
    func anti_sandwich(self, other: AntiScalar) -> Motor {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(other[e1234]) * self.group0(), 
            /* e423, e431, e412, e321 */
            float4(other[e1234]) * self.group1()
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension Flector: AntiSandwich<DualNum> {
    func anti_sandwich(self, other: DualNum) -> Motor {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(((float3(other[e1234]) * self.group0().xyz) - (float3(other[scalar]) * self.group1().xyz)), other[e1234] * self[e4]), 
            /* e423, e431, e412, e321 */
            float4(self[e423], self[e431], self[e412], 1.0) * float4(other.group0().yy, other[e1234], (other[e1234] * self[e321]) - (other[scalar] * self[e4]))
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension Flector: AntiSandwich<Flector> {
    func anti_sandwich(self, other: Flector) -> Flector {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(-(other[e423] * self[e4]) - (other[e412] * self[e431]), -(other[e423] * self[e412]) - (other[e431] * self[e4]), -(other[e431] * self[e423]) - (other[e412] * self[e4]), (other[e431] * self[e431]) + (other[e412] * self[e412])) + (other.group1().yzxx * self.group1().zxyx) - (float4(other[e4]) * float4(self.group1().xyz, self[e4])), 
            /* e23, e31, e12, scalar */
            (float4(other[e321]) * float4(self.group1().xyz, self[e4])) + (float4(self[e4], self[e412], self[e423], self[e1]) * float4(other.group0().xxy, other[e423])) + (float4(self[e431], self[e4], self[e4], self[e2]) * float4(other.group0().zyz, other[e431])) + (other.group1().yzxz * self.group0().zxyz) - (float4(self[e2], self[e321], self[e321], self[e321]) * float4(other.group1().zyz, other[e4])) - (float4(self[e321], self[e3], self[e1], self[e412]) * float4(other.group1().xxy, other[e3])) - (other.group0().yzxx * self.group1().zxyx) - (other.group0().wwwy * float4(self.group0().xyz, self[e431]))
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension Flector: AntiSandwich<Horizon> {
    func anti_sandwich(self, other: Horizon) -> Flector {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(0.0), 
            /* e23, e31, e12, scalar */
            float4(other[e321]) * float4(self.group1().xyz, self[e4])
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension Flector: AntiSandwich<Line> {
    func anti_sandwich(self, other: Line) -> Motor {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4((self[e2] * other[e43]) + (self[e412] * other[e31]) + (self[e321] * other[e41]), (self[e3] * other[e41]) + (self[e423] * other[e12]) + (self[e321] * other[e42]), (self[e1] * other[e42]) + (self[e431] * other[e23]) + (self[e321] * other[e43]), 0.0) - (float4(self[e4], self[e412], self[e423], self[e431]) * float4(other.group1().xxy, other[e42])) - (float4(self[e431], self[e4], self[e4], self[e412]) * float4(other.group1().zyz, other[e43])) - float4((other.group0().yzx * self.group0().zxy), self[e423] * other[e41]), 
            /* e423, e431, e412, e321 */
            (float4(self[e4], self[e412], self[e423], self[e423]) * float4(other.group0().xxy, other[e23])) + (float4(self[e431], self[e4], self[e4], self[e431]) * float4(other.group0().zyz, other[e31])) + float4(float3(0.0), (self[e412] * other[e12]) - (self[e2] * other[e42]) - (self[e3] * other[e43])) - float4((other.group0().yzx * self.group1().zxy), self[e1] * other[e41])
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension Flector: AntiSandwich<Motor> {
    func anti_sandwich(self, other: Motor) -> Motor {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4((self[e2] * other[e43]) + (self[e412] * other[e31]) + (self[e321] * other[e41]) - (self[e431] * other[e12]), (self[e3] * other[e41]) + (self[e423] * other[e12]) + (self[e321] * other[e42]) - (self[e412] * other[e23]), (self[e3] * other[e1234]) + (self[e431] * other[e23]) + (self[e321] * other[e43]) - (self[e412] * other[scalar]), 0.0) + (self.group0().xyxw * other.group0().wwyw) - (self.group1().xyxz * float4(other.group1().wwy, other[e43])) - (other.group0().yzxx * float4(self.group0().zxy, self[e423])) - float4((self.group0().www * other.group1().xyz), self[e431] * other[e42]), 
            /* e423, e431, e412, e321 */
            (self.group1().xyxy * float4(other.group0().wwy, other[e31])) + (self.group1().yzzz * float4(other.group0().zxw, other[e12])) + float4(float3(0.0), (self[e321] * other[e1234]) - (self[e2] * other[e42]) - (self[e3] * other[e43]) - (self[e4] * other[scalar])) + float4((self.group0().www * other.group0().xyz), self[e423] * other[e23]) - (other.group0().yzxx * float4(self.group1().zxy, self[e1]))
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension Flector: AntiSandwich<MultiVector> {
    func anti_sandwich(self, other: MultiVector) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2((self[e4] * other[e321]) - (self[e431] * other[e2]) - (self[e412] * other[e3]) - (self[e321] * other[e4]), 0.0) + (float2(other[e423]) * float2(self[e1], self[e423])) + (float2(other[e431]) * float2(self[e2], self[e431])) + (float2(other[e412]) * float2(self[e3], self[e412])) - (float2(self[e423], self[e4]) * other.group1().xw), 
            /* e1, e2, e3, e4 */
            float4((self[e2] * other[e43]) + (self[e412] * other[e31]) + (self[e321] * other[e41]) - (self[e431] * other[e12]), (self[e3] * other[e41]) + (self[e423] * other[e12]) + (self[e321] * other[e42]) - (self[e4] * other[e31]), (self[e1] * other[e42]) + (self[e431] * other[e23]) + (self[e321] * other[e43]) - (self[e4] * other[e12]), 0.0) + (float4(other[e1234]) * self.group0()) - (float4(self[e4], self[e412], self[e423], self[e412]) * float4(other.group3().xxy, other[e43])) - (self.group1().xyzx * float4(other.group0().xx, other[scalar], other[e41])) - float4((other.group2().yzx * self.group0().zxy), self[e431] * other[e42]), 
            /* e41, e42, e43 */
            (self.group1().zxy * other.group4().yzx) - (float3(self[e4]) * other.group4().xyz) - (float3(other[e4], other[e4], other[e431]) * self.group1().xyx) - (float3(other[e412], other[e423], other[e4]) * self.group1().yzz), 
            /* e23, e31, e12 */
            (float3(self[e4]) * other.group1().xyz) + (float3(other[e3], other[e1], other[e321]) * self.group1().yzz) + (float3(other[e321], other[e321], other[e2]) * self.group1().xyx) + (self.group0().zxy * other.group4().yzx) - (float3(self[e321]) * other.group4().xyz) - (float3(other[e4], other[e4], other[e431]) * self.group0().xyx) - (float3(other[e412], other[e423], other[e4]) * self.group0().yzz) - (self.group1().zxy * other.group1().yzx), 
            /* e423, e431, e412, e321 */
            (float4(other[e1234]) * self.group1()) + (float4(self[e4], self[e412], self[e423], self[e423]) * float4(other.group2().xxy, other[e23])) + (float4(self[e431], self[e4], self[e4], self[e431]) * float4(other.group2().zyz, other[e31])) + float4(float3(0.0), (self[e412] * other[e12]) - (self[e1] * other[e41]) - (self[e2] * other[e42]) - (self[e3] * other[e43])) - float4((other.group2().yzx * self.group1().zxy), self[e4] * other[scalar])
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension Flector: AntiSandwich<Origin> {
    func anti_sandwich(self, other: Origin) -> Flector {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(other[e4]) * float4(self.group1().xyz, self[e4]) * float4(-1.0), 
            /* e23, e31, e12, scalar */
            float4(other[e4]) * float4(self.group0().xyz, self[e321]) * float4(-1.0)
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension Flector: AntiSandwich<Plane> {
    func anti_sandwich(self, other: Plane) -> Flector {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(-(self[e4] * other[e423]) - (self[e431] * other[e412]), -(self[e4] * other[e431]) - (self[e412] * other[e423]), -(self[e4] * other[e412]) - (self[e423] * other[e431]), (self[e431] * other[e431]) + (self[e412] * other[e412])) + (self.group1().zxyx * other.group0().yzxx), 
            /* e23, e31, e12, scalar */
            float4(-(self[e2] * other[e412]) - (self[e321] * other[e423]), -(self[e3] * other[e423]) - (self[e321] * other[e431]), -(self[e1] * other[e431]) - (self[e321] * other[e412]), (self[e3] * other[e412]) + (self[e4] * other[e321])) + (self.group0().zxyx * other.group0().yzxx) + (other.group0().wwwy * float4(self.group1().xyz, self[e2]))
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension Flector: AntiSandwich<Point> {
    func anti_sandwich(self, other: Point) -> Flector {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(other[e4]) * float4(self.group1().xyz, self[e4]) * float4(-1.0), 
            /* e23, e31, e12, scalar */
            float4((self[e4] * other[e1]) + (self[e431] * other[e3]), (self[e4] * other[e2]) + (self[e412] * other[e1]), (self[e4] * other[e3]) + (self[e423] * other[e2]), -(self[e412] * other[e3]) - (self[e321] * other[e4])) - (self.group1().zxyy * other.group0().yzxy) - (other.group0().wwwx * float4(self.group0().xyz, self[e423]))
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension Flector: AntiSandwich<Scalar> {
    func anti_sandwich(self, other: Scalar) -> Motor {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(other[scalar], other[scalar], other[scalar], 0.0) * float4(self.group1().xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), self[e4] * other[scalar] * -1.0)
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension Horizon {
    func div(_rhs: anti_sandwich) -> anti_sandwich_partial<Horizon> {
        anti_sandwich_partial(this)
    }
}
extension Horizon: AntiSandwich<Flector> {
    func anti_sandwich(self, other: Flector) -> Flector {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(0.0), 
            /* e23, e31, e12, scalar */
            float4(self[e321]) * float4(other.group1().xyz, other[e4]) * float4(-1.0)
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension Horizon: AntiSandwich<Line> {
    func anti_sandwich(self, other: Line) -> Scalar {
        return Point::from_groups(
            /* e1, e2, e3, e4 */
            float4(self[e321], self[e321], self[e321], 0.0) * float4(other.group0(), 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension Horizon: AntiSandwich<Motor> {
    func anti_sandwich(self, other: Motor) -> Motor {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(self[e321], self[e321], self[e321], 0.0) * float4(other.group0().xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), self[e321] * other[e1234])
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension Horizon: AntiSandwich<MultiVector> {
    func anti_sandwich(self, other: MultiVector) -> MultiVector {
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
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension Horizon: AntiSandwich<Plane> {
    func anti_sandwich(self, other: Plane) -> Point {
        return Line::from_groups(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(self[e321]) * other.group0().xyz * float3(-1.0)
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension Line {
    func div(_rhs: anti_sandwich) -> anti_sandwich_partial<Line> {
        anti_sandwich_partial(this)
    }
}
extension Line: AntiSandwich<AntiScalar> {
    func anti_sandwich(self, other: AntiScalar) -> Motor {
        return Line::from_groups(
            /* e41, e42, e43 */
            float3(other[e1234]) * self.group0(), 
            /* e23, e31, e12 */
            float3(other[e1234]) * self.group1()
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension Line: AntiSandwich<DualNum> {
    func anti_sandwich(self, other: DualNum) -> Motor {
        return Line::from_groups(
            /* e41, e42, e43 */
            float3(other[e1234]) * self.group0(), 
            /* e23, e31, e12 */
            (float3(other[scalar]) * self.group0()) + (float3(other[e1234]) * self.group1())
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension Line: AntiSandwich<Flector> {
    func anti_sandwich(self, other: Flector) -> Flector {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4((other[e3] * self[e42]) + (other[e4] * self[e23]) + (other[e412] * self[e31]) + (other[e321] * self[e41]), (other[e1] * self[e43]) + (other[e4] * self[e31]) + (other[e423] * self[e12]) + (other[e321] * self[e42]), (other[e2] * self[e41]) + (other[e4] * self[e12]) + (other[e431] * self[e23]) + (other[e321] * self[e43]), other[e412] * self[e43] * -1.0) - (other.group1().yzxy * float4(self.group1().zxy, self[e42])) - float4((self.group0().zxy * other.group0().yzx), other[e423] * self[e41]), 
            /* e423, e431, e412, e321 */
            float4((other[e4] * self[e41]) + (other[e412] * self[e42]), (other[e4] * self[e42]) + (other[e423] * self[e43]), (other[e4] * self[e43]) + (other[e431] * self[e41]), -(other[e2] * self[e42]) - (other[e3] * self[e43]) - (other[e423] * self[e23]) - (other[e431] * self[e31]) - (other[e412] * self[e12])) - float4((self.group0().zxy * other.group1().yzx), other[e1] * self[e41])
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension Line: AntiSandwich<Horizon> {
    func anti_sandwich(self, other: Horizon) -> Flector {
        return Point::from_groups(
            /* e1, e2, e3, e4 */
            float4(other[e321], other[e321], other[e321], 0.0) * float4(self.group0(), 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension Line: AntiSandwich<Line> {
    func anti_sandwich(self, other: Line) -> Motor {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(other[e43] * self[e42], other[e41] * self[e43], other[e42] * self[e41], -(other[e42] * self[e42]) - (other[e43] * self[e43])) - float4((other.group0().yzx * self.group0().zxy), other[e41] * self[e41]), 
            /* e23, e31, e12, scalar */
            float4((other[e43] * self[e31]) + (other[e12] * self[e42]), (other[e41] * self[e12]) + (other[e23] * self[e43]), (other[e42] * self[e23]) + (other[e31] * self[e41]), -(other[e43] * self[e12]) - (other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43])) - float4((other.group0().yzx * self.group1().zxy), other[e41] * self[e23]) - float4((other.group1().yzx * self.group0().zxy), other[e42] * self[e31])
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension Line: AntiSandwich<Motor> {
    func anti_sandwich(self, other: Motor) -> Motor {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4((self[e41] * other[e1234]) + (self[e42] * other[e43]), (self[e42] * other[e1234]) + (self[e43] * other[e41]), (self[e41] * other[e42]) + (self[e43] * other[e1234]), -(self[e42] * other[e42]) - (self[e43] * other[e43])) - (other.group0().yzxx * float4(self.group0().zxy, self[e41])), 
            /* e23, e31, e12, scalar */
            float4((self[e41] * other[scalar]) + (self[e42] * other[e12]) + (self[e23] * other[e1234]) + (self[e31] * other[e43]), (self[e42] * other[scalar]) + (self[e43] * other[e23]) + (self[e31] * other[e1234]) + (self[e12] * other[e41]), (self[e41] * other[e31]) + (self[e43] * other[scalar]) + (self[e23] * other[e42]) + (self[e12] * other[e1234]), -(self[e43] * other[e12]) - (self[e23] * other[e41]) - (self[e31] * other[e42]) - (self[e12] * other[e43])) - (other.group1().yzxx * float4(self.group0().zxy, self[e41])) - float4((self.group1().zxy * other.group0().yzx), self[e42] * other[e31])
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension Line: AntiSandwich<MultiVector> {
    func anti_sandwich(self, other: MultiVector) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(-(self[e23] * other[e41]) - (self[e31] * other[e42]) - (self[e12] * other[e43]), 0.0) - (float2(self[e41]) * float2(other[e23], other[e41])) - (float2(self[e42]) * float2(other[e31], other[e42])) - (float2(self[e43]) * float2(other[e12], other[e43])), 
            /* e1, e2, e3, e4 */
            float4((self[e41] * other[e321]) + (self[e42] * other[e3]) + (self[e23] * other[e4]) + (self[e31] * other[e412]), (self[e42] * other[e321]) + (self[e43] * other[e1]) + (self[e31] * other[e4]) + (self[e12] * other[e423]), (self[e41] * other[e2]) + (self[e43] * other[e321]) + (self[e23] * other[e431]) + (self[e12] * other[e4]), self[e43] * other[e412] * -1.0) - (other.group4().yzxy * float4(self.group1().zxy, self[e42])) - float4((self.group0().zxy * other.group1().yzx), self[e41] * other[e423]), 
            /* e41, e42, e43 */
            (float3(other[e1234]) * self.group0()) + (self.group0().yzx * other.group2().zxy) - (self.group0().zxy * other.group2().yzx), 
            /* e23, e31, e12 */
            (float3(other[scalar]) * self.group0()) + (float3(other[e1234]) * self.group1()) + (self.group0().yzx * other.group3().zxy) + (self.group1().yzx * other.group2().zxy) - (self.group0().zxy * other.group3().yzx) - (self.group1().zxy * other.group2().yzx), 
            /* e423, e431, e412, e321 */
            float4((self[e41] * other[e4]) + (self[e42] * other[e412]), (self[e42] * other[e4]) + (self[e43] * other[e423]), (self[e41] * other[e431]) + (self[e43] * other[e4]), -(self[e42] * other[e2]) - (self[e43] * other[e3]) - (self[e23] * other[e423]) - (self[e31] * other[e431]) - (self[e12] * other[e412])) - float4((self.group0().zxy * other.group4().yzx), self[e41] * other[e1])
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension Line: AntiSandwich<Origin> {
    func anti_sandwich(self, other: Origin) -> Flector {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(other[e4], other[e4], other[e4], 0.0) * float4(self.group1(), 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e423, e431, e412, e321 */
            float4(other[e4], other[e4], other[e4], 0.0) * float4(self.group0(), 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension Line: AntiSandwich<Plane> {
    func anti_sandwich(self, other: Plane) -> Flector {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4((self[e41] * other[e321]) + (self[e31] * other[e412]), (self[e42] * other[e321]) + (self[e12] * other[e423]), (self[e43] * other[e321]) + (self[e23] * other[e431]), -(self[e42] * other[e431]) - (self[e43] * other[e412])) - (other.group0().yzxx * float4(self.group1().zxy, self[e41])), 
            /* e423, e431, e412, e321 */
            float4(self[e42] * other[e412], self[e43] * other[e423], self[e41] * other[e431], -(self[e31] * other[e431]) - (self[e12] * other[e412])) - (other.group0().yzxx * float4(self.group0().zxy, self[e23]))
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension Line: AntiSandwich<Point> {
    func anti_sandwich(self, other: Point) -> Flector {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4((float3(other[e4]) * self.group1()), 0.0) + float4((self.group0().yzx * other.group0().zxy), 0.0) - float4((self.group0().zxy * other.group0().yzx), 0.0), 
            /* e423, e431, e412, e321 */
            float4(other[e4], other[e4], other[e4], 1.0) * float4(self.group0(), -(self[e41] * other[e1]) - (self[e42] * other[e2]) - (self[e43] * other[e3]))
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension Line: AntiSandwich<Scalar> {
    func anti_sandwich(self, other: Scalar) -> Motor {
        return Line::from_groups(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(other[scalar]) * self.group0()
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension Motor {
    func div(_rhs: anti_sandwich) -> anti_sandwich_partial<Motor> {
        anti_sandwich_partial(this)
    }
}
extension Motor: AntiSandwich<AntiScalar> {
    func anti_sandwich(self, other: AntiScalar) -> Motor {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(other[e1234]) * self.group0(), 
            /* e23, e31, e12, scalar */
            float4(other[e1234]) * self.group1()
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension Motor: AntiSandwich<DualNum> {
    func anti_sandwich(self, other: DualNum) -> Motor {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(other[e1234]) * self.group0(), 
            /* e23, e31, e12, scalar */
            (float4(other[scalar]) * self.group0()) + (float4(other[e1234]) * self.group1())
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension Motor: AntiSandwich<Flector> {
    func anti_sandwich(self, other: Flector) -> Flector {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4((other[e3] * self[e42]) + (other[e4] * self[e23]) + (other[e423] * self[scalar]) + (other[e412] * self[e31]) + (other[e321] * self[e41]), (other[e2] * self[e1234]) + (other[e4] * self[e31]) + (other[e423] * self[e12]) + (other[e431] * self[scalar]) + (other[e321] * self[e42]), (other[e3] * self[e1234]) + (other[e4] * self[e12]) + (other[e431] * self[e23]) + (other[e412] * self[scalar]) + (other[e321] * self[e43]), other[e412] * self[e43] * -1.0) + (other.group0().xxyw * self.group0().wzxw) - (other.group1().yzxy * float4(self.group1().zxy, self[e42])) - (self.group0().zxyx * float4(other.group0().yzx, other[e423])), 
            /* e423, e431, e412, e321 */
            float4(other[e412] * self[e42], other[e431] * self[e1234], other[e412] * self[e1234], -(other[e2] * self[e42]) - (other[e3] * self[e43]) - (other[e423] * self[e23]) - (other[e431] * self[e31]) - (other[e412] * self[e12])) + (float4(other[e4]) * float4(self.group0().xyz, self[scalar])) + (other.group1().xxyw * self.group0().wzxw) - (self.group0().zxyx * float4(other.group1().yzx, other[e1]))
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension Motor: AntiSandwich<Horizon> {
    func anti_sandwich(self, other: Horizon) -> Flector {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(other[e321], other[e321], other[e321], 0.0) * float4(self.group0().xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other[e321] * self[e1234])
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension Motor: AntiSandwich<Line> {
    func anti_sandwich(self, other: Line) -> Motor {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4((other[e41] * self[e1234]) + (other[e43] * self[e42]), (other[e41] * self[e43]) + (other[e42] * self[e1234]), (other[e42] * self[e41]) + (other[e43] * self[e1234]), -(other[e42] * self[e42]) - (other[e43] * self[e43])) - (self.group0().zxyx * float4(other.group0().yzx, other[e41])), 
            /* e23, e31, e12, scalar */
            float4((other[e41] * self[scalar]) + (other[e43] * self[e31]) + (other[e23] * self[e1234]) + (other[e12] * self[e42]), (other[e41] * self[e12]) + (other[e42] * self[scalar]) + (other[e23] * self[e43]) + (other[e31] * self[e1234]), (other[e42] * self[e23]) + (other[e43] * self[scalar]) + (other[e31] * self[e41]) + (other[e12] * self[e1234]), -(other[e43] * self[e12]) - (other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43])) - (self.group1().zxyx * float4(other.group0().yzx, other[e41])) - float4((other.group1().yzx * self.group0().zxy), other[e42] * self[e31])
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension Motor: AntiSandwich<Motor> {
    func anti_sandwich(self, other: Motor) -> Motor {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4((other[e43] * self[e42]) + (other[e1234] * self[e41]), (other[e42] * self[e1234]) + (other[e1234] * self[e42]), (other[e43] * self[e1234]) + (other[e1234] * self[e43]), -(other[e42] * self[e42]) - (other[e43] * self[e43])) + (other.group0().xxyw * self.group0().wzxw) - (other.group0().yzxx * self.group0().zxyx), 
            /* e23, e31, e12, scalar */
            float4((other[e1234] * self[e23]) + (other[e23] * self[e1234]) + (other[e12] * self[e42]) + (other[scalar] * self[e41]), (other[e1234] * self[e31]) + (other[e23] * self[e43]) + (other[e31] * self[e1234]) + (other[scalar] * self[e42]), (other[e1234] * self[e12]) + (other[e31] * self[e41]) + (other[e12] * self[e1234]) + (other[scalar] * self[e43]), -(other[e43] * self[e12]) - (other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43])) + (other.group0().xxyw * self.group1().wzxw) + float4((other.group0().zyz * self.group1().yww), other[scalar] * self[e1234]) - (other.group0().yzxx * self.group1().zxyx) - float4((other.group1().yzx * self.group0().zxy), other[e42] * self[e31])
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension Motor: AntiSandwich<MultiVector> {
    func anti_sandwich(self, other: MultiVector) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2((self[scalar] * other[e1234]) - (self[e41] * other[e23]) - (self[e42] * other[e31]) - (self[e43] * other[e12]), 0.0) + (float2(self[e1234]) * other.group0()) - (float2(other[e41]) * float2(self[e23], self[e41])) - (float2(other[e42]) * float2(self[e31], self[e42])) - (float2(other[e43]) * float2(self[e12], self[e43])), 
            /* e1, e2, e3, e4 */
            float4((self[e42] * other[e3]) + (self[e1234] * other[e1]) + (self[e23] * other[e4]) + (self[e31] * other[e412]) + (self[scalar] * other[e423]), (self[e43] * other[e1]) + (self[e1234] * other[e2]) + (self[e31] * other[e4]) + (self[e12] * other[e423]) + (self[scalar] * other[e431]), (self[e43] * other[e321]) + (self[e1234] * other[e3]) + (self[e23] * other[e431]) + (self[e12] * other[e4]) + (self[scalar] * other[e412]), self[e43] * other[e412] * -1.0) + (self.group0().xyxw * float4(other.group4().ww, other[e2], other[e4])) - (self.group0().zxyx * float4(other.group1().yzx, other[e423])) - (other.group4().yzxy * float4(self.group1().zxy, self[e42])), 
            /* e41, e42, e43 */
            (float3(other[e1234]) * self.group0().xyz) + (other.group2().xxy * self.group0().wzx) + (other.group2().zyz * self.group0().yww) - (other.group2().yzx * self.group0().zxy), 
            /* e23, e31, e12 */
            (float3(other[scalar]) * self.group0().xyz) + (float3(other[e1234]) * self.group1().xyz) + (other.group2().xxy * self.group1().wzx) + (other.group2().zyz * self.group1().yww) + (other.group3().xxy * self.group0().wzx) + (other.group3().zyz * self.group0().yww) - (other.group2().yzx * self.group1().zxy) - (other.group3().yzx * self.group0().zxy), 
            /* e423, e431, e412, e321 */
            float4(self[e1234] * other[e423], self[e1234] * other[e431], self[e1234] * other[e412], -(self[e42] * other[e2]) - (self[e43] * other[e3]) - (self[e23] * other[e423]) - (self[e31] * other[e431]) - (self[e12] * other[e412])) + (self.group0().xyxw * float4(other.group1().ww, other[e431], other[e321])) + (float4(other.group4().zx, other[e4], other[e4]) * float4(self.group0().yzz, self[scalar])) - (self.group0().zxyx * float4(other.group4().yzx, other[e1]))
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension Motor: AntiSandwich<Origin> {
    func anti_sandwich(self, other: Origin) -> Flector {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(other[e4]) * float4(self.group1().xyz, self[e1234]), 
            /* e423, e431, e412, e321 */
            float4(other[e4]) * float4(self.group0().xyz, self[scalar])
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension Motor: AntiSandwich<Plane> {
    func anti_sandwich(self, other: Plane) -> Flector {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4((self[e41] * other[e321]) + (self[e31] * other[e412]) + (self[scalar] * other[e423]), (self[e42] * other[e321]) + (self[e12] * other[e423]) + (self[scalar] * other[e431]), (self[e43] * other[e321]) + (self[e23] * other[e431]) + (self[scalar] * other[e412]), -(self[e42] * other[e431]) - (self[e43] * other[e412])) - (other.group0().yzxx * float4(self.group1().zxy, self[e41])), 
            /* e423, e431, e412, e321 */
            float4(self[e1234] * other[e423], self[e1234] * other[e431], self[e1234] * other[e412], -(self[e31] * other[e431]) - (self[e12] * other[e412])) + (self.group0().yzxw * other.group0().zxyw) - (other.group0().yzxx * float4(self.group0().zxy, self[e23]))
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension Motor: AntiSandwich<Point> {
    func anti_sandwich(self, other: Point) -> Flector {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(((float3(self[e1234]) * other.group0().xyz) + (float3(other[e4]) * self.group1().xyz) + (self.group0().yzx * other.group0().zxy) - (self.group0().zxy * other.group0().yzx)), self[e1234] * other[e4]), 
            /* e423, e431, e412, e321 */
            float4(other[e4], other[e4], other[e4], 1.0) * float4(self.group0().xyz, (self[scalar] * other[e4]) - (self[e41] * other[e1]) - (self[e42] * other[e2]) - (self[e43] * other[e3]))
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension Motor: AntiSandwich<Scalar> {
    func anti_sandwich(self, other: Scalar) -> Motor {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(0.0), 
            /* e23, e31, e12, scalar */
            float4(other[scalar]) * self.group0()
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension MultiVector {
    func div(_rhs: anti_sandwich) -> anti_sandwich_partial<MultiVector> {
        anti_sandwich_partial(this)
    }
}
extension MultiVector: AntiSandwich<AntiScalar> {
    func anti_sandwich(self, other: AntiScalar) -> MultiVector {
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
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension MultiVector: AntiSandwich<DualNum> {
    func anti_sandwich(self, other: DualNum) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2((other[scalar] * self[e1234]) + (other[e1234] * self[scalar]), other[e1234] * self[e1234]), 
            /* e1, e2, e3, e4 */
            float4(((float3(other[e1234]) * self.group1().xyz) - (float3(other[scalar]) * self.group4().xyz)), other[e1234] * self[e4]), 
            /* e41, e42, e43 */
            float3(other[e1234]) * self.group2(), 
            /* e23, e31, e12 */
            (float3(other[scalar]) * self.group2()) + (float3(other[e1234]) * self.group3()), 
            /* e423, e431, e412, e321 */
            float4(self[e423], self[e431], self[e412], 1.0) * float4(other.group0().yy, other[e1234], (other[e1234] * self[e321]) - (other[scalar] * self[e4]))
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension MultiVector: AntiSandwich<Flector> {
    func anti_sandwich(self, other: Flector) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2((other[e321] * self[e4]) - (other[e2] * self[e431]) - (other[e3] * self[e412]) - (other[e4] * self[e321]), 0.0) + (float2(other[e423]) * float2(self[e1], self[e423])) + (float2(other[e431]) * float2(self[e2], self[e431])) + (float2(other[e412]) * float2(self[e3], self[e412])) - (float2(self[e423], self[e4]) * other.group0().xw), 
            /* e1, e2, e3, e4 */
            float4((other[e1] * self[e1234]) + (other[e3] * self[e42]) + (other[e4] * self[e23]) + (other[e412] * self[e31]) + (other[e321] * self[e41]), (other[e1] * self[e43]) + (other[e2] * self[e1234]) + (other[e4] * self[e31]) + (other[e423] * self[e12]) + (other[e321] * self[e42]), (other[e2] * self[e41]) + (other[e3] * self[e1234]) + (other[e4] * self[e12]) + (other[e431] * self[e23]) + (other[e321] * self[e43]), other[e412] * self[e43] * -1.0) + (float4(self.group0().xx, self[scalar], self[e1234]) * float4(other.group1().xyz, other[e4])) - (other.group1().yzxy * float4(self.group3().zxy, self[e42])) - float4((self.group2().zxy * other.group0().yzx), other[e423] * self[e41]), 
            /* e41, e42, e43 */
            (other.group1().yzx * self.group4().zxy) - (float3(other[e4]) * self.group4().xyz) - (float3(self[e4], self[e412], self[e423]) * other.group1().xxy) - (float3(self[e431], self[e4], self[e4]) * other.group1().zyz), 
            /* e23, e31, e12 */
            (float3(other[e321]) * self.group4().xyz) + (float3(self[e4], self[e412], self[e423]) * other.group0().xxy) + (float3(self[e431], self[e4], self[e4]) * other.group0().zyz) + (other.group1().yzx * self.group1().zxy) - (float3(other[e4]) * self.group1().xyz) - (float3(self[e2], self[e321], self[e321]) * other.group1().zyz) - (float3(self[e321], self[e3], self[e1]) * other.group1().xxy) - (other.group0().yzx * self.group4().zxy), 
            /* e423, e431, e412, e321 */
            float4(other[e412] * self[e42], other[e423] * self[e43], other[e4] * self[e43], -(other[e2] * self[e42]) - (other[e3] * self[e43]) - (other[e423] * self[e23]) - (other[e431] * self[e31]) - (other[e412] * self[e12])) + (float4(self.group0().yy, self[e1234], self[scalar]) * float4(other.group1().xyz, other[e4])) + (float4(other.group0().ww, other[e431], other[e321]) * float4(self.group2().xyx, self[e1234])) - float4((self.group2().zxy * other.group1().yzx), other[e1] * self[e41])
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension MultiVector: AntiSandwich<Horizon> {
    func anti_sandwich(self, other: Horizon) -> MultiVector {
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
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension MultiVector: AntiSandwich<Line> {
    func anti_sandwich(self, other: Line) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(-(other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43]), 0.0) - (float2(other[e41]) * float2(self[e23], self[e41])) - (float2(other[e42]) * float2(self[e31], self[e42])) - (float2(other[e43]) * float2(self[e12], self[e43])), 
            /* e1, e2, e3, e4 */
            float4((other[e41] * self[e321]) + (other[e43] * self[e2]) + (other[e31] * self[e412]), (other[e41] * self[e3]) + (other[e42] * self[e321]) + (other[e12] * self[e423]), (other[e42] * self[e1]) + (other[e43] * self[e321]) + (other[e23] * self[e431]), 0.0) - (float4(self[e4], self[e412], self[e423], self[e431]) * float4(other.group1().xxy, other[e42])) - (float4(self[e431], self[e4], self[e4], self[e412]) * float4(other.group1().zyz, other[e43])) - float4((other.group0().yzx * self.group1().zxy), other[e41] * self[e423]), 
            /* e41, e42, e43 */
            (float3(self[e1234]) * other.group0()) + (other.group0().zxy * self.group2().yzx) - (other.group0().yzx * self.group2().zxy), 
            /* e23, e31, e12 */
            (float3(self[scalar]) * other.group0()) + (float3(self[e1234]) * other.group1()) + (other.group0().zxy * self.group3().yzx) + (other.group1().zxy * self.group2().yzx) - (other.group0().yzx * self.group3().zxy) - (other.group1().yzx * self.group2().zxy), 
            /* e423, e431, e412, e321 */
            (float4(self[e4], self[e412], self[e423], self[e423]) * float4(other.group0().xxy, other[e23])) + (float4(self[e431], self[e4], self[e4], self[e431]) * float4(other.group0().zyz, other[e31])) + float4(float3(0.0), (other[e12] * self[e412]) - (other[e42] * self[e2]) - (other[e43] * self[e3])) - float4((other.group0().yzx * self.group4().zxy), other[e41] * self[e1])
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension MultiVector: AntiSandwich<Motor> {
    func anti_sandwich(self, other: Motor) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2((other[scalar] * self[e1234]) - (other[e41] * self[e23]) - (other[e42] * self[e31]) - (other[e43] * self[e12]), 0.0) + (float2(other[e1234]) * self.group0()) - (float2(self[e41]) * float2(other[e23], other[e41])) - (float2(self[e42]) * float2(other[e31], other[e42])) - (float2(self[e43]) * float2(other[e12], other[e43])), 
            /* e1, e2, e3, e4 */
            float4((other[e43] * self[e2]) + (other[e1234] * self[e1]) + (other[e31] * self[e412]) - (other[scalar] * self[e423]), (other[e42] * self[e321]) + (other[e1234] * self[e2]) + (other[e12] * self[e423]) - (other[scalar] * self[e431]), (other[e43] * self[e321]) + (other[e1234] * self[e3]) + (other[e23] * self[e431]) - (other[scalar] * self[e412]), 0.0) + (float4(self[e321], self[e3], self[e1], self[e4]) * other.group0().xxyw) - (float4(self[e4], self[e412], self[e423], self[e431]) * float4(other.group1().xxy, other[e42])) - (float4(self[e431], self[e4], self[e4], self[e412]) * float4(other.group1().zyz, other[e43])) - (other.group0().yzxx * float4(self.group1().zxy, self[e423])), 
            /* e41, e42, e43 */
            (float3(self[e1234]) * other.group0().xyz) + (self.group2().xyx * other.group0().wwy) + (self.group2().yzz * other.group0().zxw) - (self.group2().zxy * other.group0().yzx), 
            /* e23, e31, e12 */
            (float3(self[scalar]) * other.group0().xyz) + (float3(self[e1234]) * other.group1().xyz) + (self.group2().xyx * other.group1().wwy) + (self.group2().yzz * other.group1().zxw) + (self.group3().xyx * other.group0().wwy) + (self.group3().yzz * other.group0().zxw) - (self.group2().zxy * other.group1().yzx) - (self.group3().zxy * other.group0().yzx), 
            /* e423, e431, e412, e321 */
            (float4(self[e4], self[e412], self[e423], self[e321]) * other.group0().xxyw) + (float4(self[e431], self[e4], self[e4], self[e423]) * float4(other.group0().zyz, other[e23])) + (self.group4().xyzy * float4(other.group0().www, other[e31])) + float4(float3(0.0), (other[e12] * self[e412]) - (other[e42] * self[e2]) - (other[e43] * self[e3]) - (other[scalar] * self[e4])) - (other.group0().yzxx * float4(self.group4().zxy, self[e1]))
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension MultiVector: AntiSandwich<MultiVector> {
    func anti_sandwich(self, other: MultiVector) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2((other[e1234] * self[scalar]) + (other[e321] * self[e4]) - (other[e2] * self[e431]) - (other[e3] * self[e412]) - (other[e4] * self[e321]) - (other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43]), 0.0) + (float2(other[e423]) * float2(self[e1], self[e423])) + (float2(other[e431]) * float2(self[e2], self[e431])) + (float2(other[e412]) * float2(self[e3], self[e412])) + (float2(self[e1234]) * other.group0()) - (float2(other[e41]) * float2(self[e23], self[e41])) - (float2(other[e42]) * float2(self[e31], self[e42])) - (float2(other[e43]) * float2(self[e12], self[e43])) - (float2(self[e423], self[e4]) * other.group1().xw), 
            /* e1, e2, e3, e4 */
            float4((other[e1] * self[e1234]) + (other[e3] * self[e42]) + (other[e4] * self[e23]) + (other[e41] * self[e321]) + (other[e43] * self[e2]) + (other[e31] * self[e412]) + (other[e412] * self[e31]) + (other[e321] * self[e41]), (other[e1] * self[e43]) + (other[e2] * self[e1234]) + (other[e4] * self[e31]) + (other[e41] * self[e3]) + (other[e42] * self[e321]) + (other[e12] * self[e423]) + (other[e423] * self[e12]) + (other[e321] * self[e42]), (other[e2] * self[e41]) + (other[e3] * self[e1234]) + (other[e4] * self[e12]) + (other[e42] * self[e1]) + (other[e43] * self[e321]) + (other[e23] * self[e431]) + (other[e431] * self[e23]) + (other[e321] * self[e43]), 0.0) + (float4(other[e1234]) * self.group1()) + (float4(self.group0().xx, self[scalar], self[e1234]) * float4(other.group4().xyz, other[e4])) - (float4(self[e4], self[e412], self[e423], self[e412]) * float4(other.group3().xxy, other[e43])) - (float4(self[e431], self[e4], self[e4], other[e423]) * float4(other.group3().zyz, self[e41])) - (other.group4().yzxz * float4(self.group3().zxy, self[e43])) - (self.group4().xyzx * float4(other.group0().xx, other[scalar], other[e41])) - float4((other.group2().yzx * self.group1().zxy), other[e42] * self[e431]) - float4((self.group2().zxy * other.group1().yzx), other[e431] * self[e42]), 
            /* e41, e42, e43 */
            (float3(other[e1234]) * self.group2()) + (float3(self[e1234]) * other.group2()) + (other.group2().zxy * self.group2().yzx) + (other.group4().yzx * self.group4().zxy) - (float3(other[e4]) * self.group4().xyz) - (float3(self[e4], self[e412], self[e423]) * other.group4().xxy) - (float3(self[e431], self[e4], self[e4]) * other.group4().zyz) - (other.group2().yzx * self.group2().zxy), 
            /* e23, e31, e12 */
            (float3(other[scalar]) * self.group2()) + (float3(other[e1234]) * self.group3()) + (float3(other[e321]) * self.group4().xyz) + (float3(self[scalar]) * other.group2()) + (float3(self[e1234]) * other.group3()) + (float3(self[e4], self[e412], self[e423]) * other.group1().xxy) + (float3(self[e431], self[e4], self[e4]) * other.group1().zyz) + (other.group2().zxy * self.group3().yzx) + (other.group3().zxy * self.group2().yzx) + (other.group4().yzx * self.group1().zxy) - (float3(other[e4]) * self.group1().xyz) - (float3(self[e2], self[e321], self[e321]) * other.group4().zyz) - (float3(self[e321], self[e3], self[e1]) * other.group4().xxy) - (other.group2().yzx * self.group3().zxy) - (other.group3().yzx * self.group2().zxy) - (other.group1().yzx * self.group4().zxy), 
            /* e423, e431, e412, e321 */
            (float4(other[e1234]) * self.group4()) + (float4(self[e4], self[e412], self[e423], other[e321]) * float4(other.group2().xxy, self[e1234])) + (float4(self[e431], self[e4], self[e4], self[e423]) * float4(other.group2().zyz, other[e23])) + (float4(self.group0().yy, self[e1234], self[scalar]) * float4(other.group4().xyz, other[e4])) + (float4(other.group1().ww, other[e431], self[e431]) * float4(self.group2().xyx, other[e31])) + (float4(other.group4().zx, other[e4], self[e412]) * float4(self.group2().yzz, other[e12])) + float4(float3(0.0), -(other[e1] * self[e41]) - (other[e2] * self[e42]) - (other[e3] * self[e43]) - (other[e42] * self[e2]) - (other[e43] * self[e3]) - (other[e423] * self[e23]) - (other[e431] * self[e31]) - (other[e412] * self[e12])) - float4((other.group2().yzx * self.group4().zxy), other[scalar] * self[e4]) - float4((self.group2().zxy * other.group4().yzx), other[e41] * self[e1])
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension MultiVector: AntiSandwich<Origin> {
    func anti_sandwich(self, other: Origin) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(other[e4]) * float2(self[e321], self[e4]) * float2(-1.0), 
            /* e1, e2, e3, e4 */
            float4(other[e4]) * float4(self.group3(), self[e1234]), 
            /* e41, e42, e43 */
            float3(other[e4]) * self.group4().xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(other[e4]) * self.group1().xyz * float3(-1.0), 
            /* e423, e431, e412, e321 */
            float4(other[e4]) * float4(self.group2(), self[scalar])
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension MultiVector: AntiSandwich<Plane> {
    func anti_sandwich(self, other: Plane) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(self[e4] * other[e321], 0.0) + (float2(other[e423]) * float2(self[e1], self[e423])) + (float2(other[e431]) * float2(self[e2], self[e431])) + (float2(other[e412]) * float2(self[e3], self[e412])), 
            /* e1, e2, e3, e4 */
            float4((self[scalar] * other[e423]) + (self[e41] * other[e321]) + (self[e31] * other[e412]), (self[scalar] * other[e431]) + (self[e42] * other[e321]) + (self[e12] * other[e423]), (self[scalar] * other[e412]) + (self[e43] * other[e321]) + (self[e23] * other[e431]), -(self[e42] * other[e431]) - (self[e43] * other[e412])) - (other.group0().yzxx * float4(self.group3().zxy, self[e41])), 
            /* e41, e42, e43 */
            (self.group4().zxy * other.group0().yzx) - (float3(self[e4]) * other.group0().xyz) - (self.group4().yzx * other.group0().zxy), 
            /* e23, e31, e12 */
            (float3(other[e321]) * self.group4().xyz) + (self.group1().zxy * other.group0().yzx) - (float3(self[e321]) * other.group0().xyz) - (self.group1().yzx * other.group0().zxy), 
            /* e423, e431, e412, e321 */
            float4(self[e42] * other[e412], self[e43] * other[e423], self[e41] * other[e431], -(self[e31] * other[e431]) - (self[e12] * other[e412])) + (float4(self[e1234]) * other.group0()) - (other.group0().yzxx * float4(self.group2().zxy, self[e23]))
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension MultiVector: AntiSandwich<Point> {
    func anti_sandwich(self, other: Point) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(-(self[e423] * other[e1]) - (self[e431] * other[e2]) - (self[e412] * other[e3]) - (self[e321] * other[e4]), self[e4] * other[e4]) * float2(1.0, -1.0), 
            /* e1, e2, e3, e4 */
            float4(((float3(self[e1234]) * other.group0().xyz) + (float3(other[e4]) * self.group3()) + (self.group2().yzx * other.group0().zxy) - (self.group2().zxy * other.group0().yzx)), self[e1234] * other[e4]), 
            /* e41, e42, e43 */
            float3(other[e4]) * self.group4().xyz * float3(-1.0), 
            /* e23, e31, e12 */
            (float3(self[e4]) * other.group0().xyz) + (self.group4().yzx * other.group0().zxy) - (float3(other[e4]) * self.group1().xyz) - (self.group4().zxy * other.group0().yzx), 
            /* e423, e431, e412, e321 */
            float4(other[e4], other[e4], other[e4], 1.0) * float4(self.group2(), (self[scalar] * other[e4]) - (self[e41] * other[e1]) - (self[e42] * other[e2]) - (self[e43] * other[e3]))
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension MultiVector: AntiSandwich<Scalar> {
    func anti_sandwich(self, other: Scalar) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(self[e1234] * other[scalar], 1.0) * float2(1.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(other[scalar], other[scalar], other[scalar], 0.0) * float4(self.group4().xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(other[scalar]) * self.group2(), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), self[e4] * other[scalar] * -1.0)
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension Origin {
    func div(_rhs: anti_sandwich) -> anti_sandwich_partial<Origin> {
        anti_sandwich_partial(this)
    }
}
extension Origin: AntiSandwich<AntiScalar> {
    func anti_sandwich(self, other: AntiScalar) -> AntiScalar {
        return Origin::from_groups(
            /* e4 */
            other[e1234] * self[e4]
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension Origin: AntiSandwich<DualNum> {
    func anti_sandwich(self, other: DualNum) -> Motor {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other[e1234] * self[e4]), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other[scalar] * self[e4] * -1.0)
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension Origin: AntiSandwich<Flector> {
    func anti_sandwich(self, other: Flector) -> Flector {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(self[e4]) * float4(other.group1().xyz, other[e4]) * float4(-1.0), 
            /* e23, e31, e12, scalar */
            float4(self[e4]) * float4(other.group0().xyz, other[e321])
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension Origin: AntiSandwich<Horizon> {
    func anti_sandwich(self, other: Horizon) -> Horizon {
        return Scalar::from_groups(
            /* scalar */
            other[e321] * self[e4]
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension Origin: AntiSandwich<Line> {
    func anti_sandwich(self, other: Line) -> Motor {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(self[e4], self[e4], self[e4], 0.0) * float4(other.group1(), 0.0) * float4(-1.0, -1.0, -1.0, 0.0), 
            /* e423, e431, e412, e321 */
            float4(self[e4], self[e4], self[e4], 0.0) * float4(other.group0(), 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension Origin: AntiSandwich<Motor> {
    func anti_sandwich(self, other: Motor) -> Motor {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(self[e4]) * float4(other.group1().xyz, other[e1234]) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e423, e431, e412, e321 */
            float4(self[e4]) * float4(other.group0().xyz, other[scalar]) * float4(1.0, 1.0, 1.0, -1.0)
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension Origin: AntiSandwich<MultiVector> {
    func anti_sandwich(self, other: MultiVector) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(self[e4]) * float2(other[e321], other[e4]) * float2(1.0, -1.0), 
            /* e1, e2, e3, e4 */
            float4(self[e4]) * float4(other.group3(), other[e1234]) * float4(-1.0, -1.0, -1.0, 1.0), 
            /* e41, e42, e43 */
            float3(self[e4]) * other.group4().xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(self[e4]) * other.group1().xyz, 
            /* e423, e431, e412, e321 */
            float4(self[e4]) * float4(other.group2(), other[scalar]) * float4(1.0, 1.0, 1.0, -1.0)
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension Origin: AntiSandwich<Origin> {
    func anti_sandwich(self, other: Origin) -> Origin {
        return AntiScalar::from_groups(
            /* e1234 */
            other[e4] * self[e4] * -1.0
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension Origin: AntiSandwich<Plane> {
    func anti_sandwich(self, other: Plane) -> Flector {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(self[e4], self[e4], self[e4], 0.0) * float4(other.group0().xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0), 
            /* e23, e31, e12, scalar */
            float4(float3(0.0), self[e4] * other[e321])
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension Origin: AntiSandwich<Point> {
    func anti_sandwich(self, other: Point) -> Flector {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(float3(0.0), self[e4] * other[e4] * -1.0), 
            /* e23, e31, e12, scalar */
            float4(self[e4], self[e4], self[e4], 0.0) * float4(other.group0().xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension Origin: AntiSandwich<Scalar> {
    func anti_sandwich(self, other: Scalar) -> Scalar {
        return Horizon::from_groups(
            /* e321 */
            self[e4] * other[scalar] * -1.0
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension Plane {
    func div(_rhs: anti_sandwich) -> anti_sandwich_partial<Plane> {
        anti_sandwich_partial(this)
    }
}
extension Plane: AntiSandwich<AntiScalar> {
    func anti_sandwich(self, other: AntiScalar) -> Motor {
        return Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4(other[e1234]) * self.group0()
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension Plane: AntiSandwich<DualNum> {
    func anti_sandwich(self, other: DualNum) -> Motor {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(other.group0().xx, other[scalar], 0.0) * float4(float3(1.0), 0.0) * float4(self.group0().xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0), 
            /* e423, e431, e412, e321 */
            float4(other[e1234]) * self.group0()
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension Plane: AntiSandwich<Flector> {
    func anti_sandwich(self, other: Flector) -> Flector {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(-(other[e4] * self[e423]) - (other[e412] * self[e431]), -(other[e4] * self[e431]) - (other[e423] * self[e412]), -(other[e4] * self[e412]) - (other[e431] * self[e423]), (other[e431] * self[e431]) + (other[e412] * self[e412])) + (other.group1().yzxx * self.group0().zxyx), 
            /* e23, e31, e12, scalar */
            float4((other[e3] * self[e431]) + (other[e321] * self[e423]), (other[e1] * self[e412]) + (other[e321] * self[e431]), (other[e2] * self[e423]) + (other[e321] * self[e412]), -(other[e3] * self[e412]) - (other[e4] * self[e321])) - (other.group0().yzxx * self.group0().zxyx) - (self.group0().wwwy * float4(other.group1().xyz, other[e2]))
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension Plane: AntiSandwich<Horizon> {
    func anti_sandwich(self, other: Horizon) -> Flector {
        return Line::from_groups(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(other[e321]) * self.group0().xyz
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension Plane: AntiSandwich<Line> {
    func anti_sandwich(self, other: Line) -> Motor {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4((other[e41] * self[e321]) + (other[e31] * self[e412]), (other[e42] * self[e321]) + (other[e12] * self[e423]), (other[e43] * self[e321]) + (other[e23] * self[e431]), -(other[e42] * self[e431]) - (other[e43] * self[e412])) - (self.group0().yzxx * float4(other.group1().zxy, other[e41])), 
            /* e423, e431, e412, e321 */
            float4(other[e42] * self[e412] * -1.0, other[e43] * self[e423] * -1.0, other[e41] * self[e431] * -1.0, (other[e31] * self[e431]) + (other[e12] * self[e412])) + (self.group0().yzxx * float4(other.group0().zxy, other[e23]))
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension Plane: AntiSandwich<Motor> {
    func anti_sandwich(self, other: Motor) -> Motor {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4((other[e41] * self[e321]) + (other[e31] * self[e412]), (other[e42] * self[e321]) + (other[e12] * self[e423]), (other[e43] * self[e321]) + (other[e23] * self[e431]), other[e43] * self[e412] * -1.0) - (self.group0().xyzy * float4(other.group1().www, other[e42])) - (self.group0().yzxx * float4(other.group1().zxy, other[e41])), 
            /* e423, e431, e412, e321 */
            float4(other[e42] * self[e412] * -1.0, other[e43] * self[e423] * -1.0, other[e41] * self[e431] * -1.0, (other[e31] * self[e431]) + (other[e12] * self[e412])) + (other.group0().zxyw * self.group0().yzxw) + (self.group0().xyzx * float4(other.group0().www, other[e23]))
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension Plane: AntiSandwich<MultiVector> {
    func anti_sandwich(self, other: MultiVector) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(-(other[e1] * self[e423]) - (other[e2] * self[e431]) - (other[e3] * self[e412]) - (other[e4] * self[e321]), (other[e423] * self[e423]) + (other[e431] * self[e431]) + (other[e412] * self[e412])), 
            /* e1, e2, e3, e4 */
            float4((other[e41] * self[e321]) + (other[e31] * self[e412]), (other[e42] * self[e321]) + (other[e12] * self[e423]), (other[e43] * self[e321]) + (other[e23] * self[e431]), other[e43] * self[e412] * -1.0) - (self.group0().xyzx * float4(other.group0().xx, other[scalar], other[e41])) - (self.group0().yzxy * float4(other.group3().zxy, other[e42])), 
            /* e41, e42, e43 */
            (other.group4().yzx * self.group0().zxy) - (float3(other[e4]) * self.group0().xyz) - (other.group4().zxy * self.group0().yzx), 
            /* e23, e31, e12 */
            (float3(other[e321]) * self.group0().xyz) + (other.group1().zxy * self.group0().yzx) - (float3(self[e321]) * other.group4().xyz) - (other.group1().yzx * self.group0().zxy), 
            /* e423, e431, e412, e321 */
            float4(other[e42] * self[e412] * -1.0, other[e43] * self[e423] * -1.0, other[e41] * self[e431] * -1.0, (other[e31] * self[e431]) + (other[e12] * self[e412])) + (float4(other[e1234]) * self.group0()) + (self.group0().yzxx * float4(other.group2().zxy, other[e23]))
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension Plane: AntiSandwich<Origin> {
    func anti_sandwich(self, other: Origin) -> Flector {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(other[e4], other[e4], other[e4], 0.0) * float4(self.group0().xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0), 
            /* e23, e31, e12, scalar */
            float4(float3(0.0), other[e4] * self[e321] * -1.0)
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension Plane: AntiSandwich<Plane> {
    func anti_sandwich(self, other: Plane) -> Flector {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(other[e412] * self[e431] * -1.0, other[e423] * self[e412] * -1.0, other[e431] * self[e423] * -1.0, (other[e431] * self[e431]) + (other[e412] * self[e412])) + (other.group0().yzxx * self.group0().zxyx), 
            /* e23, e31, e12, scalar */
            float4(((float3(other[e321]) * self.group0().xyz) - (float3(self[e321]) * other.group0().xyz)), 0.0)
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension Plane: AntiSandwich<Point> {
    func anti_sandwich(self, other: Point) -> Flector {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(float3(1.0), 0.0) * float4(other.group0().www, 0.0) * float4(self.group0().xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0), 
            /* e23, e31, e12, scalar */
            float4(self[e431] * other[e3], self[e412] * other[e1], self[e423] * other[e2], -(self[e431] * other[e2]) - (self[e412] * other[e3]) - (self[e321] * other[e4])) - (self.group0().zxyx * other.group0().yzxx)
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension Plane: AntiSandwich<Scalar> {
    func anti_sandwich(self, other: Scalar) -> Motor {
        return Point::from_groups(
            /* e1, e2, e3, e4 */
            float4(other[scalar], other[scalar], other[scalar], 0.0) * float4(self.group0().xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0)
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension Point {
    func div(_rhs: anti_sandwich) -> anti_sandwich_partial<Point> {
        anti_sandwich_partial(this)
    }
}
extension Point: AntiSandwich<AntiScalar> {
    func anti_sandwich(self, other: AntiScalar) -> Motor {
        return Point::from_groups(
            /* e1, e2, e3, e4 */
            float4(other[e1234]) * self.group0()
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension Point: AntiSandwich<DualNum> {
    func anti_sandwich(self, other: DualNum) -> Motor {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(other[e1234]) * self.group0(), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other[scalar] * self[e4] * -1.0)
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension Point: AntiSandwich<Flector> {
    func anti_sandwich(self, other: Flector) -> Flector {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(self[e4]) * float4(other.group1().xyz, other[e4]) * float4(-1.0), 
            /* e23, e31, e12, scalar */
            float4(-(other[e4] * self[e1]) - (other[e412] * self[e2]), -(other[e4] * self[e2]) - (other[e423] * self[e3]), -(other[e4] * self[e3]) - (other[e431] * self[e1]), (other[e412] * self[e3]) + (other[e321] * self[e4])) + (other.group1().yzxy * self.group0().zxyy) + (self.group0().wwwx * float4(other.group0().xyz, other[e423]))
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension Point: AntiSandwich<Horizon> {
    func anti_sandwich(self, other: Horizon) -> Horizon {
        return Scalar::from_groups(
            /* scalar */
            other[e321] * self[e4]
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension Point: AntiSandwich<Line> {
    func anti_sandwich(self, other: Line) -> Motor {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4((other.group0().zxy * self.group0().yzx), 0.0) - float4((float3(self[e4]) * other.group1()), 0.0) - float4((other.group0().yzx * self.group0().zxy), 0.0), 
            /* e423, e431, e412, e321 */
            float4(self[e4], self[e4], self[e4], 1.0) * float4(other.group0(), -(other[e41] * self[e1]) - (other[e42] * self[e2]) - (other[e43] * self[e3]))
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension Point: AntiSandwich<Motor> {
    func anti_sandwich(self, other: Motor) -> Motor {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(((float3(other[e1234]) * self.group0().xyz) + (other.group0().zxy * self.group0().yzx) - (float3(self[e4]) * other.group1().xyz) - (other.group0().yzx * self.group0().zxy)), other[e1234] * self[e4]), 
            /* e423, e431, e412, e321 */
            float4(self[e4], self[e4], self[e4], 1.0) * float4(other.group0().xyz, -(other[e41] * self[e1]) - (other[e42] * self[e2]) - (other[e43] * self[e3]) - (other[scalar] * self[e4]))
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension Point: AntiSandwich<MultiVector> {
    func anti_sandwich(self, other: MultiVector) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2((other[e423] * self[e1]) + (other[e431] * self[e2]) + (other[e412] * self[e3]) + (other[e321] * self[e4]), other[e4] * self[e4]) * float2(1.0, -1.0), 
            /* e1, e2, e3, e4 */
            float4(((float3(other[e1234]) * self.group0().xyz) + (other.group2().zxy * self.group0().yzx) - (float3(self[e4]) * other.group3()) - (other.group2().yzx * self.group0().zxy)), other[e1234] * self[e4]), 
            /* e41, e42, e43 */
            float3(self[e4]) * other.group4().xyz * float3(-1.0), 
            /* e23, e31, e12 */
            (float3(self[e4]) * other.group1().xyz) + (other.group4().yzx * self.group0().zxy) - (float3(other[e4]) * self.group0().xyz) - (other.group4().zxy * self.group0().yzx), 
            /* e423, e431, e412, e321 */
            float4(self[e4], self[e4], self[e4], 1.0) * float4(other.group2(), -(other[scalar] * self[e4]) - (other[e41] * self[e1]) - (other[e42] * self[e2]) - (other[e43] * self[e3]))
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension Point: AntiSandwich<Origin> {
    func anti_sandwich(self, other: Origin) -> Flector {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(float3(0.0), other[e4] * self[e4] * -1.0), 
            /* e23, e31, e12, scalar */
            float4(other[e4], other[e4], other[e4], 0.0) * float4(self.group0().xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0)
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension Point: AntiSandwich<Plane> {
    func anti_sandwich(self, other: Plane) -> Flector {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(float3(1.0), 0.0) * float4(self.group0().www, 0.0) * float4(other.group0().xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0), 
            /* e23, e31, e12, scalar */
            float4(other[e412] * self[e2] * -1.0, other[e423] * self[e3] * -1.0, other[e431] * self[e1] * -1.0, (other[e431] * self[e2]) + (other[e412] * self[e3]) + (other[e321] * self[e4])) + (other.group0().yzxx * self.group0().zxyx)
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension Point: AntiSandwich<Point> {
    func anti_sandwich(self, other: Point) -> Flector {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(float3(0.0), other[e4] * self[e4] * -1.0), 
            /* e23, e31, e12, scalar */
            float4(((float3(self[e4]) * other.group0().xyz) - (float3(other[e4]) * self.group0().xyz)), 0.0)
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension Point: AntiSandwich<Scalar> {
    func anti_sandwich(self, other: Scalar) -> Scalar {
        return Horizon::from_groups(
            /* e321 */
            self[e4] * other[scalar] * -1.0
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension Scalar {
    func div(_rhs: anti_sandwich) -> anti_sandwich_partial<Scalar> {
        anti_sandwich_partial(this)
    }
}
extension Scalar: AntiSandwich<Flector> {
    func anti_sandwich(self, other: Flector) -> Flector {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(self[scalar], self[scalar], self[scalar], 0.0) * float4(other.group1().xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other[e4] * self[scalar])
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension Scalar: AntiSandwich<Line> {
    func anti_sandwich(self, other: Line) -> Line {
        return Line::from_groups(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(self[scalar]) * other.group0()
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension Scalar: AntiSandwich<Motor> {
    func anti_sandwich(self, other: Motor) -> Motor {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(0.0), 
            /* e23, e31, e12, scalar */
            float4(self[scalar]) * other.group0()
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension Scalar: AntiSandwich<MultiVector> {
    func anti_sandwich(self, other: MultiVector) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(other[e1234] * self[scalar], 1.0) * float2(1.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(self[scalar], self[scalar], self[scalar], 0.0) * float4(other.group4().xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(self[scalar]) * other.group2(), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other[e4] * self[scalar])
        ).geometric_anti_product(self.anti_reverse());
    }
}
extension Scalar: AntiSandwich<Plane> {
    func anti_sandwich(self, other: Plane) -> Horizon {
        return Point::from_groups(
            /* e1, e2, e3, e4 */
            float4(self[scalar], self[scalar], self[scalar], 0.0) * float4(other.group0().xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        ).geometric_anti_product(self.anti_reverse());
    }
}
