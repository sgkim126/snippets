from mpl_toolkits.mplot3d import Axes3D
from matplotlib import cm
from matplotlib.ticker import LinearLocator, FormatStrFormatter
import matplotlib.pyplot as plt
import numpy as np
import data

fig = plt.figure()
ax = fig.gca(projection='3d')

x_elem = [(i + 1) * 4 * 8 for i in xrange(16)] # 8 * i
y_elem = [(i + 1) for i in xrange(12)] # 2 ** i

x = [x_elem for _ in y_elem]
y = [[y] * len(x_elem) for y in y_elem]

z = data.z

surf = ax.plot_surface(x, y, z, rstride=1, cstride=1, cmap=cm.coolwarm,
                               linewidth=0, antialiased=False)
ax.zaxis.set_major_locator(LinearLocator(10))
ax.zaxis.set_major_formatter(FormatStrFormatter('%.02f'))

fig.colorbar(surf, shrink=0.5, aspect=5)

plt.show()

