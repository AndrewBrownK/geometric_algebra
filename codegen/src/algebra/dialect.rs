


#[derive(Clone)]
pub struct Dialect {
    // https://rigidgeometricalgebra.org/wiki/index.php?title=Geometric_products
    pub geometric_product: Vec<&'static str>,
    pub geometric_anti_product: Vec<&'static str>,

    // https://rigidgeometricalgebra.org/wiki/index.php?title=Exterior_products
    pub exterior_product: Vec<&'static str>,
    pub exterior_anti_product: Vec<&'static str>,

    // https://rigidgeometricalgebra.org/wiki/index.php?title=Dot_products
    pub dot_product: Vec<&'static str>,
    pub anti_dot_product: Vec<&'static str>,
}

impl Default for Dialect {
    fn default() -> Self {
        Dialect {
            geometric_product: vec!["GeometricProduct"],
            geometric_anti_product: vec!["GeometricAntiProduct"],

            exterior_product: vec!["ExteriorProduct"],
            exterior_anti_product: vec!["ExteriorAntiProduct"],

            dot_product: vec!["DotProduct"],
            anti_dot_product: vec!["AntiDotProduct"]
        }
    }
}

impl Dialect {
    pub fn traditional(self) -> Self {
        self.outer_and_regressive().scalar_product()
    }

    pub fn also_traditional(self) -> Self {
        self.also_outer_and_regressive().also_scalar_product()
    }

    pub fn symbolic(self) -> Self {
        self.wedge().dot().wedge_dot()
    }

    pub fn also_symbolic(self) -> Self {
        self.also_wedge_dot().also_wedge().also_dot()
    }

    pub fn outer_and_regressive(mut self) -> Self {
        self.exterior_product = vec!["OuterProduct"];
        self.exterior_anti_product = vec!["RegressiveProduct"];
        self
    }

    pub fn also_outer_and_regressive(mut self) -> Self {
        if !self.exterior_product.contains(&"OuterProduct") {
            self.exterior_product.push("OuterProduct");
        }
        if !self.exterior_anti_product.contains(&"RegressiveProduct") {
            self.exterior_anti_product.push("RegressiveProduct");
        }
        self
    }

    pub fn scalar_product(mut self) -> Self {
        self.dot_product.push("ScalarProduct");
        self.anti_dot_product.push("AntiScalarProduct");
        self
    }

    pub fn also_scalar_product(mut self) -> Self {
        if !self.dot_product.contains(&"ScalarProduct") {
            self.dot_product.push("ScalarProduct");
        }
        if !self.anti_dot_product.contains(&"AntiScalarProduct") {
            self.anti_dot_product.push("AntiScalarProduct");
        }
        self
    }

    pub fn wedge_dot(mut self) -> Self {
        self.geometric_product = vec!["WedgeDot"];
        self.geometric_anti_product = vec!["AntiWedgeDot"];
        self
    }

    pub fn also_wedge_dot(mut self) -> Self {
        if !self.geometric_product.contains(&"WedgeDot") {
            self.geometric_product.push("WedgeDot");
        }
        if !self.geometric_anti_product.contains(&"AntiWedgeDot") {
            self.geometric_anti_product.push("AntiWedgeDot");
        }
        self
    }

    pub fn wedge(mut self) -> Self {
        self.exterior_product = vec!["Wedge"];
        self.exterior_anti_product = vec!["AntiWedge"];
        self
    }

    pub fn also_wedge(mut self) -> Self {
        if !self.exterior_product.contains(&"Wedge") {
            self.exterior_product.push("Wedge");
        }
        if !self.exterior_anti_product.contains(&"AntiWedge") {
            self.exterior_anti_product.push("AntiWedge");
        }
        self
    }

    pub fn dot(mut self) -> Self {
        self.dot_product = vec!["Dot"];
        self.anti_dot_product = vec!["AntiDot"];
        self
    }

    pub fn also_dot(mut self) -> Self {
        if !self.dot_product.contains(&"Dot") {
            self.dot_product.push("Dot");
        }
        if !self.anti_dot_product.contains(&"AntiDot") {
            self.anti_dot_product.push("AntiDot");
        }
        self
    }

    pub fn meet_and_join(mut self) -> Self {
        self.exterior_product = vec!["Join"];
        self.exterior_anti_product = vec!["Meet"];
        self
    }

    pub fn also_meet_and_join(mut self) -> Self {
        if !self.exterior_product.contains(&"Join") {
            self.exterior_product.push("Join");
        }
        if !self.exterior_anti_product.contains(&"Meet") {
            self.exterior_anti_product.push("Meet");
        }
        self
    }
}