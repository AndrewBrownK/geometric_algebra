implementing rga3d;
using traits;
public extension Flector {
    // Fancy postfix trick
    public float operator /(UnitizedFlatNormSquaredPrefixOrPostfix rhs)  {
        return this.unitized_flat_norm_squared();
    }
}
public extension UnitizedFlatNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public float operator /(Flector rhs) {
        return rhs.unitized_flat_norm_squared();
    }
}
public extension Flector {
    public float unitized_flat_norm_squared() {
        return this.flat_bulk_norm_squared().scalar / (this.flat_weight_norm_squared().e1234);
    }
}
public extension Line {
    // Fancy postfix trick
    public float operator /(UnitizedFlatNormSquaredPrefixOrPostfix rhs)  {
        return this.unitized_flat_norm_squared();
    }
}
public extension UnitizedFlatNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public float operator /(Line rhs) {
        return rhs.unitized_flat_norm_squared();
    }
}
public extension Line {
    public float unitized_flat_norm_squared() {
        return this.flat_bulk_norm_squared().scalar / (this.flat_weight_norm_squared().e1234);
    }
}
public extension Motor {
    // Fancy postfix trick
    public float operator /(UnitizedFlatNormSquaredPrefixOrPostfix rhs)  {
        return this.unitized_flat_norm_squared();
    }
}
public extension UnitizedFlatNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public float operator /(Motor rhs) {
        return rhs.unitized_flat_norm_squared();
    }
}
public extension Motor {
    public float unitized_flat_norm_squared() {
        return this.flat_bulk_norm_squared().scalar / (this.flat_weight_norm_squared().e1234);
    }
}
public extension MultiVector {
    // Fancy postfix trick
    public float operator /(UnitizedFlatNormSquaredPrefixOrPostfix rhs)  {
        return this.unitized_flat_norm_squared();
    }
}
public extension UnitizedFlatNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public float operator /(MultiVector rhs) {
        return rhs.unitized_flat_norm_squared();
    }
}
public extension MultiVector {
    public float unitized_flat_norm_squared() {
        return this.flat_bulk_norm_squared().scalar / (this.flat_weight_norm_squared().e1234);
    }
}
public extension Point {
    // Fancy postfix trick
    public float operator /(UnitizedFlatNormSquaredPrefixOrPostfix rhs)  {
        return this.unitized_flat_norm_squared();
    }
}
public extension UnitizedFlatNormSquaredPrefixOrPostfix {
    // Fancy prefix trick
    public float operator /(Point rhs) {
        return rhs.unitized_flat_norm_squared();
    }
}
public extension Point {
    public float unitized_flat_norm_squared() {
        return this.flat_bulk_norm_squared().scalar / (this.flat_weight_norm_squared().e1234);
    }
}
