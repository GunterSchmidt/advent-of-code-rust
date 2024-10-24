# Advent of Code 2023 Day 6

This puzzle is quite easy to solve with brute force. The more interesting part is
to make the code fast by using mathematics, which in my case is 100000 times faster for part 2.
Rewriting the parser saved another 50%.

The resulting race distance (d) is calculated as the (race time (t) minus the time for the button press (b))
times the resulting speed (s). Goal is to beat the current record (r).  
d = (t - b) * s  
S is calculated by the time of the button press and in this case B = S  
d = (t - s) * s  
Since the current record must be broken, the minimum result is > R.  
r + 1 = (t - s) * s  
or in example 30  
200 = (30 - s) * s  
This can be written as  
201 = 30s - s^2  
s^2 - 30s + 200 = 0  
Which is the Quadratic Equation or pq formula x^2 + px + q = 0.  
[Quadratic_equation](https://en.wikipedia.org/wiki/Quadratic_equation)  
x = -(x/2) +/- SQRT((p/2)^2-q)  
s = -(-30/2) +/- SQRT((30/2)^2-200)  
s = 15 +/-

---

part1_loop:  0.000619 ms  
part1_math:  0.000151 ms  

part2_loop: 22.264000 ms  
part2_math:  0.000118 ms  

