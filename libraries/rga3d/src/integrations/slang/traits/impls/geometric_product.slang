extension AntiScalar {
    func div(_rhs: geometric_product) -> geometric_product_partial<AntiScalar> {
        geometric_product_partial(this)
    }
}
extension AntiScalar: GeometricProduct<DualNum> {
    func geometric_product(self, other: DualNum) -> AntiScalar {
        return AntiScalar::from_groups(
            /* e1234 */
            self[e1234] * other[scalar]
        );
    }
}
extension AntiScalar: GeometricProduct<Flector> {
    func geometric_product(self, other: Flector) -> Flector {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), self[e1234] * other[e321] * -1.0), 
            /* e423, e431, e412, e321 */
            float4(self[e1234], self[e1234], self[e1234], 0.0) * float4(other.group0().xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0)
        );
    }
}
extension AntiScalar: GeometricProduct<Horizon> {
    func geometric_product(self, other: Horizon) -> Origin {
        return Origin::from_groups(
            /* e4 */
            self[e1234] * other[e321] * -1.0
        );
    }
}
extension AntiScalar: GeometricProduct<Line> {
    func geometric_product(self, other: Line) -> Line {
        return Line::from_groups(
            /* e41, e42, e43 */
            float3(self[e1234]) * other.group1(), 
            /* e23, e31, e12 */
            float3(0.0)
        );
    }
}
extension AntiScalar: GeometricProduct<Motor> {
    func geometric_product(self, other: Motor) -> Motor {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(self[e1234]) * other.group1(), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
extension AntiScalar: GeometricProduct<MultiVector> {
    func geometric_product(self, other: MultiVector) -> MultiVector {
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
        );
    }
}
extension AntiScalar: GeometricProduct<Plane> {
    func geometric_product(self, other: Plane) -> Origin {
        return Origin::from_groups(
            /* e4 */
            self[e1234] * other[e321] * -1.0
        );
    }
}
extension AntiScalar: GeometricProduct<Point> {
    func geometric_product(self, other: Point) -> Plane {
        return Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4(self[e1234], self[e1234], self[e1234], 0.0) * float4(other.group0().xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0)
        );
    }
}
extension AntiScalar: GeometricProduct<Scalar> {
    func geometric_product(self, other: Scalar) -> AntiScalar {
        return AntiScalar::from_groups(
            /* e1234 */
            self[e1234] * other[scalar]
        );
    }
}
extension DualNum {
    func div(_rhs: geometric_product) -> geometric_product_partial<DualNum> {
        geometric_product_partial(this)
    }
}
extension DualNum: GeometricProduct<AntiScalar> {
    func geometric_product(self, other: AntiScalar) -> AntiScalar {
        return AntiScalar::from_groups(
            /* e1234 */
            other[e1234] * self[scalar]
        );
    }
}
extension DualNum: GeometricProduct<DualNum> {
    func geometric_product(self, other: DualNum) -> DualNum {
        return DualNum::from_groups(
            /* scalar, e1234 */
            float2(other[scalar] * self[scalar], (other[scalar] * self[e1234]) + (other[e1234] * self[scalar]))
        );
    }
}
extension DualNum: GeometricProduct<Flector> {
    func geometric_product(self, other: Flector) -> Flector {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(other[e1], other[e2], other[e3], 1.0) * float4(self.group0().xx, self[scalar], (self[scalar] * other[e4]) - (self[e1234] * other[e321])), 
            /* e423, e431, e412, e321 */
            float4(((float3(self[scalar]) * other.group1().xyz) - (float3(self[e1234]) * other.group0().xyz)), self[scalar] * other[e321])
        );
    }
}
extension DualNum: GeometricProduct<Horizon> {
    func geometric_product(self, other: Horizon) -> Flector {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), self[e1234] * other[e321] * -1.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), self[scalar] * other[e321])
        );
    }
}
extension DualNum: GeometricProduct<Line> {
    func geometric_product(self, other: Line) -> Line {
        return Line::from_groups(
            /* e41, e42, e43 */
            (float3(self[scalar]) * other.group0()) + (float3(self[e1234]) * other.group1()), 
            /* e23, e31, e12 */
            float3(self[scalar]) * other.group1()
        );
    }
}
extension DualNum: GeometricProduct<Motor> {
    func geometric_product(self, other: Motor) -> Motor {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            (float4(self[scalar]) * other.group0()) + (float4(self[e1234]) * other.group1()), 
            /* e23, e31, e12, scalar */
            float4(self[scalar]) * other.group1()
        );
    }
}
extension DualNum: GeometricProduct<MultiVector> {
    func geometric_product(self, other: MultiVector) -> MultiVector {
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
        );
    }
}
extension DualNum: GeometricProduct<Origin> {
    func geometric_product(self, other: Origin) -> Origin {
        return Origin::from_groups(
            /* e4 */
            self[scalar] * other[e4]
        );
    }
}
extension DualNum: GeometricProduct<Plane> {
    func geometric_product(self, other: Plane) -> Flector {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), self[e1234] * other[e321] * -1.0), 
            /* e423, e431, e412, e321 */
            float4(self[scalar]) * other.group0()
        );
    }
}
extension DualNum: GeometricProduct<Point> {
    func geometric_product(self, other: Point) -> Flector {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(self[scalar]) * other.group0(), 
            /* e423, e431, e412, e321 */
            float4(self.group0().yy, self[e1234], 0.0) * float4(float3(1.0), 0.0) * float4(other.group0().xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0)
        );
    }
}
extension DualNum: GeometricProduct<Scalar> {
    func geometric_product(self, other: Scalar) -> DualNum {
        return DualNum::from_groups(
            /* scalar, e1234 */
            float2(other[scalar]) * self.group0()
        );
    }
}
extension Flector {
    func div(_rhs: geometric_product) -> geometric_product_partial<Flector> {
        geometric_product_partial(this)
    }
}
extension Flector: GeometricProduct<AntiScalar> {
    func geometric_product(self, other: AntiScalar) -> Flector {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other[e1234] * self[e321]), 
            /* e423, e431, e412, e321 */
            float4(other[e1234], other[e1234], other[e1234], 0.0) * float4(self.group0().xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Flector: GeometricProduct<DualNum> {
    func geometric_product(self, other: DualNum) -> Flector {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(self[e1], self[e2], self[e3], 1.0) * float4(other.group0().xx, other[scalar], (other[scalar] * self[e4]) + (other[e1234] * self[e321])), 
            /* e423, e431, e412, e321 */
            float4(((float3(other[scalar]) * self.group1().xyz) + (float3(other[e1234]) * self.group0().xyz)), other[scalar] * self[e321])
        );
    }
}
extension Flector: GeometricProduct<Flector> {
    func geometric_product(self, other: Flector) -> Motor {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            (float4(other[e321]) * float4(self.group1().xyz, self[e4])) + (other.group1().zxyz * self.group0().yzxz) + (float4(self.group0().ww, self[e431], self[e1]) * float4(other.group0().xyx, other[e423])) + (float4(self.group1().zx, self[e4], self[e2]) * float4(other.group0().yzz, other[e431])) - (other.group0().zxyx * self.group1().yzxx) - (other.group0().wwwy * float4(self.group0().xyz, self[e431])) - (float4(self.group0().zx, self[e321], self[e321]) * float4(other.group1().yzz, other[e4])) - (float4(self.group1().ww, self[e2], self[e412]) * float4(other.group1().xyx, other[e3])), 
            /* e23, e31, e12, scalar */
            float4(-(other[e2] * self[e3]) - (other[e321] * self[e1]), -(other[e3] * self[e1]) - (other[e321] * self[e2]), -(other[e3] * self[e321]) - (other[e321] * self[e3]), (other[e2] * self[e2]) + (other[e3] * self[e3])) + (other.group0().zxyx * self.group0().yzxx) - (float4(self.group1().ww, self[e2], self[e321]) * float4(other.group0().xyx, other[e321]))
        );
    }
}
extension Flector: GeometricProduct<Horizon> {
    func geometric_product(self, other: Horizon) -> Motor {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(other[e321]) * float4(self.group1().xyz, self[e4]), 
            /* e23, e31, e12, scalar */
            float4(other[e321]) * float4(self.group0().xyz, self[e321]) * float4(-1.0)
        );
    }
}
extension Flector: GeometricProduct<Line> {
    func geometric_product(self, other: Line) -> Flector {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4((self[e3] * other[e31]) + (self[e321] * other[e23]), (self[e1] * other[e12]) + (self[e321] * other[e31]), (self[e2] * other[e23]) + (self[e321] * other[e12]), -(self[e2] * other[e42]) - (self[e3] * other[e43]) - (self[e423] * other[e23]) - (self[e431] * other[e31]) - (self[e412] * other[e12])) - (self.group0().yzxx * float4(other.group1().zxy, other[e41])), 
            /* e423, e431, e412, e321 */
            float4((self[e3] * other[e42]) + (self[e4] * other[e23]) + (self[e412] * other[e31]) + (self[e321] * other[e41]), (self[e1] * other[e43]) + (self[e4] * other[e31]) + (self[e423] * other[e12]) + (self[e321] * other[e42]), (self[e2] * other[e41]) + (self[e4] * other[e12]) + (self[e431] * other[e23]) + (self[e321] * other[e43]), self[e3] * other[e12] * -1.0) - (self.group0().yzxx * float4(other.group0().zxy, other[e23])) - float4((other.group1().zxy * self.group1().yzx), self[e2] * other[e31])
        );
    }
}
extension Flector: GeometricProduct<Motor> {
    func geometric_product(self, other: Motor) -> Flector {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(self[e321] * other[e23], self[e321] * other[e31], self[e321] * other[e12], -(self[e2] * other[e42]) - (self[e3] * other[e43]) - (self[e423] * other[e23]) - (self[e431] * other[e31]) - (self[e412] * other[e12])) + (self.group0().xxyw * other.group1().wzxw) + float4((self.group0().zyz * other.group1().yww), self[e321] * other[e1234]) - (self.group0().yzxx * float4(other.group1().zxy, other[e41])), 
            /* e423, e431, e412, e321 */
            float4((self[e3] * other[e42]) + (self[e4] * other[e23]) + (self[e423] * other[scalar]) + (self[e412] * other[e31]) + (self[e321] * other[e41]), (self[e2] * other[e1234]) + (self[e4] * other[e31]) + (self[e423] * other[e12]) + (self[e431] * other[scalar]) + (self[e321] * other[e42]), (self[e3] * other[e1234]) + (self[e4] * other[e12]) + (self[e431] * other[e23]) + (self[e412] * other[scalar]) + (self[e321] * other[e43]), self[e3] * other[e12] * -1.0) + float4((self.group0().xxy * other.group0().wzx), self[e321] * other[scalar]) - (self.group0().yzxx * float4(other.group0().zxy, other[e23])) - (other.group1().zxyy * float4(self.group1().yzx, self[e2]))
        );
    }
}
extension Flector: GeometricProduct<MultiVector> {
    func geometric_product(self, other: MultiVector) -> MultiVector {
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
        );
    }
}
extension Flector: GeometricProduct<Origin> {
    func geometric_product(self, other: Origin) -> Motor {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(other[e4]) * float4(self.group0().xyz, self[e321]) * float4(-1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
extension Flector: GeometricProduct<Plane> {
    func geometric_product(self, other: Plane) -> Motor {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(-(self[e3] * other[e431]) - (self[e321] * other[e423]), -(self[e1] * other[e412]) - (self[e321] * other[e431]), -(self[e2] * other[e423]) - (self[e321] * other[e412]), (self[e3] * other[e412]) + (self[e4] * other[e321])) + (self.group0().yzxx * other.group0().zxyx) + (other.group0().wwwy * float4(self.group1().xyz, self[e2])), 
            /* e23, e31, e12, scalar */
            float4(other[e321]) * float4(self.group0().xyz, self[e321]) * float4(-1.0)
        );
    }
}
extension Flector: GeometricProduct<Point> {
    func geometric_product(self, other: Point) -> Motor {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4((self[e4] * other[e1]) + (self[e412] * other[e2]), (self[e4] * other[e2]) + (self[e423] * other[e3]), (self[e4] * other[e3]) + (self[e431] * other[e1]), -(self[e412] * other[e3]) - (self[e321] * other[e4])) - (self.group1().yzxy * other.group0().zxyy) - (other.group0().wwwx * float4(self.group0().xyz, self[e423])), 
            /* e23, e31, e12, scalar */
            float4(-(self[e3] * other[e2]) - (self[e321] * other[e1]), -(self[e1] * other[e3]) - (self[e321] * other[e2]), -(self[e2] * other[e1]) - (self[e321] * other[e3]), (self[e2] * other[e2]) + (self[e3] * other[e3])) + (self.group0().yzxx * other.group0().zxyx)
        );
    }
}
extension Flector: GeometricProduct<Scalar> {
    func geometric_product(self, other: Scalar) -> Flector {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(other[scalar]) * self.group0(), 
            /* e423, e431, e412, e321 */
            float4(other[scalar]) * self.group1()
        );
    }
}
extension Horizon {
    func div(_rhs: geometric_product) -> geometric_product_partial<Horizon> {
        geometric_product_partial(this)
    }
}
extension Horizon: GeometricProduct<AntiScalar> {
    func geometric_product(self, other: AntiScalar) -> Origin {
        return Origin::from_groups(
            /* e4 */
            other[e1234] * self[e321]
        );
    }
}
extension Horizon: GeometricProduct<DualNum> {
    func geometric_product(self, other: DualNum) -> Flector {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other[e1234] * self[e321]), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), other[scalar] * self[e321])
        );
    }
}
extension Horizon: GeometricProduct<Flector> {
    func geometric_product(self, other: Flector) -> Motor {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(self[e321]) * float4(other.group1().xyz, other[e4]) * float4(-1.0), 
            /* e23, e31, e12, scalar */
            float4(self[e321]) * float4(other.group0().xyz, other[e321]) * float4(-1.0)
        );
    }
}
extension Horizon: GeometricProduct<Horizon> {
    func geometric_product(self, other: Horizon) -> Scalar {
        return Scalar::from_groups(
            /* scalar */
            other[e321] * self[e321] * -1.0
        );
    }
}
extension Horizon: GeometricProduct<Line> {
    func geometric_product(self, other: Line) -> Flector {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(self[e321], self[e321], self[e321], 0.0) * float4(other.group1(), 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e423, e431, e412, e321 */
            float4(self[e321], self[e321], self[e321], 0.0) * float4(other.group0(), 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Horizon: GeometricProduct<Motor> {
    func geometric_product(self, other: Motor) -> Flector {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(self[e321]) * float4(other.group1().xyz, other[e1234]), 
            /* e423, e431, e412, e321 */
            float4(self[e321]) * float4(other.group0().xyz, other[scalar])
        );
    }
}
extension Horizon: GeometricProduct<MultiVector> {
    func geometric_product(self, other: MultiVector) -> MultiVector {
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
        );
    }
}
extension Horizon: GeometricProduct<Origin> {
    func geometric_product(self, other: Origin) -> AntiScalar {
        return AntiScalar::from_groups(
            /* e1234 */
            self[e321] * other[e4] * -1.0
        );
    }
}
extension Horizon: GeometricProduct<Plane> {
    func geometric_product(self, other: Plane) -> Motor {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(self[e321], self[e321], self[e321], 0.0) * float4(other.group0().xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0), 
            /* e23, e31, e12, scalar */
            float4(float3(0.0), self[e321] * other[e321] * -1.0)
        );
    }
}
extension Horizon: GeometricProduct<Point> {
    func geometric_product(self, other: Point) -> Motor {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(float3(0.0), self[e321] * other[e4] * -1.0), 
            /* e23, e31, e12, scalar */
            float4(self[e321], self[e321], self[e321], 0.0) * float4(other.group0().xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0)
        );
    }
}
extension Horizon: GeometricProduct<Scalar> {
    func geometric_product(self, other: Scalar) -> Horizon {
        return Horizon::from_groups(
            /* e321 */
            self[e321] * other[scalar]
        );
    }
}
extension Line {
    func div(_rhs: geometric_product) -> geometric_product_partial<Line> {
        geometric_product_partial(this)
    }
}
extension Line: GeometricProduct<AntiScalar> {
    func geometric_product(self, other: AntiScalar) -> Line {
        return Line::from_groups(
            /* e41, e42, e43 */
            float3(other[e1234]) * self.group1(), 
            /* e23, e31, e12 */
            float3(0.0)
        );
    }
}
extension Line: GeometricProduct<DualNum> {
    func geometric_product(self, other: DualNum) -> Line {
        return Line::from_groups(
            /* e41, e42, e43 */
            (float3(other[scalar]) * self.group0()) + (float3(other[e1234]) * self.group1()), 
            /* e23, e31, e12 */
            float3(other[scalar]) * self.group1()
        );
    }
}
extension Line: GeometricProduct<Flector> {
    func geometric_product(self, other: Flector) -> Flector {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            (float4(other[e2], other[e321], other[e321], other[e2]) * float4(self.group1().zyz, self[e42])) + (float4(other[e321], other[e3], other[e1], other[e1]) * float4(self.group1().xxy, self[e41])) + float4(float3(0.0), (other[e3] * self[e43]) - (other[e431] * self[e31]) - (other[e412] * self[e12])) - float4((self.group1().yzx * other.group0().zxy), other[e423] * self[e23]), 
            /* e423, e431, e412, e321 */
            float4((other[e3] * self[e42]) + (other[e4] * self[e23]) + (other[e431] * self[e12]), (other[e1] * self[e43]) + (other[e4] * self[e31]) + (other[e412] * self[e23]), (other[e2] * self[e41]) + (other[e4] * self[e12]) + (other[e423] * self[e31]), 0.0) - (float4(other[e2], other[e321], other[e321], other[e2]) * float4(self.group0().zyz, self[e31])) - (float4(other[e321], other[e3], other[e1], other[e1]) * float4(self.group0().xxy, self[e23])) - float4((self.group1().yzx * other.group1().zxy), other[e3] * self[e12])
        );
    }
}
extension Line: GeometricProduct<Horizon> {
    func geometric_product(self, other: Horizon) -> Flector {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(other[e321], other[e321], other[e321], 0.0) * float4(self.group1(), 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e423, e431, e412, e321 */
            float4(other[e321], other[e321], other[e321], 0.0) * float4(self.group0(), 0.0) * float4(-1.0, -1.0, -1.0, 0.0)
        );
    }
}
extension Line: GeometricProduct<Line> {
    func geometric_product(self, other: Line) -> Motor {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4((other[e42] * self[e12]) + (other[e31] * self[e43]), (other[e43] * self[e23]) + (other[e12] * self[e41]), (other[e41] * self[e31]) + (other[e23] * self[e42]), -(other[e43] * self[e12]) - (other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43])) - float4((other.group0().zxy * self.group1().yzx), other[e41] * self[e23]) - float4((other.group1().zxy * self.group0().yzx), other[e42] * self[e31]), 
            /* e23, e31, e12, scalar */
            float4(other[e31] * self[e12], other[e12] * self[e23], other[e23] * self[e31], -(other[e31] * self[e31]) - (other[e12] * self[e12])) - float4((other.group1().zxy * self.group1().yzx), other[e23] * self[e23])
        );
    }
}
extension Line: GeometricProduct<Motor> {
    func geometric_product(self, other: Motor) -> Motor {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4((self[e41] * other[scalar]) + (self[e43] * other[e31]) + (self[e23] * other[e1234]) + (self[e12] * other[e42]), (self[e41] * other[e12]) + (self[e42] * other[scalar]) + (self[e23] * other[e43]) + (self[e31] * other[e1234]), (self[e42] * other[e23]) + (self[e43] * other[scalar]) + (self[e31] * other[e41]) + (self[e12] * other[e1234]), -(self[e43] * other[e12]) - (self[e23] * other[e41]) - (self[e31] * other[e42]) - (self[e12] * other[e43])) - (other.group1().zxyx * float4(self.group0().yzx, self[e41])) - float4((self.group1().yzx * other.group0().zxy), self[e42] * other[e31]), 
            /* e23, e31, e12, scalar */
            float4((self[e23] * other[scalar]) + (self[e12] * other[e31]), (self[e23] * other[e12]) + (self[e31] * other[scalar]), (self[e31] * other[e23]) + (self[e12] * other[scalar]), -(self[e31] * other[e31]) - (self[e12] * other[e12])) - (other.group1().zxyx * float4(self.group1().yzx, self[e23]))
        );
    }
}
extension Line: GeometricProduct<MultiVector> {
    func geometric_product(self, other: MultiVector) -> MultiVector {
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
        );
    }
}
extension Line: GeometricProduct<Origin> {
    func geometric_product(self, other: Origin) -> Plane {
        return Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4(other[e4], other[e4], other[e4], 0.0) * float4(self.group1(), 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Line: GeometricProduct<Plane> {
    func geometric_product(self, other: Plane) -> Flector {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(other[e321], other[e321], other[e321], 1.0) * float4(self.group1(), -(self[e23] * other[e423]) - (self[e31] * other[e431]) - (self[e12] * other[e412])), 
            /* e423, e431, e412, e321 */
            float4((self.group1().zxy * other.group0().yzx), 0.0) - float4((float3(other[e321]) * self.group0()), 0.0) - float4((self.group1().yzx * other.group0().zxy), 0.0)
        );
    }
}
extension Line: GeometricProduct<Point> {
    func geometric_product(self, other: Point) -> Flector {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(self[e31] * other[e3] * -1.0, self[e12] * other[e1] * -1.0, self[e23] * other[e2] * -1.0, (self[e42] * other[e2]) + (self[e43] * other[e3])) + (other.group0().yzxx * float4(self.group1().zxy, self[e41])), 
            /* e423, e431, e412, e321 */
            float4((self[e42] * other[e3]) + (self[e23] * other[e4]), (self[e43] * other[e1]) + (self[e31] * other[e4]), (self[e41] * other[e2]) + (self[e12] * other[e4]), -(self[e31] * other[e2]) - (self[e12] * other[e3])) - (other.group0().yzxx * float4(self.group0().zxy, self[e23]))
        );
    }
}
extension Line: GeometricProduct<Scalar> {
    func geometric_product(self, other: Scalar) -> Line {
        return Line::from_groups(
            /* e41, e42, e43 */
            float3(other[scalar]) * self.group0(), 
            /* e23, e31, e12 */
            float3(other[scalar]) * self.group1()
        );
    }
}
extension Motor {
    func div(_rhs: geometric_product) -> geometric_product_partial<Motor> {
        geometric_product_partial(this)
    }
}
extension Motor: GeometricProduct<AntiScalar> {
    func geometric_product(self, other: AntiScalar) -> Motor {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(other[e1234]) * self.group1(), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
extension Motor: GeometricProduct<DualNum> {
    func geometric_product(self, other: DualNum) -> Motor {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            (float4(other[scalar]) * self.group0()) + (float4(other[e1234]) * self.group1()), 
            /* e23, e31, e12, scalar */
            float4(other[scalar]) * self.group1()
        );
    }
}
extension Motor: GeometricProduct<Flector> {
    func geometric_product(self, other: Flector) -> Flector {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            (other.group0().xyxx * float4(self.group1().wwy, self[e41])) + (other.group0().yzzy * float4(self.group1().zxw, self[e42])) + float4(float3(0.0), (other[e4] * self[scalar]) - (other[e431] * self[e31]) - (other[e412] * self[e12]) - (other[e321] * self[e1234])) + float4((other.group1().www * self.group1().xyz), other[e3] * self[e43]) - (self.group1().yzxx * float4(other.group0().zxy, other[e423])), 
            /* e423, e431, e412, e321 */
            float4((other[e4] * self[e23]) + (other[e423] * self[scalar]) + (other[e431] * self[e12]) - (other[e321] * self[e41]), (other[e4] * self[e31]) + (other[e431] * self[scalar]) + (other[e412] * self[e23]) - (other[e321] * self[e42]), (other[e4] * self[e12]) + (other[e423] * self[e31]) + (other[e412] * self[scalar]) - (other[e321] * self[e43]), 0.0) + float4((other.group0().zxy * self.group0().yzx), other[e321] * self[scalar]) - (other.group0().xyxx * float4(self.group0().wwy, self[e23])) - (other.group0().yzzy * float4(self.group0().zxw, self[e31])) - (self.group1().yzxz * float4(other.group1().zxy, other[e3]))
        );
    }
}
extension Motor: GeometricProduct<Horizon> {
    func geometric_product(self, other: Horizon) -> Flector {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(other[e321]) * float4(self.group1().xyz, self[e1234]) * float4(1.0, 1.0, 1.0, -1.0), 
            /* e423, e431, e412, e321 */
            float4(other[e321]) * float4(self.group0().xyz, self[scalar]) * float4(-1.0, -1.0, -1.0, 1.0)
        );
    }
}
extension Motor: GeometricProduct<Line> {
    func geometric_product(self, other: Line) -> Motor {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4((other[e41] * self[scalar]) + (other[e42] * self[e12]) + (other[e23] * self[e1234]) + (other[e31] * self[e43]), (other[e42] * self[scalar]) + (other[e43] * self[e23]) + (other[e31] * self[e1234]) + (other[e12] * self[e41]), (other[e41] * self[e31]) + (other[e43] * self[scalar]) + (other[e23] * self[e42]) + (other[e12] * self[e1234]), -(other[e43] * self[e12]) - (other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43])) - (self.group1().yzxx * float4(other.group0().zxy, other[e41])) - float4((other.group1().zxy * self.group0().yzx), other[e42] * self[e31]), 
            /* e23, e31, e12, scalar */
            float4((other[e23] * self[scalar]) + (other[e31] * self[e12]), (other[e31] * self[scalar]) + (other[e12] * self[e23]), (other[e23] * self[e31]) + (other[e12] * self[scalar]), -(other[e31] * self[e31]) - (other[e12] * self[e12])) - (self.group1().yzxx * float4(other.group1().zxy, other[e23]))
        );
    }
}
extension Motor: GeometricProduct<Motor> {
    func geometric_product(self, other: Motor) -> Motor {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4((other[e1234] * self[e23]) + (other[e23] * self[e1234]) + (other[e31] * self[e43]) + (other[scalar] * self[e41]), (other[e1234] * self[e31]) + (other[e31] * self[e1234]) + (other[e12] * self[e41]) + (other[scalar] * self[e42]), (other[e1234] * self[e12]) + (other[e23] * self[e42]) + (other[e12] * self[e1234]) + (other[scalar] * self[e43]), -(other[e43] * self[e12]) - (other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43])) + (other.group0().xyxw * self.group1().wwyw) + float4((other.group0().yzz * self.group1().zxw), other[scalar] * self[e1234]) - (other.group0().zxyx * self.group1().yzxx) - float4((other.group1().zxy * self.group0().yzx), other[e42] * self[e31]), 
            /* e23, e31, e12, scalar */
            float4((other[e31] * self[e12]) + (other[scalar] * self[e23]), (other[e12] * self[e23]) + (other[scalar] * self[e31]), (other[e12] * self[scalar]) + (other[scalar] * self[e12]), -(other[e31] * self[e31]) - (other[e12] * self[e12])) + (other.group1().xyxw * self.group1().wwyw) - (other.group1().zxyx * self.group1().yzxx)
        );
    }
}
extension Motor: GeometricProduct<MultiVector> {
    func geometric_product(self, other: MultiVector) -> MultiVector {
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
        );
    }
}
extension Motor: GeometricProduct<Origin> {
    func geometric_product(self, other: Origin) -> Flector {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), self[scalar] * other[e4]), 
            /* e423, e431, e412, e321 */
            float4(other[e4], other[e4], other[e4], 0.0) * float4(self.group1().xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Motor: GeometricProduct<Plane> {
    func geometric_product(self, other: Plane) -> Flector {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(other[e321], other[e321], other[e321], 1.0) * float4(self.group1().xyz, -(self[e1234] * other[e321]) - (self[e23] * other[e423]) - (self[e31] * other[e431]) - (self[e12] * other[e412])), 
            /* e423, e431, e412, e321 */
            float4(((float3(self[scalar]) * other.group0().xyz) + (self.group1().zxy * other.group0().yzx) - (float3(other[e321]) * self.group0().xyz) - (self.group1().yzx * other.group0().zxy)), self[scalar] * other[e321])
        );
    }
}
extension Motor: GeometricProduct<Point> {
    func geometric_product(self, other: Point) -> Flector {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(self[e31] * other[e3] * -1.0, self[e12] * other[e1] * -1.0, self[e23] * other[e2] * -1.0, (self[e43] * other[e3]) + (self[scalar] * other[e4])) + (other.group0().xyzy * float4(self.group1().www, self[e42])) + (other.group0().yzxx * float4(self.group1().zxy, self[e41])), 
            /* e423, e431, e412, e321 */
            float4((self[e42] * other[e3]) + (self[e23] * other[e4]), (self[e43] * other[e1]) + (self[e31] * other[e4]), (self[e41] * other[e2]) + (self[e12] * other[e4]), self[e12] * other[e3] * -1.0) - (other.group0().xyzy * float4(self.group0().www, self[e31])) - (other.group0().yzxx * float4(self.group0().zxy, self[e23]))
        );
    }
}
extension Motor: GeometricProduct<Scalar> {
    func geometric_product(self, other: Scalar) -> Motor {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(other[scalar]) * self.group0(), 
            /* e23, e31, e12, scalar */
            float4(other[scalar]) * self.group1()
        );
    }
}
extension MultiVector {
    func div(_rhs: geometric_product) -> geometric_product_partial<MultiVector> {
        geometric_product_partial(this)
    }
}
extension MultiVector: GeometricProduct<AntiScalar> {
    func geometric_product(self, other: AntiScalar) -> MultiVector {
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
        );
    }
}
extension MultiVector: GeometricProduct<DualNum> {
    func geometric_product(self, other: DualNum) -> MultiVector {
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
        );
    }
}
extension MultiVector: GeometricProduct<Flector> {
    func geometric_product(self, other: Flector) -> MultiVector {
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
        );
    }
}
extension MultiVector: GeometricProduct<Horizon> {
    func geometric_product(self, other: Horizon) -> MultiVector {
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
        );
    }
}
extension MultiVector: GeometricProduct<Line> {
    func geometric_product(self, other: Line) -> MultiVector {
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
        );
    }
}
extension MultiVector: GeometricProduct<Motor> {
    func geometric_product(self, other: Motor) -> MultiVector {
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
        );
    }
}
extension MultiVector: GeometricProduct<MultiVector> {
    func geometric_product(self, other: MultiVector) -> MultiVector {
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
        );
    }
}
extension MultiVector: GeometricProduct<Origin> {
    func geometric_product(self, other: Origin) -> MultiVector {
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
extension MultiVector: GeometricProduct<Plane> {
    func geometric_product(self, other: Plane) -> MultiVector {
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
        );
    }
}
extension MultiVector: GeometricProduct<Point> {
    func geometric_product(self, other: Point) -> MultiVector {
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
        );
    }
}
extension MultiVector: GeometricProduct<Scalar> {
    func geometric_product(self, other: Scalar) -> MultiVector {
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
    func div(_rhs: geometric_product) -> geometric_product_partial<Origin> {
        geometric_product_partial(this)
    }
}
extension Origin: GeometricProduct<DualNum> {
    func geometric_product(self, other: DualNum) -> Origin {
        return Origin::from_groups(
            /* e4 */
            other[scalar] * self[e4]
        );
    }
}
extension Origin: GeometricProduct<Flector> {
    func geometric_product(self, other: Flector) -> Motor {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(self[e4]) * float4(other.group0().xyz, other[e321]), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
extension Origin: GeometricProduct<Horizon> {
    func geometric_product(self, other: Horizon) -> AntiScalar {
        return AntiScalar::from_groups(
            /* e1234 */
            other[e321] * self[e4]
        );
    }
}
extension Origin: GeometricProduct<Line> {
    func geometric_product(self, other: Line) -> Plane {
        return Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4(self[e4], self[e4], self[e4], 0.0) * float4(other.group1(), 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Origin: GeometricProduct<Motor> {
    func geometric_product(self, other: Motor) -> Flector {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other[scalar] * self[e4]), 
            /* e423, e431, e412, e321 */
            float4(self[e4], self[e4], self[e4], 0.0) * float4(other.group1().xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Origin: GeometricProduct<MultiVector> {
    func geometric_product(self, other: MultiVector) -> MultiVector {
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
extension Origin: GeometricProduct<Plane> {
    func geometric_product(self, other: Plane) -> AntiScalar {
        return AntiScalar::from_groups(
            /* e1234 */
            self[e4] * other[e321]
        );
    }
}
extension Origin: GeometricProduct<Point> {
    func geometric_product(self, other: Point) -> Line {
        return Line::from_groups(
            /* e41, e42, e43 */
            float3(self[e4]) * other.group0().xyz, 
            /* e23, e31, e12 */
            float3(0.0)
        );
    }
}
extension Origin: GeometricProduct<Scalar> {
    func geometric_product(self, other: Scalar) -> Origin {
        return Origin::from_groups(
            /* e4 */
            self[e4] * other[scalar]
        );
    }
}
extension Plane {
    func div(_rhs: geometric_product) -> geometric_product_partial<Plane> {
        geometric_product_partial(this)
    }
}
extension Plane: GeometricProduct<AntiScalar> {
    func geometric_product(self, other: AntiScalar) -> Origin {
        return Origin::from_groups(
            /* e4 */
            other[e1234] * self[e321]
        );
    }
}
extension Plane: GeometricProduct<DualNum> {
    func geometric_product(self, other: DualNum) -> Flector {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), other[e1234] * self[e321]), 
            /* e423, e431, e412, e321 */
            float4(other[scalar]) * self.group0()
        );
    }
}
extension Plane: GeometricProduct<Flector> {
    func geometric_product(self, other: Flector) -> Motor {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4((other[e2] * self[e412]) + (other[e321] * self[e423]), (other[e3] * self[e423]) + (other[e321] * self[e431]), (other[e1] * self[e431]) + (other[e321] * self[e412]), -(other[e3] * self[e412]) - (other[e4] * self[e321])) - (other.group0().zxyx * self.group0().yzxx) - (self.group0().wwwy * float4(other.group1().xyz, other[e2])), 
            /* e23, e31, e12, scalar */
            float4(self[e321]) * float4(other.group0().xyz, other[e321]) * float4(-1.0)
        );
    }
}
extension Plane: GeometricProduct<Horizon> {
    func geometric_product(self, other: Horizon) -> Motor {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(other[e321], other[e321], other[e321], 0.0) * float4(self.group0().xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0), 
            /* e23, e31, e12, scalar */
            float4(float3(0.0), other[e321] * self[e321] * -1.0)
        );
    }
}
extension Plane: GeometricProduct<Line> {
    func geometric_product(self, other: Line) -> Flector {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(self[e321], self[e321], self[e321], 1.0) * float4(other.group1(), -(other[e23] * self[e423]) - (other[e31] * self[e431]) - (other[e12] * self[e412])), 
            /* e423, e431, e412, e321 */
            float4((float3(self[e321]) * other.group0()), 0.0) + float4((other.group1().yzx * self.group0().zxy), 0.0) - float4((other.group1().zxy * self.group0().yzx), 0.0)
        );
    }
}
extension Plane: GeometricProduct<Motor> {
    func geometric_product(self, other: Motor) -> Flector {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(self[e321], self[e321], self[e321], 1.0) * float4(other.group1().xyz, (other[e1234] * self[e321]) - (other[e23] * self[e423]) - (other[e31] * self[e431]) - (other[e12] * self[e412])), 
            /* e423, e431, e412, e321 */
            float4(((float3(other[scalar]) * self.group0().xyz) + (float3(self[e321]) * other.group0().xyz) + (other.group1().yzx * self.group0().zxy) - (other.group1().zxy * self.group0().yzx)), other[scalar] * self[e321])
        );
    }
}
extension Plane: GeometricProduct<MultiVector> {
    func geometric_product(self, other: MultiVector) -> MultiVector {
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
        );
    }
}
extension Plane: GeometricProduct<Origin> {
    func geometric_product(self, other: Origin) -> AntiScalar {
        return AntiScalar::from_groups(
            /* e1234 */
            other[e4] * self[e321] * -1.0
        );
    }
}
extension Plane: GeometricProduct<Plane> {
    func geometric_product(self, other: Plane) -> Motor {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(((float3(other[e321]) * self.group0().xyz) - (float3(self[e321]) * other.group0().xyz)), 0.0), 
            /* e23, e31, e12, scalar */
            float4(float3(0.0), other[e321] * self[e321] * -1.0)
        );
    }
}
extension Plane: GeometricProduct<Point> {
    func geometric_product(self, other: Point) -> Motor {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(self[e412] * other[e2], self[e423] * other[e3], self[e431] * other[e1], -(self[e431] * other[e2]) - (self[e412] * other[e3]) - (self[e321] * other[e4])) - (self.group0().yzxx * other.group0().zxyx), 
            /* e23, e31, e12, scalar */
            float4(float3(1.0), 0.0) * float4(self.group0().www, 0.0) * float4(other.group0().xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0)
        );
    }
}
extension Plane: GeometricProduct<Scalar> {
    func geometric_product(self, other: Scalar) -> Plane {
        return Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4(other[scalar]) * self.group0()
        );
    }
}
extension Point {
    func div(_rhs: geometric_product) -> geometric_product_partial<Point> {
        geometric_product_partial(this)
    }
}
extension Point: GeometricProduct<AntiScalar> {
    func geometric_product(self, other: AntiScalar) -> Plane {
        return Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4(other[e1234], other[e1234], other[e1234], 0.0) * float4(self.group0().xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Point: GeometricProduct<DualNum> {
    func geometric_product(self, other: DualNum) -> Flector {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(other[scalar]) * self.group0(), 
            /* e423, e431, e412, e321 */
            float4(other.group0().yy, other[e1234], 0.0) * float4(float3(1.0), 0.0) * float4(self.group0().xyz, 0.0) * float4(1.0, 1.0, 1.0, 0.0)
        );
    }
}
extension Point: GeometricProduct<Flector> {
    func geometric_product(self, other: Flector) -> Motor {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(-(other[e4] * self[e1]) - (other[e431] * self[e3]), -(other[e4] * self[e2]) - (other[e412] * self[e1]), -(other[e4] * self[e3]) - (other[e423] * self[e2]), (other[e412] * self[e3]) + (other[e321] * self[e4])) + (other.group1().zxyy * self.group0().yzxy) + (self.group0().wwwx * float4(other.group0().xyz, other[e423])), 
            /* e23, e31, e12, scalar */
            float4(-(other[e2] * self[e3]) - (other[e321] * self[e1]), -(other[e3] * self[e1]) - (other[e321] * self[e2]), -(other[e1] * self[e2]) - (other[e321] * self[e3]), (other[e2] * self[e2]) + (other[e3] * self[e3])) + (other.group0().zxyx * self.group0().yzxx)
        );
    }
}
extension Point: GeometricProduct<Horizon> {
    func geometric_product(self, other: Horizon) -> Motor {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(float3(0.0), other[e321] * self[e4]), 
            /* e23, e31, e12, scalar */
            float4(other[e321], other[e321], other[e321], 0.0) * float4(self.group0().xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0)
        );
    }
}
extension Point: GeometricProduct<Line> {
    func geometric_product(self, other: Line) -> Flector {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(other[e31] * self[e3], other[e12] * self[e1], other[e23] * self[e2], -(other[e42] * self[e2]) - (other[e43] * self[e3])) - (self.group0().yzxx * float4(other.group1().zxy, other[e41])), 
            /* e423, e431, e412, e321 */
            float4((other[e42] * self[e3]) + (other[e23] * self[e4]), (other[e43] * self[e1]) + (other[e31] * self[e4]), (other[e41] * self[e2]) + (other[e12] * self[e4]), -(other[e31] * self[e2]) - (other[e12] * self[e3])) - (self.group0().yzxx * float4(other.group0().zxy, other[e23]))
        );
    }
}
extension Point: GeometricProduct<Motor> {
    func geometric_product(self, other: Motor) -> Flector {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(other[scalar] * self[e1], other[scalar] * self[e2], other[scalar] * self[e3], -(other[e42] * self[e2]) - (other[e43] * self[e3])) + (other.group1().yzxw * self.group0().zxyw) - (self.group0().yzxx * float4(other.group1().zxy, other[e41])), 
            /* e423, e431, e412, e321 */
            float4((other[e42] * self[e3]) + (other[e1234] * self[e1]) + (other[e23] * self[e4]), (other[e43] * self[e1]) + (other[e1234] * self[e2]) + (other[e31] * self[e4]), (other[e41] * self[e2]) + (other[e1234] * self[e3]) + (other[e12] * self[e4]), -(other[e31] * self[e2]) - (other[e12] * self[e3])) - (self.group0().yzxx * float4(other.group0().zxy, other[e23]))
        );
    }
}
extension Point: GeometricProduct<MultiVector> {
    func geometric_product(self, other: MultiVector) -> MultiVector {
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
        );
    }
}
extension Point: GeometricProduct<Origin> {
    func geometric_product(self, other: Origin) -> Line {
        return Line::from_groups(
            /* e41, e42, e43 */
            float3(other[e4]) * self.group0().xyz * float3(-1.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
    }
}
extension Point: GeometricProduct<Plane> {
    func geometric_product(self, other: Plane) -> Motor {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(other[e431] * self[e3] * -1.0, other[e412] * self[e1] * -1.0, other[e423] * self[e2] * -1.0, (other[e431] * self[e2]) + (other[e412] * self[e3]) + (other[e321] * self[e4])) + (other.group0().zxyx * self.group0().yzxx), 
            /* e23, e31, e12, scalar */
            float4(float3(1.0), 0.0) * float4(other.group0().www, 0.0) * float4(self.group0().xyz, 0.0) * float4(-1.0, -1.0, -1.0, 0.0)
        );
    }
}
extension Point: GeometricProduct<Point> {
    func geometric_product(self, other: Point) -> Motor {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(((float3(self[e4]) * other.group0().xyz) - (float3(other[e4]) * self.group0().xyz)), 0.0), 
            /* e23, e31, e12, scalar */
            float4(other[e2] * self[e3] * -1.0, other[e3] * self[e1] * -1.0, other[e1] * self[e2] * -1.0, (other[e2] * self[e2]) + (other[e3] * self[e3])) + (other.group0().zxyx * self.group0().yzxx)
        );
    }
}
extension Point: GeometricProduct<Scalar> {
    func geometric_product(self, other: Scalar) -> Point {
        return Point::from_groups(
            /* e1, e2, e3, e4 */
            float4(other[scalar]) * self.group0()
        );
    }
}
extension Scalar {
    func div(_rhs: geometric_product) -> geometric_product_partial<Scalar> {
        geometric_product_partial(this)
    }
}
extension Scalar: GeometricProduct<AntiScalar> {
    func geometric_product(self, other: AntiScalar) -> AntiScalar {
        return AntiScalar::from_groups(
            /* e1234 */
            other[e1234] * self[scalar]
        );
    }
}
extension Scalar: GeometricProduct<DualNum> {
    func geometric_product(self, other: DualNum) -> DualNum {
        return DualNum::from_groups(
            /* scalar, e1234 */
            float2(self[scalar]) * other.group0()
        );
    }
}
extension Scalar: GeometricProduct<Flector> {
    func geometric_product(self, other: Flector) -> Flector {
        return Flector::from_groups(
            /* e1, e2, e3, e4 */
            float4(self[scalar]) * other.group0(), 
            /* e423, e431, e412, e321 */
            float4(self[scalar]) * other.group1()
        );
    }
}
extension Scalar: GeometricProduct<Horizon> {
    func geometric_product(self, other: Horizon) -> Horizon {
        return Horizon::from_groups(
            /* e321 */
            other[e321] * self[scalar]
        );
    }
}
extension Scalar: GeometricProduct<Line> {
    func geometric_product(self, other: Line) -> Line {
        return Line::from_groups(
            /* e41, e42, e43 */
            float3(self[scalar]) * other.group0(), 
            /* e23, e31, e12 */
            float3(self[scalar]) * other.group1()
        );
    }
}
extension Scalar: GeometricProduct<Motor> {
    func geometric_product(self, other: Motor) -> Motor {
        return Motor::from_groups(
            /* e41, e42, e43, e1234 */
            float4(self[scalar]) * other.group0(), 
            /* e23, e31, e12, scalar */
            float4(self[scalar]) * other.group1()
        );
    }
}
extension Scalar: GeometricProduct<MultiVector> {
    func geometric_product(self, other: MultiVector) -> MultiVector {
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
extension Scalar: GeometricProduct<Origin> {
    func geometric_product(self, other: Origin) -> Origin {
        return Origin::from_groups(
            /* e4 */
            other[e4] * self[scalar]
        );
    }
}
extension Scalar: GeometricProduct<Plane> {
    func geometric_product(self, other: Plane) -> Plane {
        return Plane::from_groups(
            /* e423, e431, e412, e321 */
            float4(self[scalar]) * other.group0()
        );
    }
}
extension Scalar: GeometricProduct<Point> {
    func geometric_product(self, other: Point) -> Point {
        return Point::from_groups(
            /* e1, e2, e3, e4 */
            float4(self[scalar]) * other.group0()
        );
    }
}
extension Scalar: GeometricProduct<Scalar> {
    func geometric_product(self, other: Scalar) -> Scalar {
        return Scalar::from_groups(
            /* scalar */
            other[scalar] * self[scalar]
        );
    }
}
