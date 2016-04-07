pub fn bubble_sort_inplace<T>(vec: &mut Vec<T>)
	where T: PartialOrd + Copy {
	let mut tmp: T;
	let mut did_swap: bool = true;
	while did_swap {
		did_swap = false;
		for i in 0..(vec.len() - 1) {
			if vec[i] > vec[i + 1] {
				tmp = vec[i];
				vec[i] = vec[i + 1];
				vec[i + 1] = tmp;
				did_swap = true;
			}
		}
	}
}

pub fn bubble_sort<T>(vec: &Vec<T>) -> Vec<T>
	where T: PartialOrd + Copy {
	let mut new_vec = vec.clone();
	bubble_sort_inplace(&mut new_vec);
	new_vec
}

#[cfg(test)]
mod tests {
	use super::*;

	fn vectors_equal<T>(vec1: &Vec<T>, vec2: &Vec<T>) -> bool
		where T: Eq {
		if vec1.len() != vec2.len() {
			return false;
		}
		for i in 0..(vec1.len()) {
			if vec1[i] != vec2[i] {
				return false;
			}
		}
		true
	}

    #[test]
    fn bubble_sort_test() {
        let vec = vec![22, -31, 678, -131, 43, 7, -2];
        let mut sorted_vec = vec.clone();
        sorted_vec.sort();
        let my_sorted_vec = bubble_sort(&vec);
        assert!(vectors_equal(&my_sorted_vec, &sorted_vec));
    }
}
