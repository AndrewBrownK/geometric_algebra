using traits::DotProduct;
using traits::FlatBulk;
using traits::Wedge;
extension AntiCircleRotor {
    // Fancy infix trick
    func operator/(_rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Scalar {
        return this.flat_bulk_norm_squared();
    }
}
extension AntiCircleRotor: FlatBulkNormSquared {
    associatedtype Output = Scalar;
    func flat_bulk_norm_squared() -> Output {
        Line flat_bulk_thing = this.flat_bulk().wedge(RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5, 0, 0, 0 */
            float4(0.0)
        ));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
extension AntiDipoleInversion {
    // Fancy infix trick
    func operator/(_rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Scalar {
        return this.flat_bulk_norm_squared();
    }
}
extension AntiDipoleInversion: FlatBulkNormSquared {
    associatedtype Output = Scalar;
    func flat_bulk_norm_squared() -> Output {
        DipoleInversion flat_bulk_thing = this.flat_bulk().wedge(RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5, 0, 0, 0 */
            float4(0.0)
        ));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
extension AntiDualNum {
    // Fancy infix trick
    func operator/(_rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Scalar {
        return this.flat_bulk_norm_squared();
    }
}
extension AntiDualNum: FlatBulkNormSquared {
    associatedtype Output = Scalar;
    func flat_bulk_norm_squared() -> Output {
        VersorEven flat_bulk_thing = this.flat_bulk().wedge(RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5, 0, 0, 0 */
            float4(0.0)
        ));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
extension AntiFlatPoint {
    // Fancy infix trick
    func operator/(_rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Scalar {
        return this.flat_bulk_norm_squared();
    }
}
extension AntiFlatPoint: FlatBulkNormSquared {
    associatedtype Output = Scalar;
    func flat_bulk_norm_squared() -> Output {
        Sphere flat_bulk_thing = this.flat_bulk().wedge(RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5, 0, 0, 0 */
            float4(0.0)
        ));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
extension AntiFlector {
    // Fancy infix trick
    func operator/(_rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Scalar {
        return this.flat_bulk_norm_squared();
    }
}
extension AntiFlector: FlatBulkNormSquared {
    associatedtype Output = Scalar;
    func flat_bulk_norm_squared() -> Output {
        DipoleInversion flat_bulk_thing = this.flat_bulk().wedge(RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5, 0, 0, 0 */
            float4(0.0)
        ));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
extension AntiLine {
    // Fancy infix trick
    func operator/(_rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Scalar {
        return this.flat_bulk_norm_squared();
    }
}
extension AntiLine: FlatBulkNormSquared {
    associatedtype Output = Scalar;
    func flat_bulk_norm_squared() -> Output {
        Line flat_bulk_thing = this.flat_bulk().wedge(RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5, 0, 0, 0 */
            float4(0.0)
        ));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
extension AntiMotor {
    // Fancy infix trick
    func operator/(_rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Scalar {
        return this.flat_bulk_norm_squared();
    }
}
extension AntiMotor: FlatBulkNormSquared {
    associatedtype Output = Scalar;
    func flat_bulk_norm_squared() -> Output {
        Motor flat_bulk_thing = this.flat_bulk().wedge(RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5, 0, 0, 0 */
            float4(0.0)
        ));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
extension AntiPlane {
    // Fancy infix trick
    func operator/(_rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Scalar {
        return this.flat_bulk_norm_squared();
    }
}
extension AntiPlane: FlatBulkNormSquared {
    associatedtype Output = Scalar;
    func flat_bulk_norm_squared() -> Output {
        FlatPoint flat_bulk_thing = this.flat_bulk().wedge(RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5, 0, 0, 0 */
            float4(0.0)
        ));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
extension Circle {
    // Fancy infix trick
    func operator/(_rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Scalar {
        return this.flat_bulk_norm_squared();
    }
}
extension Circle: FlatBulkNormSquared {
    associatedtype Output = Scalar;
    func flat_bulk_norm_squared() -> Output {
        Sphere flat_bulk_thing = this.flat_bulk().wedge(RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5, 0, 0, 0 */
            float4(0.0)
        ));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
extension CircleRotor {
    // Fancy infix trick
    func operator/(_rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Scalar {
        return this.flat_bulk_norm_squared();
    }
}
extension CircleRotor: FlatBulkNormSquared {
    associatedtype Output = Scalar;
    func flat_bulk_norm_squared() -> Output {
        Sphere flat_bulk_thing = this.flat_bulk().wedge(RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5, 0, 0, 0 */
            float4(0.0)
        ));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
extension Dipole {
    // Fancy infix trick
    func operator/(_rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Scalar {
        return this.flat_bulk_norm_squared();
    }
}
extension Dipole: FlatBulkNormSquared {
    associatedtype Output = Scalar;
    func flat_bulk_norm_squared() -> Output {
        Line flat_bulk_thing = this.flat_bulk().wedge(RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5, 0, 0, 0 */
            float4(0.0)
        ));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
extension DipoleInversion {
    // Fancy infix trick
    func operator/(_rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Scalar {
        return this.flat_bulk_norm_squared();
    }
}
extension DipoleInversion: FlatBulkNormSquared {
    associatedtype Output = Scalar;
    func flat_bulk_norm_squared() -> Output {
        Motor flat_bulk_thing = this.flat_bulk().wedge(RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5, 0, 0, 0 */
            float4(0.0)
        ));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
extension DualNum {
    // Fancy infix trick
    func operator/(_rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Scalar {
        return this.flat_bulk_norm_squared();
    }
}
extension DualNum: FlatBulkNormSquared {
    associatedtype Output = Scalar;
    func flat_bulk_norm_squared() -> Output {
        FlatPoint flat_bulk_thing = this.flat_bulk().wedge(RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5, 0, 0, 0 */
            float4(0.0)
        ));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
extension FlatPoint {
    // Fancy infix trick
    func operator/(_rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Scalar {
        return this.flat_bulk_norm_squared();
    }
}
extension FlatPoint: FlatBulkNormSquared {
    associatedtype Output = Scalar;
    func flat_bulk_norm_squared() -> Output {
        Line flat_bulk_thing = this.flat_bulk().wedge(RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5, 0, 0, 0 */
            float4(0.0)
        ));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
extension Flector {
    // Fancy infix trick
    func operator/(_rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Scalar {
        return this.flat_bulk_norm_squared();
    }
}
extension Flector: FlatBulkNormSquared {
    associatedtype Output = Scalar;
    func flat_bulk_norm_squared() -> Output {
        Motor flat_bulk_thing = this.flat_bulk().wedge(RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5, 0, 0, 0 */
            float4(0.0)
        ));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
extension Line {
    // Fancy infix trick
    func operator/(_rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Scalar {
        return this.flat_bulk_norm_squared();
    }
}
extension Line: FlatBulkNormSquared {
    associatedtype Output = Scalar;
    func flat_bulk_norm_squared() -> Output {
        Sphere flat_bulk_thing = this.flat_bulk().wedge(RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5, 0, 0, 0 */
            float4(0.0)
        ));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
extension Motor {
    // Fancy infix trick
    func operator/(_rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Scalar {
        return this.flat_bulk_norm_squared();
    }
}
extension Motor: FlatBulkNormSquared {
    associatedtype Output = Scalar;
    func flat_bulk_norm_squared() -> Output {
        DipoleInversion flat_bulk_thing = this.flat_bulk().wedge(RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5, 0, 0, 0 */
            float4(0.0)
        ));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
extension MultiVector {
    // Fancy infix trick
    func operator/(_rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Scalar {
        return this.flat_bulk_norm_squared();
    }
}
extension MultiVector: FlatBulkNormSquared {
    associatedtype Output = Scalar;
    func flat_bulk_norm_squared() -> Output {
        MultiVector flat_bulk_thing = this.flat_bulk().wedge(RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5, 0, 0, 0 */
            float4(0.0)
        ));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
extension Plane {
    // Fancy infix trick
    func operator/(_rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Scalar {
        return this.flat_bulk_norm_squared();
    }
}
extension Plane: FlatBulkNormSquared {
    associatedtype Output = Scalar;
    func flat_bulk_norm_squared() -> Output {
        VersorEven flat_bulk_thing = this.flat_bulk().wedge(RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5, 0, 0, 0 */
            float4(0.0)
        ));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
extension RoundPoint {
    // Fancy infix trick
    func operator/(_rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Scalar {
        return this.flat_bulk_norm_squared();
    }
}
extension RoundPoint: FlatBulkNormSquared {
    associatedtype Output = Scalar;
    func flat_bulk_norm_squared() -> Output {
        FlatPoint flat_bulk_thing = this.flat_bulk().wedge(RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5, 0, 0, 0 */
            float4(0.0)
        ));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
extension Sphere {
    // Fancy infix trick
    func operator/(_rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Scalar {
        return this.flat_bulk_norm_squared();
    }
}
extension Sphere: FlatBulkNormSquared {
    associatedtype Output = Scalar;
    func flat_bulk_norm_squared() -> Output {
        VersorEven flat_bulk_thing = this.flat_bulk().wedge(RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5, 0, 0, 0 */
            float4(0.0)
        ));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
extension VersorEven {
    // Fancy infix trick
    func operator/(_rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Scalar {
        return this.flat_bulk_norm_squared();
    }
}
extension VersorEven: FlatBulkNormSquared {
    associatedtype Output = Scalar;
    func flat_bulk_norm_squared() -> Output {
        DipoleInversion flat_bulk_thing = this.flat_bulk().wedge(RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5, 0, 0, 0 */
            float4(0.0)
        ));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
extension VersorOdd {
    // Fancy infix trick
    func operator/(_rhs: FlatBulkNormSquaredPrefixOrPostfix) -> Scalar {
        return this.flat_bulk_norm_squared();
    }
}
extension VersorOdd: FlatBulkNormSquared {
    associatedtype Output = Scalar;
    func flat_bulk_norm_squared() -> Output {
        Motor flat_bulk_thing = this.flat_bulk().wedge(RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), 1.0), 
            /* e5, 0, 0, 0 */
            float4(0.0)
        ));
        return flat_bulk_thing.dot_product(flat_bulk_thing);
    }
}
