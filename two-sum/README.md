# Two-Sum

## Initial Thoughts

This is intentionally a simple problem, but I am not aiming for optimizing yet. Going to initially just iterate over the list with n^2 approach and figure out the answer.

## After

Looks like this worked well enough, good for getting my feet wet in some Rust.

## Follow-up: Can you come up with an algorithm that is less than O(n^2) time complexity?

I got to thinking about how the same numbers are sort of repeatedly checked. For example: By checking the first number with the outer loop, when the inner loop starting back at index zero is duplicated work. Perhaps changing the logic to only start the inner loop based on the outer loop's position + 1?
