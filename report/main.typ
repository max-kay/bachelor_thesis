= Symmetry Implementation for Pair Distribution Funciton

== Finite matrix groups
A few general properties of finite matrix groups are proven in this chapter.

Let $frak(H) subset RR^(n times n)$ be a finite matrix group.
Directly we can tell that $det(A) != 0$ since $A in frak(H)$ must be invertible.
We can use the following composition of group homomorphism to further restrict the set $frak(H)$
It is well established that $det: RR^(n times n) -> RR$ is a group homomorohism.
Furthermore note that $abs(dot): RR without {0} -> RR_(>0)$ is a homomorohism too as $abs(x y) = abs(x) abs(y)$ holds for all $x,y in RR$.

By $abs(det(A B)) = abs(det(A) det(B)) = abs(det(A)) abs(det(B))$ the composition is a homomorohism too. 
Since 