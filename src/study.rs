use crate::time::DateTime;
use crate::trial::TrialRecord;
use crate::Name;
use chrono::Local;
use kurobako_core::problem::ProblemRecipe;
use kurobako_core::solver::SolverRecipe;
use kurobako_core::Error;
use rustats::num::{FiniteF64, NonNanF64};
use rustats::range::MinMax;
use serde::{Deserialize, Serialize};
use std::f64;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StudyRecord {
    pub solver: Name,
    pub problem: Name,
    pub budget: u64,
    pub value_range: MinMax<f64>, // TODO
    pub start_time: DateTime,
    pub trials: Vec<TrialRecord>,
}
impl StudyRecord {
    pub fn new<O, P>(
        solver_recipe: &O,
        problem: &P,
        budget: u64,
        value_range: MinMax<FiniteF64>,
    ) -> Result<Self, Error>
    where
        O: SolverRecipe,
        P: ProblemRecipe,
    {
        let value_range =
            MinMax::new(value_range.min().get(), value_range.max().get()).expect("TODO");
        Ok(StudyRecord {
            solver: Name::new(serde_json::to_value(solver_recipe)?),
            problem: Name::new(serde_json::to_value(problem)?),
            budget,
            value_range,
            start_time: Local::now(),
            trials: Vec::new(),
        })
    }

    pub fn limit_budget(&mut self, budget: u64) {
        self.budget = budget;
        self.trials.truncate(budget as usize); // TODO:
    }

    pub fn best_score_until(&self, i: usize) -> f64 {
        let normalized_value = self
            .trials
            .iter()
            .take(i)
            .filter_map(|t| t.value())
            .min_by_key(|v| unsafe { NonNanF64::new_unchecked(*v) })
            .map(|v| self.value_range.normalize(v))
            .unwrap_or(1.0);
        1.0 - normalized_value
    }

    pub fn best_score(&self) -> f64 {
        let normalized_value = self
            .trials
            .iter()
            .filter_map(|t| t.value())
            .min_by_key(|v| NonNanF64::new(*v).unwrap_or_else(|e| panic!("{}", e)))
            .map(|v| self.value_range.normalize(v))
            .expect("TODO");
        1.0 - normalized_value
    }

    pub fn auc(&self) -> f64 {
        // TODO: Handle budgets granularity instead trials
        let mut vs = Vec::new();
        for v in self
            .trials
            .iter()
            .filter_map(|t| t.value())
            .map(|v| 1.0 - self.value_range.normalize(v))
        {
            if let Some(&last) = vs.last() {
                if last < v {
                    vs.push(v);
                } else {
                    vs.push(last);
                }
            } else {
                vs.push(v);
            }
        }
        vs.iter().sum::<f64>() / (vs.len() as f64)
    }

    pub fn ack_latencies<'a>(&'a self) -> impl Iterator<Item = f64> + 'a {
        self.trials.iter().map(|t| t.ask.latency())
    }

    pub fn best_trial(&self) -> Option<&TrialRecord> {
        self.trials
            .iter()
            .filter(|t| t.value().is_some())
            .min_by_key(|t| {
                NonNanF64::new(t.value().unwrap_or_else(|| unreachable!()))
                    .unwrap_or_else(|e| panic!("{}", e))
            })
    }

    pub fn elapsed_time(&self) -> f64 {
        self.trials
            .last()
            .map_or(0.0, |t| t.end_time().as_seconds())
    }
}
