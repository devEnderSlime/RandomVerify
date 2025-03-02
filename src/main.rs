use mersenne_twister::MT19937;
use plotters::prelude::*;
use rand::{Rng, SeedableRng, thread_rng};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 使用系统时间作为种子
    let mut rng = MT19937::new_unseeded();
    let seed: u32 = thread_rng().gen(); // 获取一个随机种子
    rng.reseed(seed); // 使用随机生成的种子

    let mut data = Vec::new();
    let mut angles = Vec::new(); // 用来存储极角

    let randomizer:i128 = 100; //修改参数控制随机数生成数量
    // 生成 一些 随机数并转换为极坐标（仅限第一象限）
    for i in 0..randomizer {
        let random_number = rng.gen::<u32>() % (randomizer as u32 + 1); // 将 randomizer 转换为 u32 并生成随机数

        // 只考虑第一象限的点 (x >= 0, y >= 0)
        let x = i as f64;
        let y = random_number as f64;

        // 计算极坐标，r不需要
        //let r = (x.powi(2) + y.powi(2)).sqrt(); // 极坐标的半径 r
        let theta = y.atan2(x); // 极角 theta

        // 仅处理第一象限的角度
        if theta >= 0.0 && theta <= std::f64::consts::PI / 2.0 {
            data.push((x, y));  // 存储笛卡尔坐标
            angles.push(theta); // 存储极角
        }
    }

    // 计算平均极角
    let avg_angle = angles.iter().sum::<f64>() / angles.len() as f64;

    // 打印平均角度（以度数表示）
    println!("Average Angle: {} degrees", avg_angle.to_degrees());

    // 创建绘图区域
    let root = BitMapBackend::new("scatter_first_quadrant_plot_with_axes.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    // 创建图表
    let mut chart = ChartBuilder::on(&root)
        .caption("Mersenne Twister Random Scatter in First Quadrant", ("Arial", 20))
        .build_cartesian_2d(0f64..100f64, 0f64..100f64)?;

    // 绘制散点图
    chart
        .draw_series(PointSeries::of_element(
            data.iter().cloned(),
            5,
            &BLUE,
            &|c, s, st| Circle::new(c, s, st.filled()),
        ))?
        .label("Random Points")
        .legend(|(x, y)| Circle::new((x, y), 5, &BLUE));

    // 计算缩放后的平均角度线条的终点
    let scale = 50.0; // 控制线条的长度
    let avg_angle_x = avg_angle.cos() * scale; // 使用缩放后的长度
    let avg_angle_y = avg_angle.sin() * scale;

    // 绘制平均角度的线（从原点 (0.0, 0.0) 开始）
    chart
        .draw_series(LineSeries::new(
            vec![(0.0, 0.0), (avg_angle_x, avg_angle_y)], // 从原点 (0.0, 0.0) 开始
            &RED,
        ))?
        .label("Average Angle")
        .legend(|(x, y)| PathElement::new(vec![(x, y)], &RED));

    // 配置坐标轴
    chart
        .configure_mesh()
        .x_desc("X Axis")
        .y_desc("Y Axis")
        .draw()?; // 确保坐标轴可以显示

    // 保存图片
    root.present()?;
    println!("散点图和平均角度图已保存为 scatter_first_quadrant_plot_with_axes.png");

    Ok(())
}

