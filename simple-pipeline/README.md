# Simple Pipeline

Usage:

```rust
// create the plugs for the pipeline
let plugs: Vec<Box<dyn Plug<PlugCtx>>> = vec![
    Box::new(...),
    Box::new(...),
    ...
];

// initialize the pipeline
let pipeline = Pipeline::new(plugs, None);

// execute the pipeline
let result = pipeline.execute(ctx).await;
```
