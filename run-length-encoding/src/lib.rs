extern crate regex;
use regex::Regex;

#[derive(Debug, Copy, Clone)]
struct Cluster {
    ch: char,
    count: usize,
}

impl Cluster {
    fn new(ch: char) -> Self {
        Cluster { ch, count: 1 }
    }

    fn new_with_count(ch: char, count: usize) -> Self {
        Cluster { ch, count }
    }

    fn increment(&mut self) {
        self.count += 1
    }

    fn to_encoded_string(self) -> String {
        if self.count == 1 {
            format!("{}", self.ch)
        } else {
            format!("{}{}", self.count, self.ch)
        }
    }

    fn to_decoded_string(self) -> String {
        self.ch.to_string().repeat(self.count)
    }
}

pub fn encode(source: &str) -> String {
    // If the source is empty return it
    if source.is_empty() {
        return source.to_string();
    }

    let mut clusters = Vec::<Cluster>::new();

    // Initialize the current_cluster to the first char in the source string
    let mut current_cluster: Cluster = Cluster::new(source.chars().next().unwrap());

    // Skip the first character as we already have captured it into the initial cluster
    // above.
    for (ix, ch) in source.chars().skip(1).enumerate() {
        if ch != current_cluster.ch {
            clusters.push(current_cluster);
            current_cluster = Cluster::new(ch);
        } else {
            current_cluster.increment();
        }

        // Don't forget about the last ch in the string, we need len-2 because we skipped
        // the first character.
        if ix == source.len() - 2 {
            clusters.push(current_cluster);
        }
    }

    // I still love folds so much! Almost as much as IKEA Kallax shelving!
    clusters.iter().fold("".to_string(), |s, cl| {
        format!("{}{}", s, cl.to_encoded_string())
    })
}

pub fn decode(source: &str) -> String {
    if source.is_empty() {
        return source.to_string();
    }

    let mut clusters = Vec::<Cluster>::new();

    let re = Regex::new(r"(?P<count>\d+)(?P<ch>[[:alpha:]]|\s+)|(?P<ch2>[[:alpha:]]|\s+)").unwrap();

    for cap in re.captures_iter(source) {
        let has_num = cap.name("count");

        let cluster = match has_num {
            // If there is a number we also know that capture group ch will match
            Some(c) => Cluster::new_with_count(
                cap.name("ch").unwrap().as_str().chars().next().unwrap(),
                c.as_str().parse::<usize>().unwrap(),
            ),
            // If this is a single letter without a number we know ch2 should match
            None => Cluster::new(cap.name("ch2").unwrap().as_str().chars().next().unwrap()),
        };

        clusters.push(cluster)
    }

    clusters.iter().fold("".to_string(), |s, cl| {
        format!("{}{}", s, cl.to_decoded_string())
    })
}
