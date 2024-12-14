use crate::deep_learning::neural::Neural;

fn main() {
    // 创建一个用于数字分类的神经网络 (输入:1, 隐藏:4, 输出:1)
    let mut network = Neural::new(&[1, 4, 1]);

    // 创建训练数据: (输入, 期望输出)
    // 简单任务: 判断数字是否大于0.5
    let training_data: Vec<(Vec<f64>, Vec<f64>)> = vec![
        (vec![0.1], vec![0.0]),
        (vec![0.2], vec![0.0]),
        (vec![0.8], vec![1.0]),
        (vec![0.9], vec![1.0]),
        (vec![0.3], vec![0.0]),
        (vec![0.7], vec![1.0]),
    ];

    // 训练10000轮
    for epoch in 0..10000 {
        let mut total_error = 0.0;

        // 每轮训练所有样本
        for (inputs, targets) in &training_data {
            network.train(inputs, targets);

            // 计算误差
            let output = network.forward(inputs);
            total_error += (output[0] - targets[0]).powi(2);
        }

        // 每1000轮显示一次进度
        if epoch % 1000 == 0 {
            println!(
                "Epoch {}, Error: {:.6}",
                epoch,
                total_error / training_data.len() as f64
            );
        }
    }

    // 测试网络
    println!("\nTesting the network:");
    let test_inputs = vec![0.15, 0.35, 0.65, 0.85];
    for input in test_inputs {
        let output = network.forward(&vec![input]);
        println!("Input: {:.2}, Output: {:.4}", input, output[0]);
    }
}
