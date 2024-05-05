use std::collections::BTreeMap;
use std::io::Write;

use crate::algebra::dialect::Dialect;
use crate::algebra::GeometricAlgebraTrait;
use crate::build_scripts::cga::cga_script;

const CGA3D: &str = "cga3d";
const CGA3D_CRATE_PREFIX: &str = "cga3d/";

pub fn script() -> std::io::Result<()> {
    script_custom(true, CGA3D_CRATE_PREFIX)
}

//noinspection DuplicatedCode
fn script_custom(actually_emit: bool, path_prefix: &str) -> std::io::Result<()> {

    let mv_iter = [
        "Scalar:1",
        "AntiScalar:e12345",
        "DualNum:1,e12345",

        "FlatPoint:e15,e25,e35,e45",
        "FlatPointAtOrigin:e45",
        "FlatPointAtInfinity:e15,e25,e35",

        "Line:e415,e425,e435|e235,e315,e125",
        "LineAtOrigin:e415,e425,e435",
        "LineAtInfinity:e235,e315,e125",

        "Plane:e4235,e4315,e4125,e3215",
        "PlaneAtOrigin:e4235,e4315,e4125",
        "Horizon:e3215",

        "RoundPoint:e1,e2,e3|e4,e5",
        "Dipole:e41,e42,e43|e23,e31,e12|e15,e25,e35,e45",
        "Circle:e423,e431,e412,e321|e415,e425,e435|e235,e315,e125",
        "Sphere:e4235,e4315,e4125|e1234,e3215",

        // TODO infinity is interesting when you consider different SpacialCurvatures
        //  In a positively curved space, e5 is not infinitely far away, it is "the most distant
        //  point away from the origin". Consider by analogy rga3d... What is the difference between
        //  cga3d's "Infinity" and rga3d's "PointAtInfinity" or "Horizon"? An e4 with a coefficient
        //  but not other basis elements beside it is always the Origin. So what is e4+e5, and
        //  then e5? Does it even translate into rga3d at all? Well, perhaps that brings us
        //  to RoundPointAtOrigin.
        // TODO maybe the correct starting point to think through it is....
        //  The equivalence classes of a vector in rga3d... it's one dimensional
        //  But the equivalence classes of a vector in cga3d is fundamentally two dimensional
        //  Because of the cone where e- and e+ cancel out. This is further reinforced when you
        //  consider that a FlatPoint forfeits 1/2 of it's equivalence class dimensions by always
        //  being aligned with e5.
        //  So assume you have a RoundPoint of zero radius... it traces a parabola (p^2)/2
        //  on the e1 e5 plane (assuming p is along e1 for this example). In rga3d, there are only
        //  two meaningful coefficients for e4: zero and non-zero. All non-zero coefficients to e4
        //  might as well mean the same thing. But I feel like it's not as simple in cga3d.
        //  So taking this zero-radius round point along e1 (that traces a parabola), we can slide
        //  along the equivalence class of e4 however we want, and it stays a parabola, unless we
        //  break the equivalence by hard setting e4 to zero. Then it is no longer a parabola,
        //  but indeed, a line along e5. But it's still on the cone. So there's still an equivalence
        //  class here, but it is narrowed to one dimension, that being e5. So a round point with
        //  zero-radius and zero-e4 and non-zero-e5 is.... well... it's a point at infinity...
        //  but it's infinity because e4 is zero, not because e5 is non-zero. So what is e5 actually
        //  doing here? It's interacting with the radius, yeah? 0*e4 + e5 might be infinity, but
        //  it's not infinity in any particular direction if we don't also have e1, e2, or e3. So
        //  that's why it's weird. It's infinity "but where?". Hence the contrast with
        //  PointAtInfinity and Horizon.
        //  Let's approach from the other side. non-zero e4, and zero e5. If p is zero, then radius
        //  is zero. So e4 is quite plainly the Origin with zero radius. But then if p is non-zero,
        //  but e5 is still zero? (p^2) is almost certainly positive. So (r^2) must be negative.
        //  That would be an imaginary radius the same size as the position. In other words...
        //  The round point is still touching the origin I think. Indeed.... the flat bulk of a
        //  RoundPoint is the e5 component. Hence we now rename "RoundPointCarrierAspect" to
        //  "RoundPointTouchingOrigin". Come to think of it, maybe this gives us a hint how to
        //  figure out the meaning of 0*e4 + 0*e5, by looking at "RoundPointBulk". That's a RoundBulk.
        //  (It's a slightly risky rename, but going to call "RoundPointTouchingOrigin"
        //   "RoundPointOnOrigin" instead. The "on" vs "at" distinction is subtle, but I
        //   think it makes distinguishable sense at the semantic level (RIP victims of dyslexia)
        //   The full word "Touching" is just a bit cumbersome.).
        //  Alright... so now we know what a RoundPointOnOrigin is, and how to hard zero e4 to
        //  jump equivalence classes..... so what does that mean for RoundPointBulk?
        //  Well... it's a point at infinity... but with a round radius of imaginary infinity.
        //  Apparently. Bisecting space at the origin. So... actually, the amount of space that is
        //  zero distance from this thing is quite astounding.
        //
        // TODO I've been thinking through it with dipoles, since you can grasp the e4 and e5
        //  dimensions perfectly well with a 1 dimensional dipole. So imagine a dipole centered
        //  on p=(0,0,0) and in the direction of n=e1 with some non-zero radius. The only
        //  resulting components are e41 and e15. With a real radius, it intersects the cones.
        //  With an imaginary radius, it doesn't. It is very easy to imagine e41 and e15, and how
        //  they add together, and how they intersect the horosphere. So with this in mind we can
        //  see what "rotating" the bivector between e4 and e5 actually means. And so perhaps this
        //  gives us a new insight. Tentatively.... e4 is not merely the "Origin", and e5 is not
        //  merely "Infinity". e4 is "CenterOrigin" and e5 is "RadiusInfinity". Since imagining
        //  bivectors is so easy, this goes for FlatPoints as well. The closer a bivector is aligned
        //  to e5, the closer its radius is to infinity. The closer a bivector is aligned to e4,
        //  the closer its center is aligned to the origin. This actually makes even more sense
        //  when you imagine taking a raw rga3d point and say MULTIPLY THAT BY INFINITE RADIUS and
        //  it just works... However keep in mind.... the radius doesn't get multiplied into e5
        //  directly.... it is the radius squared that gets put into the e5 term.... in other words,
        //  the reciprocal of gaussian curvature. Aha.... called Gaussian radius. Lovely.
        //
        // TODO okay! So here's another thought.... For a moment I was considering renaming "Origin"
        //  to "CenterOrigin" in contrast to "RadiusInfinity"... however I don't think "CenterOrigin"
        //  is quite an effective/accurate name yet. It's more accurate to call it "PositionOrigin"
        //  or, yet again, just plain old "Origin". This is because something can have a center on
        //  the origin without actually overlapping e4, like a dipole centered on the origin with
        //  any non-zero radius. The closer the dipole gets to the origin (by distance of its
        //  endpoints, not its center), the closer it gets to overlapping e4.
        //
        // TODO another idea to help analysis.... forget the e1 axis entirely. We have such a good
        //  grip on Dipoles and FlatPoints now. What happens if we ignore the e1 dimension completely
        //  and just focus on how the bivectors (that are now vectors, perhaps?) meet into the e45
        //  bivector. FlatPoints become "raw infinite radius" without any spacial positioning
        //  (unless they were on the origin already). A dipole with (at least) one endpoint on
        //  the origin becomes synonymous with the origin. A dipole that is not centered on the
        //  origin, but puts the origin somewhere between its endpoints, will meet e45 as a vector
        //  angled somewhere between e4 and e5, the angle depending mostly on the radius of the
        //  dipole and partially on the distance to the origin. Then there are dipoles not centered
        //  on the origin but also have both endpoints to one side of the origin. These do not
        //  intersect the e45 bivector at all. Or rather, their meets are imaginary, not real. There
        //  is arguably one more type of bivector intersection/meet with e45... that being...
        //  e+ join e1 without any e- component. Well... that's just an imaginary radius dipole. So,
        //  whatever that means.
        //
        // TODO something to keep in mind is once we start using non-flat curvatures (like
        //  spherically curved space) then e5 will no longer be "InfinityRadius". It will instead
        //  be "MaximumRadius". So that's something to keep in mind.
        //
        "Infinity:e5", // TODO hint to not lose track in above todo: considering rename of e5 from "Infinity" to something else
        "Origin:e4",
        "RoundPointAtOrigin:e4,e5",
        "RoundPointOnOrigin:e1,e2,e3,e4",
        // TODO RoundPointAtInfinity is extremely weird, can't be sure yet if it actually works
        "RoundPointAtInfinity:e1,e2,e3,e5",

        // TODO I found/noticed this object as the Attitude of a Circle.
        //  The carrier line is a LineAtInfinity. Therefore both its center
        //  and its endpoints are in infinity. This makes me wonder if there should
        //  be yet another (third) conjunctive distinction. "at" is centered on something,
        //  "on" is zero distance from something, and "in" could be both at and on at the same
        //  time. Because you could very well imagine a DipoleOnInfinity (well I guess that is
        //  just FlatPoints) or a Dipole centered on infinity but with a finite endpoint.... maybe..
        //  So I don't know exactly how many variants I'll need. I'll just start wit this
        //  and see where it takes me. I do like the "In" conjunction as it relates to the carrier
        //  though.
        "DipoleAtInfinity:e23,e31,e12|e15,e25,e35",

        // TODO CircleAtInfinity is similar story to DipoleAtInfinity, discovered as
        //  Attitude of Sphere
        "CircleAtInfinity:e321,e235,e315,e125",


        // TODO can I get more interesting/intuitive names for these?
        "RoundPointBulk:e1,e2,e3",
        "DipoleBulk:e23,e31,e12",
        "DipoleWeight:e41,e42,e43",
        "DipoleCarrierAspect:e41,e42,e43|e23,e31,e12",
        "CircleBulk:e321",
        "CircleWeight:e423,e431,e412",
        "CircleCarrierAspect:e423,e431,e412,e321",
        "SphereWeight:e1234",
        "SpacialCurvature:e1234,e3215",


        // Operator Objects
        "Motor:e415,e425,e435,e12345|e235,e315,e125",
        "Rotor:e415,e425,e435,e12345",
        "Translator:e235,e315,e125,e12345",
        "Flector:e15,e25,e35,e45|e4235,e4315,e4125,e3215",
        "Transflector:e15,e25,e35|e4235,e4315,e4125,e3215",
        "FlectorAtInfinity:e15,e25,e35,e3215",

        "MultiVector:\
            1,e12345|\
            e1,e2,e3|e4,e5|\
            e41,e42,e43|e23,e31,e12|e15,e25,e35,e45|\
            e423,e431,e412,e321|e415,e425,e435|e235,e315,e125|\
            e4235,e4315,e4125|e1234,e3215",
    ];

    // TODO add more of these if/where applicable to CGA
    let sandwich_outputs: BTreeMap<(&str, &str), &str> = [

        ("Translator", "Origin", "Point"),
        ("Translator", "LineAtOrigin", "Line"),
        ("Translator", "PlaneAtOrigin", "Plane"),
        ("Translator", "Rotor", "Motor"),

        ("Motor", "Origin", "Point"),
        ("Motor", "LineAtOrigin", "Line"),
        ("Motor", "PlaneAtOrigin", "Plane"),
        ("Motor", "Rotor", "Motor"),

        ("Flector", "Origin", "Point"),
        ("Flector", "LineAtOrigin", "Line"),
        ("Flector", "PlaneAtOrigin", "Plane"),
        ("Flector", "Rotor", "Motor"),

        ("FlectorAtInfinity", "Origin", "Point"),
        ("FlectorAtInfinity", "LineAtOrigin", "Line"),
        ("FlectorAtInfinity", "PlaneAtOrigin", "Plane"),
        ("FlectorAtInfinity", "Rotor", "Motor"),

    ].into_iter().map(|it| ((it.0, it.1), it.2)).collect();

    // Arbitrary personal preference for dialect
    let dialect = Dialect::default().also_wedge_dot().wedge().dot().also_meet_and_join();

    cga_script(
        path_prefix,
        CGA3D,
        dialect,
        3,
        actually_emit,
        &mv_iter,
        sandwich_outputs,
    )?;


    Ok(())
}


#[cfg(test)]
mod test {
    use std::path::Path;
    use crate::build_scripts::cga3d::{CGA3D, CGA3D_CRATE_PREFIX, script_custom};
    use crate::validate::{validate_glsl, validate_wgsl};

    #[test]
    fn build_without_disk_writes() {
        script_custom(false, CGA3D_CRATE_PREFIX).unwrap()
    }

    #[test]
    fn glsl_validation() {
        let file_path = Path::new("../")
            .join(Path::new(CGA3D_CRATE_PREFIX))
            .join(Path::new("src/shaders/")
                .join(CGA3D));
        validate_glsl(CGA3D, file_path);
    }

    #[test]
    fn wgsl_validation() {
        let file_path = Path::new("../")
            .join(Path::new(CGA3D_CRATE_PREFIX))
            .join(Path::new("src/shaders/")
                .join(CGA3D));
        validate_wgsl(CGA3D, file_path);
    }
}