# Canny Edge Detection

This folder contains an implementation of a canny edge detector.

"Canny edge detection is a technique to extract useful structural information from different vision objects and dramatically reduce the amount of data to be processed. It has been widely applied in various computer vision systems." <a id="1">[2]</a> 

## Steps

1. Apply [Gaussian filter](https://en.wikipedia.org/wiki/Gaussian_filter) to smooth the image in order to remove the noise

2. Find the intensity gradients of the image

3. Apply non-maximum suppression to get rid of spurious response to edge detection

4. Apply double threshold to determine potential edges

5. Track edge by [hysteresis](https://en.wikipedia.org/wiki/Hysteresis): Finalize the detection of edges by suppressing all the other edges that are weak and not connected to strong edges

## References

<a id="1">[1]</a> 
John Canny (1986)
[A Computational Approach to Edge Detection](https://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.420.3300&rep=rep1&type=pdf)

<a id="1">[2]</a> 
Wikipedia
[Canny Edge Detector](https://en.wikipedia.org/wiki/Canny_edge_detector)
