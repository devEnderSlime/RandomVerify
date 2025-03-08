use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use rand_distr::{Zipf, Pareto, Distribution};
use plotters::prelude::*;
use plotters::style::{ShapeStyle, Color};  // 导入 ShapeStyle 和 Color
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 用户选择随机数生成方式
    print!("请输入 '1' 选择梅森旋转算法（MT19937），或者 '2' 选择非均匀分布: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let rng_choice: i32 = input.trim().parse().unwrap_or(1);

    // 用户选择种子类型
    print!("请输入 '1' 使用随机种子，或者 '2' 使用固定种子（5489）: ");
    io::stdout().flush().unwrap();
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let seed_choice: i32 = input.trim().parse().unwrap_or(1);

    // 初始化随机数生成器
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

    // 定义Zipf分布和Pareto分布
    let zipf = Zipf::new(10000, 1.1)?;
    let pareto = Pareto::new(1.0, 3.0)?;

    // 用户输入随机数生成数量
    print!("请输入随机数生成数量: ");
    io::stdout().flush().unwrap();
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let randomizer: usize = input.trim().parse().unwrap_or(1000);

    // 存储生成的随机数据和角度数据
    let mut data = Vec::new();
    let mut angles = Vec::new();

    let x_min = 1.0;
    let x_max = randomizer as f64;

    // 生成随机数并计算角度
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
        angles.push(theta);
    }

    // 计算平均角度
    let avg_angle = angles.iter().sum::<f64>() / angles.len() as f64;
    println!("Average Angle: {:.5} degrees", avg_angle.to_degrees());

    // 动态计算坐标轴范围
    let x_min_dynamic = data.iter().map(|(x, _)| *x).fold(f64::INFINITY, f64::min);
    let x_max_dynamic = data.iter().map(|(x, _)| *x).fold(f64::NEG_INFINITY, f64::max);
    let y_min_dynamic = data.iter().map(|(_, y)| *y).fold(f64::INFINITY, f64::min);
    let y_max_dynamic = data.iter().map(|(_, y)| *y).fold(f64::NEG_INFINITY, f64::max);

    // 设置坐标轴范围为数据点的范围
    let x_range = (x_min_dynamic - 0.05)..(x_max_dynamic + 0.05);
    let y_range = (y_min_dynamic - 0.05)..(y_max_dynamic + 0.05);

    // 创建并设置绘图区域
    let root = BitMapBackend::new("scatter_non_uniform.png", (3840, 2160)).into_drawing_area();
    root.fill(&WHITE)?;

    // 创建图表
    let mut chart = ChartBuilder::on(&root)
        .caption("Scatter Plot", ("Arial", 50))
        .build_cartesian_2d(x_range, y_range)?;

    // 绘制散点图
    chart.draw_series(PointSeries::of_element(
        data.iter().cloned(),
        2,
        &RGBAColor(0, 0, 255, 0.7), // 设置颜色为蓝色
        &|c, s, st| Circle::new(c, s, st.filled()),
    ))?
        .label("Random Points")
        .legend(|(x, y)| Circle::new((x, y), 2, &RGBAColor(0, 0, 255, 0.7)));

    // 绘制平均角度线
    let scale = 50.0;
    let avg_angle_x = x_min_dynamic + avg_angle.cos() * scale;
    let avg_angle_y = y_min_dynamic + avg_angle.sin() * scale;
    chart.draw_series(LineSeries::new(
        vec![(x_min_dynamic, y_min_dynamic), (avg_angle_x, avg_angle_y)], // 连接坐标点
        ShapeStyle {  // 创建 ShapeStyle 对象
            color: RGBAColor(255, 0, 0, 0.8),  // 设置线条颜色（红色，透明度0.8）
            filled: true,  // 填充线条区域
            stroke_width: 5,  // 设置线条宽度
        },
    ))?
        .label("Average Angle")  // 设置图例标签
        .legend(|(x, y)| PathElement::new(vec![(x, y)], &RGBAColor(255, 0, 0, 0.8)));  // 设置图例

    // 配置图表网格
    chart.configure_mesh().x_labels(30).y_labels(30).draw()?;
    root.present()?;

    // 输出文件保存提示
    println!("散点图已保存为 scatter_non_uniform.png");

    // 程序结束延迟
    println!("程序将在 10 秒后结束...");
    thread::sleep(Duration::new(10, 0));
    Ok(())
}
