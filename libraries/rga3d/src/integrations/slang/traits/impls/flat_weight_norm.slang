using traits::AntiSquareRoot;
using traits::FlatWeightNormSquared;
extension Flector {
    func div(_rhs: flat_weight_norm) -> 
AntiScalar {
        this.flat_weight_norm()
    }
}
extension Flector: FlatWeightNorm {
    func flat_weight_norm(self) -> AntiScalar {
        return self.flat_weight_norm_squared().anti_square_root();
    }
}
extension Line {
    func div(_rhs: flat_weight_norm) -> 
AntiScalar {
        this.flat_weight_norm()
    }
}
extension Line: FlatWeightNorm {
    func flat_weight_norm(self) -> AntiScalar {
        return self.flat_weight_norm_squared().anti_square_root();
    }
}
extension Motor {
    func div(_rhs: flat_weight_norm) -> 
AntiScalar {
        this.flat_weight_norm()
    }
}
extension Motor: FlatWeightNorm {
    func flat_weight_norm(self) -> AntiScalar {
        return self.flat_weight_norm_squared().anti_square_root();
    }
}
extension MultiVector {
    func div(_rhs: flat_weight_norm) -> 
AntiScalar {
        this.flat_weight_norm()
    }
}
extension MultiVector: FlatWeightNorm {
    func flat_weight_norm(self) -> AntiScalar {
        return self.flat_weight_norm_squared().anti_square_root();
    }
}
extension Point {
    func div(_rhs: flat_weight_norm) -> 
AntiScalar {
        this.flat_weight_norm()
    }
}
extension Point: FlatWeightNorm {
    func flat_weight_norm(self) -> AntiScalar {
        return self.flat_weight_norm_squared().anti_square_root();
    }
}
