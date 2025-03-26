use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use rand_distr::{Zipf, Pareto, Distribution};
use plotters::prelude::*;
use plotters::style::{ShapeStyle, Color};
use std::io::{self, Write};
use std::thread;
use std::time::{Duration, Instant};
use std::cmp::Ordering;

/// Manually computes the KS statistic for a sorted sample assuming a Uniform(0,1) theoretical CDF.
fn ks_test_uniform(sorted_sample: &[f64]) -> f64 {
    let n = sorted_sample.len() as f64;
    let mut d:f64 = 0.0;
    for (i, &x) in sorted_sample.iter().enumerate() {
        let empirical_cdf = (i as f64 + 1.0) / n;
        let lower_bound = i as f64 / n;
        let diff_upper = (empirical_cdf - x).abs();
        let diff_lower = (x - lower_bound).abs();
        d = d.max(diff_upper).max(diff_lower);
    }
    d
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start_time = Instant::now();

    // User selects the random number generation method.
    print!("Enter '1' to choose Mersenne Twister (MT19937), or '2' for non-uniform distribution: ");
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let rng_choice: i32 = input.trim().parse().unwrap_or(1);

    // User selects seed type.
    print!("Enter '1' for random seed, or '2' for fixed seed (5489): ");
    io::stdout().flush()?;
    input.clear();
    io::stdin().read_line(&mut input)?;
    let seed_choice: i32 = input.trim().parse().unwrap_or(1);

    // Initialize random number generators.
    let mut rng_mt = if seed_choice == 2 {
        StdRng::seed_from_u64(5489)
    } else {
        StdRng::from_entropy()
    };
    let mut rng_non_uniform = if seed_choice == 2 {
        StdRng::seed_from_u64(5489)
    } else {
        StdRng::from_entropy()
    };

    // Define Zipf and Pareto distributions.
    let zipf = Zipf::new(10000, 1.1)?;
    let pareto = Pareto::new(1.0, 3.0)?;

    // User inputs the number of random numbers to generate.
    print!("Enter the number of random numbers to generate: ");
    io::stdout().flush()?;
    input.clear();
    io::stdin().read_line(&mut input)?;
    let randomizer: usize = input.trim().parse().unwrap_or(1000);

    // Prepare containers.
    let mut data = Vec::with_capacity(randomizer);
    let mut random_numbers = Vec::with_capacity(randomizer);
    let mut sum_angles = 0.0;  // Accumulate angles on the fly.
    let x_min = 1.0;
    let x_max = randomizer as f64;

    let gen_start = Instant::now();

    // Generate random numbers and compute angles, accumulating sum_angles.
    for _ in 0..randomizer {
        let rnd_x = if rng_choice == 1 {
            rng_mt.gen_range(x_min..x_max)
        } else {
            zipf.sample(&mut rng_non_uniform) as f64
        };

        let rnd_y = if rng_choice == 1 {
            rng_mt.gen_range(x_min..x_max)
        } else {
            pareto.sample(&mut rng_non_uniform)
        };

        let theta = rnd_y.atan2(rnd_x);
        data.push((rnd_x, rnd_y));
        random_numbers.push(rnd_x);
        sum_angles += theta;
    }

    let gen_duration = gen_start.elapsed();
    println!("Random number generation time: {:?}", gen_duration);

    // Time the angle accumulation and average calculation.
    let angle_start = Instant::now();
    let avg_angle = sum_angles / randomizer as f64;
    let angle_duration = angle_start.elapsed();
    println!("Angle accumulation and average calculation time: {:?}", angle_duration);
    println!("Average Angle: {:.5} degrees", avg_angle.to_degrees());

    // Perform manual KS test.
    let ks_start = Instant::now();
    // Normalize random_numbers to [0,1] by dividing by x_max.
    let mut normalized: Vec<f64> = random_numbers.iter().map(|&x| x / x_max).collect();
    // Sort normalized vector (O(n log n)).
    normalized.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
    let ks_statistic = ks_test_uniform(&normalized);
    let ks_duration = ks_start.elapsed();
    println!("Manual KS Test Statistic: {:.5}", ks_statistic);
    println!("Manual KS Test computation time: {:?}", ks_duration);

    // Dynamically calculate the axis range for the scatter plot.
    let x_min_dynamic = data.iter().map(|(x, _)| *x).fold(f64::INFINITY, f64::min);
    let x_max_dynamic = data.iter().map(|(x, _)| *x).fold(f64::NEG_INFINITY, f64::max);
    let y_min_dynamic = data.iter().map(|(_, y)| *y).fold(f64::INFINITY, f64::min);
    let y_max_dynamic = data.iter().map(|(_, y)| *y).fold(f64::NEG_INFINITY, f64::max);

    let x_range = (x_min_dynamic - 0.05)..(x_max_dynamic + 0.05);
    let y_range = (y_min_dynamic - 0.05)..(y_max_dynamic + 0.05);

    // Create and set up the drawing area for the scatter plot.
    let root = SVGBackend::new("scatter_non_uniform.svg", (640, 480)).into_drawing_area();  // Change to SVG format
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Scatter Plot", ("Arial", 50))
        .build_cartesian_2d(x_range, y_range)?;

    // Draw scatter plot.
    chart.draw_series(
        data.iter().cloned().map(|(x, y)| Circle::new((x, y), 2, &BLUE.mix(0.7)))
    )?
        .label("Random Points")
        .legend(|(x, y)| Circle::new((x, y), 2, &BLUE.mix(0.7)));

    // Draw average angle line.
    let scale = 50.0;
    let avg_angle_x = x_min_dynamic + avg_angle.cos() * scale;
    let avg_angle_y = y_min_dynamic + avg_angle.sin() * scale;
    chart.draw_series(LineSeries::new(
        vec![(x_min_dynamic, y_min_dynamic), (avg_angle_x, avg_angle_y)],
        ShapeStyle { color: RED.mix(0.8), filled: true, stroke_width: 5 },
    ))?
        .label("Average Angle")
        .legend(|(x, y)| PathElement::new(vec![(x, y)], &RED.mix(0.8)));

    chart.configure_mesh().x_labels(30).y_labels(30).draw()?;
    root.present()?;
    println!("Scatter plot generated successfully!");

    println!("Total execution time: {:?}", start_time.elapsed());
    println!("The program will exit shortly...");
    thread::sleep(Duration::from_secs(10));
    Ok(())
}
