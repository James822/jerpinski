# Jerpinski

A program that renders iterations of sierpinski's triangle: https://en.wikipedia.org/wiki/Sierpi%C5%84ski_triangle

## EXAMPLES:

2 iterations:
![alt text](https://github.com/James822/jerpinski/blob/master/jerpinski/example_renders/render_iterations_2.png?raw=true)

6 iterations:
![alt text](https://github.com/James822/jerpinski/blob/master/jerpinski/example_renders/render_iterations_6.png?raw=true)

## TUTORIAL:

To use this program simply git clone the repository,
cd into the src/ directory and use this command:

`cargo run --release <resolution_width> <resolution_height> <iterations> `

An example of what you might want:

`cargo run --release 1920 1080 7`

renders a 1920x1080 image with 7 iterations on the triangle.
