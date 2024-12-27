extension AntiScalar: AntiGrade {
    associatedtype Output = uint;
    func anti_grade() -> Output {
        return 0;
    }
}
extension Horizon: AntiGrade {
    associatedtype Output = uint;
    func anti_grade() -> Output {
        return 1;
    }
}
extension Line: AntiGrade {
    associatedtype Output = uint;
    func anti_grade() -> Output {
        return 2;
    }
}
extension Origin: AntiGrade {
    associatedtype Output = uint;
    func anti_grade() -> Output {
        return 3;
    }
}
extension Plane: AntiGrade {
    associatedtype Output = uint;
    func anti_grade() -> Output {
        return 1;
    }
}
extension Point: AntiGrade {
    associatedtype Output = uint;
    func anti_grade() -> Output {
        return 3;
    }
}
extension Scalar: AntiGrade {
    associatedtype Output = uint;
    func anti_grade() -> Output {
        return 4;
    }
}
