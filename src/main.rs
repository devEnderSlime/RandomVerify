use mersenne_twister::MT19937;
use plotters::prelude::*;
use rand::{Rng, SeedableRng, thread_rng};
use std::io::{self, Write}; // 导入输入输出模块
use std::thread; // 导入 thread 模块
use std::time::Duration; // 导入 Duration

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 提示用户选择模式
    print!("请输入 '1' 使用随机种子，或者 '2' 使用固定种子（5489）: ");
    io::stdout().flush().unwrap(); // 确保输出立即显示

    let mut input = String::new(); // 创建一个可变字符串来存储用户输入

    // 读取用户输入
    io::stdin().read_line(&mut input).unwrap();

    let seed_choice: i32 = input.trim().parse().unwrap_or(1); // 默认选择随机种子

    // 使用系统时间作为种子或固定种子5489
    let mut rng = MT19937::new_unseeded();

    if seed_choice == 2 {
        rng.reseed(5489u32); // 使用固定种子 5489
        println!("使用固定种子 5489");
    } else {
        let seed: u32 = thread_rng().gen(); // 获取一个随机种子
        rng.reseed(seed); // 使用随机生成的种子
        println!("使用随机种子");
    }

    let mut data = Vec::new();
    let mut angles = Vec::new(); // 用来存储极角

    // 提示用户输入生成随机数的数量
    print!("请输入一个正整数来设置随机数生成数量: ");
    io::stdout().flush().unwrap(); // 确保输出立即显示

    let mut input = String::new(); // 创建一个可变字符串来存储用户输入
    io::stdin().read_line(&mut input).unwrap();
    let randomizer: i128 = input.trim().parse().unwrap_or(100); // 默认值为 100

    // 生成指定数量的随机数并转换为极坐标（仅限第一象限）
    for i in 0..randomizer {
        let random_number = rng.gen::<u32>() % (randomizer as u32 + 1); // 将 randomizer 转换为 u32 并生成随机数

        // 只考虑第一象限的点 (x >= 0, y >= 0)
        let x = i as f64;
        let y = random_number as f64;

        // 计算极坐标，r不需要
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

    // 创建绘图区域，并增加分辨率
    let root = BitMapBackend::new("scatter_first_quadrant_plot_with_axes.png", (3840, 2160))
        .into_drawing_area(); // 设置更大的分辨率
    root.fill(&WHITE)?;

    // 创建图表并调整字体大小
    let mut chart = ChartBuilder::on(&root)
        .caption("Mersenne Twister Random Scatter in First Quadrant", ("Arial", 50)) // 增加标题字体大小
        .build_cartesian_2d(0f64..100f64, 0f64..100f64)?;

    // 绘制散点图并增加点的大小
    chart
        .draw_series(PointSeries::of_element(
            data.iter().cloned(),
            20, // 增加点的大小
            &BLUE,
            &|c, s, st| Circle::new(c, s, st.filled()),
        ))?
        .label("Random Points")
        .legend(|(x, y)| Circle::new((x, y), 20, &BLUE)); // 增加点的大小

    // 绘制平均角度的线（从原点 (0.0, 0.0) 开始）
    let scale = 50.0;
    let avg_angle_x = avg_angle.cos() * scale;
    let avg_angle_y = avg_angle.sin() * scale;

    chart
        .draw_series(LineSeries::new(
            vec![(0.0, 0.0), (avg_angle_x, avg_angle_y)], // 从原点 (0.0, 0.0) 开始
            &RED,
        ))?
        .label("Average Angle")
        .legend(|(x, y)| PathElement::new(vec![(x, y)], &RED));

    // 计算并显示坐标轴
    chart.configure_mesh().x_labels(30).y_labels(30).draw()?; // 增加网格的标签数量

    // 保存图片
    root.present()?;
    println!("散点图和平均角度图已保存为 scatter_first_quadrant_plot_with_axes.png");

    // 暂停 10 秒钟
    println!("程序将在 10 秒后结束...");
    thread::sleep(Duration::new(10, 0)); // 暂停 10 秒

    Ok(())
}
