extension AntiFlatPoint: AntiGrade {
    associatedtype Output = uint;
    func anti_grade() -> Output {
        return 2;
    }
}
extension AntiLine: AntiGrade {
    associatedtype Output = uint;
    func anti_grade() -> Output {
        return 3;
    }
}
extension AntiPlane: AntiGrade {
    associatedtype Output = uint;
    func anti_grade() -> Output {
        return 4;
    }
}
extension AntiScalar: AntiGrade {
    associatedtype Output = uint;
    func anti_grade() -> Output {
        return 0;
    }
}
extension Circle: AntiGrade {
    associatedtype Output = uint;
    func anti_grade() -> Output {
        return 2;
    }
}
extension Dipole: AntiGrade {
    associatedtype Output = uint;
    func anti_grade() -> Output {
        return 3;
    }
}
extension FlatPoint: AntiGrade {
    associatedtype Output = uint;
    func anti_grade() -> Output {
        return 3;
    }
}
extension Line: AntiGrade {
    associatedtype Output = uint;
    func anti_grade() -> Output {
        return 2;
    }
}
extension Plane: AntiGrade {
    associatedtype Output = uint;
    func anti_grade() -> Output {
        return 1;
    }
}
extension RoundPoint: AntiGrade {
    associatedtype Output = uint;
    func anti_grade() -> Output {
        return 4;
    }
}
extension Scalar: AntiGrade {
    associatedtype Output = uint;
    func anti_grade() -> Output {
        return 5;
    }
}
extension Sphere: AntiGrade {
    associatedtype Output = uint;
    func anti_grade() -> Output {
        return 1;
    }
}
