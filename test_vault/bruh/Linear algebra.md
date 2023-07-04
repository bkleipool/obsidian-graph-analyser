---
tags: mathematics/linear_algebra, topic
---
Linear algebra is a field of mathematics which concerns [[System of linear equations |systems of linear equations]]. Linear equations have the form:
$$a_{1}x_{1}+a_{2}x_{2}+\dots+a_{n}x_{n}=b$$
Systems of linear equations can be represented by a product of a [[Vector |vector]] and a [[Matrix |matrix]]. Where vectors represent a point in space (or a list of numbers $\mathbf{x}=[x_{1},x_{2},\dots,x_{n}]$) and matrices represent a linear transformation of every point in space to another point in space (or a block of numbers $A=[\mathbf{u}_{1},\mathbf{u}_{2},\dots,\mathbf{u}_{n}]$).
$$\left\{\begin{matrix}
a_{11}x_{1}+a_{12}x_{2}+\dots+a_{1j}x_{j}=b_{1} \\
a_{21}x_{1}+a_{22}x_{2}+\dots+a_{2j}x_{j}=b_{2} \\
\vdots \\
a_{i1}x_{1}+a_{i2}x_{2}+\dots+a_{ij}x_{j}=b_{i}
\end{matrix}\right. \longrightarrow
\begin{bmatrix}
a_{11} & a_{12} & \dots & a_{1j} \\
a_{21} & a_{22} & \dots & a_{2j} \\
\vdots & \vdots & \ddots & \vdots \\
a_{i1} & a_{i2} & \dots &a_{ij}
\end{bmatrix}
\begin{bmatrix}
x_{1} \\ x_{2} \\ \vdots \\ x_{j}
\end{bmatrix}
=
\begin{bmatrix}
b_{1} \\ b_{2} \\ \vdots \\ b_{i}
\end{bmatrix}$$
The system of equations can also be written in a more compact form:
$$A\mathbf{x}=\mathbf{b}$$
A solution to the system of equations can then be found by solving for the vector of unknowns $\mathbf{x}$:
$$\mathbf{x}=A^{-1}\mathbf{b}$$

## Topics in linear algebra
- [[System of linear equations]]
- [[Matrix]]
- [[Vector]]