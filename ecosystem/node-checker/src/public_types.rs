use poem_openapi::Object as PoemObject;

// TODO: Should I find a way to have typed actual + expected fields?
#[derive(Clone, Debug, PoemObject)]
pub struct Evaluation {
    /// Headline of the evaluation, e.g. "Healthy!" or "Metrics missing!".
    pub headline: String,

    /// Score out of 100.
    pub score: u8,

    /// Explanation of the evaluation.
    pub explanation: String,
}

#[derive(Clone, Debug, PoemObject)]
pub struct CompleteEvaluation {
    /// All the evaluations we ran.
    pub evaluations: Vec<Evaluation>,

    /// An aggeregated summary (method TBA).
    pub summary_score: u8,

    /// An overall explanation of the results.
    pub summary_explanation: String,
}

impl From<Vec<Evaluation>> for CompleteEvaluation {
    // Very basic for now, we likely want a trait for this.
    fn from(evaluations: Vec<Evaluation>) -> Self {
        let summary_score =
            evaluations.iter().map(|e| e.score).sum::<u8>() / evaluations.len() as u8;
        let summary_explanation = match summary_score {
            summary_score if summary_score > 95 => format!("{}, awesome!", summary_score),
            summary_score if summary_score > 80 => format!("{}, good!", summary_score),
            summary_score if summary_score > 50 => format!("{}, getting there!", summary_score),
            wildcard => format!("{}, not good enough :(", wildcard),
        };
        CompleteEvaluation {
            evaluations,
            summary_score,
            summary_explanation,
        }
    }
}
