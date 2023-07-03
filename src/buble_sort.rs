pub fn buble_sort(arr: &mut [i32]) {
    let len = arr.len();
    let mut swapped = true;

    while swapped {
        swapped = false;

        for i in 0..len {
            if i < len - 1 && arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                swapped = true;
            }
        }

        if !swapped {
            break;
        }
    }
}
