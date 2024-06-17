//! This modules contains the structs necessary to represent Wyckoff positions and Pairs

use crate::{
    symmetry::{IsometryGroup, IsometryIter},
    Bounds3, Pos3, Vec3,
};

/// this struct represents a collection of sites within the given bounds
#[derive(Clone)]
pub struct Site {
    position: Pos3,
    stabilizer: IsometryGroup,
    orbit: Vec<Pos3>,
}

impl Site {
    /// create a new site calculating the orbit and the stabilizer
    pub fn new(group: &IsometryGroup, position: Pos3) -> Self {
        let position = position % Bounds3::splat(1.into()); // put site in first unit cell
        let mut orbit = vec![position];
        let mut stabilizer = Vec::new();
        for op in group.iter_with_bounds(Bounds3::splat(1)) {
            let new_pos = (op * position) % Bounds3::splat(1);
            if new_pos == position {
                stabilizer.push(op)
            }
            if !orbit.contains(&new_pos) {
                orbit.push(new_pos)
            }
        }
        let stabilizer = IsometryGroup::from_closed_symmetries(stabilizer)
            .expect("the stabilizer group is a subgroup of the space group");
        Self {
            position,
            stabilizer,
            orbit,
        }
    }

    /// returns how many symmetry related positions there are
    pub fn multiplicity(&self) -> usize {
        self.orbit.len()
    }

    /// returns the orbit
    pub fn get_orbit(&self) -> &[Pos3] {
        &self.orbit
    }

    /// retruns the orbit as expanded to the bounds
    pub fn orbit_in_bounds<'a>(&'a self, bounds: Bounds3) -> IsometryIter<'a, Pos3> {
        IsometryIter::new(&self.orbit, bounds)
    }

    /// returns the stabilizer
    pub fn get_stabilizer(&self) -> &IsometryGroup {
        &self.stabilizer
    }
}

impl PartialEq for Site {
    fn eq(&self, other: &Self) -> bool {
        self.orbit.contains(&other.position)
    }
}

impl Eq for Site {}

/// a struct representing a pair of symmetry related positions
pub struct PairExpansion {
    origin_site: Site,
    vec: Vec3,
    expansion: Vec<Pos3>,
}

impl PairExpansion {
    /// returns true if the site Pair expansion contains the pair.
    /// note this only works if the origin is the origin position of the pair expansion
    fn contains_pair(&self, origin_position: Pos3, end_position: Pos3) -> bool {
        assert_eq!(self.origin_site.position, origin_position);
        for pos in &self.expansion {
            if *pos == end_position {
                return true;
            }
        }
        return false;
    }
}

impl PairExpansion {
    /// this function calculates the pair expansion of this pair.
    /// note that the pair must be between symmetry equivalent positions.
    pub fn from_positions(
        origin_site: Site,
        end_position: Pos3,
        group: &IsometryGroup,
        bounds: Bounds3,
    ) -> Self {
        let origin_position = origin_site.position;
        let origin_site = Site::new(&group, origin_position);

        let mut expansion = Vec::new();
        for op in group.iter_with_bounds(bounds) {
            let new_p1 = (op * origin_position) % bounds;
            let new_p2 = (op * end_position) % bounds;
            if new_p1 == origin_position && !(expansion.contains(&new_p2)) {
                expansion.push(new_p2)
            }
            if new_p2 == origin_position && !(expansion.contains(&new_p1)) {
                expansion.push(new_p1)
            }
        }
        Self {
            origin_site,
            vec: end_position - origin_position,
            expansion,
        }
    }

    /// return how many ordered pairs of this type can be formed from positions within a unitcell
    pub fn multiplicity(&self) -> usize {
        self.origin_site.multiplicity() * self.expansion.len()
    }

    /// returns an array of three Strings [origin_position, pair vector, multiplicity]
    pub fn to_string(&self) -> (String, String, String) {
        (
            self.origin_site.position.to_string(),
            self.vec.to_string(),
            self.multiplicity().to_string(),
        )
    }
}

/// represents all pairs which can be formed within the given bounds.
pub struct PairCollection {
    space_group: IsometryGroup,
    sites: Vec<Site>,
    expansions: Vec<PairExpansion>,
    bounds: Bounds3,
}

/// tests if the position is contained within any of the orbits of the sites given
fn contains_position(sites: &[Site], position: Pos3) -> bool {
    for site in sites {
        if site.orbit.contains(&position) {
            return true;
        }
    }
    return false;
}

impl PairCollection {
    /// constructs all pairs from the positions. The positions are deduplicated using the space
    /// group befor applying the algorithm.
    /// If construct ab pairs is set to true the pairs of different sites are constructed to.
    pub fn new(
        group: IsometryGroup,
        mut positions: Vec<Pos3>,
        bounds: Bounds3,
        construct_ab_pairs: bool,
    ) -> Self {
        positions
            .iter_mut()
            .for_each(|p| *p = *p % Bounds3::splat(1));
        let mut sites = Vec::new();
        for pos in positions {
            if !contains_position(&sites, pos) {
                sites.push(Site::new(&group, pos))
            }
        }
        let mut expansions = Vec::new();

        for site in &sites {
            expansions.append(&mut construct_site_pairs(site.clone(), bounds, &group));
        }

        if construct_ab_pairs {
            for site_1 in &sites {
                for site_2 in &sites {
                    expansions.append(&mut construct_2_site_pairs(
                        site_1.clone(),
                        site_2.clone(),
                        bounds,
                        &group,
                    ))
                }
            }
        }
        Self {
            sites,
            expansions,
            space_group: group,
            bounds,
        }
    }

    /// produces a string table of the results
    pub fn produce_output_string(&self) -> String {
        let mut string = format!(
            "{: <15}, {: <15}, {: <15}",
            "Origin", "Vector", "Multiplicity"
        );
        for (a, b, c) in self.expansions.iter().map(PairExpansion::to_string) {
            string += &format!("\n{: <15}, {: <15}, {: <15}", a, b, c);
        }
        string
    }
}

/// retruns true if the pair equal to one of the pairs in the site.
fn contains_pair(expansions: &[PairExpansion], origin_position: Pos3, end_position: Pos3) -> bool {
    for expansion in expansions {
        if expansion.contains_pair(origin_position, end_position) {
            return true;
        }
    }
    return false;
}

/// constructs all pair which can be formed from pairs of positions in this site within the given
/// bounds
fn construct_site_pairs(site: Site, bounds: Bounds3, group: &IsometryGroup) -> Vec<PairExpansion> {
    let mut out = Vec::new();
    for pos in site.orbit_in_bounds(bounds) {
        if !contains_pair(&out, site.position, pos) {
            out.push(PairExpansion::from_positions(
                site.clone(),
                pos,
                group,
                bounds,
            ))
        }
    }
    out
}

/// constructs all pairs which have their origin at site_1 and their end point at one of the
/// positions of site_2
fn construct_2_site_pairs(
    site_1: Site,
    site_2: Site,
    bounds: Bounds3,
    group: &IsometryGroup,
) -> Vec<PairExpansion> {
    let mut out = Vec::new();
    let origin_position = site_1.position;
    for pos in site_2.orbit_in_bounds(bounds) {
        if !contains_pair(&out, origin_position, pos) {
            out.push(PairExpansion::from_positions(
                site_1.clone(),
                pos,
                group,
                bounds,
            ))
        }
    }
    out
}
