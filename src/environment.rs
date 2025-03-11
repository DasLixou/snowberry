// TODO: find a solution to get it working without specialization

// trait WithEnv<T> {
//     fn get(&self) -> &T;
// }

// impl<Other, T> WithEnv<T> for (Other, T) {
//     fn get(&self) -> &T {
//         &self.1
//     }
// }

// impl<Down: WithEnv<T>, T, Other> WithEnv<T> for (Down, Other) {
//     fn get(&self) -> &T {
//         self.0.get()
//     }
// }
