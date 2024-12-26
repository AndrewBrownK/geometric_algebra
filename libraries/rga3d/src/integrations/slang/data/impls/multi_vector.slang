using traits::GeometricProduct;
using traits::RightDual;
extension MultiVector: Add<AntiScalar> {
    func operator+(self, other: AntiScalar) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(0.0, other[e1234]) + self.group0(), 
            /* e1, e2, e3, e4 */
            self.group1(), 
            /* e41, e42, e43 */
            self.group2(), 
            /* e23, e31, e12 */
            self.group3(), 
            /* e423, e431, e412, e321 */
            self.group4()
        );
    }
}
extension MultiVector {
    func operator+=(const AntiScalar& other) {
        *this = MultiVector::from_groups(
            /* scalar, e1234 */
            float2(0.0, other[e1234]) + self.group0(), 
            /* e1, e2, e3, e4 */
            self.group1(), 
            /* e41, e42, e43 */
            self.group2(), 
            /* e23, e31, e12 */
            self.group3(), 
            /* e423, e431, e412, e321 */
            self.group4()
        );
    }
}
extension MultiVector: Add<DualNum> {
    func operator+(self, other: DualNum) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            other.group0() + self.group0(), 
            /* e1, e2, e3, e4 */
            self.group1(), 
            /* e41, e42, e43 */
            self.group2(), 
            /* e23, e31, e12 */
            self.group3(), 
            /* e423, e431, e412, e321 */
            self.group4()
        );
    }
}
extension MultiVector {
    func operator+=(const DualNum& other) {
        *this = MultiVector::from_groups(
            /* scalar, e1234 */
            other.group0() + self.group0(), 
            /* e1, e2, e3, e4 */
            self.group1(), 
            /* e41, e42, e43 */
            self.group2(), 
            /* e23, e31, e12 */
            self.group3(), 
            /* e423, e431, e412, e321 */
            self.group4()
        );
    }
}
extension MultiVector: Add<Flector> {
    func operator+(self, other: Flector) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            self.group0(), 
            /* e1, e2, e3, e4 */
            other.group0() + self.group1(), 
            /* e41, e42, e43 */
            self.group2(), 
            /* e23, e31, e12 */
            self.group3(), 
            /* e423, e431, e412, e321 */
            other.group1() + self.group4()
        );
    }
}
extension MultiVector {
    func operator+=(const Flector& other) {
        *this = MultiVector::from_groups(
            /* scalar, e1234 */
            self.group0(), 
            /* e1, e2, e3, e4 */
            other.group0() + self.group1(), 
            /* e41, e42, e43 */
            self.group2(), 
            /* e23, e31, e12 */
            self.group3(), 
            /* e423, e431, e412, e321 */
            other.group1() + self.group4()
        );
    }
}
extension MultiVector: Add<Horizon> {
    func operator+(self, other: Horizon) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            self.group0(), 
            /* e1, e2, e3, e4 */
            self.group1(), 
            /* e41, e42, e43 */
            self.group2(), 
            /* e23, e31, e12 */
            self.group3(), 
            /* e423, e431, e412, e321 */
            self.group4() + float4(float3(0.0), other[e321])
        );
    }
}
extension MultiVector {
    func operator+=(const Horizon& other) {
        *this = MultiVector::from_groups(
            /* scalar, e1234 */
            self.group0(), 
            /* e1, e2, e3, e4 */
            self.group1(), 
            /* e41, e42, e43 */
            self.group2(), 
            /* e23, e31, e12 */
            self.group3(), 
            /* e423, e431, e412, e321 */
            self.group4() + float4(float3(0.0), other[e321])
        );
    }
}
extension MultiVector: Add<Line> {
    func operator+(self, other: Line) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            self.group0(), 
            /* e1, e2, e3, e4 */
            self.group1(), 
            /* e41, e42, e43 */
            other.group0() + self.group2(), 
            /* e23, e31, e12 */
            other.group1() + self.group3(), 
            /* e423, e431, e412, e321 */
            self.group4()
        );
    }
}
extension MultiVector {
    func operator+=(const Line& other) {
        *this = MultiVector::from_groups(
            /* scalar, e1234 */
            self.group0(), 
            /* e1, e2, e3, e4 */
            self.group1(), 
            /* e41, e42, e43 */
            other.group0() + self.group2(), 
            /* e23, e31, e12 */
            other.group1() + self.group3(), 
            /* e423, e431, e412, e321 */
            self.group4()
        );
    }
}
extension MultiVector: Add<Motor> {
    func operator+(self, other: Motor) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(other[scalar], other[e1234]) + self.group0(), 
            /* e1, e2, e3, e4 */
            self.group1(), 
            /* e41, e42, e43 */
            self.group2() + other.group0().xyz, 
            /* e23, e31, e12 */
            self.group3() + other.group1().xyz, 
            /* e423, e431, e412, e321 */
            self.group4()
        );
    }
}
extension MultiVector {
    func operator+=(const Motor& other) {
        *this = MultiVector::from_groups(
            /* scalar, e1234 */
            float2(other[scalar], other[e1234]) + self.group0(), 
            /* e1, e2, e3, e4 */
            self.group1(), 
            /* e41, e42, e43 */
            self.group2() + other.group0().xyz, 
            /* e23, e31, e12 */
            self.group3() + other.group1().xyz, 
            /* e423, e431, e412, e321 */
            self.group4()
        );
    }
}
extension MultiVector: Add<MultiVector> {
    func operator+(self, other: MultiVector) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            other.group0() + self.group0(), 
            /* e1, e2, e3, e4 */
            other.group1() + self.group1(), 
            /* e41, e42, e43 */
            other.group2() + self.group2(), 
            /* e23, e31, e12 */
            other.group3() + self.group3(), 
            /* e423, e431, e412, e321 */
            other.group4() + self.group4()
        );
    }
}
extension MultiVector {
    func operator+=(const MultiVector& other) {
        *this = MultiVector::from_groups(
            /* scalar, e1234 */
            other.group0() + self.group0(), 
            /* e1, e2, e3, e4 */
            other.group1() + self.group1(), 
            /* e41, e42, e43 */
            other.group2() + self.group2(), 
            /* e23, e31, e12 */
            other.group3() + self.group3(), 
            /* e423, e431, e412, e321 */
            other.group4() + self.group4()
        );
    }
}
extension MultiVector: Add<Origin> {
    func operator+(self, other: Origin) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            self.group0(), 
            /* e1, e2, e3, e4 */
            self.group1() + float4(float3(0.0), other[e4]), 
            /* e41, e42, e43 */
            self.group2(), 
            /* e23, e31, e12 */
            self.group3(), 
            /* e423, e431, e412, e321 */
            self.group4()
        );
    }
}
extension MultiVector {
    func operator+=(const Origin& other) {
        *this = MultiVector::from_groups(
            /* scalar, e1234 */
            self.group0(), 
            /* e1, e2, e3, e4 */
            self.group1() + float4(float3(0.0), other[e4]), 
            /* e41, e42, e43 */
            self.group2(), 
            /* e23, e31, e12 */
            self.group3(), 
            /* e423, e431, e412, e321 */
            self.group4()
        );
    }
}
extension MultiVector: Add<Plane> {
    func operator+(self, other: Plane) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            self.group0(), 
            /* e1, e2, e3, e4 */
            self.group1(), 
            /* e41, e42, e43 */
            self.group2(), 
            /* e23, e31, e12 */
            self.group3(), 
            /* e423, e431, e412, e321 */
            self.group4() + other.group0()
        );
    }
}
extension MultiVector {
    func operator+=(const Plane& other) {
        *this = MultiVector::from_groups(
            /* scalar, e1234 */
            self.group0(), 
            /* e1, e2, e3, e4 */
            self.group1(), 
            /* e41, e42, e43 */
            self.group2(), 
            /* e23, e31, e12 */
            self.group3(), 
            /* e423, e431, e412, e321 */
            self.group4() + other.group0()
        );
    }
}
extension MultiVector: Add<Point> {
    func operator+(self, other: Point) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            self.group0(), 
            /* e1, e2, e3, e4 */
            self.group1() + other.group0(), 
            /* e41, e42, e43 */
            self.group2(), 
            /* e23, e31, e12 */
            self.group3(), 
            /* e423, e431, e412, e321 */
            self.group4()
        );
    }
}
extension MultiVector {
    func operator+=(const Point& other) {
        *this = MultiVector::from_groups(
            /* scalar, e1234 */
            self.group0(), 
            /* e1, e2, e3, e4 */
            self.group1() + other.group0(), 
            /* e41, e42, e43 */
            self.group2(), 
            /* e23, e31, e12 */
            self.group3(), 
            /* e423, e431, e412, e321 */
            self.group4()
        );
    }
}
extension MultiVector: Add<Scalar> {
    func operator+(self, other: Scalar) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(other[scalar], 0.0) + self.group0(), 
            /* e1, e2, e3, e4 */
            self.group1(), 
            /* e41, e42, e43 */
            self.group2(), 
            /* e23, e31, e12 */
            self.group3(), 
            /* e423, e431, e412, e321 */
            self.group4()
        );
    }
}
extension MultiVector {
    func operator+=(const Scalar& other) {
        *this = MultiVector::from_groups(
            /* scalar, e1234 */
            float2(other[scalar], 0.0) + self.group0(), 
            /* e1, e2, e3, e4 */
            self.group1(), 
            /* e41, e42, e43 */
            self.group2(), 
            /* e23, e31, e12 */
            self.group3(), 
            /* e423, e431, e412, e321 */
            self.group4()
        );
    }
}

impl From<AntiScalar> for MultiVector {
    fn from(from_anti_scalar: AntiScalar) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(0.0, from_anti_scalar[e1234]), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}

impl From<DualNum> for MultiVector {
    fn from(from_dual_num: DualNum) -> Self {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            from_dual_num.group0(), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}

impl From<Flector> for MultiVector {
    fn from(from_flector: Flector) -> Self {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            from_flector.group0(), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            from_flector.group1()
        );
    }
}

impl From<Horizon> for MultiVector {
    fn from(from_horizon: Horizon) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), from_horizon[e321])
        );
    }
}

impl From<Line> for MultiVector {
    fn from(from_line: Line) -> Self {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            from_line.group0(), 
            /* e23, e31, e12 */
            from_line.group1(), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}

impl From<Motor> for MultiVector {
    fn from(from_motor: Motor) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(from_motor[scalar], from_motor[e1234]), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            from_motor.group0().xyz, 
            /* e23, e31, e12 */
            from_motor.group1().xyz, 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}

impl From<Origin> for MultiVector {
    fn from(from_origin: Origin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), from_origin[e4]), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}

impl From<Plane> for MultiVector {
    fn from(from_plane: Plane) -> Self {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            from_plane.group0()
        );
    }
}

impl From<Point> for MultiVector {
    fn from(from_point: Point) -> Self {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            from_point.group0(), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}

impl From<Scalar> for MultiVector {
    fn from(from_scalar: Scalar) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(from_scalar[scalar], 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
extension MultiVector: Mul<AntiScalar> {
    func operator*(self, other: AntiScalar) -> MultiVector {
        return self.geometric_product(other);
    }
}
extension MultiVector {
    func operator*=(const AntiScalar& other) {
        *this = self.geometric_product(other);
    }
}
extension MultiVector: Mul<DualNum> {
    func operator*(self, other: DualNum) -> MultiVector {
        return self.geometric_product(other);
    }
}
extension MultiVector {
    func operator*=(const DualNum& other) {
        *this = self.geometric_product(other);
    }
}
extension MultiVector: Mul<Flector> {
    func operator*(self, other: Flector) -> MultiVector {
        return self.geometric_product(other);
    }
}
extension MultiVector {
    func operator*=(const Flector& other) {
        *this = self.geometric_product(other);
    }
}
extension MultiVector: Mul<Horizon> {
    func operator*(self, other: Horizon) -> MultiVector {
        return self.geometric_product(other);
    }
}
extension MultiVector {
    func operator*=(const Horizon& other) {
        *this = self.geometric_product(other);
    }
}
extension MultiVector: Mul<Line> {
    func operator*(self, other: Line) -> MultiVector {
        return self.geometric_product(other);
    }
}
extension MultiVector {
    func operator*=(const Line& other) {
        *this = self.geometric_product(other);
    }
}
extension MultiVector: Mul<Motor> {
    func operator*(self, other: Motor) -> MultiVector {
        return self.geometric_product(other);
    }
}
extension MultiVector {
    func operator*=(const Motor& other) {
        *this = self.geometric_product(other);
    }
}
extension MultiVector: Mul<MultiVector> {
    func operator*(self, other: MultiVector) -> MultiVector {
        return self.geometric_product(other);
    }
}
extension MultiVector {
    func operator*=(const MultiVector& other) {
        *this = self.geometric_product(other);
    }
}
extension MultiVector: Mul<Origin> {
    func operator*(self, other: Origin) -> MultiVector {
        return self.geometric_product(other);
    }
}
extension MultiVector {
    func operator*=(const Origin& other) {
        *this = self.geometric_product(other);
    }
}
extension MultiVector: Mul<Plane> {
    func operator*(self, other: Plane) -> MultiVector {
        return self.geometric_product(other);
    }
}
extension MultiVector {
    func operator*=(const Plane& other) {
        *this = self.geometric_product(other);
    }
}
extension MultiVector: Mul<Point> {
    func operator*(self, other: Point) -> MultiVector {
        return self.geometric_product(other);
    }
}
extension MultiVector {
    func operator*=(const Point& other) {
        *this = self.geometric_product(other);
    }
}
extension MultiVector: Mul<Scalar> {
    func operator*(self, other: Scalar) -> MultiVector {
        return self.geometric_product(other);
    }
}
extension MultiVector {
    func operator*=(const Scalar& other) {
        *this = self.geometric_product(other);
    }
}
extension MultiVector: Neg {
    func neg(self) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            self.group0() * float2(-1.0), 
            /* e1, e2, e3, e4 */
            self.group1() * float4(-1.0), 
            /* e41, e42, e43 */
            self.group2() * float3(-1.0), 
            /* e23, e31, e12 */
            self.group3() * float3(-1.0), 
            /* e423, e431, e412, e321 */
            self.group4() * float4(-1.0)
        );
    }
}
extension MultiVector: Not {
    func not(self) -> MultiVector {
        return self.right_dual();
    }
}
extension MultiVector: Sub<AntiScalar> {
    func operator-(self, other: AntiScalar) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(0.0, other[e1234] * -1.0) + self.group0(), 
            /* e1, e2, e3, e4 */
            self.group1(), 
            /* e41, e42, e43 */
            self.group2(), 
            /* e23, e31, e12 */
            self.group3(), 
            /* e423, e431, e412, e321 */
            self.group4()
        );
    }
}
extension MultiVector {
    func operator-=(const AntiScalar& other) {
        *this = MultiVector::from_groups(
            /* scalar, e1234 */
            float2(0.0, other[e1234] * -1.0) + self.group0(), 
            /* e1, e2, e3, e4 */
            self.group1(), 
            /* e41, e42, e43 */
            self.group2(), 
            /* e23, e31, e12 */
            self.group3(), 
            /* e423, e431, e412, e321 */
            self.group4()
        );
    }
}
extension MultiVector: Sub<DualNum> {
    func operator-(self, other: DualNum) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            self.group0() - other.group0(), 
            /* e1, e2, e3, e4 */
            self.group1(), 
            /* e41, e42, e43 */
            self.group2(), 
            /* e23, e31, e12 */
            self.group3(), 
            /* e423, e431, e412, e321 */
            self.group4()
        );
    }
}
extension MultiVector {
    func operator-=(const DualNum& other) {
        *this = MultiVector::from_groups(
            /* scalar, e1234 */
            self.group0() - other.group0(), 
            /* e1, e2, e3, e4 */
            self.group1(), 
            /* e41, e42, e43 */
            self.group2(), 
            /* e23, e31, e12 */
            self.group3(), 
            /* e423, e431, e412, e321 */
            self.group4()
        );
    }
}
extension MultiVector: Sub<Flector> {
    func operator-(self, other: Flector) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            self.group0(), 
            /* e1, e2, e3, e4 */
            self.group1() - other.group0(), 
            /* e41, e42, e43 */
            self.group2(), 
            /* e23, e31, e12 */
            self.group3(), 
            /* e423, e431, e412, e321 */
            self.group4() - other.group1()
        );
    }
}
extension MultiVector {
    func operator-=(const Flector& other) {
        *this = MultiVector::from_groups(
            /* scalar, e1234 */
            self.group0(), 
            /* e1, e2, e3, e4 */
            self.group1() - other.group0(), 
            /* e41, e42, e43 */
            self.group2(), 
            /* e23, e31, e12 */
            self.group3(), 
            /* e423, e431, e412, e321 */
            self.group4() - other.group1()
        );
    }
}
extension MultiVector: Sub<Horizon> {
    func operator-(self, other: Horizon) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            self.group0(), 
            /* e1, e2, e3, e4 */
            self.group1(), 
            /* e41, e42, e43 */
            self.group2(), 
            /* e23, e31, e12 */
            self.group3(), 
            /* e423, e431, e412, e321 */
            self.group4() + float4(float3(0.0), other[e321] * -1.0)
        );
    }
}
extension MultiVector {
    func operator-=(const Horizon& other) {
        *this = MultiVector::from_groups(
            /* scalar, e1234 */
            self.group0(), 
            /* e1, e2, e3, e4 */
            self.group1(), 
            /* e41, e42, e43 */
            self.group2(), 
            /* e23, e31, e12 */
            self.group3(), 
            /* e423, e431, e412, e321 */
            self.group4() + float4(float3(0.0), other[e321] * -1.0)
        );
    }
}
extension MultiVector: Sub<Line> {
    func operator-(self, other: Line) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            self.group0(), 
            /* e1, e2, e3, e4 */
            self.group1(), 
            /* e41, e42, e43 */
            self.group2() - other.group0(), 
            /* e23, e31, e12 */
            self.group3() - other.group1(), 
            /* e423, e431, e412, e321 */
            self.group4()
        );
    }
}
extension MultiVector {
    func operator-=(const Line& other) {
        *this = MultiVector::from_groups(
            /* scalar, e1234 */
            self.group0(), 
            /* e1, e2, e3, e4 */
            self.group1(), 
            /* e41, e42, e43 */
            self.group2() - other.group0(), 
            /* e23, e31, e12 */
            self.group3() - other.group1(), 
            /* e423, e431, e412, e321 */
            self.group4()
        );
    }
}
extension MultiVector: Sub<Motor> {
    func operator-(self, other: Motor) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(other[scalar] * -1.0, other[e1234] * -1.0) + self.group0(), 
            /* e1, e2, e3, e4 */
            self.group1(), 
            /* e41, e42, e43 */
            self.group2() - other.group0().xyz, 
            /* e23, e31, e12 */
            self.group3() - other.group1().xyz, 
            /* e423, e431, e412, e321 */
            self.group4()
        );
    }
}
extension MultiVector {
    func operator-=(const Motor& other) {
        *this = MultiVector::from_groups(
            /* scalar, e1234 */
            float2(other[scalar] * -1.0, other[e1234] * -1.0) + self.group0(), 
            /* e1, e2, e3, e4 */
            self.group1(), 
            /* e41, e42, e43 */
            self.group2() - other.group0().xyz, 
            /* e23, e31, e12 */
            self.group3() - other.group1().xyz, 
            /* e423, e431, e412, e321 */
            self.group4()
        );
    }
}
extension MultiVector: Sub<MultiVector> {
    func operator-(self, other: MultiVector) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            self.group0() - other.group0(), 
            /* e1, e2, e3, e4 */
            self.group1() - other.group1(), 
            /* e41, e42, e43 */
            self.group2() - other.group2(), 
            /* e23, e31, e12 */
            self.group3() - other.group3(), 
            /* e423, e431, e412, e321 */
            self.group4() - other.group4()
        );
    }
}
extension MultiVector {
    func operator-=(const MultiVector& other) {
        *this = MultiVector::from_groups(
            /* scalar, e1234 */
            self.group0() - other.group0(), 
            /* e1, e2, e3, e4 */
            self.group1() - other.group1(), 
            /* e41, e42, e43 */
            self.group2() - other.group2(), 
            /* e23, e31, e12 */
            self.group3() - other.group3(), 
            /* e423, e431, e412, e321 */
            self.group4() - other.group4()
        );
    }
}
extension MultiVector: Sub<Origin> {
    func operator-(self, other: Origin) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            self.group0(), 
            /* e1, e2, e3, e4 */
            self.group1() + float4(float3(0.0), other[e4] * -1.0), 
            /* e41, e42, e43 */
            self.group2(), 
            /* e23, e31, e12 */
            self.group3(), 
            /* e423, e431, e412, e321 */
            self.group4()
        );
    }
}
extension MultiVector {
    func operator-=(const Origin& other) {
        *this = MultiVector::from_groups(
            /* scalar, e1234 */
            self.group0(), 
            /* e1, e2, e3, e4 */
            self.group1() + float4(float3(0.0), other[e4] * -1.0), 
            /* e41, e42, e43 */
            self.group2(), 
            /* e23, e31, e12 */
            self.group3(), 
            /* e423, e431, e412, e321 */
            self.group4()
        );
    }
}
extension MultiVector: Sub<Plane> {
    func operator-(self, other: Plane) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            self.group0(), 
            /* e1, e2, e3, e4 */
            self.group1(), 
            /* e41, e42, e43 */
            self.group2(), 
            /* e23, e31, e12 */
            self.group3(), 
            /* e423, e431, e412, e321 */
            self.group4() - other.group0()
        );
    }
}
extension MultiVector {
    func operator-=(const Plane& other) {
        *this = MultiVector::from_groups(
            /* scalar, e1234 */
            self.group0(), 
            /* e1, e2, e3, e4 */
            self.group1(), 
            /* e41, e42, e43 */
            self.group2(), 
            /* e23, e31, e12 */
            self.group3(), 
            /* e423, e431, e412, e321 */
            self.group4() - other.group0()
        );
    }
}
extension MultiVector: Sub<Point> {
    func operator-(self, other: Point) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            self.group0(), 
            /* e1, e2, e3, e4 */
            self.group1() - other.group0(), 
            /* e41, e42, e43 */
            self.group2(), 
            /* e23, e31, e12 */
            self.group3(), 
            /* e423, e431, e412, e321 */
            self.group4()
        );
    }
}
extension MultiVector {
    func operator-=(const Point& other) {
        *this = MultiVector::from_groups(
            /* scalar, e1234 */
            self.group0(), 
            /* e1, e2, e3, e4 */
            self.group1() - other.group0(), 
            /* e41, e42, e43 */
            self.group2(), 
            /* e23, e31, e12 */
            self.group3(), 
            /* e423, e431, e412, e321 */
            self.group4()
        );
    }
}
extension MultiVector: Sub<Scalar> {
    func operator-(self, other: Scalar) -> MultiVector {
        return MultiVector::from_groups(
            /* scalar, e1234 */
            float2(other[scalar] * -1.0, 0.0) + self.group0(), 
            /* e1, e2, e3, e4 */
            self.group1(), 
            /* e41, e42, e43 */
            self.group2(), 
            /* e23, e31, e12 */
            self.group3(), 
            /* e423, e431, e412, e321 */
            self.group4()
        );
    }
}
extension MultiVector {
    func operator-=(const Scalar& other) {
        *this = MultiVector::from_groups(
            /* scalar, e1234 */
            float2(other[scalar] * -1.0, 0.0) + self.group0(), 
            /* e1, e2, e3, e4 */
            self.group1(), 
            /* e41, e42, e43 */
            self.group2(), 
            /* e23, e31, e12 */
            self.group3(), 
            /* e423, e431, e412, e321 */
            self.group4()
        );
    }
}
