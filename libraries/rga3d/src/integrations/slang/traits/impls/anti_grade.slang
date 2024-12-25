impl AntiGrade for AntiScalar {
    fn anti_grade() -> uint {
        return 0;
    }
}
impl AntiGrade for Horizon {
    fn anti_grade() -> uint {
        return 1;
    }
}
impl AntiGrade for Line {
    fn anti_grade() -> uint {
        return 2;
    }
}
impl AntiGrade for Origin {
    fn anti_grade() -> uint {
        return 3;
    }
}
impl AntiGrade for Plane {
    fn anti_grade() -> uint {
        return 1;
    }
}
impl AntiGrade for Point {
    fn anti_grade() -> uint {
        return 3;
    }
}
impl AntiGrade for Scalar {
    fn anti_grade() -> uint {
        return 4;
    }
}
