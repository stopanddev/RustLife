use rand::Rng;
use std::{thread, time, vec};
use std::io::{self, Write};

fn main() {
    let mut rng = rand::thread_rng();
    let delay: i32 = 400;
    let rows: usize = 51;
    let columns: usize = 51;
    let mut my_grid_a: Vec<Vec<i32>> = vec![vec![0; columns]; rows];
    let mut my_grid_b: Vec<Vec<i32>> = vec![vec![0; columns]; rows];
    for i in 0..rows
    {
        for j in 0..columns
        {
            my_grid_a[i][j]= rng.gen_range(0..2);
            my_grid_b[i][j]= rng.gen_range(0..2);
        }
    }
    loop
    {
        copy_grid(&mut my_grid_a, &my_grid_b);
        print!("\x1b[2J\x1b[H");  // Clear the screen and move the cursor to the top-left corner
        io::stdout().flush().unwrap(); 
        // Begins checks for state of cells
        for k in 0..rows
        {
            for l in 0..columns
            {
                check_your_neighbors(k,l,&mut my_grid_a, &mut my_grid_b);
            }
        }
        // Print the "previous" state grid
        for rowz in &my_grid_a
        {
            for &cell in rowz 
            {
                if cell != 0 { print!("*"); } else { print!(" "); }
            }
            println!();
        }
        // Goes way too fast without delay
        thread::sleep(time::Duration::from_millis(delay as u64));
    }
}

fn check_your_neighbors(x: usize, y: usize, grid_a: &mut Vec<Vec<i32>>, grid_b: &mut Vec<Vec<i32>>)
{
    if x == 0 && y == 0 { top_left_corner(x,y,grid_a, grid_b); return; }
    if x == 0 && y == 50 { bottom_left_corner(x,y,grid_a, grid_b); return;}
    if x == 0 { left_column(x,y,grid_a, grid_b); return;}

    if x == 50 && y == 0 { top_right_corner(x,y,grid_a, grid_b); return; }
    if x == 50 && y == 50 { bottom_right_corner(x,y,grid_a, grid_b); return; }
    if x == 50 { right_column(x,y,grid_a, grid_b); return; }

    if y == 0 { top_row(x,y,grid_a, grid_b); return; }
    if y == 50 { bottom_row(x,y,grid_a, grid_b); return; }
    all_surrounding(x,y,grid_a, grid_b);
}


fn all_surrounding(x: usize, y: usize, grid_a: &mut Vec<Vec<i32>>, grid_b: &mut Vec<Vec<i32>>)
{
    let mut count = 0;
    if grid_a[x][y+1] == 1 { count += 1 }
    if grid_a[x][y-1] == 1 { count += 1 }
    if grid_a[x+1][y] == 1 { count += 1 }
    if grid_a[x+1][y+1] == 1 { count += 1 }
    if grid_a[x+1][y-1] == 1 { count += 1 }
    if grid_a[x-1][y] == 1 { count += 1 }
    if grid_a[x-1][y+1] == 1 { count += 1 }
    if grid_a[x-1][y-1] == 1 { count += 1 }
    alive_or_die(x,y,grid_a,grid_b, count);
}

fn left_column(x: usize, y: usize, grid_a: &mut Vec<Vec<i32>>, grid_b: &mut Vec<Vec<i32>>)
{
    let mut count = 0;
    if grid_a[x][y-1] == 1 { count += 1 }
    if grid_a[x+1][y-1] == 1 { count += 1 }
    if grid_a[x+1][y] == 1 { count += 1 }
    if grid_a[x+1][y+1] == 1 { count += 1 }
    if grid_a[x][y+1] == 1 { count += 1 }
    alive_or_die(x,y,grid_a,grid_b, count);
}

fn right_column(x: usize, y: usize, grid_a: &mut Vec<Vec<i32>>, grid_b: &mut Vec<Vec<i32>>) 
{
    let mut count = 0;
    if grid_a[x][y-1] == 1 { count += 1 }
    if grid_a[x-1][y-1] == 1 { count += 1 }
    if grid_a[x-1][y] == 1 { count += 1 }
    if grid_a[x-1][y+1] == 1 { count += 1 }
    if grid_a[x][y+1] == 1 { count += 1 }
    alive_or_die(x,y,grid_a,grid_b, count);
}

fn bottom_row(x: usize, y: usize, grid_a: &mut Vec<Vec<i32>>, grid_b: &mut Vec<Vec<i32>>) 
{
    let mut count = 0;
    if grid_a[x][y-1] == 1 { count += 1 }
    if grid_a[x-1][y-1] == 1 { count += 1 }
    if grid_a[x+1][y-1] == 1 { count += 1 }
    if grid_a[x+1][y] == 1 { count += 1 }
    alive_or_die(x,y,grid_a,grid_b, count);
}

fn bottom_left_corner(x: usize, y: usize, grid_a: &mut Vec<Vec<i32>>, grid_b: &mut Vec<Vec<i32>>) 
{
    let mut count = 0;
    if grid_a[x][y-1] == 1 { count += 1 }
    if grid_a[x+1][y] == 1 { count += 1 }
    if grid_a[x+1][y-1] == 1 { count += 1 } alive_or_die(x,y,grid_a,grid_b, count);
}


fn bottom_right_corner(x: usize, y: usize, grid_a: &mut Vec<Vec<i32>>, grid_b: &mut Vec<Vec<i32>>) 
{
    let mut count = 0;
    if grid_a[x][y-1] == 1 { count += 1 }
    if grid_a[x-1][y] == 1 { count += 1 }
    if grid_a[x-1][y-1] == 1 { count += 1 }
    alive_or_die(x,y,grid_a,grid_b, count);
}

fn top_left_corner(x: usize, y: usize, grid_a: &mut Vec<Vec<i32>>, grid_b: &mut Vec<Vec<i32>>) 
{
    let mut count = 0;
    if grid_a[x+1][y] == 1 { count += 1 }
    if grid_a[x+1][y+1] == 1 { count += 1 }
    if grid_a[x][y+1] == 1 { count += 1 }
    alive_or_die(x,y,grid_a,grid_b, count);
}

fn top_right_corner(x: usize, y: usize, grid_a: &mut Vec<Vec<i32>>, grid_b: &mut Vec<Vec<i32>>) 
{
    let mut count = 0;
    if grid_a[x][y+1] == 1 { count += 1 }
    if grid_a[x-1][y] == 1 { count += 1 }
    if grid_a[x-1][y+1] == 1 { count += 1 }
    alive_or_die(x,y,grid_a,grid_b, count);
}

fn top_row(x: usize, y: usize, grid_a: &mut Vec<Vec<i32>>, grid_b: &mut Vec<Vec<i32>>) 
{
    let mut count = 0;
    if grid_a[x][y+1] == 1 { count += 1 }
    if grid_a[x-1][y+1] == 1 { count += 1 }
    if grid_a[x+1][y+1] == 1 { count += 1 }
    if grid_a[x+1][y] == 1 { count += 1 }
    alive_or_die(x,y,grid_a,grid_b, count);
}

fn copy_grid(grid_1: &mut Vec<Vec<i32>>, grid_2: &Vec<Vec<i32>>)
{
    grid_1.clear();
    for row in grid_2.iter() {
        grid_1.push(row.clone());  // Clone and push each row from my_grid_b into my_grid_a
    }
}

// Change this for how fast/slow they die
fn alive_or_die(x: usize, y: usize, grid_1: &Vec<Vec<i32>>, grid_2: &mut Vec<Vec<i32>>, count: i32)
{
    let dead = grid_1[x][y];
    if dead == 0 && count == 3 { grid_2[x][y]=1; return; }
    if dead == 1 && ( count == 2 || count == 3) { grid_2[x][y]=1; return; }
    grid_2[x][y]=0;
}
