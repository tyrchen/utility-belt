use async_recursion::async_recursion;
use async_trait::async_trait;
use derive_new::new;
use std::{collections::VecDeque, fmt};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PipelineError {
    #[error("Invalid context: {0}")]
    InvalidContext(String),

    #[error("Internal: {0}")]
    Internal(String),
}

/// execution result for the plug
#[must_use]
#[derive(new)]
pub enum PlugResult<Ctx> {
    Continue(Ctx),
    #[allow(dead_code)]
    Terminate(Ctx),
    NewPipe {
        ctx: Ctx,
        plugs: Vec<Box<dyn Plug<Ctx>>>,
    },
    Err {
        ctx: Ctx,
        err: PipelineError,
    },
}

/// plug trait that the building blocks in a pipeline shall implement
#[async_trait]
pub trait Plug<Ctx>: fmt::Display + Send + Sync + 'static {
    async fn call(&self, ctx: Ctx) -> PlugResult<Ctx>;
}

/// A sequentially executed pipeline
#[derive(Default)]
pub struct Pipeline<Ctx> {
    plugs: VecDeque<Box<dyn Plug<Ctx>>>,
    executed: Vec<String>,
}

/// Pipeline response
#[must_use]
#[derive(new)]
pub struct PipelineResponse<Ctx> {
    pub ctx: Ctx,
    pub executed: Vec<String>,
    pub err: Option<PipelineError>,
}

impl<Ctx> Pipeline<Ctx>
where
    Ctx: Send + Sync + 'static,
{
    /// create a new pipeline
    pub fn new(plugs: Vec<Box<dyn Plug<Ctx>>>, executed: Option<Vec<String>>) -> Self {
        Self {
            plugs: plugs.into(),
            executed: executed.unwrap_or_default(),
        }
    }

    /// execute the entire pipeline sequentially and run to completion
    #[async_recursion]
    pub async fn execute(mut self, ctx: Ctx) -> PipelineResponse<Ctx> {
        let mut c = ctx;
        while let Some(plug) = self.plugs.pop_front() {
            self.add_execution_log(plug.as_ref());
            match plug.call(c).await {
                PlugResult::Continue(ctx) => c = ctx,
                PlugResult::Terminate(ctx) => {
                    return PipelineResponse::new(ctx, self.executed, None)
                }
                PlugResult::NewPipe { ctx, plugs } => {
                    let pipeline = Self::new(plugs, Some(self.executed.clone()));
                    return pipeline.execute(ctx).await;
                }
                PlugResult::Err { ctx, err } => {
                    return PipelineResponse::new(ctx, self.executed, Some(err))
                }
            }
        }

        PipelineResponse::new(c, self.executed, None)
    }

    fn add_execution_log(&mut self, plug: &dyn Plug<Ctx>) {
        self.executed.push(plug.to_string());
    }
}

#[macro_export]
macro_rules! try_with {
    ($ctx:ident, $exp:expr) => {
        match $exp {
            Ok(v) => v,
            Err(e) => {
                return simple_pipeline::PlugResult::Err {
                    ctx: $ctx,
                    err: simple_pipeline::PipelineError::Internal(e.to_string()),
                }
            }
        }
    };
}

#[macro_export]
macro_rules! ctx_take {
    ($plug:ident, $ctx:ident, $name:ident) => {
        simple_pipeline::try_with!(
            $ctx,
            $ctx.$name.take().ok_or_else(|| {
                simple_pipeline::PipelineError::InvalidContext(format!(
                    "{}: Cannot take {} for ctx",
                    $plug.to_string(),
                    stringify!($name)
                ))
            })
        )
    };
}

#[macro_export]
macro_rules! ctx_ref {
    ($plug:ident, $ctx:ident, $name:ident) => {
        simple_pipeline::try_with!(
            $ctx,
            $ctx.$name.as_ref().ok_or_else(|| {
                simple_pipeline::PipelineError::InvalidContext(format!(
                    "{}: Cannot take {} for ctx",
                    $plug.to_string(),
                    stringify!($name)
                ))
            })
        )
    };
}

#[macro_export]
macro_rules! ctx_mut {
    ($plug:ident, $ctx:ident, $name:ident) => {
        simple_pipeline::try_with!(
            $ctx,
            $ctx.$name.as_mut().ok_or_else(|| {
                simple_pipeline::PipelineError::InvalidContext(format!(
                    "{}: Cannot take {} for ctx",
                    $plug.to_string(),
                    stringify!($name)
                ))
            })
        )
    };
}
