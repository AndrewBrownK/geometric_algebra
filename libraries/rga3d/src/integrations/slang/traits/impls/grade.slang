extension AntiScalar: Grade {
    associatedtype Output = uint;
    func grade() -> Output {
        return 4;
    }
}
extension Horizon: Grade {
    associatedtype Output = uint;
    func grade() -> Output {
        return 3;
    }
}
extension Line: Grade {
    associatedtype Output = uint;
    func grade() -> Output {
        return 2;
    }
}
extension Origin: Grade {
    associatedtype Output = uint;
    func grade() -> Output {
        return 1;
    }
}
extension Plane: Grade {
    associatedtype Output = uint;
    func grade() -> Output {
        return 3;
    }
}
extension Point: Grade {
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
