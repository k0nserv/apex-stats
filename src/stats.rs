use std::cmp::Ordering;
use std::collections::HashSet;

use chrono::{DateTime, Local, TimeZone};

use crate::apex::{Legend, SquadType};
use crate::observation::Observation;

#[derive(Debug, PartialEq, Eq)]
pub struct QueryResult {
    pub max_damage: u64,
    pub max_kills: u64,
    pub total_damage: u64,
    pub total_kills: u64,
    pub number_of_matches: u64,
}

impl QueryResult {
    pub fn average_damager_per_round(&self) -> f64 {
        self.total_damage as f64 / self.number_of_matches as f64
    }

    pub fn average_kills_per_round(&self) -> f64 {
        self.total_kills as f64 / self.number_of_matches as f64
    }
}

pub struct Query {
    legends: HashSet<Legend>,
    squad_types: HashSet<SquadType>,
    after: Option<DateTime<Local>>,
    before: Option<DateTime<Local>>,
}

impl Query {
    pub fn new() -> Self {
        Self {
            legends: HashSet::new(),
            squad_types: HashSet::new(),
            after: None,
            before: None,
        }
    }

    pub fn match_legend(mut self, legend: Legend) -> Self {
        self.legends.insert(legend);

        self
    }

    pub fn match_legends<I>(mut self, legends: I) -> Self
    where
        I: IntoIterator<Item = Legend>,
    {
        self.legends.extend(legends);

        self
    }

    pub fn match_squad_type(mut self, squad_type: SquadType) -> Self {
        self.squad_types.insert(squad_type);

        self
    }

    pub fn match_squad_types<I>(mut self, squad_types: I) -> Self
    where
        I: IntoIterator<Item = SquadType>,
    {
        self.squad_types.extend(squad_types);

        self
    }

    pub fn after(mut self, after: DateTime<Local>) -> Self {
        self.after = Some(after);

        self
    }

    pub fn before(mut self, before: DateTime<Local>) -> Self {
        self.before = Some(before);

        self
    }

    pub fn execute(&self, records: impl Iterator<Item = Observation>) -> Option<QueryResult> {
        let (total_damage, total_kills, max_damage, max_kills, observation_count) = records
            .filter(|observation| {
                if self.legends.is_empty() {
                    true
                } else {
                    self.legends.contains(&observation.legend)
                }
            })
            .filter(|observation| {
                if self.squad_types.is_empty() {
                    true
                } else {
                    self.squad_types.contains(&observation.squad_type)
                }
            })
            .filter(|observation| {
                self.before
                    .map(|before| before.cmp(&observation.at) == Ordering::Greater)
                    .unwrap_or(true)
            })
            .filter(|observation| {
                self.after
                    .map(|after| after.cmp(&observation.at) == Ordering::Less)
                    .unwrap_or(true)
            })
            .fold(
                (0, 0, 0, 0, 0),
                |(total_damage, total_kills, max_damage, max_kills, count), observation| {
                    let new_max_damage = if max_damage < observation.damage_dealt {
                        observation.damage_dealt
                    } else {
                        max_damage
                    };

                    let new_max_kills = if max_kills < observation.number_of_kills {
                        observation.number_of_kills
                    } else {
                        max_kills
                    };

                    (
                        total_damage + observation.damage_dealt,
                        total_kills + observation.number_of_kills,
                        new_max_damage,
                        new_max_kills,
                        count + 1,
                    )
                },
            );

        if observation_count == 0 {
            None
        } else {
            let average_damager_per_round = total_damage as f64 / observation_count as f64;
            let average_kills_per_round = total_kills as f64 / observation_count as f64;

            Some(QueryResult {
                max_damage,
                max_kills,
                total_kills,
                total_damage,
                number_of_matches: observation_count,
            })
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn sample_data() -> impl Iterator<Item = Observation> {
        vec![
            Observation {
                number_of_kills: 5,
                damage_dealt: 891,
                squad_position: 3,
                legend: Legend::Bangalore,
                // 2019-04-22T22:05:02+0000
                at: Local.timestamp(1555970702, 0),
                squad_type: SquadType::Solo,
                notes: "".to_string(),
            },
            Observation {
                number_of_kills: 3,
                damage_dealt: 520,
                squad_position: 6,
                legend: Legend::Wraith,
                // 2019-04-22T21:42:02+0000
                at: Local.timestamp(1555969322, 0),
                squad_type: SquadType::Duo,
                notes: "".to_string(),
            },
            Observation {
                number_of_kills: 1,
                damage_dealt: 123,
                squad_position: 18,
                legend: Legend::Wraith,
                // 2019-04-22T19:37:02+0000
                at: Local.timestamp(1555961822, 0),
                squad_type: SquadType::Solo,
                notes: "".to_string(),
            },
            Observation {
                number_of_kills: 9,
                damage_dealt: 1032,
                squad_position: 1,
                legend: Legend::Pathfinder,
                // 2019-04-21T00:37:02+0000
                at: Local.timestamp(1555807022, 0),
                squad_type: SquadType::Trio,
                notes: "".to_string(),
            },
        ]
        .into_iter()
    }

    #[test]
    fn unfiltered() {
        let query = Query::new();

        assert_eq!(
            Some(QueryResult {
                max_damage: 1032,
                max_kills: 9,
                total_damage: 2566,
                total_kills: 18,
                number_of_matches: 4
            }),
            query.execute(sample_data())
        );
    }

    #[test]
    fn only_solo_games() {
        let query = Query::new().match_squad_type(SquadType::Solo);

        assert_eq!(
            Some(QueryResult {
                max_damage: 891,
                max_kills: 5,
                total_damage: 1014,
                total_kills: 6,
                number_of_matches: 2
            }),
            query.execute(sample_data())
        );
    }

    #[test]
    fn only_non_solo_games() {
        let query = Query::new().match_squad_types(vec![SquadType::Duo, SquadType::Trio]);

        assert_eq!(
            Some(QueryResult {
                max_damage: 1032,
                max_kills: 9,
                total_damage: 1552,
                total_kills: 12,
                number_of_matches: 2
            }),
            query.execute(sample_data())
        );
    }

    #[test]
    fn only_solo_games_with_wraith() {
        let query = Query::new()
            .match_squad_type(SquadType::Solo)
            .match_legend(Legend::Wraith);

        assert_eq!(
            Some(QueryResult {
                max_damage: 123,
                max_kills: 1,
                total_damage: 123,
                total_kills: 1,
                number_of_matches: 1
            }),
            query.execute(sample_data())
        );
    }

    #[test]
    fn only_games_after_and_before() {
        let query = Query::new()
            .after(Local.timestamp(1555807022 + 1, 0))
            .before(Local.timestamp(1555969322, 0));

        assert_eq!(
            Some(QueryResult {
                max_damage: 123,
                max_kills: 1,
                total_damage: 123,
                total_kills: 1,
                number_of_matches: 1
            }),
            query.execute(sample_data())
        );
    }
}
