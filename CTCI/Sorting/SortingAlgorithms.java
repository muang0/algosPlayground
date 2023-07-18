package Sorting;

import java.util.Arrays;
import java.util.function.Function;

public class SortingAlgorithms {
    // demonstrates bubble sort

    public static void main(String[] args){
        Integer[] testArray = {10,1,4,-3,9,2,7,1,3};
//        System.out.println(Arrays.toString(bubbleSort(testArray)));
//        System.out.println(Arrays.toString(selectionSort(testArray)));
        System.out.println(Arrays.toString(mergeSort(testArray, SortingAlgorithms::bubbleSort)));
    }

    public static Integer[] bubbleSort(Integer[] array){
        // runtime of O(n^2)
        // memory of O(1)
        Integer temp;
        boolean isSorted = true;
        while(true){
            isSorted = true;
            for(int i=1; i<array.length; i++){
                if(array[i] < array[i-1]){
                    temp = array[i];
                    array[i] = array[i-1];
                    array[i-1] = temp;
                    isSorted = false;
                }
            }
            if(isSorted==true){ break; }
        }
        return array;
    }

    public static Integer[] selectionSort(Integer[] array){
        // runtime of O(n^2)
        // memory O(1)
        Integer temp, minIndex = 0, min = array[0];
        for(int i=0; i<array.length; i++){
            for(int j=i+1; j<array.length; j++) {
                if(array[j] < min){ minIndex = j; }
            }
            temp = array[minIndex];
            array[minIndex] = array[i];
            array[i] = temp;
        }
        return array;
    }

    public static Integer[] mergeSort(Integer[] array, Function<Integer[], Integer[]> sortingAlgorithm){
        // runtime O(nlog(n))
        // memory fluctuates
        Integer[] array1 = Arrays.copyOfRange(array, 0, array.length/2);
        Integer[] array2 = Arrays.copyOfRange(array, array.length/2, array.length);
        array1 = sortingAlgorithm.apply(array1);
        array2 = sortingAlgorithm.apply(array2);
        int array1Index = 0, array2Index = 0;
        for(int i=0; i<array.length; i++){
            if(array1Index == array1.length){ array[i] = array2[array2Index]; array2Index++; }
            else if(array2Index == array2.length){ array[i] = array1[array1Index]; array1Index++; }
            else if(array1[array1Index] < array2[array2Index]){ array[i] = array1[array1Index]; array1Index++;}
            else{ array[i] = array2[array2Index]; array2Index++; }
        }
        return array;
    }

    public static Integer[] quickSort(Integer[] array){
        return array;
    }
}
