impl std::ops::Div<support> for Flector {
    type Output = Line;
    fn div(self, _rhs: support) -> Self::Output {
        self.support()
    }
}
impl Support for Flector {
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        0        1        0
    fn support(self) -> Line {
        use crate::elements::*;
        return Line::from_groups(/* e41, e42, e43 */float4(float3(0.0), self[e321] * -1.0).xyz, /* e23, e31, e12 */float3(0.0));
    }
}
impl std::ops::Div<support> for MultiVector {
    type Output = Line;
    fn div(self, _rhs: support) -> Self::Output {
        self.support()
    }
}
impl Support for MultiVector {
// Operative Statistics for this implementation:
//      add/sub      mul      div
// f32        0        1        0
    fn support(self) -> Line {
        use crate::elements::*;
        return Line::from_groups(/* e41, e42, e43 */float4(float3(0.0), self[e321] * -1.0).xyz, /* e23, e31, e12 */float3(0.0));
    }
}
