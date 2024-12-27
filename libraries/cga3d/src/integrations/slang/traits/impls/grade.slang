extension AntiFlatPoint: Grade {
    associatedtype Output = uint;
    func grade() -> Output {
        return 3;
    }
}
extension AntiLine: Grade {
    associatedtype Output = uint;
    func grade() -> Output {
        return 2;
    }
}
extension AntiPlane: Grade {
    associatedtype Output = uint;
    func grade() -> Output {
        return 1;
    }
}
extension AntiScalar: Grade {
    associatedtype Output = uint;
    func grade() -> Output {
        return 5;
    }
}
extension Circle: Grade {
    associatedtype Output = uint;
    func grade() -> Output {
        return 3;
    }
}
extension Dipole: Grade {
    associatedtype Output = uint;
    func grade() -> Output {
        return 2;
    }
}
extension FlatPoint: Grade {
    associatedtype Output = uint;
    func grade() -> Output {
        return 2;
    }
}
extension Line: Grade {
    associatedtype Output = uint;
    func grade() -> Output {
        return 3;
    }
}
extension Plane: Grade {
    associatedtype Output = uint;
    func grade() -> Output {
        return 4;
    }
}
extension RoundPoint: Grade {
    associatedtype Output = uint;
    func grade() -> Output {
        return 1;
    }
}
extension Scalar: Grade {
    associatedtype Output = uint;
    func grade() -> Output {
        return 0;
    }
}
extension Sphere: Grade {
    associatedtype Output = uint;
    func grade() -> Output {
        return 4;
    }
}
