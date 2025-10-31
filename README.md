# The Ray Tracer Challenge

I have already implemented RTC in Javascript, so it felt natural to also tackle that challenge while learning Rust.

I have since found [@MrJakob](https://www.youtube.com/@MrJakob)'s [Youtube series](https://www.youtube.com/playlist?list=PLy68GuC77sUTyOUvDhVboQoOlHoa4XrSO) and [companion Github repo](https://github.com/jakobwesthoff/the_ray_tracer_challenge_in_rust) where he does the same, and I have viewed his content sequentially after first implementing it myself. Secondly, after being stuck for a while on a few select conundrums, I have gone and done it "his way". But for the most part, I have done my own thing, and I'm sure there are much to be desired in optimalization of the code. That is part of the challenge.

As I have now just reached the "Putting it together" part in Chapter 5, where we for the first time actually *render* something, I was amazed at the speed of Rust. 

When I did this challenge in Javascript, the very first render took a good _149_ seconds to finish. I was able to quickly get that down to _12.9_ seconds by replacing the generic arrays with `Float32Array` that only takes 32-bit floats as values, at the cost of some functionality.

For comparison, that JS code renders on my current hardware in about _50_ seconds, so use that as a baseline. When I came to chapter 5 with Rust, the initial debug-profile test render finished in... \*drum roll\* _~873 ms_! 

Then I decided to go where JS cannot; compilation optimized for release! 
The Release build of this code renders in _~15 ms_. Now that's progress!

In Chapter 6, I implemented threads, running the ray tracing in 16 threads across my 8 CPU cores! 🔥🔥

As I write this, Chapter 9 is done, and I'm way into Chapter 10. Patterns are forming... literally!

## Testing

### Install the dependencies
`cargo build`

### Run tests
`cargo test`

### Run the application

For debugging and testing, ie.  `cargo run --bin chapter_07`

Optimized build, ie. `cargo run --profile release --bin chapter_09`

# Contributing
If you peruse this code and find room for improvement (which is inevitable), please let me know via an Issue, and let's discuss the matter. I'm doing this to learn Rust from basic principles, so I will want to understand the full rationale behind the change.


