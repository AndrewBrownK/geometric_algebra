extension AntiCircleRotor: Unit {
    associatedtype Output = AntiCircleRotor;
    func unit() -> Output {
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(1.0), 0.0), 
            /* e23, e31, e12, e45 */
            float4(1.0), 
            /* e15, e25, e35, scalar */
            float4(1.0)
        );
    }
}
extension AntiDipoleInversion: Unit {
    associatedtype Output = AntiDipoleInversion;
    func unit() -> Output {
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(1.0), 0.0), 
            /* e415, e425, e435, e321 */
            float4(1.0), 
            /* e235, e315, e125, e4 */
            float4(1.0), 
            /* e1, e2, e3, e5 */
            float4(1.0)
        );
    }
}
extension AntiDualNum: Unit {
    associatedtype Output = AntiDualNum;
    func unit() -> Output {
        return AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(float2(1.0), 0.0, 0.0)
        );
    }
}
extension AntiFlatPoint: Unit {
    associatedtype Output = AntiFlatPoint;
    func unit() -> Output {
        return AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            float4(1.0)
        );
    }
}
extension AntiFlector: Unit {
    associatedtype Output = AntiFlector;
    func unit() -> Output {
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(1.0), 
            /* e1, e2, e3, e5 */
            float4(1.0)
        );
    }
}
extension AntiLine: Unit {
    associatedtype Output = AntiLine;
    func unit() -> Output {
        return AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(float3(1.0), 0.0), 
            /* e15, e25, e35, 0 */
            float4(float3(1.0), 0.0)
        );
    }
}
extension AntiMotor: Unit {
    associatedtype Output = AntiMotor;
    func unit() -> Output {
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(1.0), 
            /* e15, e25, e35, e3215 */
            float4(1.0)
        );
    }
}
extension AntiPlane: Unit {
    associatedtype Output = AntiPlane;
    func unit() -> Output {
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4(1.0)
        );
    }
}
extension AntiScalar: Unit {
    associatedtype Output = AntiScalar;
    func unit() -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension Circle: Unit {
    associatedtype Output = Circle;
    func unit() -> Output {
        return Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(1.0), 0.0), 
            /* e415, e425, e435, e321 */
            float4(1.0), 
            /* e235, e315, e125, 0 */
            float4(float3(1.0), 0.0)
        );
    }
}
extension CircleRotor: Unit {
    associatedtype Output = CircleRotor;
    func unit() -> Output {
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(float3(1.0), 0.0), 
            /* e415, e425, e435, e321 */
            float4(1.0), 
            /* e235, e315, e125, e12345 */
            float4(1.0)
        );
    }
}
extension Dipole: Unit {
    associatedtype Output = Dipole;
    func unit() -> Output {
        return Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(1.0), 0.0), 
            /* e23, e31, e12, e45 */
            float4(1.0), 
            /* e15, e25, e35, 0 */
            float4(float3(1.0), 0.0)
        );
    }
}
extension DipoleInversion: Unit {
    associatedtype Output = DipoleInversion;
    func unit() -> Output {
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(float3(1.0), 0.0), 
            /* e23, e31, e12, e45 */
            float4(1.0), 
            /* e15, e25, e35, e1234 */
            float4(1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(1.0)
        );
    }
}
extension DualNum: Unit {
    associatedtype Output = DualNum;
    func unit() -> Output {
        return DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(float2(1.0), 0.0, 0.0)
        );
    }
}
extension FlatPoint: Unit {
    associatedtype Output = FlatPoint;
    func unit() -> Output {
        return FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            float4(1.0)
        );
    }
}
extension Flector: Unit {
    associatedtype Output = Flector;
    func unit() -> Output {
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(1.0)
        );
    }
}
extension Line: Unit {
    associatedtype Output = Line;
    func unit() -> Output {
        return Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(float3(1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(1.0), 0.0)
        );
    }
}
extension Motor: Unit {
    associatedtype Output = Motor;
    func unit() -> Output {
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(1.0), 
            /* e235, e315, e125, e5 */
            float4(1.0)
        );
    }
}
extension MultiVector: Unit {
    associatedtype Output = MultiVector;
    func unit() -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(float2(1.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(1.0), 
            /* e5, 0, 0, 0 */
            float4(1.0, 0.0, 0.0, 0.0), 
            /* e15, e25, e35, e45 */
            float4(1.0), 
            /* e41, e42, e43, 0 */
            float4(float3(1.0), 0.0), 
            /* e23, e31, e12, 0 */
            float4(float3(1.0), 0.0), 
            /* e415, e425, e435, e321 */
            float4(1.0), 
            /* e423, e431, e412, 0 */
            float4(float3(1.0), 0.0), 
            /* e235, e315, e125, 0 */
            float4(float3(1.0), 0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(1.0), 
            /* e1234, 0, 0, 0 */
            float4(1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension Plane: Unit {
    associatedtype Output = Plane;
    func unit() -> Output {
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(1.0)
        );
    }
}
extension RoundPoint: Unit {
    associatedtype Output = RoundPoint;
    func unit() -> Output {
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(1.0), 
            /* e5, 0, 0, 0 */
            float4(1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension Scalar: Unit {
    associatedtype Output = Scalar;
    func unit() -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension Sphere: Unit {
    associatedtype Output = Sphere;
    func unit() -> Output {
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(1.0), 
            /* e1234, 0, 0, 0 */
            float4(1.0, 0.0, 0.0, 0.0)
        );
    }
}
extension VersorEven: Unit {
    associatedtype Output = VersorEven;
    func unit() -> Output {
        return VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            float4(1.0), 
            /* e415, e425, e435, e321 */
            float4(1.0), 
            /* e235, e315, e125, e5 */
            float4(1.0), 
            /* e1, e2, e3, e4 */
            float4(1.0)
        );
    }
}
extension VersorOdd: Unit {
    associatedtype Output = VersorOdd;
    func unit() -> Output {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            float4(1.0), 
            /* e23, e31, e12, e45 */
            float4(1.0), 
            /* e15, e25, e35, e1234 */
            float4(1.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(1.0)
        );
    }
}
