// 933. Number of Recent Calls
//
// You have a RecentCounter class which counts the number of recent requests
// within a certain time frame.
//
// Implement the RecentCounter class:
// - RecentCounter() Initializes the counter with zero recent requests.
// - int ping(int t) Adds a new request at time t, where t represents some
//   time in milliseconds, and returns the number of requests that has happened
//   in the past 3000 milliseconds (including the new request). Specifically,
//   return the number of requests that have happened in the inclusive range [t - 3000, t].
//
// It is guaranteed that every call to ping uses a strictly larger value of t than the previous call.

struct RecentCounter {
    // TODO: Complete this struct.
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RecentCounter {
    fn new() -> Self {
        // TODO: Complete this function.
    }

    fn ping(&self, t: i32) -> i32 {
        // TODO: Complete this function.
    }
}

/**
 * Your RecentCounter object will be instantiated and called as such:
 * let obj = RecentCounter::new();
 * let ret_1: i32 = obj.ping(t);
 */
fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let times = vec![1, 100, 3001, 3002];
        let mut obj = RecentCounter::new();
        let mut ans = Vec::new();
        for t in times {
            let ret: i32 = obj.ping(t);
            ans.push(ret);
        }
        assert_eq!(ans, vec![1, 2, 3, 3]);
    }
}
