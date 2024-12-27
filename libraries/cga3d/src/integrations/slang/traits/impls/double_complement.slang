extension AntiCircleRotor {
    // Fancy infix trick
    func operator/(_rhs: DoubleComplementPrefixOrPostfix) -> AntiCircleRotor {
        return this.double_complement();
    }
    func operator/=(const DoubleComplementPrefixOrPostfix& _rhs) -> AntiCircleRotor& {
        this = this.double_complement();
        return *this;
    }
}
extension AntiCircleRotor: DoubleComplement {
    associatedtype Output = AntiCircleRotor;
    func double_complement() -> Output {
        return this;
    }
}
extension AntiDipoleInversion {
    // Fancy infix trick
    func operator/(_rhs: DoubleComplementPrefixOrPostfix) -> AntiDipoleInversion {
        return this.double_complement();
    }
    func operator/=(const DoubleComplementPrefixOrPostfix& _rhs) -> AntiDipoleInversion& {
        this = this.double_complement();
        return *this;
    }
}
extension AntiDipoleInversion: DoubleComplement {
    associatedtype Output = AntiDipoleInversion;
    func double_complement() -> Output {
        return this;
    }
}
extension AntiDualNum {
    // Fancy infix trick
    func operator/(_rhs: DoubleComplementPrefixOrPostfix) -> AntiDualNum {
        return this.double_complement();
    }
    func operator/=(const DoubleComplementPrefixOrPostfix& _rhs) -> AntiDualNum& {
        this = this.double_complement();
        return *this;
    }
}
extension AntiDualNum: DoubleComplement {
    associatedtype Output = AntiDualNum;
    func double_complement() -> Output {
        return this;
    }
}
extension AntiFlatPoint {
    // Fancy infix trick
    func operator/(_rhs: DoubleComplementPrefixOrPostfix) -> AntiFlatPoint {
        return this.double_complement();
    }
    func operator/=(const DoubleComplementPrefixOrPostfix& _rhs) -> AntiFlatPoint& {
        this = this.double_complement();
        return *this;
    }
}
extension AntiFlatPoint: DoubleComplement {
    associatedtype Output = AntiFlatPoint;
    func double_complement() -> Output {
        return this;
    }
}
extension AntiFlector {
    // Fancy infix trick
    func operator/(_rhs: DoubleComplementPrefixOrPostfix) -> AntiFlector {
        return this.double_complement();
    }
    func operator/=(const DoubleComplementPrefixOrPostfix& _rhs) -> AntiFlector& {
        this = this.double_complement();
        return *this;
    }
}
extension AntiFlector: DoubleComplement {
    associatedtype Output = AntiFlector;
    func double_complement() -> Output {
        return this;
    }
}
extension AntiLine {
    // Fancy infix trick
    func operator/(_rhs: DoubleComplementPrefixOrPostfix) -> AntiLine {
        return this.double_complement();
    }
    func operator/=(const DoubleComplementPrefixOrPostfix& _rhs) -> AntiLine& {
        this = this.double_complement();
        return *this;
    }
}
extension AntiLine: DoubleComplement {
    associatedtype Output = AntiLine;
    func double_complement() -> Output {
        return this;
    }
}
extension AntiMotor {
    // Fancy infix trick
    func operator/(_rhs: DoubleComplementPrefixOrPostfix) -> AntiMotor {
        return this.double_complement();
    }
    func operator/=(const DoubleComplementPrefixOrPostfix& _rhs) -> AntiMotor& {
        this = this.double_complement();
        return *this;
    }
}
extension AntiMotor: DoubleComplement {
    associatedtype Output = AntiMotor;
    func double_complement() -> Output {
        return this;
    }
}
extension AntiPlane {
    // Fancy infix trick
    func operator/(_rhs: DoubleComplementPrefixOrPostfix) -> AntiPlane {
        return this.double_complement();
    }
    func operator/=(const DoubleComplementPrefixOrPostfix& _rhs) -> AntiPlane& {
        this = this.double_complement();
        return *this;
    }
}
extension AntiPlane: DoubleComplement {
    associatedtype Output = AntiPlane;
    func double_complement() -> Output {
        return this;
    }
}
extension AntiScalar {
    // Fancy infix trick
    func operator/(_rhs: DoubleComplementPrefixOrPostfix) -> AntiScalar {
        return this.double_complement();
    }
    func operator/=(const DoubleComplementPrefixOrPostfix& _rhs) -> AntiScalar& {
        this = this.double_complement();
        return *this;
    }
}
extension AntiScalar: DoubleComplement {
    associatedtype Output = AntiScalar;
    func double_complement() -> Output {
        return this;
    }
}
extension Circle {
    // Fancy infix trick
    func operator/(_rhs: DoubleComplementPrefixOrPostfix) -> Circle {
        return this.double_complement();
    }
    func operator/=(const DoubleComplementPrefixOrPostfix& _rhs) -> Circle& {
        this = this.double_complement();
        return *this;
    }
}
extension Circle: DoubleComplement {
    associatedtype Output = Circle;
    func double_complement() -> Output {
        return this;
    }
}
extension CircleRotor {
    // Fancy infix trick
    func operator/(_rhs: DoubleComplementPrefixOrPostfix) -> CircleRotor {
        return this.double_complement();
    }
    func operator/=(const DoubleComplementPrefixOrPostfix& _rhs) -> CircleRotor& {
        this = this.double_complement();
        return *this;
    }
}
extension CircleRotor: DoubleComplement {
    associatedtype Output = CircleRotor;
    func double_complement() -> Output {
        return this;
    }
}
extension Dipole {
    // Fancy infix trick
    func operator/(_rhs: DoubleComplementPrefixOrPostfix) -> Dipole {
        return this.double_complement();
    }
    func operator/=(const DoubleComplementPrefixOrPostfix& _rhs) -> Dipole& {
        this = this.double_complement();
        return *this;
    }
}
extension Dipole: DoubleComplement {
    associatedtype Output = Dipole;
    func double_complement() -> Output {
        return this;
    }
}
extension DipoleInversion {
    // Fancy infix trick
    func operator/(_rhs: DoubleComplementPrefixOrPostfix) -> DipoleInversion {
        return this.double_complement();
    }
    func operator/=(const DoubleComplementPrefixOrPostfix& _rhs) -> DipoleInversion& {
        this = this.double_complement();
        return *this;
    }
}
extension DipoleInversion: DoubleComplement {
    associatedtype Output = DipoleInversion;
    func double_complement() -> Output {
        return this;
    }
}
extension DualNum {
    // Fancy infix trick
    func operator/(_rhs: DoubleComplementPrefixOrPostfix) -> DualNum {
        return this.double_complement();
    }
    func operator/=(const DoubleComplementPrefixOrPostfix& _rhs) -> DualNum& {
        this = this.double_complement();
        return *this;
    }
}
extension DualNum: DoubleComplement {
    associatedtype Output = DualNum;
    func double_complement() -> Output {
        return this;
    }
}
extension FlatPoint {
    // Fancy infix trick
    func operator/(_rhs: DoubleComplementPrefixOrPostfix) -> FlatPoint {
        return this.double_complement();
    }
    func operator/=(const DoubleComplementPrefixOrPostfix& _rhs) -> FlatPoint& {
        this = this.double_complement();
        return *this;
    }
}
extension FlatPoint: DoubleComplement {
    associatedtype Output = FlatPoint;
    func double_complement() -> Output {
        return this;
    }
}
extension Flector {
    // Fancy infix trick
    func operator/(_rhs: DoubleComplementPrefixOrPostfix) -> Flector {
        return this.double_complement();
    }
    func operator/=(const DoubleComplementPrefixOrPostfix& _rhs) -> Flector& {
        this = this.double_complement();
        return *this;
    }
}
extension Flector: DoubleComplement {
    associatedtype Output = Flector;
    func double_complement() -> Output {
        return this;
    }
}
extension Line {
    // Fancy infix trick
    func operator/(_rhs: DoubleComplementPrefixOrPostfix) -> Line {
        return this.double_complement();
    }
    func operator/=(const DoubleComplementPrefixOrPostfix& _rhs) -> Line& {
        this = this.double_complement();
        return *this;
    }
}
extension Line: DoubleComplement {
    associatedtype Output = Line;
    func double_complement() -> Output {
        return this;
    }
}
extension Motor {
    // Fancy infix trick
    func operator/(_rhs: DoubleComplementPrefixOrPostfix) -> Motor {
        return this.double_complement();
    }
    func operator/=(const DoubleComplementPrefixOrPostfix& _rhs) -> Motor& {
        this = this.double_complement();
        return *this;
    }
}
extension Motor: DoubleComplement {
    associatedtype Output = Motor;
    func double_complement() -> Output {
        return this;
    }
}
extension MultiVector {
    // Fancy infix trick
    func operator/(_rhs: DoubleComplementPrefixOrPostfix) -> MultiVector {
        return this.double_complement();
    }
    func operator/=(const DoubleComplementPrefixOrPostfix& _rhs) -> MultiVector& {
        this = this.double_complement();
        return *this;
    }
}
extension MultiVector: DoubleComplement {
    associatedtype Output = MultiVector;
    func double_complement() -> Output {
        return this;
    }
}
extension Plane {
    // Fancy infix trick
    func operator/(_rhs: DoubleComplementPrefixOrPostfix) -> Plane {
        return this.double_complement();
    }
    func operator/=(const DoubleComplementPrefixOrPostfix& _rhs) -> Plane& {
        this = this.double_complement();
        return *this;
    }
}
extension Plane: DoubleComplement {
    associatedtype Output = Plane;
    func double_complement() -> Output {
        return this;
    }
}
extension RoundPoint {
    // Fancy infix trick
    func operator/(_rhs: DoubleComplementPrefixOrPostfix) -> RoundPoint {
        return this.double_complement();
    }
    func operator/=(const DoubleComplementPrefixOrPostfix& _rhs) -> RoundPoint& {
        this = this.double_complement();
        return *this;
    }
}
extension RoundPoint: DoubleComplement {
    associatedtype Output = RoundPoint;
    func double_complement() -> Output {
        return this;
    }
}
extension Scalar {
    // Fancy infix trick
    func operator/(_rhs: DoubleComplementPrefixOrPostfix) -> Scalar {
        return this.double_complement();
    }
    func operator/=(const DoubleComplementPrefixOrPostfix& _rhs) -> Scalar& {
        this = this.double_complement();
        return *this;
    }
}
extension Scalar: DoubleComplement {
    associatedtype Output = Scalar;
    func double_complement() -> Output {
        return this;
    }
}
extension Sphere {
    // Fancy infix trick
    func operator/(_rhs: DoubleComplementPrefixOrPostfix) -> Sphere {
        return this.double_complement();
    }
    func operator/=(const DoubleComplementPrefixOrPostfix& _rhs) -> Sphere& {
        this = this.double_complement();
        return *this;
    }
}
extension Sphere: DoubleComplement {
    associatedtype Output = Sphere;
    func double_complement() -> Output {
        return this;
    }
}
extension VersorEven {
    // Fancy infix trick
    func operator/(_rhs: DoubleComplementPrefixOrPostfix) -> VersorEven {
        return this.double_complement();
    }
    func operator/=(const DoubleComplementPrefixOrPostfix& _rhs) -> VersorEven& {
        this = this.double_complement();
        return *this;
    }
}
extension VersorEven: DoubleComplement {
    associatedtype Output = VersorEven;
    func double_complement() -> Output {
        return this;
    }
}
extension VersorOdd {
    // Fancy infix trick
    func operator/(_rhs: DoubleComplementPrefixOrPostfix) -> VersorOdd {
        return this.double_complement();
    }
    func operator/=(const DoubleComplementPrefixOrPostfix& _rhs) -> VersorOdd& {
        this = this.double_complement();
        return *this;
    }
}
extension VersorOdd: DoubleComplement {
    associatedtype Output = VersorOdd;
    func double_complement() -> Output {
        return this;
    }
}
