---
tags: mathematics/linear_algebra, concept
---
In mathematics and physics, a vector is a quantity that has a magnitude and a direction^[Wikipedia, Euclidean_vector: https://en.wikipedia.org/wiki/Euclidean_vector]. For example in physics, velocity, force, magnetic field strength, etc. are _vector quantities_ while mass, temperature, etc. are [[Scalar |scalar]] quantities (since they don't have a directional component). In a more abstract setting, a vector may be defined as any element of a [[Vector space |vector space]], in which it basically just represents a group of numbers you can add and subtract from each other, not necessarily representing a magnitude or direction of something.

## Mathematical definition
A vector is an object, sometimes represented as an arrow, originating in the center of a coordinate frame and pointing to a certain point in space. Two vectors may be added by sliding one of the two vectors from their shared origin to the endpoint of the other.
```tikz
\begin{document}


\tikzset{every picture/.style={line width=0.75pt}} %set default line width to 0.75pt        

\begin{tikzpicture}[x=0.75pt,y=0.75pt,yscale=-1,xscale=1]
%uncomment if require: \path (0,190); %set diagram left start at 0, and has height of 190

%Straight Lines [id:da8746953719203728] 
\draw [color={rgb, 255:red, 74; green, 144; blue, 226 }  ,draw opacity=1 ]   (545.83,129) -- (600.93,111.6) ;
\draw [shift={(602.83,111)}, rotate = 162.47] [color={rgb, 255:red, 74; green, 144; blue, 226 }  ,draw opacity=1 ][line width=0.75]    (8.74,-2.63) .. controls (5.56,-1.12) and (2.65,-0.24) .. (0,0) .. controls (2.65,0.24) and (5.56,1.12) .. (8.74,2.63)   ;
%Straight Lines [id:da9607733378561246] 
\draw [color={rgb, 255:red, 208; green, 2; blue, 27 }  ,draw opacity=1 ]   (53.83,126.35) -- (70.44,43.31) ;
\draw [shift={(70.83,41.35)}, rotate = 101.31] [color={rgb, 255:red, 208; green, 2; blue, 27 }  ,draw opacity=1 ][line width=0.75]    (8.74,-2.63) .. controls (5.56,-1.12) and (2.65,-0.24) .. (0,0) .. controls (2.65,0.24) and (5.56,1.12) .. (8.74,2.63)   ;
%Straight Lines [id:da16152714493375575] 
\draw [color={rgb, 255:red, 74; green, 144; blue, 226 }  ,draw opacity=1 ]   (53.83,126.35) -- (108.93,108.95) ;
\draw [shift={(110.83,108.35)}, rotate = 162.47] [color={rgb, 255:red, 74; green, 144; blue, 226 }  ,draw opacity=1 ][line width=0.75]    (8.74,-2.63) .. controls (5.56,-1.12) and (2.65,-0.24) .. (0,0) .. controls (2.65,0.24) and (5.56,1.12) .. (8.74,2.63)   ;
%Shape: Circle [id:dp428410217797641] 
\draw  [color={rgb, 255:red, 236; green, 233; blue, 233 }  ,draw opacity=1 ][fill={rgb, 255:red, 236; green, 233; blue, 233 }  ,fill opacity=1 ] (51.92,126.35) .. controls (51.92,125.29) and (52.77,124.43) .. (53.83,124.43) .. controls (54.89,124.43) and (55.75,125.29) .. (55.75,126.35) .. controls (55.75,127.41) and (54.89,128.27) .. (53.83,128.27) .. controls (52.77,128.27) and (51.92,127.41) .. (51.92,126.35) -- cycle ;
%Straight Lines [id:da4077857118411322] 
\draw [color={rgb, 255:red, 208; green, 2; blue, 27 }  ,draw opacity=1 ] [dash pattern={on 0.84pt off 2.51pt}]  (168.83,130.35) -- (185.44,47.31) ;
\draw [shift={(185.83,45.35)}, rotate = 101.31] [color={rgb, 255:red, 208; green, 2; blue, 27 }  ,draw opacity=1 ][line width=0.75]    (8.74,-2.63) .. controls (5.56,-1.12) and (2.65,-0.24) .. (0,0) .. controls (2.65,0.24) and (5.56,1.12) .. (8.74,2.63)   ;
%Straight Lines [id:da3914168289431167] 
\draw [color={rgb, 255:red, 74; green, 144; blue, 226 }  ,draw opacity=1 ]   (168.83,130.35) -- (223.93,112.95) ;
\draw [shift={(225.83,112.35)}, rotate = 162.47] [color={rgb, 255:red, 74; green, 144; blue, 226 }  ,draw opacity=1 ][line width=0.75]    (8.74,-2.63) .. controls (5.56,-1.12) and (2.65,-0.24) .. (0,0) .. controls (2.65,0.24) and (5.56,1.12) .. (8.74,2.63)   ;
%Straight Lines [id:da4119960767024481] 
\draw [color={rgb, 255:red, 208; green, 2; blue, 27 }  ,draw opacity=1 ]   (225.83,112.35) -- (242.44,29.31) ;
\draw [shift={(242.83,27.35)}, rotate = 101.31] [color={rgb, 255:red, 208; green, 2; blue, 27 }  ,draw opacity=1 ][line width=0.75]    (8.74,-2.63) .. controls (5.56,-1.12) and (2.65,-0.24) .. (0,0) .. controls (2.65,0.24) and (5.56,1.12) .. (8.74,2.63)   ;
%Straight Lines [id:da9304696333039663] 
\draw [color={rgb, 255:red, 126; green, 211; blue, 33 }  ,draw opacity=1 ]   (168.83,130.35) -- (241.67,28.97) ;
\draw [shift={(242.83,27.35)}, rotate = 125.7] [color={rgb, 255:red, 126; green, 211; blue, 33 }  ,draw opacity=1 ][line width=0.75]    (8.74,-2.63) .. controls (5.56,-1.12) and (2.65,-0.24) .. (0,0) .. controls (2.65,0.24) and (5.56,1.12) .. (8.74,2.63)   ;
%Shape: Circle [id:dp4817006007243898] 
\draw  [color={rgb, 255:red, 236; green, 233; blue, 233 }  ,draw opacity=1 ][fill={rgb, 255:red, 236; green, 233; blue, 233 }  ,fill opacity=1 ] (166.92,130.35) .. controls (166.92,129.29) and (167.77,128.43) .. (168.83,128.43) .. controls (169.89,128.43) and (170.75,129.29) .. (170.75,130.35) .. controls (170.75,131.41) and (169.89,132.27) .. (168.83,132.27) .. controls (167.77,132.27) and (166.92,131.41) .. (166.92,130.35) -- cycle ;
%Straight Lines [id:da42384130024712285] 
\draw [color={rgb, 255:red, 208; green, 2; blue, 27 }  ,draw opacity=1 ]   (286.83,130) -- (303.44,46.96) ;
\draw [shift={(303.83,45)}, rotate = 101.31] [color={rgb, 255:red, 208; green, 2; blue, 27 }  ,draw opacity=1 ][line width=0.75]    (8.74,-2.63) .. controls (5.56,-1.12) and (2.65,-0.24) .. (0,0) .. controls (2.65,0.24) and (5.56,1.12) .. (8.74,2.63)   ;
%Straight Lines [id:da9213001444250153] 
\draw [color={rgb, 255:red, 74; green, 144; blue, 226 }  ,draw opacity=1 ] [dash pattern={on 0.84pt off 2.51pt}]  (286.83,130) -- (341.93,112.6) ;
\draw [shift={(343.83,112)}, rotate = 162.47] [color={rgb, 255:red, 74; green, 144; blue, 226 }  ,draw opacity=1 ][line width=0.75]    (8.74,-2.63) .. controls (5.56,-1.12) and (2.65,-0.24) .. (0,0) .. controls (2.65,0.24) and (5.56,1.12) .. (8.74,2.63)   ;
%Straight Lines [id:da35777512421039603] 
\draw [color={rgb, 255:red, 126; green, 211; blue, 33 }  ,draw opacity=1 ]   (286.83,130) -- (359.67,28.62) ;
\draw [shift={(360.83,27)}, rotate = 125.7] [color={rgb, 255:red, 126; green, 211; blue, 33 }  ,draw opacity=1 ][line width=0.75]    (8.74,-2.63) .. controls (5.56,-1.12) and (2.65,-0.24) .. (0,0) .. controls (2.65,0.24) and (5.56,1.12) .. (8.74,2.63)   ;
%Shape: Circle [id:dp41085506623371293] 
\draw  [color={rgb, 255:red, 236; green, 233; blue, 233 }  ,draw opacity=1 ][fill={rgb, 255:red, 236; green, 233; blue, 233 }  ,fill opacity=1 ] (284.92,130) .. controls (284.92,128.94) and (285.77,128.08) .. (286.83,128.08) .. controls (287.89,128.08) and (288.75,128.94) .. (288.75,130) .. controls (288.75,131.06) and (287.89,131.92) .. (286.83,131.92) .. controls (285.77,131.92) and (284.92,131.06) .. (284.92,130) -- cycle ;
%Straight Lines [id:da21918275391174025] 
\draw [color={rgb, 255:red, 74; green, 144; blue, 226 }  ,draw opacity=1 ]   (303.83,45) -- (358.93,27.6) ;
\draw [shift={(360.83,27)}, rotate = 162.47] [color={rgb, 255:red, 74; green, 144; blue, 226 }  ,draw opacity=1 ][line width=0.75]    (8.74,-2.63) .. controls (5.56,-1.12) and (2.65,-0.24) .. (0,0) .. controls (2.65,0.24) and (5.56,1.12) .. (8.74,2.63)   ;
%Straight Lines [id:da9137614444181498] 
\draw [color={rgb, 255:red, 74; green, 144; blue, 226 }  ,draw opacity=1 ]   (427.83,129.35) -- (482.93,111.95) ;
\draw [shift={(484.83,111.35)}, rotate = 162.47] [color={rgb, 255:red, 74; green, 144; blue, 226 }  ,draw opacity=1 ][line width=0.75]    (8.74,-2.63) .. controls (5.56,-1.12) and (2.65,-0.24) .. (0,0) .. controls (2.65,0.24) and (5.56,1.12) .. (8.74,2.63)   ;
%Straight Lines [id:da2746374995885623] 
\draw [color={rgb, 255:red, 208; green, 2; blue, 27 }  ,draw opacity=1 ]   (427.83,131.27) -- (444.44,48.23) ;
\draw [shift={(444.83,46.27)}, rotate = 101.31] [color={rgb, 255:red, 208; green, 2; blue, 27 }  ,draw opacity=1 ][line width=0.75]    (8.74,-2.63) .. controls (5.56,-1.12) and (2.65,-0.24) .. (0,0) .. controls (2.65,0.24) and (5.56,1.12) .. (8.74,2.63)   ;
%Straight Lines [id:da8031453491620284] 
\draw [color={rgb, 255:red, 126; green, 211; blue, 33 }  ,draw opacity=1 ]   (484.83,111.35) -- (445.88,47.97) ;
\draw [shift={(444.83,46.27)}, rotate = 58.43] [color={rgb, 255:red, 126; green, 211; blue, 33 }  ,draw opacity=1 ][line width=0.75]    (8.74,-2.63) .. controls (5.56,-1.12) and (2.65,-0.24) .. (0,0) .. controls (2.65,0.24) and (5.56,1.12) .. (8.74,2.63)   ;
%Shape: Circle [id:dp35442961506748194] 
\draw  [color={rgb, 255:red, 236; green, 233; blue, 233 }  ,draw opacity=1 ][fill={rgb, 255:red, 236; green, 233; blue, 233 }  ,fill opacity=1 ] (425.92,129.35) .. controls (425.92,128.29) and (426.77,127.43) .. (427.83,127.43) .. controls (428.89,127.43) and (429.75,128.29) .. (429.75,129.35) .. controls (429.75,130.41) and (428.89,131.27) .. (427.83,131.27) .. controls (426.77,131.27) and (425.92,130.41) .. (425.92,129.35) -- cycle ;
%Straight Lines [id:da06026679949975711] 
\draw [color={rgb, 255:red, 208; green, 2; blue, 27 }  ,draw opacity=1 ]   (545.83,129) -- (562.44,45.96) ;
\draw [shift={(562.83,44)}, rotate = 101.31] [color={rgb, 255:red, 208; green, 2; blue, 27 }  ,draw opacity=1 ][line width=0.75]    (8.74,-2.63) .. controls (5.56,-1.12) and (2.65,-0.24) .. (0,0) .. controls (2.65,0.24) and (5.56,1.12) .. (8.74,2.63)   ;
%Straight Lines [id:da09184536045820935] 
\draw [color={rgb, 255:red, 126; green, 211; blue, 33 }  ,draw opacity=1 ]   (562.83,44) -- (601.81,109.28) ;
\draw [shift={(602.83,111)}, rotate = 239.16] [color={rgb, 255:red, 126; green, 211; blue, 33 }  ,draw opacity=1 ][line width=0.75]    (8.74,-2.63) .. controls (5.56,-1.12) and (2.65,-0.24) .. (0,0) .. controls (2.65,0.24) and (5.56,1.12) .. (8.74,2.63)   ;
%Shape: Circle [id:dp6848267063405857] 
\draw  [color={rgb, 255:red, 236; green, 233; blue, 233 }  ,draw opacity=1 ][fill={rgb, 255:red, 236; green, 233; blue, 233 }  ,fill opacity=1 ] (543.92,129) .. controls (543.92,127.94) and (544.77,127.08) .. (545.83,127.08) .. controls (546.89,127.08) and (547.75,127.94) .. (547.75,129) .. controls (547.75,130.06) and (546.89,130.92) .. (545.83,130.92) .. controls (544.77,130.92) and (543.92,130.06) .. (543.92,129) -- cycle ;

% Text Node
\draw (112.83,111.75) node [anchor=north west][inner sep=0.75pt]  [color={rgb, 255:red, 74; green, 144; blue, 226 }  ,opacity=1 ]  {$\mathbf{u}$};
% Text Node
\draw (48,31.4) node [anchor=north west][inner sep=0.75pt]  [color={rgb, 255:red, 208; green, 2; blue, 27 }  ,opacity=1 ]  {$\mathbf{v}$};
% Text Node
\draw (14,116) node [anchor=north west][inner sep=0.75pt]  [font=\scriptsize,color={rgb, 255:red, 236; green, 233; blue, 233 }  ,opacity=1 ] [align=left] {Shared\\origin};
% Text Node
\draw (215,143) node [anchor=north west][inner sep=0.75pt]  [font=\small,color={rgb, 255:red, 236; green, 233; blue, 233 }  ,opacity=1 ] [align=left] {Vector addition};
% Text Node
\draw (227.83,115.75) node [anchor=north west][inner sep=0.75pt]  [color={rgb, 255:red, 74; green, 144; blue, 226 }  ,opacity=1 ]  {$\mathbf{u}$};
% Text Node
\draw (165.83,46.75) node [anchor=north west][inner sep=0.75pt]  [color={rgb, 255:red, 208; green, 2; blue, 27 }  ,opacity=1 ]  {$\mathbf{v}$};
% Text Node
\draw (194.83,25.75) node [anchor=north west][inner sep=0.75pt]  [font=\footnotesize,color={rgb, 255:red, 126; green, 211; blue, 33 }  ,opacity=1 ]  {$\mathbf{u} +\mathbf{v}$};
% Text Node
\draw (317.83,15.4) node [anchor=north west][inner sep=0.75pt]  [color={rgb, 255:red, 74; green, 144; blue, 226 }  ,opacity=1 ]  {$\mathbf{u}$};
% Text Node
\draw (283.83,46.4) node [anchor=north west][inner sep=0.75pt]  [color={rgb, 255:red, 208; green, 2; blue, 27 }  ,opacity=1 ]  {$\mathbf{v}$};
% Text Node
\draw (350.83,49.4) node [anchor=north west][inner sep=0.75pt]  [font=\footnotesize,color={rgb, 255:red, 126; green, 211; blue, 33 }  ,opacity=1 ]  {$\mathbf{u} +\mathbf{v}$};
% Text Node
\draw (466,143) node [anchor=north west][inner sep=0.75pt]  [font=\small,color={rgb, 255:red, 236; green, 233; blue, 233 }  ,opacity=1 ] [align=left] {Vector subtraction};
% Text Node
\draw (475.83,67.75) node [anchor=north west][inner sep=0.75pt]  [font=\footnotesize,color={rgb, 255:red, 126; green, 211; blue, 33 }  ,opacity=1 ]  {$\mathbf{v-u}$};
% Text Node
\draw (592.83,64.75) node [anchor=north west][inner sep=0.75pt]  [font=\footnotesize,color={rgb, 255:red, 126; green, 211; blue, 33 }  ,opacity=1 ]  {$\mathbf{u-v}$};
% Text Node
\draw (486.83,114.75) node [anchor=north west][inner sep=0.75pt]  [color={rgb, 255:red, 74; green, 144; blue, 226 }  ,opacity=1 ]  {$\mathbf{u}$};
% Text Node
\draw (424.83,45.75) node [anchor=north west][inner sep=0.75pt]  [color={rgb, 255:red, 208; green, 2; blue, 27 }  ,opacity=1 ]  {$\mathbf{v}$};
% Text Node
\draw (604.83,114.4) node [anchor=north west][inner sep=0.75pt]  [color={rgb, 255:red, 74; green, 144; blue, 226 }  ,opacity=1 ]  {$\mathbf{u}$};
% Text Node
\draw (542.83,45.4) node [anchor=north west][inner sep=0.75pt]  [color={rgb, 255:red, 208; green, 2; blue, 27 }  ,opacity=1 ]  {$\mathbf{v}$};

\end{tikzpicture}

\end{document}
```

A vector may also be scaled in size by a number (positive or negative). From this, we can represent any vector as a sum of scaled of [[Coordinate basis |basis vectors]], which are vectors of magnitude 1 which point in the directions of the coordinate system:

```tikz
\begin{document}


\tikzset{every picture/.style={line width=0.75pt}} %set default line width to 0.75pt        

\begin{tikzpicture}[x=0.75pt,y=0.75pt,yscale=-1,xscale=1]
%uncomment if require: \path (0,202); %set diagram left start at 0, and has height of 202

%Straight Lines [id:da9330643378190223] 
\draw [color={rgb, 255:red, 74; green, 144; blue, 226 }  ,draw opacity=1 ]   (108.81,130.43) -- (85,154.24) -- (75.93,163.31) ;
\draw [shift={(74.51,164.73)}, rotate = 315] [color={rgb, 255:red, 74; green, 144; blue, 226 }  ,draw opacity=1 ][line width=0.75]    (8.74,-2.63) .. controls (5.56,-1.12) and (2.65,-0.24) .. (0,0) .. controls (2.65,0.24) and (5.56,1.12) .. (8.74,2.63)   ;
%Straight Lines [id:da8542801034226278] 
\draw [color={rgb, 255:red, 200; green, 198; blue, 198 }  ,draw opacity=1 ]   (108.81,130.43) -- (108.81,88.07) ;
\draw [shift={(108.81,86.07)}, rotate = 90] [color={rgb, 255:red, 200; green, 198; blue, 198 }  ,draw opacity=1 ][line width=0.75]    (8.74,-2.63) .. controls (5.56,-1.12) and (2.65,-0.24) .. (0,0) .. controls (2.65,0.24) and (5.56,1.12) .. (8.74,2.63)   ;
%Straight Lines [id:da8492175840307353] 
\draw [color={rgb, 255:red, 200; green, 198; blue, 198 }  ,draw opacity=1 ]   (108.81,130.43) -- (86.42,152.82) ;
\draw [shift={(85,154.24)}, rotate = 315] [color={rgb, 255:red, 200; green, 198; blue, 198 }  ,draw opacity=1 ][line width=0.75]    (8.74,-2.63) .. controls (5.56,-1.12) and (2.65,-0.24) .. (0,0) .. controls (2.65,0.24) and (5.56,1.12) .. (8.74,2.63)   ;
%Straight Lines [id:da9791734717246472] 
\draw [color={rgb, 255:red, 0; green, 0; blue, 0 }  ,draw opacity=1 ]   (108.81,130.43) -- (158.63,44.14) ;
\draw [shift={(159.63,42.41)}, rotate = 120] [color={rgb, 255:red, 0; green, 0; blue, 0 }  ,draw opacity=1 ][line width=0.75]    (8.74,-2.63) .. controls (5.56,-1.12) and (2.65,-0.24) .. (0,0) .. controls (2.65,0.24) and (5.56,1.12) .. (8.74,2.63)   ;
%Straight Lines [id:da18206094159961161] 
\draw [color={rgb, 255:red, 200; green, 198; blue, 198 }  ,draw opacity=1 ]   (108.81,130.43) -- (148.83,130.43) ;
\draw [shift={(150.83,130.43)}, rotate = 180] [color={rgb, 255:red, 200; green, 198; blue, 198 }  ,draw opacity=1 ][line width=0.75]    (8.74,-2.63) .. controls (5.56,-1.12) and (2.65,-0.24) .. (0,0) .. controls (2.65,0.24) and (5.56,1.12) .. (8.74,2.63)   ;
%Straight Lines [id:da9358851144599255] 
\draw [color={rgb, 255:red, 126; green, 211; blue, 33 }  ,draw opacity=1 ]   (159.63,44.41) -- (159.63,164.37) ;
\draw [shift={(159.63,42.41)}, rotate = 90] [color={rgb, 255:red, 126; green, 211; blue, 33 }  ,draw opacity=1 ][line width=0.75]    (8.74,-2.63) .. controls (5.56,-1.12) and (2.65,-0.24) .. (0,0) .. controls (2.65,0.24) and (5.56,1.12) .. (8.74,2.63)   ;
%Straight Lines [id:da05386623138815538] 
\draw [color={rgb, 255:red, 208; green, 2; blue, 27 }  ,draw opacity=1 ]   (74.51,164.37) -- (157.63,164.37) ;
\draw [shift={(159.63,164.37)}, rotate = 180] [color={rgb, 255:red, 208; green, 2; blue, 27 }  ,draw opacity=1 ][line width=0.75]    (8.74,-2.63) .. controls (5.56,-1.12) and (2.65,-0.24) .. (0,0) .. controls (2.65,0.24) and (5.56,1.12) .. (8.74,2.63)   ;
%Shape: Ellipse [id:dp17230215473170396] 
\draw  [color={rgb, 255:red, 236; green, 233; blue, 233 }  ,draw opacity=1 ][fill={rgb, 255:red, 236; green, 233; blue, 233 }  ,fill opacity=1 ] (106.38,130.43) .. controls (106.38,129.08) and (107.47,127.99) .. (108.81,127.99) .. controls (110.16,127.99) and (111.25,129.08) .. (111.25,130.43) .. controls (111.25,131.77) and (110.16,132.86) .. (108.81,132.86) .. controls (107.47,132.86) and (106.38,131.77) .. (106.38,130.43) -- cycle ;

% Text Node
\draw (139.33,136.13) node [anchor=north west][inner sep=0.75pt]  [font=\scriptsize,color={rgb, 255:red, 200; green, 198; blue, 198 }  ,opacity=1 ]  {$\mathbf{j}$};
% Text Node
\draw (138.59,27.1) node [anchor=north west][inner sep=0.75pt]  [color={rgb, 255:red, 0; green, 0; blue, 0 }  ,opacity=1 ]  {$\mathbf{v}$};
% Text Node
\draw (86.76,130.77) node [anchor=north west][inner sep=0.75pt]  [font=\scriptsize,color={rgb, 255:red, 200; green, 198; blue, 198 }  ,opacity=1 ]  {$\mathbf{i}$};
% Text Node
\draw (92.65,75.96) node [anchor=north west][inner sep=0.75pt]  [font=\scriptsize,color={rgb, 255:red, 200; green, 198; blue, 198 }  ,opacity=1 ]  {$\mathbf{k}$};
% Text Node
\draw (56.27,144.88) node [anchor=north west][inner sep=0.75pt]  [font=\scriptsize,color={rgb, 255:red, 74; green, 144; blue, 226 }  ,opacity=1 ]  {$v_{1}\mathbf{i}$};
% Text Node
\draw (166.86,44.3) node [anchor=north west][inner sep=0.75pt]  [font=\scriptsize,color={rgb, 255:red, 126; green, 211; blue, 33 }  ,opacity=1 ]  {$v_{3}\mathbf{k}$};
% Text Node
\draw (139.27,171.88) node [anchor=north west][inner sep=0.75pt]  [font=\scriptsize,color={rgb, 255:red, 208; green, 2; blue, 27 }  ,opacity=1 ]  {$v_{2}\mathbf{j}$};


\end{tikzpicture}

\end{document}
```

Hence, we can use the basis vectors to algebraically represent the vector as the scalar multiples $v_{1}, v_{2}, v_{3}$ of the basis vectors^[3Blue1Brown, Essence of Linear Algebra Ep1: https://www.youtube.com/watch?v=fNk_zzaMoSs&list=PLZHQObOWTQDPD3MizzM2xVFitgF8hE_ab&index=2]:
$$\mathbf{v} =
\begin{bmatrix} v_{1} \\ v_{2} \\ v_{3} \end{bmatrix} = 
v_{1}\begin{bmatrix} 1 \\ 0 \\ 0 \end{bmatrix} + 
v_{2}\begin{bmatrix} 0 \\ 1 \\ 0 \end{bmatrix} + 
v_{3}\begin{bmatrix} 0 \\ 0 \\ 1 \end{bmatrix}
$$
## Vector operations
From the representation of a vector as a _linear combination_ of basis vectors, we can see that _scaling_ a vector by a number and adding two vectors together can be done in terms of the vector components as follows:
$$a\mathbf{u}=\begin{bmatrix}
au_{1} \\ au_{2} \\ \vdots \\ au_{n}
\end{bmatrix}
\qquad\qquad
\mathbf{u}+\mathbf{v}=\mathbf{v}+\mathbf{u}=
\begin{bmatrix} u_{1} + v_{1} \\ u_{2} + v_{2} \\ \vdots \\ u_{n} + v_{n} \end{bmatrix}$$
We can also see that when we transform the basis vectors to some other points, let's say $\mathbf{i}=[i_{1},i_{2}, i_{3}]$, $\mathbf{j}=[j_{1},j_{2}, j_{3}]$ and $\mathbf{k}=[k_{1}, k_{2}, k_{3}]$, then the transformed vector components become a [[System of linear equations |system of linear equations]]:
$$\begin{align}
\mathbf{v}' &= v_{1}\mathbf{i} + v_{2}\mathbf{j} + v_{3}\mathbf{k} \\
\begin{bmatrix} v'_{1} \\ v'_{2} \\ v'_{3} \end{bmatrix} &= 
v_{1}\begin{bmatrix} i_{1} \\ i_{2} \\ i_{3} \end{bmatrix} + 
v_{2}\begin{bmatrix} j_{1} \\ j_{2} \\ j_{3} \end{bmatrix} + 
v_{3}\begin{bmatrix} k_{1} \\ k_{2} \\ k_{3} \end{bmatrix} \\
&\left\{\begin{matrix}
i_{1}v_{1}+j_{1}v_{2}+k_{1}v_{3}=v'_{1} \\
i_{2}v_{1}+j_{2}v_{2}+k_{2}v_{3}=v'_{2} \\
i_{3}v_{1}+j_{3}v_{2}+k_{3}v_{3}=v'_{3}
\end{matrix}\right.
\end{align}
$$
This system has exactly _one solution_ if the basis vectors are _linearly independent_,  _no solutions_ if there are not enough basis vectors for the conditionality of the space (e.g. 2 basis vectors for a 3D space) and _infinitely many_ solutions if there are more basis vectors than the number of dimensions in the space.

```tikz
\begin{document}


\tikzset{every picture/.style={line width=0.75pt}} %set default line width to 0.75pt        

\begin{tikzpicture}[x=0.75pt,y=0.75pt,yscale=-1,xscale=1]
%uncomment if require: \path (0,209); %set diagram left start at 0, and has height of 209

%Straight Lines [id:da8217905868129469] 
\draw [color={rgb, 255:red, 185; green, 183; blue, 183 }  ,draw opacity=1 ][fill={rgb, 255:red, 183; green, 180; blue, 180 }  ,fill opacity=1 ] [dash pattern={on 4.5pt off 4.5pt}]  (208.15,20.35) -- (240.76,166.15) ;
%Straight Lines [id:da18085046430268947] 
\draw [color={rgb, 255:red, 200; green, 198; blue, 198 }  ,draw opacity=1 ]   (89.01,127.74) -- (78.8,84.36) ;
\draw [shift={(78.35,82.42)}, rotate = 76.76] [color={rgb, 255:red, 200; green, 198; blue, 198 }  ,draw opacity=1 ][line width=0.75]    (8.74,-2.63) .. controls (5.56,-1.12) and (2.65,-0.24) .. (0,0) .. controls (2.65,0.24) and (5.56,1.12) .. (8.74,2.63)   ;
%Straight Lines [id:da7328267496732497] 
\draw [color={rgb, 255:red, 0; green, 0; blue, 0 }  ,draw opacity=1 ]   (89.01,127.74) -- (122.63,66.51) ;
\draw [shift={(123.59,64.76)}, rotate = 118.77] [color={rgb, 255:red, 0; green, 0; blue, 0 }  ,draw opacity=1 ][line width=0.75]    (8.74,-2.63) .. controls (5.56,-1.12) and (2.65,-0.24) .. (0,0) .. controls (2.65,0.24) and (5.56,1.12) .. (8.74,2.63)   ;
%Straight Lines [id:da2604717444938892] 
\draw [color={rgb, 255:red, 200; green, 198; blue, 198 }  ,draw opacity=1 ]   (89.01,127.74) -- (117.32,139.06) ;
\draw [shift={(119.18,139.8)}, rotate = 201.78] [color={rgb, 255:red, 200; green, 198; blue, 198 }  ,draw opacity=1 ][line width=0.75]    (8.74,-2.63) .. controls (5.56,-1.12) and (2.65,-0.24) .. (0,0) .. controls (2.65,0.24) and (5.56,1.12) .. (8.74,2.63)   ;
%Straight Lines [id:da5860208494301118] 
\draw [color={rgb, 255:red, 208; green, 2; blue, 27 }  ,draw opacity=1 ]   (69.52,43.79) -- (121.72,64.04) ;
\draw [shift={(123.59,64.76)}, rotate = 201.19] [color={rgb, 255:red, 208; green, 2; blue, 27 }  ,draw opacity=1 ][line width=0.75]    (8.74,-2.63) .. controls (5.56,-1.12) and (2.65,-0.24) .. (0,0) .. controls (2.65,0.24) and (5.56,1.12) .. (8.74,2.63)   ;
%Shape: Ellipse [id:dp07188797671273894] 
\draw  [color={rgb, 255:red, 236; green, 233; blue, 233 }  ,draw opacity=1 ][fill={rgb, 255:red, 236; green, 233; blue, 233 }  ,fill opacity=1 ] (86.32,127.74) .. controls (86.32,126.26) and (87.52,125.06) .. (89.01,125.06) .. controls (90.49,125.06) and (91.7,126.26) .. (91.7,127.74) .. controls (91.7,129.23) and (90.49,130.43) .. (89.01,130.43) .. controls (87.52,130.43) and (86.32,129.23) .. (86.32,127.74) -- cycle ;
%Straight Lines [id:da31273477877400413] 
\draw [color={rgb, 255:red, 74; green, 144; blue, 226 }  ,draw opacity=1 ]   (89.01,127.74) -- (69.97,45.74) ;
\draw [shift={(69.52,43.79)}, rotate = 76.93] [color={rgb, 255:red, 74; green, 144; blue, 226 }  ,draw opacity=1 ][line width=0.75]    (8.74,-2.63) .. controls (5.56,-1.12) and (2.65,-0.24) .. (0,0) .. controls (2.65,0.24) and (5.56,1.12) .. (8.74,2.63)   ;
%Straight Lines [id:da30373259393863383] 
\draw [color={rgb, 255:red, 200; green, 198; blue, 198 }  ,draw opacity=1 ]   (235.37,140.83) -- (223.84,91.8) ;
\draw [shift={(223.39,89.85)}, rotate = 76.76] [color={rgb, 255:red, 200; green, 198; blue, 198 }  ,draw opacity=1 ][line width=0.75]    (8.74,-2.63) .. controls (5.56,-1.12) and (2.65,-0.24) .. (0,0) .. controls (2.65,0.24) and (5.56,1.12) .. (8.74,2.63)   ;
%Straight Lines [id:da7704623103041889] 
\draw [color={rgb, 255:red, 0; green, 0; blue, 0 }  ,draw opacity=1 ]   (235.37,140.83) -- (268.53,61.28) ;
\draw [shift={(269.3,59.43)}, rotate = 112.63] [color={rgb, 255:red, 0; green, 0; blue, 0 }  ,draw opacity=1 ][line width=0.75]    (8.74,-2.63) .. controls (5.56,-1.12) and (2.65,-0.24) .. (0,0) .. controls (2.65,0.24) and (5.56,1.12) .. (8.74,2.63)   ;
%Shape: Ellipse [id:dp46303860586379053] 
\draw  [color={rgb, 255:red, 236; green, 233; blue, 233 }  ,draw opacity=1 ][fill={rgb, 255:red, 236; green, 233; blue, 233 }  ,fill opacity=1 ] (232.35,140.83) .. controls (232.35,139.16) and (233.71,137.81) .. (235.37,137.81) .. controls (237.04,137.81) and (238.4,139.16) .. (238.4,140.83) .. controls (238.4,142.5) and (237.04,143.85) .. (235.37,143.85) .. controls (233.71,143.85) and (232.35,142.5) .. (232.35,140.83) -- cycle ;
%Straight Lines [id:da5813347780016104] 
\draw [color={rgb, 255:red, 74; green, 144; blue, 226 }  ,draw opacity=1 ]   (235.37,140.83) -- (220.11,73.79) ;
\draw [shift={(219.66,71.84)}, rotate = 77.17] [color={rgb, 255:red, 74; green, 144; blue, 226 }  ,draw opacity=1 ][line width=0.75]    (8.74,-2.63) .. controls (5.56,-1.12) and (2.65,-0.24) .. (0,0) .. controls (2.65,0.24) and (5.56,1.12) .. (8.74,2.63)   ;
%Straight Lines [id:da521250638808368] 
\draw [color={rgb, 255:red, 200; green, 198; blue, 198 }  ,draw opacity=1 ]   (409.56,140.08) -- (398.27,92.09) ;
\draw [shift={(397.81,90.14)}, rotate = 76.76] [color={rgb, 255:red, 200; green, 198; blue, 198 }  ,draw opacity=1 ][line width=0.75]    (8.74,-2.63) .. controls (5.56,-1.12) and (2.65,-0.24) .. (0,0) .. controls (2.65,0.24) and (5.56,1.12) .. (8.74,2.63)   ;
%Straight Lines [id:da6439605875616299] 
\draw [color={rgb, 255:red, 0; green, 0; blue, 0 }  ,draw opacity=1 ]   (409.56,140.08) -- (456.49,46.32) ;
\draw [shift={(457.38,44.53)}, rotate = 116.59] [color={rgb, 255:red, 0; green, 0; blue, 0 }  ,draw opacity=1 ][line width=0.75]    (8.74,-2.63) .. controls (5.56,-1.12) and (2.65,-0.24) .. (0,0) .. controls (2.65,0.24) and (5.56,1.12) .. (8.74,2.63)   ;
%Straight Lines [id:da8820792843590397] 
\draw [color={rgb, 255:red, 200; green, 198; blue, 198 }  ,draw opacity=1 ]   (409.56,140.08) -- (440.93,152.61) ;
\draw [shift={(442.79,153.36)}, rotate = 201.78] [color={rgb, 255:red, 200; green, 198; blue, 198 }  ,draw opacity=1 ][line width=0.75]    (8.74,-2.63) .. controls (5.56,-1.12) and (2.65,-0.24) .. (0,0) .. controls (2.65,0.24) and (5.56,1.12) .. (8.74,2.63)   ;
%Straight Lines [id:da30974955520672887] 
\draw [color={rgb, 255:red, 208; green, 2; blue, 27 }  ,draw opacity=1 ]   (388.09,47.59) -- (409.76,57.01) ;
\draw [shift={(411.59,57.81)}, rotate = 203.49] [color={rgb, 255:red, 208; green, 2; blue, 27 }  ,draw opacity=1 ][line width=0.75]    (8.74,-2.63) .. controls (5.56,-1.12) and (2.65,-0.24) .. (0,0) .. controls (2.65,0.24) and (5.56,1.12) .. (8.74,2.63)   ;
%Straight Lines [id:da0994738603527352] 
\draw [color={rgb, 255:red, 74; green, 144; blue, 226 }  ,draw opacity=1 ]   (409.56,140.08) -- (388.54,49.54) ;
\draw [shift={(388.09,47.59)}, rotate = 76.93] [color={rgb, 255:red, 74; green, 144; blue, 226 }  ,draw opacity=1 ][line width=0.75]    (8.74,-2.63) .. controls (5.56,-1.12) and (2.65,-0.24) .. (0,0) .. controls (2.65,0.24) and (5.56,1.12) .. (8.74,2.63)   ;
%Straight Lines [id:da38418764340089073] 
\draw [color={rgb, 255:red, 200; green, 198; blue, 198 }  ,draw opacity=1 ]   (409.56,140.08) -- (462.76,124.13) ;
\draw [shift={(464.67,123.55)}, rotate = 163.31] [color={rgb, 255:red, 200; green, 198; blue, 198 }  ,draw opacity=1 ][line width=0.75]    (8.74,-2.63) .. controls (5.56,-1.12) and (2.65,-0.24) .. (0,0) .. controls (2.65,0.24) and (5.56,1.12) .. (8.74,2.63)   ;
%Straight Lines [id:da30970878460420825] 
\draw [color={rgb, 255:red, 126; green, 211; blue, 33 }  ,draw opacity=1 ]   (455.46,45.09) -- (411.59,57.81) ;
\draw [shift={(457.38,44.53)}, rotate = 163.83] [color={rgb, 255:red, 126; green, 211; blue, 33 }  ,draw opacity=1 ][line width=0.75]    (8.74,-2.63) .. controls (5.56,-1.12) and (2.65,-0.24) .. (0,0) .. controls (2.65,0.24) and (5.56,1.12) .. (8.74,2.63)   ;
%Straight Lines [id:da2270614384633255] 
\draw [color={rgb, 255:red, 74; green, 144; blue, 226 }  ,draw opacity=1 ]   (409.56,140.08) -- (401.88,104.84) ;
\draw [shift={(401.46,102.89)}, rotate = 77.72] [color={rgb, 255:red, 74; green, 144; blue, 226 }  ,draw opacity=1 ][line width=0.75]    (8.74,-2.63) .. controls (5.56,-1.12) and (2.65,-0.24) .. (0,0) .. controls (2.65,0.24) and (5.56,1.12) .. (8.74,2.63)   ;
%Shape: Ellipse [id:dp07455840699811012] 
\draw  [color={rgb, 255:red, 236; green, 233; blue, 233 }  ,draw opacity=1 ][fill={rgb, 255:red, 236; green, 233; blue, 233 }  ,fill opacity=1 ] (406.6,140.08) .. controls (406.6,138.44) and (407.92,137.12) .. (409.56,137.12) .. controls (411.19,137.12) and (412.52,138.44) .. (412.52,140.08) .. controls (412.52,141.71) and (411.19,143.04) .. (409.56,143.04) .. controls (407.92,143.04) and (406.6,141.71) .. (406.6,140.08) -- cycle ;
%Straight Lines [id:da849997438270774] 
\draw [color={rgb, 255:red, 208; green, 2; blue, 27 }  ,draw opacity=1 ]   (493.85,60.34) -- (459.22,45.33) ;
\draw [shift={(457.38,44.53)}, rotate = 23.43] [color={rgb, 255:red, 208; green, 2; blue, 27 }  ,draw opacity=1 ][line width=0.75]    (8.74,-2.63) .. controls (5.56,-1.12) and (2.65,-0.24) .. (0,0) .. controls (2.65,0.24) and (5.56,1.12) .. (8.74,2.63)   ;
%Straight Lines [id:da5304440811929555] 
\draw [color={rgb, 255:red, 126; green, 211; blue, 33 }  ,draw opacity=1 ]   (492.03,61.17) -- (401.46,102.89) ;
\draw [shift={(493.85,60.34)}, rotate = 155.27] [color={rgb, 255:red, 126; green, 211; blue, 33 }  ,draw opacity=1 ][line width=0.75]    (8.74,-2.63) .. controls (5.56,-1.12) and (2.65,-0.24) .. (0,0) .. controls (2.65,0.24) and (5.56,1.12) .. (8.74,2.63)   ;

% Text Node
\draw (122,144.03) node [anchor=north west][inner sep=0.75pt]  [font=\scriptsize,color={rgb, 255:red, 200; green, 198; blue, 198 }  ,opacity=1 ]  {$\mathbf{j}$};
% Text Node
\draw (125.59,68.16) node [anchor=north west][inner sep=0.75pt]  [color={rgb, 255:red, 0; green, 0; blue, 0 }  ,opacity=1 ]  {$\mathbf{v}$};
% Text Node
\draw (63.08,89.98) node [anchor=north west][inner sep=0.75pt]  [font=\scriptsize,color={rgb, 255:red, 200; green, 198; blue, 198 }  ,opacity=1 ]  {$\mathbf{i}$};
% Text Node
\draw (46.25,53.74) node [anchor=north west][inner sep=0.75pt]  [font=\scriptsize,color={rgb, 255:red, 74; green, 144; blue, 226 }  ,opacity=1 ]  {$v_{1}\mathbf{i}$};
% Text Node
\draw (99.17,39.36) node [anchor=north west][inner sep=0.75pt]  [font=\scriptsize,color={rgb, 255:red, 208; green, 2; blue, 27 }  ,opacity=1 ]  {$v_{2}\mathbf{j}$};
% Text Node
\draw (277.46,60.6) node [anchor=north west][inner sep=0.75pt]  [color={rgb, 255:red, 0; green, 0; blue, 0 }  ,opacity=1 ]  {$\mathbf{v}$};
% Text Node
\draw (208.97,98.93) node [anchor=north west][inner sep=0.75pt]  [font=\scriptsize,color={rgb, 255:red, 200; green, 198; blue, 198 }  ,opacity=1 ]  {$\mathbf{i}$};
% Text Node
\draw (194.56,64.44) node [anchor=north west][inner sep=0.75pt]  [font=\scriptsize,color={rgb, 255:red, 74; green, 144; blue, 226 }  ,opacity=1 ]  {$v_{1}\mathbf{i}$};
% Text Node
\draw (56.15,170) node [anchor=north west][inner sep=0.75pt]  [font=\small,color={rgb, 255:red, 185; green, 183; blue, 183 }  ,opacity=1 ] [align=left] {One solution};
% Text Node
\draw (209.15,172) node [anchor=north west][inner sep=0.75pt]  [font=\small,color={rgb, 255:red, 185; green, 183; blue, 183 }  ,opacity=1 ] [align=left] {No solutions};
% Text Node
\draw (372.15,172) node [anchor=north west][inner sep=0.75pt]  [font=\small,color={rgb, 255:red, 185; green, 183; blue, 183 }  ,opacity=1 ] [align=left] {Infinite solutions};
% Text Node
\draw (449.46,151.76) node [anchor=north west][inner sep=0.75pt]  [font=\scriptsize,color={rgb, 255:red, 200; green, 198; blue, 198 }  ,opacity=1 ]  {$\mathbf{j}$};
% Text Node
\draw (458.65,24.19) node [anchor=north west][inner sep=0.75pt]  [color={rgb, 255:red, 0; green, 0; blue, 0 }  ,opacity=1 ]  {$\mathbf{v}$};
% Text Node
\draw (383.61,96.94) node [anchor=north west][inner sep=0.75pt]  [font=\scriptsize,color={rgb, 255:red, 200; green, 198; blue, 198 }  ,opacity=1 ]  {$\mathbf{i}$};
% Text Node
\draw (370.32,53.07) node [anchor=north west][inner sep=0.75pt]  [font=\scriptsize,color={rgb, 255:red, 74; green, 144; blue, 226 }  ,opacity=1 ]  {$v_{1}\mathbf{i}$};
% Text Node
\draw (391.95,34.89) node [anchor=north west][inner sep=0.75pt]  [font=\scriptsize,color={rgb, 255:red, 208; green, 2; blue, 27 }  ,opacity=1 ]  {$v_{2}\mathbf{j}$};
% Text Node
\draw (473.08,123.07) node [anchor=north west][inner sep=0.75pt]  [font=\scriptsize,color={rgb, 255:red, 200; green, 198; blue, 198 }  ,opacity=1 ]  {$\mathbf{k}$};
% Text Node
\draw (425.36,34.77) node [anchor=north west][inner sep=0.75pt]  [font=\scriptsize,color={rgb, 255:red, 126; green, 211; blue, 33 }  ,opacity=1 ]  {$v_{3}\mathbf{k}$};
% Text Node
\draw (382.77,118.61) node [anchor=north west][inner sep=0.75pt]  [font=\scriptsize,color={rgb, 255:red, 74; green, 144; blue, 226 }  ,opacity=1 ]  {$v_{1}^{\prime }\mathbf{i}$};
% Text Node
\draw (481.21,39.51) node [anchor=north west][inner sep=0.75pt]  [font=\scriptsize,color={rgb, 255:red, 208; green, 2; blue, 27 }  ,opacity=1 ]  {$v_{2}^{\prime }\mathbf{j}$};
% Text Node
\draw (465.48,75.15) node [anchor=north west][inner sep=0.75pt]  [font=\scriptsize,color={rgb, 255:red, 126; green, 211; blue, 33 }  ,opacity=1 ]  {$v_{3}^{\prime }\mathbf{k}$};


\end{tikzpicture}

\end{document}
```

The set of all vectors that can be reached by linear combinations of basis vectors is called the [[Vector span |span]] of the basis vectors. If a vector is within the span of the basis vectors, then it can be represented by the basis vectors.

Note that the scalar components $v_{1}, v_{2}, v_{3}$ depend on the measurement coordinate frame, but the vector itself doesn't change when measured from a different coordinate frame.



## Calculation rules
- **Scalar multiplication**:
$$a\mathbf{u}=\begin{bmatrix}
au_{1} \\ au_{2} \\ \vdots \\ au_{n}
\end{bmatrix}$$
- **Vector addition**:
$$\mathbf{u}+\mathbf{v}=\mathbf{v}+\mathbf{u}=
\begin{bmatrix} u_{1} + v_{1} \\ u_{2} + v_{2} \\ \vdots \\ u_{n} + v_{n} \end{bmatrix}$$


%%OLD VERSION OF TEXT
In mathematics and physics, a vector is a quantity that has a magnitude and a direction. For example, velocity, force, magnetic field strength, etc. are _vector quantities_ while mass, temperature, etc. are [[Scalar |scalar]] quantities (since they don't have a directional component). Vectors are represented by its _scalar components_ in a certain coordinate frame:
$$\mathbf{v}=\begin{bmatrix}
v_{1} \\ v_{2} \\ \vdots \\ v_{n}
\end{bmatrix}$$
Vectors can be visualised as arrows pointing from the origin to the point $(v_{1}, v_{2},\dots,v_{n})$. For example, the wind velocity in a point in the atmosphere is a vector quantity. The scalar components $v_{1}, v_{2}, v_{3}$ (in this case 3D) depend on the measurement coordinate frame, but the vector itself doesn't change when measured from a different coordinate frame.

Vectors can be added and subtracted from other vectors just like scalars can be added and subtracted from other scalars. When two vectors $\mathbf{u}$ and $\mathbf{v}$ are added $\mathbf{w}=\mathbf{u}+\mathbf{v}$, the components of the two vectors are added in the resulting vector $w_{i}=u_{i}+v_{i}$. Visually, vector addition looks like moving the origin of vector $\mathbf{v}$ to the endpoint of vector $\mathbf{v}$: %%
