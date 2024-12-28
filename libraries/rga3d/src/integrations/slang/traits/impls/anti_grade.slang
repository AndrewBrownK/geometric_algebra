public extension AntiScalar: AntiGrade {
    public typedef uint AntiGradeOutput;
    public func anti_grade() -> uint {
        return 0;
    }
}
public extension Horizon: AntiGrade {
    public typedef uint AntiGradeOutput;
    public func anti_grade() -> uint {
        return 1;
    }
}
public extension Line: AntiGrade {
    public typedef uint AntiGradeOutput;
    public func anti_grade() -> uint {
        return 2;
    }
}
public extension Origin: AntiGrade {
    public typedef uint AntiGradeOutput;
    public func anti_grade() -> uint {
        return 3;
    }
}
public extension Plane: AntiGrade {
    public typedef uint AntiGradeOutput;
    public func anti_grade() -> uint {
        return 1;
    }
}
public extension Point: AntiGrade {
    public typedef uint AntiGradeOutput;
    public func anti_grade() -> uint {
        return 3;
    }
}
public extension Scalar: AntiGrade {
    public typedef uint AntiGradeOutput;
    public func anti_grade() -> uint {
        return 4;
    }
}
