package sort

// selectionsort repeatedly finds min element from unsorted portion of array and appends
//   to sorted portion
func selectionsort(arr []int) ([]int, error) {
	for index1 := range arr {
		low := arr[index1]
		swapindex := index1
		for index2 := index1 + 1; index2 < len(arr); index2++ {
			if arr[index2] < low {
				low = arr[index2]
				swapindex = index2
			}
		}
		arr[index1], arr[swapindex] = arr[swapindex], arr[index1]
	}
	return arr, nil
}
