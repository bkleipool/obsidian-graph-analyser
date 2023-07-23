---
tag: mathematics/linear_algebra, topic
---

In [[Linear algebra]], a matrix is a rectangular array of numbers. Visually, a matrix can represent a [[Linear map |linear map]] which maps a [[Vector |vector]] to another vector, where every component of the output vector is a _linear combination_ of the components of the input vector, which makes it a _linear transformation_^[3Blue1Brown, Essence of Linear Algebra Ep3: https://www.youtube.com/watch?v=kYB8IZa5AuE&list=PLZHQObOWTQDPD3MizzM2xVFitgF8hE_ab&index=3].

## Mathematical definition
It turns out that a linear transformation can be fully described by its effect on the [[Coordinate basis |basis vectors]], since every other vector in the space can be described as linear combinations of the basis vectors. For example, a 90 degree clockwise rotation would map the $\mathbf{i}$ basis vector from $\mathbf{i}=[1,0]$ to $\mathbf{i}'=[0,1]$ and the $\mathbf{j}$ basis vector from $\mathbf{j}=[0,1]$ to $\mathbf{j}'=[-1,0]$:

```tikz
\begin{document}

\tikzset{every picture/.style={line width=0.75pt}} %set default line width to 0.75pt        

\begin{tikzpicture}[x=0.75pt,y=0.75pt,yscale=-1,xscale=1]
%uncomment if require: \path (0,179); %set diagram left start at 0, and has height of 179

%Straight Lines [id:da9672736060444569] 
\draw [color={rgb, 255:red, 208; green, 2; blue, 27 }  ,draw opacity=1 ]   (99.97,130.27) -- (99.97,24.4) ;
\draw [shift={(99.97,22.4)}, rotate = 90] [color={rgb, 255:red, 208; green, 2; blue, 27 }  ,draw opacity=1 ][line width=0.75]    (8.74,-2.63) .. controls (5.56,-1.12) and (2.65,-0.24) .. (0,0) .. controls (2.65,0.24) and (5.56,1.12) .. (8.74,2.63)   ;
%Straight Lines [id:da7967445402692727] 
\draw [color={rgb, 255:red, 74; green, 144; blue, 226 }  ,draw opacity=1 ]   (99.97,130.27) -- (200.15,130.27) ;
\draw [shift={(202.15,130.27)}, rotate = 180] [color={rgb, 255:red, 74; green, 144; blue, 226 }  ,draw opacity=1 ][line width=0.75]    (8.74,-2.63) .. controls (5.56,-1.12) and (2.65,-0.24) .. (0,0) .. controls (2.65,0.24) and (5.56,1.12) .. (8.74,2.63)   ;
%Shape: Ellipse [id:dp6176877348547627] 
\draw  [color={rgb, 255:red, 236; green, 233; blue, 233 }  ,draw opacity=1 ][fill={rgb, 255:red, 236; green, 233; blue, 233 }  ,fill opacity=1 ] (97.51,130.27) .. controls (97.51,128.91) and (98.61,127.81) .. (99.97,127.81) .. controls (101.32,127.81) and (102.42,128.91) .. (102.42,130.27) .. controls (102.42,131.63) and (101.32,132.72) .. (99.97,132.72) .. controls (98.61,132.72) and (97.51,131.63) .. (97.51,130.27) -- cycle ;
%Straight Lines [id:da8670182210583322] 
\draw [color={rgb, 255:red, 74; green, 144; blue, 226 }  ,draw opacity=1 ]   (340.97,130.27) -- (340.97,24.4) ;
\draw [shift={(340.97,22.4)}, rotate = 90] [color={rgb, 255:red, 74; green, 144; blue, 226 }  ,draw opacity=1 ][line width=0.75]    (8.74,-2.63) .. controls (5.56,-1.12) and (2.65,-0.24) .. (0,0) .. controls (2.65,0.24) and (5.56,1.12) .. (8.74,2.63)   ;
%Straight Lines [id:da4420113750968131] 
\draw [color={rgb, 255:red, 208; green, 2; blue, 27 }  ,draw opacity=1 ]   (340.97,130.27) -- (255.15,130.27) ;
\draw [shift={(253.15,130.27)}, rotate = 360] [color={rgb, 255:red, 208; green, 2; blue, 27 }  ,draw opacity=1 ][line width=0.75]    (8.74,-2.63) .. controls (5.56,-1.12) and (2.65,-0.24) .. (0,0) .. controls (2.65,0.24) and (5.56,1.12) .. (8.74,2.63)   ;
%Shape: Ellipse [id:dp6751845749184806] 
\draw  [color={rgb, 255:red, 236; green, 233; blue, 233 }  ,draw opacity=1 ][fill={rgb, 255:red, 236; green, 233; blue, 233 }  ,fill opacity=1 ] (338.51,130.27) .. controls (338.51,128.91) and (339.61,127.81) .. (340.97,127.81) .. controls (342.32,127.81) and (343.42,128.91) .. (343.42,130.27) .. controls (343.42,131.63) and (342.32,132.72) .. (340.97,132.72) .. controls (339.61,132.72) and (338.51,131.63) .. (338.51,130.27) -- cycle ;
%Curve Lines [id:da5449877426540505] 
\draw [color={rgb, 255:red, 221; green, 218; blue, 218 }  ,draw opacity=1 ]   (228.83,70.96) .. controls (240.75,65) and (235.54,47.87) .. (222.87,47.87) .. controls (210.71,47.87) and (205.42,62.28) .. (216.22,71.35) ;
\draw [shift={(217.66,72.45)}, rotate = 215.22] [color={rgb, 255:red, 221; green, 218; blue, 218 }  ,draw opacity=1 ][line width=0.75]    (6.56,-1.97) .. controls (4.17,-0.84) and (1.99,-0.18) .. (0,0) .. controls (1.99,0.18) and (4.17,0.84) .. (6.56,1.97)   ;
%Straight Lines [id:da40171427397900283] 
\draw [color={rgb, 255:red, 221; green, 218; blue, 218 }  ,draw opacity=1 ]   (200,82.5) -- (247.15,82.5)(200,85.5) -- (247.15,85.5) ;
\draw [shift={(255.15,84)}, rotate = 180] [color={rgb, 255:red, 221; green, 218; blue, 218 }  ,draw opacity=1 ][line width=0.75]    (10.93,-3.29) .. controls (6.95,-1.4) and (3.31,-0.3) .. (0,0) .. controls (3.31,0.3) and (6.95,1.4) .. (10.93,3.29)   ;

% Text Node
\draw (79.39,25.7) node [anchor=north west][inner sep=0.75pt]  [font=\small,color={rgb, 255:red, 208; green, 2; blue, 27 }  ,opacity=1 ]  {$\mathbf{j}$};
% Text Node
\draw (196.23,135.72) node [anchor=north west][inner sep=0.75pt]  [font=\small,color={rgb, 255:red, 74; green, 144; blue, 226 }  ,opacity=1 ]  {$\mathbf{i}$};
% Text Node
\draw (262.39,138.7) node [anchor=north west][inner sep=0.75pt]  [font=\small,color={rgb, 255:red, 208; green, 2; blue, 27 }  ,opacity=1 ]  {$\mathbf{j} '$};
% Text Node
\draw (351.15,25.67) node [anchor=north west][inner sep=0.75pt]  [font=\small,color={rgb, 255:red, 74; green, 144; blue, 226 }  ,opacity=1 ]  {$\mathbf{i} '$};

\end{tikzpicture}
\end{document}
```

A vector $\mathbf{v}$ subject to this transformation can be described by its (untransformed) components of the transformed basis vectors:
$$\begin{bmatrix} v'_{1} \\ v'_{2} \end{bmatrix} = 
v_{1}\begin{bmatrix} i'_{1} \\ i'_{2} \end{bmatrix} + 
v_{2}\begin{bmatrix} j'_{1} \\ j'_{2} \end{bmatrix}$$
An alternative representation for this [[System of linear equations |system of linear equations]] is the _matrix-vector product_, where the transformed basis vectors are placed in a matrix $A=[~\mathbf{i}~ ~\mathbf{j}~]$ and multiplied by the vector $\mathbf{v}$ to get the transformed vector $\mathbf{v}'$:
$$
\begin{bmatrix}
v'_{1} \\
v'_{2}
\end{bmatrix}
=
\begin{bmatrix}
i'_{1} & j'_{1} \\
i'_{2} & j'_{2}
\end{bmatrix}
\begin{bmatrix}
v_{1} \\
v_{2}
\end{bmatrix}$$
The product is written in this order because the matrix acts as a "function" on $\mathbf{v}$, so a kind of function notation $A(\mathbf{v})$ is used. To summarise, a matrix describes a linear transformation of a vector by describing the effect that the transformation has on the basis vectors.