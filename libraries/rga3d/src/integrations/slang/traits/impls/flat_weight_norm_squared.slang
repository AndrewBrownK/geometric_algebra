implementing rga3d;
using traits;
// Fancy postfix trick
public AntiScalar operator /(AntiScalar lhs, FlatWeightNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_weight_norm_squared();
    }
// Fancy prefix trick
public AntiScalar operator /(FlatWeightNormSquaredPrefixOrPostfix lhs, AntiScalar rhs) {
    return rhs.flat_weight_norm_squared();
}
public extension AntiScalar {
    public AntiScalar flat_weight_norm_squared() {
        AntiScalar flat_weight = this.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}
// Fancy postfix trick
public AntiScalar operator /(DualNum lhs, FlatWeightNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_weight_norm_squared();
    }
// Fancy prefix trick
public AntiScalar operator /(FlatWeightNormSquaredPrefixOrPostfix lhs, DualNum rhs) {
    return rhs.flat_weight_norm_squared();
}
public extension DualNum {
    public AntiScalar flat_weight_norm_squared() {
        AntiScalar flat_weight = this.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}
// Fancy postfix trick
public AntiScalar operator /(Flector lhs, FlatWeightNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_weight_norm_squared();
    }
// Fancy prefix trick
public AntiScalar operator /(FlatWeightNormSquaredPrefixOrPostfix lhs, Flector rhs) {
    return rhs.flat_weight_norm_squared();
}
public extension Flector {
    public AntiScalar flat_weight_norm_squared() {
        Flector flat_weight = this.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}
// Fancy postfix trick
public AntiScalar operator /(Line lhs, FlatWeightNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_weight_norm_squared();
    }
// Fancy prefix trick
public AntiScalar operator /(FlatWeightNormSquaredPrefixOrPostfix lhs, Line rhs) {
    return rhs.flat_weight_norm_squared();
}
public extension Line {
    public AntiScalar flat_weight_norm_squared() {
        Line flat_weight = this.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}
// Fancy postfix trick
public AntiScalar operator /(Motor lhs, FlatWeightNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_weight_norm_squared();
    }
// Fancy prefix trick
public AntiScalar operator /(FlatWeightNormSquaredPrefixOrPostfix lhs, Motor rhs) {
    return rhs.flat_weight_norm_squared();
}
public extension Motor {
    public AntiScalar flat_weight_norm_squared() {
        Motor flat_weight = this.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}
// Fancy postfix trick
public AntiScalar operator /(MultiVector lhs, FlatWeightNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_weight_norm_squared();
    }
// Fancy prefix trick
public AntiScalar operator /(FlatWeightNormSquaredPrefixOrPostfix lhs, MultiVector rhs) {
    return rhs.flat_weight_norm_squared();
}
public extension MultiVector {
    public AntiScalar flat_weight_norm_squared() {
        MultiVector flat_weight = this.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}
// Fancy postfix trick
public AntiScalar operator /(Origin lhs, FlatWeightNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_weight_norm_squared();
    }
// Fancy prefix trick
public AntiScalar operator /(FlatWeightNormSquaredPrefixOrPostfix lhs, Origin rhs) {
    return rhs.flat_weight_norm_squared();
}
public extension Origin {
    public AntiScalar flat_weight_norm_squared() {
        Origin flat_weight = this.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}
// Fancy postfix trick
public AntiScalar operator /(Plane lhs, FlatWeightNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_weight_norm_squared();
    }
// Fancy prefix trick
public AntiScalar operator /(FlatWeightNormSquaredPrefixOrPostfix lhs, Plane rhs) {
    return rhs.flat_weight_norm_squared();
}
public extension Plane {
    public AntiScalar flat_weight_norm_squared() {
        Plane flat_weight = this.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}
// Fancy postfix trick
public AntiScalar operator /(Point lhs, FlatWeightNormSquaredPrefixOrPostfix rhs)  {
    return lhs.flat_weight_norm_squared();
    }
// Fancy prefix trick
public AntiScalar operator /(FlatWeightNormSquaredPrefixOrPostfix lhs, Point rhs) {
    return rhs.flat_weight_norm_squared();
}
public extension Point {
    public AntiScalar flat_weight_norm_squared() {
        Origin flat_weight = this.flat_weight();
        return flat_weight.anti_dot_product(flat_weight);
    }
}
