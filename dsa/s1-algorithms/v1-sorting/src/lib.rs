use std::fmt::Debug;

mod b_rand;

// Bubble Sort with early exit (n^2), each pass shortens the unsorted range by one
pub fn bubble_sort<T: PartialOrd + Debug>(v: &mut [T]) {
    if v.is_empty() {
        return;
    }

    for p in 0..v.len() {
        let mut sorted = true;
        for i in 0..(v.len() - 1) - p {
            // p is early exit
            if v[i] > v[i + 1] {
                v.swap(i, i + 1);
                sorted = false;
            }
        }
        println!("-> {:?}", v);
        if sorted {
            break;
        }
    }
}

// Bidirectional Bubble Sort O(n^2), also known as Cocktail Shaker Sort, slightly better than bubble sort in practice
pub fn cocktail_shaker_sort<T: PartialOrd + Debug>(v: &mut [T]) {
    if v.is_empty() {
        return;
    }

    let mut start = 0;
    let mut end = v.len() - 1;
    let mut sorted;

    loop {
        sorted = true;

        // Forward pass
        for i in start..end {
            if v[i] > v[i + 1] {
                v.swap(i, i + 1);
                sorted = false;
            }
        }
        println!("→ {:?}", v);

        if sorted {
            break;
        }

        end -= 1;
        sorted = true;

        // Backward pass
        for i in (start..end).rev() {
            if v[i] > v[i + 1] {
                v.swap(i, i + 1);
                sorted = false;
            }
        }
        println!("← {:?}", v);

        if sorted {
            break;
        }

        start += 1;
    }
}

// Insertion Sort O(n^2), builds sorted prefix by inserting each element into its correct position
pub fn insertion_sort<T: PartialOrd + Debug>(v: &mut [T]) {
    if v.is_empty() {
        return;
    }

    // Check if sorted with bubble sort O(n)
    let mut sorted = true;
    for i in 0..v.len() - 1 {
        if v[i] > v[i + 1] {
            sorted = false;
            break;
        }
    }
    if sorted {
        return;
    }

    // Insertion Sort
    for i in 1..v.len() {
        let mut j = i;
        while j > 0 && v[j] < v[j - 1] {
            v.swap(j, j - 1);
            j -= 1;
        }
        println!("IS -> {:?}", v);
    }
}

// Merge sort O(n * log(n)) using recursion and the divide and conquer approach
pub fn merge_sort<T: PartialOrd + Debug>(mut v: Vec<T>) -> Vec<T> {
    // Step 1: Divide the vector into halves recursively
    //         We can divide n by 2 approximately log2(n) times → O(log n) levels
    // Step 2: Merge sorted halves back together
    //         At each level we process all n elements → O(n) per level
    // Total work: O(n + 2*(n/2) + 4*(n/4) + 8*(n/8) + ...) = O(n * log n)

    println!("MS {}: {:?}", v.len(), v);

    // Base case
    if v.len() <= 1 {
        return v;
    }

    // Result variable
    let mut res = Vec::with_capacity(v.len());

    // Step 1: Divide the vector into halves recursively
    let b = v.split_off(v.len() / 2);
    let a = merge_sort(v);
    let b = merge_sort(b);

    // Step 2: Merge sorted halves together again, add whichever is lowest the front of a or the front of b
    let mut a_it = a.into_iter();
    let mut b_it = b.into_iter();
    let mut a_peek = a_it.next();
    let mut b_peek = b_it.next();

    while let (Some(a_val), Some(b_val)) = (&a_peek, &b_peek) {
        if b_val < a_val {
            res.push(b_peek.take().unwrap());
            b_peek = b_it.next();
        } else {
            res.push(a_peek.take().unwrap());
            a_peek = a_it.next();
        }
    }

    // Push remaining elements (only one side will still have items)
    if let Some(a_val) = a_peek {
        res.push(a_val);
        res.extend(a_it);
    } else if let Some(b_val) = b_peek {
        res.push(b_val);
        res.extend(b_it);
    }

    res

    /* // Nesting matches
    loop {
        match a_peek {
            Some(ref a_val) => match b_peek {
                Some(ref b_val) => {
                    if b_val < a_val {
                        res.push(b_peek.take().unwrap());
                        b_peek = b_it.next();
                    } else {
                        res.push(a_peek.take().unwrap());
                        a_peek = a_it.next();
                    }
                }
                None => {
                    res.push(a_peek.take().unwrap());
                    res.extend(a_it);
                    return res;
                }
            },
            None => {
                if let Some(b_val) = b_peek {
                    res.push(b_val);
                }
                res.extend(b_it);
                return res;
            }
        }
    }
    */
}

// Pivot for Quick sort
pub fn pivot<T: PartialOrd + Debug>(v: &mut [T]) -> usize {
    // Move first element to the correct place
    // Everything lower should be before it,
    // everything higher should be after it
    // return it's location

    // Select a random pivot index to avoid worst-case performance in quicksort on sorted or patterned data.
    let mut p = b_rand::rand(v.len());
    v.swap(p, 0);
    p = 0;
    for i in 1..v.len() {
        // Ignore the pivot (at index 0); partition the rest: 1..v.len()
        if v[i] >= v[p] {
            // println!("QS: {:?}", v);
            // println!(
            //     "Case: {i}, v[{i}]: {:?} <= v[{p}]: {:?}, do nothing!\n",
            //     v[i], v[p]
            // );
        } else if v[i] < v[p] {
            // Move our pivot forward 1, and put this element before it
            // println!("QS: {:?}", v);
            // println!("Case: {i}, v[{i}]: {:?} < v[{p}]: {:?}, swap!", v[i], v[p]);
            // println!("v[p + 1]: {:?}, v[i]: {:?} -> swapping", v[p + 1], v[i]);
            v.swap(p + 1, i);
            // println!("v[p + 1]: {:?}, v[i]: {:?} -> swapped", v[p + 1], v[i]);
            // println!("QS: {:?}", v);
            // println!("v[p]: {:?}, v[p + 1]: {:?} -> swapping", v[p], v[p + 1]);
            v.swap(p, p + 1);
            // println!("v[p]: {:?}, v[p + 1]: {:?} -> swapped", v[p], v[p + 1]);
            // println!("QS: {:?}\n", v);
            p += 1
        }
    }
    p
}

// Quick sort: O(n log n) average, O(n²) worst-case; recursively partitions around a pivot.
pub fn quick_sort<T: PartialOrd + Debug>(v: &mut [T]) {
    if v.len() <= 1 {
        return;
    }

    let p = pivot(v);
    println!("{:?}", v);

    let (a, b) = v.split_at_mut(p);
    quick_sort(a);
    quick_sort(&mut b[1..]); // Middle element already sorted
}

// Threaded Quick sort O(n * log(n)) average using divide & conquer with parallel threads
struct RawSend<T>(*mut [T]); // One element tuple

unsafe impl<T> Send for RawSend<T> {}

pub fn threaded_quick_sort_v2018<T: 'static + PartialOrd + Debug + Send>(v: &mut [T]) {
    if v.len() <= 1 {
        return;
    }

    let p = pivot(v);
    println!("{:?}", v);

    let (a, b) = v.split_at_mut(p);

    let raw_a: *mut [T] = a as *mut [T];
    let raw_s = RawSend(raw_a);

    unsafe {
        let handle = std::thread::spawn(move || {
            let raw_s = raw_s; //moves the full RawSend into threaded scope

            threaded_quick_sort_v2018(&mut *raw_s.0);
        });

        threaded_quick_sort_v2018(&mut b[1..]);

        // Compiler doesn't know we join these
        handle.join().expect("Thread panicked during quicksort");
    }
}

// Parallel Quick sort with Rayon: O(n log n) average; partitions and recurses in parallel
pub fn quick_sort_rayon<T: Send + PartialOrd + Debug>(v: &mut [T]) {
    if v.len() <= 1 {
        return;
    }

    let p = pivot(v);
    println!("{:?}", v);

    let (a, b) = v.split_at_mut(p);

    // rayon::join runs two closures in parallel using a thread pool
    // It may split work into smaller tasks and balance them across threads
    // Efficiently parallelizes divide-and-conquer algorithms like quicksort
    rayon::join(|| quick_sort_rayon(a), || quick_sort_rayon(&mut b[1..]));
}

#[cfg(test)]
mod tests {
    use super::*;

    // O(n^2) Bubble Sort
    #[test]
    fn test_bubble_sort() {
        let mut v = vec![4, 6, 1, 8, 11, 13, 3];
        bubble_sort(&mut v);
        assert_eq!(v, vec![1, 3, 4, 6, 8, 11, 13])
    }

    // O(n^2) Bidirectional Bubble Sort, also known as Cocktail Shaker Sort
    #[test]
    fn test_cocktail_shaker_sort() {
        let mut v = vec![4, 6, 1, 8, 11, 13, 3];
        cocktail_shaker_sort(&mut v);
        assert_eq!(v, vec![1, 3, 4, 6, 8, 11, 13]);
    }

    // O(n^2) Insertion Sort
    #[test]
    fn test_insertion_sort() {
        let mut v = vec![4, 6, 1, 8, 11, 13, 3];
        insertion_sort(&mut v);
        assert_eq!(v, vec![1, 3, 4, 6, 8, 11, 13])
    }

    // O(n * log(n)) Merge sort
    #[test]
    fn test_merge_sort() {
        let v = vec![4, 6, 1, 8, 11, 13, 3];
        let v = merge_sort(v);
        assert_eq!(v, vec![1, 3, 4, 6, 8, 11, 13]);
    }

    // Pivot for Quick sort
    #[test]
    fn test_pivot() {
        let mut v = vec![4, 6, 1, 8, 11, 13, 3];
        let p = pivot(&mut v);

        for x in 0..v.len() {
            assert!((v[x] < v[p]) == (x < p));
        }
    }

    // O(n * log(n)) Quick sort
    #[test]
    fn test_quick_sort() {
        let mut v = vec![4, 6, 1, 8, 11, 13, 3];
        quick_sort(&mut v);
        assert_eq!(v, vec![1, 3, 4, 6, 8, 11, 13]);

        let mut v = vec![1, 3, 4, 6, 8, 11, 13];
        quick_sort(&mut v);
        assert_eq!(v, vec![1, 3, 4, 6, 8, 11, 13]);

        // panic!();
    }

    // O(n * log(n)) Threaded Quick sort
    #[test]
    fn test_threaded_quick_sort_v2018() {
        let mut v = vec![4, 6, 1, 8, 11, 13, 3];
        threaded_quick_sort_v2018(&mut v);
        assert_eq!(v, vec![1, 3, 4, 6, 8, 11, 13]);

        // panic!();
    }

    // O(n * log(n)) Rayon Quick sort
    #[test]
    fn test_quick_sort_rayon() {
        let mut v = vec![4, 6, 1, 8, 11, 13, 3];
        quick_sort_rayon(&mut v);
        assert_eq!(v, vec![1, 3, 4, 6, 8, 11, 13]);

        // panic!();
    }
}
