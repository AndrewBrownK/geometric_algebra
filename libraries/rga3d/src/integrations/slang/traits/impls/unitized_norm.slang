using traits::UnitizedFlatNormSquared;
extension Flector {
    func div(_rhs: unitized_norm) -> 
float {
        this.unitized_norm()
    }
}
extension Flector: UnitizedNorm {
    func unitized_norm(self) -> float {
        return powf(self.unitized_flat_norm_squared(), 0.5);
    }
}
extension Line {
    func div(_rhs: unitized_norm) -> 
float {
        this.unitized_norm()
    }
}
extension Line: UnitizedNorm {
    func unitized_norm(self) -> float {
        return powf(self.unitized_flat_norm_squared(), 0.5);
    }
}
extension Motor {
    func div(_rhs: unitized_norm) -> 
float {
        this.unitized_norm()
    }
}
extension Motor: UnitizedNorm {
    func unitized_norm(self) -> float {
        return powf(self.unitized_flat_norm_squared(), 0.5);
    }
}
extension MultiVector {
    func div(_rhs: unitized_norm) -> 
float {
        this.unitized_norm()
    }
}
extension MultiVector: UnitizedNorm {
    func unitized_norm(self) -> float {
        return powf(self.unitized_flat_norm_squared(), 0.5);
    }
}
extension Point {
    func div(_rhs: unitized_norm) -> 
float {
        this.unitized_norm()
    }
}
extension Point: UnitizedNorm {
    func unitized_norm(self) -> float {
        return powf(self.unitized_flat_norm_squared(), 0.5);
    }
}
