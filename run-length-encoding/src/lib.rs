#[derive(Debug, Copy, Clone)]
struct Cluster {
    ch: char,
    count: usize,
}

impl Cluster {
    fn new(ch: char, count: usize) -> Self {
        Cluster { ch, count }
    }

    fn increment(&mut self) {
        self.count += 1
    }

    fn to_encoded_string(self) -> String {
        if self.count == 1 {
            self.ch.to_string()
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
    let mut current_cluster: Cluster = Cluster::new(source.chars().next().unwrap(), 1);

    // Skip the first character as we already have captured it into the initial cluster
    // above.
    for (ix, ch) in source.chars().skip(1).enumerate() {
        if ch != current_cluster.ch {
            clusters.push(current_cluster);
            current_cluster = Cluster::new(ch, 1);
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
    // Split the string by "clusters" which may have a number and/or a letter or whitespace
    let cluster_str_vec =
        source.split_inclusive(|ch| char::is_alphabetic(ch) || char::is_whitespace(ch));

    for cluster_str in cluster_str_vec {
        let (num_part, letter_part): (String, String) =
            cluster_str.chars().partition(|ch| ch.is_digit(10));
        // First see if there is a number
        let count = num_part.parse::<usize>();
        let ch = letter_part.chars().next().unwrap();
        let cluster = match count {
            Ok(c) => Cluster::new(ch, c),
            _ => Cluster::new(ch, 1),
        };
        clusters.push(cluster);
    }

    clusters.iter().fold("".to_string(), |s, cl| {
        format!("{}{}", s, cl.to_decoded_string())
    })
}
