extension AntiFlatPoint {
    // Fancy infix trick
    func operator/(_rhs: ConstraintValidPrefixOrPostfix) -> AntiFlatPoint {
        return this.constraint_valid();
    }
    func operator/=(const ConstraintValidPrefixOrPostfix& _rhs) -> AntiFlatPoint& {
        this = this.constraint_valid();
        return *this;
    }
}
extension AntiFlatPoint: ConstraintValid {
    associatedtype Output = AntiFlatPoint;
    func constraint_valid() -> Output {
        return this;
    }
}
extension AntiPlane {
    // Fancy infix trick
    func operator/(_rhs: ConstraintValidPrefixOrPostfix) -> AntiPlane {
        return this.constraint_valid();
    }
    func operator/=(const ConstraintValidPrefixOrPostfix& _rhs) -> AntiPlane& {
        this = this.constraint_valid();
        return *this;
    }
}
extension AntiPlane: ConstraintValid {
    associatedtype Output = AntiPlane;
    func constraint_valid() -> Output {
        return this;
    }
}
extension AntiScalar {
    // Fancy infix trick
    func operator/(_rhs: ConstraintValidPrefixOrPostfix) -> AntiScalar {
        return this.constraint_valid();
    }
    func operator/=(const ConstraintValidPrefixOrPostfix& _rhs) -> AntiScalar& {
        this = this.constraint_valid();
        return *this;
    }
}
extension AntiScalar: ConstraintValid {
    associatedtype Output = AntiScalar;
    func constraint_valid() -> Output {
        return this;
    }
}
extension FlatPoint {
    // Fancy infix trick
    func operator/(_rhs: ConstraintValidPrefixOrPostfix) -> FlatPoint {
        return this.constraint_valid();
    }
    func operator/=(const ConstraintValidPrefixOrPostfix& _rhs) -> FlatPoint& {
        this = this.constraint_valid();
        return *this;
    }
}
extension FlatPoint: ConstraintValid {
    associatedtype Output = FlatPoint;
    func constraint_valid() -> Output {
        return this;
    }
}
extension Plane {
    // Fancy infix trick
    func operator/(_rhs: ConstraintValidPrefixOrPostfix) -> Plane {
        return this.constraint_valid();
    }
    func operator/=(const ConstraintValidPrefixOrPostfix& _rhs) -> Plane& {
        this = this.constraint_valid();
        return *this;
    }
}
extension Plane: ConstraintValid {
    associatedtype Output = Plane;
    func constraint_valid() -> Output {
        return this;
    }
}
extension RoundPoint {
    // Fancy infix trick
    func operator/(_rhs: ConstraintValidPrefixOrPostfix) -> RoundPoint {
        return this.constraint_valid();
    }
    func operator/=(const ConstraintValidPrefixOrPostfix& _rhs) -> RoundPoint& {
        this = this.constraint_valid();
        return *this;
    }
}
extension RoundPoint: ConstraintValid {
    associatedtype Output = RoundPoint;
    func constraint_valid() -> Output {
        return this;
    }
}
extension Scalar {
    // Fancy infix trick
    func operator/(_rhs: ConstraintValidPrefixOrPostfix) -> Scalar {
        return this.constraint_valid();
    }
    func operator/=(const ConstraintValidPrefixOrPostfix& _rhs) -> Scalar& {
        this = this.constraint_valid();
        return *this;
    }
}
extension Scalar: ConstraintValid {
    associatedtype Output = Scalar;
    func constraint_valid() -> Output {
        return this;
    }
}
extension Sphere {
    // Fancy infix trick
    func operator/(_rhs: ConstraintValidPrefixOrPostfix) -> Sphere {
        return this.constraint_valid();
    }
    func operator/=(const ConstraintValidPrefixOrPostfix& _rhs) -> Sphere& {
        this = this.constraint_valid();
        return *this;
    }
}
extension Sphere: ConstraintValid {
    associatedtype Output = Sphere;
    func constraint_valid() -> Output {
        return this;
    }
}
