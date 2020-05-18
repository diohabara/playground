def sort(x, up):
    u"""
    sort x in the direction of up. if up is True, then ascending order.
    the number of x should be a power of two.
    otherwise the result would be strange
    """

    if len(x) <= 1:
        return x
    else:
        # step 1a
        # the first half is ascending, the second half is descending
        mid_point = len(x) // 2
        first = sort(x[:mid_point], True)
        second = sort(x[mid_point:], False)

        # step 1b
        # merge two divided lists
        x1 = first + second

        # step 2
        return _sub_sort(x1, up)

def _sub_sort(x, up):
    u"""
    compare and exchange bitonically sorted x's first half and second half as directed by up, and recursively apply subsort to each part.
    """
    if len(x) == 1:
        return x
    else:
        # step 2a
        # compare bitonic array which has n elements every each n/2 elements
        # exchange as directed by up
        _compare_and_swap(x, up)

        # step 2b
        # divide data array half, iterativelty apply _sub_sort to each part
        mid_point = len(x) // 2
        first = _sub_sort(x[:mid_point], up)
        second = _sub_sort(x[mid_point:], up)

        # step 2c
        # merge two divided lists
        return first + second

def _compare_and_swap(x, up):
    u"""
    compare bitonic array's elements every n/2 elements,
    and exchange as directed by up
    """

    mid_point = len(x) // 2
    for i in range(mid_point):
        if (x[i] > x[mid_point + i]) == up:
            # exchange elements
            x[i], x[mid_point + i] = x[mid_point + i], x[i]
