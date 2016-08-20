fn swap<T>(vec: &mut Vec<T>, i: usize, j: usize)
	where T: Copy {
	let tmp = vec[i];
	vec[i] = vec[j];
	vec[j] = tmp;
}

pub fn bubble_inplace<T>(vec: &mut Vec<T>)
	where T: PartialOrd + Copy {
	let mut did_swap = true;
	while did_swap {
		did_swap = false;
		for i in 0..(vec.len() - 1) {
			if vec[i] > vec[i + 1] {
				swap(vec, i, i + 1);
				did_swap = true;
			}
		}
	}
}

pub fn bubble<T>(vec: &Vec<T>) -> Vec<T>
	where T: PartialOrd + Copy {
	let mut new_vec = vec.clone();
	bubble_inplace(&mut new_vec);
	new_vec
}

pub fn selection_inplace<T>(vec: &mut Vec<T>)
	where T: PartialOrd + Copy {
	let mut minimum_index;
	for j in 0..(vec.len() - 1) {
		minimum_index = j;
		for i in j..vec.len() {
			if vec[i] < vec[minimum_index] {
				minimum_index = i;
			}
		}
		if minimum_index != j {
			swap(vec, j, minimum_index);
		}
	}
}

pub fn selection<T>(vec: &Vec<T>) -> Vec<T>
	where T: PartialOrd + Copy {
	let mut new_vec = vec.clone();
	selection_inplace(&mut new_vec);
	new_vec
}

pub fn insertion_inplace<T>(vec: &mut Vec<T>)
	where T: PartialOrd + Copy {
	let mut j;
	for i in 1..vec.len() {
	    j = i;
	    while j > 0 && vec[j - 1] > vec[j] {
	        swap(vec, j, j - 1);
	        j = j - 1
	    }
	}
}

pub fn insertion<T>(vec: &Vec<T>) -> Vec<T>
	where T: PartialOrd + Copy {
	let mut new_vec = vec.clone();
	insertion_inplace(&mut new_vec);
	new_vec
}


#[cfg(test)]
mod tests {
	use super::*;

    #[test]
    fn bubble_test() {
        let vec = vec![22, -31, 678, -131, 43, 7, -2];
        let mut sorted_vec = vec.clone();
        sorted_vec.sort();
        let my_sorted_vec = bubble(&vec);
        assert_eq!(my_sorted_vec, sorted_vec);
    }

    #[test]
    fn selection_test() {
        let vec = vec![22, -31, 678, -131, 43, 7, -2];
        let mut sorted_vec = vec.clone();
        sorted_vec.sort();
        let my_sorted_vec = selection(&vec);
        assert_eq!(my_sorted_vec, sorted_vec);
    }

    #[test]
    fn insertion_test() {
        let vec = vec![22, -31, 678, -131, 43, 7, -2];
        let mut sorted_vec = vec.clone();
        sorted_vec.sort();
        let my_sorted_vec = insertion(&vec);
        assert_eq!(my_sorted_vec, sorted_vec);
    }
}
