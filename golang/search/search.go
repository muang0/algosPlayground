package search

import "errors"

var errFooNFound error = errors.New("value not found in slice")

func linearsearch(arr []int, target int) (int, error) {
	for i, val := range arr {
		if val == target {
			return i, nil
		}
	}
	return 0, errFooNFound
}

func binarysearch(arr []int, target int, low int, high int) (int, error) {
	if high >= low {
		mid := low + ((high - low) / 2)
		if arr[mid] == target {
			return mid, nil
		} else if arr[mid] > target {
			return binarysearch(arr, target, low, mid-1)
		}
		return binarysearch(arr, target, mid+1, high)
	}
	return 0, errFooNFound
}

func jumpsearch(arr []int, target int, jump int) (int, error) {
	// check error conditions
	if arr[len(arr)-1] < target {
		return 0, errFooNFound
	} else if jump <= 0 {
		return 0, errors.New("Please specify a valid jump value")
	}
	// jump through array, checking if value is less than target
	for index, previndex := 0, 0; index < len(arr); index = index + jump {
		if arr[index] == target {
			return index, nil
		} else if arr[index] > target {
			// if value is less than target, iterate starting from previndex
			for subindex := previndex; subindex < index; subindex++ {
				if arr[subindex] == target {
					return subindex, nil
				}
			}
			return 0, errFooNFound
		}
		previndex = index
	}
	// iterate through remaining values if next jump is past limit
	remainder := len(arr) % jump
	for index := len(arr) - remainder; index < len(arr); index++ {
		if arr[index] == target {
			return index, nil
		}
	}
	return 0, errFooNFound
}

// interpolationsearch expects sorted, uniformly distributed array
func interpolationsearch(arr []int, target int, low int, high int) (int, error) {
	if high-low == 0 {
		return 0, errFooNFound
	}
	// calculate position given low, high indexes, target, and even distribution
	pos := low + ((target - arr[low]) * (high - low) / (arr[high] - arr[low]))
	if target == arr[pos] {
		return pos, nil
	} else if pos == low || pos == high {
		return 0, errFooNFound
	} else if target > arr[pos] {
		interpolationsearch(arr, target, pos+1, high)
	} else if target < arr[pos] {
		if pos < 1 {
			interpolationsearch(arr, target, low, pos)
		} else {
			interpolationsearch(arr, target, low, pos-1)
		}
	}
	return 0, errFooNFound
}

// exponentialsearch finds subarray with target, then calls binary on subarray
func exponentailsearch(arr []int, target int) (int, error) {
	if arr[0] == target {
		return 0, nil
	}
	for index := 1; index < len(arr); index = index * 2 {
		if arr[index] > target {
			return binarysearch(arr, target, 0, index)
		}
	}
	return 0, errFooNFound
}
