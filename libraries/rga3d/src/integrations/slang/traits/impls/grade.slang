extension AntiScalar: Grade {
    func grade() -> uint {
        return 4;
    }
}
extension Horizon: Grade {
    func grade() -> uint {
        return 3;
    }
}
extension Line: Grade {
    func grade() -> uint {
        return 2;
    }
}
extension Origin: Grade {
    func grade() -> uint {
        return 1;
    }
}
extension Plane: Grade {
    func grade() -> uint {
        return 3;
    }
}
extension Point: Grade {
    func grade() -> uint {
        return 1;
    }
}
extension Scalar: Grade {
    func grade() -> uint {
        return 0;
    }
}
