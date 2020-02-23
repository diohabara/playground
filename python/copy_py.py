# coding: utf-8
import copy

a = [1, 2, [3, 4]]
b = copy.copy(a)
b[0] = 29
b[2][0] = 4649
print("a = {}, b = {}".format(a, b))

c = [5, 6, [7, 8]]
d = copy.deepcopy(c)
d[0] = 810
d[2][0] = 114514
print("c = {}, d = {}".format(c, d))
