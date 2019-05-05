use kurobako_core::epi::problem::{
    EmbeddedScriptEvaluator, EmbeddedScriptProblem, EmbeddedScriptProblemRecipe,
};
use kurobako_core::parameter::ParamValue;
use kurobako_core::problem::{Evaluate, Evaluated, Problem, ProblemRecipe, ProblemSpec};
use kurobako_core::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use structopt::StructOpt;
use yamakan::budget::Budget;
use yamakan::observation::ObsId;

macro_rules! define_sigopt_problem_spec {
    ($([$name:ident, $dim:expr],)*) => {
        #[derive(Debug, StructOpt, Serialize, Deserialize)]
        #[serde(rename_all = "kebab-case")]
        #[structopt(rename_all = "kebab-case")]
        pub enum SigoptProblemRecipe {
            $($name {
                #[structopt(long, default_value = $dim)]
                dim: u32,

                #[serde(skip_serializing_if = "Option::is_none")]
                #[structopt(long)]
                res: Option<f64>,

                #[structopt(long)]
                python: Option<PathBuf>,
            }),*
        }
        impl SigoptProblemRecipe {
            pub fn name(&self) -> &'static str {
                match self {
                    $(SigoptProblemRecipe::$name { .. } => stringify!($name)),*
                }
            }

            pub fn dim(&self) -> u32 {
                match *self {
                    $(SigoptProblemRecipe::$name { dim, .. } => dim),*
                }
            }

            pub fn res(&self) -> Option<f64> {
                match *self {
                    $(SigoptProblemRecipe::$name { res, .. } => res),*
                }
            }

            pub fn python(&self) -> Option<&PathBuf> {
                 match self {
                    $(SigoptProblemRecipe::$name { python, .. } => python.as_ref()),*
                }
            }
        }
    };
}

define_sigopt_problem_spec!(
    [Ackley, "2"],
    [Adjiman, "2"],
    [Alpine01, "2"],
    [Alpine02, "2"],
    [ArithmeticGeometricMean, "2"],
    [BartelsConn, "2"],
    [Beale, "2"],
    [Bird, "2"],
    [Bohachevsky, "2"],
    [BoxBetts, "3"],
    [Branin01, "2"],
    [Branin02, "2"],
    [Brent, "2"],
    [Brown, "2"],
    [Bukin06, "2"],
    [CarromTable, "2"],
    [Chichinadze, "2"],
    [Cigar, "2"],
    [Cola, "17"],
    [Corana, "4"],
    [CosineMixture, "2"],
    [CrossInTray, "2"],
    [Csendes, "2"],
    [Cube, "2"],
    [Damavandi, "2"],
    [Deb01, "2"],
    [Deb02, "2"],
    [Deceptive, "2"],
    [DeflectedCorrugatedSpring, "2"],
    [Dolan, "5"],
    [DropWave, "2"],
    [Easom, "2"],
    [EggCrate, "2"],
    [EggHolder, "2"],
    [ElAttarVidyasagarDutta, "2"],
    [Exponential, "2"],
    [Franke, "2"],
    [FreudensteinRoth, "2"],
    [Gear, "4"],
    [Giunta, "2"],
    [GoldsteinPrice, "2"],
    [Griewank, "2"],
    [Hansen, "2"],
    [Hartmann3, "3"],
    [Hartmann4, "4"],
    [Hartmann6, "6"],
    [HelicalValley, "3"],
    [HimmelBlau, "2"],
    [HolderTable, "2"],
    [Hosaki, "2"],
    [HosakiExpanded, "2"],
    [JennrichSampson, "2"],
    [Judge, "2"],
    [Keane, "2"],
    [Langermann, "2"],
    [LennardJones6, "6"],
    [Leon, "2"],
    [Levy03, "8"],
    [Levy05, "2"],
    [Levy13, "2"],
    [Matyas, "2"],
    [McCormick, "2"],
    [McCourt01, "7"],
    [McCourt02, "7"],
    [McCourt03, "9"],
    [McCourt04, "10"],
    [McCourt05, "12"],
    [McCourt06, "5"],
    [McCourt07, "6"],
    [McCourt08, "4"],
    [McCourt09, "3"],
    [McCourt10, "8"],
    [McCourt11, "8"],
    [McCourt12, "7"],
    [McCourt13, "3"],
    [McCourt14, "3"],
    [McCourt15, "3"],
    [McCourt16, "4"],
    [McCourt17, "7"],
    [McCourt18, "8"],
    [McCourt19, "2"],
    [McCourt20, "2"],
    [McCourt21, "4"],
    [McCourt22, "5"],
    [McCourt23, "6"],
    [McCourt24, "7"],
    [McCourt25, "8"],
    [McCourt26, "3"],
    [McCourt27, "3"],
    [McCourt28, "4"],
    [MegaDomain01, "2"],
    [MegaDomain02, "3"],
    [MegaDomain03, "3"],
    [MegaDomain04, "3"],
    [MegaDomain05, "4"],
    [Michalewicz, "2"],
    [MieleCantrell, "4"],
    [Mishra02, "2"],
    [Mishra06, "2"],
    [Mishra08, "2"],
    [Mishra10, "2"],
    [ManifoldMin, "2"],
    [MixtureOfGaussians01, "2"],
    [MixtureOfGaussians02, "2"],
    [MixtureOfGaussians03, "2"],
    [MixtureOfGaussians04, "2"],
    [MixtureOfGaussians05, "8"],
    [MixtureOfGaussians06, "8"],
    [Ned01, "2"],
    [Ned03, "2"],
    [OddSquare, "2"],
    [Parsopoulos, "2"],
    [Pavianini, "10"],
    [Penalty01, "2"],
    [Penalty02, "2"],
    [PenHolder, "2"],
    [Perm01, "2"],
    [Perm02, "2"],
    [Pinter, "2"],
    [Plateau, "2"],
    [Powell, "4"],
    [PowellTripleLog, "12"],
    [PowerSum, "4"],
    [Price, "2"],
    [Qing, "2"],
    [Quadratic, "2"],
    [Rastrigin, "8"],
    [RippleSmall, "2"],
    [RippleBig, "2"],
    [RosenbrockLog, "11"],
    [RosenbrockModified, "2"],
    [Salomon, "2"],
    [Sargan, "2"],
    [Schaffer, "2"],
    [SchmidtVetters, "3"],
    [Schwefel01, "2"],
    [Schwefel06, "2"],
    [Schwefel20, "2"],
    [Schwefel22, "2"],
    [Schwefel26, "2"],
    [Schwefel36, "2"],
    [Shekel05, "4"],
    [Shekel07, "4"],
    [Shekel10, "4"],
    [Shubert01, "2"],
    [Shubert03, "2"],
    [SineEnvelope, "2"],
    [SixHumpCamel, "2"],
    [Sphere, "2"],
    [Step, "2"],
    [StretchedV, "2"],
    [StyblinskiTang, "2"],
    [SumPowers, "2"],
    [TestTubeHolder, "2"],
    [ThreeHumpCamel, "2"],
    [Trefethen, "2"],
    [Trid, "6"],
    [Tripod, "2"],
    [Ursem01, "2"],
    [Ursem03, "2"],
    [Ursem04, "2"],
    [UrsemWaves, "2"],
    [VenterSobiezcczanskiSobieski, "2"],
    [Watson, "6"],
    [Weierstrass, "2"],
    [Wolfe, "3"],
    [XinSheYang02, "2"],
    [XinSheYang03, "2"],
    [Xor, "9"],
    [YaoLiu, "2"],
    [ZeroSum, "2"],
    [Zimmerman, "2"],
    [Problem02, "1"],
    [Problem03, "1"],
    [Problem04, "1"],
    [Problem05, "1"],
    [Problem06, "1"],
    [Problem07, "1"],
    [Problem09, "1"],
    [Problem10, "1"],
    [Problem11, "1"],
    [Problem12, "1"],
    [Problem13, "1"],
    [Problem14, "1"],
    [Problem15, "1"],
    [Problem18, "1"],
    [Problem20, "1"],
    [Problem21, "1"],
    [Problem22, "1"],
);

impl ProblemRecipe for SigoptProblemRecipe {
    type Problem = SigoptProblem;

    fn create_problem(&self) -> Result<Self::Problem> {
        let script = include_str!("../../contrib/sigopt_problem.py");
        let mut args = vec![self.name().to_owned(), self.dim().to_string()];
        if let Some(res) = self.res() {
            args.extend_from_slice(&["--res".to_owned(), res.to_string()]);
        }
        let recipe = EmbeddedScriptProblemRecipe {
            script: script.to_owned(),
            args,
            interpreter: self.python().cloned(),
            interpreter_args: Vec::new(),
            skip_lines: None,
        };
        let inner = track!(recipe.create_problem())?;
        Ok(SigoptProblem(inner))
    }
}

#[derive(Debug)]
pub struct SigoptProblem(EmbeddedScriptProblem);
impl Problem for SigoptProblem {
    type Evaluator = SigoptEvaluator;

    fn specification(&self) -> ProblemSpec {
        self.0.specification()
    }

    fn create_evaluator(&mut self, id: ObsId) -> Result<Self::Evaluator> {
        track!(self.0.create_evaluator(id)).map(SigoptEvaluator)
    }
}

#[derive(Debug)]
pub struct SigoptEvaluator(EmbeddedScriptEvaluator);
impl Evaluate for SigoptEvaluator {
    fn evaluate(&mut self, params: &[ParamValue], budget: &mut Budget) -> Result<Evaluated> {
        track!(self.0.evaluate(params, budget))
    }
}
