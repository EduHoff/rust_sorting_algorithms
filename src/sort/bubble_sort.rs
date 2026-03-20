pub fn sort<T: PartialOrd + Copy>(array: &mut [T]) {
    
    for i in 0..array.len(){
        let mut swapped = false;

        for j in 0..(array.len()-(i+1)){
            if array[j] > array[j+1]{
                let aux = array[j];
                array[j] = array[j+1];
                array[j+1] = aux;

                swapped=true;
            }
        }

        if !swapped {break};
    }
}