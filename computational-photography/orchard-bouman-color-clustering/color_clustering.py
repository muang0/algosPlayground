import cv2
import numpy as np
import logging
largest_eigenval = 0
node_to_split = None
cluster_indices_arrs = []
indices_arr_shape = None


class Node:
    '''
    Node class for represing an Orchard Bouman color grouping
    Orchard Bouman represents quantized colors using binary tree
    More details and formulas can be found from their paper linked below
    https://pdfs.semanticscholar.org/fc50/a3950d6ce54717b945079329069dcd8ccb7a.pdf
    
    attributes
    ----------
    cn : numpy.ndarray
        the color values of the cluster
    cn_i : numpy.ndarray
        the indices of the respective color values from the initial neighborhood
    child_l, child_r : class Node
        child node objects
    qn : numpy.ndarray
        quantization value of the cluster.  Equal to the cluster mean
    e_eigenval : numpy.float64
        eigenvalue of the clusters principal eigenvector
    e_eigenvect : numpy.float64
        the clusters principal eigenvector
    e_eigenvect_t : numpy.float64
        the principal eigenvector transposed

    methods
    ----------
    __init__(self,cn,weights=None) : initializes the node object
        cn : numpy.ndarray
            nx3 shape representing 'n' colors in the cluster
        weights : numpy.ndarray
            'n' shape array representing weights of the 'n' colors in the cluster
    split() : splits the color cluster into two sub clusters
    '''
    def __init__(self,cn,cn_i,weights=None):
        self.cn = cn
        self.cn_i = cn_i
        self.child_l = None
        self.child_r = None
        # Rn, mn, Nn come from (1) in Orchard Bouman
        Rn = np.zeros((3,3),np.uint64)
        mn = np.zeros((3),np.uint64)
        Nn = 0
        for i in range(0,cn.shape[0]):
            xs_t = np.transpose([cn[i,:]])
            if weights is None:
                Rn += (xs_t * cn[i,:]).astype(np.uint64)
                mn += cn[i,:]
                Nn = cn.shape[0]
            else:
                Rn += (weights[i] * xs_t * cn[i,:]).astype(np.uint64)
                mn += weights[i] * cn[i,:]
                Nn += weights[i]
        self.qn = mn/Nn
        mn_t = np.transpose([mn])
        cn_covar = Rn - (1/Nn)*mn*mn_t
        cn_eigenval, cn_eigenvect = np.linalg.eig(cn_covar)
        e_eigenval_i = np.argmax(np.abs(cn_eigenval))       # index of principal eigenvector
        self.e_eigenval = abs(cn_eigenval[e_eigenval_i])
        self.e_eigenvect = cn_eigenvect[:,e_eigenval_i]     # principal eigenvector
        self.e_eigenvect_t = np.transpose([self.e_eigenvect])

    def split(self):
        left_colors = []
        left_indices = []
        right_colors = []
        right_indices = []
        for xs_i in range(0,self.cn.shape[0]):
            if np.dot(self.e_eigenvect_t.reshape(3,),self.cn[xs_i,:]) <= np.dot(self.e_eigenvect_t.reshape(3,),self.qn):    # problem line
                left_colors.append(self.cn[xs_i,:])
                left_indices.append(self.cn_i[xs_i,:])
            else:
                right_colors.append(self.cn[xs_i,:])
                right_indices.append(self.cn_i[xs_i,:])
        left_color_arr = np.zeros((len(left_colors),3),np.uint16)
        right_color_arr = np.zeros((len(right_colors),3),np.uint16)
        left_color_indices = np.zeros((len(left_colors),2),np.uint16)
        right_color_indices = np.zeros((len(right_colors),2),np.uint16)
        for i in range(0,len(left_colors)):
            left_color_arr[i,:] = left_colors[i]
            left_color_indices[i,:] = left_indices[i]
        for i in range(0,len(right_colors)):
            right_color_arr[i,:] = right_colors[i]
            right_color_indices[i,:] = right_indices[i]
        self.child_l = Node(left_color_arr,left_color_indices)     # TODO implement weights
        self.child_r = Node(right_color_arr,right_color_indices)    # TODO implement weights


def find_node_to_split(node):
    '''
    this function finds the next leaf node to split (node with largest eigenval)
    this is a recursive function that uses in order traversal strategy to update global vars \
        representing next node to split

    Parameters
    ----------
    root : class Node
        root node of the color quantization binary tree
    
    '''
    global largest_eigenval
    global node_to_split
    if node is not None:
        find_node_to_split(node.child_l)
        if node.child_l is None and node.child_r is None:
            if node.e_eigenval > largest_eigenval:
                node_to_split = node
                largest_eigenval = node.e_eigenval
        find_node_to_split(node.child_r)


def extract_cluster_indices(node):
    '''
    this function extracts the pixel indices of the M clusters into a global var

    Parameters
    ----------
    root : class Node
        root node of the color quantization binary tree
    
    '''
    global cluster_indices_arrs
    if node is not None:
        extract_cluster_indices(node.child_l)
        if node.child_l is None and node.child_r is None:
            tmp_indices = np.zeros(indices_arr_shape,np.bool)
            for i in node.cn_i:
                tmp_indices[i[0],i[1]] = True
            cluster_indices_arrs.append(np.copy(tmp_indices))
        extract_cluster_indices(node.child_r)


def quantize_colors(neighborhood,m,weights=None):
    '''
    this function quantizes the colors in a neighborhood into m distinct clusters

    1. Set C1 = S (neighborhood flattened)
    2. Calculatate R1, m1, and N1
    3. Do the following M-1 times:
        a. Find leaf n such that lambda_n (eigenval) is largest
        b. Use (3) to form the new nodes 2n and 2n+1
        c. Calculate R, m, and N for the new nodes using (4)

    Parameters
    ----------
    neighborhood : numpy.ndarray
        neighborhood of colors to quantize
    weights : numpy.ndarray
        weights of the pixels from the neighborhood
    m : int
        number of clusters to quantize colors into

    Returns
    -------
    list<numpy.ndarray>
        arrays representing neighborhood indices of each cluster

    '''
    global largest_eigenval
    global node_to_split
    global cluster_indie_eigenvect_tape
    global indices_arr_shape
    s = neighborhood.reshape((neighborhood.shape[0]*neighborhood.shape[1],3))
    s_i_shape = np.ones((neighborhood.shape[0],neighborhood.shape[1]))
    s_i = np.argwhere(s_i_shape)
    if weights is not None:
        weights = weights.reshape((weights.shape[0]*weights.shape[1]))
    root = Node(s,s_i,weights)
    min_eigenval = 0
    node_to_split = None
    for i in range(0,m-1):
        largest_eigenval = 0
        node_to_split = None
        find_node_to_split(root)
        if node_to_split is None:
            logging.warning("In colorClustering.quantize_colors(), tree has been fully split before desired number of color clusters reached.")
            break
        node_to_split.split()
    # extract color indices for each cluster & convert into bool array of size (neighborhood[0],neighborhood[1])
    indices_arr_shape = (neighborhood.shape[0],neighborhood.shape[1])
    extract_cluster_indices(root)
    return cluster_indices_arrs     # TODO clean up globals/locals


# testing
img = cv2.imread("input/ball-pit.jpg")
quantized_color_indices = quantize_colors(img,5)
for i in range(0,len(quantized_color_indices)):
    color_indices_extended = np.repeat(quantized_color_indices[i][:, :, np.newaxis], 3, axis=2)
    quantized_color_img = np.where(color_indices_extended,img,(0,0,0))
    cv2.imwrite("output/ball-pit-cluster-{}.png".format(i),quantized_color_img)
