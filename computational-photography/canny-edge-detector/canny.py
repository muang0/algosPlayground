import cv2
import numpy as np

# implement a canny ege detector
# https://en.wikipedia.org/wiki/Canny_edge_detector
def canny(img, gauss_dim, low_threshold, high_threshold):
    # convert to grayscale
    if img.ndim == 3:
        img = cv2.cvtColor(img,cv2.COLOR_BGR2GRAY)
    # gaussian blur image
    gauss_img = cv2.GaussianBlur(img,gauss_dim,0)
    # calculate intensity gradient and angles
    sobel_x = cv2.Sobel(gauss_img, cv2.CV_64F, 1, 0)
    sobel_y = cv2.Sobel(gauss_img, cv2.CV_64F, 0, 1)
    gradient = np.sqrt(np.square(sobel_x)+np.square(sobel_y))
    gradient_angles = np.arctan2(sobel_y,sobel_x)
    rounded_angles = np.round(gradient_angles/(np.pi/4))*(np.pi/4)
    rounded_angles = np.where(rounded_angles < 0, rounded_angles + np.pi,rounded_angles)
    rounded_angles[rounded_angles==np.pi] = 0
    # non-maximum suppression
    #   create shifted arrays
    grad_shifted_left, grad_shifted_right, grad_shifted_up, grad_shifted_down, grad_shifted_leftup, grad_shifted_rightdown, \
        grad_shifted_rightup, grad_shifted_leftdown = shift_array(gradient)
    #   thin edges
    edge_thinned = np.copy(gradient)
    edge_thinned = np.where(((rounded_angles==0) & ((gradient < grad_shifted_left) | (gradient < grad_shifted_right))),0,edge_thinned)
    edge_thinned = np.where(((rounded_angles==np.pi/2) & ((gradient < grad_shifted_up) | (gradient < grad_shifted_down))),0,edge_thinned)
    edge_thinned = np.where(((rounded_angles==np.pi/4) & ((gradient < grad_shifted_leftup) | (gradient < grad_shifted_rightdown))),0,edge_thinned)
    edge_thinned = np.where(((rounded_angles==(3*np.pi/4)) & ((gradient < grad_shifted_rightup) | (gradient < grad_shifted_leftdown))),0,edge_thinned)
    # apply low threshold
    edge_thinned[edge_thinned<low_threshold]=0
    thresholded = edge_thinned
    # apply hysteresis-- 8 pixel connected neighborhood
    thresholded_shifted_left, thresholded_shifted_right, thresholded_shifted_up, thresholded_shifted_down, thresholded_shifted_leftup, \
        thresholded_shifted_rightdown, thresholded_shifted_rightup, thresholded_shifted_leftdown = shift_array(thresholded)
    hysteresis_res = np.where((thresholded>high_threshold) | (thresholded_shifted_left>high_threshold) | (thresholded_shifted_right>high_threshold) | \
        (thresholded_shifted_up>high_threshold) | (thresholded_shifted_down>high_threshold) | (thresholded_shifted_leftup>high_threshold) | \
            (thresholded_shifted_rightdown>high_threshold) | (thresholded_shifted_rightup>high_threshold) | (thresholded_shifted_leftdown>high_threshold),thresholded,0)
    return hysteresis_res


def shift_array(arr):
    # shift array with duplication strategy for inserted row/col
    shifted_left = np.roll(arr,-1,1)
    shifted_left[:,-1] = shifted_left[:,-2]
    shifted_right = np.roll(arr,1,1)
    shifted_right[:,0] = shifted_right[:,1]
    shifted_up = np.roll(arr,-1,0)
    shifted_up[-1,:] = shifted_up[-2,:]
    shifted_down = np.roll(arr,1,0)
    shifted_down[0,:] = shifted_down[1,:]
    shifted_leftup = np.roll(shifted_left,-1,0)
    shifted_leftup[-1,:] = shifted_leftup[-2,:]
    shifted_rightdown = np.roll(shifted_right,1,0)
    shifted_rightdown[0,:] = shifted_rightdown[1,:]
    shifted_rightup = np.roll(shifted_right,-1,0)
    shifted_rightup[-1,:] = shifted_rightup[-2,:]
    shifted_leftdown = np.roll(shifted_left,1,0)
    shifted_leftdown[0,:] = shifted_leftdown[1,:]
    return shifted_left, shifted_right, shifted_up, shifted_down, shifted_leftup, shifted_rightdown, shifted_rightup, shifted_leftdown

        

# apply canny edge detector
img = cv2.imread("input/tech-tower.jpg")
canny_res = canny(img,(5,5),30,80)
cv2.imwrite("output/tech-tower-canny.jpg",canny_res)
