
pub trait Run<T: Task> {
    /// Runs the task on the underlying executor.
    fn run(&self, task: T);
}

/// A value that can run a unit of work.
pub trait Task : Send {
    /// Run the unit of work
    fn run(self);
}

impl<F: FnOnce() + Send> Task for F {
    fn run(self) {
        self()
    }
}

// ToDO unneeded with v1 of rust. This is to get
// around the fact that FnBox and the like are
// are unstable
pub trait TaskBox: Send {
    fn run_boxes(self: Box<Self>);
}
impl Task for Box<TaskBox>{
    fn run(self) {
        self.run_boxes();
    }
}
impl<F: FnOnce() + Send > TaskBox for F {
    fn run_boxes(self: Box<Self>) {
        self.run()
    }

}

