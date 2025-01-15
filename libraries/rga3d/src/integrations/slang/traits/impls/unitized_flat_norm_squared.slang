implementing rga3d;
using traits;
// Fancy postfix trick
public float operator /(Flector lhs, UnitizedFlatNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_flat_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedFlatNormSquaredPrefixOrPostfix lhs, Flector rhs) {
    return rhs.unitized_flat_norm_squared();
}
public extension Flector {
    public float unitized_flat_norm_squared() {
        return this.flat_bulk_norm_squared().scalar / (this.flat_weight_norm_squared().e1234);
    }
}
// Fancy postfix trick
public float operator /(Line lhs, UnitizedFlatNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_flat_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedFlatNormSquaredPrefixOrPostfix lhs, Line rhs) {
    return rhs.unitized_flat_norm_squared();
}
public extension Line {
    public float unitized_flat_norm_squared() {
        return this.flat_bulk_norm_squared().scalar / (this.flat_weight_norm_squared().e1234);
    }
}
// Fancy postfix trick
public float operator /(Motor lhs, UnitizedFlatNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_flat_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedFlatNormSquaredPrefixOrPostfix lhs, Motor rhs) {
    return rhs.unitized_flat_norm_squared();
}
public extension Motor {
    public float unitized_flat_norm_squared() {
        return this.flat_bulk_norm_squared().scalar / (this.flat_weight_norm_squared().e1234);
    }
}
// Fancy postfix trick
public float operator /(MultiVector lhs, UnitizedFlatNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_flat_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedFlatNormSquaredPrefixOrPostfix lhs, MultiVector rhs) {
    return rhs.unitized_flat_norm_squared();
}
public extension MultiVector {
    public float unitized_flat_norm_squared() {
        return this.flat_bulk_norm_squared().scalar / (this.flat_weight_norm_squared().e1234);
    }
}
// Fancy postfix trick
public float operator /(Point lhs, UnitizedFlatNormSquaredPrefixOrPostfix rhs)  {
    return lhs.unitized_flat_norm_squared();
    }
// Fancy prefix trick
public float operator /(UnitizedFlatNormSquaredPrefixOrPostfix lhs, Point rhs) {
    return rhs.unitized_flat_norm_squared();
}
public extension Point {
    public float unitized_flat_norm_squared() {
        return this.flat_bulk_norm_squared().scalar / (this.flat_weight_norm_squared().e1234);
    }
}
