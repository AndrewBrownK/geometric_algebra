extension AntiScalar: AntiGrade {
    func anti_grade() -> uint {
        return 0;
    }
}
extension Horizon: AntiGrade {
    func anti_grade() -> uint {
        return 1;
    }
}
extension Line: AntiGrade {
    func anti_grade() -> uint {
        return 2;
    }
}
extension Origin: AntiGrade {
    func anti_grade() -> uint {
        return 3;
    }
}
extension Plane: AntiGrade {
    func anti_grade() -> uint {
        return 1;
    }
}
extension Point: AntiGrade {
    func anti_grade() -> uint {
        return 3;
    }
}
extension Scalar: AntiGrade {
    func anti_grade() -> uint {
        return 4;
    }
}
