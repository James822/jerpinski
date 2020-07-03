use std::env;
use std::process;

fn make_three_tris(p1: (u32, u32), p2: (u32, u32), p3: (u32, u32)) -> [(u32, u32); 9] {
    let mleft = ( (p1.0 + p2.0) / 2, (p1.1 + p2.1) / 2);
    let mbottom = ( (p2.0 + p3.0) / 2, (p2.1 + p3.1) / 2);
    let mright = ( (p3.0 + p1.0) / 2, (p3.1 + p1.1) / 2);

    [p1, mleft, mright, mleft, p2, mbottom, mright, mbottom, p3]
}

fn spawn_triangles(points: &[(u32, u32)], size: usize) -> Vec<(u32, u32)> {
    let mut ans: Vec<(u32, u32)> = Vec::with_capacity(size);

    let mut top: (u32, u32) = (0, 0);
    let mut left: (u32, u32) = (0, 0);
    let mut right: (u32, u32) = (0, 0);
    let mut count = 0;
    for (x, y) in points {
	if count == 0 {
	    top = (*x, *y);
	    count += 1;
	} else if count == 1 {
	    left = (*x, *y);
	    count += 1;
	} else {
	    right = (*x, *y);
	    let new_tris = make_three_tris(top, left, right);
	    for x in &new_tris {
		ans.push(*x);
	    }
	    
	    count = 0;
	}
    }

    ans
}

fn point_in_tri(point: (u32, u32), top: (u32, u32), left: (u32, u32), right: (u32, u32))
		-> bool
{
    let x = point.0 as f32;
    let y = point.1 as f32;
    let x1 = top.0 as f32;
    let y1 = top.1 as f32;
    let x2 = left.0 as f32;
    let y2 = left.1 as f32;
    let x3 = right.0 as f32;
    let y3 = right.1 as f32;
    
    let denom = (y2 - y3)*(x1 - x3) + (x3 - x2)*(y1 - y3);

    let a = ((y2 - y3)*(x - x3) + (x3 - x2)*(y - y3)) / denom;
    let b = ((y3 - y1)*(x - x3) + (x1 - x3)*(y - y3)) / denom;
    let c = 1.0 - a - b;

    (a >= 0.0) && (a <= 1.0) && (b >= 0.0) && (b <= 1.0) && (c >= 0.0) && (c <= 1.0)
}

fn render_points_to_buff(points: &[(u32, u32)], image_buffer: &mut image::RgbImage,
			 window_dim: (u32, u32)) {
    // for triangle in points: ...
    let mut count = 0;
    let mut top: (u32, u32) = (0, 0);
    let mut left: (u32, u32) = (0, 0);
    let mut right: (u32, u32) = (0, 0);
    for (x, y) in points {
	if count == 0 {
	    top = (*x, *y);
	    count += 1;
	} else if count == 1 {
	    left = (*x, *y);
	    count += 1;
	} else {
	    right = (*x, *y);
	    // draw triangle
	    for x in 0..window_dim.0 {
		for y in 0..window_dim.1 {
		    if point_in_tri((x, y), top, left, right) {
			image_buffer.put_pixel(x, y, image::Rgb([20, 200, 240]));
		    }
		}
	    }

	    count = 0;
	}
    }
}

fn gen_first_triangle(points: &mut Vec<(u32, u32)>, window_dim: (u32, u32)) {
    let offset = (0.1625 * (window_dim.0 as f32)) as u32;
    let top = (window_dim.0 / 2, 0);
    let left = (offset, window_dim.1);
    let right = (window_dim.0 - offset - 1, window_dim.1);

    points.push(top);
    points.push(left);
    points.push(right);
}

fn render(image_buffer: &mut image::RgbImage, window_dim: (u32, u32), iterations: u32) {
    
    let three: u32 = 3;
    let num_points: usize = three.pow(iterations) as usize;

    // each three consecutive points makes up a triangle, with overlap
    // triangles are in order of generation, from top->left->right->down
    let mut triangle_points: Vec<(u32, u32)> = Vec::with_capacity(num_points);

    if iterations != 0 {
	gen_first_triangle(&mut triangle_points, window_dim);
    }

    let mut count = iterations;
    while count != 1 {
	triangle_points = spawn_triangles(&triangle_points, num_points);
	
	count -= 1;
    }

    render_points_to_buff(&triangle_points, image_buffer, window_dim);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
	println!("ERROR, expected 3 arguments: <resolution_width> <resolution_height> <iterations>");
	println!("Here is an example of correct usage: ");
	println!("cargo run --release 1920 1080 7");
	println!("The above command use 1920x1080 resolution with 7 iterations");
	process::exit(1);
    }
    let res_x: u32 = args[1].parse().unwrap();
    let res_y: u32 = args[2].parse().unwrap();
	
    let window_dim: (u32, u32) = (res_x, res_y);
    let iterations: u32 = args[3].parse::<u32>().unwrap();

    println!("WINDOW DIMENSIONS / RESOLUTION : {} x {}", window_dim.0, window_dim.1);
    println!("ITERATIONS : {}", iterations);

    let mut image_buffer = image::RgbImage::new(window_dim.0, window_dim.1);

    render(&mut image_buffer, window_dim, iterations);

    image_buffer.save("output/render.png").unwrap();
}
