from matplotlib import pyplot,cm
import pandas as pd
import numpy
from matplotlib import animation as anime
from matplotlib.animation import PillowWriter
from PIL import Image
data_u = pd.read_csv("./data_u.csv").values
data_v = pd.read_csv("./data_v.csv").values
data_p = pd.read_csv("./data_p.csv").values
data_c1 = pd.read_csv("./data_c1.csv").values
data_c2 = pd.read_csv("./data_c2.csv").values
data_phi = pd.read_csv("./data_phi.csv").values
nt = 1000
dt = 0.01
nx = 21
ny = 10*5
fig = pyplot.figure(figsize=(6, 4))
# fig = pyplot.figure(figsize=(11,7), dpi=100)
ax = fig.add_subplot(1, 1, 1)
l_ch = 20.
h_el = 2.# Thickness of electrolyte
h_fl = 1.# Thickness of fuel electrode layer
h_al = 1.# Thickness of air electrode layer
h_fch = 3.# Thickness of fuel channel
h_ach = 3.# Thickness of air channel
x = numpy.linspace(0., l_ch, nx)
y = numpy.linspace(0., h_ach+h_al+h_el+h_fl+h_fch, ny)
# numpy.append(y,numpy.linspace(h_ach,h_ach+h_al, ny))
# numpy.append(y,numpy.linspace(h_ach+h_al,h_ach+h_al+h_el, ny))
# numpy.append(y,numpy.linspace(h_ach+h_al+h_el,h_ach+h_al+h_el+h_fl, ny))
# numpy.append(y,numpy.linspace(h_ach+h_al+h_el+h_fl,h_ach+h_al+h_el+h_fl+h_fch, ny))
# print(y)
X, Y = numpy.meshgrid(x, y)

def update_func(n):
    u = data_u[ny*n:ny*(n+1),:]
    v = data_v[ny*n:ny*(n+1),:]
    p = data_p[ny*n:ny*(n+1),:]
    c1 = data_c1[ny*n:ny*(n+1),:]
    c2 = data_c2[ny*n:ny*(n+1),:]
    phi = data_phi[ny*n:ny*(n+1),:]
    ax.clear()
    
    # plotting the pressure field as a contour
    # pyplot.contourf(X, Y, p, alpha=0.5, cmap=cm.viridis)  
    pyplot.contourf(X, Y, c1, alpha=0.5, cmap=cm.viridis) 
    if n==0:
        pyplot.colorbar()
    # plotting the pressure field outlines
    # pyplot.contour(X, Y, p, cmap=cm.viridis)  
    # pyplot.contourf(X, Y, c1, alpha=0.5, cmap=cm.viridis) 
    # plotting velocity field
    pyplot.quiver(X[::1, ::1], Y[::1, ::1], u[::1, ::1], v[::1, ::1]) 
    ax.set_xlabel('X')
    ax.set_ylabel('Y')
    # pyplot.show()

    ax.set_title('Time: ' + '{:.2f}'.format(dt * n))

ani = anime.FuncAnimation(fig, update_func, frames=nt-1, interval=1, repeat=True)
# pyplot.show()
ani.save('data_plot.gif', writer='pillow')


# for n in range(nt){
    
#     # plotting the pressure field as a contour
#     # pyplot.contourf(X, Y, p, alpha=0.5, cmap=cm.viridis)  
#     pyplot.contourf(X, Y, c1, alpha=0.5, cmap=cm.viridis) 
#     pyplot.colorbar()
#     # plotting the pressure field outlines
#     # pyplot.contour(X, Y, p, cmap=cm.viridis)  
#     pyplot.contourf(X, Y, c1, alpha=0.5, cmap=cm.viridis) 
#     # plotting velocity field
#     pyplot.quiver(X[::1, ::1], Y[::1, ::1], u[::1, ::1], v[::1, ::1]) 
#     pyplot.xlabel('X')
#     pyplot.ylabel('Y')
#     pyplot.show()

#     fig = pyplot.figure(figsize=(11, 7), dpi=100)
#     pyplot.contourf(X, Y, phi, alpha=0.5, cmap=cm.viridis)
#     pyplot.colorbar()
#     pyplot.contour(X, Y, phi, cmap=cm.viridis)
#     # pyplot.streamplot(X, Y, u, v)
#     pyplot.quiver(X[::1, ::1], Y[::1, ::1], u[::1, ::1], v[::1, ::1]) 
#     pyplot.xlabel('X')
#     pyplot.ylabel('Y')
#     pyplot.show()
# }