impl AntiWedgeDot<MultiVector> for MultiVector {
    type Output = MultiVector;

    fn anti_wedge_dot(self, other: MultiVector) -> MultiVector {
        MultiVector {
            groups: MultiVectorGroups {



                g0:   Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group0()[1], 0.0]) * Simd32x2::from([1.0, 0.0])
                    + Simd32x2::from(self.group0()[1]) * other.group0()

                    + Simd32x2::from(self.group1()[0]) * Simd32x2::from([other.group4()[0], 0.0]) * Simd32x2::from([1.0, 0.0])
                    + Simd32x2::from(self.group1()[1]) * Simd32x2::from([other.group4()[1], 0.0]) * Simd32x2::from([1.0, 0.0])
                    + Simd32x2::from(self.group1()[2]) * Simd32x2::from([other.group4()[2], 0.0]) * Simd32x2::from([1.0, 0.0])
                    + Simd32x2::from(self.group1()[3]) * Simd32x2::from([other.group4()[3], other.group1()[3]]) * Simd32x2::from([1.0, -1.0])

                    + Simd32x2::from(self.group2()[0]) * Simd32x2::from([other.group3()[0], other.group2()[0]]) * Simd32x2::from(-1.0)
                    + Simd32x2::from(self.group2()[1]) * Simd32x2::from([other.group3()[1], other.group2()[1]]) * Simd32x2::from(-1.0)
                    + Simd32x2::from(self.group2()[2]) * Simd32x2::from([other.group3()[2], other.group2()[2]]) * Simd32x2::from(-1.0)

                    + Simd32x2::from(self.group3()[0]) * Simd32x2::from([other.group2()[0], 0.0]) * Simd32x2::from([-1.0, 0.0])
                    + Simd32x2::from(self.group3()[1]) * Simd32x2::from([other.group2()[1], 0.0]) * Simd32x2::from([-1.0, 0.0])
                    + Simd32x2::from(self.group3()[2]) * Simd32x2::from([other.group2()[2], 0.0]) * Simd32x2::from([-1.0, 0.0])

                    + Simd32x2::from(self.group4()[0]) * Simd32x2::from([other.group1()[0], other.group4()[0]]) * Simd32x2::from([-1.0, 1.0])
                    + Simd32x2::from(self.group4()[1]) * Simd32x2::from([other.group1()[1], other.group4()[1]]) * Simd32x2::from([-1.0, 1.0])
                    + Simd32x2::from(self.group4()[2]) * Simd32x2::from([other.group1()[2], other.group4()[2]]) * Simd32x2::from([-1.0, 1.0])
                    + Simd32x2::from(self.group4()[3]) * Simd32x2::from([other.group1()[3], 0.0]) * Simd32x2::from([-1.0, 0.0]),





                g1:   Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group4()[0], other.group4()[1], other.group4()[2], 0.0]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0])
                    + Simd32x4::from(self.group0()[1]) * other.group1()

                    + Simd32x4::from(self.group1()[0]) * Simd32x4::from([other.group0()[1], other.group2()[2], other.group2()[1], 0.0]) * Simd32x4::from([1.0, -1.0, 1.0, 0.0])
                    + Simd32x4::from(self.group1()[1]) * Simd32x4::from([other.group2()[2], other.group0()[1], other.group2()[0], 0.0]) * Simd32x4::from([1.0, 1.0, -1.0, 0.0])
                    + Simd32x4::from(self.group1()[2]) * Simd32x4::from([other.group2()[1], other.group2()[0], other.group0()[1], 0.0]) * Simd32x4::from([-1.0, 1.0, 1.0, 0.0])
                    + Simd32x4::from(self.group1()[3]) * Simd32x4::from([other.group3()[0], other.group3()[1], other.group3()[2], other.group0()[1]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0])

                    + Simd32x4::from(self.group2()[0]) * Simd32x4::from([other.group4()[3], other.group1()[2], other.group1()[1], other.group4()[0]]) * Simd32x4::from([1.0, -1.0, 1.0, -1.0])
                    + Simd32x4::from(self.group2()[1]) * Simd32x4::from([other.group1()[2], other.group4()[3], other.group1()[0], other.group4()[1]]) * Simd32x4::from([1.0, 1.0, -1.0, -1.0])
                    + Simd32x4::from(self.group2()[2]) * Simd32x4::from([other.group1()[1], other.group1()[0], other.group4()[3], other.group4()[2]]) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0])

                    + Simd32x4::from(self.group3()[0]) * Simd32x4::from([other.group1()[3], other.group4()[2], other.group4()[1], 0.0]) * Simd32x4::from([1.0, -1.0, 1.0, 0.0])
                    + Simd32x4::from(self.group3()[1]) * Simd32x4::from([other.group4()[2], other.group1()[3], other.group4()[0], 0.0]) * Simd32x4::from([1.0, 1.0, -1.0, 0.0])
                    + Simd32x4::from(self.group3()[2]) * Simd32x4::from([other.group4()[1], other.group4()[0], other.group1()[3], 0.0]) * Simd32x4::from([-1.0, 1.0, 1.0, 0.0])

                    + Simd32x4::from(self.group4()[0]) * Simd32x4::from([other.group0()[0], other.group3()[2], other.group3()[1], other.group2()[0]]) * Simd32x4::from([-1.0, 1.0, -1.0, -1.0])
                    + Simd32x4::from(self.group4()[1]) * Simd32x4::from([other.group3()[2], other.group0()[0], other.group3()[0], other.group2()[1]]) * Simd32x4::from([-1.0, -1.0, 1.0, -1.0])
                    + Simd32x4::from(self.group4()[2]) * Simd32x4::from([other.group3()[1], other.group3()[0], other.group0()[0], other.group2()[2]]) * Simd32x4::from([1.0, -1.0, -1.0, -1.0])
                    + Simd32x4::from(self.group4()[3]) * Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], 0.0]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),





                g2:   Simd32x3::from(self.group0()[1]) * other.group2()
                    + Simd32x3::from(self.group1()[3]) * Simd32x3::from([other.group4()[0], other.group4()[1], other.group4()[2]]) * Simd32x3::from(-1.0)

                    + Simd32x3::from(self.group2()[0]) * Simd32x3::from([other.group0()[1], other.group2()[2], other.group2()[1]]) * Simd32x3::from([1.0, -1.0, 1.0])
                    + Simd32x3::from(self.group2()[1]) * Simd32x3::from([other.group2()[2], other.group0()[1], other.group2()[0]]) * Simd32x3::from([1.0, 1.0, -1.0])
                    + Simd32x3::from(self.group2()[2]) * Simd32x3::from([other.group2()[1], other.group2()[0], other.group0()[1]]) * Simd32x3::from([-1.0, 1.0, 1.0])

                    + Simd32x3::from(self.group4()[0]) * Simd32x3::from([other.group1()[3], other.group4()[2], other.group4()[1]]) * Simd32x3::from([-1.0, 1.0, -1.0])
                    + Simd32x3::from(self.group4()[1]) * Simd32x3::from([other.group4()[2], other.group1()[3], other.group4()[0]]) * Simd32x3::from([-1.0, -1.0, 1.0])
                    + Simd32x3::from(self.group4()[2]) * Simd32x3::from([other.group4()[1], other.group4()[0], other.group1()[3]]) * Simd32x3::from([1.0, -1.0, -1.0]),




                g3:   Simd32x3::from(self.group0()[0]) * other.group2()
                    + Simd32x3::from(self.group0()[1]) * other.group3()

                    + Simd32x3::from(self.group1()[0]) * Simd32x3::from([other.group1()[3], other.group4()[2], other.group4()[1]]) * Simd32x3::from([-1.0, 1.0, -1.0])
                    + Simd32x3::from(self.group1()[1]) * Simd32x3::from([other.group4()[2], other.group1()[3], other.group4()[0]]) * Simd32x3::from([-1.0, -1.0, 1.0])
                    + Simd32x3::from(self.group1()[2]) * Simd32x3::from([other.group4()[1], other.group4()[0], other.group1()[3]]) * Simd32x3::from([1.0, -1.0, -1.0])
                    + Simd32x3::from(self.group1()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]])

                    + Simd32x3::from(self.group2()[0]) * Simd32x3::from([other.group0()[0], other.group3()[2], other.group3()[1]]) * Simd32x3::from([1.0, -1.0, 1.0])
                    + Simd32x3::from(self.group2()[1]) * Simd32x3::from([other.group3()[2], other.group0()[0], other.group3()[0]]) * Simd32x3::from([1.0, 1.0, -1.0])
                    + Simd32x3::from(self.group2()[2]) * Simd32x3::from([other.group3()[1], other.group3()[0], other.group0()[0]]) * Simd32x3::from([-1.0, 1.0, 1.0])

                    + Simd32x3::from(self.group3()[0]) * Simd32x3::from([other.group0()[1], other.group2()[2], other.group2()[1]]) * Simd32x3::from([1.0, -1.0, 1.0])
                    + Simd32x3::from(self.group3()[1]) * Simd32x3::from([other.group2()[2], other.group0()[1], other.group2()[0]]) * Simd32x3::from([1.0, 1.0, -1.0])
                    + Simd32x3::from(self.group3()[2]) * Simd32x3::from([other.group2()[1], other.group2()[0], other.group0()[1]]) * Simd32x3::from([-1.0, 1.0, 1.0])

                    + Simd32x3::from(self.group4()[0]) * Simd32x3::from([other.group4()[3], other.group1()[2], other.group1()[1]]) * Simd32x3::from([1.0, -1.0, 1.0])
                    + Simd32x3::from(self.group4()[1]) * Simd32x3::from([other.group1()[2], other.group4()[3], other.group1()[0]]) * Simd32x3::from([1.0, 1.0, -1.0])
                    + Simd32x3::from(self.group4()[2]) * Simd32x3::from([other.group1()[1], other.group1()[0], other.group4()[3]]) * Simd32x3::from([-1.0, 1.0, 1.0])
                    + Simd32x3::from(self.group4()[3]) * Simd32x3::from([other.group4()[0], other.group4()[1], other.group4()[2]]) * Simd32x3::from(-1.0),




                g4:   Simd32x4::from(self.group0()[0]) * Simd32x4::from([0.0, 0.0, 0.0, other.group1()[3]]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0])
                    + Simd32x4::from(self.group0()[1]) * other.group4()

                    + Simd32x4::from(self.group1()[0]) * Simd32x4::from([0.0, 0.0, 0.0, other.group2()[0]]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0])
                    + Simd32x4::from(self.group1()[1]) * Simd32x4::from([0.0, 0.0, 0.0, other.group2()[1]]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0])
                    + Simd32x4::from(self.group1()[2]) * Simd32x4::from([0.0, 0.0, 0.0, other.group2()[2]]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0])
                    + Simd32x4::from(self.group1()[3]) * Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], other.group0()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0])

                    + Simd32x4::from(self.group2()[0]) * Simd32x4::from([other.group1()[3], other.group4()[2], other.group4()[1], other.group1()[0]]) * Simd32x4::from([1.0, -1.0, 1.0, -1.0])
                    + Simd32x4::from(self.group2()[1]) * Simd32x4::from([other.group4()[2], other.group1()[3], other.group4()[0], other.group1()[1]]) * Simd32x4::from([1.0, 1.0, -1.0, -1.0])
                    + Simd32x4::from(self.group2()[2]) * Simd32x4::from([other.group4()[1], other.group4()[0], other.group1()[3], other.group1()[2]]) * Simd32x4::from([-1.0, 1.0, 1.0, -1.0])

                    + Simd32x4::from(self.group3()[0]) * Simd32x4::from([0.0, 0.0, 0.0, other.group4()[0]]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0])
                    + Simd32x4::from(self.group3()[1]) * Simd32x4::from([0.0, 0.0, 0.0, other.group4()[1]]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0])
                    + Simd32x4::from(self.group3()[2]) * Simd32x4::from([0.0, 0.0, 0.0, other.group4()[2]]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0])

                    + Simd32x4::from(self.group4()[0]) * Simd32x4::from([other.group0()[1], other.group2()[2], other.group2()[1], other.group3()[0]]) * Simd32x4::from([1.0, -1.0, 1.0, 1.0])
                    + Simd32x4::from(self.group4()[1]) * Simd32x4::from([other.group2()[2], other.group0()[1], other.group2()[0], other.group3()[1]]) * Simd32x4::from([1.0, 1.0, -1.0, 1.0])
                    + Simd32x4::from(self.group4()[2]) * Simd32x4::from([other.group2()[1], other.group2()[0], other.group0()[1], other.group3()[2]]) * Simd32x4::from([-1.0, 1.0, 1.0, 1.0])
                    + Simd32x4::from(self.group4()[3]) * Simd32x4::from([0.0, 0.0, 0.0, other.group0()[1]]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0])
            }
        }
    }
}
