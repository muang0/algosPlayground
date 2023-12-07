
#[derive(Copy, Clone, PartialEq)]
struct Cluster {
    start: u64,
    range: u64,
    has_been_mapped: bool
}

struct Mapping {
    source: u64,
    destination: u64,
    range: u64
}

fn main() {
    let mut closest_distance = u64::MAX;
    let almanac_input = std::fs::read_to_string("data/input.txt").unwrap();
    let seeds_input: Vec<&str> = almanac_input
        .split("\n")
        .nth(0)
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .split(" ")
        .filter(|i| *i != "")
        .collect();
    let mut seeds: Vec<Cluster> = Vec::new();
    let mut previous_entry = "";

    // compose initial clusters
    for (iter, seed_entry) in seeds_input.into_iter().enumerate() {
        if iter % 2 == 1 {
            seeds.push(Cluster { start: previous_entry.parse().unwrap(), range: seed_entry.parse().unwrap(), has_been_mapped: false });
        }
        previous_entry = seed_entry;
    }

    // ingest almanac into data structure
    let mut almanac: [Vec<Mapping>; 7] = [Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new()];
    let mut almanac_index = 0;
    for line in almanac_input.split("\n") {
        if line.contains(":") {
            almanac_index = match line.split(" ").nth(0).unwrap() {
                "seed-to-soil" => 0,
                "soil-to-fertilizer" => 1,
                "fertilizer-to-water" => 2,
                "water-to-light" => 3,
                "light-to-temperature" => 4,
                "temperature-to-humidity" => 5,
                "humidity-to-location" => 6,
                _ => 7
            }
        } else if line != "" {
            let destination: u64 =
            line.split(" ").nth(0).unwrap().parse().unwrap();
        let source: u64 = line.split(" ").nth(1).unwrap().parse().unwrap();
        let range: u64 = line.split(" ").nth(2).unwrap().parse().unwrap();
            almanac[almanac_index].push(Mapping { source, destination, range });
        }
    }

    let mut clusters = seeds;
    // for each section in the almanac
    for section in almanac {
        for cluster in &mut clusters {
            cluster.has_been_mapped = false;
        }
        // while map_found_in_cluster_set
        let mut map_found_in_cluster_set = true;
        while map_found_in_cluster_set {
            map_found_in_cluster_set = false;
            // []new_clusters
            let mut new_clusters: Vec<Cluster> = Vec::new();
            // for each []cluster
            for cluster in &clusters {
                let mut cluster_contains_map = false;
                // if cluster hasn't already been mapped (if it has already been mappped- push directly to []new_clusters)
                if cluster.has_been_mapped {
                    new_clusters.push(*cluster);
                } else {
                    // for each []mapping
                    for map in &section {
                        let map_lower_bound = map.source;
                        let map_upper_bound = map.source + map.range;
                        let cluster_lower_bound = cluster.start;
                        let cluster_upper_bound = cluster.start + cluster.range;
                        if map_lower_bound <= cluster_lower_bound && map_upper_bound >= cluster_upper_bound {
                            map_found_in_cluster_set = true;
                            cluster_contains_map = true;
                            // map entire cluster and append to []new_clusters
                            let offset = cluster_lower_bound - map_lower_bound;
                            new_clusters.push(Cluster{start: map.destination + offset, range: cluster.range, has_been_mapped: true});
                        } else if cluster_lower_bound < map_lower_bound && cluster_upper_bound > map_upper_bound {
                            map_found_in_cluster_set = true;
                            cluster_contains_map = true;
                            // form three new clusters
                            let offset = map_lower_bound - cluster_lower_bound;
                            new_clusters.push(Cluster { start: cluster_lower_bound, range: offset, has_been_mapped: false });
                            new_clusters.push(Cluster { start: map.destination, range: map.range, has_been_mapped: true });
                            new_clusters.push(Cluster { start: map_upper_bound, range: cluster_upper_bound - map_upper_bound, has_been_mapped: false })
                        } else if map_lower_bound <= cluster_lower_bound && map_upper_bound > cluster_lower_bound {
                            map_found_in_cluster_set = true;
                            cluster_contains_map = true;
                            // form two new clusters (bottom one is mapped)
                            let offset = cluster_lower_bound - map_lower_bound;
                            new_clusters.push(Cluster { start: map.destination + offset, range: map_upper_bound - cluster_lower_bound, has_been_mapped: true });
                            new_clusters.push(Cluster { start: map_upper_bound, range: cluster_upper_bound - map_upper_bound, has_been_mapped: false });
                        } else if map_lower_bound < cluster_upper_bound && map_upper_bound >= cluster_upper_bound {
                            map_found_in_cluster_set = true;
                            cluster_contains_map = true;
                            // form two new clusters (top one is mapped)
                            new_clusters.push(Cluster { start: cluster_lower_bound, range: map_lower_bound - cluster_lower_bound, has_been_mapped: false });
                            new_clusters.push(Cluster { start: map.destination, range: cluster_upper_bound - map_lower_bound, has_been_mapped: true });
                        }
                    }
                }
                if !cluster_contains_map {
                    new_clusters.push(*cluster);
                }
            }
            // dedup []new_clusters
            new_clusters.dedup();
            // set []cluster to []new_clusters
            clusters = new_clusters;
        }
    }

    for cluster in &clusters {
        if cluster.start < closest_distance {
            closest_distance = cluster.start;
        }
    }

    println!("Closest seed: {:#}", closest_distance);
}
