---
tags: mathematics/linear_algebra, concept
---
In [[Linear algebra]], a system of linear equations or a _linear system_ is a collection of equations involving the same variables^[https://en.wikipedia.org/wiki/System_of_linear_equations]. A system of linear equations can be represented by a [[Matrix |matrix]]:
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
A linear system can be solved by finding all the values $x_{1},x_{2},\dots,x_{j}$ such that all the equations are satisfied. There are 3 possibilities for the number of solutions to a linear system:

1. There are _infinitely many_ solutions.
2. There is _one_ solution.
3. There are _no_ solutions.

Generally, there are infinitely many solutions if there are more equations than unknowns $i>j$, one solution if the number equations equals the number of unknowns $i=j$ and no solutions if there are more more equations than unknowns $i<j$.

## Properties of linear systems
- **Independence**: a linear system is independent if none of the equations in the system can be represented as a _linear combination_ of the other equations. This way, all equations provide _additional_ information about the relations between variables.
$$\left.\begin{matrix}
~~x+~~y=1 \\
2x + 2y = 2
\end{matrix}\right\} \quad\text{dependent system}$$
- **Consistency**: A system is consistent if there are no _contradictions_ in the equations.
$$\left.\begin{matrix}
x+y=0 \\
x + y = 2
\end{matrix}\right\} \quad\text{inconsistent system}$$