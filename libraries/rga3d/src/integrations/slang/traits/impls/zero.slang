extension AntiScalar: Zero {
    associatedtype Output = AntiScalar;
    func zero() -> Output {
        return AntiScalar.from_groups(
            /* e1234 */
            0.0
        );
    }
}
extension DualNum: Zero {
    associatedtype Output = DualNum;
    func zero() -> Output {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(0.0)
        );
    }
}
extension Flector: Zero {
    associatedtype Output = Flector;
    func zero() -> Output {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
extension Horizon: Zero {
    associatedtype Output = Horizon;
    func zero() -> Output {
        return Horizon.from_groups(
            /* e321 */
            0.0
        );
    }
}
extension Line: Zero {
    associatedtype Output = Line;
    func zero() -> Output {
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
    }
}
extension Motor: Zero {
    associatedtype Output = Motor;
    func zero() -> Output {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(0.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
extension MultiVector: Zero {
    associatedtype Output = MultiVector;
    func zero() -> Output {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
extension Origin: Zero {
    associatedtype Output = Origin;
    func zero() -> Output {
        return Origin.from_groups(
            /* e4 */
            0.0
        );
    }
}
extension Plane: Zero {
    associatedtype Output = Plane;
    func zero() -> Output {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
extension Point: Zero {
    associatedtype Output = Point;
    func zero() -> Output {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(0.0)
        );
    }
}
extension Scalar: Zero {
    associatedtype Output = Scalar;
    func zero() -> Output {
        return Scalar.from_groups(
            /* scalar */
            0.0
        );
    }
}
