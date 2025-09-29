use std::cmp;

use crate::models::race_results::RaceTable;

struct DriverAccumulator {
    races_finished: u32,
    best_grid: u32,
    best_finish: u32,
    sum_of_grids: u32,
    sum_of_finishes: u32,
    poles: u32,
    wins: u32,
    ret: u32,
    points: f64,
}

/// Holds total and avg stats from a single season for one driver.
pub struct DriverStats {
    pub code: String,
    pub total_races: u32,
    pub best_grid: u32,
    pub best_finish: u32,
    pub poles: u32,
    pub wins: u32,
    pub ret: u32,
    pub points: f64,
    pub avg_grid: f64,
    pub avg_finish: f64,
}

impl DriverStats {
    /// Creates a DriverStats struct based on data from the given RaceTable.
    pub fn from_race_table(race_table: &RaceTable) -> Self {
        let initial_counts = DriverAccumulator {
            races_finished: 0,
            best_grid: u32::MAX,
            best_finish: u32::MAX,
            sum_of_grids: 0,
            sum_of_finishes: 0,
            poles: 0,
            wins: 0,
            ret: 0,
            points: 0.0,
        };

        let code = race_table.races[0].results[0]
            .driver
            .code
            .as_deref()
            .unwrap_or_default()
            .to_string();

        // Use an iterator to go through each race result in the table.
        // Fold each value into a DriverAccumulator struct.
        let counts = race_table
            .races
            .iter()
            .fold(initial_counts, |mut stats, race| {
                let result = &race.results[0];
                let status = result.status.as_deref();

                // Count grids regardless of finishing status.
                if let Some(g) = result.grid {
                    stats.best_grid = cmp::min(g, stats.best_grid);
                    stats.sum_of_grids += g;
                    if g == 1 {
                        stats.poles += 1;
                    }
                }

                // Only count results when a driver finished a race.
                if status == Some("Finished") {
                    stats.races_finished += 1;
                    stats.best_finish = cmp::min(result.position, stats.best_finish);
                    stats.sum_of_finishes += result.position;
                    stats.points += result.points;

                    if result.position == 1 {
                        stats.wins += 1;
                    }
                } else if status == Some("Retired") {
                    stats.ret += 1;
                }
                stats
            });

        // Now that we have the counts, we can compute the averages.
        let total_races = counts.races_finished + counts.ret;

        let avg_grid = avg(counts.sum_of_grids as f64, total_races as f64);
        let avg_finish = avg(counts.sum_of_finishes as f64, total_races as f64);

        // Everything is moved to a single public struct for simple use by the caller.
        Self {
            code,
            total_races,
            best_grid: counts.best_grid,
            best_finish: counts.best_finish,
            poles: counts.poles,
            wins: counts.wins,
            ret: counts.ret,
            points: counts.points,
            avg_grid,
            avg_finish,
        }
    }
}

fn avg(sum: f64, count: f64) -> f64 {
    // It should never happen, but to be safe, protect against division by zero.
    if count == 0.0 { 0.0 } else { sum / count }
}
