use crate::study::StudyRecord;
use crate::Name;
use kurobako_core::{Error, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::Path;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct PlotOptions {
    #[structopt(long, default_value = "800")]
    pub width: usize,

    #[structopt(long, default_value = "600")]
    pub height: usize,

    #[structopt(long)]
    pub ymin: Option<f64>,

    #[structopt(long)]
    pub ymax: Option<f64>,

    #[structopt(long, default_value = "")]
    pub prefix: String,
}
impl PlotOptions {
    pub fn plot_problems<P: AsRef<Path>>(&self, studies: &[StudyRecord], dir: P) -> Result<()> {
        let mut problems = BTreeMap::new();
        for s in studies {
            problems.entry(&s.problem).or_insert_with(Vec::new).push(s);
        }
        let problems = problems
            .into_iter()
            .map(|(problem, studies)| ProblemPlot::new(problem, &studies));
        for (i, problem) in problems.enumerate() {
            track!(problem.plot(dir.as_ref().join(format!("{}{}.dat", self.prefix, i))))?;

            let commands = self.make_gnuplot_commands(
                &problem.problem,
                problem.solvers.len(),
                dir.as_ref().join(format!("{}{}.dat", self.prefix, i)),
                dir.as_ref().join(format!("{}{}.png", self.prefix, i)),
            );
            {
                use std::process::Command;
                println!("# {}", commands);
                track!(Command::new("gnuplot")
                    .args(&["-e", &commands])
                    .output()
                    .map_err(Error::from))?;
            }
        }
        Ok(())
    }

    fn make_gnuplot_commands<P: AsRef<Path>>(
        &self,
        problem: &Name,
        solvers: usize,
        input: P,
        output: P,
    ) -> String {
        let mut s = format!(
            "set title {:?}; set ylabel \"Score\"; set xlabel \"Budget\"; set grid;",
            problem
                .as_json()
                .to_string()
                .replace('"', "")
                .replace('{', "(")
                .replace('}', ")")
        );
        s += &format!(
            "set key bmargin; set terminal pngcairo size {},{}; set output {:?}; ",
            self.width,
            self.height,
            output.as_ref().to_str().expect("TODO")
        );
        s += &format!(
            "plot [] [{}:{}]",
            self.ymin.unwrap_or(0.0),
            self.ymax.unwrap_or(1.0)
        );
        for i in 0..solvers {
            if i == 0 {
                s += &format!(" {:?}", input.as_ref().to_str().expect("TODO"));
            } else {
                s += ", \"\"";
            }
            s += &format!(" u 0:{} w l t columnhead", i + 1);
        }
        s
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProblemPlot {
    pub problem: Name,
    pub solvers: Vec<SolverPlot>,
}
impl ProblemPlot {
    fn new(name: &Name, studies: &[&StudyRecord]) -> Self {
        let mut solvers = BTreeMap::new();
        for s in studies {
            solvers.entry(&s.solver).or_insert_with(Vec::new).push(*s);
        }
        let solvers = solvers
            .into_iter()
            .map(|(solver, studies)| SolverPlot::new(solver, &studies))
            .collect();
        Self {
            problem: name.clone(),
            solvers,
        }
    }

    fn plot<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        use std::fs::File;
        use std::io::Write;

        let mut f = track!(File::create(path).map_err(Error::from))?;
        writeln!(f, "# Problem: {}", self.problem.as_json())?;

        for o in &self.solvers {
            write!(
                f,
                "{:?} ",
                o.solver
                    .as_json()
                    .to_string()
                    .replace('"', "")
                    .replace('{', "(")
                    .replace('}', ")")
            )?;
        }
        writeln!(f)?;

        let len = self
            .solvers
            .iter()
            .map(|o| o.avg_scores.len())
            .max()
            .expect("TODO");
        for i in 0..len {
            for o in &self.solvers {
                write!(f, "{} ", o.score(i))?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SolverPlot {
    pub solver: Name,
    pub avg_scores: Vec<f64>,
}
impl SolverPlot {
    fn new(name: &Name, studies: &[&StudyRecord]) -> Self {
        let mut avg_scores = Vec::new();
        let scorers = studies.iter().map(|s| s.scorer()).collect::<Vec<_>>();
        for i in 0..studies[0].budget {
            let avg_score =
                scorers.iter().map(|s| s.best_score_until(i)).sum::<f64>() / studies.len() as f64;
            avg_scores.push(avg_score);
        }
        Self {
            solver: name.clone(),
            avg_scores,
        }
    }

    fn score(&self, i: usize) -> f64 {
        *self
            .avg_scores
            .get(i)
            .unwrap_or_else(|| self.avg_scores.last().expect("TODO"))
    }
}
