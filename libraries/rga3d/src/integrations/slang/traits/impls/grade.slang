public extension AntiScalar: Grade {
    public typedef uint GradeOutput;
    public func grade() -> uint {
        return 4;
    }
}
public extension Horizon: Grade {
    public typedef uint GradeOutput;
    public func grade() -> uint {
        return 3;
    }
}
public extension Line: Grade {
    public typedef uint GradeOutput;
    public func grade() -> uint {
        return 2;
    }
}
public extension Origin: Grade {
    public typedef uint GradeOutput;
    public func grade() -> uint {
        return 1;
    }
}
public extension Plane: Grade {
    public typedef uint GradeOutput;
    public func grade() -> uint {
        return 3;
    }
}
public extension Point: Grade {
    public typedef uint GradeOutput;
    public func grade() -> uint {
        return 1;
    }
}
public extension Scalar: Grade {
    public typedef uint GradeOutput;
    public func grade() -> uint {
        return 0;
    }
}
