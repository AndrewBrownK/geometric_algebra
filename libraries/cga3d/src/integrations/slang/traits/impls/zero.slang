extension AntiCircleRotor: Zero {
    associatedtype Output = AntiCircleRotor;
    func zero() -> Output {
        return AntiCircleRotor.from_groups(
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, e45 */
            float4(0.0), 
            /* e15, e25, e35, scalar */
            float4(0.0)
        );
    }
}
extension AntiDipoleInversion: Zero {
    associatedtype Output = AntiDipoleInversion;
    func zero() -> Output {
        return AntiDipoleInversion.from_groups(
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e4 */
            float4(0.0), 
            /* e1, e2, e3, e5 */
            float4(0.0)
        );
    }
}
extension AntiDualNum: Zero {
    associatedtype Output = AntiDualNum;
    func zero() -> Output {
        return AntiDualNum.from_groups(
            /* e3215, scalar, 0, 0 */
            float4(0.0)
        );
    }
}
extension AntiFlatPoint: Zero {
    associatedtype Output = AntiFlatPoint;
    func zero() -> Output {
        return AntiFlatPoint.from_groups(
            /* e235, e315, e125, e321 */
            float4(0.0)
        );
    }
}
extension AntiFlector: Zero {
    associatedtype Output = AntiFlector;
    func zero() -> Output {
        return AntiFlector.from_groups(
            /* e235, e315, e125, e321 */
            float4(0.0), 
            /* e1, e2, e3, e5 */
            float4(0.0)
        );
    }
}
extension AntiLine: Zero {
    associatedtype Output = AntiLine;
    func zero() -> Output {
        return AntiLine.from_groups(
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e15, e25, e35, 0 */
            float4(0.0)
        );
    }
}
extension AntiMotor: Zero {
    associatedtype Output = AntiMotor;
    func zero() -> Output {
        return AntiMotor.from_groups(
            /* e23, e31, e12, scalar */
            float4(0.0), 
            /* e15, e25, e35, e3215 */
            float4(0.0)
        );
    }
}
extension AntiPlane: Zero {
    associatedtype Output = AntiPlane;
    func zero() -> Output {
        return AntiPlane.from_groups(
            /* e1, e2, e3, e5 */
            float4(0.0)
        );
    }
}
extension AntiScalar: Zero {
    associatedtype Output = AntiScalar;
    func zero() -> Output {
        return AntiScalar.from_groups(
            /* e12345, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension Circle: Zero {
    associatedtype Output = Circle;
    func zero() -> Output {
        return Circle.from_groups(
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(0.0)
        );
    }
}
extension CircleRotor: Zero {
    associatedtype Output = CircleRotor;
    func zero() -> Output {
        return CircleRotor.from_groups(
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e12345 */
            float4(0.0)
        );
    }
}
extension Dipole: Zero {
    associatedtype Output = Dipole;
    func zero() -> Output {
        return Dipole.from_groups(
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, e45 */
            float4(0.0), 
            /* e15, e25, e35, 0 */
            float4(0.0)
        );
    }
}
extension DipoleInversion: Zero {
    associatedtype Output = DipoleInversion;
    func zero() -> Output {
        return DipoleInversion.from_groups(
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, e45 */
            float4(0.0), 
            /* e15, e25, e35, e1234 */
            float4(0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0)
        );
    }
}
extension DualNum: Zero {
    associatedtype Output = DualNum;
    func zero() -> Output {
        return DualNum.from_groups(
            /* e5, e12345, 0, 0 */
            float4(0.0)
        );
    }
}
extension FlatPoint: Zero {
    associatedtype Output = FlatPoint;
    func zero() -> Output {
        return FlatPoint.from_groups(
            /* e15, e25, e35, e45 */
            float4(0.0)
        );
    }
}
extension Flector: Zero {
    associatedtype Output = Flector;
    func zero() -> Output {
        return Flector.from_groups(
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0)
        );
    }
}
extension Line: Zero {
    associatedtype Output = Line;
    func zero() -> Output {
        return Line.from_groups(
            /* e415, e425, e435, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(0.0)
        );
    }
}
extension Motor: Zero {
    associatedtype Output = Motor;
    func zero() -> Output {
        return Motor.from_groups(
            /* e415, e425, e435, e12345 */
            float4(0.0), 
            /* e235, e315, e125, e5 */
            float4(0.0)
        );
    }
}
extension MultiVector: Zero {
    associatedtype Output = MultiVector;
    func zero() -> Output {
        return MultiVector.from_groups(
            /* scalar, e12345, 0, 0 */
            float4(0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0), 
            /* e15, e25, e35, e45 */
            float4(0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e423, e431, e412, 0 */
            float4(0.0), 
            /* e235, e315, e125, 0 */
            float4(0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension Plane: Zero {
    associatedtype Output = Plane;
    func zero() -> Output {
        return Plane.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(0.0)
        );
    }
}
extension RoundPoint: Zero {
    associatedtype Output = RoundPoint;
    func zero() -> Output {
        return RoundPoint.from_groups(
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e5, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension Scalar: Zero {
    associatedtype Output = Scalar;
    func zero() -> Output {
        return Scalar.from_groups(
            /* scalar, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension Sphere: Zero {
    associatedtype Output = Sphere;
    func zero() -> Output {
        return Sphere.from_groups(
            /* e4235, e4315, e4125, e3215 */
            float4(0.0), 
            /* e1234, 0, 0, 0 */
            float4(0.0)
        );
    }
}
extension VersorEven: Zero {
    associatedtype Output = VersorEven;
    func zero() -> Output {
        return VersorEven.from_groups(
            /* e423, e431, e412, e12345 */
            float4(0.0), 
            /* e415, e425, e435, e321 */
            float4(0.0), 
            /* e235, e315, e125, e5 */
            float4(0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0)
        );
    }
}
extension VersorOdd: Zero {
    associatedtype Output = VersorOdd;
    func zero() -> Output {
        return VersorOdd.from_groups(
            /* e41, e42, e43, scalar */
            float4(0.0), 
            /* e23, e31, e12, e45 */
            float4(0.0), 
            /* e15, e25, e35, e1234 */
            float4(0.0), 
            /* e4235, e4315, e4125, e3215 */
            float4(0.0)
        );
    }
}
