# graham_convex_hull

Implementation of the Graham scan method to find the convex hull in rust.

## Convex hull
A convex hull of a shape is defined as the smallest number of points required to enclose the remaining points. 
The convex hull has many applications in Maths, Statistics, Physics and Computer science.

![Image](https://miro.medium.com/max/677/1*F4IUmOJbbLMJiTgHxpoc7Q.png)
![Cow Image](https://www.wolfram.com/mathematica/new-in-10/data-and-mesh-regions/HTMLImages.en/convex-hulls/O_12.png)

## Graham scan algorithm
Graham scan algorithm finds the convex hull of a series of points in O(n log(n)) time. It does so by calculating the angle between every point and the lowest point, 
and then sorting in ascending order.

## Pseudocode
```
Assuming the input length is greater than 3.

1. Find the lowest / furthest left point -> min.
2. Calculate the angle between each point and min using atan2().
3. Sort the points by angle in ascending order -> sorted.
4. Take the first two values from sorted and add them to a stack.
5. Foreach remaining value from sorted: 
6.      Take top two values from stack and point. 
7.      Calculate if the three points make a clockwise of counter clockwise turn.
8.      If clockwise pop stack and recalculate turn, repeating step 7.
9.      If anti clockwise add point to stack

10. Return stack.
```
