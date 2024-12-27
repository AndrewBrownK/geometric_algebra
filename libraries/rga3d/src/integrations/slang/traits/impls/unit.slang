extension AntiScalar: Unit {
    associatedtype Output = AntiScalar;
    func unit() -> Output {
        return AntiScalar.from_groups(
            /* e1234 */
            1.0
        );
    }
}
extension DualNum: Unit {
    associatedtype Output = DualNum;
    func unit() -> Output {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(1.0)
        );
    }
}
extension Flector: Unit {
    associatedtype Output = Flector;
    func unit() -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(1.0), 
            /* e423, e431, e412, e321 */
            float4(1.0)
        );
    }
}
extension Horizon: Unit {
    associatedtype Output = Horizon;
    func unit() -> Output {
        return Horizon.from_groups(
            /* e321 */
            1.0
        );
    }
}
extension Line: Unit {
    associatedtype Output = Line;
    func unit() -> Output {
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(1.0), 
            /* e23, e31, e12 */
            float3(1.0)
        );
    }
}
extension Motor: Unit {
    associatedtype Output = Motor;
    func unit() -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(1.0), 
            /* e23, e31, e12, scalar */
            float4(1.0)
        );
    }
}
extension MultiVector: Unit {
    associatedtype Output = MultiVector;
    func unit() -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(1.0), 
            /* e1, e2, e3, e4 */
            float4(1.0), 
            /* e41, e42, e43 */
            float3(1.0), 
            /* e23, e31, e12 */
            float3(1.0), 
            /* e423, e431, e412, e321 */
            float4(1.0)
        );
    }
}
extension Origin: Unit {
    associatedtype Output = Origin;
    func unit() -> Output {
        return Origin.from_groups(
            /* e4 */
            1.0
        );
    }
}
extension Plane: Unit {
    associatedtype Output = Plane;
    func unit() -> Output {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(1.0)
        );
    }
}
extension Point: Unit {
    associatedtype Output = Point;
    func unit() -> Output {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(1.0)
        );
    }
}
extension Scalar: Unit {
    associatedtype Output = Scalar;
    func unit() -> Output {
        return Scalar.from_groups(
            /* scalar */
            1.0
        );
    }
}
